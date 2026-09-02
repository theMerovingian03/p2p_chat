use crate::data_channel::dc_manager::DcManager;
// PeerConnection, create_peer_connection, etc.
use crate::utilities::peer_handler::*;
use crate::utilities::signalizer::Signaling;
use crate::websocket::manager::WebSocketManager;
use shared::models::websocket_models::{ClientEvent, IceCandidate};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, error};
use uuid::Uuid;
use webrtc::peer_connection::{PeerConnection, RTCIceCandidateInit, RTCSessionDescription};

/// Event sent when a PeerConnection cleanup is needed.
/// Includes both peer_id and connection_id to safely identify which specific connection
/// needs cleanup. This prevents race conditions where an old connection's cleanup callback
/// could accidentally remove a newly-created connection for the same peer.
#[derive(Clone, Copy, Debug)]
pub struct PeerCleanupEvent {
    pub peer_id: Uuid,
    pub connection_id: Uuid,
}

/// Maps each peer_id to the connection_id of its currently-active PeerConnection.
/// When cleanup occurs, we check if the connection_id still matches before removing.
pub type PeerConnectionMap = HashMap<Uuid, (Arc<dyn PeerConnection>, Uuid)>;

pub struct WebRtcManager {
    /// Stores (PeerConnection, connection_id) tuples indexed by peer_id
    peers: Mutex<PeerConnectionMap>,
    // Buffered ICE candidates - also keyed by peer_id, but ideally should also track connection_id
    pending_candidates: Mutex<HashMap<Uuid, Vec<IceCandidate>>>,
    dc_manager: Arc<DcManager>,
    signaling: Arc<dyn Signaling>,
    cleanup_tx: mpsc::Sender<PeerCleanupEvent>,
    cleanup_rx: Mutex<Option<mpsc::Receiver<PeerCleanupEvent>>>,
}

#[async_trait::async_trait]
impl Signaling for WebSocketManager {
    async fn send(&self, event: ClientEvent) -> Result<(), String> {
        self.send_event(event).await
    }
}

impl WebRtcManager {
    pub fn new(signaling: Arc<dyn Signaling>, dc_manager: Arc<DcManager>) -> Arc<Self> {
        let (cleanup_tx, cleanup_rx) = mpsc::channel::<PeerCleanupEvent>(100);

        Arc::new(Self {
            peers: Mutex::new(HashMap::new()),
            pending_candidates: Mutex::new(HashMap::new()),
            dc_manager,
            signaling,
            cleanup_tx,
            cleanup_rx: Mutex::new(Some(cleanup_rx)),
        })
    }

    /// Initialize the cleanup task. Must be called after Tokio runtime is available.
    /// This is typically called from the Tauri setup function.
    pub async fn init_cleanup_task(self: &Arc<Self>) {
        let cleanup_rx = {
            let mut rx = self.cleanup_rx.lock().await;
            rx.take()
        };

        if let Some(mut cleanup_rx) = cleanup_rx {
            let manager_clone = Arc::clone(self);
            tokio::spawn(async move {
                while let Some(cleanup_event) = cleanup_rx.recv().await {
                    if let Err(error) = manager_clone.cleanup_peer_connection(cleanup_event).await {
                        error!(
                            peer_id = %cleanup_event.peer_id,
                            connection_id = %cleanup_event.connection_id,
                            %error,
                            "Failed to cleanup peer connection"
                        );
                    }
                }
            });
        }
    }
    // Helper to check if peer connection already exists
    async fn get_or_create_peer_connection(
        &self,
        peer_id: Uuid,
    ) -> Result<Arc<dyn PeerConnection>, String> {
        // First, check whether it already exists.
        debug!("Creating peer connection if not exists");
        {
            let peers = self.peers.lock().await;
            if let Some((peer_connection, _connection_id)) = peers.get(&peer_id) {
                return Ok(Arc::clone(peer_connection)); // Clone of Arc pointer for PeerConnection
            }
        }

        // Generate a unique connection_id for this PeerConnection
        let connection_id = Uuid::new_v4();
        debug!(
            "Creating new peer connection with connection_id: {}",
            connection_id
        );

        // Create the connection without holding the lock
        let peer_handler = PeerHandler {
            peer_id,
            connection_id,
            signaling: Arc::clone(&self.signaling),
            dc_manager: Arc::clone(&self.dc_manager),
            cleanup_tx: self.cleanup_tx.clone(),
        };

        // TODO: Optimization If existing connection is used, this is an unnecessary create
        let peer_connection = create_peer_connection(peer_handler).await?;
        let mut peers = self.peers.lock().await;

        // Another task could have created one while we were awaiting.
        // Prefer the existing one in that case.
        if let Some((existing, _)) = peers.get(&peer_id) {
            return Ok(Arc::clone(existing));
        }
        peers.insert(peer_id, (Arc::clone(&peer_connection), connection_id));

        Ok(peer_connection)
    }

    pub async fn create_offer(&self, peer_id: Uuid) -> Result<(), String> {
        // Peer connection
        let pc = self.get_or_create_peer_connection(peer_id).await?;

        // Data channel for media
        let data_channel = pc
            .create_data_channel("chat", None)
            .await
            .map_err(|e| e.to_string())?;

        self.dc_manager
            .add_data_channel(peer_id, data_channel)
            .await;

        // Create offer
        let offer = pc.create_offer(None).await.map_err(|e| e.to_string())?;

        // Set sdp
        pc.set_local_description(offer)
            .await
            .map_err(|e| e.to_string())?;

        // Extract local descrption for sdp to ensure ICE Candidate is present
        let local_description = pc
            .local_description()
            .await
            .ok_or_else(|| "Local description not available".to_string())?;

        // Finally send offer to server, which will route to appropriate ID
        self.signaling
            // calls WebsocketManager's send_event
            .send(ClientEvent::WebRtcOffer {
                to: peer_id,
                sdp: local_description.sdp,
            })
            .await?;

        Ok(())
    }

    pub async fn handle_offer(&self, peer_id: Uuid, sdp: String) -> Result<(), String> {
        debug!("Handling WebRTC Offer");
        let pc = self.get_or_create_peer_connection(peer_id).await?;

        // Convert sdp string to RTCSessionDescription
        let offer = RTCSessionDescription::offer(sdp).map_err(|e| e.to_string())?;

        // Process ICE candidates that arrived before the offer
        let pending_candidates = {
            let mut pending = self.pending_candidates.lock().await;
            // Set this as the remote descriptor of the connection.
            pc.set_remote_description(offer)
                .await
                .map_err(|e| e.to_string())?;
            pending.remove(&peer_id).unwrap_or_default()
        };

        for ice_candidate in pending_candidates {
            let candidate = RTCIceCandidateInit {
                candidate: ice_candidate.candidate,
                sdp_mid: ice_candidate.sdp_mid,
                sdp_mline_index: ice_candidate.sdp_mline_index,
                username_fragment: ice_candidate.username_fragment,
                url: None,
            };

            pc.add_ice_candidate(candidate)
                .await
                .map_err(|e| e.to_string())?;
        }
        let answer = pc.create_answer(None).await.map_err(|e| e.to_string())?;
        pc.set_local_description(answer)
            .await
            .map_err(|e| e.to_string())?;

        let local_description = pc
            .local_description()
            .await
            .ok_or_else(|| "Local description not available".to_string())?;

        self.signaling
            .send(ClientEvent::WebRtcAnswer {
                to: peer_id,
                sdp: local_description.sdp,
            })
            .await?;

        Ok(())
    }

    pub async fn handle_answer(&self, peer_id: Uuid, sdp: String) -> Result<(), String> {
        let pc = {
            let peers = self.peers.lock().await;
            peers
                .get(&peer_id)
                .map(|(pc, _)| Arc::clone(pc))
                .ok_or_else(|| "No peer found!".to_string())?
        };

        let answer = RTCSessionDescription::answer(sdp).map_err(|e| e.to_string())?;
        pc.set_remote_description(answer)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn cleanup_peer_connection(
        &self,
        cleanup_event: PeerCleanupEvent,
    ) -> Result<(), String> {
        let peer_id = cleanup_event.peer_id;
        let connection_id = cleanup_event.connection_id;

        // Only remove the PeerConnection if the connection_id still matches.
        // This prevents an old connection's cleanup callback from accidentally removing
        // a newly-created connection for the same peer.
        let pc = {
            let mut peers = self.peers.lock().await;
            if let Some((_stored_pc, stored_connection_id)) = peers.get(&peer_id) {
                if *stored_connection_id == connection_id {
                    // This is the correct connection - remove it
                    peers.remove(&peer_id).map(|(pc, _)| pc)
                } else {
                    // A newer connection exists for this peer - do not remove
                    debug!(
                        peer_id = %peer_id,
                        old_connection_id = %connection_id,
                        current_connection_id = %stored_connection_id,
                        "Ignoring cleanup for stale connection; newer connection exists for this peer"
                    );
                    None
                }
            } else {
                // No connection stored for this peer - nothing to clean up
                None
            }
        };

        if let Some(pc) = pc {
            if let Err(e) = pc.close().await {
                error!(
                    peer_id = %peer_id,
                    connection_id = %connection_id,
                    %e,
                    "Error closing peer connection"
                );
            }
        }

        // Remove data channel (idempotent operation)
        self.dc_manager.remove_data_channel(peer_id);

        // Remove pending ICE candidates (idempotent operation)
        self.pending_candidates.lock().await.remove(&peer_id);

        Ok(())
    }

    pub async fn handle_ice_candidate(
        &self,
        from: Uuid,
        ice_candidate: IceCandidate,
    ) -> Result<(), String> {
        let pc = {
            let peers = &self.peers.lock().await;
            peers
                .get(&from)
                .map(|(pc, _)| Arc::clone(pc))
                .ok_or_else(|| "No peer found!".to_string())?
        };

        // Check whether we have received/set a remote description yet before adding an ICE candidate
        // Avoids race condition
        let mut pending = self.pending_candidates.lock().await;
        if pc.remote_description().await.is_none() {
            pending.entry(from).or_default().push(ice_candidate);
            return Ok(());
        }
        drop(pending);

        let candidate = RTCIceCandidateInit {
            candidate: ice_candidate.candidate,
            sdp_mid: ice_candidate.sdp_mid,
            sdp_mline_index: ice_candidate.sdp_mline_index,
            username_fragment: ice_candidate.username_fragment,
            url: None,
        };

        pc.add_ice_candidate(candidate)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Get the current connection_id for a peer, if a connection exists.
    /// Used for manual cleanup operations where we want to close the current connection.
    async fn get_current_connection_id(&self, peer_id: Uuid) -> Option<Uuid> {
        self.peers
            .lock()
            .await
            .get(&peer_id)
            .map(|(_, connection_id)| *connection_id)
    }

    /// Manually close a peer connection (called from frontend).
    /// This closes whatever connection is currently active for the peer.
    pub async fn close_peer_connection_manual(&self, peer_id: Uuid) -> Result<(), String> {
        if let Some(connection_id) = self.get_current_connection_id(peer_id).await {
            self.cleanup_peer_connection(PeerCleanupEvent {
                peer_id,
                connection_id,
            })
            .await
        } else {
            // No active connection to close
            Ok(())
        }
    }

    pub async fn clear(&self) {
        let peers = {
            let mut peers = self.peers.lock().await;
            peers.drain().collect::<Vec<_>>()
        };

        for (peer_id, (pc, _connection_id)) in peers {
            if let Err(e) = pc.close().await {
                error!(
                    peer_id = %peer_id,
                    error = %e,
                    "Failed to close peer connection!"
                );
            }
        }

        // Also clear pending ICE candidates
        self.pending_candidates.lock().await.clear();
    }
}

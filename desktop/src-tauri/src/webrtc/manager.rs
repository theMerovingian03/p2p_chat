use crate::data_channel::dc_manager::DcManager;
// PeerConnection, create_peer_connection, etc.
use crate::utilities::peer_handler::*;
use crate::utilities::signalizer::Signaling;
use crate::websocket::manager::WebSocketManager;
use shared::models::websocket_models::{ClientEvent, IceCandidate};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::debug;
use uuid::Uuid;
use webrtc::peer_connection::{PeerConnection, RTCIceCandidateInit, RTCSessionDescription};

pub struct WebRtcManager {
    peers: Mutex<HashMap<Uuid, Arc<dyn PeerConnection>>>,
    // Buffered ICE candidates
    pending_candidates: Mutex<HashMap<Uuid, Vec<IceCandidate>>>,
    dc_manager: Arc<DcManager>,
    signaling: Arc<dyn Signaling>,
}

#[async_trait::async_trait]
impl Signaling for WebSocketManager {
    async fn send(&self, event: ClientEvent) -> Result<(), String> {
        self.send_event(event).await
    }
}

impl WebRtcManager {
    pub fn new(signaling: Arc<dyn Signaling>, dc_manager: Arc<DcManager>) -> Self {
        Self {
            peers: Mutex::new(HashMap::new()),
            pending_candidates: Mutex::new(HashMap::new()),
            dc_manager,
            signaling,
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
            if let Some(peer_connection) = peers.get(&peer_id) {
                return Ok(Arc::clone(peer_connection)); // Clone of Arc pointer for PeerConnection
            }
        }

        // Create the connection without holding the lock
        let peer_handler = PeerHandler {
            peer_id,
            signaling: Arc::clone(&self.signaling),
            dc_manager: Arc::clone(&self.dc_manager),
        };

        // TODO: Optimization If existing connection is used, this is an unnecessary create
        let peer_connection = create_peer_connection(peer_handler).await?;
        let mut peers = self.peers.lock().await;

        // Another task could have created one while we were awaiting.
        // Prefer the existing one in that case.
        if let Some(existing) = peers.get(&peer_id) {
            return Ok(Arc::clone(existing));
        }
        peers.insert(peer_id, Arc::clone(&peer_connection));

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
                .cloned()
                .ok_or_else(|| "No peer found!".to_string())?
        };

        let answer = RTCSessionDescription::answer(sdp).map_err(|e| e.to_string())?;
        pc.set_remote_description(answer)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn cleanup_peer_connection(&self, peer_id: Uuid) -> Result<(), String> {
        let pc = self.peers.lock().await.remove(&peer_id);
        if let Some(pc) = pc {
            pc.close().await.map_err(|e| e.to_string())?;
        }
        // Remove data channel
        // No need to call channel.close() since it's already closed at this point.
        // self.data_channels.lock().await.remove(&peer_id);
        self.dc_manager.remove_data_channel(peer_id);

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
                .cloned()
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
}

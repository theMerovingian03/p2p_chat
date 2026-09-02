use crate::{
    data_channel::dc_manager::DcManager, utilities::signalizer::Signaling,
    webrtc::manager::PeerCleanupEvent,
};
use shared::models::websocket_models::{ClientEvent, IceCandidate};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info};
use uuid::Uuid;
use webrtc::{
    data_channel::DataChannel,
    peer_connection::{
        PeerConnection, PeerConnectionBuilder, PeerConnectionEventHandler, RTCConfigurationBuilder,
        RTCIceServer, RTCPeerConnectionIceEvent, RTCPeerConnectionState,
    },
};

#[derive(Clone)]
pub struct PeerHandler {
    pub peer_id: Uuid,
    /// Unique identifier for this specific PeerConnection. Generated when the connection is created.
    /// Used to safely identify which connection should be cleaned up, preventing race conditions
    /// where an old connection's cleanup callback could accidentally remove a newer connection.
    pub connection_id: Uuid,
    pub signaling: Arc<dyn Signaling>,
    pub dc_manager: Arc<DcManager>,
    pub cleanup_tx: mpsc::Sender<PeerCleanupEvent>,
}

#[async_trait::async_trait]
impl PeerConnectionEventHandler for PeerHandler {
    async fn on_ice_candidate(&self, event: RTCPeerConnectionIceEvent) {
        match event.candidate.to_json() {
            Ok(candidate) => {
                let event = ClientEvent::IceCandidate {
                    to: self.peer_id,
                    candidate: IceCandidate {
                        candidate: candidate.candidate,
                        sdp_mid: candidate.sdp_mid,
                        sdp_mline_index: candidate.sdp_mline_index,
                        username_fragment: candidate.username_fragment,
                    },
                };

                if let Err(e) = self.signaling.send(event).await {
                    tracing::error!(
                        peer_id = %self.peer_id,
                        error = %e,
                        "Failed to send ICE candidate"
                    );
                }
            }
            Err(e) => {
                tracing::error!(
                    peer_id = %self.peer_id,
                    error = %e,
                    "Failed to serialize ICE candidate"
                );
            }
        }
    }
    // TODO: Cleanup, retry, etc.
    async fn on_connection_state_change(&self, state: RTCPeerConnectionState) {
        match state {
            RTCPeerConnectionState::New => info!("New peer connection available!"),
            RTCPeerConnectionState::Connected => info!("Peer connected!"),
            RTCPeerConnectionState::Connecting => info!("Connecting to peer..."),
            RTCPeerConnectionState::Disconnected => {
                info!("Peer disconnected (but not terminal): {}", self.peer_id);
                let _ = self
                    .cleanup_tx
                    .send(crate::webrtc::manager::PeerCleanupEvent {
                        peer_id: self.peer_id,
                        connection_id: self.connection_id,
                    })
                    .await;
                self.dc_manager.remove_data_channel(self.peer_id);
            }
            RTCPeerConnectionState::Failed => {
                error!("Peer connection failed: {}", self.peer_id);
                // Failed is a terminal state - trigger cleanup of the PeerConnection.
                // Include connection_id to ensure we only clean up this specific connection.
                let _ = self
                    .cleanup_tx
                    .send(crate::webrtc::manager::PeerCleanupEvent {
                        peer_id: self.peer_id,
                        connection_id: self.connection_id,
                    })
                    .await;
                self.dc_manager.remove_data_channel(self.peer_id);
            }
            RTCPeerConnectionState::Closed => {
                info!("Closed peer connection: {}", self.peer_id);
                // Closed is a terminal state - trigger cleanup of the PeerConnection.
                // Include connection_id to ensure we only clean up this specific connection.
                self.dc_manager.remove_data_channel(self.peer_id);
                let _ = self
                    .cleanup_tx
                    .send(crate::webrtc::manager::PeerCleanupEvent {
                        peer_id: self.peer_id,
                        connection_id: self.connection_id,
                    })
                    .await;
            }
            _ => {}
        }
    }
    async fn on_data_channel(&self, channel: Arc<dyn DataChannel>) {
        info!("Data channel received");

        if channel.label().await != Ok("chat".to_string()) {
            error!("Invalid Data channel label");
            return;
        }

        // let event_tx = self.event_tx.clone();
        let peer_id = self.peer_id;

        // Add data channel for current user(receiver) and listen
        debug!("Spawning data channel listener for receiver");
        self.dc_manager.add_data_channel(peer_id, channel).await;
    }
}

pub async fn create_peer_connection(
    handler: PeerHandler,
) -> Result<Arc<dyn PeerConnection>, String> {
    let config = RTCConfigurationBuilder::default()
        .with_ice_servers(vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }])
        .build();

    let pc = PeerConnectionBuilder::new()
        .with_configuration(config)
        .with_handler(Arc::new(handler))
        .with_udp_addrs(vec!["0.0.0.0:0"])
        .build()
        .await
        .map_err(|e| e.to_string())?;

    Ok(Arc::new(pc))
}

use crate::utilities::signalizer::Signaling;
use shared::models::websocket_models::{ClientEvent, IceCandidate};
use std::sync::Arc;
use uuid::Uuid;
use webrtc::peer_connection::{
    PeerConnection, PeerConnectionBuilder, PeerConnectionEventHandler, RTCConfigurationBuilder,
    RTCIceServer, RTCPeerConnectionIceEvent,
};

#[derive(Clone)]
pub struct PeerHandler {
    pub peer_id: Uuid,
    pub signaling: Arc<dyn Signaling>,
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

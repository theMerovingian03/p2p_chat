use crate::utilities::signalizer::Signaling;
use shared::models::websocket_models::ClientEvent;
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
// Temporary event handler
#[async_trait::async_trait]
impl PeerConnectionEventHandler for PeerHandler {
    async fn on_ice_candidate(&self, event: RTCPeerConnectionIceEvent) {
        if let Err(e) = self
            .signaling
            // calls WebsocketManager's send_event
            .send(ClientEvent::IceCandidate {
                to: self.peer_id,
                candidate: event.candidate.to_string(),
            })
            .await
        {
            tracing::error!(
                peer_id = %self.peer_id,
                error = %e,
                "Failed to send ICE candidate"
            );
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

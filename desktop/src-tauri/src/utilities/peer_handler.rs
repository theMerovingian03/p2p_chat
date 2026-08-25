use crate::utilities::{
    dc_events::{spawn_data_channel_listener, DcEvent},
    signalizer::Signaling,
};
use shared::models::websocket_models::{ClientEvent, IceCandidate};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
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
    pub signaling: Arc<dyn Signaling>,
    pub event_tx: mpsc::Sender<DcEvent>,
    pub data_channels: Arc<Mutex<HashMap<Uuid, Arc<dyn DataChannel>>>>,
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
            RTCPeerConnectionState::New => info!("New peer connection established!"),
            RTCPeerConnectionState::Connected => info!("Peer connected!"),
            RTCPeerConnectionState::Connecting => info!("Connecting to peer..."),
            RTCPeerConnectionState::Disconnected => info!("Peer disconnected."),
            RTCPeerConnectionState::Failed => error!("Peer connection failed"),
            RTCPeerConnectionState::Closed => info!("Closed peer connection"),
            _ => {}
        }
    }
    async fn on_data_channel(&self, channel: Arc<dyn DataChannel>) {
        info!("Data channel received");

        if channel.label().await != Ok("chat".to_string()) {
            error!("Invalid Data channel label");
            return;
        }

        self.data_channels
            .lock()
            .await
            .insert(self.peer_id, Arc::clone(&channel));

        let event_tx = self.event_tx.clone();
        let peer_id = self.peer_id;

        debug!("Spawning data channel listener for receiver");
        spawn_data_channel_listener(channel, event_tx, peer_id).await;
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

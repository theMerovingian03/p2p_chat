use crate::websocket::WebSocketManager;
use shared::models::websocket_models::ClientEvent;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;
use webrtc::peer_connection::{
    self, PeerConnection, PeerConnectionBuilder, PeerConnectionEventHandler,
    RTCConfigurationBuilder, RTCIceServer, RTCPeerConnectionIceEvent, RTCSessionDescription,
};

#[derive(Clone)]
pub struct PeerHandler {
    peer_id: Uuid,
    websocket: Arc<WebSocketManager>,
}

// Temporary event handler
#[async_trait::async_trait]
impl PeerConnectionEventHandler for PeerHandler {
    async fn on_ice_candidate(&self, event: RTCPeerConnectionIceEvent) {
        println!("New local ice candidate gathered! {}", event.candidate);
    }
}

pub struct WebRtcManager {
    peers: Mutex<HashMap<Uuid, Arc<dyn PeerConnection>>>,
    websocket: Arc<WebSocketManager>,
}

impl WebRtcManager {
    async fn get_or_create_peer_connection(
        &self,
        peer_id: Uuid,
    ) -> Result<Arc<dyn PeerConnection>, String> {
        // First, check whether it already exists.
        {
            let peers = self.peers.lock().await;
            if let Some(peer_connection) = peers.get(&peer_id) {
                return Ok(Arc::clone(peer_connection)); // Clone of Arc pointer for PeerConnection
            }
        }
        // Create the connection without holding the lock
        let peer_handler = PeerHandler {
            peer_id,
            websocket: Arc::clone(&self.websocket),
        };
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
        // Create offer
        let offer = pc.create_offer(None).await.map_err(|e| e.to_string())?;
        // Set sdp
        pc.set_local_description(offer.clone())
            .await
            .map_err(|e| e.to_string())?;
        // Finally send offer to server, which will route to appropriate ID
        self.websocket
            .send_event(ClientEvent::WebRtcOffer {
                to: peer_id,
                sdp: offer.sdp,
            })
            .await?;
        Ok(())
    }

    pub async fn handle_offer(&self, peer_id: Uuid, sdp: String) -> Result<(), String> {
        let pc = self.get_or_create_peer_connection(peer_id).await?;
        // Convert sdp string to RTCSessionDescription
        let offer = RTCSessionDescription::offer(sdp).map_err(|e| e.to_string())?;
        // Set this as the remote descriptor of the connection.
        pc.set_remote_description(offer)
            .await
            .map_err(|e| e.to_string())?;
        let answer = pc.create_answer(None).await.map_err(|e| e.to_string())?;
        pc.set_local_description(answer.clone())
            .await
            .map_err(|e| e.to_string())?;
        self.websocket
            .send_event(ClientEvent::WebRtcAnswer {
                to: peer_id,
                sdp: answer.sdp,
            })
            .await?;
        Ok(())
    }

    pub async fn handle_answer(&self, peer_id: Uuid, sdp: String) -> Result<(), String> {
        let peers = self.peers.lock().await;
        let pc = peers
            .get(&peer_id)
            .ok_or_else(|| "No peer found!".to_string())?;
        let answer = RTCSessionDescription::answer(sdp).map_err(|e| e.to_string())?;
        pc.set_local_description(answer)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
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

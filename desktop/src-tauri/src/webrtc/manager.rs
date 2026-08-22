// PeerConnection, create_peer_connection, etc.
use crate::utilities::peer_handler::*;
use crate::utilities::signalizer::Signaling;
use crate::websocket::manager::WebSocketManager;
use shared::models::websocket_models::ClientEvent;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::debug;
use uuid::Uuid;
use webrtc::peer_connection::{PeerConnection, RTCSessionDescription};
pub struct WebRtcManager {
    peers: Mutex<HashMap<Uuid, Arc<dyn PeerConnection>>>,
    signaling: Arc<dyn Signaling>,
}

#[async_trait::async_trait]
impl Signaling for WebSocketManager {
    async fn send(&self, event: ClientEvent) -> Result<(), String> {
        self.send_event(event).await
    }
}

impl WebRtcManager {
    pub fn new(signaling: Arc<dyn Signaling>) -> Self {
        Self {
            peers: Mutex::new(HashMap::new()),
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
        // // Data channel for media
        // let data_channel = pc
        //     .create_data_channel("chat", None)
        //     .await
        //     .map_err(|e| e.to_string())?;
        // Create offer
        let offer = pc.create_offer(None).await.map_err(|e| e.to_string())?;
        // Set sdp
        pc.set_local_description(offer.clone())
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
        // Set this as the remote descriptor of the connection.
        pc.set_remote_description(offer)
            .await
            .map_err(|e| e.to_string())?;
        let answer = pc.create_answer(None).await.map_err(|e| e.to_string())?;
        pc.set_local_description(answer.clone())
            .await
            .map_err(|e| e.to_string())?;
        // let local_description = pc
        //     .local_description()
        //     .await
        //     .ok_or_else(|| "Local description not available".to_string())?;
        self.signaling
            .send(ClientEvent::WebRtcAnswer {
                to: peer_id,
                // sdp: local_description.sdp,
                sdp: answer.sdp,
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
        if let Some(pc) = self.peers.lock().await.remove(&peer_id) {
            pc.close().await.map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn handle_ice_candidate(&self, from: Uuid, candidate: String) -> Result<(), String> {
        println!("Handling ICE Candicate: {} from {}", candidate, from);
        Ok(())
    }
}

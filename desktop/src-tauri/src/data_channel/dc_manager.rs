use crate::utilities::dc_events::{spawn_data_channel_listener, DcEvent};
use bytes::BytesMut;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info};
use uuid::Uuid;
use webrtc::data_channel::{DataChannel, DataChannelEvent};
pub struct DcManager {
    pub channels: DashMap<Uuid, Arc<dyn DataChannel>>,
    pub event_tx: mpsc::Sender<DcEvent>,
}

impl DcManager {
    pub fn new(event_tx: mpsc::Sender<DcEvent>) -> Self {
        Self {
            channels: DashMap::new(),
            event_tx,
        }
    }

    pub async fn process_events(&self, mut rx: mpsc::Receiver<DcEvent>) {
        while let Some(event) = rx.recv().await {
            self.handle_dc_events(event).await;
        }
    }

    pub async fn add_data_channel(&self, peer_id: Uuid, dc: Arc<dyn DataChannel>) {
        debug!("Inserting data channel");
        self.channels.insert(peer_id, Arc::clone(&dc));
        debug!("Spawning data channel listener for offerer");

        spawn_data_channel_listener(dc, self.event_tx.clone(), peer_id).await;
    }

    pub fn remove_data_channel(&self, peer_id: Uuid) {
        self.channels.remove(&peer_id);
    }

    pub async fn send_message(&self, peer_id: Uuid, message: Vec<u8>) -> Result<(), String> {
        let channel = self
            .channels
            .get(&peer_id)
            .map(|entry| Arc::clone(entry.value()))
            .ok_or_else(|| "No data channel found for peer!".to_string())?;

        channel
            .send(BytesMut::from(message.as_slice()))
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn handle_dc_events(&self, event: DcEvent) {
        let DcEvent { peer_id, event } = event;

        match event {
            DataChannelEvent::OnOpen => {
                info!("Opened DataChannel on peer: {}", peer_id);
            }
            DataChannelEvent::OnClosing => {
                info!("Closing DataChannel on peer: {}", peer_id);
            }
            DataChannelEvent::OnClose => {
                info!("Closed DataChannel on peer: {}", peer_id);
            }
            DataChannelEvent::OnError => {
                error!("Error occured on DataChannel");
            }
            DataChannelEvent::OnMessage(message) => {
                info!(
                    "Message received on DataChannel of length: {}",
                    message.data.len()
                );
            }
            _ => {}
        }
    }
}

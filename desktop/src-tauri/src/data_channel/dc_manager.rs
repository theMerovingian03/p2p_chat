use crate::utilities::dc_events::{spawn_data_channel_listener, DcEvent};
use bytes::BytesMut;
use dashmap::DashMap;
use parking_lot::Mutex;
use serde_json::{json, Value};
use shared::models::dc_models::DataChannelAppEvent as AppEvent;
use std::sync::Arc;
use tauri::Emitter;
use tokio::sync::mpsc;
use tracing::{debug, error, info};
use uuid::Uuid;
use webrtc::data_channel::{DataChannel, DataChannelEvent};
pub struct DcManager {
    pub channels: DashMap<Uuid, Arc<dyn DataChannel>>,
    pub event_tx: mpsc::Sender<DcEvent>,
    pub app_handle: Arc<Mutex<Option<tauri::AppHandle>>>,
}

impl DcManager {
    pub fn new(event_tx: mpsc::Sender<DcEvent>) -> Self {
        Self {
            channels: DashMap::new(),
            event_tx,
            app_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_app_handle(&self, app_handle: tauri::AppHandle) {
        *self.app_handle.lock() = Some(app_handle);
    }

    /// Emit a Tauri event to the frontend
    fn emit_event(&self, event_name: &str, payload: Value) {
        // Avoid taking ownership
        if let Some(app) = self.app_handle.lock().as_ref() {
            if let Err(e) = app.emit(event_name, &payload) {
                error!("Failed to emit event {}: {}", event_name, e);
            }
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
        self.emit_event("dc-event", json!(&AppEvent::PeerDisconnected { peer_id }));
    }

    // pub fn has_channel(&self, peer_id: Uuid) -> bool {
    //     self.channels.contains_key(&peer_id)
    // }

    // pub async fn connected_peers(&self) -> Vec<Uuid> {
    //     let mut peers = Vec::new();
    //     for entry in self.channels.iter() {
    //         if let Ok(state) = entry.value().ready_state().await {
    //             if state == RTCDataChannelState::Open {
    //                 peers.push(*entry.key());
    //             }
    //         }
    //     }
    //     peers
    // }

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
                self.emit_event("dc-event", json!(&AppEvent::PeerConnected { peer_id }));
            }
            DataChannelEvent::OnClosing => {
                info!("Closing DataChannel on peer: {}", peer_id);
            }
            DataChannelEvent::OnClose => {
                info!("Closed DataChannel on peer: {}", peer_id);
                // Remove the data channel from the map. This is idempotent - if already removed,
                // remove_data_channel will simply do nothing.
                self.remove_data_channel(peer_id);
            }
            DataChannelEvent::OnError => {
                error!("Error occured on DataChannel");
            }
            DataChannelEvent::OnMessage(message) => {
                info!(
                    "Message received on DataChannel of length: {}",
                    message.data.len()
                );
                self.emit_event(
                    "dc-event",
                    json!(&AppEvent::MessageReceived {
                        peer_id,
                        message: message.data.to_vec(),
                    }),
                );
            }
            _ => {}
        }
    }

    pub async fn clear(&self) {
        // Collect channels first
        info!("Clearing data channels");
        let channels: Vec<_> = self
            .channels
            .iter()
            .map(|entry| (*entry.key(), Arc::clone(entry.value())))
            .collect();

        // Clear main object channels
        self.channels.clear();

        for (peer_id, channel) in channels.iter() {
            if let Err(e) = channel.close().await {
                error!(
                    peer_id = %peer_id,
                    error = %e,
                    "Failed to close data channel!"
                );
            }
        }
    }
}

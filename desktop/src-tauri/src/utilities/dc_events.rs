// use std::sync::mpsc;
use std::sync::Arc;
use tokio::sync::mpsc;

use tracing::error;
use uuid::Uuid;
use webrtc::data_channel::{DataChannel, DataChannelEvent};

// Data channel event
pub struct DcEvent {
    pub peer_id: Uuid,
    pub event: DataChannelEvent,
}

pub async fn spawn_data_channel_listener(
    channel: Arc<dyn DataChannel>,
    event_tx: mpsc::Sender<DcEvent>,
    peer_id: Uuid,
) {
    tokio::spawn(async move {
        while let Some(event) = channel.poll().await {
            if event_tx.send(DcEvent { peer_id, event }).await.is_err() {
                error!("Failed to send DataChannelEvent!");
                break;
            }
        }
    });
}

// use std::sync::mpsc;
use std::sync::Arc;
use tokio::sync::mpsc;

use tracing::{error, info};
use uuid::Uuid;
use webrtc::data_channel::{DataChannel, DataChannelEvent};

// Data channel event
pub struct DcEvent {
    pub peer_id: Uuid,
    pub event: DataChannelEvent,
}

pub async fn process_events(mut rx: mpsc::Receiver<DcEvent>) {
    while let Some(event) = rx.recv().await {
        handle_dc_events(event).await;
    }
}

async fn handle_dc_events(event: DcEvent) {
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

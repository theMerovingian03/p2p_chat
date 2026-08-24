// use std::sync::mpsc;
use tokio::sync::mpsc;

use tracing::{error, info};
use uuid::Uuid;
use webrtc::data_channel::DataChannelEvent;

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

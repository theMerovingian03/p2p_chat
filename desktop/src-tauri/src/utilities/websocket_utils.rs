use crate::webrtc::manager::WebRtcManager;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use parking_lot::Mutex;
use serde_json::json;
use shared::models::websocket_models::ClientEvent;
use shared::models::websocket_models::ServerEvent;
use std::sync::Arc;
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info};

/// WebSocket connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebSocketStatus {
    Disconnected,
    Connecting,
    Connected,
}

impl WebSocketStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            WebSocketStatus::Disconnected => "disconnected",
            WebSocketStatus::Connecting => "connecting",
            WebSocketStatus::Connected => "connected",
        }
    }
}

/// Task to read messages from the WebSocket server
pub async fn read_websocket_messages(
    mut read: futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
    app_handle: Arc<Mutex<Option<tauri::AppHandle>>>,
    webrtc_manager: Arc<WebRtcManager>,
) {
    while let Some(result) = read.next().await {
        match result {
            Ok(Message::Text(text)) => {
                debug!("Received message: {}", text);

                match serde_json::from_str::<ServerEvent>(&text) {
                    Ok(event) => {
                        match event {
                            // Handle WebRTC related events here itself
                            ServerEvent::WebRtcOffer { from, sdp } => {
                                if let Err(e) = webrtc_manager.handle_offer(from, sdp).await {
                                    error!("Failed to handle WebRTC Offer: {}", e);
                                }
                            }
                            ServerEvent::WebRtcAnswer { from, sdp } => {
                                if let Err(e) = webrtc_manager.handle_answer(from, sdp).await {
                                    error!("Failed to handle WebRTC answer: {}", e);
                                }
                            }
                            ServerEvent::IceCandidate { from, candidate } => {
                                if let Err(e) =
                                    webrtc_manager.handle_ice_candidate(from, candidate).await
                                {
                                    error!("Failed to handle ICE candidate: {}", e);
                                }
                            }
                            ServerEvent::ChatRequestAccepted { from } => {
                                if let Err(e) = webrtc_manager.create_offer(from).await {
                                    error!("Failed to send WebRTC Offer: {}", e);
                                }
                            }
                            // Everything else goes to React
                            event => {
                                if let Some(app) = app_handle.lock().as_ref() {
                                    let payload = serde_json::to_value(&event).unwrap_or_else(
                                        |_| json!({"error": "serialization failed"}),
                                    );

                                    if let Err(e) = app.emit("ws-event", &payload) {
                                        error!("Failed to emit ws-event: {}", e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to deserialize ServerEvent: {}", e);
                    }
                }
            }
            Ok(Message::Binary(_)) => {
                debug!("Received binary message, ignoring");
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket closed by server");
                break;
            }
            Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => {
                debug!("Received ping/pong");
            }
            Ok(_) => {
                debug!("Received other message type");
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    // Connection closed
    info!("WebSocket read task ended");
}

/// Task to write messages to the WebSocket server
pub async fn write_websocket_messages(
    mut write: futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    mut rx: mpsc::Receiver<ClientEvent>,
) {
    while let Some(event) = rx.recv().await {
        match serde_json::to_string(&event) {
            Ok(text) => {
                debug!("Sending event: {}", text);
                if let Err(e) = write.send(Message::Text(text.into())).await {
                    error!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }
            Err(e) => {
                error!("Failed to serialize ClientEvent: {}", e);
            }
        }
    }

    // Channel closed, close the connection
    let _ = write.close().await;
    info!("WebSocket write task ended");
}

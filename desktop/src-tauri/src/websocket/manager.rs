use crate::{utilities::websocket_utils::*, webrtc::manager::WebRtcManager};
use futures_util::stream::StreamExt;
use parking_lot::Mutex;
use serde_json::{json, Value};
use shared::models::websocket_models::ClientEvent;
use std::sync::Arc;
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tracing::{debug, error, info};

// WebSocket manager responsible for connection lifecycle and event handling
// Unbounded causes backpressure
pub struct WebSocketManager {
    status: Arc<Mutex<WebSocketStatus>>,
    sender: Arc<Mutex<Option<mpsc::Sender<ClientEvent>>>>,
    app_handle: Arc<Mutex<Option<tauri::AppHandle>>>,
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(WebSocketStatus::Disconnected)),
            sender: Arc::new(Mutex::new(None)),
            app_handle: Arc::new(Mutex::new(None)),
        }
    }

    /// Get current connection status
    pub fn status(&self) -> WebSocketStatus {
        *self.status.lock()
    }

    /// Set the app handle for emitting events
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

    /// Emit status change event
    fn emit_status_change(&self, status: WebSocketStatus) {
        self.emit_event("ws-status-changed", json!({ "status": status.as_str() }));
    }

    /// Connect to WebSocket server
    pub async fn connect(
        &self,
        ws_url: String,
        ws_token: String,
        webrtc_manager: Arc<WebRtcManager>,
    ) {
        // Drops the mutex guard before continuing
        let old_status = self.status();

        if old_status != WebSocketStatus::Disconnected {
            debug!("WebSocket already in use, disconnecting first");
            self.disconnect().await;
        }
        {
            let mut status = self.status.lock();
            *status = WebSocketStatus::Connecting;
        }

        self.emit_status_change(WebSocketStatus::Connecting);

        // Build connection URL with token
        let url = format!("{}?ws_token={}", ws_url, urlencoding::encode(&ws_token));

        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                info!("WebSocket connected successfully");
                *self.status.lock() = WebSocketStatus::Connected;
                self.emit_status_change(WebSocketStatus::Connected);

                // Split the stream into sender and receiver
                let (write, read) = ws_stream.split();
                let (tx, rx) = mpsc::channel::<ClientEvent>(100);

                // Store sender for sending messages
                *self.sender.lock() = Some(tx);

                // Spawn tasks for handling the connection
                let status = Arc::clone(&self.status);
                let app_handle = Arc::clone(&self.app_handle);

                // Task to read messages from server
                let read_handle = tokio::spawn(read_websocket_messages(
                    read,
                    app_handle.clone(),
                    webrtc_manager,
                ));

                // Task to write messages to server
                tokio::spawn(write_websocket_messages(write, rx));

                // Task to handle connection close detection
                tokio::spawn(async move {
                    // Wait for the read task to complete (connection closed)
                    let _ = read_handle.await;

                    // Connection closed, update status
                    *status.lock() = WebSocketStatus::Disconnected;
                    if let Some(app) = app_handle.lock().as_ref() {
                        let _ = app.emit(
                            "ws-status-changed",
                            &serde_json::json!({ "status": "disconnected" }),
                        );
                    }
                });
            }
            Err(e) => {
                error!("Failed to connect to WebSocket: {}", e);
                *self.status.lock() = WebSocketStatus::Disconnected;
                self.emit_status_change(WebSocketStatus::Disconnected);
                self.emit_event(
                    "ws-error",
                    json!({ "message": format!("Connection failed: {}", e) }),
                );
            }
        }
    }

    /// Send a client event to the server
    pub async fn send_event(&self, event: ClientEvent) -> Result<(), String> {
        if self.status() != WebSocketStatus::Connected {
            return Err("WebSocket not connected".to_string());
        }

        let sender = {
            let guard = self.sender.lock();

            guard
                .clone()
                .ok_or_else(|| "WebSocket sender not available".to_string())?
        };

        sender
            .send(event)
            .await
            .map_err(|e| format!("Failed to send event: {}", e))?;

        Ok(())
    }

    /// Disconnect from WebSocket server
    pub async fn disconnect(&self) {
        debug!("Disconnecting WebSocket");
        let was_connected = {
            let mut status = self.status.lock();
            if *status == WebSocketStatus::Disconnected {
                false
            } else {
                *status = WebSocketStatus::Disconnected;
                true
            }
        };

        if !was_connected {
            return;
        }

        *self.sender.lock() = None;
        self.emit_status_change(WebSocketStatus::Disconnected);
    }
}

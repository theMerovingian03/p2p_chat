use shared::models::websocket_models::ClientEvent;

#[async_trait::async_trait]
pub trait Signaling: Send + Sync {
    async fn send(&self, event: ClientEvent) -> Result<(), String>;
}

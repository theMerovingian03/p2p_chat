use crate::errors::ConnectionError;
use dashmap::DashMap;
use shared::models::websocket_models::ServerEvent;
use tokio::sync::mpsc;
use uuid::Uuid;

type Connection = mpsc::Sender<ServerEvent>;
// type ConnId = Uuid;

#[derive(Clone)]
pub struct ConnectionManager {
    // TODO (Future improvement): Use DashMap<Uuid, DashMap<Uuid, Connection>>
    // For multi-device sign in support
    connections: DashMap<Uuid, Connection>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {
            connections: DashMap::new(),
        }
    }

    pub fn is_online(&self, user_id: &Uuid) -> bool {
        self.connections.contains_key(user_id)
    }

    // pub fn is_online(&self, user_id: &Uuid) -> bool {
    //     self.connections
    //         .get(user_id)
    //         .map(|conns| !conns.is_empty()) // atleast one connection is sufficient
    //         .unwrap_or(false)
    // }

    pub fn connect(&self, user_id: Uuid, sender: mpsc::Sender<ServerEvent>) {
        self.connections.insert(user_id, sender);
    }

    // pub fn connect(&self, user_id: Uuid, conn_id: ConnId, sender: Connection) {
    //     self.connections
    //         .entry(user_id)
    //         .or_default()
    //         .insert(conn_id, sender);
    // }

    pub fn disconnect(&self, user_id: &Uuid) {
        self.connections.remove(user_id);
    }

    // /// Returns true if this was the user's last connection (i.e. they just went offline)
    // pub fn disconnect(&self, user_id: &Uuid, conn_id: &ConnId) -> bool {
    //     if let Some(conns) = self.connections.get(user_id) {
    //         conns.remove(conn_id);
    //         if conns.is_empty() {
    //             drop(conns);
    //             self.connections.remove(user_id);
    //             return true;
    //         }
    //     }
    //     false
    // }

    pub async fn send_to_user(
        &self,
        user_id: &Uuid,
        message: ServerEvent,
    ) -> Result<(), ConnectionError> {
        let connection = self
            .connections
            .get(user_id)
            .ok_or(ConnectionError::NotConnected)?;

        connection.send(message).await?;

        Ok(())
    }

    // pub async fn send_to_user(
    //     &self,
    //     user_id: &Uuid,
    //     message: ServerEvent,
    // ) -> Result<(), ConnectionError> {
    //     let conns = self
    //         .connections
    //         .get(user_id)
    //         .ok_or(ConnectionError::NotConnected)?;
    //     for entry in conns.iter() {
    //         let _ = entry.value().send(message.clone()).await;
    //     }
    //     Ok(())
    // }

    // pub async fn broadcast_to_users(&self, user_ids: &[Uuid], message: ServerEvent) {
    //     let futs = user_ids
    //         .iter()
    //         .map(|id| self.send_to_user(id, message.clone()));
    //     futures::future::join_all(futs).await;
    // }
}

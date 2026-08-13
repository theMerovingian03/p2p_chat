use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use shared::models::websocket_models::{ClientEvent, ServerEvent};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    errors::AppError,
    repositories::friend_repository::are_friends,
    utilities::{app_to_ws_err::app_error_to_ws_error, connection_manager::ConnectionManager},
};

pub async fn handle_socket(
    socket: WebSocket,
    user_id: Uuid,
    connection_manager: Arc<ConnectionManager>,
    db_pool: PgPool,
) {
    // Splits into Sink + Stream
    let (mut socket_tx, mut socket_rx) = socket.split();
    // Internal app logic => ServerEvent
    // Outgoing: Convert ServerEvent to Message::Text(...)
    // Incoming: Parse Message::Text(...) to ClientEvent
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ServerEvent>(100);
    connection_manager.connect(user_id, tx);

    loop {
        // Run branches concurrently
        tokio::select! {
            // Server => Client
            Some(event) = rx.recv() => {
                match serde_json::to_string(&event) {
                    Ok(json) => {
                        if socket_tx.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }

                    Err(error) => {
                        tracing::error!(?error, "Failed to serialize ServerEvent");
                        break;
                    }
                }
            }

            // Client => Server
            Some(result) = socket_rx.next() => {
                match result {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<ClientEvent>(&text) {
                            Ok(event) => {
                                handle_client_event(
                                    user_id,
                                    event,
                                    &connection_manager,
                                    &db_pool
                                ).await;
                            }

                            Err(error) => {
                                tracing::error!(
                                    ?error,
                                    "Failed to deserialize ClientEvent"
                                );
                            }
                        }
                    }

                    Ok(Message::Close(_)) => {
                        break;
                    }

                    Ok(_) => {
                        // Handle binary/ping/pong if needed
                    }

                    Err(error) => {
                        tracing::error!(?error, "WebSocket error");
                        break;
                    }
                }
            }

            else => {
                break;
            }
        }
    }

    connection_manager.disconnect(&user_id);
}

pub async fn handle_client_event(
    user_id: Uuid,
    event: ClientEvent,
    connection_manager: &ConnectionManager,
    db_pool: &PgPool,
) {
    match event {
        ClientEvent::ChatRequestSend { to } => {
            // service_create_chat_request already sends ServerEvent::ChatRequestIncoming
            if let Err(error) =
                service_create_chat_request(user_id, to, db_pool, connection_manager).await
            {
                let event = app_error_to_ws_error(error);
                // TODO: Handle send_to_user errors
                let _ = connection_manager.send_to_user(&user_id, event).await;
            }
        }
        ClientEvent::ChatRequestAccept { from } => {
            let _ = connection_manager
                // TODO: Handle send_to_user errors
                // Sends ServerEvent::ChatRequestAccepted event to the user who originally sent ClientEvent::ChatRequest
                .send_to_user(&from, ServerEvent::ChatRequestAccepted { from: user_id })
                .await;
        }
        _ => {
            let _ = connection_manager
                // TODO: Handle send_to_user errors
                .send_to_user(
                    &user_id,
                    ServerEvent::GenericMessage {
                        message: "Not implemented!".to_string(),
                    },
                )
                .await;
        }
    };
    println!("Hello!");
}

pub async fn service_create_chat_request(
    user_id: Uuid,
    receiver_id: Uuid,
    db: &PgPool,
    connection_manager: &ConnectionManager,
) -> Result<(), AppError> {
    if user_id == receiver_id {
        return Err(AppError::BadRequest(
            "Cannot send chat request to same user!".to_string(),
        ));
    }
    if !connection_manager.is_online(&receiver_id) {
        return Err(AppError::BadRequest(
            "Could not send chat request as user is offline!".to_string(),
        ));
    }

    // Make sure they are friends.
    let are_friends = are_friends(db, user_id, receiver_id).await;
    if !are_friends {
        return Err(AppError::Forbidden);
    }

    connection_manager
        .send_to_user(
            &receiver_id,
            ServerEvent::ChatRequestIncoming { from: user_id },
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(())
}

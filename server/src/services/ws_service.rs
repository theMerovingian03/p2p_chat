use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use shared::models::websocket_models::{ClientEvent, ServerEvent};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

use crate::{
    errors::AppError,
    repositories::friend_repository::get_friend_username,
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
    let conn_id = Uuid::new_v4();
    connection_manager.connect(user_id, tx, conn_id);

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

    connection_manager.disconnect(&user_id, &conn_id);
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
        ClientEvent::WebRtcOffer { to, sdp } => {
            if let Err(error) = connection_manager
                .send_to_user(&to, ServerEvent::WebRtcOffer { from: user_id, sdp })
                .await
            {
                error!("Error occured while sending WebRtcOffer: {}", error);
            }
        }
        ClientEvent::WebRtcAnswer { to, sdp } => {
            if let Err(error) = connection_manager
                .send_to_user(&to, ServerEvent::WebRtcAnswer { from: user_id, sdp })
                .await
            {
                error!("Error occured while sending WebRtcAnswer: {}", error);
            }
        }
        ClientEvent::IceCandidate { to, candidate } => {
            if let Err(error) = connection_manager
                .send_to_user(
                    &to,
                    ServerEvent::IceCandidate {
                        from: user_id,
                        candidate,
                    },
                )
                .await
            {
                error!("Error occured while sending IceCandidate: {}", error);
            }
        }
    };
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

    let username = get_friend_username(db, user_id, receiver_id)
        .await?
        .ok_or(AppError::Forbidden)?;

    connection_manager
        .send_to_user(
            &receiver_id,
            ServerEvent::ChatRequestIncoming {
                from: user_id,
                username,
            },
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(())
}

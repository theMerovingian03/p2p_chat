use crate::errors::AppError;
use crate::services::auth_service::service_verify_ws_token;
use crate::services::ws_service::handle_socket;
use crate::state::AppState;
use axum::extract::Query;
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::response::IntoResponse;
use shared::models::auth_models::WsAuth;
use uuid::Uuid;

pub async fn websocket_handler(
    // Upgrade from HTTP to WebSocket
    ws: WebSocketUpgrade,
    // TODO: Change this to get token from  Sec-Websocket-Protocol
    Query(query): Query<WsAuth>,
    // Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // verify_ws_token(token, jwt_secret);
    let claims = service_verify_ws_token(&query.ws_token, &state.config)?;
    // Extract user_id if claims is successfully parsed.
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;
    let db_pool = state.db_pool.clone();
    // Move the actual received socket (upgrade) to our handle_socket function, since it needs to take ownership
    Ok(ws.on_upgrade(move |socket| {
        handle_socket(socket, user_id, state.connection_manager.clone(), db_pool)
    }))
}

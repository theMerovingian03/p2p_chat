use crate::errors::AppError;
use shared::models::websocket_models::{ServerEvent, WsErrorCode};

pub fn app_error_to_ws_error(error: AppError) -> ServerEvent {
    match error {
        AppError::NotFound(_) => ServerEvent::Error {
            code: WsErrorCode::UserNotFound,
            message: "User not found.".into(),
        },

        AppError::Forbidden => ServerEvent::Error {
            code: WsErrorCode::NotFriends,
            message: "You are not allowed to perform this action.".into(),
        },

        AppError::BadRequest(message) => ServerEvent::Error {
            code: WsErrorCode::InvalidRequest,
            message,
        },

        AppError::Unauthorized => ServerEvent::Error {
            code: WsErrorCode::Unauthorized,
            message: "Unauthorized.".into(),
        },

        // Don't expose internal error details to the client.
        _ => ServerEvent::Error {
            code: WsErrorCode::InvalidRequest,
            message: "An internal server error occurred.".into(),
        },
    }
}

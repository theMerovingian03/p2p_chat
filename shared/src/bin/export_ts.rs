use shared::models::auth_models::{
    AuthResponse, LoginRequest, RefreshSessionRequest, RefreshSessionResponse, RegisterRequest,
    UserDto, WsAuth,
};
use shared::models::dc_models::DataChannelAppEvent;
use shared::models::friend_models::*;
use shared::models::user_models::{UserSearchModel, UserSearchRequestModel};
use shared::models::websocket_models::{ClientEvent, ServerEvent};
use specta::Types;
use specta_typescript::Typescript;

pub fn main() {
    let types = Types::default()
        .register::<AuthResponse>()
        .register::<LoginRequest>()
        .register::<RegisterRequest>()
        .register::<RefreshSessionRequest>()
        .register::<RefreshSessionResponse>()
        .register::<UserSearchModel>()
        .register::<CreateFriendReqRequest>()
        .register::<AcceptReqRequest>()
        .register::<DeleteReqRequest>()
        .register::<FriendRequestRowDto>()
        .register::<FriendRowDto>()
        .register::<ServerEvent>()
        .register::<ClientEvent>()
        .register::<UserSearchRequestModel>()
        .register::<DataChannelAppEvent>()
        .register::<UserDto>()
        .register::<WsAuth>();

    Typescript::default()
        .export_to(
            "./desktop/src/generated/bindings.ts",
            &types,
            specta_serde::Format,
        )
        .unwrap();
}

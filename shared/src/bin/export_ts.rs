use shared::models::auth_models::{
    AuthResponse, LoginRequest, RefreshSessionRequest, RefreshSessionResponse, RegisterRequest,
    UserDto,
};
use shared::models::friend_models::{AcceptReqRequest, CreateFriendReqRequest};
use shared::models::user_models::{UserSearchModel, UserSearchRequestModel};
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
        .register::<UserSearchRequestModel>()
        .register::<UserDto>();

    Typescript::default()
        .export_to(
            "./desktop/src/generated/bindings.ts",
            &types,
            specta_serde::Format,
        )
        .unwrap();
}

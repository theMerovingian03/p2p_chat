use shared::models::auth_models::{
    AuthResponse, LoginRequest, RefreshSessionRequest, RefreshSessionResponse, RegisterRequest,
    UserDto,
};
use specta::Types;
use specta_typescript::Typescript;

pub fn main() {
    let types = Types::default()
        .register::<AuthResponse>()
        .register::<LoginRequest>()
        .register::<RegisterRequest>()
        .register::<RefreshSessionRequest>()
        .register::<RefreshSessionResponse>()
        .register::<UserDto>();

    Typescript::default()
        .export_to(
            "./desktop/src/generated/bindings.ts",
            &types,
            specta_serde::Format,
        )
        .unwrap();
}

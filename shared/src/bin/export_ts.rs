use shared::models::auth_models::{AuthResponse, LoginRequest, RegisterRequest, UserDto};
use specta::Types;
use specta_typescript::Typescript;

pub fn main() {
    // println!("Running shared/bin/export_ts.rs");
    let types = Types::default()
        .register::<AuthResponse>()
        .register::<LoginRequest>()
        .register::<RegisterRequest>()
        .register::<UserDto>();

    Typescript::default()
        .export_to(
            "./desktop/src/generated/bindings.ts",
            &types,
            specta_serde::Format,
        )
        .unwrap();
    // println!("Successful! /bin/export_ts.rs");
}

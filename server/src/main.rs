mod auth;
mod config;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod state;

use axum::{
    Router, middleware as axum_middleware,
    routing::{get, post},
};
use config::Config;
use routes::auth_routes::*;
use routes::user_routes::*;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::middleware::auth::auth_middleware;

#[tokio::main]
async fn main() {
    // Initialize logging subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Config
    let conf = Config::from_env().expect("Failed to load configuration");
    // Database Pool
    let pool = PgPoolOptions::new()
        .connect(&conf.database_url)
        .await
        .expect("Failed to connect to Database");
    // State
    let state = AppState {
        config: conf,
        db_pool: pool,
    };
    // Register auth routes
    let auth_routes = Router::new()
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user));

    // TODO: Register protected routes
    let protected_routes =
        Router::new()
            .route("/me", get(me))
            .layer(axum_middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ));
    // Main app
    let app = Router::new()
        .merge(auth_routes)
        .merge(protected_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Bind listener to host:port
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // Serve app on listener
    axum::serve(listener, app).await.unwrap();
}

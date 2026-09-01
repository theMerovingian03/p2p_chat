mod auth;
mod config;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod state;
mod utilities;

use crate::services::user_service::guest_cleanup_task;
use crate::utilities::connection_manager::ConnectionManager;
use crate::{
    middleware::auth::auth_middleware,
    routes::{friend_routes::*, ws_routes::websocket_handler},
};
use axum::routing::delete;
use axum::{
    Router,
    http::{
        Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    middleware as axum_middleware,
    routing::{get, post},
};
use config::Config;
use routes::auth_routes::*;
use routes::user_routes::*;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    // Initialize logging subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("server=debug,tokio_tungstenite=off"))
        .init();
    // Config
    let conf = Config::from_env().expect("Failed to load configuration");
    // Seperate here to avoid move errors
    let client_url = conf.client_url.clone();
    let client_url_prod = conf.client_url_prod.clone();
    // Database Pool
    let pool = PgPoolOptions::new()
        .connect(&conf.database_url)
        .await
        .expect("Failed to connect to Database");
    // Websocket Connection manager
    let connection_manager = Arc::new(ConnectionManager::new());
    // Cleanup Task
    tokio::spawn(guest_cleanup_task(pool.clone()));
    // State
    let state = AppState {
        config: conf,
        db_pool: pool,
        connection_manager,
    };

    let public_routes = Router::new().route("/health", get(health));

    // Register auth routes
    let auth_routes = Router::new()
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .route("/auth/guest", post(create_guest_user))
        .route("/auth/refresh", post(refresh_session))
        .route("/ws", get(websocket_handler));

    // TODO: Register protected routes
    let protected_routes = Router::new()
        // /user routes
        .route("/user/me", get(me))
        .route("/user/search", get(search_user))
        // friend/ routes
        .route("/friend", get(get_friends))
        .route("/friend/create_request", post(create_friend_request))
        .route("/friend/accept_request", post(accept_friend_request))
        .route("/friend/delete_request", delete(delete_friend_request))
        .route("/friend/sent", get(get_sent_friend_requests))
        .route("/friend/received", get(get_received_friend_requests))
        // Websocket for chat requests
        // Auth WS needs to be protected since it should allow only authenticated users to get a ws token
        .route("/auth/ws", post(get_ws_token))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));
    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin([
            client_url.parse().unwrap(),
            client_url_prod.parse().unwrap(),
        ])
        // .allow_origin(HeaderValue::from_str(&client_url).unwrap())
        .allow_methods([Method::GET, Method::POST, Method::GET, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);
    // Main app
    // TODO: Add tracing debug logs
    let app = Router::new()
        .merge(public_routes)
        .merge(auth_routes)
        .merge(protected_routes)
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Bind listener to host:port
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    // Serve app on listener
    axum::serve(listener, app).await.unwrap();
}

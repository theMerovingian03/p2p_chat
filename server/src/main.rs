mod auth;
mod config;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod state;

use crate::middleware::auth::auth_middleware;
use crate::services::user_service::guest_cleanup_task;
use axum::{
    Router,
    http::{
        HeaderValue, Method,
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
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    // Initialize logging subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Config
    let conf = Config::from_env().expect("Failed to load configuration");
    // Seperate here to avoid move errors
    let client_url = conf.client_url.clone();
    // Database Pool
    let pool = PgPoolOptions::new()
        .connect(&conf.database_url)
        .await
        .expect("Failed to connect to Database");
    // Cleanup Task
    tokio::spawn(guest_cleanup_task(pool.clone()));
    // State
    let state = AppState {
        config: conf,
        db_pool: pool,
    };

    let public_routes = Router::new().route("/health", get(health));

    // Register auth routes
    let auth_routes = Router::new()
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .route("/auth/guest", post(create_guest_user))
        .route("/auth/refresh", post(refresh_session));

    // TODO: Register protected routes
    let protected_routes =
        Router::new()
            .route("/me", get(me))
            .layer(axum_middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ));
    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_str(&client_url).unwrap())
        .allow_methods([Method::GET, Method::POST, Method::GET, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);
    // Main app
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

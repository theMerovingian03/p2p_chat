use crate::config::Config;
use crate::utilities::connection_manager::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub connection_manager: Arc<ConnectionManager>,
    pub config: Config,
}

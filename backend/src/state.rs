use crate::config::Config;
use sqlx::PgPool;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    pub db: Option<PgPool>,
    pub config: Arc<Config>,
}

impl AppState {
    #[allow(dead_code)]
    pub fn new(db: Option<PgPool>, config: Config) -> Self {
        AppState {
            db,
            config: Arc::new(config),
        }
    }
}

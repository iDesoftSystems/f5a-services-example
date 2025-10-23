use std::sync::Arc;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppContext {
    pub conn: Arc<DatabaseConnection>,
}

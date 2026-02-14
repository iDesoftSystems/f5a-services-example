use std::sync::Arc;

use f5a_services::{routes, shared::context::AppContext};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

pub struct TestContext {
    pub db: Arc<DatabaseConnection>,
}

impl TestContext {
    pub async fn new() -> Self {
        let db = Database::connect("sqlite::memory:").await.unwrap();

        Self { db: Arc::new(db) }
    }

    pub async fn setup_schema(&self) -> &Self {
        let db_schema = sea_orm::Schema::new(self.db.get_database_backend());

        let stmt = db_schema.create_table_from_entity(schemas::user::Entity);

        self.db
            .execute(self.db.get_database_backend().build(&stmt))
            .await
            .unwrap();

        self
    }

    pub fn configure(&self) -> axum::Router {
        routes::router().with_state(AppContext {
            conn: Arc::clone(&self.db),
        })
    }
}

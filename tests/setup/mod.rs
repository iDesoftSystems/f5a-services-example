use f5a_services::{context::AppContext, routes};
use sea_orm::{ConnectionTrait, Database, DbConn};

pub async fn configure() -> axum::Router {
    let db = Database::connect("sqlite::memory:").await.unwrap();

    setup_db_schema(&db).await;

    routes::router().with_state(AppContext { conn: db })
}

async fn setup_db_schema(db: &DbConn) {
    let db_schema = sea_orm::Schema::new(db.get_database_backend());

    let stmt = db_schema.create_table_from_entity(schemas::user::Entity);

    db.execute(db.get_database_backend().build(&stmt))
        .await
        .unwrap();
}

use axum::http::HeaderName;
use f5a_services::{context::AppContext, handlers};
use sea_orm::Database;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in the environment");
    let conn = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind to {}: {}", addr, e));

    let service_layers = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )));

    let app_ctx = AppContext { conn };
    let router = axum::Router::new()
        .route("/", axum::routing::get(handlers::root_handler))
        .route(
            "/users",
            axum::routing::get(handlers::read_users).post(handlers::create_user),
        )
        .route(
            "/users/{user_id}",
            axum::routing::get(handlers::read_user)
                .put(handlers::update_user)
                .patch(handlers::partial_update_user)
                .delete(handlers::delete_user),
        )
        .with_state(app_ctx)
        .layer(service_layers);

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .unwrap_or_else(|e| panic!("failed to start server: {}", e));
}

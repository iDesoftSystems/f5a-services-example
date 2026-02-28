use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderName, HeaderValue};
use f5a_services::routes;
use f5a_services::shared::context::AppContext;
use f5a_services::shared::trace::RequestIdSpan;
use sea_orm::Database;
use std::sync::Arc;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

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
        .layer(TraceLayer::new_for_http().make_span_with(RequestIdSpan))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )));

    let app_ctx = AppContext {
        conn: Arc::new(conn),
    };

    let router = routes::router()
        .with_state(app_ctx)
        .layer(service_layers)
        .layer(
            CorsLayer::new()
                .allow_methods(cors::Any)
                .allow_origin("http://localhost:8081".parse::<HeaderValue>().unwrap())
                .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
        );

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .unwrap_or_else(|e| panic!("failed to start server: {}", e));
}

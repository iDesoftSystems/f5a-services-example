use f5a_services::handlers;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind to {}: {}", addr, e));

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
        );

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .unwrap_or_else(|e| panic!("failed to start server: {}", e));
}

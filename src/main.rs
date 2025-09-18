use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind to {}: {}", addr, e));

    let router = axum::Router::new();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .unwrap_or_else(|e| panic!("failed to start server: {}", e));
}

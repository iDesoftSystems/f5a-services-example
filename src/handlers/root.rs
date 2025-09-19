use axum::response::IntoResponse;

pub async fn root_handler() -> impl IntoResponse {
    "iDesoft f5a services"
}

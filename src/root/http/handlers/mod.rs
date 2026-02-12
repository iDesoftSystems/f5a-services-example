use axum::response::IntoResponse;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body=String),
    )
)]
pub async fn root_handler() -> impl IntoResponse {
    "iDesoft f5a services"
}

use axum::extract::Path;
use axum::response::IntoResponse;

pub async fn create_user() -> impl IntoResponse {}
pub async fn read_users() -> impl IntoResponse {}

pub async fn read_user(Path(user_id): Path<u32>) -> impl IntoResponse {
    format!("fetching user with id: {}", user_id)
}

pub async fn update_user() -> impl IntoResponse {}
pub async fn delete_user() -> impl IntoResponse {}
pub async fn partial_update_user() -> impl IntoResponse {}

use crate::om::{CreateUserParams, UserCreated};
use crate::pagination::Pagination;
use axum::Json;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;

pub async fn create_user(Json(payload): Json<CreateUserParams>) -> impl IntoResponse {
    println!("received payload: {:?}", payload);

    Json(UserCreated { id: 1 })
}

pub async fn read_users(Query(pagination): Query<Pagination>) -> impl IntoResponse {
    format!(
        "fetching users on page: {} with page size: {}",
        pagination.page, pagination.page_size
    )
}

pub async fn read_user(Path(user_id): Path<u32>) -> impl IntoResponse {
    format!("fetching user with id: {}", user_id)
}

pub async fn update_user() -> impl IntoResponse {}
pub async fn delete_user() -> impl IntoResponse {}
pub async fn partial_update_user() -> impl IntoResponse {}

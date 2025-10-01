use crate::context::AppContext;
use crate::error::ApiError;
use crate::om::{CreateUserParams, UserCreated};
use crate::pagination::Pagination;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, TryIntoModel};

pub async fn create_user(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateUserParams>,
) -> Result<Json<UserCreated>, ApiError> {
    println!("received payload: {:?}", payload);

    let current_user_id = 1;
    let user_model = schemas::user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(payload.username),
        password: ActiveValue::Set("password".into()),
        disabled: ActiveValue::Set(true.into()),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        creator_id: ActiveValue::Set(current_user_id),
    }
    .save(&ctx.conn)
    .await?
    .try_into_model()?;

    println!("created user with id: {:?}", user_model.id);
    Ok(Json(UserCreated { id: user_model.id }))
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

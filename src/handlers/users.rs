use crate::context::AppContext;
use crate::error::ApiError;
use crate::om::{
    CreateUserParams, PartialUserParams, UpdateUserParams, UserCreated, UserDetail, UserPage,
};
use crate::pagination::Pagination;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::NoContent;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait,
    TryIntoModel,
};
use validator::Validate;

pub async fn create_user(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateUserParams>,
) -> Result<Json<UserCreated>, ApiError> {
    println!("received payload: {:?}", payload);

    payload.validate()?;

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

pub async fn read_users(
    State(ctx): State<AppContext>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<UserPage>>, ApiError> {
    let users = schemas::user::Entity::find()
        .paginate(&ctx.conn, pagination.page_size)
        .fetch_page(pagination.page)
        .await?;

    let users_page = users
        .into_iter()
        .map(|model| UserPage {
            id: model.id,
            username: model.username,
            disabled: model.disabled.is_positive(),
            created_at: model.created_at.and_utc(),
        })
        .collect();

    Ok(Json(users_page))
}

pub async fn read_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserDetail>, ApiError> {
    let user_model = schemas::user::Entity::find_by_id(user_id)
        .one(&ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(UserDetail {
        id: user_model.id,
        username: user_model.username,
        disabled: user_model.disabled.is_positive(),
        created_at: user_model.created_at.and_utc(),
    }))
}

pub async fn update_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserParams>,
) -> Result<NoContent, ApiError> {
    let user_model = schemas::user::Entity::find_by_id(user_id)
        .one(&ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    let mut user_am = user_model.into_active_model();
    user_am.username = ActiveValue::Set(payload.username);
    user_am.disabled = ActiveValue::Set(payload.disabled.into());

    user_am.update(&ctx.conn).await?;

    Ok(NoContent)
}

pub async fn delete_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<NoContent, ApiError> {
    let user_model = schemas::user::Entity::find_by_id(user_id)
        .one(&ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    user_model.delete(&ctx.conn).await?;

    Ok(NoContent)
}

pub async fn partial_update_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(payload): Json<PartialUserParams>,
) -> Result<NoContent, ApiError> {
    let user_model = schemas::user::Entity::find_by_id(user_id)
        .one(&ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    let mut user_am = user_model.into_active_model();
    if let Some(username) = payload.username {
        user_am.username = ActiveValue::Set(username);
    }

    if let Some(disabled) = payload.disabled {
        user_am.disabled = ActiveValue::Set(disabled.into());
    }

    user_am.update(&ctx.conn).await?;

    Ok(NoContent)
}

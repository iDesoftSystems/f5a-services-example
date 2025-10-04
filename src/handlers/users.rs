use crate::commands::{self, PartialUpdateUserCommand};
use crate::context::AppContext;
use crate::error::ApiError;
use crate::om::{
    CreateUserParams, PartialUserParams, UpdateUserParams, UserCreated, UserDetail, UserPage,
};
use crate::pagination::Pagination;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::NoContent;
use sea_orm::{EntityTrait, PaginatorTrait};

pub async fn create_user(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateUserParams>,
) -> Result<Json<UserCreated>, ApiError> {
    println!("received payload: {:?}", payload);

    let saved_id = commands::CreateUserCommand {
        name: payload.name,
        username: payload.username,
    }
    .execute(&ctx.conn)
    .await?;

    Ok(Json(UserCreated { id: saved_id }))
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
    commands::UpdateUserCommand {
        user_id,
        username: payload.username,
        disabled: payload.disabled,
    }
    .execute(&ctx.conn)
    .await?;

    Ok(NoContent)
}

pub async fn delete_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<NoContent, ApiError> {
    commands::DeleteUserCommand { user_id }
        .execute(&ctx.conn)
        .await?;

    Ok(NoContent)
}

pub async fn partial_update_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
    Json(payload): Json<PartialUserParams>,
) -> Result<NoContent, ApiError> {
    PartialUpdateUserCommand {
        user_id,
        username: payload.username,
        disabled: payload.disabled,
    }
    .execute(&ctx.conn)
    .await?;

    Ok(NoContent)
}

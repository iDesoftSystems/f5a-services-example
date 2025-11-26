use crate::commands::{self, PartialUpdateUserCommand};
use crate::context::AppContext;
use crate::error::ApiError;
use crate::om::{
    CreateUserParams, PartialUserParams, UpdateUserParams, UserCreated, UserDetail, UserPage,
};
use crate::pagination::Pagination;
use crate::queries;
use crate::response::ProblemDetails;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::NoContent;

#[utoipa::path(
    post,
    path = "/api/users", 
    tag = "user", 
    responses(
        (status = CREATED, body=UserCreated, description = "User item created successfully"),
        (status = INTERNAL_SERVER_ERROR),
        (status = BAD_REQUEST, body = ProblemDetails),
    )
)]
#[tracing::instrument(skip(ctx))]
pub async fn create_user(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateUserParams>,
) -> Result<Json<UserCreated>, ApiError> {
    let saved_id = commands::CreateUserCommand {
        username: payload.username,
    }
    .execute(ctx.conn.as_ref())
    .await?;

    Ok(Json(UserCreated { id: saved_id }))
}

#[utoipa::path(
    get, 
    path = "/api/users", 
    tag = "user",
    params(
        Pagination
    ),
    responses(
        (status=OK, body = [UserPage], description = "Get all users paginated"),
        (status = INTERNAL_SERVER_ERROR),
    )
)]
pub async fn read_users(
    State(ctx): State<AppContext>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<UserPage>>, ApiError> {
    let users = queries::find_all_users_paginated(ctx.conn.as_ref(), &pagination).await?;

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

#[utoipa::path(
    get, 
    path = "/api/users/{user_id}", 
    tag = "user", 
    params(
        ("user_id"=i32, Path, description = "User item unique id")
    ),
    responses(
        (status = OK, body = UserDetail),
        (status = NOT_FOUND, description = "User was not found"),
        (status = INTERNAL_SERVER_ERROR),
    )
)]
pub async fn read_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserDetail>, ApiError> {
    let user_model = queries::find_user_by_id(ctx.conn.as_ref(), user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(UserDetail {
        id: user_model.id,
        username: user_model.username,
        disabled: user_model.disabled.is_positive(),
        created_at: user_model.created_at.and_utc(),
    }))
}

#[utoipa::path(
    put,
    path = "/api/users/{user_id}",
    tag = "user",
    params(
        ("user_id"=i32, Path, description = "User item unique id")
    ),
    responses(
        (status = NO_CONTENT),
        (status = INTERNAL_SERVER_ERROR),
        (status = BAD_REQUEST, body = ProblemDetails),
    )
)]
#[tracing::instrument(skip(ctx))]
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
    .execute(ctx.conn.as_ref())
    .await?;

    Ok(NoContent)
}

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}",
    tag = "user",
    params(
        ("user_id"=i32, Path, description = "User item unique id")
    ),
    responses(
        (status = NO_CONTENT),
        (status = INTERNAL_SERVER_ERROR),
    )
)]
#[tracing::instrument(skip(ctx))]
pub async fn delete_user(
    State(ctx): State<AppContext>,
    Path(user_id): Path<i32>,
) -> Result<NoContent, ApiError> {
    commands::DeleteUserCommand { user_id }
        .execute(ctx.conn.as_ref())
        .await?;

    Ok(NoContent)
}

#[utoipa::path(
    patch,
    path = "/api/users/{user_id}",
    tag = "user",
    params(
        ("user_id"=i32, Path, description = "User item unique id")
    ),
    responses(
        (status = NO_CONTENT),
        (status = INTERNAL_SERVER_ERROR),
        (status = BAD_REQUEST, body = ProblemDetails),
    )
)]
#[tracing::instrument(skip(ctx))]
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
    .execute(ctx.conn.as_ref())
    .await?;

    Ok(NoContent)
}

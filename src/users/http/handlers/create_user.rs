use crate::context::AppContext;
use crate::error::ApiError;
use crate::response::ProblemDetails;
use crate::users::application::commands::{self, CreateUserCommandHandler};
use crate::users::http::om::{CreateUserParams, UserCreated};
use axum::Json;
use axum::extract::State;
use std::sync::Arc;

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
    let command = commands::CreateUserCommand {
        name: payload.name,
        username: payload.username,
        email: payload.email,
        website: payload.website,
        age: payload.age,
        password: payload.password,
        confirm_password: payload.confirm_password,
    };

    let saved_id = CreateUserCommandHandler {
        conn: Arc::clone(&ctx.conn),
    }
    .handle(command)
    .await?;

    Ok(Json(UserCreated { id: saved_id }))
}

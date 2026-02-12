use crate::context::AppContext;
use crate::error::ApiError;
use crate::response::ProblemDetails;
use crate::users::application::commands;
use crate::users::http::om::{CreateUserParams, UserCreated};
use axum::Json;
use axum::extract::State;

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
        name: payload.name,
        username: payload.username,
        email: payload.email,
        website: payload.website,
        age: payload.age,
        password: payload.password,
        confirm_password: payload.confirm_password,
    }
    .execute(ctx.conn.as_ref())
    .await?;

    Ok(Json(UserCreated { id: saved_id }))
}

use crate::context::AppContext;
use crate::error::ApiError;
use crate::response::ProblemDetails;
use crate::users::application::commands;
use crate::users::http::om::UpdateUserParams;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::NoContent;

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

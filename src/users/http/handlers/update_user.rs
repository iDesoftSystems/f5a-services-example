use crate::context::AppContext;
use crate::error::ApiError;
use crate::response::ProblemDetails;
use crate::users::application::commands;
use crate::users::http::om::UpdateUserParams;
use crate::users::persistence::uow::UnitOfWorkFactory;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::NoContent;
use std::sync::Arc;

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
    let command = commands::UpdateUserCommand {
        user_id,
        username: payload.username,
        disabled: payload.disabled,
    };

    commands::UpdateUserCommandHandler {
        uow_factory: UnitOfWorkFactory::new(Arc::clone(&ctx.conn)),
    }
    .handle(command)
    .await?;

    Ok(NoContent)
}

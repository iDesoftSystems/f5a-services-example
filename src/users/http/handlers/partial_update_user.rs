use crate::context::AppContext;
use crate::error::ApiError;
use crate::response::ProblemDetails;
use crate::users::application::commands;
use crate::users::application::commands::PartialUpdateUserCommand;
use crate::users::http::om::PartialUserParams;
use crate::users::persistence::uow::UnitOfWorkFactory;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::NoContent;
use std::sync::Arc;

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
    let command = PartialUpdateUserCommand {
        user_id,
        username: payload.username,
        disabled: payload.disabled,
    };

    commands::PartialUpdateUserCommandHandler {
        uow_factory: UnitOfWorkFactory::new(Arc::clone(&ctx.conn)),
    }
    .handle(command)
    .await?;

    Ok(NoContent)
}

use crate::context::AppContext;
use crate::error::ApiError;
use crate::users::application::commands;
use crate::users::persistence::uow::UnitOfWorkFactory;
use axum::extract::{Path, State};
use axum::response::NoContent;
use std::sync::Arc;

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
    let command = commands::DeleteUserCommand { user_id };

    commands::DeleteUserCommandHandler {
        uow_factory: UnitOfWorkFactory::new(Arc::clone(&ctx.conn)),
    }
    .handle(command)
    .await?;

    Ok(NoContent)
}

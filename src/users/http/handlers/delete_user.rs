use crate::shared::context::AppContext;
use crate::shared::error::ApiError;
use crate::users::application::commands;
use crate::users::persistence::uow::UnitOfWorkFactory;
use axum::extract::{Path, State};
use axum::response::NoContent;
use std::sync::Arc;

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}",
    tag = "user",
    summary = "Delete a user",
    description = r#"
## Use Case
Permanently deletes a user from the system.
"#,
    params(
        ("user_id"=i32, Path, description = "The unique identifier of the user to delete")
    ),
    responses(
        (status = NO_CONTENT, description = "User deleted successfully"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
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

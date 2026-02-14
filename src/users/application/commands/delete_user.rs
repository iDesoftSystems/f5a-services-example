use crate::shared::error::ApiError;
use crate::users::persistence::uow::{UnitOfWork, UnitOfWorkFactory};

pub struct DeleteUserCommand {
    pub user_id: i32,
}

pub struct DeleteUserCommandHandler {
    pub uow_factory: UnitOfWorkFactory,
}

impl DeleteUserCommandHandler {
    pub async fn handle(self, command: DeleteUserCommand) -> Result<(), ApiError> {
        let uow = self.uow_factory.begin().await?;

        let user_repo = uow.user_repository();

        let user_model = user_repo
            .find_by_id(command.user_id)
            .await?
            .ok_or(ApiError::NotFound)?;

        user_repo.delete(user_model).await?;

        uow.commit().await?;

        tracing::info!(user_id = command.user_id, "deleted user");

        Ok(())
    }
}

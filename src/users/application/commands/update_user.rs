use sea_orm::{ActiveValue, IntoActiveModel};
use validator::Validate;

use crate::error::ApiError;
use crate::users::persistence::uow::{UnitOfWork, UnitOfWorkFactory};

#[derive(Validate)]
pub struct UpdateUserCommand {
    pub user_id: i32,
    #[validate(length(
        min = 3,
        max = 100,
        message = "The username must be between 3 and 100 characters"
    ))]
    pub username: String,
    pub disabled: bool,
}

pub struct UpdateUserCommandHandler {
    pub uow_factory: UnitOfWorkFactory,
}

impl UpdateUserCommandHandler {
    pub async fn handle(self, command: UpdateUserCommand) -> Result<(), ApiError> {
        command.validate()?;

        let uow = self.uow_factory.begin().await?;

        let user_repo = uow.user_repository();

        let user_model = user_repo
            .find_by_id(command.user_id)
            .await?
            .ok_or(ApiError::NotFound)?;

        let mut user_am = user_model.into_active_model();
        user_am.username = ActiveValue::Set(command.username);
        user_am.disabled = ActiveValue::Set(command.disabled.into());

        user_repo.update(user_am).await?;

        uow.commit().await?;

        tracing::info!(user_id = command.user_id, "updated user");

        Ok(())
    }
}

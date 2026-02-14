use sea_orm::{ActiveValue, IntoActiveModel};

use crate::shared::error::ApiError;
use crate::users::persistence::uow::{UnitOfWork, UnitOfWorkFactory};

pub struct PartialUpdateUserCommand {
    pub user_id: i32,
    pub username: Option<String>,
    pub disabled: Option<bool>,
}

pub struct PartialUpdateUserCommandHandler {
    pub uow_factory: UnitOfWorkFactory,
}

impl PartialUpdateUserCommandHandler {
    pub async fn handle(self, command: PartialUpdateUserCommand) -> Result<(), ApiError> {
        let uow = self.uow_factory.begin().await?;

        let user_repo = uow.user_repository();

        let user_model = user_repo
            .find_by_id(command.user_id)
            .await?
            .ok_or(ApiError::NotFound)?;

        let mut user_am = user_model.into_active_model();
        if let Some(username) = command.username {
            user_am.username = ActiveValue::Set(username);
        }

        if let Some(disabled) = command.disabled {
            user_am.disabled = ActiveValue::Set(disabled.into());
        }

        user_repo.update(user_am).await?;

        uow.commit().await?;

        tracing::info!(user_id = command.user_id, "updated user");

        Ok(())
    }
}

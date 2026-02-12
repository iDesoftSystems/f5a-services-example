use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, IntoActiveModel};
use std::sync::Arc;
use validator::Validate;

use crate::error::ApiError;
use crate::users::persistence;

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
    pub conn: Arc<DatabaseConnection>,
}

impl UpdateUserCommandHandler {
    pub async fn handle(self, command: UpdateUserCommand) -> Result<(), ApiError> {
        command.validate()?;

        let user_model = persistence::dao::find_user_by_id(self.conn.as_ref(), command.user_id)
            .await?
            .ok_or(ApiError::NotFound)?;

        let mut user_am = user_model.into_active_model();
        user_am.username = ActiveValue::Set(command.username);
        user_am.disabled = ActiveValue::Set(command.disabled.into());

        user_am.update(self.conn.as_ref()).await?;

        tracing::info!(user_id = command.user_id, "updated user");

        Ok(())
    }
}

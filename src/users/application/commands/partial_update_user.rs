use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, IntoActiveModel};
use std::sync::Arc;

use crate::error::ApiError;
use crate::users::persistence;

pub struct PartialUpdateUserCommand {
    pub user_id: i32,
    pub username: Option<String>,
    pub disabled: Option<bool>,
}

pub struct PartialUpdateUserCommandHandler {
    pub conn: Arc<DatabaseConnection>,
}

impl PartialUpdateUserCommandHandler {
    pub async fn handle(self, command: PartialUpdateUserCommand) -> Result<(), ApiError> {
        let user_model = persistence::dao::find_user_by_id(self.conn.as_ref(), command.user_id)
            .await?
            .ok_or(ApiError::NotFound)?;

        let mut user_am = user_model.into_active_model();
        if let Some(username) = command.username {
            user_am.username = ActiveValue::Set(username);
        }

        if let Some(disabled) = command.disabled {
            user_am.disabled = ActiveValue::Set(disabled.into());
        }

        user_am.update(self.conn.as_ref()).await?;

        tracing::info!(user_id = command.user_id, "updated user");

        Ok(())
    }
}

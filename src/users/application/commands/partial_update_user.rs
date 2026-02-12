use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, IntoActiveModel};

use crate::error::ApiError;
use crate::users::persistence;

pub struct PartialUpdateUserCommand {
    pub user_id: i32,
    pub username: Option<String>,
    pub disabled: Option<bool>,
}

impl PartialUpdateUserCommand {
    pub async fn execute(self, client: &impl ConnectionTrait) -> Result<(), ApiError> {
        let user_model = persistence::dao::find_user_by_id(client, self.user_id)
            .await?
            .ok_or(ApiError::NotFound)?;

        let mut user_am = user_model.into_active_model();
        if let Some(username) = self.username {
            user_am.username = ActiveValue::Set(username);
        }

        if let Some(disabled) = self.disabled {
            user_am.disabled = ActiveValue::Set(disabled.into());
        }

        user_am.update(client).await?;

        tracing::info!(user_id = self.user_id, "updated user");

        Ok(())
    }
}

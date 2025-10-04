use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, EntityTrait, IntoActiveModel};

use crate::error::ApiError;

pub struct PartialUpdateUserCommand {
    pub user_id: i32,
    pub username: Option<String>,
    pub disabled: Option<bool>,
}

impl PartialUpdateUserCommand {
    pub async fn execute(self, client: &impl ConnectionTrait) -> Result<(), ApiError> {
        let user_model = schemas::user::Entity::find_by_id(self.user_id)
            .one(client)
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

        Ok(())
    }
}

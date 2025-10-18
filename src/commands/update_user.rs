use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, IntoActiveModel};

use crate::{error::ApiError, queries};

pub struct UpdateUserCommand {
    pub user_id: i32,
    pub username: String,
    pub disabled: bool,
}

impl UpdateUserCommand {
    pub async fn execute(self, client: &impl ConnectionTrait) -> Result<(), ApiError> {
        let user_model = queries::find_user_by_id(client, self.user_id)
            .await?
            .ok_or(ApiError::NotFound)?;

        let mut user_am = user_model.into_active_model();
        user_am.username = ActiveValue::Set(self.username);
        user_am.disabled = ActiveValue::Set(self.disabled.into());

        user_am.update(client).await?;

        tracing::info!(user_id = self.user_id, "updated user");

        Ok(())
    }
}

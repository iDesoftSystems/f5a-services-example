use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, IntoActiveModel};
use validator::Validate;

use crate::{error::ApiError, queries};

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

impl UpdateUserCommand {
    pub async fn execute(self, client: &impl ConnectionTrait) -> Result<(), ApiError> {
        self.validate()?;

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

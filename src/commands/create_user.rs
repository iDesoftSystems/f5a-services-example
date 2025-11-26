use sea_orm::{
    ActiveModelTrait, ActiveValue, ConnectionTrait, TryIntoModel, sqlx::types::chrono::Utc,
};
use validator::Validate;

use crate::error::ApiError;

#[derive(Validate, Debug)]
pub struct CreateUserCommand {
    #[validate(length(
        min = 3,
        max = 100,
        message = "The username must be between 3 and 100 characters"
    ))]
    pub username: String,
}

impl CreateUserCommand {
    pub async fn execute(self, client: &impl ConnectionTrait) -> Result<i32, ApiError> {
        self.validate()?;

        let current_user_id = 1;
        let user_model = schemas::user::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(self.username),
            password: ActiveValue::Set("password".into()),
            disabled: ActiveValue::Set(true.into()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            creator_id: ActiveValue::Set(current_user_id),
        }
        .save(client)
        .await?
        .try_into_model()?;

        tracing::info!(user_id = user_model.id, "created user");

        Ok(user_model.id)
    }
}

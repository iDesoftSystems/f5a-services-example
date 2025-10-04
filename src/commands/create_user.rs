use sea_orm::{
    ActiveModelTrait, ActiveValue, ConnectionTrait, TryIntoModel, sqlx::types::chrono::Utc,
};
use validator::Validate;

use crate::error::ApiError;

#[derive(Validate, Debug)]
pub struct CreateUserCommand {
    #[validate(length(
        min = 1,
        max = 100,
        message = "The name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "The username must be between 3 and 100 characters"
    ))]
    pub username: String,
}

impl CreateUserCommand {
    pub async fn execute(self, client: &impl ConnectionTrait) -> Result<i32, ApiError> {
        println!("received payload: {:?}", self);

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

        println!("created user with id: {:?}", user_model.id);

        Ok(user_model.id)
    }
}

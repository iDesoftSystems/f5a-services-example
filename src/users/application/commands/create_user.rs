use std::sync::Arc;

use sea_orm::DatabaseConnection;
use sea_orm::{ActiveModelTrait, ActiveValue, sqlx::types::chrono::Utc};
use validator::Validate;

use crate::error::ApiError;
use crate::users::persistence;

#[derive(Validate, Debug)]
pub struct CreateUserCommand {
    #[validate(length(
        min = 3,
        max = 100,
        message = "name must be between 3 and 100 characters long"
    ))]
    pub name: String,

    #[validate(email(message = "Email address is not valid"))]
    pub email: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "Username must be between 3 and 100 characters long"
    ))]
    pub username: String,

    #[validate(url(message = "Website URL is not valid"))]
    pub website: String,

    #[validate(range(min = 18, max = 100, message = "Age must be between 18 and 100"))]
    pub age: u8,

    #[validate(custom(function = "crate::validators::password_strength"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: String,
}

pub struct CreateUserCommandHandler {
    pub conn: Arc<DatabaseConnection>,
}

impl CreateUserCommandHandler {
    pub async fn handle(&self, command: CreateUserCommand) -> Result<i32, ApiError> {
        tracing::info!("creating a new user");

        command.validate()?;

        if persistence::dao::find_user_by_username(self.conn.as_ref(), &command.username)
            .await?
            .is_some()
        {
            return Err(ApiError::UnprocessableEntity(
                "Username already exists".into(),
            ));
        }

        let current_user_id = 1;
        let user_model = schemas::user::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(command.username),
            password: ActiveValue::Set("password".into()),
            disabled: ActiveValue::Set(true.into()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            creator_id: ActiveValue::Set(current_user_id),
        }
        .insert(self.conn.as_ref())
        .await?;

        tracing::info!(user_id = user_model.id, "created user");

        Ok(user_model.id)
    }
}

use sea_orm::ActiveValue;
use sea_orm::sqlx::types::chrono::Utc;
use validator::Validate;

use crate::shared::error::ApiError;
use crate::users::persistence::uow::{UnitOfWork, UnitOfWorkFactory};

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

    #[validate(custom(function = "crate::shared::validators::password_strength"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: String,
}

pub struct CreateUserCommandHandler {
    pub uow_factory: UnitOfWorkFactory,
}

impl CreateUserCommandHandler {
    pub async fn handle(&self, command: CreateUserCommand) -> Result<i32, ApiError> {
        command.validate()?;

        let uow = self.uow_factory.begin().await?;

        let user_repo = uow.user_repository();

        if user_repo
            .find_by_username(&command.username)
            .await?
            .is_some()
        {
            return Err(ApiError::UnprocessableEntity(
                "Username already exists".into(),
            ));
        }

        let current_user_id = 1;
        let user_am = schemas::user::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(command.username),
            password: ActiveValue::Set("password".into()),
            disabled: ActiveValue::Set(true.into()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            creator_id: ActiveValue::Set(current_user_id),
        };

        let user_model = user_repo.insert(user_am).await?;

        uow.commit().await?;

        tracing::info!(user_id = user_model.id, "created user");

        Ok(user_model.id)
    }
}

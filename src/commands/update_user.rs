use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, EntityTrait, IntoActiveModel};

use crate::error::ApiError;

pub struct UpdateUserCommand {
    pub user_id: i32,
    pub username: String,
    pub disabled: bool,
}

impl UpdateUserCommand {
    pub async fn execute(self, client: &impl ConnectionTrait) -> Result<(), ApiError> {
        let user_model = schemas::user::Entity::find_by_id(self.user_id)
            .one(client)
            .await?
            .ok_or(ApiError::NotFound)?;

        let mut user_am = user_model.into_active_model();
        user_am.username = ActiveValue::Set(self.username);
        user_am.disabled = ActiveValue::Set(self.disabled.into());

        user_am.update(client).await?;

        Ok(())
    }
}

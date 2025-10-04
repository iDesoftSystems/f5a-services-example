use sea_orm::{ConnectionTrait, EntityTrait, ModelTrait};

use crate::error::ApiError;

pub struct DeleteUserCommand {
    pub user_id: i32,
}

impl DeleteUserCommand {
    pub async fn execute(self, client: &impl ConnectionTrait) -> Result<(), ApiError> {
        let user_model = schemas::user::Entity::find_by_id(self.user_id)
            .one(client)
            .await?
            .ok_or(ApiError::NotFound)?;

        user_model.delete(client).await?;

        Ok(())
    }
}

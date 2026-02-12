use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use std::sync::Arc;

use crate::error::ApiError;

pub struct DeleteUserCommand {
    pub user_id: i32,
}

pub struct DeleteUserCommandHandler {
    pub conn: Arc<DatabaseConnection>,
}

impl DeleteUserCommandHandler {
    pub async fn handle(self, command: DeleteUserCommand) -> Result<(), ApiError> {
        let user_model = schemas::user::Entity::find_by_id(command.user_id)
            .one(self.conn.as_ref())
            .await?
            .ok_or(ApiError::NotFound)?;

        user_model.delete(self.conn.as_ref()).await?;

        tracing::info!(user_id = command.user_id, "deleted user");

        Ok(())
    }
}

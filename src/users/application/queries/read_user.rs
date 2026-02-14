use crate::shared::error::ApiError;
use crate::users::http::om::UserDetail;
use crate::users::persistence;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct ReadUserQuery {
    pub user_id: i32,
}

pub struct ReadUserQueryHandler {
    pub conn: Arc<DatabaseConnection>,
}

impl ReadUserQueryHandler {
    pub async fn handle(&self, query: ReadUserQuery) -> Result<UserDetail, ApiError> {
        let user_model = persistence::dao::find_user_by_id(self.conn.as_ref(), query.user_id)
            .await?
            .ok_or(ApiError::NotFound)?;

        Ok(UserDetail {
            id: user_model.id,
            username: user_model.username,
            disabled: user_model.disabled.is_positive(),
            created_at: user_model.created_at.and_utc(),
        })
    }
}

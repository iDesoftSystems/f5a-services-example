use crate::error::ApiError;
use crate::pagination::Pagination;
use crate::users::http::om::UserPage;
use crate::users::persistence;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct ReadUsersQuery {
    pub pagination: Pagination,
}

pub struct ReadUsersQueryHandler {
    pub conn: Arc<DatabaseConnection>,
}

impl ReadUsersQueryHandler {
    pub async fn handle(&self, query: ReadUsersQuery) -> Result<Vec<UserPage>, ApiError> {
        let users =
            persistence::dao::find_all_users_paginated(self.conn.as_ref(), &query.pagination)
                .await?;

        let users_page = users
            .into_iter()
            .map(|model| UserPage {
                id: model.id,
                username: model.username,
                disabled: model.disabled.is_positive(),
                created_at: model.created_at.and_utc(),
            })
            .collect::<Vec<UserPage>>();

        Ok(users_page)
    }
}

use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, DeleteResult, ModelTrait};

use crate::shared::pagination::Pagination;
use crate::users::persistence::dao;

pub struct UserRepository<'a, C: ConnectionTrait> {
    pub conn: &'a C,
}

impl<'a, C: ConnectionTrait> UserRepository<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }

    pub async fn find_all_paginated(
        &self,
        pagination: &Pagination,
    ) -> Result<Vec<schemas::user::Model>, DbErr> {
        dao::find_all_users_paginated(self.conn, pagination).await
    }

    pub async fn find_by_id(&self, user_id: i32) -> Result<Option<schemas::user::Model>, DbErr> {
        dao::find_user_by_id(self.conn, user_id).await
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<schemas::user::Model>, DbErr> {
        dao::find_user_by_username(self.conn, username).await
    }

    pub async fn insert(
        &self,
        active_model: schemas::user::ActiveModel,
    ) -> Result<schemas::user::Model, DbErr> {
        active_model.insert(self.conn).await
    }

    pub async fn update(
        &self,
        active_model: schemas::user::ActiveModel,
    ) -> Result<schemas::user::Model, DbErr> {
        active_model.update(self.conn).await
    }

    pub async fn delete(&self, model: schemas::user::Model) -> Result<DeleteResult, DbErr> {
        model.delete(self.conn).await
    }
}

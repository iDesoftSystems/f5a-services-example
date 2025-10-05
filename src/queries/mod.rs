use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait};

use crate::pagination::Pagination;

pub async fn find_all_users_paginated(
    client: &impl ConnectionTrait,
    pagination: &Pagination,
) -> Result<Vec<schemas::user::Model>, DbErr> {
    let users = schemas::user::Entity::find()
        .paginate(client, pagination.page_size)
        .fetch_page(pagination.page)
        .await?;

    Ok(users)
}

pub async fn find_user_by_id(
    client: &impl ConnectionTrait,
    user_id: i32,
) -> Result<Option<schemas::user::Model>, DbErr> {
    schemas::user::Entity::find_by_id(user_id).one(client).await
}

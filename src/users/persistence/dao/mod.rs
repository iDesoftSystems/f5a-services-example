use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::pagination::Pagination;

pub async fn find_all_users_paginated(
    client: &impl ConnectionTrait,
    pagination: &Pagination,
) -> Result<Vec<schemas::user::Model>, DbErr> {
    let users = schemas::user::Entity::find()
        .order_by_desc(schemas::user::Column::Id)
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

pub async fn find_user_by_username(
    client: &impl ConnectionTrait,
    username: &str,
) -> Result<Option<schemas::user::Model>, DbErr> {
    schemas::user::Entity::find()
        .filter(schemas::user::Column::Username.eq(username))
        .one(client)
        .await
}

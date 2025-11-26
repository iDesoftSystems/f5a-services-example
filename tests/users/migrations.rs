use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, DbErr, sqlx::types::chrono::Utc};

pub async fn insert_idesoft_user(
    conn: &impl ConnectionTrait,
) -> Result<schemas::user::ActiveModel, DbErr> {
    let created_at = Utc::now();

    schemas::user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set("idesoftd".into()),
        password: ActiveValue::Set("idesoftd".into()),
        disabled: ActiveValue::Set(true.into()),
        created_at: ActiveValue::Set(created_at.naive_utc()),
        creator_id: ActiveValue::Set(1),
    }
    .save(conn)
    .await
}

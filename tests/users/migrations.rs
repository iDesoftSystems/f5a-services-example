use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, DbErr};

pub async fn insert_idesoft_user(
    conn: &impl ConnectionTrait,
) -> Result<schemas::user::ActiveModel, DbErr> {
    let created_at = NaiveDate::from_ymd_opt(2026, 03, 19)
        .and_then(|date| date.and_hms_opt(10, 10, 10))
        .unwrap();

    schemas::user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set("idesoftd".into()),
        password: ActiveValue::Set("idesoftd".into()),
        disabled: ActiveValue::Set(true.into()),
        created_at: ActiveValue::Set(created_at),
        creator_id: ActiveValue::Set(1),
    }
    .save(conn)
    .await
}

pub async fn insert_blue_bird_user(
    conn: &impl ConnectionTrait,
) -> Result<schemas::user::ActiveModel, DbErr> {
    let created_at = NaiveDate::from_ymd_opt(2026, 03, 19)
        .and_then(|date| date.and_hms_opt(10, 10, 10))
        .unwrap();

    schemas::user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set("bluebird".into()),
        password: ActiveValue::Set("bluebird".into()),
        disabled: ActiveValue::Set(false.into()),
        created_at: ActiveValue::Set(created_at),
        creator_id: ActiveValue::Set(1),
    }
    .save(conn)
    .await
}

pub async fn insert_chameleon_user(
    conn: &impl ConnectionTrait,
) -> Result<schemas::user::Model, DbErr> {
    let created_at = NaiveDate::from_ymd_opt(2025, 12, 19)
        .and_then(|date| date.and_hms_opt(10, 10, 10))
        .unwrap();

    let model = schemas::user::ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::Set("chameleon".into()),
        password: ActiveValue::Set("chameleon".into()),
        disabled: ActiveValue::Set(false.into()),
        created_at: ActiveValue::Set(created_at),
        creator_id: ActiveValue::Set(1),
    }
    .insert(conn)
    .await?;

    Ok(model)
}

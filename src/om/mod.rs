use sea_orm::sqlx::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CreateUserParams {
    pub name: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct UserCreated {
    pub id: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPage {
    pub id: i32,
    pub username: String,
    pub disabled: bool,
    pub created_at: DateTime<Utc>,
}

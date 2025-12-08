use sea_orm::sqlx::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateUserParams {
    pub name: String,
    pub username: String,
    pub email: String,
    pub website: String,
    pub age: u8,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, ToSchema, Deserialize)]
pub struct UserCreated {
    pub id: i32,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserPage {
    pub id: i32,
    pub username: String,
    pub disabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    pub id: i32,
    pub username: String,
    pub disabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateUserParams {
    pub username: String,
    pub disabled: bool,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct PartialUserParams {
    pub username: Option<String>,
    pub disabled: Option<bool>,
}

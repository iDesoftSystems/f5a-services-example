use sea_orm::sqlx::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct CreateUserParams {
    #[validate(length(
        min = 1,
        max = 100,
        message = "The name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "The username must be between 3 and 100 characters"
    ))]
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    pub id: i32,
    pub username: String,
    pub disabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UpdateUserParams {
    pub username: String,
    pub disabled: bool,
}

#[derive(Deserialize)]
pub struct PartialUserParams {
    pub username: Option<String>,
    pub disabled: Option<bool>,
}

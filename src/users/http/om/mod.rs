use sea_orm::sqlx::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
#[schema(description = "Request payload to create a new user")]
pub struct CreateUserParams {
    #[schema(example = "iDesoft Systems")]
    pub name: String,

    #[schema(example = "idesoft")]
    /// Username must be unique across the system
    pub username: String,

    #[schema(example = "idesoft@idesoft.co")]
    /// Email must be unique across the system
    pub email: String,

    #[schema(example = "https://idesoft.co")]
    pub website: String,

    #[schema(example = 25, minimum = 18, maximum = 100)]
    pub age: u8,

    #[schema(example = "SecureP@ssw0rd")]
    /// Password must be at least 8 characters
    pub password: String,

    #[schema(example = "SecureP@ssw0rd")]
    pub confirm_password: String,
}

#[derive(Serialize, ToSchema, Deserialize)]
#[schema(description = "Response after successfully creating a user")]
pub struct UserCreated {
    #[schema(example = 1)]
    pub id: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(description = "User summary for list views")]
pub struct UserPage {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "idesoftd")]
    pub username: String,

    #[schema(example = false)]
    pub disabled: bool,

    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(description = "Detailed user information for single user views")]
pub struct UserDetail {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "idesoft")]
    pub username: String,

    #[schema(example = false)]
    pub disabled: bool,

    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, ToSchema)]
#[schema(description = "Request payload to fully update a user")]
pub struct UpdateUserParams {
    #[schema(example = "johndoe")]
    /// New username
    pub username: String,

    #[schema(example = false)]
    /// Flag to enable or disable the user account.
    /// if True, the user will be prevented from logging in.
    pub disabled: bool,
}

#[derive(Deserialize, Debug, ToSchema)]
#[schema(description = "Request payload to partially update a user")]
pub struct PartialUserParams {
    #[schema(example = "newusername")]
    /// New username
    pub username: Option<String>,

    #[schema(example = true)]
    /// Flag to enable or disable the user account.
    /// if True, the user will be prevented from logging in.
    pub disabled: Option<bool>,
}

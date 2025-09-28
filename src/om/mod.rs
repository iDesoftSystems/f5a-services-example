use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CreateUserParams {
    pub name: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct UserCreated {
    pub id: u32,
}

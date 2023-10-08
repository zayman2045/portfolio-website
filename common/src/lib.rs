use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

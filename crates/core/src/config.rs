use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Credential {
    pub access_key: String,
    pub cookies: String,
    pub uid: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ProgramConfig {
    pub credential: Credential,
    pub rooms: Vec<i64>,
}

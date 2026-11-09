use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: String,
    pub token: String,
    pub topic: String,
}

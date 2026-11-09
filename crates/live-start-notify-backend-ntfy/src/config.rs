use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub url: String,
}

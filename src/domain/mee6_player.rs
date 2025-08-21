use async_trait::async_trait;
use serde::Deserialize;

use crate::Error;
use crate::domain::parser::deserialize_discord_id;

#[derive(Debug, Deserialize)]
pub struct Mee6Player {
    #[serde(rename = "id", deserialize_with = "deserialize_discord_id")]
    pub discord_id: u64,
    pub level: i32,
}

#[async_trait]
pub trait Mee6Repository {
    async fn get_players(&self) -> Result<Vec<Mee6Player>, Error>;
}

pub struct ApiMee6Repository {
    api_url: String,
    token: String,
}

impl ApiMee6Repository {
    pub fn new(api_url: String, token: String) -> Self {
        Self { api_url, token }
    }
}

#[derive(Debug, Deserialize)]
struct Mee6Data {
    players: Vec<Mee6Player>,
}

#[async_trait]
impl Mee6Repository for ApiMee6Repository {
    async fn get_players(&self) -> Result<Vec<Mee6Player>, Error> {
        let client = reqwest::Client::new();
        let data: Mee6Data = client
            .get(&self.api_url)
            .header("Authorization", &self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(data.players)
    }
}

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
    async fn get_players(&self, guild_id: u64) -> Result<Vec<Mee6Player>, Error>;
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
    async fn get_players(&self, guild_id: u64) -> Result<Vec<Mee6Player>, Error> {
        let base_url = format!("{}/plugins/levels/leaderboard/{guild_id}", self.api_url);

        let client = reqwest::Client::new();
        let mut players: Vec<Mee6Player> = vec![];
        let mut page = 0;

        loop {
            let url = format!("{base_url}?page={page}");

            let data: Mee6Data = client
                .get(&url)
                .header("Authorization", &self.token)
                .send()
                .await?
                .json()
                .await?;

            let mut players_level5plus: Vec<Mee6Player> =
                data.players.into_iter().filter(|p| p.level >= 5).collect();

            if players_level5plus.is_empty() {
                break;
            }

            players.append(&mut players_level5plus);

            page += 1;
        }

        Ok(players)
    }
}

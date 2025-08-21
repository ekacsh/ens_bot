use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::Deserialize;

use crate::Error;
use crate::domain::parser::deserialize_discord_id;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    #[serde(rename = "discordId", deserialize_with = "deserialize_discord_id")]
    pub discord_id: u64,
    pub username: String,
    #[serde(rename = "weekGP")]
    pub week_gp: u64,
    #[serde(rename = "oldRank")]
    pub old_rank: String,
    #[serde(rename = "currentRank")]
    pub current_rank: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self) -> Result<Vec<User>, Error>;
    fn invalidate_cache(&self);
}

pub struct GSUserRepository {
    api_url: String,
    cache: Arc<Mutex<Option<Vec<User>>>>,
}

impl GSUserRepository {
    pub fn new(api_url: impl Into<String>) -> Self {
        Self {
            api_url: api_url.into(),
            cache: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl UserRepository for GSUserRepository {
    async fn get_users(&self) -> Result<Vec<User>, Error> {
        if let Some(cached) = self.cache.lock().unwrap().clone() {
            return Ok(cached);
        }

        let users: Vec<User> = reqwest::get(&self.api_url).await?.json().await?;

        *self.cache.lock().unwrap() = Some(users.clone());
        Ok(users)
    }

    fn invalidate_cache(&self) {
        *self.cache.lock().unwrap() = None;
    }
}

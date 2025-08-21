use std::sync::Arc;

use crate::domain::{mee6_player::Mee6Repository, user::UserRepository};

pub mod app;
pub mod domain;

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    mee6_repository: Arc<dyn Mee6Repository + Send + Sync>,
}

impl Data {
    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
        mee6_repository: Arc<dyn Mee6Repository + Send + Sync>,
    ) -> Self {
        Self {
            user_repository,
            mee6_repository,
        }
    }
}

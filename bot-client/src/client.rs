use std::{env, sync::Arc};

use anyhow::{Context, Error, Result};
use poise::{serenity_prelude as serenity, Framework};

use crate::commands::*;

/// The bot client
pub struct Client {
    framework: Arc<Framework<(), Error>>,
}

impl Client {
    pub async fn default() -> Result<Self> {
        // Get the bot token
        let token =
            env::var("BOT_TOKEN").context("Environmental variable 'BOT_TOKEN' not found")?;

        // Create the client using the token
        Client::new(token).await
    }

    pub async fn new(token: String) -> Result<Self> {
        // Build the framework
        let framework = Framework::builder()
            .token(token)
            .intents(serenity::GatewayIntents::empty())
            .options(poise::FrameworkOptions {
                commands: vec![ping::ping()],
                ..Default::default()
            })
            .setup(|ctx, ready, framework| Box::pin(Self::client_ready(ctx, ready, framework)))
            .build()
            .await?;

        Ok(Client { framework })
    }

    async fn client_ready(
        ctx: &serenity::Context,
        _ready: &serenity::Ready,
        framework: &Framework<(), Error>,
    ) -> Result<()> {
        poise::builtins::register_globally(ctx, &framework.options().commands).await?;
        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        self.framework.clone().start().await.map_err(Error::msg)
    }
}

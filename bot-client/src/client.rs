use std::{env, sync::Arc};

use anyhow::{Context, Error, Result};
use migration::{Migrator, MigratorTrait};
use poise::{serenity_prelude as serenity, Framework};
use sea_orm::{Database, DatabaseConnection};

use crate::commands::*;

/// User data
pub struct Data {
    pub db_conn: DatabaseConnection,
}

/// The bot client
pub struct Client {
    framework: Arc<Framework<Data, Error>>,
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
        framework: &Framework<Data, Error>,
    ) -> Result<Data> {
        // Get DB connection parameters
        let db_user = env::var("DB_USER").context("Environmental variable 'DB_USER' not found")?;
        let db_password =
            env::var("DB_PASSWORD").context("Environmental variable 'DB_PASSWORD' not found")?;
        let db_hostname =
            env::var("DB_HOSTNAME").context("Environmental variable 'DB_HOSTNAME' not found")?;
        let db_port = env::var("DB_PORT").context("Environmental variable 'DB_PORT' not found")?;
        let db_name = env::var("DB_NAME").context("Environmental variable 'DB_NAME' not found")?;

        // Connect to the database
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_hostname, db_port, db_name
        );
        let db_conn = Database::connect(&db_url).await?;

        // Migrate database
        Migrator::up(&db_conn, None).await?;

        // Register all commands globally
        poise::builtins::register_globally(ctx, &framework.options().commands).await?;

        // Create data struct
        let data = Data { db_conn };

        Ok(data)
    }

    pub async fn start(&mut self) -> Result<()> {
        self.framework.clone().start().await.map_err(Error::msg)
    }
}

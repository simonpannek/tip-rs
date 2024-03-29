use std::{env, sync::Arc};

use anyhow::{Context, Error, Result};
use migration::{Migrator, MigratorTrait};
use poise::{serenity_prelude as serenity, Event, Framework, FrameworkError};
use sea_orm::{Database, DatabaseConnection};
use tracing::error;

use crate::{commands::*, events::*, utils::create_embed};

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
            .intents(
                serenity::GatewayIntents::non_privileged()
                    | serenity::GatewayIntents::GUILD_MEMBERS,
            )
            .options(poise::FrameworkOptions {
                commands: vec![event::event(), ping::ping(), settings::settings()],
                on_error: |why| {
                    Box::pin(async move {
                        match why {
                            // Send command errors as embeds
                            FrameworkError::Command { ctx, error } => {
                                let embed = create_embed(
                                    "Something went wrong",
                                    &format!("**I couldn't execute the command:** {}", error),
                                );
                                ctx.send(|reply| {
                                    reply.embeds = vec![embed];
                                    reply
                                })
                                .await
                                .unwrap();
                            }
                            // Overwrite on_error behavior when check fails
                            FrameworkError::CommandCheckFailed { ctx, error: None } => {
                                let response = "It looks like you're missing the execution role.";
                                ctx.send(|b| b.content(response).ephemeral(true))
                                    .await
                                    .unwrap();
                            }
                            why => {
                                if let Err(why) = poise::builtins::on_error(why).await {
                                    error!("Failed to send error message: {}", why);
                                }
                            }
                        }
                    })
                },
                event_handler: |_ctx, event, _framework, data| {
                    Box::pin(async move {
                        match event {
                            Event::GuildDelete { incomplete, .. } => {
                                guild_delete::on_guild_delete(data, incomplete).await?;
                            }
                            Event::ChannelDelete { channel } => {
                                guild_channel_delete::on_channel_delete(data, channel).await?;
                            }
                            Event::GuildRoleDelete {
                                guild_id,
                                removed_role_id,
                                ..
                            } => {
                                guild_role_delete::on_role_delete(data, guild_id, removed_role_id)
                                    .await?;
                            }
                            Event::GuildMemberUpdate { new, .. } => {
                                guild_member_update::on_member_update(data, new).await?;
                            }
                            _ => {}
                        }

                        Ok(())
                    })
                },
                require_cache_for_guild_check: true,
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
        self.framework
            .clone()
            .start_autosharded()
            .await
            .map_err(Error::msg)
    }
}

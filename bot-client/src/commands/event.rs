use anyhow::{Error, Result};

use crate::{client::Data, utils::create_embed};

type Context<'a> = poise::Context<'a, Data, Error>;

/// Create an event.
#[poise::command(slash_command, guild_only)]
pub async fn event(ctx: Context<'_>) -> Result<()> {
    let embed = create_embed("Pong!", "I am listening 🐧");
    ctx.send(|reply| {
        reply.embeds = vec![embed];
        reply.ephemeral(true)
    })
    .await?;

    Ok(())
}

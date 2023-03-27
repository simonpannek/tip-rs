use anyhow::{Context as AnyhowContext, Error, Result};
use entity::guild;
use poise::serenity_prelude as serenity;
use sea_orm::EntityTrait;

use crate::{client::Data, utils::create_embed};

type Context<'a> = poise::Context<'a, Data, Error>;

/// Create an event.
#[poise::command(slash_command, guild_only, check = "check_execution")]
pub async fn event(ctx: Context<'_>) -> Result<()> {
    let embed = create_embed("Pong!", "I am listening 🐧");
    ctx.send(|reply| {
        reply.embeds = vec![embed];
        reply.ephemeral(true)
    })
    .await?;

    Ok(())
}

async fn check_execution(ctx: Context<'_>) -> Result<bool> {
    // Unwrap is safe here due to guild_only constraint
    let execution_role = guild::Entity::find_by_id(ctx.guild_id().unwrap().0 as i64)
        .one(&ctx.data().db_conn)
        .await?
        .and_then(|guild| guild.execution_role);

    let result = match execution_role {
        Some(execution_role) => {
            let member = ctx
                .author_member()
                .await
                .context("Failed to fetch member data")?;
            member
                .roles
                .contains(&serenity::RoleId(execution_role as u64))
        }
        None => false,
    };

    Ok(result)
}

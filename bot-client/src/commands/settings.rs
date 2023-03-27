use anyhow::{Error, Result};
use entity::guild;
use poise::{serenity_prelude as serenity, serenity_prelude::Mentionable};
use sea_orm::{sea_query::OnConflict, ActiveValue::Set, EntityTrait};

use crate::{client::Data, utils::create_embed};

type Context<'a> = poise::Context<'a, Data, Error>;

/// The parent settings command.
#[poise::command(
    slash_command,
    guild_only,
    subcommands("channel", "role"),
    required_permissions = "MANAGE_GUILD"
)]
pub async fn settings(_ctx: Context<'_>) -> Result<()> {
    unreachable!();
}

/// Set the default channel for announcements.
#[poise::command(slash_command)]
async fn channel(
    ctx: Context<'_>,
    #[description = "Default channel"]
    #[channel_types("Text")]
    channel: serenity::Channel,
) -> Result<()> {
    guild::Entity::insert(guild::ActiveModel {
        // Unwrap is safe here due to guild_only constraint
        id: Set(ctx.guild_id().unwrap().0 as i64),
        default_channel: Set(Some(channel.id().0 as i64)),
        ..Default::default()
    })
    .on_conflict(
        OnConflict::column(guild::Column::Id)
            .update_column(guild::Column::DefaultChannel)
            .to_owned(),
    )
    .exec(&ctx.data().db_conn)
    .await?;

    let embed = create_embed(
        "Default announcement channel updated",
        &format!(
            "The default announcement channel was set to {}.",
            channel.mention()
        ),
    );
    ctx.send(|reply| {
        reply.embeds = vec![embed];
        reply
    })
    .await?;

    Ok(())
}

/// Set the role that is allowed to create events.
#[poise::command(slash_command)]
async fn role(
    ctx: Context<'_>,
    #[description = "Execution role"] role: serenity::Role,
) -> Result<()> {
    guild::Entity::insert(guild::ActiveModel {
        // Unwrap is safe here due to guild_only constraint
        id: Set(ctx.guild_id().unwrap().0 as i64),
        execution_role: Set(Some(role.id.0 as i64)),
        ..Default::default()
    })
    .on_conflict(
        OnConflict::column(guild::Column::Id)
            .update_column(guild::Column::ExecutionRole)
            .to_owned(),
    )
    .exec(&ctx.data().db_conn)
    .await?;

    let embed = create_embed(
        "Execution role updated",
        &format!("The execution role was set to {}.", role.mention()),
    );
    ctx.send(|reply| {
        reply.embeds = vec![embed];
        reply
    })
    .await?;

    Ok(())
}

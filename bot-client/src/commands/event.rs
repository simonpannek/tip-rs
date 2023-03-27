use anyhow::{bail, Context as AnyhowContext, Error, Result};
use entity::{event, guild, user};
use poise::serenity_prelude as serenity;
use sea_orm::{
    error::DbErr,
    sea_query::OnConflict,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, InsertResult,
};

use crate::{client::Data, utils::create_embed};

type Context<'a> = poise::Context<'a, Data, Error>;

/// Create or overwrite an event.
#[poise::command(slash_command, guild_only, check = "check_execution")]
pub async fn event(
    ctx: Context<'_>,
    #[description = "Name"] name: String,
    #[description = "Description"] description: String,
    #[description = "Event channel"]
    #[channel_types("Text")]
    channel: serenity::Channel,
    #[description = "Event role"] role: serenity::Role,
    #[description = "Announcement channel (if not set, use server default as fallback)"]
    #[channel_types("Text")]
    announcement_channel: Option<serenity::Channel>,
    #[description = "Event id (in case you want to overwrite)"] id: Option<i32>,
) -> Result<()> {
    // Create user
    user::Entity::insert(user::ActiveModel {
        id: Set(ctx.author().id.0 as i64),
    })
    // TODO: Replace the update statement with an update user name
    // statement, as soon as we change the table structure
    .on_conflict(
        OnConflict::column(user::Column::Id)
            .update_column(user::Column::Id)
            .to_owned(),
    )
    .exec(&ctx.data().db_conn)
    .await?;

    // Remove forbidden character from name
    let name = name.replace("`", "");

    // Insert or update event
    let result = event::Entity::insert(event::ActiveModel {
        id: {
            match id {
                Some(id) => Set(id),
                None => NotSet,
            }
        },
        name: Set(name.clone()),
        // Unwrap is safe here due to guild_only constraint
        guild_id: Set(ctx.guild_id().unwrap().0 as i64),
        owner_id: Set(Some(ctx.author().id.0 as i64)),
        description: Set(description),
        channel_id: Set(Some(channel.id().0 as i64)),
        role_id: Set(Some(role.id.0 as i64)),
        announcement_channel_id: Set(announcement_channel.map(|channel| channel.id().0 as i64)),
    })
    .on_conflict(
        OnConflict::column(event::Column::Id)
            .update_columns([
                event::Column::Name,
                event::Column::Description,
                event::Column::ChannelId,
                event::Column::RoleId,
                event::Column::AnnouncementChannelId,
            ])
            .action_and_where(event::Column::OwnerId.eq(ctx.author().id.0 as i64))
            .to_owned(),
    )
    .exec(&ctx.data().db_conn)
    .await;

    match result {
        Ok(InsertResult { last_insert_id }) => {
            let embed = create_embed(
                "Event updated",
                &format!(
                    "The event `{}` can be accessed [here](https://tip.panik.me/guild/{}/event/{}).",
                    name,
                    ctx.guild_id().unwrap().0,
                    last_insert_id
                ),
            );
            ctx.send(|reply| {
                reply.embeds = vec![embed];
                reply
            })
            .await?;

            Ok(())
        }
        Err(DbErr::RecordNotInserted) => {
            bail!("It looks like you're not allowed to overwrite this event. Are you sure you're the owner?")
        }
        Err(why) => Err(why.into()),
    }
}

async fn check_execution(ctx: Context<'_>) -> Result<bool> {
    // Unwrap is safe here due to guild_only constraint
    let execution_role = guild::Entity::find_by_id(ctx.guild_id().unwrap().0 as i64)
        .one(&ctx.data().db_conn)
        .await?
        .and_then(|guild| guild.execution_role_id);

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

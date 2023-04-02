use anyhow::{Error, Result};
use entity::{event, guild};
use poise::serenity_prelude as serenity;
use sea_orm::{entity::EntityTrait, ActiveValue::Set, ColumnTrait, QueryFilter};

use crate::client::Data;

pub async fn on_channel_delete(
    framework: poise::FrameworkContext<'_, Data, Error>,
    channel: &serenity::GuildChannel,
) -> Result<()> {
    guild::Entity::update_many()
        .set(guild::ActiveModel {
            default_channel_id: Set(None),
            ..Default::default()
        })
        .filter(
            guild::Column::Id
                .eq(channel.guild_id.0 as i64)
                .and(guild::Column::DefaultChannelId.eq(channel.id.0 as i64)),
        )
        .exec(&framework.user_data.db_conn)
        .await?;

    event::Entity::update_many()
        .set(event::ActiveModel {
            channel_id: Set(None),
            ..Default::default()
        })
        .filter(
            event::Column::GuildId
                .eq(channel.guild_id.0 as i64)
                .and(event::Column::ChannelId.eq(channel.id.0 as i64)),
        )
        .exec(&framework.user_data.db_conn)
        .await?;

    event::Entity::update_many()
        .set(event::ActiveModel {
            announcement_channel_id: Set(None),
            ..Default::default()
        })
        .filter(
            event::Column::GuildId
                .eq(channel.guild_id.0 as i64)
                .and(event::Column::AnnouncementChannelId.eq(channel.id.0 as i64)),
        )
        .exec(&framework.user_data.db_conn)
        .await?;

    Ok(())
}

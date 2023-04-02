use anyhow::{Error, Result};
use entity::guild;
use poise::serenity_prelude as serenity;
use sea_orm::entity::EntityTrait;

use crate::client::Data;

pub async fn on_guild_delete(
    framework: poise::FrameworkContext<'_, Data, Error>,
    unavailable: &serenity::UnavailableGuild,
) -> Result<()> {
    guild::Entity::delete_by_id(unavailable.id.0 as i64)
        .exec(&framework.user_data.db_conn)
        .await?;
    Ok(())
}

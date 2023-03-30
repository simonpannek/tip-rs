use anyhow::Result;
use entity::{event, guild};
use poise::serenity_prelude as serenity;
use sea_orm::{entity::EntityTrait, ActiveValue::Set, ColumnTrait, QueryFilter};

use crate::client::Data;

pub async fn on_role_delete(
    data: &Data,
    guild_id: &serenity::GuildId,
    removed_role_id: &serenity::RoleId,
) -> Result<()> {
    guild::Entity::update_many()
        .set(guild::ActiveModel {
            execution_role_id: Set(None),
            ..Default::default()
        })
        .filter(
            guild::Column::Id
                .eq(guild_id.0 as i64)
                .and(guild::Column::ExecutionRoleId.eq(removed_role_id.0 as i64)),
        )
        .exec(&data.db_conn)
        .await?;

    event::Entity::update_many()
        .set(event::ActiveModel {
            role_id: Set(None),
            ..Default::default()
        })
        .filter(
            event::Column::GuildId
                .eq(guild_id.0 as i64)
                .and(event::Column::RoleId.eq(removed_role_id.0 as i64)),
        )
        .exec(&data.db_conn)
        .await?;

    Ok(())
}

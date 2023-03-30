use anyhow::Result;
use entity::user;
use poise::serenity_prelude as serenity;
use sea_orm::{entity::EntityTrait, ActiveValue::Set, DbErr};

use crate::client::Data;

pub async fn on_member_update(data: &Data, new: &serenity::Member) -> Result<()> {
    let result = user::Entity::update(user::ActiveModel {
        id: Set(new.user.id.0 as i64),
        name: Set(format!("{}#{}", new.user.name, new.user.discriminator)),
        avatar: Set(new.user.avatar.clone()),
    })
    .exec(&data.db_conn)
    .await;

    // Allow no updated rows
    if !matches!(result, Err(DbErr::RecordNotUpdated)) {
        result?;
    }

    Ok(())
}

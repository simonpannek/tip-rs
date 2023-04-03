use anyhow::{Error, Result};
use entity::{event, guild, scheduled_action};
use poise::serenity_prelude as serenity;
use sea_orm::JsonValue;

use crate::client::Data;

pub async fn create_survey(
    _ctx: &serenity::Context,
    _framework: &poise::FrameworkContext<'_, Data, Error>,
    _action: &scheduled_action::Model,
    _event: &event::Model,
    _guild: &guild::Model,
) -> Result<JsonValue> {
    todo!()
}

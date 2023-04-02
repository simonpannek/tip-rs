use anyhow::Result;
use entity::event;
use poise::serenity_prelude as serenity;

pub async fn resolve_survey(
    _ctx: &serenity::Context,
    _data: &serde_json::Value,
    _event: &event::Model,
) -> Result<()> {
    todo!()
}

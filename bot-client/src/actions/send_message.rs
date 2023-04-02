use anyhow::Result;
use entity::event;
use poise::serenity_prelude as serenity;

pub async fn send_message(
    _ctx: &serenity::Context,
    _data: &serde_json::Value,
    _event: &event::Model,
) -> Result<()> {
    todo!()
}

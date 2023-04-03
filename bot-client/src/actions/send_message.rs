use anyhow::{anyhow, Error, Result};
use entity::{action_data::SendMessageData, event, guild, scheduled_action};
use poise::serenity_prelude as serenity;
use sea_orm::EntityTrait;
use serde_json::Value;

use crate::{
    client::Data,
    utils::{create_embed_with_color, create_or_edit_message},
};

pub async fn send_message(
    ctx: &serenity::Context,
    framework: &poise::FrameworkContext<'_, Data, Error>,
    action: &scheduled_action::Model,
    event: &event::Model,
    guild: &guild::Model,
) -> Result<Value> {
    // Get data of current action
    let mut data: SendMessageData = serde_json::from_value(action.action_data.clone())?;

    // Get data of parent action, if available
    let parent_data: Option<SendMessageData> = match action.parent_action_id {
        Some(parent_action_id) => {
            // Unwrap is safe here due to database constraint 'fk-scheduled_action-parent_action_id'
            let parent_action = scheduled_action::Entity::find_by_id(parent_action_id)
                .one(&framework.user_data.db_conn)
                .await?
                .unwrap();

            Some(serde_json::from_value(parent_action.action_data.clone())?)
        }
        None => None,
    };

    // Find the correct channel
    let channel_id = {
        let parent_id = parent_data.as_ref().and_then(|data| data.channel_id);

        let id = if data.announcement {
            vec![
                parent_id,
                event.announcement_channel_id,
                guild.default_channel_id,
            ]
        } else {
            vec![
                parent_id,
                event.channel_id,
                event.announcement_channel_id,
                guild.default_channel_id,
            ]
        }
        .iter()
        .find_map(|id| id.clone())
        .ok_or(anyhow!("No channel found to send the message to."))?;

        serenity::ChannelId(id as u64)
    };

    let message_id =
        parent_data.and_then(|data| data.message_id.map(|id| serenity::MessageId(id as u64)));

    let embed = create_embed_with_color(&data.title, &data.description, data.color);

    let message = create_or_edit_message(&ctx.http, channel_id, message_id, embed).await?;

    data.channel_id = Some(message.channel_id.0 as i64);
    data.message_id = Some(message.id.0 as i64);

    Ok(serde_json::to_value(&data)?)
}

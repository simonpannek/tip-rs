use anyhow::{anyhow, Error, Result};
use entity::{
    action_data::{CreateSurveyData, SendMessageData},
    event, guild, scheduled_action, survey_question,
};
use poise::serenity_prelude as serenity;
use sea_orm::{ActiveValue::Set, EntityTrait, JsonValue};

use crate::client::Data;

pub async fn create_survey(
    ctx: &serenity::Context,
    framework: &poise::FrameworkContext<'_, Data, Error>,
    action: &scheduled_action::Model,
    _event: &event::Model,
    _guild: &guild::Model,
) -> Result<JsonValue> {
    // Get data of current action
    let data: CreateSurveyData = serde_json::from_value(action.action_data.clone())?;

    // Get data of parent action
    let parent_data: SendMessageData = match action.parent_action_id {
        Some(parent_action_id) => {
            // Unwrap is safe here due to database constraint 'fk-scheduled_action-parent_action_id'
            let parent_action = scheduled_action::Entity::find_by_id(parent_action_id)
                .one(&framework.user_data.db_conn)
                .await?
                .unwrap();

            Ok(serde_json::from_value(parent_action.action_data.clone())?)
        }
        None => Err(anyhow!("No parent action found to attach survey to.")),
    }?;

    // Find the correct channel
    let channel_id = parent_data
        .channel_id
        .map(|id| serenity::ChannelId(id as u64))
        .ok_or(anyhow!("No channel found to attach survey to."))?;

    let message_id = parent_data
        .message_id
        .map(|id| serenity::MessageId(id as u64))
        .ok_or(anyhow!("No message found to attach survey to."))?;

    let components = {
        // Only loop over the first 25 questions (one message cannot hold more than 25 buttons)
        let mut buttons = Vec::new();
        for option in data.survey_options.iter().take(25) {
            // Create question entry in database
            let result = survey_question::Entity::insert(survey_question::ActiveModel {
                event_id: Set(action.event_id),
                message_id: Set(message_id.0 as i64),
                question: Set(option.question.clone()),
                ..Default::default()
            })
            .exec(&framework.user_data.db_conn)
            .await?;

            let mut button = serenity::CreateButton::default();
            button
                .custom_id(result.last_insert_id)
                .style(serenity::ButtonStyle::Primary)
                .label(option.label.clone());

            buttons.push(button);
        }

        // Loop over buttons in chunks of 5 (one action row cannot hold more than 5 buttons)
        let mut rows = Vec::new();
        for button_chunk in buttons.chunks(5) {
            let mut row = serenity::CreateActionRow::default();
            for button in button_chunk {
                row.add_button(button.clone());
            }
            rows.push(row);
        }

        let mut components = serenity::CreateComponents::default();
        components.set_action_rows(rows);

        components
    };

    channel_id
        .edit_message(&ctx.http, message_id, |edit| {
            edit.set_components(components)
        })
        .await?;

    Ok(serde_json::to_value(&data)?)
}

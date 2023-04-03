use anyhow::{Error, Result};
use entity::{
    action_data::SendMessageData,
    event, guild,
    scheduled_action::{self, ActionType},
};
use poise::serenity_prelude as serenity;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder,
};

use crate::{actions::*, client::Data};

pub async fn run_actions(
    ctx: &serenity::Context,
    framework: &poise::FrameworkContext<'_, Data, Error>,
) -> Result<()> {
    let mut action_pages = scheduled_action::Entity::find()
        .find_also_related(event::Entity)
        .filter(
            scheduled_action::Column::Time
                .lte(chrono::Utc::now())
                .and(scheduled_action::Column::Executed.eq(false)),
        )
        .order_by_asc(scheduled_action::Column::Time)
        .paginate(&framework.user_data.db_conn, 50);

    let data = SendMessageData {
        title: "Test".to_string(),
        description: "Test description".to_string(),
        color: None,
        announcement: false,
        channel_id: None,
        message_id: None,
    };

    println!("DATA: {}", serde_json::to_string(&data)?);

    while let Some(actions) = action_pages.fetch_and_next().await? {
        for (action, event) in actions {
            // Unwrap is safe here due to database constraint 'fk-scheduled_action-event_id'
            let event = event.unwrap();
            // Unwrap is safe here due to database constraint 'fk-event-guild_id'
            // TODO: Optimize this
            let guild = event
                .find_related(guild::Entity)
                .one(&framework.user_data.db_conn)
                .await?
                .unwrap();

            // Execute action
            let action_data = match action.action_type {
                ActionType::SendMessage => {
                    send_message::send_message(ctx, framework, &action, &event, &guild).await?
                }
                ActionType::CreateSurvey => {
                    create_survey::create_survey(ctx, framework, &action, &event, &guild).await?
                }
                ActionType::ResolveSurvey => {
                    resolve_survey::resolve_survey(ctx, framework, &action, &event, &guild).await?
                }
            };

            // Mark action as executed
            let mut action: scheduled_action::ActiveModel = action.into();
            action.executed = Set(true);
            action.action_data = Set(action_data);
            action.update(&framework.user_data.db_conn).await?;
        }
    }

    Ok(())
}

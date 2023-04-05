use anyhow::{Error, Result};
use entity::{
    action_data::{SurveyQuestion, SurveyResponseOption},
    survey_question, survey_response,
};
use poise::serenity_prelude::{
    self as serenity, model::application::interaction::Interaction, ActionRowComponent,
    ComponentType, InputTextStyle, InteractionResponseType,
};
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{client::Data, utils::create_embed};

pub async fn on_interaction_create(
    ctx: &serenity::Context,
    framework: poise::FrameworkContext<'_, Data, Error>,
    interaction: &Interaction,
) -> Result<()> {
    match interaction {
        Interaction::MessageComponent(interaction) => {
            match interaction.data.component_type {
                ComponentType::Button => {
                    let question = survey_question::Entity::find_by_id(
                        interaction.data.custom_id.parse::<i64>()?,
                    )
                    .one(&framework.user_data.db_conn)
                    .await?;

                    // Only respond to registered buttons
                    if let Some(question) = question {
                        let data: SurveyQuestion = serde_json::from_value(question.question_data)?;

                        match data.response_options {
                            SurveyResponseOption::SelectMenu(options) => {
                                let embed = create_embed(&data.label, &data.question);

                                let mut select_options = Vec::new();
                                for option in options {
                                    select_options.push(serenity::CreateSelectMenuOption::new(
                                        option.clone(),
                                        option,
                                    ));
                                }

                                let mut row = serenity::CreateActionRow::default();
                                row.create_select_menu(|input_text| {
                                    input_text
                                        .custom_id(&interaction.data.custom_id)
                                        .placeholder("Select your response")
                                        .options(|options| options.set_options(select_options))
                                });

                                interaction
                                    .create_interaction_response(&ctx.http, |response| {
                                        response
                                            .kind(InteractionResponseType::ChannelMessageWithSource)
                                            .interaction_response_data(|response_data| {
                                                response_data
                                                    .set_embed(embed)
                                                    .components(|components| {
                                                        components.set_action_row(row)
                                                    })
                                                    .ephemeral(true)
                                            })
                                    })
                                    .await?;
                            }
                            SurveyResponseOption::InputText => {
                                let mut row = serenity::CreateActionRow::default();
                                row.create_input_text(|input_text| {
                                    input_text
                                        .custom_id(&interaction.data.custom_id)
                                        .style(InputTextStyle::Short)
                                        .label(&data.question)
                                        .placeholder("Enter your response")
                                        .required(true)
                                });

                                interaction
                                    .create_interaction_response(&ctx.http, |response| {
                                        response
                                            .kind(InteractionResponseType::Modal)
                                            .interaction_response_data(|response_data| {
                                                response_data
                                                    .custom_id(interaction.data.custom_id.clone())
                                                    .title(&data.label)
                                                    .components(|components| {
                                                        components.set_action_row(row)
                                                    })
                                            })
                                    })
                                    .await?;
                            }
                        };
                    }
                }
                ComponentType::SelectMenu => {
                    let question_id = interaction.data.custom_id.parse()?;
                    let user_id = interaction.user.id.0 as i64;

                    survey_response::Entity::delete_many()
                        .filter(
                            survey_response::Column::QuestionId
                                .eq(question_id)
                                .and(survey_response::Column::UserId.eq(user_id)),
                        )
                        .exec(&framework.user_data.db_conn)
                        .await?;

                    for value in &interaction.data.values {
                        survey_response::Entity::insert(survey_response::ActiveModel {
                            question_id: Set(question_id),
                            user_id: Set(user_id),
                            response: Set(value.clone()),
                            ..Default::default()
                        })
                        .exec(&framework.user_data.db_conn)
                        .await?;

                        interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.kind(InteractionResponseType::DeferredUpdateMessage)
                            })
                            .await?;
                    }
                }
                _ => {}
            }

            Ok(())
        }
        Interaction::ModalSubmit(interaction) => {
            if let Some(row) = interaction.data.components.first() {
                if let Some(component) = row.components.first() {
                    if let ActionRowComponent::InputText(input_text) = component {
                        let question_id = interaction.data.custom_id.parse()?;
                        let user_id = interaction.user.id.0 as i64;

                        survey_response::Entity::delete_many()
                            .filter(
                                survey_response::Column::QuestionId
                                    .eq(question_id)
                                    .and(survey_response::Column::UserId.eq(user_id)),
                            )
                            .exec(&framework.user_data.db_conn)
                            .await?;

                        survey_response::Entity::insert(survey_response::ActiveModel {
                            question_id: Set(question_id),
                            user_id: Set(user_id),
                            response: Set(input_text.value.clone()),
                            ..Default::default()
                        })
                        .exec(&framework.user_data.db_conn)
                        .await?;

                        interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.kind(InteractionResponseType::DeferredUpdateMessage)
                            })
                            .await?;
                    }
                }
            }

            Ok(())
        }
        _ => Ok(()),
    }
}

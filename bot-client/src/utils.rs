use anyhow::Result;
use itertools::Itertools;
use poise::serenity_prelude::{self as serenity, Color, CreateEmbed};

pub fn create_embed(title: &str, content: &str) -> CreateEmbed {
    create_embed_with_color(title, content, None)
}

pub fn create_embed_with_color(title: &str, content: &str, color: Option<Color>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title(title.chars().take(256).join(""))
        .description(content)
        .color(color.unwrap_or(Color::from((43, 45, 49))));
    embed
}

pub async fn create_or_edit_message(
    http: impl AsRef<serenity::Http>,
    channel_id: serenity::ChannelId,
    message_id: Option<serenity::MessageId>,
    embed: CreateEmbed,
) -> Result<serenity::Message> {
    let result = match message_id {
        Some(message_id) => {
            channel_id
                .edit_message(http, message_id, |edit| edit.set_embed(embed))
                .await?
        }
        None => {
            channel_id
                .send_message(http, |create| create.set_embed(embed))
                .await?
        }
    };

    Ok(result)
}

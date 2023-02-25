use itertools::Itertools;
use serenity::{builder::CreateEmbed, utils::Colour};

pub fn create_embed(title: &str, content: &str) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title(title.chars().take(256).join(""))
        .description(content)
        .color(Colour::from((43, 45, 49)));
    embed
}

use anyhow::{Error, Result};
use poise::serenity_prelude as serenity;
use tokio::time;
use tracing::error;

use crate::{actions::run_actions::run_actions, client::Data};

pub async fn on_ready(
    ctx: &serenity::Context,
    framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<()> {
    // Create an interval to check for new actions twice a minute
    let mut interval = time::interval(time::Duration::from_secs(30));

    // Start checking for actions not executed yet
    loop {
        interval.tick().await;

        if let Err(why) = run_actions(ctx, &framework).await {
            error!("Executing actions failed: {}", why);
        }
    }
}

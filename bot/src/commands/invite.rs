use std::env;

use crate::prelude::*;
use poise::say_reply;

/// このBotの招待URLを表示します
#[poise::command(slash_command, prefix_command)]
pub async fn invite(ctx: Context<'_>) -> Result<(), BotError> {
    let invitation_url = match env::var("INVITATION_URL") {
        Ok(url) => url,
        Err(_) => {
            send_custom_error(ctx, "INVITATION URL NOT SET").await?;
            return Ok(());
        }
    };

    say_reply(ctx, &invitation_url).await?;
    Ok(())
}

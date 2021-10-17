use std::env;

use crate::prelude::*;
use poise::send_reply;

/// このBotの使い方を表示します
#[poise::command(slash_command, prefix_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), BotError> {
    let description = format!(include_str!("./help.txt"), dummy="　", invite=env::var("INVITATION_URL").unwrap());
    let user = ctx.discord().cache.current_user();
    let thumbnail = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());
    send_reply(ctx, |f| {
        f.embed(|e| {
            e.title("DisCalendar - Help");
            e.description(description);
            e.color(0x0000dd);
            e.timestamp(chrono::Utc::now());
            e.footer(|f| f.text(format!("v{}", crate::VERSION)));
            e.thumbnail(thumbnail)
        });
        f.ephemeral(true)
    })
        .await?;
    Ok(())
}

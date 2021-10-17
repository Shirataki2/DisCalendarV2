pub mod paginator;
pub mod checks;

use crate::prelude::*;

const LOG_CHANNEL_ID: u64 = 783740453173985280;

pub async fn send_custom_error(
    ctx: Context<'_>,
    code: &str
) -> Result<(), BotError> {
    poise::say_reply(ctx, format!("予期せぬエラーです＞＜\n開発者に以下のコードをお知らせください！\n[{}]", code)).await?;
    Ok(())
}

pub async fn send_log<'a, F>(ctx: &'a serenity::Context, f: F) -> Result<(), BotError>
where
    for<'b> F: FnOnce(&'b mut serenity::CreateMessage<'a>) -> &'b mut serenity::CreateMessage<'a>,
{
    let log_channel = match ctx.cache.guild_channel(LOG_CHANNEL_ID) {
        Some(channel) => channel,
        None => {
            info!("Failed to find log channel");
            return Ok(());
        }
    };

    log_channel.send_message(ctx, f).await?;

    Ok(())
}

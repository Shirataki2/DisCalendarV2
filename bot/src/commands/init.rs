use crate::prelude::*;
use poise::say_reply;

/// このBotの通知の送信先を設定します
#[poise::command(
    slash_command,
    prefix_command,
    check = "has_manage_perms"
)]
pub async fn init(
    ctx: Context<'_>,
    #[description = "通知先のチャンネル(指定しない場合はこのコマンドを送信したチャンネルになります)"]
    channel: Option<serenity::ChannelId>,
) -> Result<(), BotError> {
    let pool = &ctx.data().pool;

    let channel = channel.unwrap_or_else(|| ctx.channel_id());
    let guild = match ctx.guild() {
        Some(guild) => guild,
        None => {
            say_reply(ctx, "このコマンドはサーバー内で実行してください").await?;
            return Ok(());
        }
    };
    match EventSettings::get(pool, &guild.id.0.to_string()).await? {
        Some(s) => {
            EventSettings::update(pool, &guild.id.0, &channel.0).await?;
            say_reply(
                ctx,
                format!("イベント通知先を変更しました\n通知先: <#{}> → <#{}>", s.channel_id, channel.0)
            ).await?;
        }
        None => {
            EventSettings::create(pool, &guild.id.0, &channel.0).await?;
            say_reply(
                ctx,
                format!("イベント通知を有効にしました\n通知先: <#{}>", channel.0)
            ).await?;
        }
    }
    Ok(())
}

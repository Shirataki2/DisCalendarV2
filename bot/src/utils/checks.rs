#![allow(dead_code)]
use crate::prelude::*;

pub async fn has_manage_perms(ctx: Context<'_>) -> Result<bool, BotError> {
    let author = ctx.author();
    let guild = match ctx.guild() {
        Some(g) => g,
        None => return Ok(false),
    };
    let owner_id = guild.owner_id;

    let is_owner = author.id == owner_id;

    let perms = match guild.member(ctx.discord(), &author.id).await.ok() {
        Some(m) => m.permissions(ctx.discord())?,
        None => return Ok(false),
    };

    let flag = perms.manage_guild()
        || perms.administrator()
        || perms.manage_messages()
        || perms.manage_roles()
        || is_owner;

    if !flag {
        poise::send_reply(ctx, |f| {
            f.content("このコマンドを実行するには`メッセージ管理権限`,`サーバー管理権限`,`ロール管理権限`,`管理者`のいずれかの権限を持っている必要があります");
            f.ephemeral(true);
            f
        }).await?;
    }

    Ok(flag)
}

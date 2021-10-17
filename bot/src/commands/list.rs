use crate::prelude::*;

use chrono::{NaiveDateTime, Utc};
use poise::say_reply;

/// 予定の一覧を表示します
#[poise::command(slash_command, prefix_command)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "表示する予定の範囲"]range: Option<EventRange>,
) -> Result<(), BotError> {
    let range = range.unwrap_or_default();
    let pool = &ctx.data().pool;
    let guild_id = ctx.guild_id().unwrap().0.to_string();
    let now = Utc::now().naive_utc();

    let events = match range {
        EventRange::Past => Event::find_past_events(pool, &guild_id, now).await?,
        EventRange::Future => Event::find_future_events(pool, &guild_id, now).await?,
        EventRange::All => Event::find_by_guild_id(pool, &guild_id).await?,
    };
    if events.is_empty() {
        say_reply(ctx, "現在登録されている予定はありません").await?;
        return Ok(());
    }
    let template = serenity::CreateEmbed::default()
        .title("予定一覧")
        .color(0x0000ff)
        .clone();
    let mut paginator = Paginator::new(4, template);
    for event in events.iter() {
        let value = format!(
            "`開始時刻`:{}\n`終了時刻`:{}\n`　通知　`:{}",
            format_date(event.start_at),
            format_date(event.end_at),
            format_notification(event)
        );
        format_notification(event);
        paginator.add(event.name.clone(), value, false);
    }
    paginator.start(ctx).await?;

    Ok(())
}

fn format_date(date: NaiveDateTime) -> String {
    let time = date.format("%Y/%m/%d %H:%M").to_string();
    time
}

fn format_notification(event: &Event) -> String{
    let time = event.notifications.clone();
    let notifications = time.iter().filter_map(|t| {
        serde_json::from_str::<NotificationPayload>(t).ok()
    })
    .collect::<Vec<events::NotificationPayload>>();
    let notifications = notifications.iter().map(|n| {
        format!("{}{}", n.num, n.ty)
    }).collect::<Vec<String>>();
    notifications.join(", ")
}

#[derive(Debug, poise::SlashChoiceParameter)]
pub enum EventRange {
    #[name = "過去"]
    Past,
    #[name = "未来"]
    Future,
    #[name = "全て"]
    All,
}

impl Default for EventRange {
    fn default() -> Self {
        EventRange::Future
    }
}

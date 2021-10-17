use chrono::{NaiveDateTime, NaiveTime, Timelike};
use sqlx::PgPool;

use crate::prelude::*;
use std::sync::Arc;

macro_rules! ensure {
    ($e:expr) => {
        match $e {
            Ok(obj) => obj,
            Err(why) => {
                error!("{}", why);
                return;
            }
        }
    };
}

pub async fn run(ctx: Arc<serenity::Context>, data: Arc<Data>) {
    info!("Running task: notify");
    let pool = &data.pool;
    let jst_now = chrono::Utc::now().naive_utc() + chrono::Duration::hours(9);
    let jst_now = jst_now.date().and_hms(jst_now.hour(), jst_now.minute(), 0);
    let events = ensure!(Event::find_all_future_events(pool, jst_now).await);
    info!("Fetch {} events", events.len());
    let tasks = events
        .iter()
        .map(|e| subroutine(pool, Arc::clone(&ctx), e))
        .collect::<Vec<_>>();
    futures::future::join_all(tasks).await;
}

async fn subroutine(pool: &PgPool, ctx: Arc<serenity::Context>, event: &Event) {
    let setting = match ensure!(EventSettings::get(pool, &event.guild_id).await) {
        Some(setting) => setting,
        None => return,
    };
    let guild = match ctx.cache.guild(event.guild_id.parse().unwrap_or(0)) {
        Some(guild) => guild,
        None => return,
    };
    let channel = match guild
        .channels
        .get(&setting.channel_id.parse().unwrap_or_default())
    {
        Some(channel) => channel,
        None => return,
    };
    let notifications = event.notifications.clone();
    let mut notifications: Vec<events::NotificationPayload> = notifications
        .iter()
        .filter_map(|n| serde_json::from_str(n).ok())
        .collect();
    notifications.push(NotificationPayload {
        key: -1,
        num: 0,
        ty: "分前".to_string(),
    });
    notifications.sort_by(|a, b| a.key.cmp(&b.key));
    for notification in notifications.iter() {
        let mut minutes = notification.num;
        if notification.ty == "時間前" {
            minutes *= 60;
        } else if notification.ty == "日前" {
            minutes *= 60 * 24;
        } else if notification.ty == "週間前" {
            minutes *= 60 * 24 * 7;
        }
        let (start, end) = if !event.is_all_day {
            (event.start_at, event.end_at)
        } else {
            let zero_oclock = NaiveTime::from_hms(0, 0, 0);
            let start = NaiveDateTime::new(event.start_at.date(), zero_oclock);
            let end = NaiveDateTime::new(event.end_at.date(), zero_oclock);
            (start, end)
        };
        let jst_now = chrono::Utc::now().naive_utc()
            + chrono::Duration::hours(9)
            + chrono::Duration::minutes(minutes.into());
        let jst_now_minus_1 = jst_now - chrono::Duration::minutes(1);
        if start >= jst_now_minus_1 && start < jst_now {
            let result = channel
                .id()
                .send_message(&*ctx, |f| {
                    f.embed(|e| {
                        let color = event.color.trim_start_matches('#');
                        let color = i32::from_str_radix(color, 16).unwrap_or(0xff0000);
                        e.color(color);
                        e.title(event.name.clone());
                        if let Some(description) = &event.description {
                            e.description(description);
                        }
                        let author = if notification.key == -1 {
                            "以下の予定が開催されます".to_string()
                        } else {
                            format!(
                                "{}{}に以下の予定が開催されます",
                                notification.num,
                                notification.ty.replace("前", "後")
                            )
                        };
                        e.author(|a| a.name(author));
                        let date = if event.is_all_day {
                            if start == end {
                                format!("{}", start.format("%Y/%m/%d"))
                            } else {
                                format!("{} - {}", start.format("%Y/%m/%d"), end.format("%Y/%m/%d"))
                            }
                        } else {
                            let start_date = start.date();
                            let end_date = end.date();
                            if start_date == end_date {
                                format!(
                                    "{} - {}",
                                    start.format("%Y/%m/%d %H:%M"),
                                    end.format("%H:%M")
                                )
                            } else {
                                format!(
                                    "{} - {}",
                                    start.format("%Y/%m/%d %H:%M"),
                                    end.format("%Y/%m/%d %H:%M")
                                )
                            }
                        };
                        e.field("日時", date, false);

                        e
                    })
                })
                .await;
            if let Err(why) = result {
                error!("{}", why);
            }
        }
    }
}

#![allow(clippy::too_many_arguments)]

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use crate::prelude::*;

use poise::send_reply;

/// 予定を新たに作成します
#[poise::command(slash_command, prefix_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "予定の名称"] name: String,
    #[description = "予定の説明"] description: Option<String>,
    #[description = "予定開始時間(年)"] start_year: i32,
    #[description = "予定開始時間(月)"] start_month: u32,
    #[description = "予定開始時間(日)"] start_day: u32,
    #[description = "予定開始時間(時)"] start_hour: u32,
    #[description = "予定開始時間(分)"] start_minute: u32,
    #[description = "予定終了時間(年)"] end_year: i32,
    #[description = "予定終了時間(月)"] end_month: u32,
    #[description = "予定終了時間(日)"] end_day: u32,
    #[description = "予定終了時間(時)"] end_hour: u32,
    #[description = "予定終了時間(分)"] end_minute: u32,
    #[description = "終日行う予定か"] is_all_day: Option<bool>,
    #[description = "予定の配色"] color: Option<Color>,
    #[description = "予定の事前通知"] notify_1: Option<Notification>,
    #[description = "予定の事前通知"] notify_2: Option<Notification>,
    #[description = "予定の事前通知"] notify_3: Option<Notification>,
    #[description = "予定の事前通知"] notify_4: Option<Notification>,

) -> Result<(), BotError> {
    let pool = &ctx.data().pool;
    let guild = match ctx.guild() {
        Some(guild) => guild,
        None => {
            poise::say_reply(ctx, "このコマンドはサーバーでのみ実行可能です").await?;
            return Ok(())
        },
    };
    let guild_id = guild.id.0.to_string();

    let restricted = guilds::GuildConfig::get(pool, &guild_id).await?.map(|c| c.restricted).unwrap_or_default();

    if restricted {
        let perms = guild.member_permissions(ctx.discord(), ctx.author().id).await?;
        let is_manager = perms.administrator() || perms.manage_roles() || perms.manage_messages() || perms.manage_guild();
        if !is_manager {
            poise::send_reply(ctx, |f| {
                f.content("このコマンドを実行するためには「管理者」「サーバー管理」「ロールの管理」「メッセージの管理」のいずれかの権限が必要です");
                f.ephemeral(true)
            }).await?;
            return Ok(())
        }
    }

    check_date(start_year, start_month, start_day, start_hour, start_minute)?;
    check_date(end_year, end_month, end_day, end_hour, end_minute)?;
    let start = NaiveDateTime::new(
        NaiveDate::from_ymd(start_year, start_month, start_day),
        NaiveTime::from_hms(start_hour, start_minute, 0),
    );
    let end = NaiveDateTime::new(
        NaiveDate::from_ymd(end_year, end_month, end_day),
        NaiveTime::from_hms(end_hour, end_minute, 0),
    );
    if start > end {
        return Err(BotError::new(
            "開始時間が終了時間より後になっています",
            "start_time_after_end_time",
        ));
    }
    let is_all_day = is_all_day.unwrap_or(false);
    let color = color.unwrap_or_default().to_colorcode();
    
    let raw_notifications = vec![notify_1, notify_2, notify_3, notify_4];
    let raw_notifications = raw_notifications.into_iter().flatten().collect::<Vec<_>>();
    let notifications = raw_notifications
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(i, n)| n.query(i).ok())
        .collect::<Vec<_>>();

    let data = EventCreateQuery {
        name: name.clone(),
        description: description.clone(),
        start_at: start,
        end_at: end,
        is_all_day,
        color: format!("#{:06x}", color),
        notifications,
        created_at: Utc::now().naive_utc(),
        guild_id,
    };

    info!("Successfully create event: {:?}", &data);
    Event::create(pool, data).await?;

    send_reply(ctx, |f| {
        f.content("正常に予定を作成しました");
        f.embed(|e| {
            e.title(name);
            e.description(description.unwrap_or_default());
            if is_all_day {
                e.fields(vec![
                    ("開始時間", format!("{}", start.format("%Y/%m/%d")), true),
                    ("終了時間", format!("{}", end.format("%Y/%m/%d")), true),
                ]);
            } else {
                e.fields(vec![
                    ("開始時間", format!("{}", start.format("%Y/%m/%d %H:%M")), true),
                    ("終了時間", format!("{}", end.format("%Y/%m/%d %H:%M")), true),
                ]);
            }
            if !raw_notifications.is_empty() {
                e.field("通知", raw_notifications.iter().map(|n| format!("{}", n)).collect::<Vec<_>>().join(","), true);
            }
            e.color(color);
            e.timestamp(chrono::Utc::now())
        })
    }).await?;


    Ok(())
}

fn check_date(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
) -> Result<(), BotError> {
    if !(1970..=2099).contains(&year) {
        return Err(BotError::InvalidDate(year, month, day, hour, minute));
    }
    if !(1..=12).contains(&month) {
        return Err(BotError::InvalidDate(year, month, day, hour, minute));
    }
    if !(1..=31).contains(&day) {
        return Err(BotError::InvalidDate(year, month, day, hour, minute));
    }
    if hour > 23 {
        return Err(BotError::InvalidDate(year, month, day, hour, minute));
    }
    if minute > 59 {
        return Err(BotError::InvalidDate(year, month, day, hour, minute));
    }
    if (month == 4 || month == 6 || month == 9 || month == 11) && day == 31 {
        return Err(BotError::InvalidDate(year, month, day, hour, minute));
    }
    if month == 2 {
        if day > 29 {
            return Err(BotError::InvalidDate(year, month, day, hour, minute));
        }
        if day == 29 && year % 4 != 0 {
            return Err(BotError::InvalidDate(year, month, day, hour, minute));
        }
    }
    Ok(())
}

#[derive(Debug, poise::SlashChoiceParameter)]
pub enum Color {
    #[name = "白"]
    White,
    #[name = "黒"]
    Black,
    #[name = "赤"]
    Red,
    #[name = "青"]
    Blue,
    #[name = "緑"]
    Green,
    #[name = "黄"]
    Yellow,
    #[name = "紫"]
    Purple,
    #[name = "灰"]
    Gray,
    #[name = "茶"]
    Brown,
    #[name = "水色"]
    Aqua,
}

impl Color {
    pub fn to_colorcode(&self) -> u32 {
        let code = match self {
            Color::White => 0xffffff,
            Color::Black => 0x000000,
            Color::Red => 0xfd4028,
            Color::Blue => 0x3e44f7,
            Color::Green => 0x33f54b,
            Color::Yellow => 0xeaff33,
            Color::Purple => 0xa31ce0,
            Color::Gray => 0x808080,
            Color::Brown => 0xa54f4f,
            Color::Aqua => 0x44f3f3,
        };
        code as u32
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Blue
    }
}

#[derive(Clone, Debug, poise::SlashChoiceParameter)]
pub enum Notification {
    #[name = "5分前"]
    FiveMinutes,
    #[name = "10分前"]
    TenMinutes,
    #[name = "15分前"]
    FifteenMinutes,
    #[name = "30分前"]
    ThirtyMinutes,
    #[name = "1時間前"]
    OneHour,
    #[name = "2時間前"]
    TwoHours,
    #[name = "3時間前"]
    ThreeHours,
    #[name = "6時間前"]
    SixHours,
    #[name = "12時間前"]
    TwelveHours,
    #[name = "1日前"]
    OneDay,
    #[name = "2日前"]
    TwoDays,
    #[name = "3日前"]
    ThreeDays,
    #[name = "7日前"]
    SevenDays,
}

impl Notification {
    #[allow(dead_code)]
    pub fn subtract_from(self, date: NaiveDateTime) -> NaiveDateTime {
        match self {
            Notification::FiveMinutes => date - Duration::minutes(5),
            Notification::TenMinutes => date - Duration::minutes(10),
            Notification::FifteenMinutes => date - Duration::minutes(15),
            Notification::ThirtyMinutes => date - Duration::minutes(30),
            Notification::OneHour => date - Duration::hours(1),
            Notification::TwoHours => date - Duration::hours(2),
            Notification::ThreeHours => date - Duration::hours(3),
            Notification::SixHours => date - Duration::hours(6),
            Notification::TwelveHours => date - Duration::hours(12),
            Notification::OneDay => date - Duration::days(1),
            Notification::TwoDays => date - Duration::days(2),
            Notification::ThreeDays => date - Duration::days(3),
            Notification::SevenDays => date - Duration::days(7),
        }
    }

    pub fn query(self, key: usize) -> Result<String, BotError> {
        let query = match self {
            Notification::FiveMinutes => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "5", "分前"),
            Notification::TenMinutes => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "10", "分前"),
            Notification::FifteenMinutes => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "15", "分前"),
            Notification::ThirtyMinutes => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "30", "分前"),
            Notification::OneHour => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "1", "時間前"),
            Notification::TwoHours => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "2", "時間前"),
            Notification::ThreeHours => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "3", "時間前"),
            Notification::SixHours => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "6", "時間前"),
            Notification::TwelveHours => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "12", "時間前"),
            Notification::OneDay => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "1", "日前"),
            Notification::TwoDays => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "2", "日前"),
            Notification::ThreeDays => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "3", "日前"),
            Notification::SevenDays => format!("{{\"key\": {},\"num\": {},\"type\": \"{}\"}}", key, "7", "日前"),
        };
        Ok(query)
    }
}

impl std::fmt::Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Notification::FiveMinutes => write!(f, "5分前"),
            Notification::TenMinutes => write!(f, "10分前"),
            Notification::FifteenMinutes => write!(f, "15分前"),
            Notification::ThirtyMinutes => write!(f, "30分前"),
            Notification::OneHour => write!(f, "1時間前"),
            Notification::TwoHours => write!(f, "2時間前"),
            Notification::ThreeHours => write!(f, "3時間前"),
            Notification::SixHours => write!(f, "6時間前"),
            Notification::TwelveHours => write!(f, "12時間前"),
            Notification::OneDay => write!(f, "1日前"),
            Notification::TwoDays => write!(f, "2日前"),
            Notification::ThreeDays => write!(f, "3日前"),
            Notification::SevenDays => write!(f, "7日前"),
        }
    }
}
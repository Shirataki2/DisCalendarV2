use crate::prelude::*;
use std::{ffi::OsStr, fs::File, io::Read, path::Path, sync::Arc};

use chrono::{Datelike, NaiveDateTime, Timelike, Weekday};

pub async fn run(ctx: Arc<serenity::Context>) {
    let jst_now = chrono::Utc::now().naive_utc() + chrono::Duration::hours(9);
    if jst_now.hour() == 0 && jst_now.minute() == 0 {
        info!("Running task: icon update");
        let mut me = ctx.cache.current_user();
        let icon = match get_date_icon(jst_now) {
            Ok(icon) => icon,
            Err(e) => {
                error!("Failed to update icon: {}", e);
                return;
            }
        };
        let _ = me.edit(&*ctx, |f| f.avatar(Some(&icon))).await;
        info!("Updating Server Icon");
        let mut guild = match ctx.cache.guild(782168943967469569) {
            Some(guild) => guild,
            None => {
                error!("Failed to update icon: Guild not found");
                return;
            }
        };
        let _ = guild.edit(&*ctx, |f| f.icon(Some(&icon))).await;
    }
}

fn get_date_icon(date: NaiveDateTime) -> Result<String, BotError> {
    let jph = jpholiday::jpholiday::JPHoliday::new();
    let is_holiday = jph.is_holiday(&date.date());
    let filename = if is_holiday {
        format!("assets/{}_r.png", date.format("%d"))
    } else {
        match date.weekday() {
            Weekday::Sun => {
                format!("assets/{}_r.png", date.format("%d"))
            },
            Weekday::Sat => {
                format!("assets/{}_b.png", date.format("%d"))
            },
            _ => {
                format!("assets/{}.png", date.format("%d"))
            }
        }
    };
    read_image(filename)
}

fn read_image<P: AsRef<Path>>(path: P) -> Result<String, BotError> {
    let path = path.as_ref();
    let mut v = Vec::default();
    let mut f = File::open(path)?;

    #[allow(clippy::let_underscore_must_use)]
    let _ = f.read_to_end(&mut v);

    let b64 = base64::encode(&v);
    let ext = if path.extension() == Some(OsStr::new("png")) { "png" } else { "jpg" };

    Ok(format!("data:image/{};base64,{}", ext, b64))
}
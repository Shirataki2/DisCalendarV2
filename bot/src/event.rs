use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crate::prelude::*;
use crate::tasks::start_loop;
use once_cell::sync::Lazy;
use poise::Event;


static INITIALIZED: Lazy<Arc<Mutex<AtomicBool>>> =
    Lazy::new(|| Arc::new(Mutex::new(AtomicBool::new(false))));

pub async fn handle_event<'a>(
    ctx: &'a serenity::Context,
    event: &'a poise::Event<'a>,
    _fw: &'a poise::Framework<Data, BotError>,
    data: &'a Data,
) -> Result<(), BotError> {
    match event {
        Event::Ready { data_about_bot } => {
            info!("Log in as {}", data_about_bot.user.name);
        }
        Event::CacheReady { .. } => {
            let is_initialized = {
                match INITIALIZED.lock() {
                    Ok(mutex) => mutex.load(Ordering::Relaxed),
                    Err(_) => false,
                }
            };
            if !is_initialized {
                info!("Initializing bot...");
                let ctx = Arc::new(ctx.clone());
                let data = Arc::new(data.clone());
                start_loop(ctx, data).await;
                let lock = INITIALIZED.lock();
                if let Ok(guard) = lock {
                    guard.store(true, Ordering::Relaxed);
                }
            }
        }
        Event::GuildUpdate {
            old_data_if_available: Some(old_data),
            new_but_incomplete,
        } => {
            let guild_id = old_data.id.0.to_string();
            let old_name = old_data.name.clone();
            let new_name = new_but_incomplete.name.clone();
            let old_icon = old_data.icon_url().unwrap_or_else(|| "OLD".to_string());
            let new_icon = new_but_incomplete
                .icon_url()
                .unwrap_or_else(|| "NEW".to_string());
            if old_name != new_name || old_icon != new_icon {
                info!("Update Guild: {}", &new_name);
                let pool = &data.pool;
                let g = guilds::GuildMessage {
                    guild_id: guild_id.clone(),
                    name: new_name,
                    avatar_url: Some(new_icon),
                    locale: "ja".to_string(),
                };
                Guild::update(pool, &guild_id, &g).await?;
            }
        }
        Event::GuildCreate { guild, .. } => {
            let pool = &data.pool;
            if Guild::find_by_guild_id(pool, &guild.id.0.to_string())
                .await?
                .is_some()
            {
                return Ok(());
            }
            let g = guilds::GuildMessage {
                guild_id: guild.id.0.to_string(),
                name: guild.name.clone(),
                avatar_url: guild.icon_url(),
                locale: "ja".to_string(),
            };
            if let Err(e) = Guild::create(pool, &g).await {
                error!("Error Occured while inserting guild: {:?}", e);
            };
            send_log(ctx, |f| {
                f.embed(|e| {
                    e.title("DisCalendar - New Guild");
                    e.color(0x0000ff);
                    e.description(format!("{} ({})", guild.name, guild.id.0));
                    e
                })
            })
            .await?;
        }
        Event::GuildDelete { incomplete, full } => {
            let pool = &data.pool;
            if let Err(e) = Guild::delete(pool, &incomplete.id.0.to_string()).await {
                error!("Error Occured while deleting guild: {:?}", e);
            };
            if let Some(guild) = full {
                send_log(ctx, |f| {
                    f.embed(|e| {
                        e.title("DisCalendar - Leave Guild");
                        e.color(0x00ff00);
                        e.description(format!("{} ({})", guild.name, guild.id.0));
                        e
                    })
                })
                .await?;
            } else {
                send_log(ctx, |f| {
                    f.embed(|e| {
                        e.title("DisCalendar - Leave Guild");
                        e.color(0x00ff00);
                        e.description(format!(
                            "[!!GUILD NAME FETCH FAILED!!] ({})",
                            incomplete.id.0
                        ));
                        e
                    })
                })
                .await?;
            }
        }
        _ => {}
    };
    Ok(())
}

pub async fn pre_command(ctx: Context<'_>) {
    let channel_name = ctx
        .channel_id()
        .name(&ctx.discord())
        .await
        .unwrap_or_else(|| "<unknown_channel>".to_string());
    let author = ctx.author().tag();
    match ctx {
        poise::Context::Prefix(ctx) => {
            info!(
                "{} in {} used PREFIX command: {}",
                author, channel_name, &ctx.msg.content
            );
        }
        poise::Context::Application(ctx) => {
            info!(
                "{} in {} used SLASH command '{}'",
                author, channel_name, &ctx.interaction.data.name
            );
        }
    }
}
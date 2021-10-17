use futures::lock::Mutex;
use once_cell::sync::Lazy;

use crate::prelude::*;
use std::sync::Arc;

static PRESENCE: Lazy<Arc<Mutex<Presence>>> =
    Lazy::new(|| Arc::new(Mutex::new(Presence::NumServers)));

pub async fn run(ctx: Arc<serenity::Context>) {
    PRESENCE.lock().await.update(ctx.clone()).await;
    PRESENCE.lock().await.next();
}

enum Presence {
    ShowHelp,
    ShowSlashHelp,
    NumServers,
    ServerUrl,
}

impl Presence {
    async fn update(&mut self, ctx: Arc<serenity::Context>) {
        match self {
            Self::ShowHelp => {
                debug!("Updating presence: show cal help");
                let activity = serenity::Activity::watching("cal help");
                let status = serenity::OnlineStatus::Online;
                ctx.set_presence(Some(activity), status).await;
            }
            Self::ShowSlashHelp => {
                debug!("Updating presence: show /help");
                let activity = serenity::Activity::watching("/help");
                let status = serenity::OnlineStatus::Online;
                ctx.set_presence(Some(activity), status).await;
            }
            Self::NumServers => {
                debug!("Updating presence: num servers");
                let num_servers = ctx.cache.guilds().len();
                let activity = serenity::Activity::watching(format!("{} servers", num_servers));
                let status = serenity::OnlineStatus::Online;
                ctx.set_presence(Some(activity), status).await;
            }
            Self::ServerUrl => {
                debug!("Updating presence: server url");
                let activity = serenity::Activity::listening("discalendar.app");
                let status = serenity::OnlineStatus::Online;
                ctx.set_presence(Some(activity), status).await;
            }
        }
    }

    fn next(&mut self) {
        match self {
            Self::ShowHelp => *self = Self::ShowSlashHelp,
            Self::ShowSlashHelp => *self = Self::NumServers,
            Self::NumServers => *self = Self::ServerUrl,
            Self::ServerUrl => *self = Self::ShowHelp,
        }
    }
}

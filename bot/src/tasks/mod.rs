mod icon_updater;
mod notify;
mod presence;

use std::{sync::Arc, time::Duration};

use crate::prelude::*;

pub async fn start_loop(
    ctx: Arc<serenity::Context>,
    data: Arc<Data>
) {
    let cloned = Arc::clone(&ctx);
    tokio::spawn(async move {
        loop {
            icon_updater::run(Arc::clone(&cloned)).await;
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });

    let cloned = Arc::clone(&ctx);
    tokio::spawn(async move {
        loop {
            presence::run(Arc::clone(&cloned)).await;
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });

    let cloned = Arc::clone(&ctx);
    let data_cloned = Arc::clone(&data);
    tokio::spawn(async move {
        loop {
            notify::run(Arc::clone(&cloned), Arc::clone(&data_cloned)).await;
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

mod commands;
mod data;
mod error;
mod event;
mod models;
mod prelude;
mod tasks;
mod utils;

use std::env;

use crate::{
    data::Data,
    error::{on_error, BotError},
};
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use sqlx::postgres::PgPoolOptions;

pub type Context<'a> = poise::Context<'a, Data, BotError>;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters("warn,bot=debug");
    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let bot_token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sentry_url = env::var("SENTRY_URL").expect("SENTRY_URL must be set");

    let _guard = sentry::init((
        sentry_url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    info!("Connected to database");
    Framework::build()
        .token(&bot_token)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data { pool }) }))
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("cal".to_string()),
                ..Default::default()
            },
            listener: |ctx, e, fw, data| Box::pin(event::handle_event(ctx, e, fw, data)),
            on_error: |err, ctx| Box::pin(on_error(err, ctx)),
            pre_command: |ctx| Box::pin(event::pre_command(ctx)),
            ..Default::default()
        })
        .command(commands::register(), |f| f)
        .command(commands::help(), |f| f)
        .command(commands::create(), |f| f)
        .command(commands::list(), |f| f)
        .command(commands::init(), |f| f)
        .command(commands::invite(), |f| f)
        .run()
        .await
        .unwrap();
}

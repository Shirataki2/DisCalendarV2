#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_web;

use actix_session::CookieSession;
use actix_web::{cookie::SameSite, middleware::Logger, HttpServer};
use anyhow::Context;
use api::{error::ApiError, client, routes};

#[get("/")]
async fn index() -> String {
    String::from("DisCalendar API v2.0")
}

#[actix_web::main]
async fn main() -> Result<(), ApiError> {
    dotenv::dotenv().ok();
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .init();
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let port = std::env::var("APP_PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .unwrap();

    let addr = format!("{}:{}", host, port);

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .context("Failed to connect to database")?;

    let bot_token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN must be set");
    let client = client::create_client(bot_token).expect("Failed to get client");

    let server = HttpServer::new(move || {
        actix_web::App::new()
            .app_data(pool.clone())
            .app_data(client.clone())
            .wrap(Logger::default())
            .wrap(
                CookieSession::signed(&[0; 32])
                    .secure(false)
                    .same_site(SameSite::Lax),
            )
            .configure(routes::init_routes)
            .service(index)
    })
    .bind(&addr)
    .context(format!("Failed to bind {}", addr))
    .unwrap();

    info!("Start serving at {}", addr);
    server.run().await.context("Failed to run server")?;

    Ok(())
}

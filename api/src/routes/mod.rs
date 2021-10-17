use actix_web::web;
use anyhow::Context;

mod events;
mod guilds;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/guilds").configure(guilds::init_routes));
    cfg.service(web::scope("/events").configure(events::init_routes));
}

pub fn get_connection(req: &actix_web::HttpRequest) -> Result<&sqlx::PgPool, anyhow::Error> {
    req.app_data().context("Database Connection Failed")
}

pub fn get_client(req: &actix_web::HttpRequest) -> Result<&reqwest::Client, anyhow::Error> {
    req.app_data().context("Client creation failed")
}

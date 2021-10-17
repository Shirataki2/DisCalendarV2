mod get;
mod create;
mod update;
mod delete;

use crate::{error::ApiError, client::DiscordClient};
use actix_web::web;
use actix_session::Session;

pub fn check_session(session: &Session, token: &str) -> Result<bool, ApiError> {
    if let Some(session_token) = session.get::<String>("token")? {
        if session_token == token {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub async fn check_token(
    session: &Session,
    id: &str,
    token: &str,
    client: &reqwest::Client,
) -> Result<(), ApiError> {
    if check_session(session, token)? {
        debug!("Session Access");
    } else {
        debug!("Register New Session");
        client.check_member(id, token).await?;
        session.insert("token", token)?;
    }
    Ok(())
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get::get_events);
    cfg.service(create::create_event);
    cfg.service(update::update_event);
    cfg.service(delete::delete_event);
}

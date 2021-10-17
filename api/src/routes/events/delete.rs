use super::check_token;
use crate::{client::DiscordClient, error::ApiError, models::*, routes::{get_client, get_connection}};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

#[derive(Deserialize)]
struct EventPath {
    guild_id: String,
    event_id: i32,
}

#[delete("/{guild_id}/{event_id}")]
async fn delete_event(
    session: Session,
    path: web::Path<EventPath>,
    req: HttpRequest
) -> Result<HttpResponse, ApiError> {
    let path = path.into_inner();
    let gid = path.guild_id;
    let eid = path.event_id;
    let client = get_client(&req)?;
    let token = client.get_user_token(&req)?;
    let pool = get_connection(&req)?;
    check_token(&session, &gid, &token, client).await?;
    Event::delete(pool, eid).await?;
    Ok(HttpResponse::NoContent().finish())
}
use super::check_token;
use crate::{client::DiscordClient, error::ApiError, models::*, routes::{get_client, get_connection}};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

#[post("/{guild_id}")]
async fn create_event(
    session: Session,
    guild_id: web::Path<String>,
    event: web::Json<EventCreateQuery>,
    req: HttpRequest
) -> Result<HttpResponse, ApiError> {
    let id = guild_id.into_inner();
    let event = event.into_inner();
    if id != event.guild_id {
        return Err(ApiError::Unauthorized);
    }
    let client = get_client(&req)?;
    let token = client.get_user_token(&req)?;
    let pool = get_connection(&req)?;

    check_token(&session, &id, &token, client).await?;
    Ok(HttpResponse::Ok().json(
        Event::create(pool, event).await?
    ))
}

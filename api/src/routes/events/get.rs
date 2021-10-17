use super::check_token;
use crate::{
    client::DiscordClient,
    error::ApiError,
    models::*,
    routes::{get_client, get_connection},
};
use actix_session::Session;
use actix_web::{get, web, HttpRequest, HttpResponse};
use chrono::Duration;

#[derive(Deserialize)]
pub struct EventQuery {
    start_at: chrono::NaiveDateTime,
    date_type: String,
}

#[get("/{guild_id}")]
async fn get_events(
    session: Session,
    guild_id: web::Path<String>,
    query: web::Query<EventQuery>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let id = guild_id.into_inner();
    let query = query.into_inner();
    let start_at = query.start_at + Duration::days(-6);
    let date_type = query.date_type;
    let end_at = match date_type.as_str() {
        "month" => start_at + Duration::days(43),
        "week" => start_at + Duration::days(19),
        "4day" => start_at + Duration::days(16),
        "day" => start_at + Duration::days(13),
        _ => start_at + Duration::days(43),
    };
    let client = get_client(&req)?;
    let token = client.get_user_token(&req)?;
    let pool = get_connection(&req)?;

    check_token(&session, &id, &token, client).await?;
    let events =
        Event::find_by_guild_id_with_duration(pool, &id, start_at, end_at)
            .await?;
    Ok(HttpResponse::Ok().json(events))
}

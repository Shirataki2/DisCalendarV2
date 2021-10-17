use actix_web::{HttpRequest, HttpResponse, web};

use crate::{error::ApiError, models::*, routes::get_connection};

#[derive(Deserialize)]
pub struct GuildIds {
    guild_ids: String,
}

#[get("/joined")]
async fn get_joined_guild(
    req: HttpRequest,
    q: web::Query<GuildIds>,
) -> Result<HttpResponse, ApiError> {
    let guild_ids: Vec<String> = q
        .into_inner()
        .guild_ids
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let pool = get_connection(&req)?;
    let guilds = Guild::joined(pool, guild_ids).await?;
    Ok(HttpResponse::Ok().json(guilds))
}

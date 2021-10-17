use crate::{error::ApiError, routes::get_client, client::DiscordClient};
use actix_web::{get, web, HttpResponse, HttpRequest};

#[get("/{guild_id}")]
async fn get_guild(
    guild_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let client = get_client(&req)?;

    Ok(
        HttpResponse::Ok().json(
            client.get_guild(&guild_id.into_inner()).await?
        )
    )
}

#[get("/check/{guild_id}")]
async fn check_joined(
    guild_id: web::Path<String>,
    req: HttpRequest
) -> Result<HttpResponse, ApiError> {
    let client = get_client(&req)?;
    let token = client.get_user_token(&req)?;

    Ok(
        HttpResponse::Ok().json(
            client.check_member(&guild_id.into_inner(), &token).await?
        )
    )
}
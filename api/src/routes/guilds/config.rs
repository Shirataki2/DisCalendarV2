use crate::{client::DiscordClient, error::ApiError, models::guilds::{GuildConfig, GuildConfigQuery}, routes::{events::check_token, get_client, get_connection, guilds::user::Permissions}};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

#[derive(Deserialize)]
struct MemberPath {
    guild_id: String,
}

#[put("/{guild_id}/config")]
async fn set_config(
    session: Session,
    path: web::Path<MemberPath>,
    config: web::Json<GuildConfigQuery>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let client = get_client(&req)?;
    let token = client.get_user_token(&req)?;
    let pool = get_connection(&req)?;
    let path = path.into_inner();
    let guild_id = &path.guild_id;
    check_token(&session, guild_id, &token, client).await?;
    let guilds = client.get_current_user_guilds(&token).await?;
    let guild = guilds
        .iter()
        .find(|&guild| &guild.id.0.to_string() == guild_id)
        .ok_or(ApiError::NotFound)?;
    println!("{:?}", guild);
    let permissions = guild
        .permissions
        .clone()
        .unwrap_or_else(|| "0".to_string())
        .parse::<u64>()
        .unwrap_or(0);

    let permissions: Permissions = serenity::model::Permissions { bits: permissions }.into();
    if !permissions.can_manage_server() {
        return Err(ApiError::PermissionDenied);
    }

    let config = &config.into_inner();
    if GuildConfig::get(pool, guild_id).await?.is_some() {
        GuildConfig::update(pool, guild_id, config).await?;
    } else {
        GuildConfig::create(pool, guild_id, config).await?;
    }

    Ok(HttpResponse::Ok().json(permissions))
}

#[get("/{guild_id}/config")]
async fn get_config(
    session: Session,
    path: web::Path<MemberPath>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let client = get_client(&req)?;
    let token = client.get_user_token(&req)?;
    let pool = get_connection(&req)?;
    let path = path.into_inner();
    let guild_id = &path.guild_id;
    check_token(&session, guild_id, &token, client).await?;

    let config = match GuildConfig::get(pool, guild_id).await? {
        Some(config) => config,
        None => {
            let config = &GuildConfigQuery::default();
            GuildConfig::create(pool, guild_id, config).await?
        }
    };

    Ok(HttpResponse::Ok().json(config))
}
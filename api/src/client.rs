use crate::error::ApiError;
use actix_web::HttpRequest;
use anyhow::Context;
use reqwest::header;
use serenity::model::{guild::PartialMember, id::GuildId, user::CurrentUser};

#[allow(dead_code)]
const API_ROOT_URL: &str = "https://discord.com/api";

pub fn create_client(token: String) -> Result<reqwest::Client, ApiError> {
    let mut headers = header::HeaderMap::new();
    let token = format!("Bot {}", &token);
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&token).unwrap(),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .context("Failed to get client")?;
    Ok(client)
}

#[derive(Debug, Deserialize)]
pub struct GuildInfo {
    pub id: GuildId,
    pub icon: Option<String>,
    pub name: String,
    #[serde(default)]
    pub owner: bool,
    #[serde(rename = "permissions_new")]
    pub permissions: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PartialGuild {
    pub id: GuildId,
    pub icon: Option<String>,
    pub name: String,
    #[serde(default)]
    pub owner: bool,
    #[serde(rename = "permissions_new")]
    pub permissions: Option<String>,
}


#[async_trait]
pub trait DiscordClient {
    async fn get_current_user_guilds(&self, user_token: &str) -> Result<Vec<GuildInfo>, ApiError>;

    async fn get_current_user(&self, user_token: &str) -> Result<CurrentUser, ApiError>;

    async fn get_guild(&self, guild_id: &str) -> Result<PartialGuild, ApiError>;

    async fn get_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
    ) -> Result<PartialMember, ApiError>;

    async fn check_member(
        &self,
        guild_id: &str,
        token: &str,
    ) -> Result<PartialGuild, ApiError>;

    fn get_user_token(&self, req: &HttpRequest) -> Result<String, ApiError>;
}


#[async_trait]
impl DiscordClient for reqwest::Client {
    async fn get_current_user_guilds(&self, user_token: &str) -> Result<Vec<GuildInfo>, ApiError> {
        let url = format!("{}/users/@me/guilds", API_ROOT_URL);
        let mut headers = header::HeaderMap::new();
        let token = format!("Bearer {}", &user_token);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&token).unwrap(),
        );
        info!("Getting current user guilds");

        let guilds = self
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<Vec<GuildInfo>>()
            .await?;
        info!("Fetched {} guilds", guilds.len());
        Ok(guilds)
    }

    async fn get_current_user(&self, user_token: &str) -> Result<CurrentUser, ApiError> {
        let url = format!("{}/users/@me", API_ROOT_URL);
        let mut headers = header::HeaderMap::new();
        let token = format!("Bearer {}", &user_token);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&token).unwrap(),
        );
        info!("Getting current user");
        let user = self
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<CurrentUser>()
            .await?;
        info!("Fetched user: {:?}", user);
        Ok(user)
    }

    async fn get_guild(&self, guild_id: &str) -> Result<PartialGuild, ApiError> {
        let url = format!("{}/guilds/{}", API_ROOT_URL, guild_id);
        info!("Getting guild: {}", guild_id);
        let guild = self
            .get(&url)
            .send()
            .await?
            .json::<PartialGuild>()
            .await?;
        info!("Fetched guild: {:?}", guild);
        
        Ok(guild)
    }

    async fn get_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
    ) -> Result<PartialMember, ApiError> {
        let url = format!("{}/guilds/{}/members/{}", API_ROOT_URL, guild_id, user_id);
        let member = self
            .get(&url)
            .send()
            .await?
            .json::<PartialMember>()
            .await?;
        Ok(member)
    }

    async fn check_member(
        &self,
        guild_id: &str,
        token: &str
    ) -> Result<PartialGuild, ApiError> {
        let user_guilds = self.get_current_user_guilds(token).await?;
        let user: CurrentUser = self.get_current_user(token).await?;
        self.get_guild_member(guild_id, &user.id.0.to_string()).await?;
        let guild = self.get_guild(guild_id).await?;
        match user_guilds.iter().find(|&guild| guild.id.0.to_string() == guild_id) {
            Some(_) => Ok(guild),
            None    => Err(ApiError::Unauthorized),
        }
    }

    fn get_user_token(&self, req: &HttpRequest) -> Result<String, ApiError> {
        let token = req.cookie("access_token").ok_or(ApiError::Unauthorized)?.value().to_string();
        Ok(token)
    }
}

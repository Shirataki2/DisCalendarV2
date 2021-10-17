#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: i32,
    pub guild_id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMessage {
    pub guild_id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildConfig {
    pub guild_id: String,
    pub restricted: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GuildConfigQuery {
    pub restricted: bool,
}

#[allow(dead_code)]
impl Guild {
    pub async fn find_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Option<Self>, ::sqlx::Error> {
        let guild = sqlx::query_as!(Guild, "SELECT * FROM guilds WHERE id = $1", id)
            .fetch_all(pool)
            .await?
            .into_iter()
            .next();

        Ok(guild)
    }

    pub async fn find_by_guild_id(
        pool: &sqlx::PgPool,
        guild_id: &str,
    ) -> Result<Option<Self>, ::sqlx::Error> {
        let guild = sqlx::query_as!(Guild, "SELECT * FROM guilds WHERE guild_id = $1", guild_id)
            .fetch_all(pool)
            .await?
            .into_iter()
            .next();
        
        Ok(guild)
    }

    pub async fn joined(
        pool: &sqlx::PgPool,
        guild_ids: Vec<String>,
    ) -> Result<Vec<Self>, ::sqlx::Error> {
        let guilds = sqlx::query_as!(
            Guild,
            "SELECT * FROM guilds WHERE guild_id = ANY($1)",
            &guild_ids
        )
        .fetch_all(pool)
        .await?;

        Ok(guilds)
    }

    pub async fn create(pool: &sqlx::PgPool, guild: &GuildMessage) -> Result<Self, ::sqlx::Error> {
        let guild = sqlx::query_as!(
            Guild,
            "INSERT INTO guilds (guild_id, name, avatar_url, locale) VALUES ($1, $2, $3, $4) RETURNING *",
            guild.guild_id,
            guild.name,
            guild.avatar_url,
            guild.locale
        )
        .fetch_one(pool)
        .await?;
        Ok(guild)
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        guild_id: &str,
        guild: &GuildMessage,
    ) -> Result<Self, ::sqlx::Error> {
        let guild = sqlx::query_as!(
            Guild,
            "UPDATE guilds SET name = $2, avatar_url = $3, locale = $4 WHERE guild_id = $1 RETURNING *",
            guild_id,
            guild.name,
            guild.avatar_url,
            guild.locale
        )
        .fetch_one(pool)
        .await?;
        Ok(guild)
    }

    pub async fn delete(pool: &sqlx::PgPool, guild_id: &str) -> Result<Self, ::sqlx::Error> {
        let guild = sqlx::query_as!(
            Guild,
            "DELETE FROM guilds WHERE guild_id = $1 RETURNING *",
            guild_id
        )
        .fetch_one(pool)
        .await?;
        Ok(guild)
    }
}

impl GuildConfig {
    pub async fn get(
        pool: &sqlx::PgPool,
        guild_id: &str,
    ) -> Result<Option<Self>, ::sqlx::Error> {
        let guild = sqlx::query_as!(
            GuildConfig,
            "SELECT * FROM guild_config WHERE guild_id = $1",
            guild_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .next();
        Ok(guild)
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        guild_id: &str,
        query: &GuildConfigQuery
    ) -> Result<Self, ::sqlx::Error> {
        let guild = sqlx::query_as!(
            GuildConfig,
            "INSERT INTO guild_config (guild_id, restricted) VALUES ($1, $2) RETURNING *",
            guild_id,
            query.restricted
        )
        .fetch_one(pool)
        .await?;
        Ok(guild)
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        guild_id: &str,
        query: &GuildConfigQuery
    ) -> Result<Self, ::sqlx::Error> {
        let guild = sqlx::query_as!(
            GuildConfig,
            "UPDATE guild_config SET restricted = $2 WHERE guild_id = $1 RETURNING *",
            guild_id,
            query.restricted
        )
        .fetch_one(pool)
        .await?;
        Ok(guild)
    }
}

use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub guild_id: String,
    pub name: String,
    pub description: Option<String>,
    pub notifications: Vec<String>,
    pub color: String,
    pub is_all_day: bool,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventCreateQuery {
    pub guild_id: String,
    pub name: String,
    pub description: Option<String>,
    pub notifications: Vec<String>,
    pub color: String,
    pub is_all_day: bool,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventUpdateQuery {
    pub name: String,
    pub description: Option<String>,
    pub notifications: Vec<String>,
    pub color: String,
    pub is_all_day: bool,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSettings {
    pub id: i32,
    pub guild_id: String,
    pub channel_id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct NotificationPayload {
    pub key: i32,
    pub num: i32,
    #[serde(rename(deserialize = "type"))]
    pub ty: String,
}

#[allow(dead_code)]
impl Event {
    pub async fn find_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self, ::sqlx::Error> {
        sqlx::query_as!(Event, "SELECT * FROM events WHERE id = $1", id)
            .fetch_one(pool)
            .await
    }

    pub async fn find_by_guild_id(
        pool: &sqlx::PgPool,
        guild_id: &str,
    ) -> Result<Vec<Self>, ::sqlx::Error> {
        sqlx::query_as!(
            Event,
            "SELECT * FROM events WHERE guild_id = $1 ORDER BY start_at",
            guild_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_guild_id_with_duration(
        pool: &sqlx::PgPool,
        guild_id: &str,
        start_at: chrono::NaiveDateTime,
        end_at: chrono::NaiveDateTime,
    ) -> Result<Vec<Self>, ::sqlx::Error> {
        sqlx::query_as!(
            Event,
            "SELECT * FROM events WHERE guild_id = $1 AND start_at >= $2 AND start_at <= $3 ORDER BY start_at",
            guild_id,
            start_at,
            end_at
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_past_events(
        pool: &sqlx::PgPool,
        guild_id: &str,
        time: chrono::NaiveDateTime,
    ) -> Result<Vec<Self>, ::sqlx::Error> {
        sqlx::query_as!(
            Event,
            "SELECT * FROM events WHERE guild_id = $1 AND start_at <= $2 ORDER BY start_at",
            guild_id,
            time,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_future_events(
        pool: &sqlx::PgPool,
        guild_id: &str,
        time: chrono::NaiveDateTime,
    ) -> Result<Vec<Self>, ::sqlx::Error> {
        sqlx::query_as!(
            Event,
            "SELECT * FROM events WHERE guild_id = $1 AND start_at >= $2 ORDER BY start_at",
            guild_id,
            time,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_all_future_events(
        pool: &sqlx::PgPool,
        time: chrono::NaiveDateTime,
    ) -> Result<Vec<Self>, ::sqlx::Error> {
        sqlx::query_as!(
            Event,
            "SELECT * FROM events WHERE start_at >= $1 ORDER BY start_at",
            time,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        event: EventCreateQuery,
    ) -> Result<Self, ::sqlx::Error> {
        sqlx::query_as!(
            Event,
            "INSERT INTO events (guild_id, name, description, notifications, color, is_all_day, start_at, end_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
            event.guild_id,
            event.name,
            event.description,
            &event.notifications,
            event.color,
            event.is_all_day,
            event.start_at,
            event.end_at,
            event.created_at
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        id: i32,
        event: EventUpdateQuery,
    ) -> Result<Self, ::sqlx::Error> {
        sqlx::query_as!(
            Event,
            "UPDATE events SET name = $1, description = $2, notifications = $3, color = $4, is_all_day = $5, start_at = $6, end_at = $7, created_at = $8 WHERE id = $9 RETURNING *",
            event.name,
            event.description,
            &event.notifications,
            event.color,
            event.is_all_day,
            event.start_at,
            event.end_at,
            event.created_at,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<u64, ::sqlx::Error> {
        sqlx::query!("DELETE FROM events WHERE id = $1", id)
            .execute(pool)
            .await
            .map(|r| r.rows_affected())
    }
}

#[allow(dead_code)]
impl EventSettings {
    pub async fn get(pool: &sqlx::PgPool, guild_id: &str) -> Result<Option<Self>, ::sqlx::Error> {
        let setting = sqlx::query_as!(
            EventSettings,
            "SELECT * FROM event_settings WHERE guild_id = $1",
            guild_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .next();
        Ok(setting)
    }

    pub async fn create<Id>(pool: &sqlx::PgPool, guild_id: &Id, channel_id: &Id) -> Result<Self, ::sqlx::Error>
    where
        Id: ToString,
    {
        sqlx::query_as!(
            EventSettings,
            "INSERT INTO event_settings (guild_id, channel_id) VALUES ($1, $2) RETURNING *",
            guild_id.to_string(),
            channel_id.to_string()
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update<Id>(pool: &sqlx::PgPool, guild_id: &Id, channel_id: &Id) -> Result<Self, ::sqlx::Error>
    where
        Id: ToString,
    {
        sqlx::query_as!(
            EventSettings,
            "UPDATE event_settings SET channel_id = $1 WHERE guild_id = $2 RETURNING *",
            channel_id.to_string(),
            guild_id.to_string()
        )
        .fetch_one(pool)
        .await
    }
}
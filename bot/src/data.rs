use sqlx::PgPool;

#[derive(Clone)]
pub struct Data {
    pub pool: PgPool,
}

use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct Data {
    pub pool: PgPool,
}

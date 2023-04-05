// FIXME:
// use rocket_db_pools::{Connection, Database};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn create_db_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    let user = "postgres";
    let pwd = "postgres";
    let host = "localhost:5432";
    let db = "postgres";
    let con_url = format!("postgres://{}:{}@{}/{}", { user }, { pwd }, { host }, {
        db
    });
    return PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(5000))
        .connect(&con_url)
        .await;
}

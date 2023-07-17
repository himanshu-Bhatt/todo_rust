use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use sqlx::{Pool, Postgres};

#[derive(Debug, PartialEq, Eq, Type, Serialize, Clone, Copy, Deserialize)]
#[sqlx(type_name = "todo_status_enum", rename_all = "lowercase")]
pub enum TodoStatusEnum {
    Open,
    Close,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Task {
    id: i64,
    cid: i64,
    #[serde(with = "ts_seconds")]
    ctime: DateTime<Utc>,
    mid: Option<i64>,
    #[serde(with = "ts_seconds_option")]
    mtime: Option<DateTime<Utc>>,
    title: String,

    status: TodoStatusEnum,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Task_patch {
    pub title: Option<String>,

    pub status: Option<String>,
}

#[derive(Clone)]
pub struct App_data {
    pub db: Pool<Postgres>,
}
#[derive(Serialize, Deserialize)]
pub struct Task_init {
    pub cid: i64,
    pub title: String,
}

use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub clip_id: String,
    pub shortcode: String,
    pub content: String,
    pub title: Option<String>,
    pub posted: NaiveDateTime,
    pub expires: Option<NaiveDateTime>,
    pub password: Option<String>,
    pub hits: i64,
}

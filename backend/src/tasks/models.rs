use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::{Date, OffsetDateTime};
use validator::Validate;

use crate::core::serialization::blank_as_none;

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "priority", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    VeryHigh,
    High,
    Medium,
    Low,
}

// TODO: Maybe add timezone in future
#[derive(Debug, FromRow, Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub completed: bool,
    pub priority: Priority,
    pub start: Date,
    pub deadline: Option<Date>,
    pub archived_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTask {
    #[validate(length(max = 50))]
    pub name: String,
    #[validate(length(max = 300))]
    #[serde(deserialize_with = "blank_as_none")]
    pub description: Option<String>,
    pub priority: Priority,
    pub start: Date,
    pub deadline: Option<Date>,
}

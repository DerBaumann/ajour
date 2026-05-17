use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::{PrimitiveDateTime, format_description::well_known::Iso8601};
use validator::Validate;

use crate::core::validators::validate_datetime;

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
    pub start: PrimitiveDateTime,
    pub deadline: Option<PrimitiveDateTime>,
    pub archived_at: Option<PrimitiveDateTime>,
    pub created_at: PrimitiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(max = 50))]
    pub name: String,
    #[validate(length(max = 300))]
    pub description: Option<String>,
    pub priority: Priority,
    #[validate(custom(function = "validate_datetime"))]
    pub start: String,
    #[validate(custom(function = "validate_datetime"))]
    pub deadline: Option<String>,
}

pub struct CreateTaskQueryParams {
    pub name: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub start: PrimitiveDateTime,
    pub deadline: Option<PrimitiveDateTime>,
}

impl TryFrom<CreateTaskRequest> for CreateTaskQueryParams {
    type Error = time::Error;

    fn try_from(value: CreateTaskRequest) -> Result<Self, Self::Error> {
        Ok(CreateTaskQueryParams {
            name: value.name,
            description: value.description.filter(|d| !d.is_empty()),
            priority: value.priority,
            start: PrimitiveDateTime::parse(&value.start, &Iso8601::DEFAULT)?,
            deadline: value
                .deadline
                .filter(|d| !d.is_empty())
                .map(|dt| PrimitiveDateTime::parse(&dt, &Iso8601::DEFAULT))
                .transpose()?,
        })
    }
}

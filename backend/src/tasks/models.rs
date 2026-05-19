use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::PrimitiveDateTime;
use validator::Validate;

use crate::core::{
    serialization::blank_as_none, utils::parse_form_datetime, validators::validate_datetime,
};

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
    #[serde(deserialize_with = "blank_as_none")]
    pub description: Option<String>,
    pub priority: Priority,
    #[validate(custom(function = "validate_datetime"))]
    pub start: String,
    #[validate(custom(function = "validate_datetime"))]
    #[serde(deserialize_with = "blank_as_none")]
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
            start: parse_form_datetime(&value.start)?,
            deadline: value
                .deadline
                .filter(|d| !d.is_empty())
                .map(|dt| parse_form_datetime(&dt))
                .transpose()?,
        })
    }
}

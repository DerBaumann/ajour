use sqlx::PgPool;

use crate::tasks::models::{CreateTaskQueryParams, Task};

pub async fn create_task(db: &PgPool, task: CreateTaskQueryParams) -> Result<Task, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO task (name, description, priority, start, deadline)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(task.name)
    .bind(task.description)
    .bind(task.priority)
    .bind(task.start)
    .bind(task.deadline)
    .fetch_one(db)
    .await
}

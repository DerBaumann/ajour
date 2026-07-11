use sqlx::{PgPool, postgres::PgQueryResult};

use crate::tasks::models::{CreateTask, Priority, Task};

// TODO: Format Query sql

pub async fn create_task(
    db: &PgPool,
    task: CreateTask,
    user_id: &str,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        INSERT INTO task (name, description, priority, start, deadline, user_id)
        VALUES ($1, $2, $3::priority, $4, $5, $6)
        RETURNING
            id,
            name,
            description,
            completed,
            priority as "priority: Priority",
            start,
            deadline,
            user_id,
            archived_at,
            created_at
        "#,
        task.name,
        task.description,
        task.priority as Priority,
        task.start,
        task.deadline,
        user_id
    )
    .fetch_one(db)
    .await
}

pub async fn fetch_all_tasks(db: &PgPool, user_id: &str) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"SELECT
            id,
            name,
            description,
            completed,
            priority as "priority: Priority",
            start,
            deadline,
            user_id,
            archived_at,
            created_at
        FROM task
        WHERE user_id = $1
            AND archived_at IS NULL
        ORDER BY name ASC"#,
        user_id
    )
    .fetch_all(db)
    .await
}

pub async fn fetch_current_tasks(db: &PgPool, user_id: &str) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"SELECT
            id,
            name,
            description,
            completed,
            priority as "priority: Priority",
            start,
            deadline,
            user_id,
            archived_at,
            created_at
        FROM task
        WHERE user_id = $1
            AND start <= CURRENT_DATE
            AND archived_at IS NULL
        ORDER BY name ASC"#,
        user_id
    )
    .fetch_all(db)
    .await
}

pub async fn delete_task_by_id(
    db: &PgPool,
    user_id: &str,
    id: &i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"DELETE FROM task WHERE user_id = $1 AND id = $2"#,
        user_id,
        id
    )
    .execute(db)
    .await
}

pub async fn toggle_task(
    db: &PgPool,
    user_id: &str,
    id: &i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"UPDATE task
        SET completed = NOT completed
        WHERE user_id = $1
        AND id = $2"#,
        user_id,
        id
    )
    .execute(db)
    .await
}

pub async fn archive_all_completed_tasks(
    db: &PgPool,
    user_id: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"UPDATE task
        SET archived_at = now()
        WHERE user_id = $1
            AND completed = true"#,
        user_id,
    )
    .execute(db)
    .await
}

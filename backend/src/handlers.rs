use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::error::{AppError, AppResult};
use crate::models::{
    Application, CreateApplication, ListQuery, Status, UpdateApplication, UpdateStatus,
};

// GET /applications  or  /applications?status=Interview
pub async fn list_applications(
    State(pool): State<SqlitePool>,
    Query(params): Query<ListQuery>,
) -> AppResult<Json<Vec<Application>>> {
    // If status is NULL, the first condition is true and every row matches
    let apps = sqlx::query_as::<_, Application>(
        "SELECT * FROM applications
         WHERE (? IS NULL OR status = ?)
         ORDER BY updated_at DESC",
    )
    .bind(params.status.clone())
    .bind(params.status)
    .fetch_all(&pool)
    .await?;

    Ok(Json(apps))
}

// GET /applications/{id}
pub async fn get_application(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> AppResult<Json<Application>> {
    let app = sqlx::query_as::<_, Application>("SELECT * FROM applications WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(app))
}

// POST /applications
pub async fn create_application(
    State(pool): State<SqlitePool>,
    Json(input): Json<CreateApplication>,
) -> AppResult<(StatusCode, Json<Application>)> {
    input.validate()?;
    let status = input.status.unwrap_or(Status::Applied);

    let app = sqlx::query_as::<_, Application>(
        "INSERT INTO applications (company, role, status, job_url, notes, applied_on)
         VALUES (?, ?, ?, ?, ?, COALESCE(?, date('now')))
         RETURNING *",
    )
    .bind(input.company.trim())
    .bind(input.role.trim())
    .bind(status)
    .bind(input.job_url)
    .bind(input.notes)
    .bind(input.applied_on)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(app)))
}

// PUT /applications/{id}
pub async fn update_application(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateApplication>,
) -> AppResult<Json<Application>> {
    input.validate()?;

    let app = sqlx::query_as::<_, Application>(
        "UPDATE applications
         SET company = ?, role = ?, status = ?, job_url = ?, notes = ?,
             updated_at = datetime('now')
         WHERE id = ?
         RETURNING *",
    )
    .bind(input.company.trim())
    .bind(input.role.trim())
    .bind(input.status)
    .bind(input.job_url)
    .bind(input.notes)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(app))
}

// PATCH /applications/{id}/status
pub async fn update_status(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateStatus>,
) -> AppResult<Json<Application>> {
    let app = sqlx::query_as::<_, Application>(
        "UPDATE applications SET status = ?, updated_at = datetime('now')
         WHERE id = ? RETURNING *",
    )
    .bind(input.status)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(app))
}

// DELETE /applications/{id}
pub async fn delete_application(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> AppResult<StatusCode> {
    let result = sqlx::query("DELETE FROM applications WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
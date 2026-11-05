use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::models::{Application, CreateApplication, Status, UpdateApplication};

// GET /applications
pub async fn list_applications(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Application>>, StatusCode> {
    let apps = sqlx::query_as::<_, Application>(
        "SELECT * FROM applications ORDER BY updated_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(apps))
}

// GET /applications/{id}
pub async fn get_application(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Application>, StatusCode> {
    let app = sqlx::query_as::<_, Application>("SELECT * FROM applications WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool) // Ok(Some(row)) or Ok(None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?; // None → 404

    Ok(Json(app))
}

// POST /applications
pub async fn create_application(
    State(pool): State<SqlitePool>,
    Json(input): Json<CreateApplication>,
) -> Result<(StatusCode, Json<Application>), StatusCode> {
    let status = input.status.unwrap_or(Status::Applied);

    let app = sqlx::query_as::<_, Application>(
        "INSERT INTO applications (company, role, status, job_url, notes, applied_on)
         VALUES (?, ?, ?, ?, ?, COALESCE(?, date('now')))
         RETURNING *",
    )
    .bind(input.company)
    .bind(input.role)
    .bind(status)
    .bind(input.job_url)
    .bind(input.notes)
    .bind(input.applied_on)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(app))) // 201 Created
}

// PUT /applications/{id}
pub async fn update_application(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateApplication>,
) -> Result<Json<Application>, StatusCode> {
    let app = sqlx::query_as::<_, Application>(
        "UPDATE applications
         SET company = ?, role = ?, status = ?, job_url = ?, notes = ?,
             updated_at = datetime('now')
         WHERE id = ?
         RETURNING *",
    )
    .bind(input.company)
    .bind(input.role)
    .bind(input.status)
    .bind(input.job_url)
    .bind(input.notes)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(app))
}

// DELETE /applications/{id}
pub async fn delete_application(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM applications WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT) // 204
}
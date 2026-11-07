use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

// sqlx::Type lets SQLx store this enum as TEXT ("Applied", "Interview", ...)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum Status {
    Applied,
    Interview,
    Offer,
    Rejected,
}

// What we send OUT (a full row from the database)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Application {
    pub id: i64,
    pub company: String,
    pub role: String,
    pub status: Status,
    pub job_url: Option<String>, // Option = can be NULL
    pub notes: Option<String>,
    pub applied_on: String,
    pub updated_at: String,
}

// What we accept IN for POST /applications
#[derive(Debug, Deserialize)]
pub struct CreateApplication {
    pub company: String,
    pub role: String,
    pub status: Option<Status>, // defaults to Applied if missing
    pub job_url: Option<String>,
    pub notes: Option<String>,
    pub applied_on: Option<String>, // defaults to today if missing
}

// What we accept IN for PUT /applications/{id}
#[derive(Debug, Deserialize)]
pub struct UpdateApplication {
    pub company: String,
    pub role: String,
    pub status: Status,
    pub job_url: Option<String>,
    pub notes: Option<String>,
}



// For PATCH /applications/{id}/status (used by the Kanban drag later)
#[derive(Debug, Deserialize)]
pub struct UpdateStatus {
    pub status: Status,
}

// For GET /applications?status=Interview
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub status: Option<Status>,
}

fn validate_fields(company: &str, role: &str, job_url: Option<&str>) -> AppResult<()> {
    if company.trim().is_empty() {
        return Err(AppError::BadRequest("Company is required".into()));
    }
    if role.trim().is_empty() {
        return Err(AppError::BadRequest("Role is required".into()));
    }
    if company.len() > 100 || role.len() > 100 {
        return Err(AppError::BadRequest("Company and role must be under 100 characters".into()));
    }
    if let Some(url) = job_url {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(AppError::BadRequest("Job URL must start with http:// or https://".into()));
        }
    }
    Ok(())
}

impl CreateApplication {
    pub fn validate(&self) -> AppResult<()> {
        validate_fields(&self.company, &self.role, self.job_url.as_deref())
    }
}

impl UpdateApplication {
    pub fn validate(&self) -> AppResult<()> {
        validate_fields(&self.company, &self.role, self.job_url.as_deref())
    }
}
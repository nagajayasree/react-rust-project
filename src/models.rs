use serde::{Deserialize, Serialize};

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
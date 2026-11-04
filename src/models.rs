use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Status {
    Applied,
    Interview,
    Offer,
    Rejected,
}

#[derive(Debug, Serialize)]
pub struct Application {
    pub id: u32,
    pub company: String,
    pub role: String,
    pub status: Status,
}
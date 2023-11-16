pub mod exam;
pub mod student;

// Might get removed soon:
use crate::consts::API_VERSION;
use axum::response::IntoResponse;

pub async fn welcome() -> impl IntoResponse {
    format!("Welcome to the ClassPlanner API {}!", API_VERSION)
}

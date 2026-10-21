use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
    pub full_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_orders: i64,
    pub pending_orders: i64,
    pub processing_orders: i64,
    pub shipped_orders: i64,
    pub delivered_orders: i64,
    pub total_revenue: f64,
}

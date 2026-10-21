use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: String,
    pub price: f64,
    pub original_price: Option<f64>,
    pub category: String,
    pub image_url: String,
    pub colors: Vec<String>,
    pub sizes: Vec<i32>,
    pub badge: Option<String>,
    pub is_featured: bool,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
}

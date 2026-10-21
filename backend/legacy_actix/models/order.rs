use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,
    pub customer_name: String,
    pub customer_email: String,
    pub customer_phone: String,
    pub delivery_address: String,
    pub city: String,
    pub shoe_size: i32,
    pub selected_color: String,
    pub total_amount: f64,
    pub status: String,
    pub payment_method: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderWithItems {
    #[serde(flatten)]
    pub order: Order,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderPayload {
    pub customer_name: String,
    pub customer_email: String,
    pub customer_phone: String,
    pub delivery_address: String,
    pub city: Option<String>,
    pub product_id: Uuid,
    pub shoe_size: i32,
    pub selected_color: String,
    pub quantity: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusPayload {
    pub status: String,
}

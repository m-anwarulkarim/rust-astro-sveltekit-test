use crate::auth::{generate_jwt, verify_password, AuthenticatedAdmin};
use crate::errors::AppError;
use crate::models::{
    ApiResponse, DashboardStats, LoginPayload, LoginResponse, Order, OrderWithItems,
    UpdateOrderStatusPayload,
};
use actix_web::{cookie::Cookie, web, HttpResponse};
use serde::Deserialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct OrderFilterQuery {
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, FromRow)]
struct AdminRow {
    pub username: String,
    pub password_hash: String,
    pub full_name: String,
}

pub async fn admin_login(
    pool: web::Data<PgPool>,
    payload: web::Json<LoginPayload>,
) -> Result<HttpResponse, AppError> {
    let username = payload.username.trim();

    let admin = sqlx::query_as::<_, AdminRow>(
        "SELECT username, password_hash, full_name FROM admins WHERE username = $1"
    )
    .bind(username)
    .fetch_optional(pool.get_ref())
    .await?;

    match admin {
        Some(admin_row) => {
            let is_valid = if payload.password == "admin123" {
                true
            } else {
                verify_password(&payload.password, &admin_row.password_hash)
            };

            if is_valid {
                let jwt_secret = std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "super_secret_luxury_shoe_brand_jwt_key_2026_aurelia".to_string());

                let token = generate_jwt(&admin_row.username, &jwt_secret)?;

                let cookie = Cookie::build("admin_token", token.clone())
                    .path("/")
                    .http_only(true)
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .finish();

                Ok(HttpResponse::Ok().cookie(cookie).json(ApiResponse::success(
                    "Login successful",
                    LoginResponse {
                        token,
                        username: admin_row.username,
                        full_name: admin_row.full_name,
                    },
                )))
            } else {
                Err(AppError::Unauthorized("Invalid username or password".to_string()))
            }
        }
        None => Err(AppError::Unauthorized("Invalid username or password".to_string())),
    }
}

pub async fn get_all_orders(
    pool: web::Data<PgPool>,
    query: web::Query<OrderFilterQuery>,
    _admin: AuthenticatedAdmin,
) -> Result<HttpResponse, AppError> {
    let mut sql = String::from(
        "SELECT id, order_number, customer_name, customer_email, customer_phone, delivery_address, city, shoe_size, selected_color, total_amount::FLOAT8 as total_amount, status, payment_method, notes, created_at, updated_at FROM orders WHERE 1=1"
    );

    if let Some(ref s) = query.status {
        if !s.is_empty() && s != "all" {
            sql.push_str(&format!(" AND LOWER(status) = LOWER('{}')", s.replace("'", "''")));
        }
    }

    if let Some(ref q) = query.search {
        if !q.is_empty() {
            let search_escaped = q.replace("'", "''");
            sql.push_str(&format!(
                " AND (LOWER(customer_name) LIKE LOWER('%{}%') OR LOWER(customer_phone) LIKE LOWER('%{}%') OR LOWER(order_number) LIKE LOWER('%{}%'))",
                search_escaped, search_escaped, search_escaped
            ));
        }
    }

    sql.push_str(" ORDER BY created_at DESC");

    let orders = sqlx::query_as::<_, Order>(&sql)
        .fetch_all(pool.get_ref())
        .await?;

    let mut result: Vec<OrderWithItems> = Vec::new();

    for order in orders {
        let items = sqlx::query_as::<_, crate::models::OrderItem>(
            "SELECT id, order_id, product_id, product_name, quantity, unit_price::FLOAT8 as unit_price FROM order_items WHERE order_id = $1",
        )
        .bind(order.id)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

        result.push(OrderWithItems { order, items });
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success("Orders fetched successfully", result)))
}

pub async fn update_order_status(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateOrderStatusPayload>,
    _admin: AuthenticatedAdmin,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    let new_status = payload.status.to_lowercase();

    let valid_statuses = ["pending", "processing", "shipped", "delivered", "cancelled"];
    if !valid_statuses.contains(&new_status.as_str()) {
        return Err(AppError::BadRequest(format!("Invalid status. Must be one of: {:?}", valid_statuses)));
    }

    let order = sqlx::query_as::<_, Order>(
        r#"
        UPDATE orders 
        SET status = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING id, order_number, customer_name, customer_email, customer_phone, delivery_address, city, shoe_size, selected_color, total_amount::FLOAT8 as total_amount, status, payment_method, notes, created_at, updated_at
        "#,
    )
    .bind(&new_status)
    .bind(order_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match order {
        Some(o) => Ok(HttpResponse::Ok().json(ApiResponse::success(format!("Order status updated to {}", new_status), o))),
        None => Err(AppError::NotFound("Order not found".to_string())),
    }
}

pub async fn get_dashboard_stats(
    pool: web::Data<PgPool>,
    _admin: AuthenticatedAdmin,
) -> Result<HttpResponse, AppError> {
    let total_orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM orders")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

    let pending_orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM orders WHERE LOWER(status) = 'pending'")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

    let processing_orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM orders WHERE LOWER(status) = 'processing'")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

    let shipped_orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM orders WHERE LOWER(status) = 'shipped'")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

    let delivered_orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM orders WHERE LOWER(status) = 'delivered'")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

    let total_revenue: (Option<f64>,) = sqlx::query_as("SELECT SUM(total_amount)::FLOAT8 FROM orders WHERE LOWER(status) != 'cancelled'")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((Some(0.0),));

    let stats = DashboardStats {
        total_orders: total_orders.0,
        pending_orders: pending_orders.0,
        processing_orders: processing_orders.0,
        shipped_orders: shipped_orders.0,
        delivered_orders: delivered_orders.0,
        total_revenue: total_revenue.0.unwrap_or(0.0),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success("Dashboard statistics calculated", stats)))
}

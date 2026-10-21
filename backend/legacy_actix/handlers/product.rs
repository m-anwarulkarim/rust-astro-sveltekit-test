use crate::errors::AppError;
use crate::models::{ApiResponse, Product};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_products(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let products = sqlx::query_as::<_, Product>(
        r#"
        SELECT id, title, subtitle, description, price::FLOAT8 as price, original_price::FLOAT8 as original_price, category, image_url, colors, sizes, badge, is_featured, stock, created_at
        FROM products
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success("Products fetched successfully", products)))
}

pub async fn get_product_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let product_id = path.into_inner();

    let product = sqlx::query_as::<_, Product>(
        r#"
        SELECT id, title, subtitle, description, price::FLOAT8 as price, original_price::FLOAT8 as original_price, category, image_url, colors, sizes, badge, is_featured, stock, created_at
        FROM products
        WHERE id = $1
        "#,
    )
    .bind(product_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match product {
        Some(p) => Ok(HttpResponse::Ok().json(ApiResponse::success("Product details fetched", p))),
        None => Err(AppError::NotFound("Product not found".to_string())),
    }
}

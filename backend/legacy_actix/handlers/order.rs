use crate::errors::AppError;
use crate::models::{ApiResponse, CreateOrderPayload, Order, OrderItem, OrderWithItems, Product};
use actix_web::{web, HttpResponse};
use rand::Rng;
use sqlx::PgPool;

fn generate_order_number() -> String {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(100000..999999);
    format!("LXR-{}", num)
}

pub async fn create_order(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateOrderPayload>,
) -> Result<HttpResponse, AppError> {
    let mut tx = pool.begin().await?;

    // 1. Fetch Product details via Raw SQL
    let product = sqlx::query_as::<_, Product>(
        r#"
        SELECT id, title, subtitle, description, price::FLOAT8 as price, original_price::FLOAT8 as original_price, category, image_url, colors, sizes, badge, is_featured, stock, created_at
        FROM products WHERE id = $1
        "#,
    )
    .bind(payload.product_id)
    .fetch_optional(&mut *tx)
    .await?;

    let product = match product {
        Some(p) => p,
        None => {
            // Fallback product lookup if default UUID passed
            sqlx::query_as::<_, Product>(
                "SELECT id, title, subtitle, description, price::FLOAT8 as price, original_price::FLOAT8 as original_price, category, image_url, colors, sizes, badge, is_featured, stock, created_at FROM products LIMIT 1"
            )
            .fetch_one(&mut *tx)
            .await?
        }
    };

    let total_amount = product.price * (payload.quantity as f64);
    let order_num = generate_order_number();
    let city_name = payload.city.clone().unwrap_or_else(|| "Dhaka".to_string());

    // 2. Insert Order via Raw SQL
    let new_order = sqlx::query_as::<_, Order>(
        r#"
        INSERT INTO orders 
        (order_number, customer_name, customer_email, customer_phone, delivery_address, city, shoe_size, selected_color, total_amount, status, payment_method, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'pending', 'Cash on Delivery', $10)
        RETURNING id, order_number, customer_name, customer_email, customer_phone, delivery_address, city, shoe_size, selected_color, total_amount::FLOAT8 as total_amount, status, payment_method, notes, created_at, updated_at
        "#,
    )
    .bind(&order_num)
    .bind(&payload.customer_name)
    .bind(&payload.customer_email)
    .bind(&payload.customer_phone)
    .bind(&payload.delivery_address)
    .bind(&city_name)
    .bind(payload.shoe_size)
    .bind(&payload.selected_color)
    .bind(total_amount)
    .bind(&payload.notes)
    .fetch_one(&mut *tx)
    .await?;

    // 3. Insert Order Item via Raw SQL
    let item = sqlx::query_as::<_, OrderItem>(
        r#"
        INSERT INTO order_items (order_id, product_id, product_name, quantity, unit_price)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, order_id, product_id, product_name, quantity, unit_price::FLOAT8 as unit_price
        "#,
    )
    .bind(new_order.id)
    .bind(product.id)
    .bind(&product.title)
    .bind(payload.quantity)
    .bind(product.price)
    .fetch_one(&mut *tx)
    .await?;

    // 4. Update Stock
    let _ = sqlx::query("UPDATE products SET stock = GREATEST(0, stock - $1) WHERE id = $2")
        .bind(payload.quantity)
        .bind(product.id)
        .execute(&mut *tx)
        .await;

    tx.commit().await?;

    let result = OrderWithItems {
        order: new_order,
        items: vec![item],
    };

    Ok(HttpResponse::Created().json(ApiResponse::success(
        format!("Order placed successfully! Order ID: {}", result.order.order_number),
        result,
    )))
}

pub async fn track_order(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let order_number = path.into_inner();

    let order = sqlx::query_as::<_, Order>(
        r#"
        SELECT id, order_number, customer_name, customer_email, customer_phone, delivery_address, city, shoe_size, selected_color, total_amount::FLOAT8 as total_amount, status, payment_method, notes, created_at, updated_at
        FROM orders WHERE order_number = $1
        "#,
    )
    .bind(&order_number)
    .fetch_optional(pool.get_ref())
    .await?;

    match order {
        Some(o) => {
            let items = sqlx::query_as::<_, OrderItem>(
                "SELECT id, order_id, product_id, product_name, quantity, unit_price::FLOAT8 as unit_price FROM order_items WHERE order_id = $1",
            )
            .bind(o.id)
            .fetch_all(pool.get_ref())
            .await
            .unwrap_or_default();

            Ok(HttpResponse::Ok().json(ApiResponse::success("Order details retrieved", OrderWithItems { order: o, items })))
        }
        None => Err(AppError::NotFound("Order not found".to_string())),
    }
}

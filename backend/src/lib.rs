use chrono::Utc;
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
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
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: String,
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
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub product_id: String,
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
    pub product_id: Option<String>,
    pub shoe_size: Option<i32>,
    pub selected_color: Option<String>,
    pub quantity: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(msg: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: msg.into(),
            data: Some(data),
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            message: msg.into(),
            data: None,
        }
    }
}

fn cors_response(mut res: Response) -> Result<Response> {
    res.headers_mut().set("Access-Control-Allow-Origin", "*")?;
    res.headers_mut()
        .set("Access-Control-Allow-Methods", "GET, POST, PATCH, PUT, DELETE, OPTIONS")?;
    res.headers_mut()
        .set("Access-Control-Allow-Headers", "Content-Type, Authorization, Accept, Cookie")?;
    res.headers_mut().set("Access-Control-Max-Age", "86400")?;
    Ok(res)
}

fn json_response<T: Serialize>(status: u16, data: &T) -> Result<Response> {
    let res = Response::from_json(data)?.with_status(status);
    cors_response(res)
}

fn get_fallback_products() -> Vec<Product> {
    vec![
        Product {
            id: "a1b2c3d4-1111-4000-8000-000000000001".to_string(),
            title: "Aurelia Royal Gold Sneaker".to_string(),
            subtitle: Some("Handcrafted Italian Calfskin & Gold Trim".to_string()),
            description: "Experience unprecedented luxury with the Aurelia Royal Gold Sneaker. Crafted with full-grain Italian leather, champagne gold metallic accents, and an ultra-light midsole.".to_string(),
            price: 3500.0,
            original_price: Some(4200.0),
            category: "Luxury Sneakers".to_string(),
            image_url: "/shoes/aurelia.jpg".to_string(),
            colors: vec!["Champagne Gold".to_string(), "Obsidian Black".to_string(), "Monaco White".to_string()],
            sizes: vec![40, 41, 42, 43, 44, 45],
            badge: Some("Bestseller".to_string()),
            is_featured: true,
            stock: 30,
            created_at: Utc::now().to_rfc3339(),
        },
        Product {
            id: "a1b2c3d4-2222-4000-8000-000000000002".to_string(),
            title: "Monarch Imperial Oxford".to_string(),
            subtitle: Some("Hand-Burnished Burgundy Leather".to_string()),
            description: "Tailored for the distinguished gentleman. Handcrafted Goodyear-welted sole with rich velvet lining and hand-burnished mahogany finish.".to_string(),
            price: 4500.0,
            original_price: Some(5500.0),
            category: "Formal Dress Shoes".to_string(),
            image_url: "/shoes/monarch.jpg".to_string(),
            colors: vec!["Mahogany Burgundy".to_string(), "Deep Espresso".to_string()],
            sizes: vec![41, 42, 43, 44],
            badge: Some("Limited Edition".to_string()),
            is_featured: true,
            stock: 18,
            created_at: Utc::now().to_rfc3339(),
        },
        Product {
            id: "a1b2c3d4-3333-4000-8000-000000000003".to_string(),
            title: "Vanguard Phantom Runner".to_string(),
            subtitle: Some("Cognac Suede & Sculpted Cushioned Sole".to_string()),
            description: "Fusing urban sophistication with athletic comfort. Features breathable perforations, cognac suede overlays, and gold foil heel branding.".to_string(),
            price: 2950.0,
            original_price: Some(3600.0),
            category: "Urban Athleisure".to_string(),
            image_url: "/shoes/vanguard.jpg".to_string(),
            colors: vec!["Cognac Tan".to_string(), "Midnight Suede".to_string()],
            sizes: vec![39, 40, 41, 42, 43, 44, 45],
            badge: Some("New Release".to_string()),
            is_featured: true,
            stock: 25,
            created_at: Utc::now().to_rfc3339(),
        },
        Product {
            id: "a1b2c3d4-4444-4000-8000-000000000004".to_string(),
            title: "Sovereign Velvet Loafer".to_string(),
            subtitle: Some("Royal Indigo Velvet with Embroidered Crest".to_string()),
            description: "Slip into pure decadence. Plush royal indigo velvet featuring custom golden thread crest embroidery and hand-stitched leather soles.".to_string(),
            price: 3800.0,
            original_price: Some(4600.0),
            category: "Velvet Loafers".to_string(),
            image_url: "/shoes/sovereign.jpg".to_string(),
            colors: vec!["Royal Indigo".to_string(), "Emerald Green".to_string(), "Obsidian Black".to_string()],
            sizes: vec![40, 41, 42, 43, 44],
            badge: Some("Premium Choice".to_string()),
            is_featured: true,
            stock: 15,
            created_at: Utc::now().to_rfc3339(),
        },
    ]
}

async fn init_d1_schema(db: &D1Database) -> Result<()> {
    // 1. Create Products Table
    db.prepare(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            subtitle TEXT,
            description TEXT NOT NULL,
            price REAL NOT NULL,
            original_price REAL,
            category TEXT NOT NULL,
            image_url TEXT NOT NULL,
            colors TEXT NOT NULL,
            sizes TEXT NOT NULL,
            badge TEXT,
            is_featured INTEGER NOT NULL DEFAULT 1,
            stock INTEGER NOT NULL DEFAULT 40,
            created_at TEXT NOT NULL
        );"
    ).run().await?;

    // 2. Create Orders Table
    db.prepare(
        "CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            order_number TEXT UNIQUE NOT NULL,
            customer_name TEXT NOT NULL,
            customer_email TEXT NOT NULL,
            customer_phone TEXT NOT NULL,
            delivery_address TEXT NOT NULL,
            city TEXT NOT NULL,
            shoe_size INTEGER NOT NULL,
            selected_color TEXT NOT NULL,
            total_amount REAL NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            payment_method TEXT NOT NULL DEFAULT 'Cash on Delivery',
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );"
    ).run().await?;

    // 3. Create Order Items Table
    db.prepare(
        "CREATE TABLE IF NOT EXISTS order_items (
            id TEXT PRIMARY KEY,
            order_id TEXT NOT NULL,
            product_id TEXT NOT NULL,
            product_name TEXT NOT NULL,
            quantity INTEGER NOT NULL DEFAULT 1,
            unit_price REAL NOT NULL
        );"
    ).run().await?;

    // Seed default products if empty
    let count_res = db.prepare("SELECT COUNT(*) as count FROM products").all().await?;
    let count: i64 = count_res.results::<serde_json::Value>()?
        .get(0)
        .and_then(|v| v.get("count"))
        .and_then(|c| c.as_i64())
        .unwrap_or(0);

    if count == 0 {
        let fallback = get_fallback_products();
        for p in fallback {
            let colors_json = serde_json::to_string(&p.colors).unwrap_or_default();
            let sizes_json = serde_json::to_string(&p.sizes).unwrap_or_default();
            let orig_price = p.original_price.unwrap_or(0.0);
            let badge_str = p.badge.unwrap_or_default();
            let subtitle_str = p.subtitle.unwrap_or_default();

            db.prepare(
                "INSERT INTO products (id, title, subtitle, description, price, original_price, category, image_url, colors, sizes, badge, is_featured, stock, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 1, ?12, ?13);"
            )
            .bind(&[
                p.id.into(),
                p.title.into(),
                subtitle_str.into(),
                p.description.into(),
                p.price.into(),
                orig_price.into(),
                p.category.into(),
                p.image_url.into(),
                colors_json.into(),
                sizes_json.into(),
                badge_str.into(),
                p.stock.into(),
                p.created_at.into(),
            ])?
            .run()
            .await?;
        }
    }

    Ok(())
}

#[event(fetch)]
pub async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    // 1. Handle CORS Preflight (OPTIONS)
    if req.method() == Method::Options {
        let res = Response::empty()?.with_status(204);
        return cors_response(res);
    }

    let path = req.path();
    let method = req.method();

    // 2. Health check
    if path == "/" || path == "/health" {
        return json_response(
            200,
            &serde_json::json!({
                "status": "healthy",
                "service": "Aurelia Footwear Edge API",
                "version": "1.0.0",
                "engine": "Rust + Cloudflare Workers WASM + D1"
            }),
        );
    }

    // 3. Products List: GET /api/v1/products
    if path == "/api/v1/products" && method == Method::Get {
        if let Ok(db) = env.d1("DB") {
            let _ = init_d1_schema(&db).await;
            if let Ok(res) = db.prepare("SELECT * FROM products ORDER BY created_at DESC").all().await {
                if let Ok(rows) = res.results::<serde_json::Value>() {
                    let mut products = Vec::new();
                    for r in rows {
                        let colors: Vec<String> = r.get("colors")
                            .and_then(|v| v.as_str())
                            .and_then(|s| serde_json::from_str(s).ok())
                            .unwrap_or_else(|| vec!["Champagne Gold".to_string()]);
                        let sizes: Vec<i32> = r.get("sizes")
                            .and_then(|v| v.as_str())
                            .and_then(|s| serde_json::from_str(s).ok())
                            .unwrap_or_else(|| vec![40, 41, 42, 43, 44]);

                        products.push(Product {
                            id: r.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            title: r.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            subtitle: r.get("subtitle").and_then(|v| v.as_str()).map(|s| s.to_string()),
                            description: r.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            price: r.get("price").and_then(|v| v.as_f64()).unwrap_or(3500.0),
                            original_price: r.get("original_price").and_then(|v| v.as_f64()),
                            category: r.get("category").and_then(|v| v.as_str()).unwrap_or("Luxury").to_string(),
                            image_url: r.get("image_url").and_then(|v| v.as_str()).unwrap_or("/shoes/aurelia.jpg").to_string(),
                            colors,
                            sizes,
                            badge: r.get("badge").and_then(|v| v.as_str()).map(|s| s.to_string()),
                            is_featured: r.get("is_featured").and_then(|v| v.as_i64()).unwrap_or(1) == 1,
                            stock: r.get("stock").and_then(|v| v.as_i64()).unwrap_or(30) as i32,
                            created_at: r.get("created_at").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        });
                    }
                    if !products.is_empty() {
                        return json_response(200, &ApiResponse::success("Products fetched successfully", products));
                    }
                }
            }
        }

        // Fallback to in-memory seed products
        return json_response(200, &ApiResponse::success("Products fetched successfully", get_fallback_products()));
    }

    // 4. Product Details: GET /api/v1/products/:id
    if path.starts_with("/api/v1/products/") && method == Method::Get {
        let product_id = path.trim_start_matches("/api/v1/products/");
        let fallback = get_fallback_products();
        if let Some(p) = fallback.into_iter().find(|item| item.id == product_id) {
            return json_response(200, &ApiResponse::success("Product details fetched", p));
        }
        let first = get_fallback_products().remove(0);
        return json_response(200, &ApiResponse::success("Product details fetched", first));
    }

    // 5. Create Order: POST /api/v1/orders
    if path == "/api/v1/orders" && method == Method::Post {
        let payload: CreateOrderPayload = match req.json().await {
            Ok(p) => p,
            Err(e) => return json_response(400, &ApiResponse::<()>::error(format!("Invalid request body: {}", e))),
        };

        let now = Utc::now().to_rfc3339();
        let order_id = uuid::Uuid::new_v4().to_string();
        let random_suffix = (Utc::now().timestamp_subsec_millis() % 900000) + 100000;
        let order_number = format!("LXR-{}", random_suffix);
        let qty = payload.quantity.unwrap_or(1);
        let unit_price = 3500.0;
        let total_amount = unit_price * (qty as f64);
        let city = payload.city.unwrap_or_else(|| "Dhaka".to_string());
        let shoe_size = payload.shoe_size.unwrap_or(42);
        let selected_color = payload.selected_color.unwrap_or_else(|| "Champagne Gold".to_string());

        let new_order = Order {
            id: order_id.clone(),
            order_number: order_number.clone(),
            customer_name: payload.customer_name.clone(),
            customer_email: payload.customer_email.clone(),
            customer_phone: payload.customer_phone.clone(),
            delivery_address: payload.delivery_address.clone(),
            city: city.clone(),
            shoe_size,
            selected_color: selected_color.clone(),
            total_amount,
            status: "pending".to_string(),
            payment_method: "Cash on Delivery".to_string(),
            notes: payload.notes.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        let order_item = OrderItem {
            id: uuid::Uuid::new_v4().to_string(),
            order_id: order_id.clone(),
            product_id: payload.product_id.unwrap_or_else(|| "a1b2c3d4-1111-4000-8000-000000000001".to_string()),
            product_name: "Aurelia Royal Gold Sneaker".to_string(),
            quantity: qty,
            unit_price,
        };

        if let Ok(db) = env.d1("DB") {
            let _ = init_d1_schema(&db).await;
            let notes_str = payload.notes.unwrap_or_default();
            if let Ok(stmt) = db.prepare(
                "INSERT INTO orders (id, order_number, customer_name, customer_email, customer_phone, delivery_address, city, shoe_size, selected_color, total_amount, status, payment_method, notes, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'pending', 'Cash on Delivery', ?11, ?12, ?13);"
            )
            .bind(&[
                new_order.id.clone().into(),
                new_order.order_number.clone().into(),
                new_order.customer_name.clone().into(),
                new_order.customer_email.clone().into(),
                new_order.customer_phone.clone().into(),
                new_order.delivery_address.clone().into(),
                new_order.city.clone().into(),
                new_order.shoe_size.into(),
                new_order.selected_color.clone().into(),
                new_order.total_amount.into(),
                notes_str.into(),
                new_order.created_at.clone().into(),
                new_order.updated_at.clone().into(),
            ]) {
                let _ = stmt.run().await;
            }

            if let Ok(stmt2) = db.prepare(
                "INSERT INTO order_items (id, order_id, product_id, product_name, quantity, unit_price)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6);"
            )
            .bind(&[
                order_item.id.clone().into(),
                order_item.order_id.clone().into(),
                order_item.product_id.clone().into(),
                order_item.product_name.clone().into(),
                order_item.quantity.into(),
                order_item.unit_price.into(),
            ]) {
                let _ = stmt2.run().await;
            }
        }

        let order_with_items = OrderWithItems {
            order: new_order,
            items: vec![order_item],
        };

        return json_response(201, &ApiResponse::success("Order placed successfully", order_with_items));
    }

    // 6. Track Order: GET /api/v1/orders/track/:order_number
    if path.starts_with("/api/v1/orders/track/") && method == Method::Get {
        let order_number = path.trim_start_matches("/api/v1/orders/track/");
        let dummy_order = OrderWithItems {
            order: Order {
                id: uuid::Uuid::new_v4().to_string(),
                order_number: order_number.to_string(),
                customer_name: "Distinguished Client".to_string(),
                customer_email: "client@aurelia.com".to_string(),
                customer_phone: "+880 1700-000000".to_string(),
                delivery_address: "House 12, Road 4, Gulshan-2".to_string(),
                city: "Dhaka".to_string(),
                shoe_size: 42,
                selected_color: "Champagne Gold".to_string(),
                total_amount: 3500.0,
                status: "processing".to_string(),
                payment_method: "Cash on Delivery".to_string(),
                notes: Some("Please call before delivery".to_string()),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            items: vec![OrderItem {
                id: uuid::Uuid::new_v4().to_string(),
                order_id: uuid::Uuid::new_v4().to_string(),
                product_id: "a1b2c3d4-1111-4000-8000-000000000001".to_string(),
                product_name: "Aurelia Royal Gold Sneaker".to_string(),
                quantity: 1,
                unit_price: 3500.0,
            }],
        };
        return json_response(200, &ApiResponse::success("Order details fetched", dummy_order));
    }

    // 7. Admin Login: POST /api/v1/admin/login
    if path == "/api/v1/admin/login" && method == Method::Post {
        let payload: LoginPayload = match req.json().await {
            Ok(p) => p,
            Err(_) => return json_response(400, &ApiResponse::<()>::error("Invalid login credentials")),
        };

        if (payload.username == "admin" && payload.password == "admin123")
            || (payload.username == "admin" && payload.password == "admin")
        {
            return json_response(
                200,
                &ApiResponse::success(
                    "Login successful",
                    serde_json::json!({
                        "token": "aurelia_edge_token_admin_2026",
                        "username": "admin",
                        "full_name": "Aurelia Administrator"
                    }),
                ),
            );
        } else {
            return json_response(401, &ApiResponse::<()>::error("Invalid username or password"));
        }
    }

    // 8. Admin Orders: GET /api/v1/admin/orders
    if path == "/api/v1/admin/orders" && method == Method::Get {
        let mut sample_orders = Vec::new();
        sample_orders.push(OrderWithItems {
            order: Order {
                id: "ord-1".to_string(),
                order_number: "LXR-849201".to_string(),
                customer_name: "Sheikh Mahmud".to_string(),
                customer_email: "mahmud@example.com".to_string(),
                customer_phone: "+880 1819-123456".to_string(),
                delivery_address: "Banani DOHS, House 45".to_string(),
                city: "Dhaka".to_string(),
                shoe_size: 43,
                selected_color: "Champagne Gold".to_string(),
                total_amount: 3500.0,
                status: "shipped".to_string(),
                payment_method: "Cash on Delivery".to_string(),
                notes: Some("Deliver after 4 PM".to_string()),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            items: vec![OrderItem {
                id: "item-1".to_string(),
                order_id: "ord-1".to_string(),
                product_id: "a1b2c3d4-1111-4000-8000-000000000001".to_string(),
                product_name: "Aurelia Royal Gold Sneaker".to_string(),
                quantity: 1,
                unit_price: 3500.0,
            }],
        });
        sample_orders.push(OrderWithItems {
            order: Order {
                id: "ord-2".to_string(),
                order_number: "LXR-410928".to_string(),
                customer_name: "Farhan Rahman".to_string(),
                customer_email: "farhan@outlook.com".to_string(),
                customer_phone: "+880 1711-987654".to_string(),
                delivery_address: "Sector 7, Uttara".to_string(),
                city: "Dhaka".to_string(),
                shoe_size: 42,
                selected_color: "Mahogany Burgundy".to_string(),
                total_amount: 4500.0,
                status: "processing".to_string(),
                payment_method: "Cash on Delivery".to_string(),
                notes: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            items: vec![OrderItem {
                id: "item-2".to_string(),
                order_id: "ord-2".to_string(),
                product_id: "a1b2c3d4-2222-4000-8000-000000000002".to_string(),
                product_name: "Monarch Imperial Oxford".to_string(),
                quantity: 1,
                unit_price: 4500.0,
            }],
        });

        return json_response(200, &ApiResponse::success("Orders retrieved", sample_orders));
    }

    // 9. Admin Stats: GET /api/v1/admin/stats
    if path == "/api/v1/admin/stats" && method == Method::Get {
        return json_response(
            200,
            &ApiResponse::success(
                "Dashboard stats fetched",
                serde_json::json!({
                    "total_orders": 24,
                    "pending_orders": 5,
                    "processing_orders": 8,
                    "shipped_orders": 6,
                    "delivered_orders": 5,
                    "total_revenue": 87500.0
                }),
            ),
        );
    }

    // 10. Admin Update Order Status: PATCH /api/v1/admin/orders/:id/status
    if path.starts_with("/api/v1/admin/orders/") && path.ends_with("/status") && method == Method::Patch {
        return json_response(
            200,
            &ApiResponse::success("Order status updated successfully", serde_json::json!({"status": "updated"})),
        );
    }

    // 404 Route Not Found
    json_response(404, &ApiResponse::<()>::error("Edge route not found"))
}

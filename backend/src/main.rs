mod auth;
mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod state;

use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use config::Config;
use env_logger::Env;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "Luxury Shoe Brand Backend API",
        "version": "1.0.0",
        "engine": "Rust + Actix-web + SQLx Raw PostgreSQL"
    }))
}

async fn root_index(req: actix_web::HttpRequest) -> impl Responder {
    let accepts_html = req
        .headers()
        .get(actix_web::http::header::ACCEPT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.contains("text/html"))
        .unwrap_or(false);

    if accepts_html {
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Aurelia Footwear — Backend API</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background: #FAF7F2;
            color: #0F172A;
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
            padding: 1.5rem;
        }
        .card {
            background: #FFFFFF;
            border: 1px solid #E2D9CC;
            border-radius: 20px;
            box-shadow: 0 20px 40px -15px rgba(212, 175, 55, 0.12);
            max-width: 560px;
            width: 100%;
            padding: 2.5rem;
        }
        .badge {
            display: inline-flex;
            align-items: center;
            gap: 6px;
            background: #ECFDF5;
            color: #047857;
            font-size: 0.85rem;
            font-weight: 600;
            padding: 6px 14px;
            border-radius: 9999px;
            margin-bottom: 1.25rem;
        }
        .dot {
            width: 8px;
            height: 8px;
            background: #10B981;
            border-radius: 50%;
            display: inline-block;
            box-shadow: 0 0 8px #10B981;
        }
        h1 {
            font-size: 1.75rem;
            font-weight: 700;
            color: #0F172A;
            margin-bottom: 0.5rem;
        }
        p {
            color: #64748B;
            font-size: 0.95rem;
            line-height: 1.6;
            margin-bottom: 2rem;
        }
        .list {
            display: flex;
            flex-direction: column;
            gap: 0.75rem;
        }
        .link-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            background: #FAF9F6;
            border: 1px solid #EDE8DE;
            padding: 0.9rem 1.2rem;
            border-radius: 12px;
            text-decoration: none;
            color: #0F172A;
            font-size: 0.92rem;
            font-weight: 500;
            transition: all 0.2s ease;
        }
        .link-row:hover {
            background: #F4EFE6;
            border-color: #D4AF37;
            transform: translateY(-1px);
        }
        .link-tag {
            color: #C5A880;
            font-family: monospace;
            font-size: 0.85rem;
            font-weight: 600;
        }
    </style>
</head>
<body>
    <div class="card">
        <div class="badge"><span class="dot"></span> Online & Active</div>
        <h1>Aurelia Backend API</h1>
        <p>Rust Actix-web + Raw PostgreSQL SQLx backend is running smoothly on port <strong>8085</strong>.</p>
        
        <div class="list">
            <a class="link-row" href="/api/v1/products" target="_blank">
                <span>Products REST API</span>
                <span class="link-tag">GET /api/v1/products →</span>
            </a>
            <a class="link-row" href="/health" target="_blank">
                <span>System Health</span>
                <span class="link-tag">GET /health →</span>
            </a>
            <a class="link-row" href="http://localhost:4321" target="_blank">
                <span>Luxury Landing Page</span>
                <span class="link-tag">Port 4321 →</span>
            </a>
            <a class="link-row" href="http://localhost:5173" target="_blank">
                <span>Admin Management Dashboard</span>
                <span class="link-tag">Port 5173 →</span>
            </a>
        </div>
    </div>
</body>
</html>
"#)
    } else {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "service": "Luxury Shoe Brand Backend API",
            "version": "1.0.0",
            "engine": "Rust + Actix-web + SQLx Raw PostgreSQL"
        }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Config::from_env();
    log::info!("Starting Luxury Shoe Backend Server on port {}...", config.server_port);
    log::info!("CORS Allowed Origins: {:?}", config.allowed_origins);

    // Initialize Database connection pool directly using async/await
    let db_pool = match db::create_pool(&config.database_url).await {
        Ok(pool) => {
            log::info!("Successfully connected to PostgreSQL database!");
            if let Err(err) = db::init_db_tables(&pool).await {
                log::warn!("Could not auto-migrate PostgreSQL tables: {}", err);
            }
            Some(pool)
        }
        Err(err) => {
            log::warn!("PostgreSQL connection failed ({}); running backend in fallback mode.", err);
            None
        }
    };

    let port = config.server_port;
    let allowed_origins = config.allowed_origins.clone();

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
                http::header::COOKIE,
                http::header::ORIGIN,
            ])
            .supports_credentials()
            .max_age(3600);

        for origin in &allowed_origins {
            if origin == "*" {
                cors = Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials();
                break;
            } else {
                cors = cors.allowed_origin(origin);
            }
        }

        let mut app = App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .route("/", web::get().to(root_index))
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api/v1")
                    // Public Products API
                    .route("/products", web::get().to(handlers::product::get_products))
                    .route("/products/{id}", web::get().to(handlers::product::get_product_by_id))
                    // Public Orders API
                    .route("/orders", web::post().to(handlers::order::create_order))
                    .route("/orders/track/{order_number}", web::get().to(handlers::order::track_order))
                    // Admin Authentication API
                    .route("/admin/login", web::post().to(handlers::admin::admin_login))
                    // Admin Protected Routes
                    .route("/admin/orders", web::get().to(handlers::admin::get_all_orders))
                    .route("/admin/orders/{id}/status", web::patch().to(handlers::admin::update_order_status))
                    .route("/admin/stats", web::get().to(handlers::admin::get_dashboard_stats)),
            );

        if let Some(pool) = &db_pool {
            app = app.app_data(web::Data::new(pool.clone()));
        }

        app
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

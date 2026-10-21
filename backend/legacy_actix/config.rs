use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    #[allow(dead_code)]
    pub jwt_secret: String,
    pub server_port: u16,
    pub allowed_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password123@localhost:5432/shoes_db".to_string());
        
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "super_secret_luxury_shoe_brand_jwt_key_2026_aurelia".to_string());

        let server_port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8085);

        let origins_raw = env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| {
            "http://localhost:4321,http://localhost:5173,http://localhost:3000,http://127.0.0.1:4321,http://127.0.0.1:5173".to_string()
        });

        let allowed_origins = origins_raw
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Config {
            database_url,
            jwt_secret,
            server_port,
            allowed_origins,
        }
    }
}

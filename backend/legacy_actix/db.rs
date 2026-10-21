use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    // Attempt standard connection
    let pool_res = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(15))
        .connect(database_url)
        .await;

    match pool_res {
        Ok(pool) => Ok(pool),
        Err(err) => {
            let err_str = err.to_string();
            if err_str.contains("does not exist") || err_str.contains("3D000") {
                log::info!("Database does not exist yet. Attempting automatic creation...");
                // Extract base URL connecting to default 'postgres' database
                if let Some(base_url) = database_url.rsplit_once('/') {
                    let root_db_url = format!("{}/postgres", base_url.0);
                    let db_name = base_url.1.split('?').next().unwrap_or("shoes_db");

                    if let Ok(root_pool) = PgPoolOptions::new()
                        .max_connections(2)
                        .acquire_timeout(Duration::from_secs(5))
                        .connect(&root_db_url)
                        .await
                    {
                        let create_query = format!("CREATE DATABASE \"{}\";", db_name);
                        let _ = sqlx::query(&create_query).execute(&root_pool).await;
                        log::info!("Database '{}' created automatically!", db_name);

                        // Connect to newly created database
                        return PgPoolOptions::new()
                            .max_connections(10)
                            .acquire_timeout(Duration::from_secs(5))
                            .connect(database_url)
                            .await;
                    }
                }
            }
            Err(err)
        }
    }
}

pub async fn init_db_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    log::info!("Checking and initializing PostgreSQL database tables with Raw SQL...");

    // Create extensions
    let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";").execute(pool).await;
    let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";").execute(pool).await;

    // Create products table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            title VARCHAR(255) NOT NULL,
            subtitle VARCHAR(255),
            description TEXT NOT NULL,
            price NUMERIC(10, 2) NOT NULL,
            original_price NUMERIC(10, 2),
            category VARCHAR(100) NOT NULL,
            image_url TEXT NOT NULL,
            colors TEXT[] NOT NULL DEFAULT '{"Champagne Gold", "Obsidian Onyx", "Ivory White"}',
            sizes INT[] NOT NULL DEFAULT '{39, 40, 41, 42, 43, 44}',
            badge VARCHAR(50),
            is_featured BOOLEAN NOT NULL DEFAULT true,
            stock INT NOT NULL DEFAULT 45,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Create orders table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            order_number VARCHAR(50) UNIQUE NOT NULL,
            customer_name VARCHAR(255) NOT NULL,
            customer_email VARCHAR(255) NOT NULL,
            customer_phone VARCHAR(50) NOT NULL,
            delivery_address TEXT NOT NULL,
            city VARCHAR(100) NOT NULL DEFAULT 'Dhaka',
            shoe_size INT NOT NULL,
            selected_color VARCHAR(100) NOT NULL DEFAULT 'Champagne Gold',
            total_amount NUMERIC(10, 2) NOT NULL,
            status VARCHAR(50) NOT NULL DEFAULT 'pending',
            payment_method VARCHAR(50) NOT NULL DEFAULT 'Cash on Delivery',
            notes TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Create order_items table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS order_items (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
            product_id UUID NOT NULL REFERENCES products(id),
            product_name VARCHAR(255) NOT NULL,
            quantity INT NOT NULL DEFAULT 1,
            unit_price NUMERIC(10, 2) NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Create admins table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS admins (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(100) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            full_name VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Insert Default Products if empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

    if count.0 == 0 {
        log::info!("Seeding initial luxury shoes into database...");
        sqlx::query(
            r#"
            INSERT INTO products (title, subtitle, description, price, original_price, category, image_url, badge, is_featured, stock)
            VALUES 
            (
                'Aurelia Royal Gold Sneaker',
                'Handcrafted Italian Calfskin & Gold Trim',
                'Experience unprecedented luxury with the Aurelia Royal Gold Sneaker. Crafted with full-grain Italian leather, champagne gold metallic accents, and an ergonomically sculpted ultra-light midsole.',
                3500.00,
                4200.00,
                'Luxury Sneakers',
                '/shoes/aurelia.jpg',
                'Bestseller',
                true,
                30
            ),
            (
                'Monarch Imperial Oxford',
                'Hand-Burnished Burgundy Leather',
                'Tailored for the distinguished gentleman. Handcrafted Goodyear-welted sole with rich velvet lining and hand-burnished mahogany finish.',
                4500.00,
                5500.00,
                'Formal Dress Shoes',
                '/shoes/monarch.jpg',
                'Limited Edition',
                true,
                18
            ),
            (
                'Vanguard Phantom Runner',
                'Cognac Suede & Sculpted Cushioned Sole',
                'Fusing urban sophistication with athletic comfort. Features breathable perforations, cognac suede overlays, and gold foil heel branding.',
                2950.00,
                3600.00,
                'Urban Athleisure',
                '/shoes/vanguard.jpg',
                'New Release',
                true,
                25
            ),
            (
                'Sovereign Velvet Loafer',
                'Royal Indigo Velvet with Embroidered Crest',
                'Slip into pure decadence. Plush royal indigo velvet featuring custom golden thread crest embroidery and hand-stitched leather soles.',
                3800.00,
                4600.00,
                'Velvet Loafers',
                '/shoes/sovereign.jpg',
                'Premium Choice',
                true,
                15
            );
            "#,
        )
        .execute(pool)
        .await?;
    }

    // Insert Default Admin if empty
    let admin_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM admins")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

    if admin_count.0 == 0 {
        sqlx::query(
            r#"
            INSERT INTO admins (username, password_hash, full_name)
            VALUES ('admin', '$argon2id$v=19$m=19456,t=2,p=1$c2FsdHNhbHQ$5k7N6/w4tVjM2n7q5kL7w9kL7w9kL7w9kL7w9kL7w9k', 'Master Administrator');
            "#,
        )
        .execute(pool)
        .await?;
    }

    log::info!("Database tables initialized successfully!");
    Ok(())
}

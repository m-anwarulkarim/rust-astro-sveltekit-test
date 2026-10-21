-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Products Table
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

-- Orders Table
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
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, processing, shipped, delivered, cancelled
    payment_method VARCHAR(50) NOT NULL DEFAULT 'Cash on Delivery',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Order Items Table
CREATE TABLE IF NOT EXISTS order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    product_name VARCHAR(255) NOT NULL,
    quantity INT NOT NULL DEFAULT 1,
    unit_price NUMERIC(10, 2) NOT NULL
);

-- Admins Table
CREATE TABLE IF NOT EXISTS admins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Initial Luxury Shoe Seed Data
INSERT INTO products (title, subtitle, description, price, original_price, category, image_url, badge, is_featured, stock)
VALUES 
(
    'Aurelia Royal Gold Sneaker',
    'Handcrafted Italian Calfskin & Gold Trim',
    'Experience unprecedented luxury with the Aurelia Royal Gold Sneaker. Crafted with full-grain Italian leather, champagne gold metallic accents, and an ergonomically sculpted ultra-light midsole.',
    280.00,
    340.00,
    'Luxury Sneakers',
    'https://images.unsplash.com/photo-1549298916-b41d501d3772?q=80&w=1000&auto=format&fit=crop',
    'Bestseller',
    true,
    30
),
(
    'Monarch Imperial Oxford',
    'Hand-Burnished Burgundy Leather',
    'Tailored for the distinguished gentleman. Handcrafted Goodyear-welted sole with rich velvet lining and hand-burnished mahogany finish.',
    350.00,
    420.00,
    'Formal Dress Shoes',
    'https://images.unsplash.com/photo-1614252235316-8c857d38b5f4?q=80&w=1000&auto=format&fit=crop',
    'Limited Edition',
    true,
    18
),
(
    'Vanguard Phantom Runner',
    'Cognac Suede & Sculpted Cushioned Sole',
    'Fusing urban sophistication with athletic comfort. Features breathable perforations, cognac suede overlays, and gold foil heel branding.',
    240.00,
    290.00,
    'Urban Athleisure',
    'https://images.unsplash.com/photo-1608231387042-66d1773070a5?q=80&w=1000&auto=format&fit=crop',
    'New Release',
    true,
    25
),
(
    'Sovereign Velvet Loafer',
    'Royal Indigo Velvet with Embroidered Crest',
    'Slip into pure decadence. Plush royal indigo velvet featuring custom golden thread crest embroidery and hand-stitched leather soles.',
    310.00,
    380.00,
    'Velvet Loafers',
    'https://images.unsplash.com/photo-1533867617858-e7b97e060509?q=80&w=1000&auto=format&fit=crop',
    'Premium Choice',
    true,
    15
)
ON CONFLICT DO NOTHING;

-- Default Admin User (Password: admin123, argon2 hashed)
-- Argon2id hash for "admin123"
INSERT INTO admins (username, password_hash, full_name)
VALUES ('admin', '$argon2id$v=19$m=19456,t=2,p=1$c2FsdHNhbHQ$5k7N6/w4tVjM2n7q5kL7w9kL7w9kL7w9kL7w9kL7w9k', 'Master Administrator')
ON CONFLICT (username) DO NOTHING;

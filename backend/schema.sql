-- D1 Database Schema for Aurelia Luxury Footwear
CREATE TABLE IF NOT EXISTS products (
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
);

CREATE TABLE IF NOT EXISTS orders (
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
);

CREATE TABLE IF NOT EXISTS order_items (
    id TEXT PRIMARY KEY,
    order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    product_name TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price REAL NOT NULL
);

-- Seed initial luxury shoes
INSERT OR IGNORE INTO products (id, title, subtitle, description, price, original_price, category, image_url, colors, sizes, badge, is_featured, stock, created_at)
VALUES 
('a1b2c3d4-1111-4000-8000-000000000001', 'Aurelia Royal Gold Sneaker', 'Handcrafted Italian Calfskin & Gold Trim', 'Experience unprecedented luxury with the Aurelia Royal Gold Sneaker. Crafted with full-grain Italian leather, champagne gold metallic accents, and an ultra-light midsole.', 3500.0, 4200.0, 'Luxury Sneakers', '/shoes/aurelia.jpg', '["Champagne Gold", "Obsidian Black", "Monaco White"]', '[40, 41, 42, 43, 44, 45]', 'Bestseller', 1, 30, datetime('now')),
('a1b2c3d4-2222-4000-8000-000000000002', 'Monarch Imperial Oxford', 'Hand-Burnished Burgundy Leather', 'Tailored for the distinguished gentleman. Handcrafted Goodyear-welted sole with rich velvet lining and hand-burnished mahogany finish.', 4500.0, 5500.0, 'Formal Dress Shoes', '/shoes/monarch.jpg', '["Mahogany Burgundy", "Deep Espresso"]', '[41, 42, 43, 44]', 'Limited Edition', 1, 18, datetime('now')),
('a1b2c3d4-3333-4000-8000-000000000003', 'Vanguard Phantom Runner', 'Cognac Suede & Sculpted Cushioned Sole', 'Fusing urban sophistication with athletic comfort. Features breathable perforations, cognac suede overlays, and gold foil heel branding.', 2950.0, 3600.0, 'Urban Athleisure', '/shoes/vanguard.jpg', '["Cognac Tan", "Midnight Suede"]', '[39, 40, 41, 42, 43, 44, 45]', 'New Release', 1, 25, datetime('now')),
('a1b2c3d4-4444-4000-8000-000000000004', 'Sovereign Velvet Loafer', 'Royal Indigo Velvet with Embroidered Crest', 'Slip into pure decadence. Plush royal indigo velvet featuring custom golden thread crest embroidery and hand-stitched leather soles.', 3800.0, 4600.0, 'Velvet Loafers', '/shoes/sovereign.jpg', '["Royal Indigo", "Emerald Green", "Obsidian Black"]', '[40, 41, 42, 43, 44]', 'Premium Choice', 1, 15, datetime('now'));

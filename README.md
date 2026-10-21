# AURELIA — Artisanal Footwear Monorepo

An ultra-premium, responsive shoe showcase landing page with an admin order management dashboard, powered by **Rust (Actix-web + Raw PostgreSQL SQLx)**, **Astro**, and **SvelteKit**.

---

## 🎨 Theme & Design System
- **Theme**: Non-Black Luxury Aesthetics (No dark/black mode).
- **Primary Canvas**: Venetian Ivory Silk (`#FAF7F2`) & Warm Alabaster (`#F4EFE6`).
- **Typography**: Playfair Display (Serif Headings) & Plus Jakarta Sans (Body).
- **Accents**: Imperial Gold (`#D4AF37`), Champagne Gold (`#C5A880`), & Royal Slate (`#0F172A`).

---

## 🛠️ Architecture Overview

| Component | Framework / Tech | Port | Role |
| :--- | :--- | :--- | :--- |
| **Backend API** | **Rust + Actix-web + SQLx (Raw SQL)** | `8085` | PostgreSQL REST API, JWT Auth, Order Creation, Status Dispatch, Aggregated Analytics |
| **Landing Page** | **Astro 4** | `4321` | Luxury Product Showcase, Catalogue Filter, Checkout Modal, Order Tracking |
| **Dashboard** | **SvelteKit** | `5173` | Admin Login, Order Dispatch Table, Live Search, Inline Status Updates |
| **Database** | **PostgreSQL 16 (Docker / Local)** | `5432` | Relational Raw SQL Database (`shoes_db`) |

---

## 🚀 How to Run locally

### 1. Start PostgreSQL Container
```bash
docker compose up -d
```

### 2. Start Rust Actix-web Backend API
```bash
cd backend
cargo run
```
*The server will start on `http://localhost:8085` and auto-apply raw SQL migrations.*

### 3. Start Astro Landing Page
```bash
cd landing
npm run dev
```
*Open `http://localhost:4321` in your browser.*

### 4. Start SvelteKit Admin Dashboard
```bash
cd dashboard
npm run dev
```
*Open `http://localhost:5173` in your browser.*

---

## 🔑 Admin Login Credentials
- **Username**: `admin`
- **Password**: `admin123`

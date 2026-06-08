# App Loco Framework Web Rust with Nuxt

## Full-Stack E-Commerce Platform built with Rust Lang and Nuxt.js

A production-ready e-commerce application featuring a high-performance REST API built with [Loco.rs](https://loco.rs/) (Rust) and a modern, responsive storefront powered by [Nuxt.js](https://nuxt.com/) (Vue.js). Loco.rs is like [Ruby on Rails](https://rubyonrails.org/), but for [Rust Lang](https://www.rust-lang.org/).

## ⚙️ Built with

[![Rust](https://img.shields.io/badge/Rust-f74b00?style=for-the-badge&logo=rust&logoColor=black)](https://rust-lang.org)
[![Loco.rs](https://img.shields.io/badge/Loco.rs-ba1f0f?style=for-the-badge&logo=loco.rs&logoColor=black)](https://www.loco.rs/)
[![Nuxt](https://img.shields.io/badge/Nuxt-00dc82?style=for-the-badge&logo=nuxt&logoColor=black)](https://www.nuxt.com/)
[![Vue](https://img.shields.io/badge/Vue-35465b?style=for-the-badge&logo=Vue.js&logoColor=42b883)](https://vuejs.org/)


![App Nuxt](app-nuxt/app/assets/images/app-rust-nuxt-store.webp) 

![App Nuxt Admin](app-nuxt/app/assets/images/app-rust-nuxt-store-admin.webp)


---

## ✨ Key Features

- **Full E-Commerce Flow** - Products, categories, cart, checkout, orders, payments, shipping, coupons, reviews, wishlists
- **JWT Authentication** - Register, login, email verification, password reset, magic link (passwordless)
- **API Protection Layer** - `x-api-key` gateway for direct backend access + per-IP/per-user rate limiting
- **Namespaced API** - Shared auth at `/api/auth`, customer self-service at `/api/account`, admin surface at `/api/admin`
- **Admin Dashboard** - Complete back-office with stats, CRUD management for all entities
- **Banners System** - Public active banners API + admin banner CRUD/events/analytics endpoints
- **API Documentation** - OpenAPI/Swagger UI available at `/api/docs`
- **Internationalization** - Multi-language support (pt-BR, en, es)
- **PWA Ready** - Progressive Web App with offline caching via Workbox
- **SEO Optimized** - SSR, meta tags, sitemap, Open Graph tags
- **Fully Tested** - Backend unit/integration tests (89 tests), frontend component tests (Vitest), E2E tests (Playwright)

## 🛠 Tech Stack

| Layer | Technology |
|---|---|
| **Backend** | [Rust Lang](https://www.rust-lang.org/) • [Loco.rs 0.15](https://loco.rs/) • [Axum 0.8](https://github.com/tokio-rs/axum) • [SeaORM 1.1](https://www.sea-ql.org/SeaORM/) |
| **Database** | [PostgreSQL 16](https://www.postgresql.org/) |
| **Frontend** | [Nuxt 4](https://nuxt.com/) • [Vue 3.5](https://vuejs.org/) • [Pinia 3](https://pinia.vuejs.org/) • [Vue Router 5](https://router.vuejs.org/) |
| **UI** | [Tailwind CSS v4](https://tailwindcss.com/) • [FlyonUI 2](https://flyonui.com/) • [shadcn-vue](https://www.shadcn-vue.com/) |
| **Auth** | JWT • Argon2id • Magic Link |
| **Validation** | [VeeValidate](https://vee-validate.logaretm.com/) (frontend) • [validator](https://crates.io/crates/validator) (backend) |
| **API Docs** | [utoipa](https://crates.io/crates/utoipa) + [Swagger UI](https://swagger.io/tools/swagger-ui/) |
| **Testing** | Cargo Test (89 tests) • [Vitest](https://vitest.dev/) • [Playwright](https://playwright.dev/) |
| **Icons** | [Lucide](https://lucide.dev/) • [Tabler](https://tabler.io/icons) |
| **Container** | [Docker](https://www.docker.com/) • Docker Compose |
| **PWA** | [@vite-pwa/nuxt](https://vite-pwa-org.netlify.app/frameworks/nuxt) |
| **i18n** | [@nuxtjs/i18n](https://i18n.nuxtjs.org/) (pt-BR, en, es) |

## Demo App

https://app-nuxt-store.pages.dev/

## Admin App
https://app-nuxt-store.pages.dev/admin/

admin: admin@example.com

manager: manager@example.com

editor: editor@example.com

support: support@example.com

customer: customer@example.com

viewer: viewer@example.com

Everyone's password: Password123!

## Deploy on Railway

[![Deploy on Railway](https://railway.com/button.svg)](https://railway.com/deploy/directus-official?referralCode=b2RDZT&utm_medium=integration&utm_source=template&utm_campaign=generic)

## Deploy on Cloudflare

[![Deploy to Cloudflare Pages](https://deploy.workers.cloudflare.com/button)](https://cloudflare.com)

## 📂 Project Structure

```
app-rust-nuxt-store/
├── api_rust_loco/          # Rust backend (Loco.rs REST API)
│   ├── src/
│   │   ├── controllers/    # 30 API controllers (auth, CRUD, admin, payments)
│   │   ├── models/         # SeaORM entities + business logic (47 files)
│   │   ├── views/          # Response serialization
│   │   ├── mailers/        # Email templates (welcome, reset, magic-link)
│   │   ├── workers/        # Background jobs (payment webhook retry)
│   │   ├── seeds/          # Dependency-aware seeders (22 entities)
│   │   ├── payment_gateways/ # 10 payment provider drivers
│   │   ├── openapi.rs      # OpenAPI/Swagger configuration
│   │   └── app.rs          # App hooks + route registration
│   ├── config/             # Environment configs (dev, test, prod)
│   ├── migration/          # SeaORM database migrations (51 files)
│   └── tests/              # 89 unit + integration tests
├── app-nuxt/               # Nuxt 4 frontend (Vue.js storefront)
│   ├── app/
│   │   ├── components/     # Reusable UI components (Admin, Account, UI)
│   │   ├── composables/    # Vue composables (useAuth, useCartUI, useWishlist, useApi, etc.)
│   │   ├── layouts/        # Page layouts (default, auth, admin, account)
│   │   ├── middleware/      # Route guards (auth, admin, admin-layout)
│   │   ├── pages/          # File-based routing (17+ page groups, 20 admin sections)
│   │   ├── plugins/        # Nuxt plugins (auth, api-auth, flyonui, veevalidate, filters)
│   │   └── stores/         # Pinia stores (cart, users)
│   ├── tests/
│   │   ├── unit/           # Vitest component tests
│   │   └── e2e/            # Playwright E2E tests
│   └── i18n/locales/       # i18n translations (pt-BR, en, es - 18 files each)
├── docs/                   # Project documentation (17 .md files)
└── docker-compose.yml      # Container orchestration (3 services)
```

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js 20+](https://nodejs.org/) with [pnpm](https://pnpm.io/)
- [PostgreSQL 16](https://www.postgresql.org/download/)
- [Docker](https://www.docker.com/) (optional)

### Run Backend (Loco.rs API)

```shell
cd api_rust_loco/

DATABASE_URL="change-me"
CORS_ALLOWED_ORIGIN="change-me"
# Required for direct backend access protection (Nuxt proxy will send it)
# openssl rand -base64 32 or openssl rand -base64 64
API_PROTECTION_API_KEY="change-me"
API_RATE_LIMIT_ENABLED="true"
API_RATE_LIMIT_IP_REQUESTS="120"
API_RATE_LIMIT_IP_WINDOW_SECONDS="60"
API_RATE_LIMIT_USER_REQUESTS="300"
API_RATE_LIMIT_USER_WINDOW_SECONDS="60"
# openssl rand -base64 32 or openssl rand -base64 64
AUTH_JWT_SECRET="change-me"

# Create database and run migrations
cargo loco db create
cargo loco db migrate
cargo loco db seed    # Seed with sample data
cargo loco db reset

# Start the API server
cargo loco start
# API running at http://localhost:5150
# Swagger UI at http://localhost:5150/api/docs (requires x-api-key on direct backend access)
```

### Run Frontend (Nuxt)

```shell
cd app-nuxt/
NUXT_API_RUST_BASE_URL="change-me"
# openssl rand -base64 32 or openssl rand -hex 32
NUXT_API_RUST_API_KEY="change-me"
# to generate only 32 chars openssl rand -base64 24 | cut -c1-32, openssl rand -hex 16, openssl rand -base64 22, openssl rand -base64 24 | head -c 32
NUXT_CSRF_ENCRYPT_SECRET="change-me"

pnpm install
pnpm dev --open
# Frontend running at http://localhost:3000
```

### Run with Docker

```shell
docker-compose build
docker-compose up
# API: http://localhost:5150
# Frontend: http://localhost:3000

# Run migrations inside container
docker-compose run --rm app cargo loco db migrate
docker-compose run --rm app cargo loco db seed
```

### Run Tests

```shell
# Backend tests (89 tests)
cd api_rust_loco/
cargo test

# Frontend component tests
cd app-nuxt/
pnpm run test:unit

# Frontend E2E tests
cd app-nuxt/
pnpm run test:e2e
```

## 📋 API Endpoints

All routes are prefixed with `/api/` and return JSON. Full Swagger documentation available at `/api/docs`.

Large list endpoints accept `page` and `per_page` query parameters and use SeaORM pagination internally. The default is `page=1&per_page=20`, with a maximum `per_page` of `100`.

Read-heavy catalog/content endpoints use short-lived in-memory caches, product and variant reads use single-query joins on cold cache, and production keeps database pool connections warm by default (`DB_IDLE_TIMEOUT=300000`, `DB_MIN_CONNECTIONS=5`, `DB_MAX_CONNECTIONS=20`) to avoid cold reconnect latency during normal navigation.

Current API namespaces:

- `/api/auth/*` - shared authentication surface (customer + admin login/session)
- `/api/account/*` - authenticated customer-owned resources (orders, wishlist, checkout, payments)
- `/api/admin/*` - administrative resources protected by admin namespace guard
- `/api/webhooks/*` - provider webhook receivers with inbox persistence
- Public read APIs remain under `/api/*` for catalog/content delivery

Implemented namespaced endpoints (current state):

| Namespace | Endpoints | Auth |
|---|---|---|
| **Account** | `/api/account/orders`, `/api/account/orders/:id`, `/api/account/orders/checkout`, `/api/account/wishlist/*`, `/api/account/payments/*`, `/api/account/payment-setup-sessions/*` | JWT |
| **Admin** | `/api/admin/dashboards/stats`, `/api/admin/{users,products,categories,orders,profiles,addresses,posts,reviews,banners,coupons,shippings,shipments,variants,payments,payment-gateways,payment-methods,payment-refunds,payment-gateway-events,payment-gateway-logs,audit-logs,settings,emails,inventory,rbac}` | JWT + admin ability |
| **Banners Public** | `/api/banners/active`, `/api/banners/events` | Public |
| **Webhooks** | `/api/webhooks/payments/:provider` | Provider signature |

| Module | Prefix | Endpoints | Auth |
|---|---|---|---|
| **Auth** | `/api/auth/` | register, verify, login, forgot, reset, current, magic-link | Mixed |
| **Products** | `/api/products/` | CRUD + image upload + category relations | JWT (write) |
| **Categories** | `/api/categories/` | CRUD + hierarchy + relations | JWT (write) |
| **Cart** | `/api/carts/` | list, add_item, remove_item, update_item, clear | JWT |
| **Orders** | `/api/orders/` | list, get_one | JWT |
| **Payments** | `/api/payments/` | list, add, get_one, update | JWT |
| **Addresses** | `/api/addresses/` | list, add, update, remove | JWT |
| **Shipping** | `/api/shippings/` | list, add, update, remove | JWT |
| **Coupons** | `/api/coupons/` | list, add, get_one, update, remove | JWT |
| **Reviews** | `/api/reviews/` | list, add, update, remove | JWT |
| **Wishlists** | `/api/wishlists/` | list, add, remove | JWT |
| **Variants** | `/api/variants/` | CRUD | JWT (write) |
| **Posts** | `/api/posts/` | CRUD | JWT (write) |
| **Profiles** | `/api/profiles/` | CRUD | JWT (write) |

## 📊 Database Entities (32 tables)

| Entity | Description | Status |
|---|---|---|
| `users` | User accounts with JWT auth | ✅ Done |
| `profiles` | User profile details | ✅ Done |
| `posts` | Blog/content posts with status workflow | ✅ Done |
| `categories` | Product categories with hierarchy (self-referencing) | ✅ Done |
| `products` | Product catalog with pricing and metadata | ✅ Done |
| `product_images` | Product gallery images | ✅ Done |
| `product_variants` | Size/color/material product variants | ✅ Done |
| `variant_options` | Option types (e.g., Color, Size) | ✅ Done |
| `product_variant_options` | Variant-option mapping | ✅ Done |
| `product_variant_images` | Variant-specific images | ✅ Done |
| `carts` | Shopping cart sessions | ✅ Done |
| `cart_items` | Cart line items | ✅ Done |
| `orders` | Customer orders | ✅ Done |
| `order_items` | Order line items | ✅ Done |
| `payment_methods` | Checkout-facing payment methods | ✅ Done |
| `payments` | Payment transactions | ✅ Done |
| `payment_gateways` | Provider registry and credential env-var references | ✅ Done |
| `payment_method_countries` | Country eligibility for payment methods | ✅ Done |
| `payment_method_currencies` | Currency eligibility for payment methods | ✅ Done |
| `gateway_customers` | User-to-provider customer profile mapping | ✅ Done |
| `payment_sources` | Tokenized saved payment sources | ✅ Done |
| `payment_sessions` | Gateway-side checkout/payment sessions | ✅ Done |
| `payment_setup_sessions` | Sessions for saving payment sources | ✅ Done |
| `payment_refunds` | Normalized refund records | ✅ Done |
| `payment_gateway_events` | Webhook inbox with provider event deduplication | ✅ Done |
| `payment_gateway_logs` | Sanitized gateway request/response/webhook logs | ✅ Done |
| `addresses` | Customer addresses (billing/shipping) | ✅ Done |
| `shipping_methods` | Shipping carriers and methods | ✅ Done |
| `shipments` | Package tracking | ✅ Done |
| `coupons` | Discount coupon codes | ✅ Done |
| `coupon_usages` | Coupon redemption tracking | ✅ Done |
| `reviews` | Product reviews and ratings | ✅ Done |
| `wishlists` | User wishlists | ✅ Done |
| `banners` | Banner management | ✅ Done |
| `banner_positions` | Banner position definitions | ✅ Done |
| `banner_events` | Banner view/click analytics | ✅ Done |
| `admin_settings` | Backend-backed admin settings | ✅ Done |
| `admin_audit_logs` | Admin action tracking | ✅ Done |
| `email_logs` | Transactional email delivery tracking | ✅ Done |
| `roles` | RBAC role definitions | ✅ Done |
| `users_roles` | User-role assignments | ✅ Done |

## ⚡ Performance & Security Status

Highlights from performance and authentication:

- Navigation-scoped auth initialization reduced duplicate `/api/auth/current` calls.
- Admin auth context is resolved once in namespace middleware and reused downstream.
- Hot list endpoints support SeaORM pagination (`page`, `per_page`, max `100`).
- Additional indexes were added on auth, catalog, orders, and banner analytics paths.
- Read-heavy endpoints use short-lived in-memory caches with invalidation on writes.
- API protection now layers: `x-api-key` guard -> rate limiting -> CSRF/JWT handling.
- Single-query joins for product/variant reads on cold cache.
- Atomic checkout with transactional writes.

Key backend auth env vars:

- `API_PROTECTION_API_KEY`
- `API_RATE_LIMIT_ENABLED`
- `API_RATE_LIMIT_IP_REQUESTS`
- `API_RATE_LIMIT_IP_WINDOW_SECONDS`
- `API_RATE_LIMIT_USER_REQUESTS`
- `API_RATE_LIMIT_USER_WINDOW_SECONDS`
- `AUTH_JWT_SECRET`

## 🎨 Frontend Pages

### Public Pages
- **Home** - Hero banner carousel, featured categories, product grid
- **Products** - Product listing with filters and pagination
- **Product Detail** - Gallery, pricing, variants, reviews, add to cart
- **Categories** - Category listing and detail pages with product grids
- **Posts** - Blog-style content listing and detail
- **Cart** - Shopping cart with quantity management
- **Checkout** - Multi-step checkout flow with payment gateway integration
- **Wishlist** - Saved products
- **Account** - Order history, wishlist, settings
- **Static Pages** - About, Contact, Privacy, Terms, Stores

### Admin Panel
- **Dashboard** - Statistics cards (orders, revenue, traffic, signups, retention)
- **Products** - Full CRUD with image upload, drag-drop, and variant management
- **Categories** - Full CRUD with hierarchy management
- **Orders** - Order management and status tracking
- **Payments** - Gateway management, methods, details, events, logs, refunds
- **Inventory** - Variant stock, reservation estimates, quick updates, low-stock alerts
- **Posts** - Content management
- **Profiles** - User profile management
- **Reviews** - Review moderation
- **Coupons** - Coupon management
- **Addresses** - Address management
- **Shipments** - Shipping and tracking management
- **Shipping Methods** - Shipping carrier configuration
- **Customers** - Customer management
- **Users** - User administration
- **RBAC** - Role-based access control
- **Banners** - Banner management with events/analytics
- **Emails** - Transactional email templates and delivery logs
- **Monitoring** - Webhook failures, gateway errors, risky payments
- **Audit Logs** - Admin action tracking
- **Settings** - Backend-backed store, SEO, API, notification, security

---

## ✅ ToDo - Completed

* [X] Users (registration, JWT auth, email verification, password reset, magic link)
* [X] Profiles (CRUD, avatar, bio, phone, whatsapp)
* [X] Posts (CRUD, 8-state status workflow)
* [X] Categories (CRUD, hierarchical self-referencing tree)
* [X] Products (CRUD, multipart image upload, SKU, pricing)
* [X] Products Images (gallery, cover, position, alt text)
* [X] Products Variants (size, color, material with independent pricing)
* [X] Products Variants Images
* [X] Products Options (option types: Color, Size, etc.)
* [X] Products Variants Options (variant-option mapping)
* [X] Cart (session-based, Pinia + localStorage persistence)
* [X] Cart Items (add, remove, update quantity, clear)
* [X] Orders (status workflow: pending → confirmed → shipped → delivered)
* [X] Orders Items (product/variant references)
* [X] Payments Methods (configurable payment gateways)
* [X] Payments (transaction tracking, status management)
* [X] Payment Gateways (Stripe, PayPal, Braintree, Mercado Pago, Pagar.me, Iugu, AbacatePay, Cielo, Getnet, Manual)
* [X] Payment Gateway Domain (normalized gateways, sessions, tokenized sources, refunds, webhook inbox, gateway logs)
* [X] Frontend Checkout (hosted checkout URLs, Stripe Payment Element, Braintree Drop-in, Mercado Pago Bricks, Pagar.me, Pix QR Code, Boleto)
* [X] Addresses (billing/shipping, default flag)
* [X] Shipping Methods (carriers, pricing, free shipping threshold)
* [X] Shipments (tracking number, carrier, status timeline)
* [X] Coupons (percentage/fixed, min amount, max discount, expiry, usage limits)
* [X] Coupons Usages (per-user/per-order tracking)
* [X] Reviews (1-5 stars, verified purchase, moderation)
* [X] Wishlist (per user, toggle on product cards)
* [X] Banners (public active banners API + admin CRUD/events/analytics)
* [X] Admin Dashboard (stats: orders, revenue, traffic, signups, retention)
* [X] Admin CRUD (products, categories, orders, posts, profiles, reviews, coupons, addresses, shipments, shippings, customers, users, banners)
* [X] Admin Payments (gateway management, payment methods, payment details, gateway events/logs, refunds, capture/void actions, metrics)
* [X] Admin Inventory (variant stock, cart reservation estimate, quick stock updates, low-stock alerts)
* [X] Admin RBAC (roles, permissions, resource-scoped assignments)
* [X] Admin Audit Logs (admin action tracking)
* [X] Admin Email Management (templates, delivery status, resend actions)
* [X] Admin Settings (backend-backed store, SEO, API, notification, security settings)
* [X] Admin Monitoring (webhook failures, gateway errors, risky payments)
* [X] Customer Account Dashboard (order history, wishlist, settings)
* [X] i18n (pt-BR, en, es)
* [X] SEO (SSR, meta tags, sitemap, Open Graph)
* [X] PWA (offline caching, Workbox)
* [X] OpenAPI/Swagger documentation (`/api/docs`)
* [X] Backend tests (89 unit + integration tests)
* [X] Frontend component tests (Vitest)
* [X] E2E tests (Playwright)
* [X] Docker + Docker Compose
* [X] API rate limiting (per IP + per user)
* [X] CSRF protection
* [X] Cookies HttpOnly
* [X] API protection API key, error 401, 403, 422
* [X] JWT
* [X] Performance optimization (auth dedup, admin context reuse, pagination, indexes, in-memory caching)

## 📌 ToDo - Pending

### Core Commerce — Production Hardening
* [ ] Inventory management hardening (normalized stock reservations, stock movements, automatic out-of-stock handling)
* [ ] Transactional order emails (order confirmation, shipping updates, delivery confirmation, abandoned cart, back-in-stock)
* [ ] Payment production hardening (expired-session cleanup, provider failure alerts, PCI/security review)
* [ ] Order invoice generation (PDF)
* [ ] Tax calculation by region
* [ ] Multi-currency support with exchange rates
* [ ] Subscription products (recurring billing)
* [ ] Digital products (downloadable files, license keys)
* [ ] Gift cards and store credit
* [ ] Loyalty points / rewards program
* [ ] Product bundles and kits
* [ ] Shipping carrier API integration (USA: UPS/FedEx/USPS, Brazil: Correios/Melhor Envio/Jadlog)
* [ ] Conditional checkout logic based on user's country
* [ ] Weight/dimension unit mapping (Metric vs. Imperial)
* [ ] Multi-currency shipping cost support

### Search & Discovery
* [ ] Full-text product search (PostgreSQL tsvector or Meilisearch)
* [ ] Product filtering (price range, category, rating, availability)
* [ ] Product sorting (price, newest, best-selling, rating)
* [ ] Cursor-based API pagination
* [ ] Infinite scroll on product listings
* [ ] Product recommendations ("Customers also bought", "Similar products")
* [ ] Recently viewed products
* [ ] Product comparison (side-by-side specs)

### Infrastructure & Performance
* [ ] Redis cache layer (product listings, categories, sessions)
* [ ] S3-compatible image storage (AWS S3 / MinIO)
* [ ] CDN for static assets and images
* [ ] API response compression and caching (ETags)
* [ ] Background job queue (email sending, image processing)
* [ ] Health checks and readiness probes

### Customer Experience
* [ ] Social login (Google, Facebook, GitHub, Apple)
* [ ] Notification center (in-app alerts for orders, promotions)
* [ ] Dark mode toggle
* [ ] Breadcrumb navigation

### Integrations & APIs
* [ ] Webhook system (order events, payment events beyond provider webhooks)
* [ ] GraphQL API (alternative to REST)
* [ ] Real-time notifications (WebSocket / SSE)
* [ ] Email marketing integration (Mailchimp, SendGrid)
* [ ] Analytics integration (Google Analytics, Plausible)

### Admin & Analytics
* [ ] Granular RBAC hardening (Super Admin, Store Manager, Content Editor, Customer Support, Viewer)
* [ ] Database-backed permission rules (beyond code-defined Ability)
* [ ] Advanced analytics dashboard (conversion funnels, CLV, revenue by category)
* [ ] Customer segmentation (VIP, new, inactive)
* [ ] Bulk import/export (CSV/Excel for products, categories, customers)
* [ ] A/B testing framework
* [ ] Broader audit coverage across additional admin modules

### Security & DevOps
* [ ] OAuth 2.0 (social login)
* [ ] Two-factor authentication (2FA/TOTP)
* [ ] CI/CD pipeline (GitHub Actions)
* [ ] Error monitoring (Sentry integration)
* [ ] Automated database backups
* [ ] Container orchestration (Kubernetes / Docker Swarm)

### Future Vision
* [ ] Multi-vendor marketplace (seller storefronts, commissions)
* [ ] Mobile app (Flutter / React Native)
* [ ] AI-powered product descriptions (LLM integration)
* [ ] Headless CMS integration (Strapi / Directus)


---

Built by [gilcierweb](https://gilcierweb.com.br) - https://gilcierweb.com.br

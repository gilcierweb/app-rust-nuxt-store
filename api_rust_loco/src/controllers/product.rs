#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::collections::HashMap;

use axum::body::Bytes;
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::{FromQueryResult, Statement};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::cache::{
    invalidate_dashboard_cache, invalidate_products_cache, product_detail_cache, products_cache,
};
use crate::models::_entities::product_images::ActiveModel as ProductImageActiveModel;
use crate::models::_entities::products::{ActiveModel, Entity, Model};
use crate::models::products::{CategoryJson, ProductImageJson, ProductWithCategory};
use crate::utils::pagination::PaginationParams;
use crate::utils::slug::parameterize;

#[derive(Debug, FromQueryResult)]
struct ProductJoinedRow {
    id: i32,
    name: Option<String>,
    slug: Option<String>,
    sku: Option<String>,
    short_description: Option<String>,
    description: Option<String>,
    price: Option<Decimal>,
    cost_price: Option<Decimal>,
    compare_price: Option<Decimal>,
    featured: Option<bool>,
    active: Option<bool>,
    status: Option<i32>,
    category_id: Option<i32>,
    category_name: Option<String>,
    category_slug: Option<String>,
    image_id: Option<i32>,
    image: Option<String>,
    alt_text: Option<String>,
    image_active: Option<bool>,
    cover: Option<bool>,
    position: Option<i32>,
    image_product_id: Option<i32>,
}

fn product_from_row(row: &ProductJoinedRow) -> ProductWithCategory {
    ProductWithCategory {
        id: row.id,
        name: row.name.clone(),
        slug: row.slug.clone(),
        sku: row.sku.clone(),
        short_description: row.short_description.clone(),
        description: row.description.clone(),
        price: row.price,
        cost_price: row.cost_price,
        compare_price: row.compare_price,
        featured: row.featured,
        active: row.active,
        status: row.status,
        category: row.category_id.map(|id| CategoryJson {
            id,
            name: row.category_name.clone(),
            slug: row.category_slug.clone(),
        }),
        images: Some(Vec::new()),
    }
}

fn image_from_row(row: &ProductJoinedRow) -> Option<ProductImageJson> {
    Some(ProductImageJson {
        id: row.image_id?,
        image: row.image.clone(),
        alt_text: row.alt_text.clone(),
        active: row.image_active,
        cover: row.cover,
        position: row.position,
        product_id: row.image_product_id.unwrap_or(row.id),
    })
}

fn products_from_rows(rows: Vec<ProductJoinedRow>) -> Vec<ProductWithCategory> {
    let mut products = Vec::new();
    let mut positions = HashMap::new();

    for row in rows {
        let index = *positions.entry(row.id).or_insert_with(|| {
            products.push(product_from_row(&row));
            products.len() - 1
        });

        if let Some(image) = image_from_row(&row) {
            products[index]
                .images
                .get_or_insert_with(Vec::new)
                .push(image);
        }
    }

    products
}

fn product_select_sql(from_clause: &str, image_join_filter: &str) -> String {
    format!(
        r#"
        SELECT
            p.id,
            p.name,
            p.slug,
            p.sku,
            p.short_description,
            p.description,
            p.price,
            p.cost_price,
            p.compare_price,
            p.featured,
            p.active,
            p.status,
            c.id AS category_id,
            c.name AS category_name,
            c.slug AS category_slug,
            pi.id AS image_id,
            pi.image,
            pi.alt_text,
            pi.active AS image_active,
            pi.cover,
            pi.position,
            pi.product_id AS image_product_id
        FROM {from_clause} p
        LEFT JOIN categories c ON c.id = p.category_id
        LEFT JOIN product_images pi
            ON pi.product_id = p.id
            {image_join_filter}
        ORDER BY p.id ASC, pi.position ASC, pi.id ASC
        "#
    )
}

fn product_list_select_sql() -> String {
    let from_clause = r#"
        (
            SELECT *
            FROM products
            ORDER BY id ASC
            LIMIT $1 OFFSET $2
        )
    "#;
    format!(
        r#"
        SELECT
            p.id,
            p.name,
            p.slug,
            p.sku,
            p.short_description,
            p.description,
            p.price,
            p.cost_price,
            p.compare_price,
            p.featured,
            p.active,
            p.status,
            c.id AS category_id,
            c.name AS category_name,
            c.slug AS category_slug,
            pi.id AS image_id,
            pi.image,
            pi.alt_text,
            pi.active AS image_active,
            pi.cover,
            pi.position,
            pi.product_id AS image_product_id
        FROM {from_clause} p
        LEFT JOIN categories c ON c.id = p.category_id
        LEFT JOIN LATERAL (
            SELECT id, product_id, image, alt_text, active, cover, position
            FROM product_images
            WHERE product_id = p.id
            ORDER BY cover DESC, position ASC NULLS LAST, id ASC
            LIMIT 1
        ) pi ON TRUE
        ORDER BY p.id ASC
        "#
    )
}

fn product_detail_select_sql() -> String {
    product_select_sql(
        r#"
        (
            SELECT *
            FROM products
            WHERE id = $1
        )
        "#,
        "",
    )
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub sku: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    #[serde(with = "crate::utils::decimal::opt")]
    pub price: Option<Decimal>,
    #[serde(with = "crate::utils::decimal::opt")]
    pub cost_price: Option<Decimal>,
    #[serde(with = "crate::utils::decimal::opt")]
    pub compare_price: Option<Decimal>,
    pub featured: Option<bool>,
    pub active: Option<bool>,
    pub status: Option<i32>,
    pub category_id: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.slug = Set(self.slug.as_deref().map(parameterize));
        item.sku = Set(self.sku.clone());
        item.short_description = Set(self.short_description.clone());
        item.description = Set(self.description.clone());
        item.price = Set(self.price.clone());
        item.cost_price = Set(self.cost_price.clone());
        item.compare_price = Set(self.compare_price.clone());
        item.featured = Set(self.featured.clone());
        item.active = Set(self.active.clone());
        item.status = Set(self.status.clone());
        item.category_id = Set(self.category_id.clone());
    }
}
async fn save_image(
    content: Bytes,
    filename: String,
    product_id: i32,
    position: i32,
    _ctx: &AppContext,
) -> Result<ProductImageActiveModel> {
    let path_buf = PathBuf::from(&filename);
    let extension = path_buf
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("jpg");
    let unique_name = format!("{}_{}.{}", Uuid::new_v4(), position, extension);

    // Save file directly to filesystem
    let upload_dir = "uploads/products";
    tokio::fs::create_dir_all(upload_dir).await.map_err(|e| {
        tracing::error!(error = ?e, "could not create upload directory");
        Error::BadRequest(t!("errors.upload_dir_creation").into())
    })?;

    let file_path = format!("{}/{}", upload_dir, unique_name);
    tokio::fs::write(&file_path, &content).await.map_err(|e| {
        tracing::error!(error = ?e, "could not write file");
        Error::BadRequest(t!("errors.file_write").into())
    })?;

    let now = chrono::Utc::now();
    Ok(ProductImageActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        image: sea_orm::ActiveValue::Set(Some(unique_name)), // Caminho relativo salvo no banco
        alt_text: sea_orm::ActiveValue::Set(Some(filename)),
        active: sea_orm::ActiveValue::Set(Some(true)),
        cover: sea_orm::ActiveValue::Set(Some(position == 0)),
        position: sea_orm::ActiveValue::Set(Some(position)),
        product_id: sea_orm::ActiveValue::Set(product_id),
        created_at: sea_orm::ActiveValue::Set(now.into()),
        updated_at: sea_orm::ActiveValue::Set(now.into()),
    })
}

#[utoipa::path(
    get,
    path = "/api/products/",
    responses(
        (status = 200, description = "List of products with their categories")
    ),
    tag = "Products"
)]
pub async fn get_products_with_categories(
    State(ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<impl IntoResponse> {
    let cache_key = format!(
        "list:{}:{}",
        pagination.page_index(),
        pagination.page_size()
    );
    if let Some(value) = products_cache().get(&cache_key) {
        return format::json(value);
    }

    let rows = ProductJoinedRow::find_by_statement(Statement::from_sql_and_values(
        ctx.db.get_database_backend(),
        &product_list_select_sql(),
        [
            (pagination.page_size() as i64).into(),
            pagination.offset().into(),
        ],
    ))
    .all(&ctx.db)
    .await?;

    let data = products_from_rows(rows);
    let data = Arc::new(data);
    products_cache().insert(cache_key, Arc::clone(&data));
    format::json(data)
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[utoipa::path(
    get,
    path = "/api/products/list", // Wait, the route in routes() is actually commented out for list, but let's assume it might be used or we can just document the get_one
    responses(
        (status = 200, description = "Raw list of all products")
    ),
    tag = "Products"
)]
#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    mut multipart: axum::extract::Multipart,
) -> Result<Response> {
    let mut product = ActiveModel {
        ..Default::default()
    };
    let mut images = Vec::new();
    let mut position = 0;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!(error = ?e, "could not read multipart");
        Error::BadRequest(t!("errors.multipart_read").into())
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name.starts_with("image") {
            // Process image field immediately since Field cannot be stored
            let filename = field.file_name().unwrap_or("image.jpg").to_string();
            let content = field.bytes().await.map_err(|e| {
                tracing::error!(error = ?e, "could not read file bytes");
                Error::BadRequest(t!("errors.file_bytes_read").into())
            })?;

            if !content.is_empty() {
                images.push((content, filename, position));
                position += 1;
            }
        } else {
            if let Ok(text) = field.text().await {
                match name.as_str() {
                    "name" => product.name = Set(Some(text)),
                    "slug" => product.slug = Set(Some(parameterize(&text))),
                    "sku" => product.sku = Set(Some(text)),
                    "short_description" => product.short_description = Set(Some(text)),
                    "description" => product.description = Set(Some(text)),
                    "price" => {
                        if let Ok(p) = text.parse::<Decimal>() {
                            product.price = Set(Some(p))
                        }
                    }
                    "cost_price" => {
                        if let Ok(c) = text.parse::<Decimal>() {
                            product.cost_price = Set(Some(c))
                        }
                    }
                    "compare_price" => {
                        if let Ok(cp) = text.parse::<Decimal>() {
                            product.compare_price = Set(Some(cp))
                        }
                    }
                    "featured" => product.featured = Set(Some(text == "true")),
                    "active" => product.active = Set(Some(text == "true")),
                    "status" => {
                        if let Ok(s) = text.parse::<i32>() {
                            product.status = Set(Some(s))
                        }
                    }
                    "category_id" => {
                        if let Ok(cid) = text.parse::<i32>() {
                            product.category_id = Set(cid)
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let saved_product = product
        .insert(&ctx.db)
        .await
        .map_err(|e| Error::Message(t!("errors.product_save", error = e.to_string()).into()))?;

    let images_count = images.len();
    for (content, filename, pos) in images {
        let image_model = save_image(content, filename, saved_product.id, pos, &ctx).await?;
        image_model
            .insert(&ctx.db)
            .await
            .map_err(|e| Error::Message(t!("errors.image_save", error = e.to_string()).into()))?;
    }

    info!(
        "Produto criado com {} imagens: {:?}",
        images_count, saved_product
    );
    invalidate_products_cache();
    invalidate_dashboard_cache();
    format::json(saved_product)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    invalidate_products_cache();
    invalidate_dashboard_cache();
    format::json(item)
}

#[utoipa::path(
    delete,
    path = "/api/products/{id}",
    params(
        ("id" = i32, Path, description = "Product ID")
    ),
    responses(
        (status = 200, description = "Product removed successfully"),
        (status = 404, description = "Product not found")
    ),
    tag = "Products"
)]
#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    invalidate_products_cache();
    invalidate_dashboard_cache();
    format::empty()
}

#[utoipa::path(
    get,
    path = "/api/products/{id}",
    params(
        ("id" = i32, Path, description = "Product ID")
    ),
    responses(
        (status = 200, description = "Product with category returned"),
        (status = 404, description = "Product not found")
    ),
    tag = "Products"
)]
#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let cache_key = format!("detail:{id}");
    if let Some(value) = product_detail_cache().get(&cache_key) {
        return format::json(value);
    }

    let rows = ProductJoinedRow::find_by_statement(Statement::from_sql_and_values(
        ctx.db.get_database_backend(),
        &product_detail_select_sql(),
        [id.into()],
    ))
    .all(&ctx.db)
    .await?;

    let Some(product_with_category) = products_from_rows(rows).into_iter().next() else {
        return Err(Error::NotFound);
    };

    let product_with_category = Arc::new(product_with_category);
    product_detail_cache().insert(cache_key, Arc::clone(&product_with_category));
    format::json(product_with_category)
}

pub fn routes() -> Routes {
    routes_with_prefix("api/products/")
}

pub fn admin_routes() -> Routes {
    routes_with_prefix("api/admin/products/")
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new()
        .prefix(prefix)
        // .add("/", get(list))
        .add("/", get(get_products_with_categories))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}

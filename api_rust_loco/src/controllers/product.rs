#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;
use rust_decimal::Decimal;
use sea_orm::QueryOrder;
use std::path::PathBuf;
use uuid::Uuid;
use axum::body::Bytes;

use crate::models::_entities::categories::Entity as Categories;
use crate::models::_entities::products::{ActiveModel, Entity, Model};
use crate::models::_entities::product_images::{ActiveModel as ProductImageActiveModel, Entity as ProductImageEntity};
use crate::models::products::{ProductWithCategory, Products};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub sku: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub compare_price: Option<Decimal>,
    pub featured: Option<bool>,
    pub active: Option<bool>,
    pub status: Option<i32>,
    pub category_id: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.slug = Set(self.slug.clone());
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
    _ctx: &AppContext
) -> Result<ProductImageActiveModel> {
    let path_buf = PathBuf::from(&filename);
    let extension = path_buf.extension().and_then(|ext| ext.to_str()).unwrap_or("jpg");
    let unique_name = format!("{}_{}.{}", Uuid::new_v4(), position, extension);
    
    // Save file directly to filesystem
    let upload_dir = "uploads/products";
    tokio::fs::create_dir_all(upload_dir).await.map_err(|e| {
        tracing::error!(error = ?e, "could not create upload directory");
        Error::BadRequest("could not create upload directory".into())
    })?;
    
    let file_path = format!("{}/{}", upload_dir, unique_name);
    tokio::fs::write(&file_path, &content).await.map_err(|e| {
        tracing::error!(error = ?e, "could not write file");
        Error::BadRequest("could not write file".into())
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

pub async fn get_products_with_categories(
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let products = Products::find()
        .find_also_related(Categories)
        .all(&ctx.db)
        .await?;

    let mut data: Vec<ProductWithCategory> = products.into_iter().map(Into::into).collect();
    
    // Load images for each product
    for product_with_category in &mut data {
        let images = ProductImageEntity::find()
            .filter(crate::models::_entities::product_images::Column::ProductId.eq(product_with_category.id))
            .order_by_asc(crate::models::_entities::product_images::Column::Position)
            .all(&ctx.db)
            .await?;
            
        product_with_category.images = Some(images.into_iter().map(|img| crate::models::products::ProductImageJson {
            id: img.id,
            image: img.image,
            alt_text: img.alt_text,
            active: img.active,
            cover: img.cover,
            position: img.position,
            product_id: img.product_id,
        }).collect());
    }

    format::json(data)
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, mut multipart: axum::extract::Multipart) -> Result<Response> {
    let mut product = ActiveModel { ..Default::default() };
    let mut images = Vec::new();
    let mut position = 0;
    
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!(error = ?e, "could not read multipart");
        Error::BadRequest("could not read multipart".into())
    })? {
        let name = field.name().unwrap_or("").to_string();
        
        if name.starts_with("image") {
            // Process image field immediately since Field cannot be stored
            let filename = field.file_name().unwrap_or("image.jpg").to_string();
            let content = field.bytes().await.map_err(|e| {
                tracing::error!(error = ?e, "could not read file bytes");
                Error::BadRequest("could not read file bytes".into())
            })?;
            
            if !content.is_empty() {
                images.push((content, filename, position));
                position += 1;
            }
        } else {
            if let Ok(text) = field.text().await {
                match name.as_str() {
                    "name" => product.name = Set(Some(text)),
                    "slug" => product.slug = Set(Some(text)),
                    "sku" => product.sku = Set(Some(text)),
                    "short_description" => product.short_description = Set(Some(text)),
                    "description" => product.description = Set(Some(text)),
                    "price" => if let Ok(p) = text.parse::<Decimal>() { product.price = Set(Some(p)) },
                    "cost_price" => if let Ok(c) = text.parse::<Decimal>() { product.cost_price = Set(Some(c)) },
                    "compare_price" => if let Ok(cp) = text.parse::<Decimal>() { product.compare_price = Set(Some(cp)) },
                    "featured" => product.featured = Set(Some(text == "true")),
                    "active" => product.active = Set(Some(text == "true")),
                    "status" => if let Ok(s) = text.parse::<i32>() { product.status = Set(Some(s)) },
                    "category_id" => if let Ok(cid) = text.parse::<i32>() { product.category_id = Set(cid) },
                    _ => {}
                }
            }
        }
    }
    
    let saved_product = product.insert(&ctx.db).await.map_err(|e| Error::Message(format!("Erro ao salvar produto: {}", e)))?;
    
    let images_count = images.len();
    for (content, filename, pos) in images {
        let image_model = save_image(content, filename, saved_product.id, pos, &ctx).await?;
        image_model.insert(&ctx.db).await.map_err(|e| Error::Message(format!("Erro ao salvar imagem: {}", e)))?;
    }
    
    info!("Produto criado com {} imagens: {:?}", images_count, saved_product);
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
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let result = Products::find_by_id(id)
        .find_also_related(Categories)
        .one(&ctx.db)
        .await?;

    match result {
        Some((product, category)) => {
            let mut product_with_category = ProductWithCategory::from((product, category));
            
            // Load images for the product
            let images = ProductImageEntity::find()
                .filter(crate::models::_entities::product_images::Column::ProductId.eq(product_with_category.id))
                .order_by_asc(crate::models::_entities::product_images::Column::Position)
                .all(&ctx.db)
                .await?;
                
            product_with_category.images = Some(images.into_iter().map(|img| crate::models::products::ProductImageJson {
                id: img.id,
                image: img.image,
                alt_text: img.alt_text,
                active: img.active,
                cover: img.cover,
                position: img.position,
                product_id: img.product_id,
            }).collect());
            
            format::json(product_with_category)
        },
        None => Err(Error::NotFound),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products/")
        // .add("/", get(list))
        .add("/", get(get_products_with_categories))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
}

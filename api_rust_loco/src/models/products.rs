use sea_orm::entity::prelude::*;
pub use super::_entities::products::{ActiveModel, Model, Entity};
pub type Products = Entity;

use super::_entities::categories::Model as CategoryModel;
// use super::_entities::product_images::Model as ProductImageModel;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProductWithCategory {
    pub id: i32,
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
    pub category: Option<CategoryJson>,
    pub images: Option<Vec<ProductImageJson>>,
}

#[derive(Debug, Serialize)]
pub struct CategoryJson {
    pub id: i32,
    pub name: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductImageJson {
    pub id: i32,
    pub image: Option<String>,
    pub alt_text: Option<String>,
    pub active: Option<bool>,
    pub cover: Option<bool>,
    pub position: Option<i32>,
    pub product_id: i32,
}

impl From<(Model, Option<CategoryModel>)> for ProductWithCategory {
    fn from((product, category): (Model, Option<CategoryModel>)) -> Self {
        Self {
            id: product.id,
            name: product.name,
            slug: product.slug,          
            sku: product.sku,
            short_description: product.short_description,
            description: product.description,
            price: product.price,
            cost_price: product.cost_price,
            compare_price: product.compare_price,
            featured: product.featured,
            active: product.active,
            status: product.status,
            category: category.map(|c| CategoryJson {
                id: c.id,
                name: c.name,
                slug: c.slug,
            }),
            images: None, // Will be populated separately
        }
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}


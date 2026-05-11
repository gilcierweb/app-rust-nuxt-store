use chrono::Utc;
use loco_rs::Result;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::product_variant_options::{ActiveModel, Entity};
use crate::models::_entities::product_variants::Entity as VariantEntity;
use crate::models::_entities::variant_options::Entity as OptionEntity;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Product variant options already exist, skipping.");
        return Ok(());
    }

    let variants = VariantEntity::find().all(db).await?;
    let variant_options = OptionEntity::find().all(db).await?;

    if variants.is_empty() || variant_options.is_empty() {
        tracing::warn!("No product variants or variant options found. Product variant options cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();
    let size_values = vec!["XS", "S", "M", "L", "XL", "XXL"];
    let color_values = vec!["Red", "Blue", "Green", "Black", "White"];

    for variant in &variants {
        for (i, opt) in variant_options.iter().enumerate() {
            let value = match opt.name.as_deref() {
                Some("Size") => size_values[i % size_values.len()],
                Some("Color") => color_values[i % color_values.len()],
                Some("Material") => "Cotton",
                Some("Style") => "Casual",
                _ => "Standard",
            };

            let variant_option = ActiveModel {
                value: Set(Some(value.to_string())),
                product_variant_id: Set(variant.id),
                variant_option_id: Set(opt.id),
                created_at: Set(now.into()),
                updated_at: Set(now.into()),
                ..Default::default()
            };
            variant_option.insert(db).await?;
        }
    }

    tracing::info!("Product variant options generated for {} variants", variants.len());
    Ok(())
}

use chrono::Utc;
use loco_rs::Result;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::variant_options::{ActiveModel, Entity};

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Variant options already exist, skipping.");
        return Ok(());
    }

    let now = Utc::now();

    let options = vec![
        ("Size", "XS,S,M,L,XL,XXL"),
        ("Color", "Red,Blue,Green,Black,White,Yellow,Purple,Pink"),
        ("Material", "Cotton,Polyester,Wool,Silk,Linen"),
        ("Style", "Casual,Formal,Sport,Vintage,Modern"),
    ];

    for (name, values) in options {
        let option = ActiveModel {
            name: Set(Some(name.to_string())),
            values: Set(Some(values.to_string())),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        option.insert(db).await?;
    }

    tracing::info!("4 variant options generated (Size, Color, Material, Style)");
    Ok(())
}

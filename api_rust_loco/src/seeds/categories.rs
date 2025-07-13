use chrono::{ Utc};
use fakeit::{company, unique, words};
use loco_rs::Result;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::_entities::categories::{ActiveModel, Entity};

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Categories already exist, ignoring.");
        return Ok(());
    }

    let now = Utc::now();
    let date_current = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();

    let mut root_ids = Vec::new();
    for i in 0..5 {
        let name_val = company::company();
        let slug_val = unique::uuid_v4();
        let active = date_current.subsec_nanos() % 2 == 0;     
        let paragraph = words::paragraph(5, 4, 11, "\n".to_string());

        let category = ActiveModel {
            name: Set(Some(name_val)),
            slug: Set(Some(slug_val)),
            description: Set(Some(paragraph)),
            active: Set(Some(active)),
            position: Set(Some(i)),
            parent_id: Set(None),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };

        let inserted = category.insert(db).await?;
        root_ids.push(inserted.id);
    }

    for i in 0..15 {
        let name_val = company::company();
        let slug_val = unique::uuid_v4();
                let active = date_current.subsec_nanos() % 2 == 0;     

        let paragraph = words::paragraph(5, 4, 11, "\n".to_string());
        let index = {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            (now.subsec_nanos() as usize) % root_ids.len()
        };

        let parent_id = root_ids[index];

        let category = ActiveModel {
            name: Set(Some(name_val)),
            slug: Set(Some(slug_val)),
            description: Set(Some(paragraph)),
            active: Set(Some(active)),
            position: Set(Some(i + 10)),
            parent_id: Set(Some(parent_id)),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };

        category.insert(db).await?;
    }

    tracing::info!("20 categories generated with fakeit (5 root, 15 children)");
    Ok(())
}

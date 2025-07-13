use chrono::Utc;
use loco_rs::Result;
use fakeit::words;
use sea_orm::{ActiveModelTrait, Set, EntityTrait, PaginatorTrait};
use crate::models::_entities::posts::{ActiveModel as Post, Entity as Posts};
use crate::models::post_status::PostStatus;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Posts::find().count(db).await?;
    if count > 0 {
        tracing::info!("Posts already exist, ignoring.");
        return Ok(());
    }


    for _ in 0..10 {
        let paragraph = words::paragraph(5, 4, 11, "\n".to_string());
        let post = Post {
            title: Set(Some(fakeit::words::sentence(3))),
            content: Set(Some(paragraph)),
            status: Set(Some(PostStatus::Published.into())),
            user_id: Set(1),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };

        post.insert(db).await?;
    }

    tracing::info!("10 posts generated with fakeit");
    Ok(())
}

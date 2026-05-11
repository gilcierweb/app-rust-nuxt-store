use chrono::Utc;
use loco_rs::Result;
use fakeit::words;
use rand::Rng;
use sea_orm::{ActiveModelTrait, Set, EntityTrait, PaginatorTrait};
use crate::models::_entities::posts::{ActiveModel as Post, Entity as Posts};
use crate::models::_entities::users::Entity as UserEntity;
use crate::models::post_status::PostStatus;

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Posts::find().count(db).await?;
    if count > 0 {
        tracing::info!("Posts already exist, skipping.");
        return Ok(());
    }

    let users = UserEntity::find().all(db).await?;
    if users.is_empty() {
        tracing::warn!("No users found. Posts cannot be seeded.");
        return Ok(());
    }

    let now = Utc::now();

    for _ in 0..10 {
        let user = &users[rand::rng().random_range(0..users.len())];
        let paragraph = words::paragraph(5, 4, 11, "\n".to_string());
        let post = Post {
            title: Set(Some(fakeit::words::sentence(3))),
            content: Set(Some(paragraph)),
            status: Set(Some(PostStatus::Published.into())),
            user_id: Set(user.id),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };

        post.insert(db).await?;
    }

    tracing::info!("10 posts generated with fakeit");
    Ok(())
}

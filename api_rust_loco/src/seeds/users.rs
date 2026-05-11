use chrono::Utc;
use fakeit::{name, unique, password};
use loco_rs::Result;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set, NotSet};
use uuid::Uuid;

use crate::models::_entities::users::{ActiveModel, Entity};

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Users already exist, skipping.");
        return Ok(());
    }

    let now = Utc::now();

    // Create first admin user with known credentials
    let admin = ActiveModel {
        id: NotSet,
        pid: Set(Uuid::new_v4()),
        email: Set("admin@example.com".to_string()),
        password: Set(password::generate(true, true, true, 12)),
        api_key: Set(unique::uuid_v4()),
        name: Set("Administrator".to_string()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        reset_token: Set(None),
        reset_sent_at: Set(None),
        email_verification_token: Set(None),
        email_verification_sent_at: Set(None),
        email_verified_at: Set(Some(now.into())),
        magic_link_token: Set(None),
        magic_link_expiration: Set(None),
    };
    admin.insert(db).await?;

    // Create additional fake users
    for i in 0..9 {
        let first = name::first();
        let last = name::last();
        let email = format!("{}.{}{}@example.com", first.to_lowercase(), last.to_lowercase(), i);
        let full_name = format!("{} {}", first, last);

        let user = ActiveModel {
            id: NotSet,
            pid: Set(Uuid::new_v4()),
            email: Set(email),
            password: Set(password::generate(true, true, true, 12)),
            api_key: Set(unique::uuid_v4()),
            name: Set(full_name),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            reset_token: Set(None),
            reset_sent_at: Set(None),
            email_verification_token: Set(None),
            email_verification_sent_at: Set(None),
            email_verified_at: Set(Some(now.into())),
            magic_link_token: Set(None),
            magic_link_expiration: Set(None),
        };
        user.insert(db).await?;
    }

    tracing::info!("10 users generated (1 admin + 9 fake users)");
    Ok(())
}

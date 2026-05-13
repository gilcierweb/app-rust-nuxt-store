use chrono::Utc;
use fakeit::{name, password, unique};
use loco_rs::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::models::_entities::{
    roles,
    users::{ActiveModel, Entity},
    users_roles,
};

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Users already exist, skipping.");
        return Ok(());
    }

    let now = Utc::now();

    let admin_role = find_or_create_role(db, "admin").await?;
    let user_role = find_or_create_role(db, "user").await?;

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
    let admin = admin.insert(db).await?;

    users_roles::ActiveModel {
        user_id: Set(admin.id),
        role_id: Set(admin_role.id),
    }
    .insert(db)
    .await?;

    // Create additional fake users
    for i in 0..9 {
        let first = name::first();
        let last = name::last();
        let email = format!(
            "{}.{}{}@example.com",
            first.to_lowercase(),
            last.to_lowercase(),
            i
        );
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
        let user = user.insert(db).await?;
        users_roles::ActiveModel {
            user_id: Set(user.id),
            role_id: Set(user_role.id),
        }
        .insert(db)
        .await?;
    }

    tracing::info!("10 users generated (1 admin + 9 fake users)");
    Ok(())
}

async fn find_or_create_role(db: &sea_orm::DatabaseConnection, name: &str) -> Result<roles::Model> {
    if let Some(role) = roles::Entity::find()
        .filter(roles::Column::Name.eq(name))
        .filter(roles::Column::ResourceType.is_null())
        .filter(roles::Column::ResourceId.is_null())
        .one(db)
        .await?
    {
        return Ok(role);
    }

    let now = Utc::now();
    Ok(roles::ActiveModel {
        name: Set(name.to_string()),
        resource_type: Set(None),
        resource_id: Set(None),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(db)
    .await?)
}

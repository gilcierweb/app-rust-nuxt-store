use chrono::Utc;
use loco_rs::{hash, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::_entities::{
    roles,
    users::{ActiveModel, Entity, Model},
    users_roles,
};

const TEST_PASSWORD: &str = "Password123!";

struct RoleSeed {
    name: &'static str,
    resource_type: Option<&'static str>,
    resource_id: Option<i32>,
}

struct TestUserSeed {
    email: &'static str,
    name: &'static str,
    roles: &'static [&'static str],
}

const ROLE_SEEDS: &[RoleSeed] = &[
    RoleSeed {
        name: "admin",
        resource_type: None,
        resource_id: None,
    },
    RoleSeed {
        name: "user",
        resource_type: None,
        resource_id: None,
    },
    RoleSeed {
        name: "store_manager",
        resource_type: None,
        resource_id: None,
    },
    RoleSeed {
        name: "editor",
        resource_type: None,
        resource_id: None,
    },
    RoleSeed {
        name: "support",
        resource_type: None,
        resource_id: None,
    },
    RoleSeed {
        name: "viewer",
        resource_type: None,
        resource_id: None,
    },
    RoleSeed {
        name: "manager",
        resource_type: Some("store"),
        resource_id: Some(1),
    },
];

const TEST_USERS: &[TestUserSeed] = &[
    TestUserSeed {
        email: "admin@example.com",
        name: "Admin User",
        roles: &["admin"],
    },
    TestUserSeed {
        email: "manager@example.com",
        name: "Store Manager",
        roles: &["user", "store_manager", "manager:store:1"],
    },
    TestUserSeed {
        email: "editor@example.com",
        name: "Content Editor",
        roles: &["user", "editor"],
    },
    TestUserSeed {
        email: "support@example.com",
        name: "Support Agent",
        roles: &["user", "support"],
    },
    TestUserSeed {
        email: "customer@example.com",
        name: "Customer User",
        roles: &["user"],
    },
    TestUserSeed {
        email: "viewer@example.com",
        name: "Readonly Viewer",
        roles: &["user", "viewer"],
    },
];

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let mut seeded_roles = HashMap::new();
    for role_seed in ROLE_SEEDS {
        let role = find_or_create_role(
            db,
            role_seed.name,
            role_seed.resource_type,
            role_seed.resource_id,
        )
        .await?;
        seeded_roles.insert(
            role_key(
                role_seed.name,
                role_seed.resource_type,
                role_seed.resource_id,
            ),
            role,
        );
    }

    for user_seed in TEST_USERS {
        let user = find_or_create_test_user(db, user_seed).await?;
        for role_ref in user_seed.roles {
            let Some(role) = seeded_roles.get(*role_ref) else {
                tracing::warn!(
                    user_email = user_seed.email,
                    role = role_ref,
                    "Role reference not found for seeded user"
                );
                continue;
            };
            assign_role(db, user.id, role.id).await?;
        }
    }

    tracing::info!(
        password = TEST_PASSWORD,
        "Seeded test users with known credentials"
    );
    for user_seed in TEST_USERS {
        tracing::info!(
            email = user_seed.email,
            password = TEST_PASSWORD,
            roles = user_seed.roles.join(","),
            "Seed test user"
        );
    }

    Ok(())
}

async fn find_or_create_test_user(
    db: &sea_orm::DatabaseConnection,
    seed: &TestUserSeed,
) -> Result<Model> {
    let now = Utc::now();
    let password_hash = hash::hash_password(TEST_PASSWORD)?;

    if let Some(user) = Entity::find()
        .filter(crate::models::_entities::users::Column::Email.eq(seed.email))
        .one(db)
        .await?
    {
        let mut active = user.into_active_model();
        active.name = Set(seed.name.to_string());
        active.password = Set(password_hash);
        active.email_verified_at = Set(Some(now.into()));
        active.updated_at = Set(now.into());
        return Ok(active.update(db).await?);
    }

    let user = ActiveModel {
        id: Default::default(),
        pid: Set(Uuid::new_v4()),
        email: Set(seed.email.to_string()),
        password: Set(password_hash),
        api_key: Set(format!("lo-{}", Uuid::new_v4())),
        name: Set(seed.name.to_string()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        reset_token: Set(None),
        reset_sent_at: Set(None),
        email_verification_token: Set(None),
        email_verification_sent_at: Set(None),
        email_verified_at: Set(Some(now.into())),
        magic_link_token: Set(None),
        magic_link_expiration: Set(None),
    }
    .insert(db)
    .await?;

    Ok(user)
}

async fn find_or_create_role(
    db: &sea_orm::DatabaseConnection,
    name: &str,
    resource_type: Option<&str>,
    resource_id: Option<i32>,
) -> Result<roles::Model> {
    let mut query = roles::Entity::find().filter(roles::Column::Name.eq(name));
    query = match resource_type {
        Some(resource_type) => query.filter(roles::Column::ResourceType.eq(resource_type)),
        None => query.filter(roles::Column::ResourceType.is_null()),
    };
    query = match resource_id {
        Some(resource_id) => query.filter(roles::Column::ResourceId.eq(resource_id)),
        None => query.filter(roles::Column::ResourceId.is_null()),
    };

    if let Some(role) = query.one(db).await? {
        return Ok(role);
    }

    let now = Utc::now();
    Ok(roles::ActiveModel {
        name: Set(name.to_string()),
        resource_type: Set(resource_type.map(str::to_string)),
        resource_id: Set(resource_id),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
        ..Default::default()
    }
    .insert(db)
    .await?)
}

async fn assign_role(db: &sea_orm::DatabaseConnection, user_id: i32, role_id: i32) -> Result<()> {
    let exists = users_roles::Entity::find()
        .filter(users_roles::Column::UserId.eq(user_id))
        .filter(users_roles::Column::RoleId.eq(role_id))
        .one(db)
        .await?
        .is_some();
    if exists {
        return Ok(());
    }

    users_roles::ActiveModel {
        user_id: Set(user_id),
        role_id: Set(role_id),
    }
    .insert(db)
    .await?;

    Ok(())
}

fn role_key(name: &str, resource_type: Option<&str>, resource_id: Option<i32>) -> String {
    match (resource_type, resource_id) {
        (Some(resource_type), Some(resource_id)) => {
            format!("{name}:{resource_type}:{resource_id}")
        }
        _ => name.to_string(),
    }
}

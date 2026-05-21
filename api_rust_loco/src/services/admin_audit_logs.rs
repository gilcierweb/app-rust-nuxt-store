use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait, EntityTrait};

use crate::models::_entities::{admin_audit_logs, users};

pub async fn record<C>(
    db: &C,
    actor_user_id: i32,
    action: &str,
    resource_type: &str,
    resource_id: Option<i32>,
    resource_label: Option<String>,
    message: Option<String>,
) -> Result<admin_audit_logs::Model>
where
    C: ConnectionTrait,
{
    let action = action.trim();
    let resource_type = resource_type.trim();

    if action.is_empty() || resource_type.is_empty() {
        return Err(Error::BadRequest(
            "audit action and resource type are required".into(),
        ));
    }

    let actor = users::Entity::find_by_id(actor_user_id).one(db).await?;
    let (actor_name, actor_email) =
        actor
            .map(|user| (user.name, user.email))
            .unwrap_or_else(|| {
                (
                    "Unknown Admin".to_string(),
                    "unknown@example.com".to_string(),
                )
            });

    let now = chrono::Utc::now().into();
    Ok(admin_audit_logs::ActiveModel {
        actor_user_id: Set(Some(actor_user_id)),
        actor_name: Set(actor_name),
        actor_email: Set(actor_email),
        action: Set(action.to_string()),
        resource_type: Set(resource_type.to_string()),
        resource_id: Set(resource_id),
        resource_label: Set(resource_label),
        message: Set(message),
        created_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await?)
}

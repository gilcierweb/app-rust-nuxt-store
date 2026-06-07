#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::collections::HashMap;

use axum::debug_handler;
use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AdminSession;
use crate::models::_entities::admin_settings;
use crate::services::admin_audit_logs;

const VALUE_TYPE_STRING: i16 = 1;
const VALUE_TYPE_BOOLEAN: i16 = 2;

#[derive(Clone, Copy)]
struct SettingDefinition {
    namespace: &'static str,
    key: &'static str,
    label: &'static str,
    value_type: i16,
    default_value: &'static str,
}

const SETTINGS: &[SettingDefinition] = &[
    SettingDefinition {
        namespace: "general",
        key: "store_name",
        label: "Store Name",
        value_type: VALUE_TYPE_STRING,
        default_value: "App Rust Nuxt Store",
    },
    SettingDefinition {
        namespace: "general",
        key: "support_email",
        label: "Support Email",
        value_type: VALUE_TYPE_STRING,
        default_value: "support@example.com",
    },
    SettingDefinition {
        namespace: "general",
        key: "default_currency",
        label: "Default Currency",
        value_type: VALUE_TYPE_STRING,
        default_value: "BRL",
    },
    SettingDefinition {
        namespace: "seo",
        key: "meta_title",
        label: "Meta Title",
        value_type: VALUE_TYPE_STRING,
        default_value: "App Rust Nuxt Store",
    },
    SettingDefinition {
        namespace: "seo",
        key: "meta_description",
        label: "Meta Description",
        value_type: VALUE_TYPE_STRING,
        default_value: "Modern ecommerce store powered by Rust and Nuxt.",
    },
    SettingDefinition {
        namespace: "api",
        key: "public_base_url",
        label: "Public Base URL",
        value_type: VALUE_TYPE_STRING,
        default_value: "http://localhost:5150",
    },
    SettingDefinition {
        namespace: "notifications",
        key: "order_emails_enabled",
        label: "Order Emails",
        value_type: VALUE_TYPE_BOOLEAN,
        default_value: "false",
    },
    SettingDefinition {
        namespace: "notifications",
        key: "shipping_emails_enabled",
        label: "Shipping Emails",
        value_type: VALUE_TYPE_BOOLEAN,
        default_value: "false",
    },
    SettingDefinition {
        namespace: "security",
        key: "admin_api_keys_enabled",
        label: "Admin API Keys",
        value_type: VALUE_TYPE_BOOLEAN,
        default_value: "false",
    },
];

#[derive(Clone, Debug, Serialize)]
pub struct SettingJson {
    pub namespace: String,
    pub key: String,
    pub label: String,
    pub value_type: i16,
    pub value: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct SettingsGroupJson {
    pub namespace: String,
    pub label: String,
    pub settings: Vec<SettingJson>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SettingsJson {
    pub groups: Vec<SettingsGroupJson>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateSettingJson {
    pub namespace: String,
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateSettingsJson {
    pub settings: Vec<UpdateSettingJson>,
}

fn group_label(namespace: &str) -> &'static str {
    match namespace {
        "general" => "General",
        "seo" => "SEO",
        "api" => "API",
        "notifications" => "Notifications",
        "security" => "Security",
        _ => "Settings",
    }
}

fn setting_definition(namespace: &str, key: &str) -> Option<SettingDefinition> {
    SETTINGS
        .iter()
        .find(|setting| setting.namespace == namespace && setting.key == key)
        .copied()
}

fn normalize_value(definition: SettingDefinition, value: String) -> Result<String> {
    match definition.value_type {
        VALUE_TYPE_BOOLEAN => match value.as_str() {
            "true" | "false" => Ok(value),
            _ => Err(Error::Message(format!(
                "Invalid boolean value for {}.{}",
                definition.namespace, definition.key
            ))),
        },
        _ => Ok(value),
    }
}

async fn settings_response(ctx: &AppContext) -> Result<SettingsJson> {
    let persisted = admin_settings::Entity::find()
        .order_by_asc(admin_settings::Column::Namespace)
        .order_by_asc(admin_settings::Column::Key)
        .all(&ctx.db)
        .await?;

    let values: HashMap<(String, String), String> = persisted
        .into_iter()
        .map(|setting| {
            (
                (setting.namespace, setting.key),
                setting.value.unwrap_or_default(),
            )
        })
        .collect();

    let namespaces = ["general", "seo", "api", "notifications", "security"];
    let groups = namespaces
        .into_iter()
        .map(|namespace| {
            let settings = SETTINGS
                .iter()
                .filter(|setting| setting.namespace == namespace)
                .map(|setting| {
                    let value = values
                        .get(&(setting.namespace.to_string(), setting.key.to_string()))
                        .cloned()
                        .unwrap_or_else(|| setting.default_value.to_string());

                    SettingJson {
                        namespace: setting.namespace.to_string(),
                        key: setting.key.to_string(),
                        label: setting.label.to_string(),
                        value_type: setting.value_type,
                        value,
                    }
                })
                .collect();

            SettingsGroupJson {
                namespace: namespace.to_string(),
                label: group_label(namespace).to_string(),
                settings,
            }
        })
        .collect();

    Ok(SettingsJson { groups })
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(settings_response(&ctx).await?)
}

#[debug_handler]
pub async fn update(
    admin_session: Extension<AdminSession>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateSettingsJson>,
) -> Result<Response> {
    let settings_count = params.settings.len();
    let namespaces = params
        .settings
        .iter()
        .map(|setting| setting.namespace.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let now = chrono::Utc::now().into();
    for setting in params.settings {
        let Some(definition) = setting_definition(&setting.namespace, &setting.key) else {
            return Err(Error::Message(format!(
                "Unknown setting {}.{}",
                setting.namespace, setting.key
            )));
        };

        let value = normalize_value(definition, setting.value.clone())?;
        let value_for_fallback = value.clone();

        let active_model = admin_settings::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            namespace: Set(definition.namespace.to_string()),
            key: Set(definition.key.to_string()),
            value: Set(Some(value)),
            value_type: Set(definition.value_type),
            created_at: Set(now),
            updated_at: Set(now),
        };

        // Use UPSERT (PostgreSQL/SQLite) to avoid the find-then-insert-or-update roundtrip.
        let res = admin_settings::Entity::insert(active_model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::columns([
                    admin_settings::Column::Namespace,
                    admin_settings::Column::Key,
                ])
                .update_columns([
                    admin_settings::Column::Value,
                    admin_settings::Column::ValueType,
                    admin_settings::Column::UpdatedAt,
                ])
                .to_owned(),
            )
            .exec(&ctx.db)
            .await;

        if let Err(err) = res {
            // Some backends (e.g. older SQLite) may not support ON CONFLICT with
            // UPDATE columns in this form. Fall back to a single SELECT + branch.
            tracing::warn!(
                error = ?err,
                "upsert of admin_settings failed, falling back to find/update"
            );

            let existing = admin_settings::Entity::find()
                .filter(admin_settings::Column::Namespace.eq(definition.namespace))
                .filter(admin_settings::Column::Key.eq(definition.key))
                .one(&ctx.db)
                .await?;

            if let Some(existing) = existing {
                let mut active_model: admin_settings::ActiveModel = existing.into();
                active_model.value = Set(Some(value_for_fallback.clone()));
                active_model.value_type = Set(definition.value_type);
                active_model.updated_at = Set(chrono::Utc::now().into());
                active_model.update(&ctx.db).await?;
            } else {
                let now = chrono::Utc::now().into();
                admin_settings::ActiveModel {
                    namespace: Set(definition.namespace.to_string()),
                    key: Set(definition.key.to_string()),
                    value: Set(Some(value_for_fallback.clone())),
                    value_type: Set(definition.value_type),
                    created_at: Set(now),
                    updated_at: Set(now),
                    ..Default::default()
                }
                .insert(&ctx.db)
                .await?;
            }
        }
    }

    admin_audit_logs::record(
        &ctx.db,
        admin_session.current_user_id,
        "settings.update",
        "admin_settings",
        None,
        Some(namespaces.join(", ")),
        Some(format!("Updated {settings_count} settings")),
    )
    .await?;

    format::json(settings_response(&ctx).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/settings")
        .add("/", get(list))
        .add("/", put(update))
        .add("/", patch(update))
}

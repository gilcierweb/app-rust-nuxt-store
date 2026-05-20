//! `SeaORM` Entity for persisted admin settings.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "admin_settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub namespace: String,
    pub key: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub value: Option<String>,
    pub value_type: i16,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

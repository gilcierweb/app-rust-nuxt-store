//! `SeaORM` Entity, generated from the banner_positions scaffold.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "banner_positions")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    pub code: String,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::banners::Entity")]
    Banners,
}

impl Related<super::banners::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Banners.def()
    }
}

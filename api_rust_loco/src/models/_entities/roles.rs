//! `SeaORM` Entity, manually aligned with Rolify-style roles.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::users_roles::Entity")]
    UsersRoles,
}

impl Related<super::users_roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersRoles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

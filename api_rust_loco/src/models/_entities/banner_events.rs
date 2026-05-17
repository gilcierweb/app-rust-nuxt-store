//! `SeaORM` Entity, generated from the banner_events scaffold.

use ipnetwork::IpNetwork;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "banner_events")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    pub banner_id: i64,
    pub event_type: i16,
    pub user_id: Option<i64>,
    pub session_id: Option<String>,
    pub ip_address: Option<IpNetwork>,
    #[sea_orm(column_type = "Text", nullable)]
    pub user_agent: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub page_url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::banners::Entity",
        from = "Column::BannerId",
        to = "super::banners::Column::Id",
        on_delete = "Cascade"
    )]
    Banners,
}

impl Related<super::banners::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Banners.def()
    }
}

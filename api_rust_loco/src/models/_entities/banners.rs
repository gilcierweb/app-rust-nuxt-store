//! `SeaORM` Entity, generated from the banners scaffold.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "banners")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub image_desktop_url: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub image_mobile_url: Option<String>,
    pub alt_text: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub link_url: Option<String>,
    pub link_target: i16,
    pub position_id: i64,
    pub device: i16,
    pub start_at: DateTimeWithTimeZone,
    pub end_at: Option<DateTimeWithTimeZone>,
    pub priority: i32,
    pub status: i16,
    pub campaign_name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::banner_events::Entity")]
    BannerEvents,
    #[sea_orm(
        belongs_to = "super::banner_positions::Entity",
        from = "Column::PositionId",
        to = "super::banner_positions::Column::Id"
    )]
    BannerPositions,
}

impl Related<super::banner_events::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BannerEvents.def()
    }
}

impl Related<super::banner_positions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BannerPositions.def()
    }
}

pub use super::_entities::banner_events::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;

pub type BannerEvents = Entity;

pub mod banner_event_type {
    pub const IMPRESSION: i16 = 1;
    pub const CLICK: i16 = 2;

    pub fn is_valid(value: i16) -> bool {
        matches!(value, IMPRESSION | CLICK)
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

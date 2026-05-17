pub use super::_entities::banners::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;

pub type Banners = Entity;

pub mod link_target {
    pub const SAME_TAB: i16 = 1;
    pub const NEW_TAB: i16 = 2;

    pub fn is_valid(value: i16) -> bool {
        matches!(value, SAME_TAB | NEW_TAB)
    }
}

pub mod device {
    pub const ALL: i16 = 1;
    pub const DESKTOP: i16 = 2;
    pub const MOBILE: i16 = 3;

    pub fn is_valid(value: i16) -> bool {
        matches!(value, ALL | DESKTOP | MOBILE)
    }
}

pub mod banner_status {
    pub const DRAFT: i16 = 1;
    pub const ACTIVE: i16 = 2;
    pub const PAUSED: i16 = 3;
    pub const EXPIRED: i16 = 4;

    pub fn is_valid(value: i16) -> bool {
        matches!(value, DRAFT | ACTIVE | PAUSED | EXPIRED)
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

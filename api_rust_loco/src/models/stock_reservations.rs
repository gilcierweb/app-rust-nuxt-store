pub use super::_entities::stock_reservations::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type StockReservations = Entity;

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

impl Model {
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now().naive_utc() > expires_at.naive_utc()
        } else {
            false
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == "active" && !self.is_expired()
    }
}

impl ActiveModel {}

impl Entity {}

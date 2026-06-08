pub use super::_entities::product_variants::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type ProductVariants = Entity;

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
    pub fn available_quantity(&self) -> i32 {
        self.inventory_quantity - self.reserved_quantity
    }

    pub fn is_in_stock(&self) -> bool {
        if self.track_inventory {
            self.available_quantity() > 0 || self.allow_backorder
        } else {
            true
        }
    }

    pub fn is_low_stock(&self) -> bool {
        if self.track_inventory {
            self.available_quantity() > 0 && self.available_quantity() <= self.low_stock_threshold
        } else {
            false
        }
    }

    pub fn is_out_of_stock(&self) -> bool {
        if self.track_inventory {
            self.available_quantity() <= 0 && !self.allow_backorder
        } else {
            false
        }
    }

    pub fn can_reserve(&self, quantity: i32) -> bool {
        if !self.track_inventory {
            return true;
        }
        if self.allow_backorder {
            return true;
        }
        self.available_quantity() >= quantity
    }
}

impl ActiveModel {}

impl Entity {}

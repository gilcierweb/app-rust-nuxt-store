pub use super::_entities::stock_movements::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type StockMovements = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn is_increase(&self) -> bool {
        self.quantity > 0
    }

    pub fn is_decrease(&self) -> bool {
        self.quantity < 0
    }
}

impl ActiveModel {}

impl Entity {}

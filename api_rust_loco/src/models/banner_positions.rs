pub use super::_entities::banner_positions::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;

pub type BannerPositions = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

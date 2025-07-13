use sea_orm::entity::prelude::*;
pub use super::_entities::posts::{ActiveModel, Model, Entity};
pub type Posts = Entity;
use super::post_status::PostStatus;

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

// implement your read-oriented logic here
impl Model {
    pub fn status_enum(&self) -> Option<PostStatus> {
        self.status.and_then(|v| PostStatus::try_from(v).ok())
    }

    pub fn set_status_enum(&mut self, status: Option<PostStatus>) {
        self.status = status.map(|s| s.into());
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}

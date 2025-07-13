use loco_rs::{model::ModelError, Result};
use sea_orm::entity::prelude::*;
pub use super::_entities::categories::{ActiveModel, Model, Entity, Column };
pub type Categories = Entity;

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
    pub async fn find_with_relations(
           db: &DatabaseConnection,
           id: i32,
       ) -> Result<(Self, Option<Self>, Vec<Self>)> {
           let category = Entity::find_by_id(id)
               .one(db)
               .await?
               .ok_or_else(|| ModelError::EntityNotFound)?;
   
           let parent = if let Some(parent_id) = category.parent_id {
               Entity::find_by_id(parent_id).one(db).await?
           } else {
               None
           };
   
           let children = Entity::find()
               .filter(Column::ParentId.eq(id))
               .all(db)
               .await?;
   
           Ok((category, parent, children))
       }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use axum::debug_handler;

use crate::models::_entities::categories::{ ActiveModel, Entity, Model};
use serde_json::json;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub position: Option<i32>,
    pub parent_id: Option<i32>,
    }

impl Params {
    fn update(&self, item: &mut ActiveModel) {
      item.name = Set(self.name.clone());
      item.slug = Set(self.slug.clone());
      item.description = Set(self.description.clone());
      item.active = Set(self.active.clone());
      item.position = Set(self.position.clone());
      item.parent_id = Set(self.parent_id.clone());
      }
      
      pub fn validate(&self, current_id: Option<i32>) -> Result<()> {
              if let Some(parent_id) = self.parent_id {
                  // Não permite parent_id negativo
                  if parent_id < 0 {
                      return Err(Error::BadRequest("Invalid parent_id".into()));
                  }
                  
                  // Não permite uma categoria ser pai de si mesma
                  if let Some(id) = current_id {
                      if parent_id == id {
                          return Err(Error::BadRequest("Category cannot be its own parent".into()));
                      }
                  }
              }
              Ok(())
          }
}

#[derive(Serialize, Clone)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub position: Option<i32>,
    pub parent_id: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub children: Vec<CategoryResponse>,
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn list_with_relations(State(ctx): State<AppContext>) -> Result<Response> {
    let categories = Entity::find()
        .find_with_related(Entity)
        .all(&ctx.db)
        .await?;
  
    format::json(categories)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    params.validate(None)?;
    
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    params.validate(Some(id))?;
     
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let item = Entity::find_by_id(id)
            .one(&ctx.db)
            .await?
            .ok_or_else(|| Error::NotFound)?;
        
    format::json(item)
}

#[debug_handler]
pub async fn get_one_with_relations(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let (category, parent, children) = Model::find_with_relations(&ctx.db, id)
        .await
        .map_err(|_| Error::NotFound)?;
    
    let response = json!({
        "category": category,
        "parent": parent,
        "children": children
    });
    
    format::json(response)
}

fn sort_children_recursive(category: &mut CategoryResponse) {
    category.children.sort_by(|a, b| {
        match (a.position, b.position) {
            (Some(pa), Some(pb)) => pa.cmp(&pb),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });

    for child in &mut category.children {
        sort_children_recursive(child);
    }
}

fn build_hierarchy_from_flat(categories: Vec<Model>) -> Vec<CategoryResponse> {
    let mut items: HashMap<i32, CategoryResponse> = HashMap::new();
   
    for category in categories {
        let item = CategoryResponse {
            id: category.id,
            name: category.name.unwrap_or_default(),
            slug: category.slug.unwrap_or_default(),
            description: category.description,
            active: category.active,
            position: category.position,
            parent_id: category.parent_id,
            created_at: category.created_at,
            updated_at: category.updated_at,
            children: vec![],
        };

        items.insert(item.id, item);
    }

    for id in items.keys().cloned().collect::<Vec<_>>() {
        if let Some(child) = items.get(&id).cloned() {
            if let Some(parent_id) = child.parent_id {
                if let Some(parent) = items.get_mut(&parent_id) {
                    parent.children.push(child);
                }
            }
        }
    }

    let mut tree: Vec<CategoryResponse> = items.into_values()
        .filter(|item| item.parent_id.is_none())
        .collect();

    for node in &mut tree {
        sort_children_recursive(node);
    }

    tree
}

#[debug_handler]
pub async fn hierarchy(State(ctx): State<AppContext>) -> Result<Response> {
    let all_categories = Entity::find().all(&ctx.db).await?;
    let tree = build_hierarchy_from_flat(all_categories);
    format::json(tree)
}

// #[debug_handler]
// pub async fn hierarchy(State(ctx): State<AppContext>) -> Result<Response> {
//     let hierarchy = build_hierarchy(&ctx.db, None).await?;
//     format::json(hierarchy)
// }
// #[debug_handler]
// pub async fn hierarchy(State(ctx): State<AppContext>) -> Result<Response> {
//     let categories = Entity::find().all(&ctx.db).await?;        
      
//     format::json(categories)
// }

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/categories/")
        .add("/", get(list))
        .add("/relations", get(list_with_relations))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}/relations", get(get_one_with_relations))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
        .add("/hierarchy", get(hierarchy))
}

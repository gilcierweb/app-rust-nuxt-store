#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::CookieJWT;
use crate::models::_entities::orders;
use crate::models::_entities::shipments::{ActiveModel, Entity, Model};
use crate::utils::pagination::PaginationParams;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub order_id: i32,
    pub shipping_method_id: i32,
    pub tracking_number: Option<String>,
    pub carrier: Option<String>,
    pub status: Option<i32>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.order_id = Set(self.order_id);
        item.shipping_method_id = Set(self.shipping_method_id);
        item.tracking_number = Set(self.tracking_number.clone());
        item.carrier = Set(self.carrier.clone());
        item.status = Set(self.status);
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Response> {
    let items = Entity::find()
        .order_by_desc(crate::models::_entities::shipments::Column::CreatedAt)
        .paginate(&ctx.db, pagination.page_size())
        .fetch_page(pagination.page_index())
        .await?;

    format::json(items)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let now = chrono::Utc::now().into();
    item.created_at = Set(now);
    item.updated_at = Set(now);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
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
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    routes_with_prefix("api/shipments/")
}

#[debug_handler]
pub async fn bulk_export(
    State(ctx): State<AppContext>,
    Json(params): Json<crate::utils::bulk_export::BulkExportParams>,
) -> Result<Response> {
    if params.ids.is_empty() {
        return Err(Error::BadRequest("No IDs provided".into()));
    }

    let (zip_bytes, filename) =
        crate::services::bulk_pdf::build_shipments_zip(&ctx.db, &params.ids)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed to build shipments ZIP");
                loco_rs::Error::string(&format!("Bulk export error: {e}"))
            })?;

    Ok(axum::response::Response::builder()
        .header("content-type", "application/zip")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{filename}\""),
        )
        .body(axum::body::Body::from(zip_bytes))
        .unwrap())
}

pub fn admin_routes() -> Routes {
    routes_with_prefix("api/admin/shipments/")
        .add("bulk-export", post(bulk_export))
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new()
        .prefix(prefix)
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
        .add("{id}", put(update))
        .add("{id}", patch(update))
        .add("{id}/label", get(label))
}

#[debug_handler]
pub async fn label(Path(shipment_id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let data =
        crate::services::shipping_label::load_shipping_label_data(&ctx.db, shipment_id)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed to load shipping label data");
                e
            })?;

    let pdf_bytes = crate::services::shipping_label::generate_shipping_label_pdf(&data).map_err(
        |e| {
            tracing::error!(error = ?e, "failed to generate shipping label PDF");
            loco_rs::Error::string(&format!("PDF generation error: {e}"))
        },
    )?;

    let tracking = data
        .shipment
        .tracking_number
        .as_deref()
        .unwrap_or("label");
    let filename = format!("shipping-label-{tracking}.pdf");

    Ok(axum::response::Response::builder()
        .header("content-type", "application/pdf")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{filename}\""),
        )
        .body(axum::body::Body::from(pdf_bytes))
        .unwrap())
}

pub fn account_routes() -> Routes {
    Routes::new()
        .prefix("api/account/shipments/")
        .add("{id}/label", get(account_label))
}

#[debug_handler]
pub async fn account_label(
    auth: CookieJWT,
    Path(shipment_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_user_id = auth
        .claims
        .claims
        .get("user_id")
        .and_then(|v| v.as_i64())
        .and_then(|v| i32::try_from(v).ok())
        .ok_or_else(|| loco_rs::Error::string("unauthorized"))?;

    let shipment = Entity::find_by_id(shipment_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let order = orders::Entity::find_by_id(shipment.order_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if order.user_id != current_user_id {
        return Err(loco_rs::Error::string("unauthorized"));
    }

    let data =
        crate::services::shipping_label::load_shipping_label_data(&ctx.db, shipment_id)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "failed to load shipping label data");
                e
            })?;

    let pdf_bytes = crate::services::shipping_label::generate_shipping_label_pdf(&data).map_err(
        |e| {
            tracing::error!(error = ?e, "failed to generate shipping label PDF");
            loco_rs::Error::string(&format!("PDF generation error: {e}"))
        },
    )?;

    let tracking = data
        .shipment
        .tracking_number
        .as_deref()
        .unwrap_or("label");
    let filename = format!("shipping-label-{tracking}.pdf");

    Ok(axum::response::Response::builder()
        .header("content-type", "application/pdf")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{filename}\""),
        )
        .body(axum::body::Body::from(pdf_bytes))
        .unwrap())
}

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::{debug_handler, extract::Query};
use ipnetwork::IpNetwork;
use loco_rs::prelude::*;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, FromQueryResult, JoinType, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, Statement,
};
use serde::{Deserialize, Serialize};

use crate::models::_entities::banner_events::{
    ActiveModel as BannerEventActiveModel, Column as BannerEventColumn, Entity as BannerEventEntity,
};
use crate::models::_entities::banner_positions::{
    ActiveModel as BannerPositionActiveModel, Column as BannerPositionColumn,
    Entity as BannerPositionEntity, Model as BannerPositionModel,
};
use crate::models::_entities::banners::{
    ActiveModel as BannerActiveModel, Column as BannerColumn, Entity as BannerEntity,
    Model as BannerModel, Relation as BannerRelation,
};
use crate::models::{
    banner_events::banner_event_type,
    banners::{banner_status, device, link_target},
};
use crate::utils::pagination::PaginationParams;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BannerPositionParams {
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BannerParams {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_desktop_url: Option<String>,
    pub image_mobile_url: Option<String>,
    pub alt_text: Option<String>,
    pub link_url: Option<String>,
    pub link_target: Option<i16>,
    pub position_id: Option<i64>,
    pub device: Option<i16>,
    pub start_at: Option<DateTimeWithTimeZone>,
    pub end_at: Option<DateTimeWithTimeZone>,
    pub priority: Option<i32>,
    pub status: Option<i16>,
    pub campaign_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BannerEventParams {
    pub banner_id: i64,
    pub event_type: i16,
    pub user_id: Option<i64>,
    pub session_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub page_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActiveBannerQuery {
    pub position_code: String,
    pub device: Option<i16>,
    pub limit: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub from: DateTimeWithTimeZone,
    pub to: DateTimeWithTimeZone,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct BannerAnalytics {
    pub id: i64,
    pub title: String,
    pub impressions: i64,
    pub clicks: i64,
    pub ctr_percent: f64,
}

fn bad_request(message: &str) -> Error {
    Error::BadRequest(message.into())
}

fn validate_banner_codes(params: &BannerParams) -> Result<()> {
    if let Some(value) = params.link_target {
        if !link_target::is_valid(value) {
            return Err(bad_request(
                "link_target must be 1 (same_tab) or 2 (new_tab)",
            ));
        }
    }

    if let Some(value) = params.device {
        if !device::is_valid(value) {
            return Err(bad_request(
                "device must be 1 (all), 2 (desktop), or 3 (mobile)",
            ));
        }
    }

    if let Some(value) = params.status {
        if !banner_status::is_valid(value) {
            return Err(bad_request(
                "status must be 1 (draft), 2 (active), 3 (paused), or 4 (expired)",
            ));
        }
    }

    if let (Some(start_at), Some(end_at)) = (params.start_at, params.end_at) {
        if end_at < start_at {
            return Err(bad_request(
                "end_at must be greater than or equal to start_at",
            ));
        }
    }

    Ok(())
}

fn update_banner(item: &mut BannerActiveModel, params: &BannerParams) -> Result<()> {
    validate_banner_codes(params)?;

    if let Some(value) = &params.title {
        item.title = Set(value.clone());
    }
    if let Some(value) = &params.description {
        item.description = Set(Some(value.clone()));
    }
    if let Some(value) = &params.image_desktop_url {
        item.image_desktop_url = Set(value.clone());
    }
    if let Some(value) = &params.image_mobile_url {
        item.image_mobile_url = Set(Some(value.clone()));
    }
    if let Some(value) = &params.alt_text {
        item.alt_text = Set(Some(value.clone()));
    }
    if let Some(value) = &params.link_url {
        item.link_url = Set(Some(value.clone()));
    }
    if let Some(value) = params.link_target {
        item.link_target = Set(value);
    }
    if let Some(value) = params.position_id {
        item.position_id = Set(value);
    }
    if let Some(value) = params.device {
        item.device = Set(value);
    }
    if let Some(value) = params.start_at {
        item.start_at = Set(value);
    }
    if let Some(value) = params.end_at {
        item.end_at = Set(Some(value));
    }
    if let Some(value) = params.priority {
        item.priority = Set(value);
    }
    if let Some(value) = params.status {
        item.status = Set(value);
    }
    if let Some(value) = &params.campaign_name {
        item.campaign_name = Set(Some(value.clone()));
    }

    Ok(())
}

async fn load_banner(ctx: &AppContext, id: i64) -> Result<BannerModel> {
    BannerEntity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

async fn load_position(ctx: &AppContext, id: i64) -> Result<BannerPositionModel> {
    BannerPositionEntity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn active(
    State(ctx): State<AppContext>,
    Query(query): Query<ActiveBannerQuery>,
) -> Result<Response> {
    let requested_device = query.device.unwrap_or(device::ALL);
    if !device::is_valid(requested_device) {
        return Err(bad_request(
            "device must be 1 (all), 2 (desktop), or 3 (mobile)",
        ));
    }

    let now: DateTimeWithTimeZone = chrono::Utc::now().into();
    let limit = query.limit.unwrap_or(10).clamp(1, 100);

    let banners = BannerEntity::find()
        .join(JoinType::InnerJoin, BannerRelation::BannerPositions.def())
        .filter(BannerPositionColumn::Code.eq(query.position_code))
        .filter(BannerPositionColumn::IsActive.eq(true))
        .filter(BannerColumn::Status.eq(banner_status::ACTIVE))
        .filter(BannerColumn::Device.is_in([device::ALL, requested_device]))
        .filter(BannerColumn::StartAt.lte(now))
        .filter(
            Condition::any()
                .add(BannerColumn::EndAt.is_null())
                .add(BannerColumn::EndAt.gte(now)),
        )
        .order_by_desc(BannerColumn::Priority)
        .order_by_desc(BannerColumn::CreatedAt)
        .limit(limit)
        .all(&ctx.db)
        .await?;

    format::json(banners)
}

#[debug_handler]
pub async fn record_event(
    State(ctx): State<AppContext>,
    Json(params): Json<BannerEventParams>,
) -> Result<Response> {
    if !banner_event_type::is_valid(params.event_type) {
        return Err(bad_request(
            "event_type must be 1 (impression) or 2 (click)",
        ));
    }

    let ip_address = params
        .ip_address
        .as_deref()
        .map(str::parse::<IpNetwork>)
        .transpose()
        .map_err(|_| bad_request("ip_address must be a valid INET value"))?;

    let item = BannerEventActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        banner_id: Set(params.banner_id),
        event_type: Set(params.event_type),
        user_id: Set(params.user_id),
        session_id: Set(params.session_id),
        ip_address: Set(ip_address),
        user_agent: Set(params.user_agent),
        page_url: Set(params.page_url),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(&ctx.db)
    .await?;

    format::json(item)
}

#[debug_handler]
pub async fn list_positions(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(
        BannerPositionEntity::find()
            .order_by_asc(BannerPositionColumn::Code)
            .all(&ctx.db)
            .await?,
    )
}

#[debug_handler]
pub async fn add_position(
    State(ctx): State<AppContext>,
    Json(params): Json<BannerPositionParams>,
) -> Result<Response> {
    let item = BannerPositionActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        code: Set(params.code.ok_or_else(|| bad_request("code is required"))?),
        name: Set(params.name.ok_or_else(|| bad_request("name is required"))?),
        description: Set(params.description),
        is_active: Set(params.is_active.unwrap_or(true)),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(&ctx.db)
    .await?;

    format::json(item)
}

#[debug_handler]
pub async fn get_position(Path(id): Path<i64>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_position(&ctx, id).await?)
}

#[debug_handler]
pub async fn update_position(
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
    Json(params): Json<BannerPositionParams>,
) -> Result<Response> {
    let mut item = load_position(&ctx, id).await?.into_active_model();

    if let Some(value) = params.code {
        item.code = Set(value);
    }
    if let Some(value) = params.name {
        item.name = Set(value);
    }
    if let Some(value) = params.description {
        item.description = Set(Some(value));
    }
    if let Some(value) = params.is_active {
        item.is_active = Set(value);
    }

    format::json(item.update(&ctx.db).await?)
}

#[debug_handler]
pub async fn remove_position(
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    load_position(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn list_banners(
    State(ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Response> {
    format::json(
        BannerEntity::find()
            .order_by_desc(BannerColumn::CreatedAt)
            .paginate(&ctx.db, pagination.page_size())
            .fetch_page(pagination.page_index())
            .await?,
    )
}

#[debug_handler]
pub async fn add_banner(
    State(ctx): State<AppContext>,
    Json(params): Json<BannerParams>,
) -> Result<Response> {
    validate_banner_codes(&params)?;

    let start_at = params
        .start_at
        .ok_or_else(|| bad_request("start_at is required"))?;
    if let Some(end_at) = params.end_at {
        if end_at < start_at {
            return Err(bad_request(
                "end_at must be greater than or equal to start_at",
            ));
        }
    }

    let item = BannerActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        title: Set(params
            .title
            .ok_or_else(|| bad_request("title is required"))?),
        description: Set(params.description),
        image_desktop_url: Set(params
            .image_desktop_url
            .ok_or_else(|| bad_request("image_desktop_url is required"))?),
        image_mobile_url: Set(params.image_mobile_url),
        alt_text: Set(params.alt_text),
        link_url: Set(params.link_url),
        link_target: Set(params.link_target.unwrap_or(link_target::SAME_TAB)),
        position_id: Set(params
            .position_id
            .ok_or_else(|| bad_request("position_id is required"))?),
        device: Set(params.device.unwrap_or(device::ALL)),
        start_at: Set(start_at),
        end_at: Set(params.end_at),
        priority: Set(params.priority.unwrap_or(0)),
        status: Set(params.status.unwrap_or(banner_status::DRAFT)),
        campaign_name: Set(params.campaign_name),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    }
    .insert(&ctx.db)
    .await?;

    format::json(item)
}

#[debug_handler]
pub async fn get_banner(Path(id): Path<i64>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_banner(&ctx, id).await?)
}

#[debug_handler]
pub async fn update_banner_handler(
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
    Json(params): Json<BannerParams>,
) -> Result<Response> {
    let mut item = load_banner(&ctx, id).await?.into_active_model();
    update_banner(&mut item, &params)?;
    format::json(item.update(&ctx.db).await?)
}

#[debug_handler]
pub async fn remove_banner(Path(id): Path<i64>, State(ctx): State<AppContext>) -> Result<Response> {
    load_banner(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn list_events(
    State(ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Response> {
    format::json(
        BannerEventEntity::find()
            .order_by_desc(BannerEventColumn::CreatedAt)
            .paginate(&ctx.db, pagination.page_size())
            .fetch_page(pagination.page_index())
            .await?,
    )
}

#[debug_handler]
pub async fn get_event(Path(id): Path<i64>, State(ctx): State<AppContext>) -> Result<Response> {
    let item = BannerEventEntity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    format::json(item)
}

#[debug_handler]
pub async fn analytics(
    State(ctx): State<AppContext>,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Response> {
    if query.to <= query.from {
        return Err(bad_request("to must be greater than from"));
    }

    let rows = BannerAnalytics::find_by_statement(Statement::from_sql_and_values(
        ctx.db.get_database_backend(),
        r#"
        SELECT
            b.id,
            b.title,
            COUNT(e.id) FILTER (WHERE e.event_type = 1) AS impressions,
            COUNT(e.id) FILTER (WHERE e.event_type = 2) AS clicks,
            CASE
                WHEN COUNT(e.id) FILTER (WHERE e.event_type = 1) = 0 THEN 0.0
                ELSE
                    COUNT(e.id) FILTER (WHERE e.event_type = 2)::FLOAT8
                    * 100.0
                    / COUNT(e.id) FILTER (WHERE e.event_type = 1)::FLOAT8
            END AS ctr_percent
        FROM banners b
        LEFT JOIN banner_events e
            ON e.banner_id = b.id
           AND e.created_at >= $1
           AND e.created_at < $2
        GROUP BY b.id, b.title
        ORDER BY ctr_percent DESC
        "#,
        [query.from.into(), query.to.into()],
    ))
    .all(&ctx.db)
    .await?;

    format::json(rows)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/banners/")
        .add("active", get(active))
        .add("events", post(record_event))
}

pub fn admin_routes() -> Routes {
    Routes::new()
        .prefix("api/admin/banners/")
        .add("positions", get(list_positions))
        .add("positions", post(add_position))
        .add("positions/{id}", get(get_position))
        .add("positions/{id}", put(update_position))
        .add("positions/{id}", patch(update_position))
        .add("positions/{id}", delete(remove_position))
        .add("events", get(list_events))
        .add("events/{id}", get(get_event))
        .add("analytics", get(analytics))
        .add("/", get(list_banners))
        .add("/", post(add_banner))
        .add("{id}", get(get_banner))
        .add("{id}", put(update_banner_handler))
        .add("{id}", patch(update_banner_handler))
        .add("{id}", delete(remove_banner))
}

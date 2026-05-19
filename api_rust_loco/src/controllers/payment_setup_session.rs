#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::auth::CookieJWT;
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::_entities::payment_methods;
use crate::models::_entities::payment_setup_sessions;
use crate::models::users;
use crate::payment_gateways::{create_payment_setup_session, CreatePaymentSetupSessionInput};

#[derive(Debug, Deserialize)]
pub struct CreatePaymentSetupSessionParams {
    pub payment_method_id: i32,
}

#[derive(Debug, Serialize)]
pub struct PaymentSetupSessionJson {
    pub id: i32,
    pub user_id: i32,
    pub payment_method_id: i32,
    pub status: i16,
    pub external_setup_id: Option<String>,
    pub external_client_secret: Option<String>,
    pub action_url: Option<String>,
    pub requires_action: bool,
}

async fn current_user_id(ctx: &AppContext, auth: &CookieJWT) -> Result<i32> {
    if let Some(user_id) = auth
        .claims
        .claims
        .get("user_id")
        .and_then(|value| value.as_i64())
        .and_then(|value| i32::try_from(value).ok())
    {
        return Ok(user_id);
    }

    Ok(users::Model::find_by_pid(&ctx.db, &auth.claims.pid)
        .await?
        .id)
}

#[debug_handler]
pub async fn index(auth: CookieJWT, State(ctx): State<AppContext>) -> Result<Response> {
    let current_user_id = current_user_id(&ctx, &auth).await?;
    let sessions = payment_setup_sessions::Entity::find()
        .filter(payment_setup_sessions::Column::UserId.eq(current_user_id))
        .order_by_desc(payment_setup_sessions::Column::CreatedAt)
        .all(&ctx.db)
        .await?
        .into_iter()
        .map(setup_session_json)
        .collect::<Vec<_>>();

    format::json(sessions)
}

#[debug_handler]
pub async fn create(
    auth: CookieJWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreatePaymentSetupSessionParams>,
) -> Result<Response> {
    let current_user_id = current_user_id(&ctx, &auth).await?;
    let payment_method = payment_methods::Entity::find_by_id(params.payment_method_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if !payment_method.active.unwrap_or(false) {
        return Err(Error::BadRequest("payment method is inactive".into()));
    }

    let session = create_payment_setup_session(
        &ctx.db,
        CreatePaymentSetupSessionInput {
            user_id: current_user_id,
            payment_method,
        },
    )
    .await?;

    format::json(setup_session_json(session))
}

#[debug_handler]
pub async fn get_one(
    auth: CookieJWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let current_user_id = current_user_id(&ctx, &auth).await?;
    let session = payment_setup_sessions::Entity::find_by_id(id)
        .filter(payment_setup_sessions::Column::UserId.eq(current_user_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    format::json(setup_session_json(session))
}

#[debug_handler]
pub async fn complete(
    _auth: CookieJWT,
    Path(_id): Path<i32>,
    State(_ctx): State<AppContext>,
    Json(_params): Json<Value>,
) -> Result<Response> {
    Err(Error::BadRequest(
        "payment setup session completion is not implemented yet".into(),
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/account/payment-setup-sessions")
        .add("/", get(index))
        .add("/", post(create))
        .add("/{id}", get(get_one))
        .add("/{id}/complete", post(complete))
}

fn setup_session_json(session: payment_setup_sessions::Model) -> PaymentSetupSessionJson {
    let action_url = session
        .external_client_secret
        .as_deref()
        .filter(|value| value.starts_with("http://") || value.starts_with("https://"))
        .map(ToString::to_string);
    let requires_action = action_url.is_some()
        || session.status
            == crate::models::payment_gateway_status::PaymentSessionStatus::RequiresAction.to_i16();

    PaymentSetupSessionJson {
        id: session.id,
        user_id: session.user_id,
        payment_method_id: session.payment_method_id,
        status: session.status,
        external_setup_id: session.external_setup_id,
        external_client_secret: session.external_client_secret,
        action_url,
        requires_action,
    }
}

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unused_async)]

use std::collections::{HashMap, HashSet};

use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::Serialize;

use crate::models::_entities::{payment_gateways, payment_methods, payment_refunds, payments};

#[derive(Serialize)]
pub struct AdminPaymentRefundListItem {
    pub refund: payment_refunds::Model,
    pub order_id: i32,
    pub payment_number: Option<String>,
    pub payment_status: Option<i32>,
    pub gateway_name: Option<String>,
}

#[derive(Serialize)]
pub struct AdminPaymentRefundDetail {
    pub refund: payment_refunds::Model,
    pub payment: payments::Model,
    pub gateway_name: Option<String>,
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let refunds = payment_refunds::Entity::find()
        .order_by_desc(payment_refunds::Column::CreatedAt)
        .limit(200)
        .all(&ctx.db)
        .await?;

    let payment_ids: Vec<i32> = refunds.iter().map(|refund| refund.payment_id).collect();
    let payments = payments::Entity::find()
        .filter(payments::Column::Id.is_in(payment_ids.clone()))
        .all(&ctx.db)
        .await?;

    let method_ids: HashSet<i32> = payments
        .iter()
        .map(|payment| payment.payment_method_id)
        .collect();
    let methods = payment_methods::Entity::find()
        .filter(payment_methods::Column::Id.is_in(method_ids.iter().copied()))
        .all(&ctx.db)
        .await?;

    let gateway_ids: HashSet<i32> = methods
        .iter()
        .filter_map(|method| method.payment_gateway_id)
        .collect();
    let gateways = payment_gateways::Entity::find()
        .filter(payment_gateways::Column::Id.is_in(gateway_ids.iter().copied()))
        .all(&ctx.db)
        .await?;

    let payment_by_id: HashMap<i32, payments::Model> = payments
        .into_iter()
        .map(|payment| (payment.id, payment))
        .collect();
    let method_by_id: HashMap<i32, payment_methods::Model> = methods
        .into_iter()
        .map(|method| (method.id, method))
        .collect();
    let gateway_by_id: HashMap<i32, payment_gateways::Model> = gateways
        .into_iter()
        .map(|gateway| (gateway.id, gateway))
        .collect();

    let items = refunds
        .into_iter()
        .filter_map(|refund| {
            let payment = payment_by_id.get(&refund.payment_id)?;
            let gateway_name = method_by_id
                .get(&payment.payment_method_id)
                .and_then(|method| method.payment_gateway_id)
                .and_then(|gateway_id| gateway_by_id.get(&gateway_id))
                .map(|gateway| gateway.name.clone());

            Some(AdminPaymentRefundListItem {
                refund,
                order_id: payment.order_id,
                payment_number: payment.number.clone(),
                payment_status: payment.status,
                gateway_name,
            })
        })
        .collect::<Vec<_>>();

    format::json(items)
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let refund = payment_refunds::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let payment = payments::Entity::find_by_id(refund.payment_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let gateway_name = if let Some(method) =
        payment_methods::Entity::find_by_id(payment.payment_method_id)
            .one(&ctx.db)
            .await?
    {
        if let Some(gateway_id) = method.payment_gateway_id {
            payment_gateways::Entity::find_by_id(gateway_id)
                .one(&ctx.db)
                .await?
                .map(|gateway| gateway.name)
        } else {
            None
        }
    } else {
        None
    };

    format::json(AdminPaymentRefundDetail {
        refund,
        payment,
        gateway_name,
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/payment-refunds/")
        .add("/", get(list))
        .add("{id}", get(get_one))
}

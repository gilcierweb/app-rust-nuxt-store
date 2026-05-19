#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::body::Bytes;
use axum::debug_handler;
use axum::http::HeaderMap;
use loco_rs::prelude::*;

use crate::payment_gateways::{receive_webhook, ReceiveWebhookInput};

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[debug_handler]
pub async fn receive(
    Path(gateway_code): Path<String>,
    State(ctx): State<AppContext>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response> {
    let headers = headers
        .iter()
        .filter_map(|(key, value)| {
            value
                .to_str()
                .ok()
                .map(|value| (key.as_str().to_string(), value.to_string()))
        })
        .collect();
    let payload = String::from_utf8(body.to_vec())
        .map_err(|_| Error::BadRequest("webhook payload must be valid utf-8".into()))?;

    let output = receive_webhook(
        &ctx.db,
        ReceiveWebhookInput {
            gateway_code,
            headers,
            payload,
        },
    )
    .await?;

    format::json(output)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/webhooks/payments/")
        .add("/", get(index))
        .add("{gateway_code}", post(receive))
}

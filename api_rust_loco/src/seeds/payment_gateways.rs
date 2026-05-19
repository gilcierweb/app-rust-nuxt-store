use chrono::Utc;
use loco_rs::Result;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::models::_entities::payment_gateways::{ActiveModel, Entity};
use crate::models::payment_gateway_status::{PaymentGatewayEnvironment, PaymentGatewayStatus};

struct GatewaySeed {
    code: &'static str,
    name: &'static str,
    driver: &'static str,
    supports_authorize: bool,
    supports_capture: bool,
    supports_refund: bool,
    supports_void: bool,
    supports_saved_sources: bool,
}

pub async fn seed(db: &sea_orm::DatabaseConnection) -> Result<()> {
    let count = Entity::find().count(db).await?;
    if count > 0 {
        tracing::info!("Payment gateways already exist, skipping.");
        return Ok(());
    }

    let now = Utc::now();
    let gateways = vec![
        GatewaySeed {
            code: "manual",
            name: "Manual",
            driver: "manual",
            supports_authorize: true,
            supports_capture: true,
            supports_refund: true,
            supports_void: true,
            supports_saved_sources: false,
        },
        GatewaySeed {
            code: "stripe",
            name: "Stripe",
            driver: "stripe",
            supports_authorize: true,
            supports_capture: true,
            supports_refund: true,
            supports_void: true,
            supports_saved_sources: true,
        },
        GatewaySeed {
            code: "paypal",
            name: "PayPal",
            driver: "paypal",
            supports_authorize: true,
            supports_capture: true,
            supports_refund: true,
            supports_void: true,
            supports_saved_sources: true,
        },
        GatewaySeed {
            code: "braintree",
            name: "Braintree",
            driver: "braintree",
            supports_authorize: true,
            supports_capture: true,
            supports_refund: true,
            supports_void: true,
            supports_saved_sources: true,
        },
        GatewaySeed {
            code: "mercado_pago",
            name: "Mercado Pago",
            driver: "mercado_pago",
            supports_authorize: false,
            supports_capture: true,
            supports_refund: true,
            supports_void: false,
            supports_saved_sources: true,
        },
        GatewaySeed {
            code: "pagarme",
            name: "Pagar.me",
            driver: "pagarme",
            supports_authorize: true,
            supports_capture: true,
            supports_refund: true,
            supports_void: true,
            supports_saved_sources: true,
        },
        GatewaySeed {
            code: "iugu",
            name: "Iugu",
            driver: "iugu",
            supports_authorize: false,
            supports_capture: true,
            supports_refund: true,
            supports_void: false,
            supports_saved_sources: true,
        },
        GatewaySeed {
            code: "abacatepay",
            name: "AbacatePay",
            driver: "abacatepay",
            supports_authorize: false,
            supports_capture: true,
            supports_refund: true,
            supports_void: false,
            supports_saved_sources: false,
        },
        GatewaySeed {
            code: "cielo",
            name: "Cielo",
            driver: "cielo",
            supports_authorize: true,
            supports_capture: true,
            supports_refund: true,
            supports_void: true,
            supports_saved_sources: true,
        },
        GatewaySeed {
            code: "getnet",
            name: "Getnet",
            driver: "getnet",
            supports_authorize: true,
            supports_capture: true,
            supports_refund: true,
            supports_void: true,
            supports_saved_sources: true,
        },
    ];

    for gateway in gateways {
        ActiveModel {
            code: Set(gateway.code.to_string()),
            name: Set(gateway.name.to_string()),
            driver: Set(gateway.driver.to_string()),
            environment: Set(PaymentGatewayEnvironment::Test.to_i16()),
            status: Set(if gateway.code == "manual" {
                PaymentGatewayStatus::Active.to_i16()
            } else {
                PaymentGatewayStatus::SandboxOnly.to_i16()
            }),
            supports_authorize: Set(gateway.supports_authorize),
            supports_capture: Set(gateway.supports_capture),
            supports_refund: Set(gateway.supports_refund),
            supports_void: Set(gateway.supports_void),
            supports_saved_sources: Set(gateway.supports_saved_sources),
            public_key_env: Set(Some(format!(
                "{}_PUBLIC_KEY",
                gateway.code.to_ascii_uppercase()
            ))),
            secret_key_env: Set(Some(format!(
                "{}_SECRET_KEY",
                gateway.code.to_ascii_uppercase()
            ))),
            webhook_secret_env: Set(Some(format!(
                "{}_WEBHOOK_SECRET",
                gateway.code.to_ascii_uppercase()
            ))),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }

    tracing::info!("10 payment gateways generated");
    Ok(())
}

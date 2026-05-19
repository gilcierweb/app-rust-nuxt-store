use loco_rs::prelude::*;

use crate::payment_gateways::abacatepay::AbacatePayGateway;
use crate::payment_gateways::braintree::BraintreeGateway;
use crate::payment_gateways::cielo::CieloGateway;
use crate::payment_gateways::drivers::{
    ABACATEPAY_DRIVER, BRAINTREE_DRIVER, CIELO_DRIVER, GETNET_DRIVER, IUGU_DRIVER, MANUAL_DRIVER,
    MERCADO_PAGO_DRIVER, PAGARME_DRIVER, PAYPAL_DRIVER, STRIPE_DRIVER,
};
use crate::payment_gateways::getnet::GetnetGateway;
use crate::payment_gateways::iugu::IuguGateway;
use crate::payment_gateways::manual::ManualGateway;
use crate::payment_gateways::mercado_pago::MercadoPagoGateway;
use crate::payment_gateways::pagarme::PagarmeGateway;
use crate::payment_gateways::paypal::PayPalGateway;
use crate::payment_gateways::stripe::StripeGateway;
use crate::payment_gateways::types::PaymentGateway;

pub fn gateway_for_driver(driver: &str) -> Result<Box<dyn PaymentGateway>> {
    match driver {
        MANUAL_DRIVER => Ok(Box::new(ManualGateway)),
        STRIPE_DRIVER => Ok(Box::new(StripeGateway)),
        PAYPAL_DRIVER => Ok(Box::new(PayPalGateway)),
        MERCADO_PAGO_DRIVER => Ok(Box::new(MercadoPagoGateway)),
        ABACATEPAY_DRIVER => Ok(Box::new(AbacatePayGateway)),
        PAGARME_DRIVER => Ok(Box::new(PagarmeGateway)),
        IUGU_DRIVER => Ok(Box::new(IuguGateway)),
        CIELO_DRIVER => Ok(Box::new(CieloGateway)),
        BRAINTREE_DRIVER => Ok(Box::new(BraintreeGateway)),
        GETNET_DRIVER => Ok(Box::new(GetnetGateway)),
        _ => Err(Error::BadRequest(
            format!("unsupported payment gateway driver: {driver}").into(),
        )),
    }
}

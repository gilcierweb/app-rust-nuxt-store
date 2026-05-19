use async_trait::async_trait;
use loco_rs::prelude::*;
use uuid::Uuid;

use crate::models::payment_gateway_status::{
    PaymentAttemptStatus, PaymentRefundStatus, PaymentSessionStatus,
};
use crate::payment_gateways::{
    CapturePaymentInput, CreatePaymentSessionInput, CreateSetupSessionInput, PaymentGateway,
    PaymentOperationOutput, PaymentSessionOutput, PaymentSetupSessionOutput, RefundOutput,
    RefundPaymentInput, VoidPaymentInput, WebhookDecision, WebhookInput, MANUAL_DRIVER,
};

pub struct ManualGateway;

#[async_trait]
impl PaymentGateway for ManualGateway {
    fn driver(&self) -> &'static str {
        MANUAL_DRIVER
    }

    async fn create_payment_session(
        &self,
        input: CreatePaymentSessionInput,
    ) -> Result<PaymentSessionOutput> {
        let external_id = format!("manual-session-{}", Uuid::new_v4());
        let payment_status = if input.auto_capture {
            PaymentAttemptStatus::Captured
        } else {
            PaymentAttemptStatus::Authorized
        };

        Ok(PaymentSessionOutput {
            status: PaymentSessionStatus::Completed,
            payment_status,
            external_session_id: Some(external_id.clone()),
            external_payment_id: Some(external_id),
            external_status: Some(payment_status.name().to_string()),
            external_client_secret: None,
        })
    }

    async fn capture_payment(&self, input: CapturePaymentInput) -> Result<PaymentOperationOutput> {
        Ok(PaymentOperationOutput {
            status: PaymentAttemptStatus::Captured,
            external_payment_id: Some(input.external_payment_id),
            external_status: Some(PaymentAttemptStatus::Captured.name().to_string()),
            failure_code: None,
            failure_message: None,
        })
    }

    async fn void_payment(&self, input: VoidPaymentInput) -> Result<PaymentOperationOutput> {
        Ok(PaymentOperationOutput {
            status: PaymentAttemptStatus::Voided,
            external_payment_id: Some(input.external_payment_id),
            external_status: Some(PaymentAttemptStatus::Voided.name().to_string()),
            failure_code: None,
            failure_message: None,
        })
    }

    async fn refund_payment(&self, _input: RefundPaymentInput) -> Result<RefundOutput> {
        Ok(RefundOutput {
            status: PaymentRefundStatus::Succeeded,
            external_refund_id: Some(format!("manual-refund-{}", Uuid::new_v4())),
            external_status: Some(PaymentRefundStatus::Succeeded.name().to_string()),
            failure_code: None,
            failure_message: None,
        })
    }

    async fn create_setup_session(
        &self,
        _input: CreateSetupSessionInput,
    ) -> Result<PaymentSetupSessionOutput> {
        Ok(PaymentSetupSessionOutput {
            status: PaymentSessionStatus::Completed,
            external_setup_id: Some(format!("manual-setup-{}", Uuid::new_v4())),
            external_client_secret: None,
        })
    }

    async fn handle_webhook(&self, _input: WebhookInput) -> Result<WebhookDecision> {
        Ok(WebhookDecision {
            event_type: None,
            external_event_id: None,
            signature_valid: true,
            processed: false,
            ignored: true,
            failure_message: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[tokio::test]
    async fn creates_captured_payment_session_when_auto_capture_is_enabled() {
        let output = ManualGateway
            .create_payment_session(CreatePaymentSessionInput {
                payment_id: 1,
                order_id: 1,
                payment_method_id: 1,
                amount: Decimal::new(1000, 2),
                currency: "BRL".to_string(),
                idempotency_key: "manual-test-key".to_string(),
                auto_capture: true,
                gateway_payload: None,
            })
            .await
            .unwrap();

        assert_eq!(output.status, PaymentSessionStatus::Completed);
        assert_eq!(output.payment_status, PaymentAttemptStatus::Captured);
        assert!(output.external_payment_id.is_some());
        assert!(output.external_client_secret.is_none());
    }

    #[tokio::test]
    async fn creates_authorized_payment_session_when_auto_capture_is_disabled() {
        let output = ManualGateway
            .create_payment_session(CreatePaymentSessionInput {
                payment_id: 1,
                order_id: 1,
                payment_method_id: 1,
                amount: Decimal::new(1000, 2),
                currency: "BRL".to_string(),
                idempotency_key: "manual-test-key".to_string(),
                auto_capture: false,
                gateway_payload: None,
            })
            .await
            .unwrap();

        assert_eq!(output.status, PaymentSessionStatus::Completed);
        assert_eq!(output.payment_status, PaymentAttemptStatus::Authorized);
    }

    #[tokio::test]
    async fn captures_voids_refunds_and_ignores_webhooks() {
        let capture = ManualGateway
            .capture_payment(CapturePaymentInput {
                payment_id: 1,
                amount: Decimal::new(1000, 2),
                currency: "BRL".to_string(),
                external_payment_id: "manual-payment".to_string(),
                idempotency_key: "manual-capture-key".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(capture.status, PaymentAttemptStatus::Captured);

        let void = ManualGateway
            .void_payment(VoidPaymentInput {
                payment_id: 1,
                external_payment_id: "manual-payment".to_string(),
                idempotency_key: "manual-void-key".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(void.status, PaymentAttemptStatus::Voided);

        let refund = ManualGateway
            .refund_payment(RefundPaymentInput {
                payment_id: 1,
                amount: Decimal::new(1000, 2),
                currency: "BRL".to_string(),
                external_payment_id: "manual-payment".to_string(),
                idempotency_key: "manual-refund-key".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(refund.status, PaymentRefundStatus::Succeeded);

        let webhook = ManualGateway
            .handle_webhook(WebhookInput {
                gateway_code: "manual".to_string(),
                headers: vec![],
                payload: "{}".to_string(),
            })
            .await
            .unwrap();
        assert!(webhook.signature_valid);
        assert!(webhook.ignored);
        assert!(!webhook.processed);
    }
}

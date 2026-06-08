use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

use crate::mailers::auth::AuthMailer;
use crate::models::email_logs;

#[derive(Debug, serde::Serialize)]
pub struct CartItemInfo {
    pub name: String,
    pub quantity: i32,
    pub price: String,
}

pub struct EmailService;

impl EmailService {
    /// Generic helper to send an email and log its details in the database
    pub async fn send_logged<F, Fut>(
        ctx: &AppContext,
        recipient: String,
        template_name: String,
        subject: String,
        locals: serde_json::Value,
        send_fn: F,
    ) -> Result<()>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let now = chrono::Utc::now().into();
        let active_model = email_logs::ActiveModel {
            recipient: Set(recipient.clone()),
            template_name: Set(template_name.clone()),
            subject: Set(subject.clone()),
            locals_json: Set(locals.to_string()),
            status: Set(0), // 0 = Pending
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let logged = active_model.insert(&ctx.db).await?;

        let res = send_fn().await;

        let mut active = logged.into_active_model();
        match res {
            Ok(_) => {
                active.status = Set(1); // 1 = Sent
                active.sent_at = Set(Some(chrono::Utc::now().naive_utc()));
                active.update(&ctx.db).await?;
                Ok(())
            }
            Err(err) => {
                active.status = Set(2); // 2 = Failed
                active.error_message = Set(Some(err.to_string()));
                active.update(&ctx.db).await?;
                Err(err)
            }
        }
    }

    /// Resend an email log by loading it, re-rendering with its parameters, and logging the attempt
    pub async fn resend(ctx: &AppContext, log_id: i32) -> Result<()> {
        let log = email_logs::Entity::find_by_id(log_id)
            .one(&ctx.db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        let locals: serde_json::Value = serde_json::from_str(&log.locals_json)
            .map_err(|e| Error::string(&format!("Failed to parse stored locals: {e}")))?;

        // Update existing log to pending/sending status
        let mut active = log.clone().into_active_model();
        active.status = Set(0); // Pending
        active.error_message = Set(None);
        let active = active.update(&ctx.db).await?;

        // Re-dispatch using the appropriate mailer template dispatcher
        let is_order_template = matches!(
            log.template_name.as_str(),
            "order_confirmation"
                | "shipping_update"
                | "delivery_confirmation"
                | "abandoned_cart"
                | "back_in_stock"
        );

        let res = if is_order_template {
            let dir = crate::mailers::order::OrderMailer::mail_template_public(&log.template_name)?;
            crate::mailers::order::OrderMailer::mail_template(
                ctx,
                dir,
                loco_rs::mailer::Args {
                    to: log.recipient.clone(),
                    locals,
                    ..Default::default()
                },
            )
            .await
        } else {
            AuthMailer::mail_template_public(
                ctx,
                &log.template_name,
                log.recipient.clone(),
                locals,
            )
            .await
        };

        let mut active = active.into_active_model();
        match res {
            Ok(_) => {
                active.status = Set(1); // Sent
                active.sent_at = Set(Some(chrono::Utc::now().naive_utc()));
                active.update(&ctx.db).await?;
                Ok(())
            }
            Err(err) => {
                active.status = Set(2); // Failed
                active.error_message = Set(Some(err.to_string()));
                active.update(&ctx.db).await?;
                Err(err)
            }
        }
    }
}

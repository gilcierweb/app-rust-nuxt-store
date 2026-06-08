#![allow(non_upper_case_globals)]

use loco_rs::prelude::*;
use serde_json::json;

use crate::models::_entities::orders;
use crate::models::_entities::order_items;
use crate::models::_entities::products;
use crate::models::_entities::users;

static order_confirmation: Dir<'_> = include_dir!("src/mailers/order/order_confirmation");
static shipping_update: Dir<'_> = include_dir!("src/mailers/order/shipping_update");
static delivery_confirmation: Dir<'_> = include_dir!("src/mailers/order/delivery_confirmation");
static abandoned_cart: Dir<'_> = include_dir!("src/mailers/order/abandoned_cart");
static back_in_stock: Dir<'_> = include_dir!("src/mailers/order/back_in_stock");

#[allow(clippy::module_name_repetitions)]
pub struct OrderMailer {}
impl Mailer for OrderMailer {}

#[derive(Debug, serde::Serialize)]
pub struct OrderItemInfo {
    pub name: String,
    pub quantity: i32,
    pub total: String,
    pub price: String,
}

impl OrderMailer {
    pub async fn send_order_confirmation(
        ctx: &AppContext,
        user: &users::Model,
        order: &orders::Model,
    ) -> Result<()> {
        let items = Self::load_order_items(ctx, order.id).await?;
        let domain = ctx.config.server.full_url();

        Self::mail_template(
            ctx,
            &order_confirmation,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                    "name": user.name,
                    "order_number": order.order_number,
                    "order_date": order.created_at.format("%B %d, %Y").to_string(),
                    "subtotal": order.subtotal.unwrap_or(rust_decimal::Decimal::ZERO),
                    "shipping_amount": order.shipping_amount.unwrap_or(rust_decimal::Decimal::ZERO),
                    "discount_amount": order.discount_amount.unwrap_or(rust_decimal::Decimal::ZERO),
                    "total_amount": order.total_amount.unwrap_or(rust_decimal::Decimal::ZERO),
                    "currency": order.currency.as_deref().unwrap_or("BRL"),
                    "items": items,
                    "domain": domain,
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    pub async fn send_shipping_update(
        ctx: &AppContext,
        user: &users::Model,
        order: &orders::Model,
        tracking_number: Option<&str>,
        carrier: Option<&str>,
        shipped_at: Option<String>,
    ) -> Result<()> {
        let domain = ctx.config.server.full_url();

        Self::mail_template(
            ctx,
            &shipping_update,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                    "name": user.name,
                    "order_number": order.order_number,
                    "tracking_number": tracking_number,
                    "carrier": carrier,
                    "shipped_at": shipped_at,
                    "tracking_url": tracking_number.map(|_| format!("{}/account/orders/{}", domain, order.id)),
                    "domain": domain,
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    pub async fn send_delivery_confirmation(
        ctx: &AppContext,
        user: &users::Model,
        order: &orders::Model,
        delivered_at: Option<String>,
        carrier: Option<&str>,
        tracking_number: Option<&str>,
    ) -> Result<()> {
        Self::mail_template(
            ctx,
            &delivery_confirmation,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                    "name": user.name,
                    "order_number": order.order_number,
                    "delivered_at": delivered_at,
                    "carrier": carrier,
                    "tracking_number": tracking_number,
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    pub async fn send_abandoned_cart(
        ctx: &AppContext,
        user: &users::Model,
        cart_items: Vec<super::email_service::CartItemInfo>,
        total: &str,
    ) -> Result<()> {
        let domain = ctx.config.server.full_url();

        Self::mail_template(
            ctx,
            &abandoned_cart,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                    "name": user.name,
                    "items": cart_items,
                    "total": total,
                    "currency": "BRL",
                    "cart_url": format!("{}/cart", domain),
                    "domain": domain,
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    pub async fn send_back_in_stock(
        ctx: &AppContext,
        user: &users::Model,
        product_name: &str,
        variant_name: Option<&str>,
        price: Option<&str>,
        product_slug: &str,
    ) -> Result<()> {
        let domain = ctx.config.server.full_url();

        Self::mail_template(
            ctx,
            &back_in_stock,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                    "name": user.name,
                    "product_name": product_name,
                    "variant_name": variant_name,
                    "price": price,
                    "currency": "BRL",
                    "product_url": format!("{}/products/{}", domain, product_slug),
                    "domain": domain,
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    async fn load_order_items(
        ctx: &AppContext,
        order_id: i32,
    ) -> Result<Vec<OrderItemInfo>> {
        let items = order_items::Entity::find()
            .filter(order_items::Column::OrderId.eq(order_id))
            .all(&ctx.db)
            .await?;

        let mut result = Vec::new();
        for item in items {
            let product_name = products::Entity::find_by_id(item.product_id)
                .one(&ctx.db)
                .await?
                .map(|p| p.name.unwrap_or_else(|| "Unknown Product".to_string()))
                .unwrap_or_else(|| "Unknown Product".to_string());

            result.push(OrderItemInfo {
                name: product_name,
                quantity: item.quantity.unwrap_or(1),
                total: item.total.unwrap_or(rust_decimal::Decimal::ZERO).to_string(),
                price: item.price.unwrap_or(rust_decimal::Decimal::ZERO).to_string(),
            });
        }

        Ok(result)
    }

    pub fn mail_template_public(
        template_name: &str,
    ) -> Result<&'static Dir<'static>> {
        match template_name {
            "order_confirmation" => Ok(&order_confirmation),
            "shipping_update" => Ok(&shipping_update),
            "delivery_confirmation" => Ok(&delivery_confirmation),
            "abandoned_cart" => Ok(&abandoned_cart),
            "back_in_stock" => Ok(&back_in_stock),
            _ => Err(Error::string(&format!("Unknown template: {template_name}"))),
        }
    }

    pub fn get_all_templates() -> Vec<super::auth::TemplateInfo> {
        let mut list = Vec::new();
        let templates = vec![
            ("order_confirmation", &order_confirmation),
            ("shipping_update", &shipping_update),
            ("delivery_confirmation", &delivery_confirmation),
            ("abandoned_cart", &abandoned_cart),
            ("back_in_stock", &back_in_stock),
        ];

        for (name, dir) in templates {
            let subject = dir
                .get_file("subject.t")
                .and_then(|f| f.contents_utf8())
                .unwrap_or("")
                .trim()
                .to_string();

            let html = dir
                .get_file("html.t")
                .and_then(|f| f.contents_utf8())
                .unwrap_or("")
                .to_string();

            let text = dir
                .get_file("text.t")
                .and_then(|f| f.contents_utf8())
                .unwrap_or("")
                .to_string();

            list.push(super::auth::TemplateInfo {
                name: name.to_string(),
                subject,
                html,
                text,
            });
        }

        list
    }
}

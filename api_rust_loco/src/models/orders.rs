use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
pub use super::_entities::orders::{ActiveModel, Model, Entity};
pub type Orders = Entity;

#[derive(Debug, Deserialize)]
pub struct CreateOrderParams {
    pub items: Vec<OrderItemParam>,
    pub subtotal: Decimal,
    pub total_amount: Decimal,
    pub shipping_amount: Option<Decimal>,
    pub discount_amount: Option<Decimal>,
    pub notes: Option<String>,
    pub payment_method_id: Option<i32>,
    pub shipping_method_id: Option<i32>,
    pub address_first_name: Option<String>,
    pub address_last_name: Option<String>,
    pub address_company: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_zip_code: Option<String>,
    pub address_country: Option<String>,
    pub address_phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OrderItemParam {
    pub product_id: i32,
    pub quantity: i32,
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusParams {
    pub status: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderItemJson {
    pub id: i32,
    pub product_id: i32,
    pub quantity: Option<i32>,
    pub price: Option<Decimal>,
    pub total: Option<Decimal>,
    pub product_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderWithItems {
    pub id: i32,
    pub order_number: Option<String>,
    pub status: Option<i32>,
    pub total_amount: Option<Decimal>,
    pub subtotal: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub shipping_amount: Option<Decimal>,
    pub discount_amount: Option<Decimal>,
    pub currency: Option<String>,
    pub payment_status: Option<i32>,
    pub fulfillment_status: Option<i32>,
    pub notes: Option<String>,
    pub user_id: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub items: Vec<OrderItemJson>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl Model {}

impl ActiveModel {}

impl Entity {}

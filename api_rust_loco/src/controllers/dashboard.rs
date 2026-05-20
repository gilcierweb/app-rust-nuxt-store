#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sea_orm::{entity::*, query::*, sea_query::Expr};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::cache::dashboard_cache;
use crate::models::_entities::{categories, order_items, orders, products, users, payments, payment_gateway_events};
use crate::models::payment_gateway_status::{PaymentAttemptStatus, PaymentGatewayEventStatus};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DashboardResponse {
    pub kpi_stats: Vec<KpiStat>,
    pub sales_data: Vec<SalesDataPoint>,
    pub category_data: Vec<CategoryDataPoint>,
    pub top_products: Vec<TopProduct>,
    pub recent_orders: Vec<RecentOrder>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KpiStat {
    pub title: String,
    pub value: String,
    pub trend: String,
    pub trend_up: bool,
    pub icon: String,
    pub color_class: String,
    pub text_class: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SalesDataPoint {
    pub date: String,
    pub sales: f64,
    pub orders: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDataPoint {
    pub name: String,
    pub value: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopProduct {
    pub name: String,
    pub sales: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentOrder {
    pub id: i32,
    pub customer: String,
    pub total: f64,
    pub status: String,
    pub status_label: String,
    pub status_class: String,
}

#[debug_handler]
pub async fn stats(State(ctx): State<AppContext>) -> Result<Response> {
    if let Some(value) = dashboard_cache().get("stats") {
        return format::json(value);
    }

    // Define all futures for parallel execution
    let total_revenue_fut = orders::Entity::find()
        .select_only()
        .column_as(orders::Column::TotalAmount.sum(), "total")
        .into_tuple::<(Option<Decimal>,)>()
        .one(&ctx.db);

    let total_orders_fut = orders::Entity::find().count(&ctx.db);
    let total_customers_fut = users::Entity::find().count(&ctx.db);

    let seven_days_ago = chrono::Utc::now() - chrono::Duration::days(7);
    let sales_results_fut = orders::Entity::find()
        .select_only()
        .column_as(Expr::cust("CAST(created_at AS DATE)"), "date")
        .column_as(orders::Column::TotalAmount.sum(), "sales")
        .column_as(orders::Column::Id.count(), "orders")
        .filter(orders::Column::CreatedAt.gte(seven_days_ago))
        .group_by(Expr::cust("CAST(created_at AS DATE)"))
        .order_by_asc(Expr::cust("CAST(created_at AS DATE)"))
        .into_tuple::<(chrono::NaiveDate, Option<Decimal>, i64)>()
        .all(&ctx.db);

    let all_category_results_fut = categories::Entity::find()
        .join(JoinType::LeftJoin, categories::Relation::Products.def())
        .join(JoinType::LeftJoin, products::Relation::OrderItems.def())
        .select_only()
        .column(categories::Column::Name)
        .column_as(order_items::Column::Id.count(), "count")
        .group_by(categories::Column::Name)
        .order_by_desc(Expr::cust("count"))
        .into_tuple::<(Option<String>, Option<i64>)>()
        .all(&ctx.db);

    let top_products_results_fut = products::Entity::find()
        .join(JoinType::LeftJoin, products::Relation::OrderItems.def())
        .select_only()
        .column(products::Column::Name)
        .column_as(order_items::Column::Quantity.sum(), "sales_sum")
        .group_by(products::Column::Name)
        .order_by_desc(Expr::cust("sales_sum"))
        .limit(5)
        .into_tuple::<(Option<String>, Option<i64>)>()
        .all(&ctx.db);

    let recent_orders_results_fut = orders::Entity::find()
        .find_also_related(users::Entity)
        .order_by_desc(orders::Column::CreatedAt)
        .limit(5)
        .all(&ctx.db);

    let total_refunds_fut = payments::Entity::find()
        .filter(payments::Column::Status.eq(PaymentAttemptStatus::Refunded.to_i16() as i32))
        .select_only()
        .column_as(payments::Column::Amount.sum(), "total")
        .into_tuple::<(Option<Decimal>,)>()
        .one(&ctx.db);

    let failed_payments_fut = payments::Entity::find()
        .filter(payments::Column::Status.eq(PaymentAttemptStatus::Failed.to_i16() as i32))
        .count(&ctx.db);

    let total_payments_fut = payments::Entity::find().count(&ctx.db);

    let webhook_failures_fut = payment_gateway_events::Entity::find()
        .filter(payment_gateway_events::Column::Status.eq(PaymentGatewayEventStatus::Failed.to_i16()))
        .count(&ctx.db);

    // Execute all queries in parallel
    let (
        total_revenue_res,
        total_orders,
        total_customers,
        sales_results,
        all_category_results,
        top_products_results,
        recent_orders_results,
        total_refunds_res,
        failed_payments,
        total_payments,
        webhook_failures,
    ) = tokio::try_join!(
        total_revenue_fut,
        total_orders_fut,
        total_customers_fut,
        sales_results_fut,
        all_category_results_fut,
        top_products_results_fut,
        recent_orders_results_fut,
        total_refunds_fut,
        failed_payments_fut,
        total_payments_fut,
        webhook_failures_fut,
    )
    .map_err(|e| {
        tracing::error!(error = ?e, "Failed to execute dashboard queries in parallel");
        Error::InternalServerError
    })?;

    // 1. Process KPI Stats
    let total_revenue = total_revenue_res.and_then(|(total,)| total);
    let revenue_f64 = total_revenue.unwrap_or_default().to_f64().unwrap_or(0.0);
    let conversion_rate = if total_customers > 0 {
        (total_orders as f64 / total_customers as f64) * 100.0
    } else {
        0.0
    };

    let kpi_stats = vec![
        KpiStat {
            title: "admin.dashboard.kpis.monthlyRevenue".to_string(),
            value: format!("{:.2}", revenue_f64),
            trend: "12%".to_string(),
            trend_up: true,
            icon: "icon-[tabler--cash]".to_string(),
            color_class: "bg-success".to_string(),
            text_class: "text-white".to_string(),
        },
        KpiStat {
            title: "admin.dashboard.kpis.newOrders".to_string(),
            value: total_orders.to_string(),
            trend: "8%".to_string(),
            trend_up: true,
            icon: "icon-[tabler--package]".to_string(),
            color_class: "bg-primary".to_string(),
            text_class: "text-white".to_string(),
        },
        KpiStat {
            title: "admin.dashboard.kpis.newCustomers".to_string(),
            value: total_customers.to_string(),
            trend: "3%".to_string(),
            trend_up: false,
            icon: "icon-[tabler--users]".to_string(),
            color_class: "bg-info".to_string(),
            text_class: "text-white".to_string(),
        },
        KpiStat {
            title: "admin.dashboard.kpis.conversionRate".to_string(),
            value: format!("{:.2}%", conversion_rate),
            trend: "0.5%".to_string(),
            trend_up: true,
            icon: "icon-[tabler--chart-pie]".to_string(),
            color_class: "bg-warning".to_string(),
            text_class: "text-white".to_string(),
        },
    ];

    let total_refunds = total_refunds_res.and_then(|(total,)| total).unwrap_or_default().to_f64().unwrap_or(0.0);
    let failed_payment_rate = if total_payments > 0 {
        (failed_payments as f64 / total_payments as f64) * 100.0
    } else {
        0.0
    };

    let mut kpi_stats = kpi_stats;
    kpi_stats.push(KpiStat {
        title: "Refunds".to_string(),
        value: format!("{:.2}", total_refunds),
        trend: "".to_string(),
        trend_up: false,
        icon: "icon-[tabler--receipt-refund]".to_string(),
        color_class: "bg-error".to_string(),
        text_class: "text-white".to_string(),
    });
    kpi_stats.push(KpiStat {
        title: "Failed Payments".to_string(),
        value: format!("{:.2}%", failed_payment_rate),
        trend: "".to_string(),
        trend_up: false,
        icon: "icon-[tabler--credit-card-off]".to_string(),
        color_class: "bg-error".to_string(),
        text_class: "text-white".to_string(),
    });
    kpi_stats.push(KpiStat {
        title: "Webhook Failures".to_string(),
        value: webhook_failures.to_string(),
        trend: "".to_string(),
        trend_up: false,
        icon: "icon-[tabler--webhook]".to_string(),
        color_class: "bg-error".to_string(),
        text_class: "text-white".to_string(),
    });

    // 2. Process Sales Data
    let sales_data = sales_results
        .into_iter()
        .map(|(date, sales, orders)| SalesDataPoint {
            date: date.format("%d/%m").to_string(),
            sales: sales.unwrap_or_default().to_f64().unwrap_or(0.0),
            orders,
        })
        .collect();

    // 3. Process Category Data
    let mut category_data = Vec::new();
    let mut others_count = 0;
    for (i, (name, count)) in all_category_results.into_iter().enumerate() {
        let name = name.unwrap_or_else(|| "admin.statusLabels.unknown".to_string());
        let count = count.unwrap_or(0);
        if i < 5 {
            category_data.push(CategoryDataPoint { name, value: count });
        } else {
            others_count += count;
        }
    }
    if others_count > 0 {
        category_data.push(CategoryDataPoint {
            name: "admin.dashboard.categories.others".to_string(),
            value: others_count,
        });
    }
    if category_data.is_empty() {
        category_data.push(CategoryDataPoint {
            name: "admin.statusLabels.unknown".to_string(),
            value: 0,
        });
    }

    // 4. Process Top Products
    let top_products = top_products_results
        .into_iter()
        .map(|(name, sales)| TopProduct {
            name: name.unwrap_or_else(|| "admin.statusLabels.unknown".to_string()),
            sales: sales.unwrap_or_default(),
        })
        .collect();

    // 5. Process Recent Orders
    let recent_orders = recent_orders_results
        .into_iter()
        .map(|(order, user)| {
            let customer_name = user
                .map(|u| u.name)
                .unwrap_or_else(|| "admin.statusLabels.unknown".to_string());
            RecentOrder {
                id: order.id,
                customer: customer_name,
                total: order
                    .total_amount
                    .unwrap_or_default()
                    .to_f64()
                    .unwrap_or(0.0),
                status: "paid".to_string(),
                status_label: "admin.statusLabels.paid".to_string(),
                status_class: "badge-success".to_string(),
            }
        })
        .collect();

    let response = Arc::new(DashboardResponse {
        kpi_stats,
        sales_data,
        category_data,
        top_products,
        recent_orders,
    });

    dashboard_cache().insert("stats", Arc::clone(&response));

    format::json(response)
}

pub fn routes() -> Routes {
    routes_with_prefix("api/dashboards")
}

pub fn admin_routes() -> Routes {
    routes_with_prefix("api/admin/dashboards")
}

fn routes_with_prefix(prefix: &str) -> Routes {
    Routes::new().prefix(prefix).add("/stats", get(stats))
}

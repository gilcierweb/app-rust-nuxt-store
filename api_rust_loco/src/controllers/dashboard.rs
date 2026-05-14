#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sea_orm::{entity::*, query::*, sea_query::Expr};
use serde::{Deserialize, Serialize};

use crate::models::_entities::{categories, order_items, orders, products, users};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DashboardResponse {
    pub kpi_stats: Vec<KpiStat>,
    pub sales_data: Vec<SalesDataPoint>,
    pub category_data: Vec<CategoryDataPoint>,
    pub top_products: Vec<TopProduct>,
    pub recent_orders: Vec<RecentOrder>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SalesDataPoint {
    pub date: String,
    pub sales: f64,
    pub orders: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDataPoint {
    pub name: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopProduct {
    pub name: String,
    pub sales: i64,
}

#[derive(Serialize, Deserialize, Debug)]
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
    // 1. KPI Stats
    let total_revenue = orders::Entity::find()
        .select_only()
        .column_as(orders::Column::TotalAmount.sum(), "total")
        .into_tuple::<(Option<Decimal>,)>()
        .one(&ctx.db)
        .await?
        .and_then(|(total,)| total);

    let total_orders = orders::Entity::find().count(&ctx.db).await?;
    let total_customers = users::Entity::find().count(&ctx.db).await?;

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
            value: format!("{:.2}", conversion_rate),
            trend: "0.5%".to_string(),
            trend_up: true,
            icon: "icon-[tabler--chart-pie]".to_string(),
            color_class: "bg-warning".to_string(),
            text_class: "text-white".to_string(),
        },
    ];

    // 2. Sales Data (Last 7 days)
    let seven_days_ago = chrono::Utc::now() - chrono::Duration::days(7);
    let sales_results = orders::Entity::find()
        .select_only()
        .column_as(Expr::cust("CAST(created_at AS DATE)"), "date")
        .column_as(orders::Column::TotalAmount.sum(), "sales")
        .column_as(orders::Column::Id.count(), "orders")
        .filter(orders::Column::CreatedAt.gte(seven_days_ago))
        .group_by(Expr::cust("CAST(created_at AS DATE)"))
        .order_by_asc(Expr::cust("CAST(created_at AS DATE)"))
        .into_tuple::<(chrono::NaiveDate, Option<Decimal>, i64)>()
        .all(&ctx.db)
        .await?;

    let sales_data = sales_results
        .into_iter()
        .map(|(date, sales, orders)| SalesDataPoint {
            date: date.format("%d/%m").to_string(),
            sales: sales.unwrap_or_default().to_f64().unwrap_or(0.0),
            orders,
        })
        .collect();

    // 3. Category Data (Top 5 + Others)
    let all_category_results = categories::Entity::find()
        .join(JoinType::LeftJoin, categories::Relation::Products.def())
        .join(JoinType::LeftJoin, products::Relation::OrderItems.def())
        .select_only()
        .column(categories::Column::Name)
        .column_as(order_items::Column::Id.count(), "count")
        .group_by(categories::Column::Name)
        .order_by_desc(Expr::cust("count"))
        .into_tuple::<(Option<String>, Option<i64>)>()
        .all(&ctx.db)
        .await?;

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

    // 4. Top Products
    let top_products_results = products::Entity::find()
        .join(JoinType::LeftJoin, products::Relation::OrderItems.def())
        .select_only()
        .column(products::Column::Name)
        .column_as(order_items::Column::Quantity.sum(), "sales_sum")
        .group_by(products::Column::Name)
        .order_by_desc(Expr::cust("sales_sum"))
        .limit(5)
        .into_tuple::<(Option<String>, Option<Decimal>)>()
        .all(&ctx.db)
        .await?;

    let top_products = top_products_results
        .into_iter()
        .map(|(name, sales)| TopProduct {
            name: name.unwrap_or_else(|| "admin.statusLabels.unknown".to_string()),
            sales: sales.unwrap_or_default().to_i64().unwrap_or(0),
        })
        .collect();

    // 5. Recent Orders
    let recent_orders_results = orders::Entity::find()
        .find_also_related(users::Entity)
        .order_by_desc(orders::Column::CreatedAt)
        .limit(5)
        .all(&ctx.db)
        .await?;

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

    format::json(DashboardResponse {
        kpi_stats,
        sales_data,
        category_data,
        top_products,
        recent_orders,
    })
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

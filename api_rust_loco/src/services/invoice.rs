use loco_rs::prelude::*;
use printpdf::{
    BuiltinFont, Color, Line, LinePoint, Mm, Op, PdfDocument, PdfFontHandle, PdfPage,
    PdfSaveOptions, Point, Pt, Rgb, TextItem,
};
use rust_decimal::Decimal;
use sea_orm::QueryOrder;

use crate::models::_entities::{addresses, admin_settings, order_items, orders, products, users};
use crate::models::order_status::{OrderStatus, PaymentStatus};

const PAGE_WIDTH_MM: f32 = 210.0;
const MARGIN_LEFT_MM: f32 = 20.0;
const MARGIN_RIGHT_MM: f32 = 20.0;

#[derive(Debug, Clone)]
pub struct StoreConfig {
    pub name: String,
    pub address: String,
    pub email: String,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            name: "Gilcierweb Store".to_string(),
            address: "Sao Paulo, SP - Brazil".to_string(),
            email: "contact@gilcierweb.com".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct InvoiceData {
    pub order: orders::Model,
    pub user: users::Model,
    pub items: Vec<InvoiceItem>,
    pub address: Option<addresses::Model>,
    pub store: StoreConfig,
}

#[derive(Debug)]
pub struct InvoiceItem {
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total: Decimal,
}

async fn get_setting(db: &impl ConnectionTrait, namespace: &str, key: &str, default: &str) -> String {
    admin_settings::Entity::find()
        .filter(admin_settings::Column::Namespace.eq(namespace))
        .filter(admin_settings::Column::Key.eq(key))
        .one(db)
        .await
        .ok()
        .flatten()
        .and_then(|s| s.value)
        .unwrap_or_else(|| default.to_string())
}

async fn load_store_config(db: &impl ConnectionTrait) -> StoreConfig {
    let name = get_setting(db, "store", "name", "Gilcierweb Store").await;
    let address = get_setting(db, "store", "address", "Sao Paulo, SP - Brazil").await;
    let email = get_setting(db, "store", "email", "contact@gilcierweb.com").await;

    StoreConfig {
        name,
        address,
        email,
    }
}

fn format_currency(amount: Decimal) -> String {
    format!("R$ {:.2}", amount)
}

fn format_date(dt: chrono::DateTime<chrono::FixedOffset>) -> String {
    crate::utils::date_format::format_date(dt)
}

fn order_status_label(status: Option<i32>) -> String {
    match status.and_then(OrderStatus::from_i32) {
        Some(OrderStatus::Pending) => "Pendente".to_string(),
        Some(OrderStatus::Confirmed) => "Confirmado".to_string(),
        Some(OrderStatus::Processing) => "Processando".to_string(),
        Some(OrderStatus::Shipped) => "Enviado".to_string(),
        Some(OrderStatus::Delivered) => "Entregue".to_string(),
        Some(OrderStatus::Cancelled) => "Cancelado".to_string(),
        None => "N/A".to_string(),
    }
}

fn payment_status_label(status: Option<i32>) -> String {
    match status.and_then(PaymentStatus::from_i32) {
        Some(PaymentStatus::Unpaid) => "Nao Pago".to_string(),
        Some(PaymentStatus::Paid) => "Pago".to_string(),
        Some(PaymentStatus::Refunded) => "Reembolsado".to_string(),
        Some(PaymentStatus::PartiallyRefunded) => "Parcialmente Reembolsado".to_string(),
        None => "N/A".to_string(),
    }
}

pub async fn load_invoice_data(
    db: &impl ConnectionTrait,
    order_id: i32,
    user_id: Option<i32>,
) -> Result<InvoiceData> {
    let order = orders::Entity::find_by_id(order_id)
        .one(db)
        .await?
        .ok_or_else(|| loco_rs::Error::NotFound)?;

    if let Some(uid) = user_id {
        if order.user_id != uid {
            return Err(loco_rs::Error::string("unauthorized"));
        }
    }

    let user = users::Entity::find_by_id(order.user_id)
        .one(db)
        .await?
        .ok_or_else(|| loco_rs::Error::NotFound)?;

    let order_items_rows = order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(order_id))
        .all(db)
        .await?;

    let mut items = Vec::new();
    for oi in order_items_rows {
        let product_name = products::Entity::find_by_id(oi.product_id)
            .one(db)
            .await?
            .map(|p| p.name.unwrap_or_else(|| "Product".to_string()))
            .unwrap_or_else(|| "Product".to_string());

        items.push(InvoiceItem {
            product_name,
            quantity: oi.quantity.unwrap_or(1),
            unit_price: oi.price.unwrap_or(Decimal::ZERO),
            total: oi.total.unwrap_or(Decimal::ZERO),
        });
    }

    let address = addresses::Entity::find()
        .filter(addresses::Column::UserId.eq(order.user_id))
        .filter(addresses::Column::Type.eq("shipping"))
        .order_by_desc(addresses::Column::Id)
        .one(db)
        .await?;

    let store = load_store_config(db).await;

    Ok(InvoiceData {
        order,
        user,
        items,
        address,
        store,
    })
}

fn line_op(x1: f32, y1: f32, x2: f32, y2: f32) -> Vec<Op> {
    let black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
    vec![
        Op::SetOutlineColor { col: black },
        Op::SetOutlineThickness { pt: Pt(0.5) },
        Op::DrawLine {
            line: Line {
                points: vec![
                    LinePoint {
                        p: Point::new(Mm(x1), Mm(y1)),
                        bezier: false,
                    },
                    LinePoint {
                        p: Point::new(Mm(x2), Mm(y2)),
                        bezier: false,
                    },
                ],
                is_closed: false,
            },
        },
    ]
}

fn text_op(text: &str, size: f32, x: f32, y: f32, font: &PdfFontHandle) -> Vec<Op> {
    vec![
        Op::StartTextSection,
        Op::SetFont {
            font: font.clone(),
            size: Pt(size),
        },
        Op::SetTextCursor {
            pos: Point::new(Mm(x), Mm(y)),
        },
        Op::ShowText {
            items: vec![TextItem::Text(text.to_string())],
        },
        Op::EndTextSection,
    ]
}

pub fn generate_invoice_pdf(data: &InvoiceData) -> Result<Vec<u8>> {
    let mut doc = PdfDocument::new(&format!(
        "Invoice {}",
        data.order.order_number.as_deref().unwrap_or("")
    ));

    let font_regular = PdfFontHandle::Builtin(BuiltinFont::Helvetica);
    let font_bold = PdfFontHandle::Builtin(BuiltinFont::HelveticaBold);

    let content_width = PAGE_WIDTH_MM - MARGIN_LEFT_MM - MARGIN_RIGHT_MM;
    let mut y: f32 = 270.0;
    let mut ops: Vec<Op> = Vec::new();

    // --- Header ---
    ops.extend(text_op(&data.store.name, 20.0, MARGIN_LEFT_MM, y, &font_bold));
    y -= 6.0;
    ops.extend(text_op(
        "INVOICE / NOTA FISCAL",
        14.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 12.0;

    // --- Order info (right side) ---
    let order_num = data.order.order_number.as_deref().unwrap_or("N/A");
    let order_date = format_date(data.order.created_at);
    let status = order_status_label(data.order.status);
    let pay_status = payment_status_label(data.order.payment_status);

    let right_x = MARGIN_LEFT_MM + content_width - 60.0;
    ops.extend(text_op(
        &format!("Invoice: {order_num}"),
        10.0,
        right_x,
        y,
        &font_regular,
    ));
    y -= 5.0;
    ops.extend(text_op(
        &format!("Date: {order_date}"),
        10.0,
        right_x,
        y,
        &font_regular,
    ));
    y -= 5.0;
    ops.extend(text_op(
        &format!("Status: {status}"),
        10.0,
        right_x,
        y,
        &font_regular,
    ));
    y -= 5.0;
    ops.extend(text_op(
        &format!("Payment: {pay_status}"),
        10.0,
        right_x,
        y,
        &font_regular,
    ));
    y -= 12.0;

    // --- Divider line ---
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 10.0;

    // --- Customer info ---
    ops.extend(text_op("BILL TO:", 10.0, MARGIN_LEFT_MM, y, &font_bold));
    y -= 6.0;
    ops.extend(text_op(
        &data.user.name,
        10.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));
    y -= 5.0;
    ops.extend(text_op(
        &data.user.email,
        10.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));

    // --- Shipping address (right side) ---
    if let Some(ref addr) = data.address {
        let addr_x = MARGIN_LEFT_MM + content_width / 2.0 + 10.0;
        let mut addr_y = y + 5.0;
        ops.extend(text_op("SHIP TO:", 10.0, addr_x, addr_y, &font_bold));
        addr_y -= 6.0;
        if let Some(ref name1) = addr.first_name {
            ops.extend(text_op(name1, 10.0, addr_x, addr_y, &font_regular));
            addr_y -= 5.0;
        }
        if let Some(ref line1) = addr.address1 {
            ops.extend(text_op(line1, 10.0, addr_x, addr_y, &font_regular));
            addr_y -= 5.0;
        }
        if let Some(ref line2) = addr.address2 {
            ops.extend(text_op(line2, 10.0, addr_x, addr_y, &font_regular));
            addr_y -= 5.0;
        }
        let city_line = format!(
            "{}/{}, {} {}",
            addr.city.as_deref().unwrap_or(""),
            addr.state.as_deref().unwrap_or(""),
            addr.zip_code.as_deref().unwrap_or(""),
            addr.country.as_deref().unwrap_or("")
        );
        ops.extend(text_op(&city_line, 10.0, addr_x, addr_y, &font_regular));
    }

    y -= 18.0;

    // --- Items table header ---
    let col_product = MARGIN_LEFT_MM;
    let col_qty = MARGIN_LEFT_MM + 90.0;
    let col_price = MARGIN_LEFT_MM + 110.0;
    let col_total = MARGIN_LEFT_MM + 140.0;

    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 7.0;

    ops.extend(text_op("Product", 10.0, col_product, y, &font_bold));
    ops.extend(text_op("Qty", 10.0, col_qty, y, &font_bold));
    ops.extend(text_op("Unit Price", 10.0, col_price, y, &font_bold));
    ops.extend(text_op("Total", 10.0, col_total, y, &font_bold));
    y -= 7.0;

    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 6.0;

    // --- Items rows ---
    for item in &data.items {
        if y < 40.0 {
            break;
        }
        let name = if item.product_name.len() > 40 {
            format!("{}...", &item.product_name[..37])
        } else {
            item.product_name.clone()
        };
        ops.extend(text_op(&name, 9.0, col_product, y, &font_regular));
        ops.extend(text_op(
            &item.quantity.to_string(),
            9.0,
            col_qty,
            y,
            &font_regular,
        ));
        ops.extend(text_op(
            &format_currency(item.unit_price),
            9.0,
            col_price,
            y,
            &font_regular,
        ));
        ops.extend(text_op(
            &format_currency(item.total),
            9.0,
            col_total,
            y,
            &font_regular,
        ));
        y -= 6.0;
    }

    y -= 4.0;

    // --- Totals ---
    let totals_label_x = MARGIN_LEFT_MM + 130.0;
    let totals_value_x = MARGIN_LEFT_MM + 155.0;

    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 7.0;

    let subtotal = data.order.subtotal.unwrap_or(Decimal::ZERO);
    let shipping = data.order.shipping_amount.unwrap_or(Decimal::ZERO);
    let discount = data.order.discount_amount.unwrap_or(Decimal::ZERO);
    let total = data.order.total_amount.unwrap_or(Decimal::ZERO);

    ops.extend(text_op("Subtotal:", 10.0, totals_label_x, y, &font_regular));
    ops.extend(text_op(
        &format_currency(subtotal),
        10.0,
        totals_value_x,
        y,
        &font_regular,
    ));
    y -= 6.0;

    if shipping > Decimal::ZERO {
        ops.extend(text_op("Shipping:", 10.0, totals_label_x, y, &font_regular));
        ops.extend(text_op(
            &format_currency(shipping),
            10.0,
            totals_value_x,
            y,
            &font_regular,
        ));
        y -= 6.0;
    }

    if discount > Decimal::ZERO {
        ops.extend(text_op("Discount:", 10.0, totals_label_x, y, &font_regular));
        ops.extend(text_op(
            &format!("-{}", format_currency(discount)),
            10.0,
            totals_value_x,
            y,
            &font_regular,
        ));
        y -= 6.0;
    }

    ops.extend(line_op(totals_label_x, y, totals_value_x + 30.0, y));
    y -= 7.0;

    ops.extend(text_op("TOTAL:", 12.0, totals_label_x, y, &font_bold));
    ops.extend(text_op(
        &format_currency(total),
        12.0,
        totals_value_x,
        y,
        &font_bold,
    ));
    y -= 20.0;

    // --- Footer ---
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 6.0;
    ops.extend(text_op(
        &format!("{} - {}", data.store.name, data.store.address),
        8.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));
    y -= 4.0;
    ops.extend(text_op(
        &format!("{} | This is a system-generated invoice.", data.store.email),
        8.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));

    // --- Build page and save ---
    let page = PdfPage::new(Mm(PAGE_WIDTH_MM), Mm(297.0), ops);
    doc.pages.push(page);

    let mut warnings = Vec::new();
    let bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);

    Ok(bytes)
}

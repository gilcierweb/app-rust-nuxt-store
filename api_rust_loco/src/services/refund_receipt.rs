use loco_rs::prelude::*;
use printpdf::{
    BuiltinFont, Color, Line, LinePoint, Mm, Op, PdfDocument, PdfFontHandle, PdfPage,
    PdfSaveOptions, Point, Pt, Rgb, TextItem,
};
use rust_decimal::Decimal;

use crate::models::_entities::{
    orders, payment_gateways, payment_methods, payment_refunds, payments, users,
};
use crate::models::payment_gateway_status::PaymentAttemptStatus;

const STORE_NAME: &str = "Gilcierweb Store";
const STORE_ADDRESS: &str = "Sao Paulo, SP - Brazil";
const STORE_EMAIL: &str = "contact@gilcierweb.com";
const PAGE_WIDTH_MM: f32 = 210.0;
const MARGIN_LEFT_MM: f32 = 20.0;
const MARGIN_RIGHT_MM: f32 = 20.0;

#[derive(Debug)]
pub struct RefundReceiptData {
    pub refund: payment_refunds::Model,
    pub payment: payments::Model,
    pub order: orders::Model,
    pub user: users::Model,
    pub payment_method: Option<payment_methods::Model>,
    pub payment_gateway: Option<payment_gateways::Model>,
}

fn format_currency(amount: Decimal) -> String {
    format!("R$ {:.2}", amount)
}

fn format_datetime_naive(dt: chrono::NaiveDateTime) -> String {
    dt.format("%d/%m/%Y %H:%M").to_string()
}

fn format_datetime_tz(dt: chrono::DateTime<chrono::FixedOffset>) -> String {
    dt.format("%d/%m/%Y %H:%M").to_string()
}

fn refund_status_label(status: i16) -> String {
    match status {
        1 => "Pendente".to_string(),
        2 => "Processando".to_string(),
        3 => "Reembolso Concluido".to_string(),
        4 => "Falhou".to_string(),
        5 => "Cancelado".to_string(),
        _ => "N/A".to_string(),
    }
}

fn payment_status_label(status: Option<i32>) -> String {
    match status.and_then(|s| PaymentAttemptStatus::from_i16(s as i16)) {
        Some(PaymentAttemptStatus::Checkout) => "Checkout".to_string(),
        Some(PaymentAttemptStatus::Pending) => "Pendente".to_string(),
        Some(PaymentAttemptStatus::Processing) => "Processando".to_string(),
        Some(PaymentAttemptStatus::Authorized) => "Autorizado".to_string(),
        Some(PaymentAttemptStatus::Captured) => "Capturado".to_string(),
        Some(PaymentAttemptStatus::Failed) => "Falhou".to_string(),
        Some(PaymentAttemptStatus::Voided) => "Cancelado".to_string(),
        Some(PaymentAttemptStatus::Cancelled) => "Cancelado".to_string(),
        Some(PaymentAttemptStatus::Refunded) => "Reembolsado".to_string(),
        Some(PaymentAttemptStatus::PartiallyRefunded) => "Parcialmente Reembolsado".to_string(),
        None => "N/A".to_string(),
    }
}

pub async fn load_refund_receipt_data(
    db: &impl ConnectionTrait,
    refund_id: i32,
    user_id: Option<i32>,
) -> Result<RefundReceiptData> {
    let refund = payment_refunds::Entity::find_by_id(refund_id)
        .one(db)
        .await?
        .ok_or_else(|| loco_rs::Error::NotFound)?;

    let payment = payments::Entity::find_by_id(refund.payment_id)
        .one(db)
        .await?
        .ok_or_else(|| loco_rs::Error::NotFound)?;

    let order = orders::Entity::find_by_id(payment.order_id)
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

    let payment_method = payment_methods::Entity::find_by_id(payment.payment_method_id)
        .one(db)
        .await?;

    let payment_gateway = if let Some(ref pm) = payment_method {
        if let Some(gw_id) = pm.payment_gateway_id {
            payment_gateways::Entity::find_by_id(gw_id)
                .one(db)
                .await?
        } else {
            None
        }
    } else {
        None
    };

    Ok(RefundReceiptData {
        refund,
        payment,
        order,
        user,
        payment_method,
        payment_gateway,
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

pub fn generate_refund_receipt_pdf(data: &RefundReceiptData) -> Result<Vec<u8>> {
    let mut doc = PdfDocument::new(&format!(
        "Refund Confirmation {}",
        data.refund.external_refund_id.as_deref().unwrap_or("")
    ));

    let font_regular = PdfFontHandle::Builtin(BuiltinFont::Helvetica);
    let font_bold = PdfFontHandle::Builtin(BuiltinFont::HelveticaBold);

    let content_width = PAGE_WIDTH_MM - MARGIN_LEFT_MM - MARGIN_RIGHT_MM;
    let mut y: f32 = 270.0;
    let mut ops: Vec<Op> = Vec::new();

    // --- Header ---
    ops.extend(text_op(STORE_NAME, 20.0, MARGIN_LEFT_MM, y, &font_bold));
    y -= 6.0;
    ops.extend(text_op(
        "REFUND CONFIRMATION / COMPROVANTE DE REEMBOLSO",
        14.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 12.0;

    // --- Refund info (right side) ---
    let refund_id_display = data
        .refund
        .external_refund_id
        .clone()
        .unwrap_or_else(|| format!("#{}", data.refund.id));
    let refund_date = format_datetime_tz(data.refund.created_at.fixed_offset());
    let status = refund_status_label(data.refund.status);

    let right_x = MARGIN_LEFT_MM + content_width - 60.0;
    ops.extend(text_op(
        &format!("Refund: {refund_id_display}"),
        10.0,
        right_x,
        y,
        &font_regular,
    ));
    y -= 5.0;
    ops.extend(text_op(
        &format!("Date: {refund_date}"),
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
    y -= 12.0;

    // --- Divider line ---
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 10.0;

    // --- Refund details ---
    ops.extend(text_op(
        "REFUND DETAILS",
        12.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 8.0;

    let label_x = MARGIN_LEFT_MM;
    let value_x = MARGIN_LEFT_MM + 60.0;

    let refund_amount = data.refund.amount;
    ops.extend(text_op("Refund Amount:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        &format_currency(refund_amount),
        14.0,
        value_x,
        y,
        &font_bold,
    ));
    y -= 6.0;

    ops.extend(text_op("Currency:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        &data.refund.currency,
        10.0,
        value_x,
        y,
        &font_regular,
    ));
    y -= 6.0;

    if let Some(ref ext_id) = data.refund.external_refund_id {
        ops.extend(text_op("External Refund ID:", 10.0, label_x, y, &font_bold));
        ops.extend(text_op(ext_id, 10.0, value_x, y, &font_regular));
        y -= 6.0;
    }

    ops.extend(text_op("Idempotency Key:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        &data.refund.idempotency_key,
        10.0,
        value_x,
        y,
        &font_regular,
    ));
    y -= 6.0;

    if let Some(ref gw) = data.payment_gateway {
        ops.extend(text_op("Gateway:", 10.0, label_x, y, &font_bold));
        ops.extend(text_op(&gw.name, 10.0, value_x, y, &font_regular));
        y -= 6.0;
    }

    if let Some(ref pm) = data.payment_method {
        ops.extend(text_op("Method:", 10.0, label_x, y, &font_bold));
        ops.extend(text_op(
            pm.name.as_deref().unwrap_or("N/A"),
            10.0,
            value_x,
            y,
            &font_regular,
        ));
        y -= 6.0;
    }

    if let Some(ref processed_at) = data.refund.processed_at {
        ops.extend(text_op("Processed At:", 10.0, label_x, y, &font_bold));
        ops.extend(text_op(
            &format_datetime_naive(*processed_at),
            10.0,
            value_x,
            y,
            &font_regular,
        ));
        y -= 6.0;
    }

    if let Some(ref failure_code) = data.refund.failure_code {
        ops.extend(text_op("Failure Code:", 10.0, label_x, y, &font_bold));
        ops.extend(text_op(failure_code, 10.0, value_x, y, &font_regular));
        y -= 6.0;
    }

    if let Some(ref failure_message) = data.refund.failure_message {
        ops.extend(text_op("Failure Message:", 10.0, label_x, y, &font_bold));
        ops.extend(text_op(failure_message, 10.0, value_x, y, &font_regular));
        y -= 6.0;
    }

    y -= 4.0;

    // --- Divider line ---
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 10.0;

    // --- Original payment reference ---
    ops.extend(text_op(
        "ORIGINAL PAYMENT REFERENCE",
        12.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 8.0;

    let payment_number = data.payment.number.as_deref().unwrap_or("N/A");
    ops.extend(text_op("Payment Number:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(payment_number, 10.0, value_x, y, &font_regular));
    y -= 6.0;

    let order_num = data.order.order_number.as_deref().unwrap_or("N/A");
    ops.extend(text_op("Order Number:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(order_num, 10.0, value_x, y, &font_regular));
    y -= 6.0;

    let order_date = data
        .order
        .created_at
        .format("%d/%m/%Y")
        .to_string();
    ops.extend(text_op("Order Date:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(&order_date, 10.0, value_x, y, &font_regular));
    y -= 6.0;

    let original_amount = data.payment.amount.unwrap_or(Decimal::ZERO);
    ops.extend(text_op("Original Amount:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        &format_currency(original_amount),
        10.0,
        value_x,
        y,
        &font_regular,
    ));
    y -= 6.0;

    let payment_status = payment_status_label(data.payment.status);
    ops.extend(text_op("Payment Status:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(&payment_status, 10.0, value_x, y, &font_regular));
    y -= 6.0;

    if let Some(ref tx_id) = data.payment.transaction_id {
        ops.extend(text_op("Transaction ID:", 10.0, label_x, y, &font_bold));
        ops.extend(text_op(tx_id, 10.0, value_x, y, &font_regular));
        y -= 6.0;
    }

    y -= 4.0;

    // --- Divider line ---
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 10.0;

    // --- Customer info ---
    ops.extend(text_op("CUSTOMER", 12.0, MARGIN_LEFT_MM, y, &font_bold));
    y -= 8.0;

    ops.extend(text_op("Name:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(&data.user.name, 10.0, value_x, y, &font_regular));
    y -= 6.0;

    ops.extend(text_op("Email:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(&data.user.email, 10.0, value_x, y, &font_regular));
    y -= 10.0;

    // --- Divider line ---
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 10.0;

    // --- Summary ---
    ops.extend(text_op("REFUND SUMMARY", 12.0, MARGIN_LEFT_MM, y, &font_bold));
    y -= 8.0;

    ops.extend(text_op("Original Payment:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        &format_currency(original_amount),
        10.0,
        value_x,
        y,
        &font_regular,
    ));
    y -= 6.0;

    ops.extend(text_op("Refund Amount:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        &format_currency(refund_amount),
        14.0,
        value_x,
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
        &format!("{STORE_NAME} - {STORE_ADDRESS}"),
        8.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));
    y -= 4.0;
    ops.extend(text_op(
        &format!("{STORE_EMAIL} | This is a system-generated refund confirmation."),
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

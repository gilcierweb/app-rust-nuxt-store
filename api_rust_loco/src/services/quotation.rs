use loco_rs::prelude::*;
use printpdf::{
    BuiltinFont, Color, Line, LinePoint, Mm, Op, PdfDocument, PdfFontHandle, PdfPage,
    PdfSaveOptions, Point, Pt, Rgb, TextItem,
};
use rust_decimal::Decimal;
use sea_orm::QueryOrder;

use crate::models::_entities::{addresses, order_items, orders, products, users};

const STORE_NAME: &str = "Gilcierweb Store";
const STORE_ADDRESS: &str = "Sao Paulo, SP - Brazil";
const STORE_EMAIL: &str = "contact@gilcierweb.com";
const STORE_PHONE: &str = "+55 11 99999-0000";
const STORE_CNPJ: &str = "00.000.000/0001-00";
const PAGE_WIDTH_MM: f32 = 210.0;
const MARGIN_LEFT_MM: f32 = 20.0;
const MARGIN_RIGHT_MM: f32 = 20.0;
const VALIDITY_DAYS: i64 = 30;

#[derive(Debug)]
pub struct QuotationItem {
    pub product_name: String,
    pub sku: Option<String>,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total: Decimal,
}

#[derive(Debug)]
pub struct QuotationData {
    pub order: orders::Model,
    pub user: users::Model,
    pub items: Vec<QuotationItem>,
    pub billing_address: Option<addresses::Model>,
    pub shipping_address: Option<addresses::Model>,
    pub valid_until: chrono::NaiveDate,
}

fn format_currency(amount: Decimal) -> String {
    format!("R$ {:.2}", amount)
}

fn format_date(dt: chrono::NaiveDateTime) -> String {
    dt.format("%d/%m/%Y").to_string()
}

fn format_date_short(dt: chrono::NaiveDate) -> String {
    dt.format("%d/%m/%Y").to_string()
}

pub async fn load_quotation_data(
    db: &impl ConnectionTrait,
    order_id: i32,
    user_id: Option<i32>,
) -> Result<QuotationData> {
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

    let product_ids: Vec<i32> = order_items_rows.iter().map(|oi| oi.product_id).collect();
    let products = products::Entity::find()
        .filter(products::Column::Id.is_in(product_ids.clone()))
        .all(db)
        .await?;

    let product_by_id: std::collections::HashMap<i32, products::Model> =
        products.into_iter().map(|p| (p.id, p)).collect();

    let items = order_items_rows
        .into_iter()
        .map(|oi| {
            let product = product_by_id.get(&oi.product_id);
            let product_name = product
                .and_then(|p| p.name.clone())
                .unwrap_or_else(|| format!("Product #{}", oi.product_id));
            let sku = product.and_then(|p| p.sku.clone());
            let quantity = oi.quantity.unwrap_or(1);
            let unit_price = oi.price.unwrap_or(Decimal::ZERO);
            let total = oi.total.unwrap_or(unit_price * Decimal::from(quantity));
            QuotationItem {
                product_name,
                sku,
                quantity,
                unit_price,
                total,
            }
        })
        .collect();

    let billing_address = addresses::Entity::find()
        .filter(addresses::Column::UserId.eq(order.user_id))
        .filter(addresses::Column::Type.eq("billing"))
        .order_by_desc(addresses::Column::Id)
        .one(db)
        .await
        .ok()
        .flatten();

    let shipping_address = addresses::Entity::find()
        .filter(addresses::Column::UserId.eq(order.user_id))
        .filter(addresses::Column::Type.eq("shipping"))
        .order_by_desc(addresses::Column::Id)
        .one(db)
        .await
        .ok()
        .flatten();

    let order_date = order.created_at.date_naive();
    let valid_until = order_date + chrono::Duration::days(VALIDITY_DAYS);

    Ok(QuotationData {
        order,
        user,
        items,
        billing_address,
        shipping_address,
        valid_until,
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

fn thick_line_op(x1: f32, y1: f32, x2: f32, y2: f32) -> Vec<Op> {
    let black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
    vec![
        Op::SetOutlineColor { col: black },
        Op::SetOutlineThickness { pt: Pt(2.0) },
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

pub fn generate_quotation_pdf(data: &QuotationData) -> Result<Vec<u8>> {
    let mut doc = PdfDocument::new(&format!(
        "Quotation {}",
        data.order.order_number.as_deref().unwrap_or("")
    ));

    let font_regular = PdfFontHandle::Builtin(BuiltinFont::Helvetica);
    let font_bold = PdfFontHandle::Builtin(BuiltinFont::HelveticaBold);

    let content_width = PAGE_WIDTH_MM - MARGIN_LEFT_MM - MARGIN_RIGHT_MM;
    let mut y: f32 = 270.0;
    let mut ops: Vec<Op> = Vec::new();

    // ===========================
    // HEADER
    // ===========================
    ops.extend(text_op(STORE_NAME, 20.0, MARGIN_LEFT_MM, y, &font_bold));
    y -= 6.0;
    ops.extend(text_op(
        "ORCAMENTO / QUOTATION",
        16.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 8.0;

    // Store info (left side)
    ops.extend(text_op(STORE_ADDRESS, 9.0, MARGIN_LEFT_MM, y, &font_regular));
    y -= 4.0;
    ops.extend(text_op(
        &format!("CNPJ: {STORE_CNPJ}"),
        9.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));
    y -= 4.0;
    ops.extend(text_op(STORE_EMAIL, 9.0, MARGIN_LEFT_MM, y, &font_regular));
    y -= 4.0;
    ops.extend(text_op(STORE_PHONE, 9.0, MARGIN_LEFT_MM, y, &font_regular));
    y -= 10.0;

    // Quotation info (right side)
    let order_number = data.order.order_number.as_deref().unwrap_or("N/A");
    let order_date = format_date(data.order.created_at.date_naive().and_hms_opt(0, 0, 0).unwrap_or_default());
    let valid_until = format_date_short(data.valid_until);

    let right_x = MARGIN_LEFT_MM + content_width - 65.0;
    ops.extend(text_op(
        &format!("Orcamento: {order_number}"),
        10.0,
        right_x,
        y,
        &font_bold,
    ));
    y -= 5.0;
    ops.extend(text_op(
        &format!("Data: {order_date}"),
        10.0,
        right_x,
        y,
        &font_regular,
    ));
    y -= 5.0;
    ops.extend(text_op(
        &format!("Valido ate: {valid_until}"),
        10.0,
        right_x,
        y,
        &font_bold,
    ));
    y -= 5.0;
    ops.extend(text_op(
        &format!("({VALIDITY_DAYS} dias)"),
        9.0,
        right_x,
        y,
        &font_regular,
    ));
    y -= 12.0;

    // ===========================
    // THICK DIVIDER
    // ===========================
    ops.extend(thick_line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 10.0;

    // ===========================
    // CUSTOMER / BILLING INFO
    // ===========================
    ops.extend(text_op(
        "CLIENTE / CUSTOMER",
        12.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 8.0;

    let col_label = MARGIN_LEFT_MM;
    let col_value = MARGIN_LEFT_MM + 50.0;

    ops.extend(text_op("Nome / Name:", 9.0, col_label, y, &font_bold));
    ops.extend(text_op(&data.user.name, 9.0, col_value, y, &font_regular));
    y -= 5.0;

    ops.extend(text_op("Email:", 9.0, col_label, y, &font_bold));
    ops.extend(text_op(&data.user.email, 9.0, col_value, y, &font_regular));
    y -= 5.0;

    if let Some(ref addr) = data.billing_address {
        if let Some(ref company) = addr.company {
            if !company.is_empty() {
                ops.extend(text_op("Empresa / Company:", 9.0, col_label, y, &font_bold));
                ops.extend(text_op(company, 9.0, col_value, y, &font_regular));
                y -= 5.0;
            }
        }
        if let Some(ref address1) = addr.address1 {
            ops.extend(text_op("Endereco / Address:", 9.0, col_label, y, &font_bold));
            ops.extend(text_op(address1, 9.0, col_value, y, &font_regular));
            y -= 5.0;
        }
        let city_line = format!(
            "{} - {} {}",
            addr.city.as_deref().unwrap_or(""),
            addr.state.as_deref().unwrap_or(""),
            addr.zip_code.as_deref().unwrap_or("")
        )
        .trim()
        .to_string();
        if !city_line.is_empty() && city_line != " -  " {
            ops.extend(text_op(&city_line, 9.0, col_value, y, &font_regular));
            y -= 5.0;
        }
    }

    y -= 5.0;

    // ===========================
    // THICK DIVIDER
    // ===========================
    ops.extend(thick_line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 10.0;

    // ===========================
    // ITEMS TABLE
    // ===========================
    ops.extend(text_op(
        "ITENS / ITEMS",
        12.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 7.0;

    let col_product = MARGIN_LEFT_MM;
    let col_sku = MARGIN_LEFT_MM + 80.0;
    let col_qty = MARGIN_LEFT_MM + 110.0;
    let col_unit = MARGIN_LEFT_MM + 128.0;
    let col_total = MARGIN_LEFT_MM + 152.0;

    ops.extend(text_op("Produto", 8.0, col_product, y, &font_bold));
    ops.extend(text_op("SKU", 8.0, col_sku, y, &font_bold));
    ops.extend(text_op("Qtd", 8.0, col_qty, y, &font_bold));
    ops.extend(text_op("Unit.", 8.0, col_unit, y, &font_bold));
    ops.extend(text_op("Total", 8.0, col_total, y, &font_bold));
    y -= 5.0;

    ops.extend(line_op(MARGIN_LEFT_MM, y, PAGE_WIDTH_MM - MARGIN_RIGHT_MM, y));
    y -= 5.0;

    for item in &data.items {
        let name_display = if item.product_name.len() > 38 {
            format!("{}...", &item.product_name[..35])
        } else {
            item.product_name.clone()
        };
        ops.extend(text_op(&name_display, 8.0, col_product, y, &font_regular));
        ops.extend(text_op(
            item.sku.as_deref().unwrap_or("-"),
            8.0,
            col_sku,
            y,
            &font_regular,
        ));
        ops.extend(text_op(
            &item.quantity.to_string(),
            8.0,
            col_qty,
            y,
            &font_regular,
        ));
        ops.extend(text_op(
            &format_currency(item.unit_price),
            8.0,
            col_unit,
            y,
            &font_regular,
        ));
        ops.extend(text_op(
            &format_currency(item.total),
            8.0,
            col_total,
            y,
            &font_regular,
        ));
        y -= 5.0;
    }

    y -= 4.0;

    // ===========================
    // TOTALS
    // ===========================
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 7.0;

    let totals_label_x = MARGIN_LEFT_MM + 100.0;
    let totals_value_x = MARGIN_LEFT_MM + 152.0;

    if let Some(subtotal) = data.order.subtotal {
        ops.extend(text_op(
            "Subtotal:",
            9.0,
            totals_label_x,
            y,
            &font_regular,
        ));
        ops.extend(text_op(
            &format_currency(subtotal),
            9.0,
            totals_value_x,
            y,
            &font_regular,
        ));
        y -= 5.0;
    }

    if let Some(tax) = data.order.tax_amount {
        if tax > Decimal::ZERO {
            ops.extend(text_op(
                "Impostos / Tax:",
                9.0,
                totals_label_x,
                y,
                &font_regular,
            ));
            ops.extend(text_op(
                &format_currency(tax),
                9.0,
                totals_value_x,
                y,
                &font_regular,
            ));
            y -= 5.0;
        }
    }

    if let Some(shipping) = data.order.shipping_amount {
        if shipping > Decimal::ZERO {
            ops.extend(text_op(
                "Frete / Shipping:",
                9.0,
                totals_label_x,
                y,
                &font_regular,
            ));
            ops.extend(text_op(
                &format_currency(shipping),
                9.0,
                totals_value_x,
                y,
                &font_regular,
            ));
            y -= 5.0;
        }
    }

    if let Some(discount) = data.order.discount_amount {
        if discount > Decimal::ZERO {
            ops.extend(text_op(
                "Desconto / Discount:",
                9.0,
                totals_label_x,
                y,
                &font_regular,
            ));
            ops.extend(text_op(
                &format_currency(discount),
                9.0,
                totals_value_x,
                y,
                &font_regular,
            ));
            y -= 5.0;
        }
    }

    y -= 2.0;
    ops.extend(thick_line_op(totals_label_x, y, PAGE_WIDTH_MM - MARGIN_RIGHT_MM, y));
    y -= 6.0;

    let total = data.order.total_amount.unwrap_or(Decimal::ZERO);
    ops.extend(text_op("TOTAL:", 12.0, totals_label_x, y, &font_bold));
    ops.extend(text_op(
        &format_currency(total),
        12.0,
        totals_value_x,
        y,
        &font_bold,
    ));
    y -= 12.0;

    // ===========================
    // VALIDITY / TERMS
    // ===========================
    ops.extend(thick_line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 10.0;

    ops.extend(text_op(
        "CONDICOES / TERMS",
        12.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 8.0;

    ops.extend(text_op(
        &format!(
            "Validade: {} dias (ate {})",
            VALIDITY_DAYS,
            format_date_short(data.valid_until)
        ),
        9.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 5.0;

    ops.extend(text_op(
        "Pagamento: A combinar apos aprovacao do orcamento.",
        9.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));
    y -= 5.0;

    ops.extend(text_op(
        "Prazo de entrega: Conforme disponibilidade do produto.",
        9.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));
    y -= 5.0;

    ops.extend(text_op(
        "Este orcamento nao gera obrigacao de compra ate a confirmacao final.",
        9.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));
    y -= 10.0;

    // ===========================
    // FOOTER
    // ===========================
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 5.0;
    ops.extend(text_op(
        &format!("{STORE_NAME} - {STORE_ADDRESS}"),
        8.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));
    y -= 4.0;
    ops.extend(text_op(
        &format!("{STORE_EMAIL} | {STORE_PHONE} | Orcamento gerado pelo sistema."),
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

use loco_rs::prelude::*;
use printpdf::{
    BuiltinFont, Color, Line, LinePoint, Mm, Op, PdfDocument, PdfFontHandle, PdfPage,
    PdfSaveOptions, Point, Pt, Rgb, TextItem,
};
use rust_decimal::Decimal;

use crate::models::_entities::{
    addresses, order_items, orders, products, shipments, shipping_methods, users,
};

const STORE_NAME: &str = "Gilcierweb Store";
const STORE_ADDRESS: &str = "Sao Paulo, SP - Brazil";
const STORE_EMAIL: &str = "contact@gilcierweb.com";
const STORE_PHONE: &str = "+55 11 99999-0000";
const PAGE_WIDTH_MM: f32 = 210.0;
const MARGIN_LEFT_MM: f32 = 20.0;
const MARGIN_RIGHT_MM: f32 = 20.0;

#[derive(Debug)]
pub struct OrderItemData {
    pub product_name: String,
    pub quantity: i32,
    pub price: Decimal,
}

#[derive(Debug)]
pub struct ShippingLabelData {
    pub shipment: shipments::Model,
    pub order: orders::Model,
    pub user: users::Model,
    pub shipping_address: Option<addresses::Model>,
    pub shipping_method: shipping_methods::Model,
    pub items: Vec<OrderItemData>,
}

fn format_currency(amount: Decimal) -> String {
    format!("R$ {:.2}", amount)
}

fn format_datetime(dt: chrono::NaiveDateTime) -> String {
    dt.format("%d/%m/%Y %H:%M").to_string()
}

fn shipment_status_label(status: Option<i32>) -> String {
    match status {
        Some(1) => "Pendente".to_string(),
        Some(2) => "Enviado".to_string(),
        Some(3) => "Entregue".to_string(),
        Some(4) => "Cancelado".to_string(),
        _ => "N/A".to_string(),
    }
}

pub async fn load_shipping_label_data(
    db: &impl ConnectionTrait,
    shipment_id: i32,
) -> Result<ShippingLabelData> {
    let shipment = shipments::Entity::find_by_id(shipment_id)
        .one(db)
        .await?
        .ok_or_else(|| loco_rs::Error::NotFound)?;

    let order = orders::Entity::find_by_id(shipment.order_id)
        .one(db)
        .await?
        .ok_or_else(|| loco_rs::Error::NotFound)?;

    let user = users::Entity::find_by_id(order.user_id)
        .one(db)
        .await?
        .ok_or_else(|| loco_rs::Error::NotFound)?;

    let shipping_method =
        shipping_methods::Entity::find_by_id(shipment.shipping_method_id)
            .one(db)
            .await?
            .ok_or_else(|| loco_rs::Error::NotFound)?;

    let shipping_address = addresses::Entity::find()
        .filter(addresses::Column::UserId.eq(order.user_id))
        .filter(addresses::Column::Default.eq(true))
        .one(db)
        .await?
        .or_else(|| {
            // Fallback: try to find any address for this user
            // This is a sync block but we need async — handle via separate query below
            None
        });

    let shipping_address = if shipping_address.is_some() {
        shipping_address
    } else {
        addresses::Entity::find()
            .filter(addresses::Column::UserId.eq(order.user_id))
            .one(db)
            .await?
    };

    let order_items_rows = order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(order.id))
        .all(db)
        .await?;

    let product_ids: Vec<i32> = order_items_rows.iter().map(|oi| oi.product_id).collect();
    let products = products::Entity::find()
        .filter(products::Column::Id.is_in(product_ids.clone()))
        .all(db)
        .await?;

    let product_by_id: std::collections::HashMap<i32, products::Model> = products
        .into_iter()
        .map(|p| (p.id, p))
        .collect();

    let items = order_items_rows
        .into_iter()
        .map(|oi| {
            let product_name = product_by_id
                .get(&oi.product_id)
                .and_then(|p| p.name.clone())
                .unwrap_or_else(|| format!("Product #{}", oi.product_id));
            OrderItemData {
                product_name,
                quantity: oi.quantity.unwrap_or(0),
                price: oi.price.unwrap_or(Decimal::ZERO),
            }
        })
        .collect();

    Ok(ShippingLabelData {
        shipment,
        order,
        user,
        shipping_address,
        shipping_method,
        items,
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

pub fn generate_shipping_label_pdf(data: &ShippingLabelData) -> Result<Vec<u8>> {
    let mut doc = PdfDocument::new(&format!(
        "Shipping Label {}",
        data.shipment
            .tracking_number
            .as_deref()
            .unwrap_or("")
    ));

    let font_regular = PdfFontHandle::Builtin(BuiltinFont::Helvetica);
    let font_bold = PdfFontHandle::Builtin(BuiltinFont::HelveticaBold);

    let mut y: f32 = 270.0;
    let mut ops: Vec<Op> = Vec::new();

    // ===========================
    // HEADER: Store / Sender info
    // ===========================
    ops.extend(text_op(STORE_NAME, 18.0, MARGIN_LEFT_MM, y, &font_bold));
    y -= 6.0;
    ops.extend(text_op(
        "REMETENTE / SENDER",
        10.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 5.0;
    ops.extend(text_op(STORE_ADDRESS, 9.0, MARGIN_LEFT_MM, y, &font_regular));
    y -= 4.0;
    ops.extend(text_op(STORE_EMAIL, 9.0, MARGIN_LEFT_MM, y, &font_regular));
    y -= 4.0;
    ops.extend(text_op(STORE_PHONE, 9.0, MARGIN_LEFT_MM, y, &font_regular));
    y -= 10.0;

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
    // DESTINATARIO / RECIPIENT
    // ===========================
    ops.extend(text_op(
        "DESTINATARIO / RECIPIENT",
        12.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 8.0;

    if let Some(ref addr) = data.shipping_address {
        let full_name = format!(
            "{} {}",
            addr.first_name.as_deref().unwrap_or(""),
            addr.last_name.as_deref().unwrap_or("")
        )
        .trim()
        .to_string();
        ops.extend(text_op(&full_name, 11.0, MARGIN_LEFT_MM, y, &font_bold));
        y -= 5.0;

        if let Some(ref company) = addr.company {
            if !company.is_empty() {
                ops.extend(text_op(company, 9.0, MARGIN_LEFT_MM, y, &font_regular));
                y -= 4.0;
            }
        }

        if let Some(ref address1) = addr.address1 {
            ops.extend(text_op(address1, 9.0, MARGIN_LEFT_MM, y, &font_regular));
            y -= 4.0;
        }

        if let Some(ref address2) = addr.address2 {
            if !address2.is_empty() {
                ops.extend(text_op(address2, 9.0, MARGIN_LEFT_MM, y, &font_regular));
                y -= 4.0;
            }
        }

        let city_line = format!(
            "{} - {} {}",
            addr.city.as_deref().unwrap_or(""),
            addr.state.as_deref().unwrap_or(""),
            addr.zip_code.as_deref().unwrap_or("")
        )
        .trim()
        .to_string();
        ops.extend(text_op(&city_line, 9.0, MARGIN_LEFT_MM, y, &font_regular));
        y -= 4.0;

        if let Some(ref country) = addr.country {
            ops.extend(text_op(country, 9.0, MARGIN_LEFT_MM, y, &font_regular));
            y -= 4.0;
        }

        if let Some(ref phone) = addr.phone {
            if !phone.is_empty() {
                ops.extend(text_op(
                    &format!("Tel: {phone}"),
                    9.0,
                    MARGIN_LEFT_MM,
                    y,
                    &font_regular,
                ));
                y -= 4.0;
            }
        }
    } else {
        ops.extend(text_op(
            "Endereco nao disponivel",
            10.0,
            MARGIN_LEFT_MM,
            y,
            &font_regular,
        ));
        y -= 5.0;
    }

    y -= 6.0;

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
    // SHIPMENT INFO (right-aligned box)
    // ===========================
    ops.extend(text_op(
        "INFORMACOES DE ENVIO",
        12.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 8.0;

    let label_x = MARGIN_LEFT_MM;
    let value_x = MARGIN_LEFT_MM + 55.0;

    ops.extend(text_op("Pedido / Order:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        data.order.order_number.as_deref().unwrap_or("N/A"),
        10.0,
        value_x,
        y,
        &font_regular,
    ));
    y -= 6.0;

    ops.extend(text_op("Transportadora / Carrier:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        data.shipment.carrier.as_deref().unwrap_or("N/A"),
        10.0,
        value_x,
        y,
        &font_regular,
    ));
    y -= 6.0;

    ops.extend(text_op("Rastreio / Tracking:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        data.shipment
            .tracking_number
            .as_deref()
            .unwrap_or("N/A"),
        10.0,
        value_x,
        y,
        &font_bold,
    ));
    y -= 6.0;

    ops.extend(text_op("Metodo / Method:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        data.shipping_method.name.as_deref().unwrap_or("N/A"),
        10.0,
        value_x,
        y,
        &font_regular,
    ));
    y -= 6.0;

    ops.extend(text_op("Status:", 10.0, label_x, y, &font_bold));
    ops.extend(text_op(
        &shipment_status_label(data.shipment.status),
        10.0,
        value_x,
        y,
        &font_regular,
    ));
    y -= 6.0;

    if let Some(ref shipped_at) = data.shipment.shipped_at {
        ops.extend(text_op("Enviado em / Shipped:", 10.0, label_x, y, &font_bold));
        ops.extend(text_op(
            &format_datetime(*shipped_at),
            10.0,
            value_x,
            y,
            &font_regular,
        ));
        y -= 6.0;
    }

    y -= 4.0;

    // ===========================
    // THIN DIVIDER
    // ===========================
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 8.0;

    // ===========================
    // ORDER ITEMS
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
    let col_qty = MARGIN_LEFT_MM + 100.0;
    let col_price = MARGIN_LEFT_MM + 130.0;

    ops.extend(text_op("Produto", 9.0, col_product, y, &font_bold));
    ops.extend(text_op("Qtd", 9.0, col_qty, y, &font_bold));
    ops.extend(text_op("Valor", 9.0, col_price, y, &font_bold));
    y -= 5.0;

    ops.extend(line_op(MARGIN_LEFT_MM, y, PAGE_WIDTH_MM - MARGIN_RIGHT_MM, y));
    y -= 5.0;

    for item in &data.items {
        let name_display = if item.product_name.len() > 45 {
            format!("{}...", &item.product_name[..42])
        } else {
            item.product_name.clone()
        };
        ops.extend(text_op(&name_display, 8.0, col_product, y, &font_regular));
        ops.extend(text_op(
            &item.quantity.to_string(),
            8.0,
            col_qty,
            y,
            &font_regular,
        ));
        ops.extend(text_op(
            &format_currency(item.price),
            8.0,
            col_price,
            y,
            &font_regular,
        ));
        y -= 5.0;
    }

    y -= 4.0;

    // ===========================
    // ORDER TOTAL
    // ===========================
    ops.extend(line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 6.0;

    let total = data.order.total_amount.unwrap_or(Decimal::ZERO);
    ops.extend(text_op("TOTAL:", 10.0, MARGIN_LEFT_MM, y, &font_bold));
    ops.extend(text_op(
        &format_currency(total),
        12.0,
        col_price,
        y,
        &font_bold,
    ));
    y -= 12.0;

    // ===========================
    // BARCODE AREA (placeholder)
    // ===========================
    ops.extend(thick_line_op(
        MARGIN_LEFT_MM,
        y,
        PAGE_WIDTH_MM - MARGIN_RIGHT_MM,
        y,
    ));
    y -= 10.0;

    ops.extend(text_op(
        "CODIGO DE BARRAS / BARCODE",
        10.0,
        MARGIN_LEFT_MM,
        y,
        &font_bold,
    ));
    y -= 5.0;

    if let Some(ref tracking) = data.shipment.tracking_number {
        ops.extend(text_op(tracking, 14.0, MARGIN_LEFT_MM, y, &font_bold));
        y -= 5.0;
    }

    // Draw barcode lines as visual placeholder
    let barcode_y = y;
    let barcode_height = 15.0;
    let mut bx = MARGIN_LEFT_MM;
    let barcode_end = PAGE_WIDTH_MM - MARGIN_RIGHT_MM;
    let pattern: Vec<f32> = vec![1.0, 2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 1.0, 3.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 1.0, 2.0, 1.0];
    let black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
    for (i, &w) in pattern.iter().enumerate() {
        if bx >= barcode_end {
            break;
        }
        if i % 2 == 0 {
            ops.push(Op::SetOutlineColor { col: black.clone() });
            ops.push(Op::SetOutlineThickness { pt: Pt(w) });
            ops.push(Op::DrawLine {
                line: Line {
                    points: vec![
                        LinePoint {
                            p: Point::new(Mm(bx), Mm(barcode_y)),
                            bezier: false,
                        },
                        LinePoint {
                            p: Point::new(Mm(bx), Mm(barcode_y - barcode_height)),
                            bezier: false,
                        },
                    ],
                    is_closed: false,
                },
            });
        }
        bx += w;
    }
    y -= barcode_height + 8.0;

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
        &format!("{STORE_EMAIL} | Etiqueta de postagem gerada pelo sistema."),
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

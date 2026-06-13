//! NF-e/DANFE PDF generation service.
//!
//! This service generates DANFE (Documento Auxiliar da Nota Fiscal Eletronica)
//! PDFs for Brazilian tax compliance. The DANFE is issued by the seller and
//! contains sensitive fiscal data (emitente CNPJ/IE, tax calculations,
//! SEFAZ authorization placeholder).
//!
//! **Access control:** Admin-only by design. The `admin_nfe` endpoint is
//! registered under `api/admin/orders/{id}/nfe` with admin auth middleware.
//! No customer-facing endpoint exists — this is intentional per Brazilian
//! tax regulations. Customers receive the DANFE as a printed or emailed
//! attachment during the fiscal operation, not via the storefront API.

use loco_rs::prelude::*;
use printpdf::{
    BuiltinFont, Color, Line, LinePoint, Mm, Op, PdfDocument, PdfFontHandle, PdfPage,
    PdfSaveOptions, Point, Pt, Rgb, TextItem,
};
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::models::_entities::{
    admin_settings, addresses, categories, order_items, orders, products, users,
};

const PAGE_WIDTH_MM: f32 = 210.0;
const PAGE_HEIGHT_MM: f32 = 297.0;
const MARGIN_LEFT_MM: f32 = 10.0;
const MARGIN_RIGHT_MM: f32 = 10.0;

const DEFAULT_ICMS_RATE: f64 = 0.18;
const DEFAULT_PIS_RATE: f64 = 0.0165;
const DEFAULT_COFINS_RATE: f64 = 0.076;

#[derive(Debug)]
pub struct NfeEmitente {
    pub cnpj: String,
    pub razao_social: String,
    pub nome_fantasia: String,
    pub logradouro: String,
    pub numero: String,
    pub complemento: String,
    pub bairro: String,
    pub codigo_municipio: String,
    pub nome_municipio: String,
    pub uf: String,
    pub cep: String,
    pub telefone: String,
    pub email: String,
    pub inscricao_estadual: String,
    pub codigo_regime_tributario: String,
}

#[derive(Debug)]
pub struct NfeDestinatario {
    pub cpf_cnpj: String,
    pub razao_social: String,
    pub logradouro: String,
    pub numero: String,
    pub complemento: String,
    pub bairro: String,
    pub codigo_municipio: String,
    pub nome_municipio: String,
    pub uf: String,
    pub cep: String,
    pub telefone: String,
    pub email: String,
    pub inscricao_estadual: String,
}

#[derive(Debug)]
pub struct NfeItem {
    pub numero_item: i32,
    pub codigo_produto: String,
    pub descricao: String,
    pub ncm: String,
    pub cfop: String,
    pub unidade_comercial: String,
    pub quantidade_comercial: Decimal,
    pub valor_unitario_comercial: Decimal,
    pub valor_bruto_produto: Decimal,
    pub unidade_tributavel: String,
    pub quantidade_tributavel: Decimal,
    pub valor_unitario_tributacao: Decimal,
    pub valor_frete: Decimal,
    pub valor_seguro: Decimal,
    pub valor_desconto: Decimal,
    pub valor_outras_despesas: Decimal,
    pub origem_mercadoria: String,
    pub csosn: String,
    pub base_calculo_icms: Decimal,
    pub aliquota_icms: Decimal,
    pub valor_icms: Decimal,
    pub base_calculo_pis: Decimal,
    pub aliquota_pis: Decimal,
    pub valor_pis: Decimal,
    pub base_calculo_cofins: Decimal,
    pub aliquota_cofins: Decimal,
    pub valor_cofins: Decimal,
}

#[derive(Debug)]
pub struct NfeTotais {
    pub base_calculo_icms: Decimal,
    pub valor_icms: Decimal,
    pub valor_icms_desonerado: Decimal,
    pub base_calculo_st: Decimal,
    pub valor_st: Decimal,
    pub valor_total_produtos: Decimal,
    pub valor_frete: Decimal,
    pub valor_seguro: Decimal,
    pub valor_desconto: Decimal,
    pub valor_ii: Decimal,
    pub valor_pis: Decimal,
    pub valor_cofins: Decimal,
    pub valor_outras_despesas: Decimal,
    pub valor_total_nota: Decimal,
}

#[derive(Debug)]
pub struct NfeData {
    pub numero: String,
    pub serie: String,
    pub data_emissao: chrono::NaiveDateTime,
    pub natureza_operacao: String,
    pub emitente: NfeEmitente,
    pub destinatario: NfeDestinatario,
    pub items: Vec<NfeItem>,
    pub totais: NfeTotais,
    pub informacoes_adicionais: String,
}

async fn load_setting(
    db: &impl ConnectionTrait,
    namespace: &str,
    key: &str,
    default: &str,
) -> String {
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

async fn load_tax_rate(db: &impl ConnectionTrait, key: &str, default: f64) -> Decimal {
    let raw = load_setting(db, "nfe", key, &default.to_string()).await;
    raw.parse::<f64>()
        .map(|v| Decimal::try_from(v).unwrap_or(Decimal::try_from(default).unwrap()))
        .unwrap_or(Decimal::try_from(default).unwrap())
}

pub async fn load_nfe_data(db: &impl ConnectionTrait, order_id: i32) -> Result<NfeData> {
    let order = orders::Entity::find_by_id(order_id)
        .one(db)
        .await?
        .ok_or_else(|| loco_rs::Error::NotFound)?;

    let user = users::Entity::find_by_id(order.user_id)
        .one(db)
        .await?
        .ok_or_else(|| loco_rs::Error::NotFound)?;

    let billing_address = addresses::Entity::find()
        .filter(addresses::Column::UserId.eq(order.user_id))
        .filter(addresses::Column::Type.eq("billing"))
        .order_by_desc(addresses::Column::Id)
        .one(db)
        .await
        .ok()
        .flatten();

    let emitente = NfeEmitente {
        cnpj: load_setting(db, "nfe", "emitente_cnpj", "00.000.000/0001-00").await,
        razao_social: load_setting(db, "nfe", "emitente_razao_social", "Gilcierweb Store LTDA").await,
        nome_fantasia: load_setting(db, "nfe", "emitente_nome_fantasia", "Gilcierweb Store").await,
        logradouro: load_setting(db, "nfe", "emitente_logradouro", "Rua Exemplo").await,
        numero: load_setting(db, "nfe", "emitente_numero", "100").await,
        complemento: load_setting(db, "nfe", "emitente_complemento", "").await,
        bairro: load_setting(db, "nfe", "emitente_bairro", "Centro").await,
        codigo_municipio: load_setting(db, "nfe", "emitente_codigo_municipio", "3550308").await,
        nome_municipio: load_setting(db, "nfe", "emitente_nome_municipio", "Sao Paulo").await,
        uf: load_setting(db, "nfe", "emitente_uf", "SP").await,
        cep: load_setting(db, "nfe", "emitente_cep", "01001-000").await,
        telefone: load_setting(db, "nfe", "emitente_telefone", "+5511999990000").await,
        email: load_setting(db, "nfe", "emitente_email", "nfe@gilcierweb.com").await,
        inscricao_estadual: load_setting(db, "nfe", "emitente_ie", "123456789").await,
        codigo_regime_tributario: load_setting(db, "nfe", "emitente_crt", "1").await,
    };

    let addr = billing_address.as_ref();
    let destinatario = NfeDestinatario {
        cpf_cnpj: load_setting(db, "nfe", "default_dest_cpf_cnpj", "000.000.000-00").await,
        razao_social: user.name.clone(),
        logradouro: addr.and_then(|a| a.address1.clone()).unwrap_or_default(),
        numero: addr.and_then(|_| Some("S/N".to_string())).unwrap_or_default(),
        complemento: addr.and_then(|a| a.address2.clone()).unwrap_or_default(),
        bairro: addr.and_then(|_| Some("".to_string())).unwrap_or_default(),
        codigo_municipio: addr.and_then(|_| Some("3550308".to_string())).unwrap_or_default(),
        nome_municipio: addr.and_then(|a| a.city.clone()).unwrap_or_default(),
        uf: addr.and_then(|a| a.state.clone()).unwrap_or_default(),
        cep: addr.and_then(|a| a.zip_code.clone()).unwrap_or_default(),
        telefone: addr.and_then(|a| a.phone.clone()).unwrap_or_default(),
        email: user.email.clone(),
        inscricao_estadual: "ISENTO".to_string(),
    };

    let order_items_rows = order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(order_id))
        .all(db)
        .await?;

    let product_ids: Vec<i32> = order_items_rows.iter().map(|oi| oi.product_id).collect();
    let products_list = products::Entity::find()
        .filter(products::Column::Id.is_in(product_ids.clone()))
        .all(db)
        .await?;

    let category_ids: Vec<i32> = products_list.iter().map(|p| p.category_id).collect();
    let categories_list = categories::Entity::find()
        .filter(categories::Column::Id.is_in(category_ids.clone()))
        .all(db)
        .await?;

    let product_by_id: std::collections::HashMap<i32, products::Model> =
        products_list.into_iter().map(|p| (p.id, p)).collect();
    let category_by_id: std::collections::HashMap<i32, categories::Model> =
        categories_list.into_iter().map(|c| (c.id, c)).collect();

    let icms_rate = load_tax_rate(db, "icms_rate", DEFAULT_ICMS_RATE).await;
    let pis_rate = load_tax_rate(db, "pis_rate", DEFAULT_PIS_RATE).await;
    let cofins_rate = load_tax_rate(db, "cofins_rate", DEFAULT_COFINS_RATE).await;

    let mut items = Vec::new();
    let mut totais = NfeTotais {
        base_calculo_icms: Decimal::ZERO,
        valor_icms: Decimal::ZERO,
        valor_icms_desonerado: Decimal::ZERO,
        base_calculo_st: Decimal::ZERO,
        valor_st: Decimal::ZERO,
        valor_total_produtos: Decimal::ZERO,
        valor_frete: Decimal::ZERO,
        valor_seguro: Decimal::ZERO,
        valor_desconto: Decimal::ZERO,
        valor_ii: Decimal::ZERO,
        valor_pis: Decimal::ZERO,
        valor_cofins: Decimal::ZERO,
        valor_outras_despesas: Decimal::ZERO,
        valor_total_nota: Decimal::ZERO,
    };

    for (idx, oi) in order_items_rows.into_iter().enumerate() {
        let product = product_by_id.get(&oi.product_id);
        let product_name = product
            .and_then(|p| p.name.clone())
            .unwrap_or_else(|| format!("Produto #{}", oi.product_id));
        let sku = product
            .and_then(|p| p.sku.clone())
            .unwrap_or_else(|| format!("SKU-{}", oi.product_id));
        let category = product.and_then(|p| category_by_id.get(&p.category_id));
        let ncm = category
            .and_then(|c| c.name.clone())
            .map(|_| "40111000".to_string())
            .unwrap_or_else(|| "00000000".to_string());

        let quantity = Decimal::from(oi.quantity.unwrap_or(1));
        let unit_price = oi.price.unwrap_or(Decimal::ZERO);
        let total_value = oi.total.unwrap_or(unit_price * quantity);

        let base_icms = total_value;
        let valor_icms = base_icms * icms_rate;
        let base_pis = total_value;
        let valor_pis = base_pis * pis_rate;
        let base_cofins = total_value;
        let valor_cofins = base_cofins * cofins_rate;

        totais.base_calculo_icms += base_icms;
        totais.valor_icms += valor_icms;
        totais.valor_total_produtos += total_value;
        totais.valor_pis += valor_pis;
        totais.valor_cofins += valor_cofins;

        items.push(NfeItem {
            numero_item: (idx + 1) as i32,
            codigo_produto: sku,
            descricao: product_name,
            ncm,
            cfop: "5102".to_string(),
            unidade_comercial: "UN".to_string(),
            quantidade_comercial: quantity,
            valor_unitario_comercial: unit_price,
            valor_bruto_produto: total_value,
            unidade_tributavel: "UN".to_string(),
            quantidade_tributavel: quantity,
            valor_unitario_tributacao: unit_price,
            valor_frete: Decimal::ZERO,
            valor_seguro: Decimal::ZERO,
            valor_desconto: Decimal::ZERO,
            valor_outras_despesas: Decimal::ZERO,
            origem_mercadoria: "0".to_string(),
            csosn: "102".to_string(),
            base_calculo_icms: base_icms,
            aliquota_icms: icms_rate * Decimal::from(100),
            valor_icms,
            base_calculo_pis: base_pis,
            aliquota_pis: pis_rate * Decimal::from(100),
            valor_pis,
            base_calculo_cofins: base_cofins,
            aliquota_cofins: cofins_rate * Decimal::from(100),
            valor_cofins,
        });
    }

    let shipping = order.shipping_amount.unwrap_or(Decimal::ZERO);
    let discount = order.discount_amount.unwrap_or(Decimal::ZERO);
    totais.valor_frete = shipping;
    totais.valor_desconto = discount;
    totais.valor_total_nota =
        totais.valor_total_produtos + shipping - discount + totais.valor_st + totais.valor_ii;

    let numero = format!("NFe-{}", order.order_number.as_deref().unwrap_or("000000"));
    let serie = load_setting(db, "nfe", "serie", "1").await;
    let natureza_operacao = load_setting(db, "nfe", "natureza_operacao", "Venda de mercadoria").await;
    let informacoes_adicionais = order.notes.clone().unwrap_or_default();

    Ok(NfeData {
        numero,
        serie,
        data_emissao: order.created_at.date_naive().and_hms_opt(0, 0, 0).unwrap_or_default(),
        natureza_operacao,
        emitente,
        destinatario,
        items,
        totais,
        informacoes_adicionais,
    })
}

fn format_currency(amount: Decimal) -> String {
    format!("R$ {:.2}", amount)
}

fn format_datetime(dt: chrono::NaiveDateTime) -> String {
    dt.format("%d/%m/%Y %H:%M").to_string()
}

fn format_decimal_4(value: Decimal) -> String {
    format!("{:.4}", value)
}

fn line_op(x1: f32, y1: f32, x2: f32, y2: f32) -> Vec<Op> {
    let black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
    vec![
        Op::SetOutlineColor { col: black },
        Op::SetOutlineThickness { pt: Pt(0.3) },
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
        Op::SetOutlineThickness { pt: Pt(1.5) },
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

fn boxed_section(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    title: &str,
    _font_regular: &PdfFontHandle,
    font_bold: &PdfFontHandle,
) -> Vec<Op> {
    let mut ops = Vec::new();
    let black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));

    // Box border
    ops.push(Op::SetOutlineColor { col: black });
    ops.push(Op::SetOutlineThickness { pt: Pt(0.5) });
    ops.push(Op::DrawLine {
        line: Line {
            points: vec![
                LinePoint {
                    p: Point::new(Mm(x), Mm(y)),
                    bezier: false,
                },
                LinePoint {
                    p: Point::new(Mm(x + width), Mm(y)),
                    bezier: false,
                },
                LinePoint {
                    p: Point::new(Mm(x + width), Mm(y - height)),
                    bezier: false,
                },
                LinePoint {
                    p: Point::new(Mm(x), Mm(y - height)),
                    bezier: false,
                },
            ],
            is_closed: true,
        },
    });

    // Title background (horizontal line as visual separator)
    ops.extend(line_op(x, y - 4.0, x + width, y - 4.0));

    // Title text
    ops.extend(text_op(title, 7.0, x + 1.0, y - 3.0, font_bold));

    ops
}

pub fn generate_nfe_pdf(data: &NfeData) -> Result<Vec<u8>> {
    let mut doc = PdfDocument::new(&format!("DANFE {}", data.numero));

    let font_regular = PdfFontHandle::Builtin(BuiltinFont::Helvetica);
    let font_bold = PdfFontHandle::Builtin(BuiltinFont::HelveticaBold);

    let mut y: f32 = 287.0;
    let mut ops: Vec<Op> = Vec::new();
    let content_width = PAGE_WIDTH_MM - MARGIN_LEFT_MM - MARGIN_RIGHT_MM;

    // ===========================
    // HEADER - NF-e identification
    // ===========================
    ops.extend(boxed_section(MARGIN_LEFT_MM, y, content_width, 14.0, "IDENTIFICACAO DO DOCUMENTO FISCAL", &font_regular, &font_bold));
    y -= 14.0;

    let col1 = MARGIN_LEFT_MM + 2.0;
    let col2 = MARGIN_LEFT_MM + 50.0;
    let col3 = MARGIN_LEFT_MM + 100.0;
    let col4 = MARGIN_LEFT_MM + 140.0;

    ops.extend(text_op("NOTA FISCAL ELETRONICA", 12.0, col1, y + 10.0, &font_bold));
    ops.extend(text_op(&format!("Numero: {}", data.numero), 9.0, col1, y + 6.0, &font_regular));
    ops.extend(text_op(&format!("Serie: {}", data.serie), 9.0, col2, y + 6.0, &font_regular));
    ops.extend(text_op(&format!("Emissao: {}", format_datetime(data.data_emissao)), 9.0, col3, y + 6.0, &font_regular));
    ops.extend(text_op(&format!("Natureza: {}", data.natureza_operacao), 8.0, col1, y + 2.0, &font_regular));

    y -= 20.0;

    // ===========================
    // EMITENTE (Sender)
    // ===========================
    let emit_h = 28.0;
    ops.extend(boxed_section(MARGIN_LEFT_MM, y, content_width, emit_h, "EMITENTE", &font_regular, &font_bold));
    y -= 5.0;

    ops.extend(text_op(&data.emitente.razao_social, 9.0, col1, y, &font_bold));
    ops.extend(text_op(&format!("CNPJ: {}", data.emitente.cnpj), 8.0, col2, y, &font_regular));
    ops.extend(text_op(&format!("IE: {}", data.emitente.inscricao_estadual), 8.0, col3, y, &font_regular));
    ops.extend(text_op(&format!("CRT: {}", data.emitente.codigo_regime_tributario), 8.0, col4, y, &font_regular));
    y -= 5.0;

    let emit_addr = format!(
        "{}, {} - {} - {} / {} - {} - CEP: {}",
        data.emitente.logradouro,
        data.emitente.numero,
        data.emitente.bairro,
        data.emitente.nome_municipio,
        data.emitente.uf,
        data.emitente.nome_municipio,
        data.emitente.cep
    );
    ops.extend(text_op(&emit_addr, 8.0, col1, y, &font_regular));
    y -= 5.0;

    ops.extend(text_op(&format!("Telefone: {} | Email: {}", data.emitente.telefone, data.emitente.email), 7.0, col1, y, &font_regular));

    y -= emit_h + 2.0;

    // ===========================
    // DESTINATARIO (Recipient)
    // ===========================
    let dest_h = 28.0;
    ops.extend(boxed_section(MARGIN_LEFT_MM, y, content_width, dest_h, "DESTINATARIO / REMETENTE", &font_regular, &font_bold));
    y -= 5.0;

    ops.extend(text_op(&data.destinatario.razao_social, 9.0, col1, y, &font_bold));
    ops.extend(text_op(&format!("CPF/CNPJ: {}", data.destinatario.cpf_cnpj), 8.0, col2, y, &font_regular));
    ops.extend(text_op(&format!("IE: {}", data.destinatario.inscricao_estadual), 8.0, col3, y, &font_regular));
    y -= 5.0;

    let dest_addr = format!(
        "{}, {} - {} - {} / {} - {} - CEP: {}",
        data.destinatario.logradouro,
        data.destinatario.numero,
        data.destinatario.bairro,
        data.destinatario.nome_municipio,
        data.destinatario.uf,
        data.destinatario.nome_municipio,
        data.destinatario.cep
    );
    ops.extend(text_op(&dest_addr, 8.0, col1, y, &font_regular));
    y -= 5.0;

    ops.extend(text_op(&format!("Telefone: {} | Email: {}", data.destinatario.telefone, data.destinatario.email), 7.0, col1, y, &font_regular));

    y -= dest_h + 2.0;

    // ===========================
    // PRODUCTS TABLE
    // ===========================
    ops.extend(boxed_section(MARGIN_LEFT_MM, y, content_width, 4.0, "DADOS DOS PRODUTOS / SERVICOS", &font_regular, &font_bold));
    y -= 7.0;

    // Column headers
    let c_item = MARGIN_LEFT_MM + 1.0;
    let c_code = MARGIN_LEFT_MM + 10.0;
    let c_desc = MARGIN_LEFT_MM + 30.0;
    let c_ncm = MARGIN_LEFT_MM + 80.0;
    let c_cfop = MARGIN_LEFT_MM + 95.0;
    let c_un = MARGIN_LEFT_MM + 107.0;
    let c_qty = MARGIN_LEFT_MM + 117.0;
    let c_vun = MARGIN_LEFT_MM + 132.0;
    let c_vtot = MARGIN_LEFT_MM + 152.0;
    let c_icms = MARGIN_LEFT_MM + 170.0;

    ops.extend(text_op("Item", 6.0, c_item, y, &font_bold));
    ops.extend(text_op("Cod", 6.0, c_code, y, &font_bold));
    ops.extend(text_op("Descricao", 6.0, c_desc, y, &font_bold));
    ops.extend(text_op("NCM", 6.0, c_ncm, y, &font_bold));
    ops.extend(text_op("CFOP", 6.0, c_cfop, y, &font_bold));
    ops.extend(text_op("Un", 6.0, c_un, y, &font_bold));
    ops.extend(text_op("Qtd", 6.0, c_qty, y, &font_bold));
    ops.extend(text_op("V.Unit", 6.0, c_vun, y, &font_bold));
    ops.extend(text_op("V.Total", 6.0, c_vtot, y, &font_bold));
    ops.extend(text_op("ICMS%", 6.0, c_icms, y, &font_bold));
    y -= 4.0;

    ops.extend(line_op(MARGIN_LEFT_MM, y, PAGE_WIDTH_MM - MARGIN_RIGHT_MM, y));
    y -= 4.0;

    // Items (max 15 per page)
    for item in data.items.iter().take(15) {
        let desc_display = if item.descricao.len() > 40 {
            format!("{}...", &item.descricao[..37])
        } else {
            item.descricao.clone()
        };

        ops.extend(text_op(&item.numero_item.to_string(), 6.0, c_item, y, &font_regular));
        ops.extend(text_op(
            if item.codigo_produto.len() > 18 { &item.codigo_produto[..18] } else { &item.codigo_produto },
            6.0, c_code, y, &font_regular,
        ));
        ops.extend(text_op(&desc_display, 6.0, c_desc, y, &font_regular));
        ops.extend(text_op(&item.ncm, 6.0, c_ncm, y, &font_regular));
        ops.extend(text_op(&item.cfop, 6.0, c_cfop, y, &font_regular));
        ops.extend(text_op(&item.unidade_comercial, 6.0, c_un, y, &font_regular));
        ops.extend(text_op(
            &format_decimal_4(item.quantidade_comercial),
            6.0, c_qty, y, &font_regular,
        ));
        ops.extend(text_op(
            &format_currency(item.valor_unitario_comercial),
            6.0, c_vun, y, &font_regular,
        ));
        ops.extend(text_op(
            &format_currency(item.valor_bruto_produto),
            6.0, c_vtot, y, &font_regular,
        ));
        ops.extend(text_op(
            &format!("{}%", item.aliquota_icms),
            6.0, c_icms, y, &font_regular,
        ));
        y -= 4.0;
    }

    // Fill remaining space if less than 15 items
    let items_count = data.items.len().min(15);
    if items_count < 15 {
        y -= ((15 - items_count) as f32) * 4.0;
    }

    y -= 2.0;
    ops.extend(line_op(MARGIN_LEFT_MM, y, PAGE_WIDTH_MM - MARGIN_RIGHT_MM, y));

    y -= 8.0;

    // ===========================
    // TAX DETAILS
    // ===========================
    let tax_h = 18.0;
    ops.extend(boxed_section(MARGIN_LEFT_MM, y, content_width, tax_h, "CALCULO DO TRIBUTOS", &font_regular, &font_bold));
    y -= 5.0;

    let tcol1 = MARGIN_LEFT_MM + 2.0;
    let tcol2 = MARGIN_LEFT_MM + 50.0;
    let tcol3 = MARGIN_LEFT_MM + 100.0;
    let tcol4 = MARGIN_LEFT_MM + 150.0;

    ops.extend(text_op("Base Calc. ICMS:", 7.0, tcol1, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.base_calculo_icms), 7.0, tcol1 + 35.0, y, &font_bold));
    ops.extend(text_op("Valor ICMS:", 7.0, tcol2, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_icms), 7.0, tcol2 + 25.0, y, &font_bold));
    ops.extend(text_op("Valor PIS:", 7.0, tcol3, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_pis), 7.0, tcol3 + 20.0, y, &font_bold));
    ops.extend(text_op("Valor COFINS:", 7.0, tcol4, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_cofins), 7.0, tcol4 + 25.0, y, &font_bold));
    y -= 6.0;

    ops.extend(text_op("Base Calc. ST:", 7.0, tcol1, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.base_calculo_st), 7.0, tcol1 + 35.0, y, &font_regular));
    ops.extend(text_op("Valor ST:", 7.0, tcol2, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_st), 7.0, tcol2 + 25.0, y, &font_regular));
    ops.extend(text_op("Valor II:", 7.0, tcol3, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_ii), 7.0, tcol3 + 20.0, y, &font_regular));
    ops.extend(text_op("ICMS Desonerado:", 7.0, tcol4, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_icms_desonerado), 7.0, tcol4 + 30.0, y, &font_regular));

    y -= tax_h + 2.0;

    // ===========================
    // TOTALS
    // ===========================
    let total_h = 16.0;
    ops.extend(boxed_section(MARGIN_LEFT_MM, y, content_width, total_h, "VALOR TOTAL DO PEDIDO", &font_regular, &font_bold));
    y -= 5.0;

    ops.extend(text_op("Produtos:", 8.0, tcol1, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_total_produtos), 8.0, tcol1 + 25.0, y, &font_bold));
    ops.extend(text_op("Frete:", 8.0, tcol2, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_frete), 8.0, tcol2 + 15.0, y, &font_bold));
    ops.extend(text_op("Seguro:", 8.0, tcol3, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_seguro), 8.0, tcol3 + 15.0, y, &font_bold));
    ops.extend(text_op("Desconto:", 8.0, tcol4, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_desconto), 8.0, tcol4 + 20.0, y, &font_bold));
    y -= 6.0;

    ops.extend(text_op("Outras Despesas:", 8.0, tcol1, y, &font_regular));
    ops.extend(text_op(&format_currency(data.totais.valor_outras_despesas), 8.0, tcol1 + 38.0, y, &font_regular));

    let total_label_x = MARGIN_LEFT_MM + 100.0;
    let total_value_x = MARGIN_LEFT_MM + 145.0;
    ops.extend(thick_line_op(total_label_x, y - 2.0, PAGE_WIDTH_MM - MARGIN_RIGHT_MM, y - 2.0));
    ops.extend(text_op("VALOR TOTAL DA NOTA:", 10.0, total_label_x, y - 6.0, &font_bold));
    ops.extend(text_op(&format_currency(data.totais.valor_total_nota), 11.0, total_value_x, y - 6.0, &font_bold));

    y -= total_h + 2.0;

    // ===========================
    // ADDITIONAL INFORMATION
    // ===========================
    if !data.informacoes_adicionais.is_empty() {
        let info_h = 12.0;
        ops.extend(boxed_section(MARGIN_LEFT_MM, y, content_width, info_h, "INFORMACOES ADICIONAIS", &font_regular, &font_bold));
        y -= 5.0;

        let info_text = if data.informacoes_adicionais.len() > 100 {
            format!("{}...", &data.informacoes_adicionais[..97])
        } else {
            data.informacoes_adicionais.clone()
        };
        ops.extend(text_op(&info_text, 7.0, MARGIN_LEFT_MM + 2.0, y, &font_regular));

        y -= info_h + 2.0;
    }

    // ===========================
    // AUTHORIZATION (placeholder)
    // ===========================
    let auth_h = 10.0;
    ops.extend(boxed_section(MARGIN_LEFT_MM, y, content_width, auth_h, "DADOS DA AUTORIZACAO DA SEFAZ", &font_regular, &font_bold));
    y -= 5.0;

    ops.extend(text_op("Chave de Acesso: [PENDENTE DE ENVIO A SEFAZ]", 7.0, MARGIN_LEFT_MM + 2.0, y, &font_regular));
    y -= 4.0;
    ops.extend(text_op("Protocolo de Autorizacao: [PENDENTE]", 7.0, MARGIN_LEFT_MM + 2.0, y, &font_regular));

    y -= auth_h + 2.0;

    // ===========================
    // FOOTER
    // ===========================
    ops.extend(line_op(MARGIN_LEFT_MM, y, PAGE_WIDTH_MM - MARGIN_RIGHT_MM, y));
    y -= 4.0;
    ops.extend(text_op(
        &format!("DANFE gerado em {} | {}", format_datetime(data.data_emissao), data.emitente.nome_fantasia),
        7.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));
    y -= 3.0;
    ops.extend(text_op(
        "Este documento nao substitui a Nota Fiscal Eletronica original. Consulte no site da SEFAZ.",
        6.0,
        MARGIN_LEFT_MM,
        y,
        &font_regular,
    ));

    // --- Build page and save ---
    let page = PdfPage::new(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), ops);
    doc.pages.push(page);

    let mut warnings = Vec::new();
    let bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);

    Ok(bytes)
}

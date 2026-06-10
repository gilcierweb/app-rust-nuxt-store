use loco_rs::prelude::*;
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::services::{invoice, receipt, refund_receipt, shipping_label};

fn zip_to_loco_err(e: zip::result::ZipError) -> loco_rs::Error {
    loco_rs::Error::string(&format!("ZIP error: {e}"))
}

/// Build a ZIP archive containing bulk-exported PDFs.
///
/// Returns the ZIP bytes and a suggested filename.
pub async fn build_orders_zip(
    db: &impl ConnectionTrait,
    order_ids: &[i32],
) -> Result<(Vec<u8>, String)> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o644);

    for &id in order_ids {
        let data = invoice::load_invoice_data(db, id, None).await?;
        let pdf = invoice::generate_invoice_pdf(&data)?;
        let number = data.order.order_number.as_deref().unwrap_or("order");
        writer
            .start_file(format!("invoice-{number}.pdf"), options)
            .map_err(zip_to_loco_err)?;
        writer.write_all(&pdf).map_err(|e| {
            loco_rs::Error::string(&format!("Write error: {e}"))
        })?;
    }

    let cursor = writer.finish().map_err(zip_to_loco_err)?;
    Ok((cursor.into_inner(), "invoices.zip".to_string()))
}

pub async fn build_payments_zip(
    db: &impl ConnectionTrait,
    payment_ids: &[i32],
) -> Result<(Vec<u8>, String)> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o644);

    for &id in payment_ids {
        let data = receipt::load_receipt_data(db, id, None).await?;
        let pdf = receipt::generate_receipt_pdf(&data)?;
        let number = data.payment.number.as_deref().unwrap_or("payment");
        writer
            .start_file(format!("receipt-{number}.pdf"), options)
            .map_err(zip_to_loco_err)?;
        writer.write_all(&pdf).map_err(|e| {
            loco_rs::Error::string(&format!("Write error: {e}"))
        })?;
    }

    let cursor = writer.finish().map_err(zip_to_loco_err)?;
    Ok((cursor.into_inner(), "receipts.zip".to_string()))
}

pub async fn build_refunds_zip(
    db: &impl ConnectionTrait,
    refund_ids: &[i32],
) -> Result<(Vec<u8>, String)> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o644);

    for &id in refund_ids {
        let data = refund_receipt::load_refund_receipt_data(db, id, None).await?;
        let pdf = refund_receipt::generate_refund_receipt_pdf(&data)?;
        let refund_number = data.refund.external_refund_id.as_deref().unwrap_or("refund");
        writer
            .start_file(format!("refund-{refund_number}.pdf"), options)
            .map_err(zip_to_loco_err)?;
        writer.write_all(&pdf).map_err(|e| {
            loco_rs::Error::string(&format!("Write error: {e}"))
        })?;
    }

    let cursor = writer.finish().map_err(zip_to_loco_err)?;
    Ok((cursor.into_inner(), "refunds.zip".to_string()))
}

pub async fn build_shipments_zip(
    db: &impl ConnectionTrait,
    shipment_ids: &[i32],
) -> Result<(Vec<u8>, String)> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o644);

    for &id in shipment_ids {
        let data = shipping_label::load_shipping_label_data(db, id).await?;
        let pdf = shipping_label::generate_shipping_label_pdf(&data)?;
        let tracking = data.shipment.tracking_number.as_deref().unwrap_or("label");
        writer
            .start_file(format!("label-{tracking}.pdf"), options)
            .map_err(zip_to_loco_err)?;
        writer.write_all(&pdf).map_err(|e| {
            loco_rs::Error::string(&format!("Write error: {e}"))
        })?;
    }

    let cursor = writer.finish().map_err(zip_to_loco_err)?;
    Ok((cursor.into_inner(), "shipping-labels.zip".to_string()))
}

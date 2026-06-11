use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum DownloadTaskType {
    BulkExportOrders,
    BulkExportPayments,
    BulkExportRefunds,
    BulkExportShipments,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadWorkerArgs {
    pub task_type: DownloadTaskType,
    pub ids: Vec<i32>,
}

pub struct DownloadWorker {
    pub ctx: AppContext,
}

#[async_trait]
impl BackgroundWorker<DownloadWorkerArgs> for DownloadWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: DownloadWorkerArgs) -> Result<()> {
        tracing::info!(
            task_type = ?args.task_type,
            ids_count = args.ids.len(),
            "starting download worker task"
        );

        let result = match args.task_type {
            DownloadTaskType::BulkExportOrders => {
                crate::services::bulk_pdf::build_orders_zip(&self.ctx.db, &args.ids).await
            }
            DownloadTaskType::BulkExportPayments => {
                crate::services::bulk_pdf::build_payments_zip(&self.ctx.db, &args.ids).await
            }
            DownloadTaskType::BulkExportRefunds => {
                crate::services::bulk_pdf::build_refunds_zip(&self.ctx.db, &args.ids).await
            }
            DownloadTaskType::BulkExportShipments => {
                crate::services::bulk_pdf::build_shipments_zip(&self.ctx.db, &args.ids).await
            }
        };

        match result {
            Ok((zip_bytes, filename)) => {
                tracing::info!(
                    filename,
                    size_bytes = zip_bytes.len(),
                    "download worker task completed"
                );
                Ok(())
            }
            Err(e) => {
                tracing::error!(error = ?e, "download worker task failed");
                Err(e)
            }
        }
    }
}

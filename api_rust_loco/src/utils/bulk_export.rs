use serde::Deserialize;

#[derive(Deserialize)]
pub struct BulkExportParams {
    pub ids: Vec<i32>,
}

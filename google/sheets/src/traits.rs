use crate::{ClientResult, Response};

#[async_trait::async_trait]
pub trait SpreadsheetOps {
    /// Get single cell value.
    /// The `cell_name` is something like `A1` and what is returned is a string representation of
    /// the cell's value.
    async fn cell_get(&self, sheet_id: &str, cell_name: &str) -> ClientResult<Response<String>>;
}

#[async_trait::async_trait]
impl SpreadsheetOps for crate::spreadsheets::Spreadsheets {
    /// Get single cell value.
    /// The `cell_name` is something like `A1` and what is returned is a string representation of
    /// the cell's value.
    async fn cell_get(&self, sheet_id: &str, cell_name: &str) -> ClientResult<Response<String>> {
        let resp = self
            .values_get(
                sheet_id,
                cell_name,
                crate::types::DateTimeRenderOption::FormattedString,
                crate::types::Dimension::Rows,
                crate::types::ValueRenderOption::FormattedValue,
            )
            .await?;

        if let Some(v) = resp.body.values.first() {
            if let Some(v) = v.first() {
                return Ok(Response::new(resp.status, resp.headers, v.to_string()));
            }
        }

        Ok(Response::new(resp.status, resp.headers, String::new()))
    }
}

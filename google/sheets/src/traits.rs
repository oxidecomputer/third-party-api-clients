use crate::ClientResult;

#[async_trait::async_trait]
pub trait SpreadsheetOps {
    /// Get single cell value.
    /// The `cell_name` is something like `A1` and what is returned is a string representation of
    /// the cell's value.
    async fn cell_get(&self, sheet_id: &str, cell_name: &str) -> ClientResult<String>;
}

#[async_trait::async_trait]
impl SpreadsheetOps for crate::spreadsheets::Spreadsheets {
    /// Get single cell value.
    /// The `cell_name` is something like `A1` and what is returned is a string representation of
    /// the cell's value.
    async fn cell_get(&self, sheet_id: &str, cell_name: &str) -> ClientResult<String> {
        let values = self
            .values_get(
                sheet_id,
                cell_name,
                crate::types::DateTimeRenderOption::FormattedString,
                crate::types::Dimension::Rows,
                crate::types::ValueRenderOption::FormattedValue,
            )
            .await
            .unwrap();

        if let Some(v) = values.values.get(0) {
            if let Some(v) = v.get(0) {
                return Ok(v.to_string());
            }
        }

        Ok(String::new())
    }
}

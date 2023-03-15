use crate::Client;
use crate::ClientResult;

pub struct Spreadsheets {
    pub client: Client,
}

impl Spreadsheets {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Spreadsheets { client }
    }

    /**
     * This function performs a `POST` to the `/v4/spreadsheets` endpoint.
     *
     * Creates a spreadsheet, returning the newly created spreadsheet.
     */
    pub async fn create(
        &self,
        body: &crate::types::Spreadsheet,
    ) -> ClientResult<crate::types::Spreadsheet> {
        let url = self.client.url("/v4/spreadsheets", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v4/spreadsheets/{spreadsheetId}` endpoint.
     *
     * Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. By default, data within grids will not be returned. You can include grid data one of two ways: * Specify a field mask listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData URL parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, it is recommended to retrieve only the specific fields of the spreadsheet that you want. To retrieve only subsets of the spreadsheet, use the ranges URL parameter. Multiple ranges can be specified. Limiting the range will return only the portions of the spreadsheet that intersect the requested ranges. Ranges are specified using A1 notation.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `include_grid_data: bool` -- True if we should generate data with the "alternate" series. This differs based on the type and amount of source data.
     * * `ranges: &[String]` -- The ranges that were cleared, in A1 notation. If the requests are for an unbounded range or a ranger larger than the bounds of the sheet, this is the actual ranges that were cleared, bounded to the sheet's limits.
     */
    pub async fn get(
        &self,
        spreadsheet_id: &str,
        include_grid_data: bool,
        ranges: &[String],
    ) -> ClientResult<crate::types::Spreadsheet> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_grid_data {
            query_args.push(("includeGridData".to_string(), include_grid_data.to_string()));
        }
        if !ranges.is_empty() {
            query_args.push(("ranges".to_string(), ranges.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}?{}",
                crate::progenitor_support::encode_path(spreadsheet_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v4/spreadsheets/{spreadsheetId}/developerMetadata/{metadataId}` endpoint.
     *
     * Returns the developer metadata with the specified ID. The caller must specify the spreadsheet ID and the developer metadata's unique metadataId.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `metadata_id: i64` -- The zero-based index where the rule should be inserted.
     */
    pub async fn developer_metadata_get(
        &self,
        spreadsheet_id: &str,
        metadata_id: i64,
    ) -> ClientResult<crate::types::DeveloperMetadata> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/developerMetadata/{}",
                crate::progenitor_support::encode_path(spreadsheet_id),
                crate::progenitor_support::encode_path(&metadata_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}/developerMetadata:search` endpoint.
     *
     * Returns all developer metadata matching the specified DataFilter. If the provided DataFilter represents a DeveloperMetadataLookup object, this will return all DeveloperMetadata entries selected by it. If the DataFilter represents a location in a spreadsheet, this will return all developer metadata associated with locations intersecting that region.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     */
    pub async fn developer_metadata_search(
        &self,
        spreadsheet_id: &str,
        body: &crate::types::SearchDeveloperMetadataRequest,
    ) -> ClientResult<crate::types::SearchDeveloperMetadataResponse> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/developerMetadata:search",
                crate::progenitor_support::encode_path(spreadsheet_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}/sheets/{sheetId}:copyTo` endpoint.
     *
     * Copies a single sheet from a spreadsheet to another spreadsheet. Returns the properties of the newly created sheet.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `sheet_id: i64` -- The zero-based index where the rule should be inserted.
     */
    pub async fn sheets_copy(
        &self,
        spreadsheet_id: &str,
        sheet_id: i64,
        body: &crate::types::CopySheetAnotherSpreadsheetRequest,
    ) -> ClientResult<crate::types::SheetProperties> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/sheets/{}/copyTo",
                crate::progenitor_support::encode_path(spreadsheet_id),
                crate::progenitor_support::encode_path(&sheet_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v4/spreadsheets/{spreadsheetId}/values/{range}` endpoint.
     *
     * Returns a range of values from a spreadsheet. The caller must specify the spreadsheet ID and a range.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `range: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `date_time_render_option: crate::types::DateTimeRenderOption` -- How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
     * * `major_dimension: crate::types::Dimension` -- The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` returns `[[1,3],[2,4]]`.
     * * `value_render_option: crate::types::ValueRenderOption` -- How values should be represented in the output. The default render option is FORMATTED_VALUE.
     */
    pub async fn values_get(
        &self,
        spreadsheet_id: &str,
        range: &str,
        date_time_render_option: crate::types::DateTimeRenderOption,
        major_dimension: crate::types::Dimension,
        value_render_option: crate::types::ValueRenderOption,
    ) -> ClientResult<crate::types::ValueRange> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date_time_render_option.to_string().is_empty() {
            query_args.push((
                "dateTimeRenderOption".to_string(),
                date_time_render_option.to_string(),
            ));
        }
        if !major_dimension.to_string().is_empty() {
            query_args.push(("majorDimension".to_string(), major_dimension.to_string()));
        }
        if !value_render_option.to_string().is_empty() {
            query_args.push((
                "valueRenderOption".to_string(),
                value_render_option.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values/{}?{}",
                crate::progenitor_support::encode_path(spreadsheet_id),
                crate::progenitor_support::encode_path(range),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `PUT` to the `/v4/spreadsheets/{spreadsheetId}/values/{range}` endpoint.
     *
     * Sets values in a range of a spreadsheet. The caller must specify the spreadsheet ID, range, and a valueInputOption.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `range: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `include_values_in_response: bool` -- Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns).
     * * `response_date_time_render_option: crate::types::DateTimeRenderOption` -- Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
     * * `response_value_render_option: crate::types::ValueRenderOption` -- Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
     * * `value_input_option: crate::types::ValueInputOption` -- How the input data should be interpreted.
     */
    pub async fn values_update(
        &self,
        spreadsheet_id: &str,
        range: &str,
        include_values_in_response: bool,
        response_date_time_render_option: crate::types::DateTimeRenderOption,
        response_value_render_option: crate::types::ValueRenderOption,
        value_input_option: crate::types::ValueInputOption,
        body: &crate::types::ValueRange,
    ) -> ClientResult<crate::types::UpdateValuesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_values_in_response {
            query_args.push((
                "includeValuesInResponse".to_string(),
                include_values_in_response.to_string(),
            ));
        }
        if !response_date_time_render_option.to_string().is_empty() {
            query_args.push((
                "responseDateTimeRenderOption".to_string(),
                response_date_time_render_option.to_string(),
            ));
        }
        if !response_value_render_option.to_string().is_empty() {
            query_args.push((
                "responseValueRenderOption".to_string(),
                response_value_render_option.to_string(),
            ));
        }
        if !value_input_option.to_string().is_empty() {
            query_args.push((
                "valueInputOption".to_string(),
                value_input_option.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values/{}?{}",
                crate::progenitor_support::encode_path(spreadsheet_id),
                crate::progenitor_support::encode_path(range),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}/values/{range}:append` endpoint.
     *
     * Appends values to a spreadsheet. The input range is used to search for existing data and find a "table" within that range. Values will be appended to the next row of the table, starting with the first column of the table. See the [guide](/sheets/api/guides/values#appending_values) and [sample code](/sheets/api/samples/writing#append_values) for specific details of how tables are detected and data is appended. The caller must specify the spreadsheet ID, range, and a valueInputOption. The `valueInputOption` only controls how the input data will be added to the sheet (column-wise or row-wise), it does not influence what cell the data starts being written to.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `range: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `include_values_in_response: bool` -- Determines if the update response should include the values of the cells that were appended. By default, responses do not include the updated values.
     * * `insert_data_option: crate::types::InsertDataOption` -- How the input data should be inserted.
     * * `response_date_time_render_option: crate::types::DateTimeRenderOption` -- Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
     * * `response_value_render_option: crate::types::ValueRenderOption` -- Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
     * * `value_input_option: crate::types::ValueInputOption` -- How the input data should be interpreted.
     */
    pub async fn values_append(
        &self,
        spreadsheet_id: &str,
        range: &str,
        include_values_in_response: bool,
        insert_data_option: crate::types::InsertDataOption,
        response_date_time_render_option: crate::types::DateTimeRenderOption,
        response_value_render_option: crate::types::ValueRenderOption,
        value_input_option: crate::types::ValueInputOption,
        body: &crate::types::ValueRange,
    ) -> ClientResult<crate::types::AppendValuesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_values_in_response {
            query_args.push((
                "includeValuesInResponse".to_string(),
                include_values_in_response.to_string(),
            ));
        }
        if !insert_data_option.to_string().is_empty() {
            query_args.push((
                "insertDataOption".to_string(),
                insert_data_option.to_string(),
            ));
        }
        if !response_date_time_render_option.to_string().is_empty() {
            query_args.push((
                "responseDateTimeRenderOption".to_string(),
                response_date_time_render_option.to_string(),
            ));
        }
        if !response_value_render_option.to_string().is_empty() {
            query_args.push((
                "responseValueRenderOption".to_string(),
                response_value_render_option.to_string(),
            ));
        }
        if !value_input_option.to_string().is_empty() {
            query_args.push((
                "valueInputOption".to_string(),
                value_input_option.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values/{}/append?{}",
                crate::progenitor_support::encode_path(spreadsheet_id),
                crate::progenitor_support::encode_path(range),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}/values/{range}:clear` endpoint.
     *
     * Clears values from a spreadsheet. The caller must specify the spreadsheet ID and range. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `range: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     */
    pub async fn values_clear(
        &self,
        spreadsheet_id: &str,
        range: &str,
        body: &crate::types::ClearValuesRequest,
    ) -> ClientResult<crate::types::ClearValuesResponse> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values/{}/clear",
                crate::progenitor_support::encode_path(spreadsheet_id),
                crate::progenitor_support::encode_path(range),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}/values:batchClear` endpoint.
     *
     * Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     */
    pub async fn values_batch_clear(
        &self,
        spreadsheet_id: &str,
        body: &crate::types::BatchClearValuesRequest,
    ) -> ClientResult<crate::types::BatchClearValuesResponse> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values:batchClear",
                crate::progenitor_support::encode_path(spreadsheet_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}/values:batchClearByDataFilter` endpoint.
     *
     * Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges matching any of the specified data filters will be cleared. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     */
    pub async fn values_batch_clear_data_filter(
        &self,
        spreadsheet_id: &str,
        body: &crate::types::BatchClearValuesByDataFilterRequest,
    ) -> ClientResult<crate::types::BatchClearValuesByDataFilterResponse> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values:batchClearByDataFilter",
                crate::progenitor_support::encode_path(spreadsheet_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v4/spreadsheets/{spreadsheetId}/values:batchGet` endpoint.
     *
     * Returns one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     * * `date_time_render_option: crate::types::DateTimeRenderOption` -- How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
     * * `major_dimension: crate::types::Dimension` -- The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` returns `[[1,3],[2,4]]`.
     * * `ranges: &[String]` -- The ranges that were cleared, in A1 notation. If the requests are for an unbounded range or a ranger larger than the bounds of the sheet, this is the actual ranges that were cleared, bounded to the sheet's limits.
     * * `value_render_option: crate::types::ValueRenderOption` -- How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE.
     */
    pub async fn values_batch_get(
        &self,
        spreadsheet_id: &str,
        date_time_render_option: crate::types::DateTimeRenderOption,
        major_dimension: crate::types::Dimension,
        ranges: &[String],
        value_render_option: crate::types::ValueRenderOption,
    ) -> ClientResult<crate::types::BatchGetValuesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date_time_render_option.to_string().is_empty() {
            query_args.push((
                "dateTimeRenderOption".to_string(),
                date_time_render_option.to_string(),
            ));
        }
        if !major_dimension.to_string().is_empty() {
            query_args.push(("majorDimension".to_string(), major_dimension.to_string()));
        }
        if !ranges.is_empty() {
            query_args.push(("ranges".to_string(), ranges.join(" ")));
        }
        if !value_render_option.to_string().is_empty() {
            query_args.push((
                "valueRenderOption".to_string(),
                value_render_option.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values:batchGet?{}",
                crate::progenitor_support::encode_path(spreadsheet_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}/values:batchGetByDataFilter` endpoint.
     *
     * Returns one or more ranges of values that match the specified data filters. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges that match any of the data filters in the request will be returned.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     */
    pub async fn values_batch_get_data_filter(
        &self,
        spreadsheet_id: &str,
        body: &crate::types::BatchGetValuesByDataFilterRequest,
    ) -> ClientResult<crate::types::BatchGetValuesByDataFilterResponse> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values:batchGetByDataFilter",
                crate::progenitor_support::encode_path(spreadsheet_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}/values:batchUpdate` endpoint.
     *
     * Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more ValueRanges.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     */
    pub async fn values_batch_update(
        &self,
        spreadsheet_id: &str,
        body: &crate::types::BatchUpdateValuesRequest,
    ) -> ClientResult<crate::types::BatchUpdateValuesResponse> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values:batchUpdate",
                crate::progenitor_support::encode_path(spreadsheet_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}/values:batchUpdateByDataFilter` endpoint.
     *
     * Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more DataFilterValueRanges.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     */
    pub async fn values_batch_update_data_filter(
        &self,
        spreadsheet_id: &str,
        body: &crate::types::BatchUpdateValuesByDataFilterRequest,
    ) -> ClientResult<crate::types::BatchUpdateValuesByDataFilterResponse> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/values:batchUpdateByDataFilter",
                crate::progenitor_support::encode_path(spreadsheet_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}:batchUpdate` endpoint.
     *
     * Applies one or more updates to the spreadsheet. Each request is validated before being applied. If any request is not valid then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. The replies will mirror the requests. For example, if you applied 4 updates and the 3rd one had a reply, then the response will have 2 empty replies, the actual reply, and another empty reply, in that order. Due to the collaborative nature of spreadsheets, it is not guaranteed that the spreadsheet will reflect exactly your changes after this completes, however it is guaranteed that the updates in the request will be applied together atomically. Your changes may be altered with respect to collaborator changes. If there are no collaborators, the spreadsheet should reflect your changes.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     */
    pub async fn batch_update(
        &self,
        spreadsheet_id: &str,
        body: &crate::types::BatchUpdateSpreadsheetRequest,
    ) -> ClientResult<crate::types::BatchUpdateSpreadsheetResponse> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/batchUpdate",
                crate::progenitor_support::encode_path(spreadsheet_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v4/spreadsheets/{spreadsheetId}:getByDataFilter` endpoint.
     *
     * Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. This method differs from GetSpreadsheet in that it allows selecting which subsets of spreadsheet data to return by specifying a dataFilters parameter. Multiple DataFilters can be specified. Specifying one or more data filters will return the portions of the spreadsheet that intersect ranges matched by any of the filters. By default, data within grids will not be returned. You can include grid data one of two ways: * Specify a field mask listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, it is recommended to retrieve only the specific fields of the spreadsheet that you want.
     *
     * **Parameters:**
     *
     * * `spreadsheet_id: &str` -- The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"\*"` can be used as short-hand for listing every field.
     */
    pub async fn get_data_filter(
        &self,
        spreadsheet_id: &str,
        body: &crate::types::GetSpreadsheetByDataFilterRequest,
    ) -> ClientResult<crate::types::Spreadsheet> {
        let url = self.client.url(
            &format!(
                "/v4/spreadsheets/{}/getByDataFilter",
                crate::progenitor_support::encode_path(spreadsheet_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}

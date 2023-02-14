//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Adds a new banded range to the spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddBandingRequest {
    /**
     * Adds a new banded range to the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bandedRange"
    )]
    pub banded_range: Option<BandedRange>,
}

/// The result of adding a banded range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddBandingResponse {
    /**
     * The result of adding a banded range.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bandedRange"
    )]
    pub banded_range: Option<BandedRange>,
}

/// Adds a chart to a sheet in the spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddChartRequest {
    /**
     * Adds a chart to a sheet in the spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart: Option<EmbeddedChart>,
}

/// The result of adding a chart to a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddChartResponse {
    /**
     * The result of adding a chart to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart: Option<EmbeddedChart>,
}

/// Adds a new conditional format rule at the given index. All subsequent rules' indexes are incremented.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddConditionalFormatRuleRequest {
    /**
     * Adds a new conditional format rule at the given index. All subsequent rules' indexes are incremented.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub index: i64,
    /**
     * Adds a new conditional format rule at the given index. All subsequent rules' indexes are incremented.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<ConditionalFormatRule>,
}

/// Adds a data source. After the data source is added successfully, an associated DATA_SOURCE sheet is created and an execution is triggered to refresh the sheet to read data from the data source. The request requires an additional `bigquery.readonly` OAuth scope.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddDataSourceRequest {
    /**
     * Adds a data source. After the data source is added successfully, an associated DATA_SOURCE sheet is created and an execution is triggered to refresh the sheet to read data from the data source. The request requires an additional `bigquery.readonly` OAuth scope.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSource"
    )]
    pub data_source: Option<DataSource>,
}

/// The result of adding a data source.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddDataSourceResponse {
    /**
     * The result of adding a data source.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataExecutionStatus"
    )]
    pub data_execution_status: Option<DataExecutionStatus>,
    /**
     * The result of adding a data source.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSource"
    )]
    pub data_source: Option<DataSource>,
}

/// Creates a group over the specified range. If the requested range is a superset of the range of an existing group G, then the depth of G is incremented and this new group G' has the depth of that group. For example, a group [C:D, depth 1] + [B:E] results in groups [B:E, depth 1] and [C:D, depth 2]. If the requested range is a subset of the range of an existing group G, then the depth of the new group G' becomes one greater than the depth of G. For example, a group [B:E, depth 1] + [C:D] results in groups [B:E, depth 1] and [C:D, depth 2]. If the requested range starts before and ends within, or starts within and ends after, the range of an existing group G, then the range of the existing group G becomes the union of the ranges, and the new group G' has depth one greater than the depth of G and range as the intersection of the ranges. For example, a group [B:D, depth 1] + [C:E] results in groups [B:E, depth 1] and [C:D, depth 2].
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddDimensionGroupRequest {
    /**
     * Creates a group over the specified range. If the requested range is a superset of the range of an existing group G, then the depth of G is incremented and this new group G' has the depth of that group. For example, a group [C:D, depth 1] + [B:E] results in groups [B:E, depth 1] and [C:D, depth 2]. If the requested range is a subset of the range of an existing group G, then the depth of the new group G' becomes one greater than the depth of G. For example, a group [B:E, depth 1] + [C:D] results in groups [B:E, depth 1] and [C:D, depth 2]. If the requested range starts before and ends within, or starts within and ends after, the range of an existing group G, then the range of the existing group G becomes the union of the ranges, and the new group G' has depth one greater than the depth of G and range as the intersection of the ranges. For example, a group [B:D, depth 1] + [C:E] results in groups [B:E, depth 1] and [C:D, depth 2].
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<DimensionRange>,
}

/// The result of adding a group.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddDimensionGroupResponse {
    /**
     * The result of adding a group.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dimensionGroups"
    )]
    pub dimension_groups: Vec<DimensionGroup>,
}

/// Adds a filter view.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddFilterViewRequest {
    /**
     * Adds a filter view.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterView>,
}

/// The result of adding a filter view.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddFilterViewResponse {
    /**
     * The result of adding a filter view.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterView>,
}

/// Adds a named range to the spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddNamedRangeRequest {
    /**
     * Adds a named range to the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "namedRange"
    )]
    pub named_range: Option<NamedRange>,
}

/// The result of adding a named range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddNamedRangeResponse {
    /**
     * The result of adding a named range.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "namedRange"
    )]
    pub named_range: Option<NamedRange>,
}

/// Adds a new protected range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddProtectedRangeRequest {
    /**
     * Adds a new protected range.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "protectedRange"
    )]
    pub protected_range: Option<ProtectedRange>,
}

/// The result of adding a new protected range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddProtectedRangeResponse {
    /**
     * The result of adding a new protected range.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "protectedRange"
    )]
    pub protected_range: Option<ProtectedRange>,
}

/// Adds a new sheet. When a sheet is added at a given index, all subsequent sheets' indexes are incremented. To add an object sheet, use AddChartRequest instead and specify EmbeddedObjectPosition.sheetId or EmbeddedObjectPosition.newSheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddSheetRequest {
    /**
     * Adds a new sheet. When a sheet is added at a given index, all subsequent sheets' indexes are incremented. To add an object sheet, use AddChartRequest instead and specify EmbeddedObjectPosition.sheetId or EmbeddedObjectPosition.newSheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SheetProperties>,
}

/// The result of adding a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddSheetResponse {
    /**
     * The result of adding a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SheetProperties>,
}

/// Adds a slicer to a sheet in the spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddSlicerRequest {
    /**
     * Adds a slicer to a sheet in the spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slicer: Option<Slicer>,
}

/// The result of adding a slicer to a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AddSlicerResponse {
    /**
     * The result of adding a slicer to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slicer: Option<Slicer>,
}

/// Adds new cells after the last row with data in a sheet, inserting new rows into the sheet if necessary.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AppendCellsRequest {
    /**
     * Adds new cells after the last row with data in a sheet, inserting new rows into the sheet if necessary.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Adds new cells after the last row with data in a sheet, inserting new rows into the sheet if necessary.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub rows: Vec<RowData>,
    /**
     * Adds new cells after the last row with data in a sheet, inserting new rows into the sheet if necessary.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/**
* Whether rows or columns should be appended.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Dimension {
    #[serde(rename = "COLUMNS")]
    Columns,
    #[serde(rename = "DIMENSION_UNSPECIFIED")]
    DimensionUnspecified,
    #[serde(rename = "ROWS")]
    Rows,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dimension::Columns => "COLUMNS",
            Dimension::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
            Dimension::Rows => "ROWS",
            Dimension::Noop => "",
            Dimension::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Dimension {
    fn default() -> Dimension {
        Dimension::Noop
    }
}
impl Dimension {
    pub fn is_noop(&self) -> bool {
        matches!(self, Dimension::Noop)
    }
}

/// Appends rows or columns to the end of a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AppendDimensionRequest {
    /**
     * Appends rows or columns to the end of a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Dimension>,
    /**
     * Appends rows or columns to the end of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub length: i64,
    /**
     * Appends rows or columns to the end of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/// The response when updating a range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AppendValuesResponse {
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "tableRange"
    )]
    pub table_range: String,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updates: Option<UpdateValuesResponse>,
}

/// Fills in more data based on existing data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutoFillRequest {
    /**
     * Fills in more data based on existing data.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * Fills in more data based on existing data.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceAndDestination"
    )]
    pub source_and_destination: Option<SourceDestination>,
    /**
     * Fills in more data based on existing data.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "useAlternateSeries"
    )]
    pub use_alternate_series: Option<bool>,
}

/// Automatically resizes one or more dimensions based on the contents of the cells in that dimension.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct AutoResizeDimensionsRequest {
    /**
     * Automatically resizes one or more dimensions based on the contents of the cells in that dimension.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceSheetDimensions"
    )]
    pub data_source_sheet_dimensions: Option<DataSourceSheetDimensionRange>,
    /**
     * Automatically resizes one or more dimensions based on the contents of the cells in that dimension.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionRange>,
}

/// A banded (alternating colors) range in a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BandedRange {
    /**
     * A banded (alternating colors) range in a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "bandedRangeId"
    )]
    pub banded_range_id: i64,
    /**
     * A banded (alternating colors) range in a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "columnProperties"
    )]
    pub column_properties: Option<BandingProperties>,
    /**
     * A banded (alternating colors) range in a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * A banded (alternating colors) range in a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "rowProperties"
    )]
    pub row_properties: Option<BandingProperties>,
}

/// Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: * header_color and footer_color take priority over band colors. * first_band_color takes priority over second_band_color. * row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BandingProperties {
    /**
     * Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: \* header_color and footer_color take priority over band colors. \* first_band_color takes priority over second_band_color. \* row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "firstBandColor"
    )]
    pub first_band_color: Option<Color>,
    /**
     * Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: \* header_color and footer_color take priority over band colors. \* first_band_color takes priority over second_band_color. \* row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "firstBandColorStyle"
    )]
    pub first_band_color_style: Option<ColorStyle>,
    /**
     * Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: \* header_color and footer_color take priority over band colors. \* first_band_color takes priority over second_band_color. \* row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "footerColor"
    )]
    pub footer_color: Option<Color>,
    /**
     * Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: \* header_color and footer_color take priority over band colors. \* first_band_color takes priority over second_band_color. \* row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "footerColorStyle"
    )]
    pub footer_color_style: Option<ColorStyle>,
    /**
     * Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: \* header_color and footer_color take priority over band colors. \* first_band_color takes priority over second_band_color. \* row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "headerColor"
    )]
    pub header_color: Option<Color>,
    /**
     * Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: \* header_color and footer_color take priority over band colors. \* first_band_color takes priority over second_band_color. \* row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "headerColorStyle"
    )]
    pub header_color_style: Option<ColorStyle>,
    /**
     * Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: \* header_color and footer_color take priority over band colors. \* first_band_color takes priority over second_band_color. \* row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "secondBandColor"
    )]
    pub second_band_color: Option<Color>,
    /**
     * Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: \* header_color and footer_color take priority over band colors. \* first_band_color takes priority over second_band_color. \* row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "secondBandColorStyle"
    )]
    pub second_band_color_style: Option<ColorStyle>,
}

/**
* The comparison type of key value with baseline value.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ComparisonType {
    #[serde(rename = "ABSOLUTE_DIFFERENCE")]
    AbsoluteDifference,
    #[serde(rename = "COMPARISON_TYPE_UNDEFINED")]
    ComparisonTypeUndefined,
    #[serde(rename = "PERCENTAGE_DIFFERENCE")]
    PercentageDifference,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ComparisonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonType::AbsoluteDifference => "ABSOLUTE_DIFFERENCE",
            ComparisonType::ComparisonTypeUndefined => "COMPARISON_TYPE_UNDEFINED",
            ComparisonType::PercentageDifference => "PERCENTAGE_DIFFERENCE",
            ComparisonType::Noop => "",
            ComparisonType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ComparisonType {
    fn default() -> ComparisonType {
        ComparisonType::Noop
    }
}
impl ComparisonType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ComparisonType::Noop)
    }
}

/// Formatting options for baseline value.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BaselineValueFormat {
    /**
     * Formatting options for baseline value.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "comparisonType"
    )]
    pub comparison_type: Option<ComparisonType>,
    /**
     * Formatting options for baseline value.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Formatting options for baseline value.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "negativeColor"
    )]
    pub negative_color: Option<Color>,
    /**
     * Formatting options for baseline value.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "negativeColorStyle"
    )]
    pub negative_color_style: Option<ColorStyle>,
    /**
     * Formatting options for baseline value.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<TextPosition>,
    /**
     * Formatting options for baseline value.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "positiveColor"
    )]
    pub positive_color: Option<Color>,
    /**
     * Formatting options for baseline value.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "positiveColorStyle"
    )]
    pub positive_color_style: Option<ColorStyle>,
    /**
     * Formatting options for baseline value.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "textFormat"
    )]
    pub text_format: Option<TextFormat>,
}

/**
* The position of this axis.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Position {
    #[serde(rename = "BASIC_CHART_AXIS_POSITION_UNSPECIFIED")]
    BasicChartAxisPositionUnspecified,
    #[serde(rename = "BOTTOM_AXIS")]
    BottomAxis,
    #[serde(rename = "LEFT_AXIS")]
    LeftAxis,
    #[serde(rename = "RIGHT_AXIS")]
    RightAxis,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::BasicChartAxisPositionUnspecified => "BASIC_CHART_AXIS_POSITION_UNSPECIFIED",
            Position::BottomAxis => "BOTTOM_AXIS",
            Position::LeftAxis => "LEFT_AXIS",
            Position::RightAxis => "RIGHT_AXIS",
            Position::Noop => "",
            Position::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Position {
    fn default() -> Position {
        Position::Noop
    }
}
impl Position {
    pub fn is_noop(&self) -> bool {
        matches!(self, Position::Noop)
    }
}

/// An axis of the chart. A chart may not have more than one axis per axis position.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BasicChartAxis {
    /**
     * An axis of the chart. A chart may not have more than one axis per axis position.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormat>,
    /**
     * An axis of the chart. A chart may not have more than one axis per axis position.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /**
     * An axis of the chart. A chart may not have more than one axis per axis position.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * An axis of the chart. A chart may not have more than one axis per axis position.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "titleTextPosition"
    )]
    pub title_text_position: Option<TextPosition>,
    /**
     * An axis of the chart. A chart may not have more than one axis per axis position.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "viewWindowOptions"
    )]
    pub view_window_options: Option<ChartAxisViewWindowOptions>,
}

/// The domain of a chart. For example, if charting stock prices over time, this would be the date.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BasicChartDomain {
    /**
     * The domain of a chart. For example, if charting stock prices over time, this would be the date.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<ChartData>,
    /**
     * The domain of a chart. For example, if charting stock prices over time, this would be the date.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub reversed: bool,
}

/**
* The type of this series. Valid only if the chartType is COMBO. Different types will change the way the series is visualized. Only LINE, AREA, and COLUMN are supported.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Type {
    #[serde(rename = "AREA")]
    Area,
    #[serde(rename = "BAR")]
    Bar,
    #[serde(rename = "BASIC_CHART_TYPE_UNSPECIFIED")]
    BasicChartTypeUnspecified,
    #[serde(rename = "COLUMN")]
    Column,
    #[serde(rename = "COMBO")]
    Combo,
    #[serde(rename = "LINE")]
    Line,
    #[serde(rename = "SCATTER")]
    Scatter,
    #[serde(rename = "STEPPED_AREA")]
    SteppedArea,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Area => "AREA",
            Type::Bar => "BAR",
            Type::BasicChartTypeUnspecified => "BASIC_CHART_TYPE_UNSPECIFIED",
            Type::Column => "COLUMN",
            Type::Combo => "COMBO",
            Type::Line => "LINE",
            Type::Scatter => "SCATTER",
            Type::SteppedArea => "STEPPED_AREA",
            Type::Noop => "",
            Type::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::Noop
    }
}
impl Type {
    pub fn is_noop(&self) -> bool {
        matches!(self, Type::Noop)
    }
}

/// A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BasicChartSeries {
    /**
     * A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /**
     * A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "colorStyle"
    )]
    pub color_style: Option<ColorStyle>,
    /**
     * A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataLabel")]
    pub data_label: Option<DataLabel>,
    /**
     * A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lineStyle")]
    pub line_style: Option<LineStyle>,
    /**
     * A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pointStyle"
    )]
    pub point_style: Option<PointStyle>,
    /**
     * A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<ChartData>,
    /**
     * A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "styleOverrides"
    )]
    pub style_overrides: Vec<BasicSeriesDataPointStyleOverride>,
    /**
     * A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "targetAxis"
    )]
    pub target_axis: Option<Position>,
    /**
     * A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
}

/**
* The behavior of tooltips and data highlighting when hovering on data and chart area.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CompareMode {
    #[serde(rename = "BASIC_CHART_COMPARE_MODE_UNSPECIFIED")]
    BasicChartCompareModeUnspecified,
    #[serde(rename = "CATEGORY")]
    Category,
    #[serde(rename = "DATUM")]
    Datum,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CompareMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompareMode::BasicChartCompareModeUnspecified => "BASIC_CHART_COMPARE_MODE_UNSPECIFIED",
            CompareMode::Category => "CATEGORY",
            CompareMode::Datum => "DATUM",
            CompareMode::Noop => "",
            CompareMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CompareMode {
    fn default() -> CompareMode {
        CompareMode::Noop
    }
}
impl CompareMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, CompareMode::Noop)
    }
}

/**
* The position of the chart legend.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LegendPosition {
    #[serde(rename = "BASIC_CHART_LEGEND_POSITION_UNSPECIFIED")]
    BasicChartLegendPositionUnspecified,
    #[serde(rename = "BOTTOM_LEGEND")]
    BottomLegend,
    #[serde(rename = "LEFT_LEGEND")]
    LeftLegend,
    #[serde(rename = "NO_LEGEND")]
    NoLegend,
    #[serde(rename = "RIGHT_LEGEND")]
    RightLegend,
    #[serde(rename = "TOP_LEGEND")]
    TopLegend,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LegendPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LegendPosition::BasicChartLegendPositionUnspecified => {
                "BASIC_CHART_LEGEND_POSITION_UNSPECIFIED"
            }
            LegendPosition::BottomLegend => "BOTTOM_LEGEND",
            LegendPosition::LeftLegend => "LEFT_LEGEND",
            LegendPosition::NoLegend => "NO_LEGEND",
            LegendPosition::RightLegend => "RIGHT_LEGEND",
            LegendPosition::TopLegend => "TOP_LEGEND",
            LegendPosition::Noop => "",
            LegendPosition::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LegendPosition {
    fn default() -> LegendPosition {
        LegendPosition::Noop
    }
}
impl LegendPosition {
    pub fn is_noop(&self) -> bool {
        matches!(self, LegendPosition::Noop)
    }
}

/**
* The stacked type for charts that support vertical stacking. Applies to Area, Bar, Column, Combo, and Stepped Area charts.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum StackedType {
    #[serde(rename = "BASIC_CHART_STACKED_TYPE_UNSPECIFIED")]
    BasicChartStackedTypeUnspecified,
    #[serde(rename = "NOT_STACKED")]
    NotStacked,
    #[serde(rename = "PERCENT_STACKED")]
    PercentStacked,
    #[serde(rename = "STACKED")]
    Stacked,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for StackedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackedType::BasicChartStackedTypeUnspecified => "BASIC_CHART_STACKED_TYPE_UNSPECIFIED",
            StackedType::NotStacked => "NOT_STACKED",
            StackedType::PercentStacked => "PERCENT_STACKED",
            StackedType::Stacked => "STACKED",
            StackedType::Noop => "",
            StackedType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for StackedType {
    fn default() -> StackedType {
        StackedType::Noop
    }
}
impl StackedType {
    pub fn is_noop(&self) -> bool {
        matches!(self, StackedType::Noop)
    }
}

/// The specification for a basic chart. See BasicChartType for the list of charts this supports.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BasicChartSpec {
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub axis: Vec<BasicChartAxis>,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "chartType")]
    pub chart_type: Option<Type>,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "compareMode"
    )]
    pub compare_mode: Option<CompareMode>,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub domains: Vec<BasicChartDomain>,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "headerCount"
    )]
    pub header_count: i64,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "interpolateNulls"
    )]
    pub interpolate_nulls: bool,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "legendPosition"
    )]
    pub legend_position: Option<LegendPosition>,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "lineSmoothing"
    )]
    pub line_smoothing: bool,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub series: Vec<BasicChartSeries>,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stackedType"
    )]
    pub stacked_type: Option<StackedType>,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "threeDimensional"
    )]
    pub three_dimensional: bool,
    /**
     * The specification for a basic chart. See BasicChartType for the list of charts this supports.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalDataLabel"
    )]
    pub total_data_label: Option<DataLabel>,
}

/// The default filter associated with a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BasicFilter {
    /**
     * The default filter associated with a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<FilterCriteria>,
    /**
     * The default filter associated with a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "filterSpecs"
    )]
    pub filter_specs: Vec<FilterSpec>,
    /**
     * The default filter associated with a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * The default filter associated with a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "sortSpecs"
    )]
    pub sort_specs: Vec<SortSpec>,
}

/// Style override settings for a single series data point.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BasicSeriesDataPointStyleOverride {
    /**
     * Style override settings for a single series data point.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /**
     * Style override settings for a single series data point.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "colorStyle"
    )]
    pub color_style: Option<ColorStyle>,
    /**
     * Style override settings for a single series data point.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub index: i64,
    /**
     * Style override settings for a single series data point.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pointStyle"
    )]
    pub point_style: Option<PointStyle>,
}

/// The request for clearing more than one range selected by a DataFilter in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchClearValuesByDataFilterRequest {
    /**
     * The request for clearing more than one range selected by a DataFilter in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dataFilters"
    )]
    pub data_filters: Vec<DataFilter>,
}

/// The response when clearing a range of values selected with DataFilters in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchClearValuesByDataFilterResponse {
    /**
     * The response when clearing a range of values selected with DataFilters in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "clearedRanges"
    )]
    pub cleared_ranges: Vec<String>,
    /**
     * The response when clearing a range of values selected with DataFilters in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
}

/// The request for clearing more than one range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchClearValuesRequest {
    /**
     * The request for clearing more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ranges: Vec<String>,
}

/// The response when clearing a range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchClearValuesResponse {
    /**
     * The response when clearing a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "clearedRanges"
    )]
    pub cleared_ranges: Vec<String>,
    /**
     * The response when clearing a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
}

/**
* How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DateTimeRenderOption {
    #[serde(rename = "FORMATTED_STRING")]
    FormattedString,
    #[serde(rename = "SERIAL_NUMBER")]
    SerialNumber,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DateTimeRenderOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTimeRenderOption::FormattedString => "FORMATTED_STRING",
            DateTimeRenderOption::SerialNumber => "SERIAL_NUMBER",
            DateTimeRenderOption::Noop => "",
            DateTimeRenderOption::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DateTimeRenderOption {
    fn default() -> DateTimeRenderOption {
        DateTimeRenderOption::Noop
    }
}
impl DateTimeRenderOption {
    pub fn is_noop(&self) -> bool {
        matches!(self, DateTimeRenderOption::Noop)
    }
}

/**
* How values should be represented in the output. The default render option is FORMATTED_VALUE.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ValueRenderOption {
    #[serde(rename = "FORMATTED_VALUE")]
    FormattedValue,
    #[serde(rename = "FORMULA")]
    Formula,
    #[serde(rename = "UNFORMATTED_VALUE")]
    UnformattedValue,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ValueRenderOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueRenderOption::FormattedValue => "FORMATTED_VALUE",
            ValueRenderOption::Formula => "FORMULA",
            ValueRenderOption::UnformattedValue => "UNFORMATTED_VALUE",
            ValueRenderOption::Noop => "",
            ValueRenderOption::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ValueRenderOption {
    fn default() -> ValueRenderOption {
        ValueRenderOption::Noop
    }
}
impl ValueRenderOption {
    pub fn is_noop(&self) -> bool {
        matches!(self, ValueRenderOption::Noop)
    }
}

/// The request for retrieving a range of values in a spreadsheet selected by a set of DataFilters.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchGetValuesByDataFilterRequest {
    /**
     * The request for retrieving a range of values in a spreadsheet selected by a set of DataFilters.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dataFilters"
    )]
    pub data_filters: Vec<DataFilter>,
    /**
     * The request for retrieving a range of values in a spreadsheet selected by a set of DataFilters.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dateTimeRenderOption"
    )]
    pub date_time_render_option: Option<DateTimeRenderOption>,
    /**
     * The request for retrieving a range of values in a spreadsheet selected by a set of DataFilters.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "majorDimension"
    )]
    pub major_dimension: Option<Dimension>,
    /**
     * The request for retrieving a range of values in a spreadsheet selected by a set of DataFilters.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "valueRenderOption"
    )]
    pub value_render_option: Option<ValueRenderOption>,
}

/// The response when retrieving more than one range of values in a spreadsheet selected by DataFilters.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchGetValuesByDataFilterResponse {
    /**
     * The response when retrieving more than one range of values in a spreadsheet selected by DataFilters.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
    /**
     * The response when retrieving more than one range of values in a spreadsheet selected by DataFilters.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "valueRanges"
    )]
    pub value_ranges: Vec<MatchedValueRange>,
}

/// The response when retrieving more than one range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchGetValuesResponse {
    /**
     * The response when retrieving more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
    /**
     * The response when retrieving more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "valueRanges"
    )]
    pub value_ranges: Vec<ValueRange>,
}

/// The request for updating any aspect of a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchUpdateSpreadsheetRequest {
    /**
     * The request for updating any aspect of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "includeSpreadsheetInResponse"
    )]
    pub include_spreadsheet_in_response: Option<bool>,
    /**
     * The request for updating any aspect of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub requests: Vec<Request>,
    /**
     * The request for updating any aspect of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseIncludeGridData"
    )]
    pub response_include_grid_data: Option<bool>,
    /**
     * The request for updating any aspect of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "responseRanges"
    )]
    pub response_ranges: Vec<String>,
}

/// The reply for batch updating a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchUpdateSpreadsheetResponse {
    /**
     * The reply for batch updating a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub replies: Vec<Response>,
    /**
     * The reply for batch updating a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
    /**
     * The reply for batch updating a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updatedSpreadsheet"
    )]
    pub updated_spreadsheet: Option<Spreadsheet>,
}

/**
* How the input data should be interpreted.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ValueInputOption {
    #[serde(rename = "INPUT_VALUE_OPTION_UNSPECIFIED")]
    InputValueOptionUnspecified,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "USER_ENTERED")]
    UserEntered,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ValueInputOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueInputOption::InputValueOptionUnspecified => "INPUT_VALUE_OPTION_UNSPECIFIED",
            ValueInputOption::Raw => "RAW",
            ValueInputOption::UserEntered => "USER_ENTERED",
            ValueInputOption::Noop => "",
            ValueInputOption::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ValueInputOption {
    fn default() -> ValueInputOption {
        ValueInputOption::Noop
    }
}
impl ValueInputOption {
    pub fn is_noop(&self) -> bool {
        matches!(self, ValueInputOption::Noop)
    }
}

/// The request for updating more than one range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchUpdateValuesByDataFilterRequest {
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub data: Vec<DataFilterValueRange>,
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "includeValuesInResponse"
    )]
    pub include_values_in_response: Option<bool>,
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseDateTimeRenderOption"
    )]
    pub response_date_time_render_option: Option<DateTimeRenderOption>,
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseValueRenderOption"
    )]
    pub response_value_render_option: Option<ValueRenderOption>,
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "valueInputOption"
    )]
    pub value_input_option: Option<ValueInputOption>,
}

/// The response when updating a range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchUpdateValuesByDataFilterResponse {
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub responses: Vec<UpdateValuesByDataFilterResponse>,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalUpdatedCells"
    )]
    pub total_updated_cells: i64,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalUpdatedColumns"
    )]
    pub total_updated_columns: i64,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalUpdatedRows"
    )]
    pub total_updated_rows: i64,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalUpdatedSheets"
    )]
    pub total_updated_sheets: i64,
}

/// The request for updating more than one range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchUpdateValuesRequest {
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub data: Vec<ValueRange>,
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "includeValuesInResponse"
    )]
    pub include_values_in_response: Option<bool>,
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseDateTimeRenderOption"
    )]
    pub response_date_time_render_option: Option<DateTimeRenderOption>,
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseValueRenderOption"
    )]
    pub response_value_render_option: Option<ValueRenderOption>,
    /**
     * The request for updating more than one range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "valueInputOption"
    )]
    pub value_input_option: Option<ValueInputOption>,
}

/// The response when updating a range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BatchUpdateValuesResponse {
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub responses: Vec<UpdateValuesResponse>,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalUpdatedCells"
    )]
    pub total_updated_cells: i64,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalUpdatedColumns"
    )]
    pub total_updated_columns: i64,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalUpdatedRows"
    )]
    pub total_updated_rows: i64,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "totalUpdatedSheets"
    )]
    pub total_updated_sheets: i64,
}

/// The specification of a BigQuery data source that's connected to a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BigQueryDataSourceSpec {
    /**
     * The specification of a BigQuery data source that's connected to a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "projectId"
    )]
    pub project_id: String,
    /**
     * The specification of a BigQuery data source that's connected to a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "querySpec")]
    pub query_spec: Option<BigQuerySpec>,
    /**
     * The specification of a BigQuery data source that's connected to a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tableSpec")]
    pub table_spec: Option<BigQueryTableSpec>,
}

/// Specifies a custom BigQuery query.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BigQuerySpec {
    /**
     * Specifies a custom BigQuery query.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "rawQuery"
    )]
    pub raw_query: String,
}

/// Specifies a BigQuery table definition. Only [native tables](https://cloud.google.com/bigquery/docs/tables-intro) is allowed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BigQueryTableSpec {
    /**
     * Specifies a BigQuery table definition. Only [native tables](https://cloud.google.com/bigquery/docs/tables-intro) is allowed.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "datasetId"
    )]
    pub dataset_id: String,
    /**
     * Specifies a BigQuery table definition. Only [native tables](https://cloud.google.com/bigquery/docs/tables-intro) is allowed.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "tableId"
    )]
    pub table_id: String,
    /**
     * Specifies a BigQuery table definition. Only [native tables](https://cloud.google.com/bigquery/docs/tables-intro) is allowed.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "tableProjectId"
    )]
    pub table_project_id: String,
}

/**
* The type of condition.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum BooleanConditionType {
    #[serde(rename = "BLANK")]
    Blank,
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "CONDITION_TYPE_UNSPECIFIED")]
    ConditionTypeUnspecified,
    #[serde(rename = "CUSTOM_FORMULA")]
    CustomFormula,
    #[serde(rename = "DATE_AFTER")]
    DateAfter,
    #[serde(rename = "DATE_BEFORE")]
    DateBefore,
    #[serde(rename = "DATE_BETWEEN")]
    DateBetween,
    #[serde(rename = "DATE_EQ")]
    DateEq,
    #[serde(rename = "DATE_IS_VALID")]
    DateIsValid,
    #[serde(rename = "DATE_NOT_BETWEEN")]
    DateNotBetween,
    #[serde(rename = "DATE_NOT_EQ")]
    DateNotEq,
    #[serde(rename = "DATE_ON_OR_AFTER")]
    DateOnOrAfter,
    #[serde(rename = "DATE_ON_OR_BEFORE")]
    DateOnOrBefore,
    #[serde(rename = "NOT_BLANK")]
    NotBlank,
    #[serde(rename = "NUMBER_BETWEEN")]
    NumberBetween,
    #[serde(rename = "NUMBER_EQ")]
    NumberEq,
    #[serde(rename = "NUMBER_GREATER")]
    NumberGreater,
    #[serde(rename = "NUMBER_GREATER_THAN_EQ")]
    NumberGreaterThanEq,
    #[serde(rename = "NUMBER_LESS")]
    NumberLess,
    #[serde(rename = "NUMBER_LESS_THAN_EQ")]
    NumberLessThanEq,
    #[serde(rename = "NUMBER_NOT_BETWEEN")]
    NumberNotBetween,
    #[serde(rename = "NUMBER_NOT_EQ")]
    NumberNotEq,
    #[serde(rename = "ONE_OF_LIST")]
    OneOfList,
    #[serde(rename = "ONE_OF_RANGE")]
    OneOfRange,
    #[serde(rename = "TEXT_CONTAINS")]
    TextContains,
    #[serde(rename = "TEXT_ENDS_WITH")]
    TextEndsWith,
    #[serde(rename = "TEXT_EQ")]
    TextEq,
    #[serde(rename = "TEXT_IS_EMAIL")]
    TextIsEmail,
    #[serde(rename = "TEXT_IS_URL")]
    TextIsUrl,
    #[serde(rename = "TEXT_NOT_CONTAINS")]
    TextNotContains,
    #[serde(rename = "TEXT_NOT_EQ")]
    TextNotEq,
    #[serde(rename = "TEXT_STARTS_WITH")]
    TextStartsWith,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for BooleanConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BooleanConditionType::Blank => "BLANK",
            BooleanConditionType::Boolean => "BOOLEAN",
            BooleanConditionType::ConditionTypeUnspecified => "CONDITION_TYPE_UNSPECIFIED",
            BooleanConditionType::CustomFormula => "CUSTOM_FORMULA",
            BooleanConditionType::DateAfter => "DATE_AFTER",
            BooleanConditionType::DateBefore => "DATE_BEFORE",
            BooleanConditionType::DateBetween => "DATE_BETWEEN",
            BooleanConditionType::DateEq => "DATE_EQ",
            BooleanConditionType::DateIsValid => "DATE_IS_VALID",
            BooleanConditionType::DateNotBetween => "DATE_NOT_BETWEEN",
            BooleanConditionType::DateNotEq => "DATE_NOT_EQ",
            BooleanConditionType::DateOnOrAfter => "DATE_ON_OR_AFTER",
            BooleanConditionType::DateOnOrBefore => "DATE_ON_OR_BEFORE",
            BooleanConditionType::NotBlank => "NOT_BLANK",
            BooleanConditionType::NumberBetween => "NUMBER_BETWEEN",
            BooleanConditionType::NumberEq => "NUMBER_EQ",
            BooleanConditionType::NumberGreater => "NUMBER_GREATER",
            BooleanConditionType::NumberGreaterThanEq => "NUMBER_GREATER_THAN_EQ",
            BooleanConditionType::NumberLess => "NUMBER_LESS",
            BooleanConditionType::NumberLessThanEq => "NUMBER_LESS_THAN_EQ",
            BooleanConditionType::NumberNotBetween => "NUMBER_NOT_BETWEEN",
            BooleanConditionType::NumberNotEq => "NUMBER_NOT_EQ",
            BooleanConditionType::OneOfList => "ONE_OF_LIST",
            BooleanConditionType::OneOfRange => "ONE_OF_RANGE",
            BooleanConditionType::TextContains => "TEXT_CONTAINS",
            BooleanConditionType::TextEndsWith => "TEXT_ENDS_WITH",
            BooleanConditionType::TextEq => "TEXT_EQ",
            BooleanConditionType::TextIsEmail => "TEXT_IS_EMAIL",
            BooleanConditionType::TextIsUrl => "TEXT_IS_URL",
            BooleanConditionType::TextNotContains => "TEXT_NOT_CONTAINS",
            BooleanConditionType::TextNotEq => "TEXT_NOT_EQ",
            BooleanConditionType::TextStartsWith => "TEXT_STARTS_WITH",
            BooleanConditionType::Noop => "",
            BooleanConditionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for BooleanConditionType {
    fn default() -> BooleanConditionType {
        BooleanConditionType::Noop
    }
}
impl BooleanConditionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, BooleanConditionType::Noop)
    }
}

/// A condition that can evaluate to true or false. BooleanConditions are used by conditional formatting, data validation, and the criteria in filters.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BooleanCondition {
    /**
     * A condition that can evaluate to true or false. BooleanConditions are used by conditional formatting, data validation, and the criteria in filters.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<BooleanConditionType>,
    /**
     * A condition that can evaluate to true or false. BooleanConditions are used by conditional formatting, data validation, and the criteria in filters.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub values: Vec<ConditionValue>,
}

/// A rule that may or may not match, depending on the condition.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BooleanRule {
    /**
     * A rule that may or may not match, depending on the condition.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<BooleanCondition>,
    /**
     * A rule that may or may not match, depending on the condition.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<CellFormat>,
}

/**
* The style of the border.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Style {
    #[serde(rename = "DASHED")]
    Dashed,
    #[serde(rename = "DOTTED")]
    Dotted,
    #[serde(rename = "DOUBLE")]
    Double,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SOLID")]
    Solid,
    #[serde(rename = "SOLID_MEDIUM")]
    SolidMedium,
    #[serde(rename = "SOLID_THICK")]
    SolidThick,
    #[serde(rename = "STYLE_UNSPECIFIED")]
    StyleUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Dashed => "DASHED",
            Style::Dotted => "DOTTED",
            Style::Double => "DOUBLE",
            Style::None => "NONE",
            Style::Solid => "SOLID",
            Style::SolidMedium => "SOLID_MEDIUM",
            Style::SolidThick => "SOLID_THICK",
            Style::StyleUnspecified => "STYLE_UNSPECIFIED",
            Style::Noop => "",
            Style::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Style {
    fn default() -> Style {
        Style::Noop
    }
}
impl Style {
    pub fn is_noop(&self) -> bool {
        matches!(self, Style::Noop)
    }
}

/// A border along a cell.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Border {
    /**
     * A border along a cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /**
     * A border along a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "colorStyle"
    )]
    pub color_style: Option<ColorStyle>,
    /**
     * A border along a cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    /**
     * A border along a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub width: i64,
}

/// The borders of the cell.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Borders {
    /**
     * The borders of the cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom: Option<Border>,
    /**
     * The borders of the cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<Border>,
    /**
     * The borders of the cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<Border>,
    /**
     * The borders of the cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<Border>,
}

/**
* Where the legend of the chart should be drawn.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum BubbleChartSpecLegendPosition {
    #[serde(rename = "BOTTOM_LEGEND")]
    BottomLegend,
    #[serde(rename = "BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED")]
    BubbleChartLegendPositionUnspecified,
    #[serde(rename = "INSIDE_LEGEND")]
    InsideLegend,
    #[serde(rename = "LEFT_LEGEND")]
    LeftLegend,
    #[serde(rename = "NO_LEGEND")]
    NoLegend,
    #[serde(rename = "RIGHT_LEGEND")]
    RightLegend,
    #[serde(rename = "TOP_LEGEND")]
    TopLegend,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for BubbleChartSpecLegendPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BubbleChartSpecLegendPosition::BottomLegend => "BOTTOM_LEGEND",
            BubbleChartSpecLegendPosition::BubbleChartLegendPositionUnspecified => {
                "BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED"
            }
            BubbleChartSpecLegendPosition::InsideLegend => "INSIDE_LEGEND",
            BubbleChartSpecLegendPosition::LeftLegend => "LEFT_LEGEND",
            BubbleChartSpecLegendPosition::NoLegend => "NO_LEGEND",
            BubbleChartSpecLegendPosition::RightLegend => "RIGHT_LEGEND",
            BubbleChartSpecLegendPosition::TopLegend => "TOP_LEGEND",
            BubbleChartSpecLegendPosition::Noop => "",
            BubbleChartSpecLegendPosition::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for BubbleChartSpecLegendPosition {
    fn default() -> BubbleChartSpecLegendPosition {
        BubbleChartSpecLegendPosition::Noop
    }
}
impl BubbleChartSpecLegendPosition {
    pub fn is_noop(&self) -> bool {
        matches!(self, BubbleChartSpecLegendPosition::Noop)
    }
}

/// A bubble chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct BubbleChartSpec {
    /**
     * A bubble chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bubbleBorderColor"
    )]
    pub bubble_border_color: Option<Color>,
    /**
     * A bubble chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bubbleBorderColorStyle"
    )]
    pub bubble_border_color_style: Option<ColorStyle>,
    /**
     * A bubble chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bubbleLabels"
    )]
    pub bubble_labels: Option<ChartData>,
    /**
     * A bubble chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "bubbleMaxRadiusSize"
    )]
    pub bubble_max_radius_size: i64,
    /**
     * A bubble chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "bubbleMinRadiusSize"
    )]
    pub bubble_min_radius_size: i64,
    /**
     * A bubble chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "bubbleOpacity"
    )]
    pub bubble_opacity: f64,
    /**
     * A bubble chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bubbleSizes"
    )]
    pub bubble_sizes: Option<ChartData>,
    /**
     * A bubble chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bubbleTextStyle"
    )]
    pub bubble_text_style: Option<TextFormat>,
    /**
     * A bubble chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<ChartData>,
    /**
     * A bubble chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupIds")]
    pub group_ids: Option<ChartData>,
    /**
     * A bubble chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "legendPosition"
    )]
    pub legend_position: Option<BubbleChartSpecLegendPosition>,
    /**
     * A bubble chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<ChartData>,
}

/// A candlestick chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CandlestickChartSpec {
    /**
     * A candlestick chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub data: Vec<CandlestickData>,
    /**
     * A candlestick chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<CandlestickDomain>,
}

/// The Candlestick chart data, each containing the low, open, close, and high values for a series.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CandlestickData {
    /**
     * The Candlestick chart data, each containing the low, open, close, and high values for a series.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "closeSeries"
    )]
    pub close_series: Option<CandlestickSeries>,
    /**
     * The Candlestick chart data, each containing the low, open, close, and high values for a series.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "highSeries"
    )]
    pub high_series: Option<CandlestickSeries>,
    /**
     * The Candlestick chart data, each containing the low, open, close, and high values for a series.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lowSeries")]
    pub low_series: Option<CandlestickSeries>,
    /**
     * The Candlestick chart data, each containing the low, open, close, and high values for a series.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "openSeries"
    )]
    pub open_series: Option<CandlestickSeries>,
}

/// The domain of a CandlestickChart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CandlestickDomain {
    /**
     * The domain of a CandlestickChart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ChartData>,
    /**
     * The domain of a CandlestickChart.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub reversed: bool,
}

/// The series of a CandlestickData.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CandlestickSeries {
    /**
     * The series of a CandlestickData.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ChartData>,
}

/// Data about a specific cell.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CellData {
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceFormula"
    )]
    pub data_source_formula: Option<DataSourceFormula>,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceTable"
    )]
    pub data_source_table: Option<DataSourceTable>,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataValidation"
    )]
    pub data_validation: Option<DataValidationRule>,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "effectiveFormat"
    )]
    pub effective_format: Option<CellFormat>,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "effectiveValue"
    )]
    pub effective_value: Option<ExtendedValue>,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "formattedValue"
    )]
    pub formatted_value: String,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hyperlink: String,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pivotTable"
    )]
    pub pivot_table: Option<PivotTable>,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "textFormatRuns"
    )]
    pub text_format_runs: Vec<TextFormatRun>,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userEnteredFormat"
    )]
    pub user_entered_format: Option<CellFormat>,
    /**
     * Data about a specific cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userEnteredValue"
    )]
    pub user_entered_value: Option<ExtendedValue>,
}

/**
* The horizontal alignment of the value in the cell.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum HorizontalAlignment {
    #[serde(rename = "CENTER")]
    Center,
    #[serde(rename = "HORIZONTAL_ALIGN_UNSPECIFIED")]
    HorizontalAlignUnspecified,
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for HorizontalAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HorizontalAlignment::Center => "CENTER",
            HorizontalAlignment::HorizontalAlignUnspecified => "HORIZONTAL_ALIGN_UNSPECIFIED",
            HorizontalAlignment::Left => "LEFT",
            HorizontalAlignment::Right => "RIGHT",
            HorizontalAlignment::Noop => "",
            HorizontalAlignment::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for HorizontalAlignment {
    fn default() -> HorizontalAlignment {
        HorizontalAlignment::Noop
    }
}
impl HorizontalAlignment {
    pub fn is_noop(&self) -> bool {
        matches!(self, HorizontalAlignment::Noop)
    }
}

/**
* How a hyperlink, if it exists, should be displayed in the cell.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum HyperlinkDisplayType {
    #[serde(rename = "HYPERLINK_DISPLAY_TYPE_UNSPECIFIED")]
    HyperlinkDisplayTypeUnspecified,
    #[serde(rename = "LINKED")]
    Linked,
    #[serde(rename = "PLAIN_TEXT")]
    PlainText,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for HyperlinkDisplayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HyperlinkDisplayType::HyperlinkDisplayTypeUnspecified => {
                "HYPERLINK_DISPLAY_TYPE_UNSPECIFIED"
            }
            HyperlinkDisplayType::Linked => "LINKED",
            HyperlinkDisplayType::PlainText => "PLAIN_TEXT",
            HyperlinkDisplayType::Noop => "",
            HyperlinkDisplayType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for HyperlinkDisplayType {
    fn default() -> HyperlinkDisplayType {
        HyperlinkDisplayType::Noop
    }
}
impl HyperlinkDisplayType {
    pub fn is_noop(&self) -> bool {
        matches!(self, HyperlinkDisplayType::Noop)
    }
}

/**
* The direction of the text in the cell.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum TextDirection {
    #[serde(rename = "LEFT_TO_RIGHT")]
    LeftToRight,
    #[serde(rename = "RIGHT_TO_LEFT")]
    RightToLeft,
    #[serde(rename = "TEXT_DIRECTION_UNSPECIFIED")]
    TextDirectionUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for TextDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextDirection::LeftToRight => "LEFT_TO_RIGHT",
            TextDirection::RightToLeft => "RIGHT_TO_LEFT",
            TextDirection::TextDirectionUnspecified => "TEXT_DIRECTION_UNSPECIFIED",
            TextDirection::Noop => "",
            TextDirection::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for TextDirection {
    fn default() -> TextDirection {
        TextDirection::Noop
    }
}
impl TextDirection {
    pub fn is_noop(&self) -> bool {
        matches!(self, TextDirection::Noop)
    }
}

/**
* The vertical alignment of the value in the cell.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum VerticalAlignment {
    #[serde(rename = "BOTTOM")]
    Bottom,
    #[serde(rename = "MIDDLE")]
    Middle,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "VERTICAL_ALIGN_UNSPECIFIED")]
    VerticalAlignUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for VerticalAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerticalAlignment::Bottom => "BOTTOM",
            VerticalAlignment::Middle => "MIDDLE",
            VerticalAlignment::Top => "TOP",
            VerticalAlignment::VerticalAlignUnspecified => "VERTICAL_ALIGN_UNSPECIFIED",
            VerticalAlignment::Noop => "",
            VerticalAlignment::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for VerticalAlignment {
    fn default() -> VerticalAlignment {
        VerticalAlignment::Noop
    }
}
impl VerticalAlignment {
    pub fn is_noop(&self) -> bool {
        matches!(self, VerticalAlignment::Noop)
    }
}

/**
* The wrap strategy for the value in the cell.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum WrapStrategy {
    #[serde(rename = "CLIP")]
    Clip,
    #[serde(rename = "LEGACY_WRAP")]
    LegacyWrap,
    #[serde(rename = "OVERFLOW_CELL")]
    OverflowCell,
    #[serde(rename = "WRAP")]
    Wrap,
    #[serde(rename = "WRAP_STRATEGY_UNSPECIFIED")]
    WrapStrategyUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for WrapStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WrapStrategy::Clip => "CLIP",
            WrapStrategy::LegacyWrap => "LEGACY_WRAP",
            WrapStrategy::OverflowCell => "OVERFLOW_CELL",
            WrapStrategy::Wrap => "WRAP",
            WrapStrategy::WrapStrategyUnspecified => "WRAP_STRATEGY_UNSPECIFIED",
            WrapStrategy::Noop => "",
            WrapStrategy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for WrapStrategy {
    fn default() -> WrapStrategy {
        WrapStrategy::Noop
    }
}
impl WrapStrategy {
    pub fn is_noop(&self) -> bool {
        matches!(self, WrapStrategy::Noop)
    }
}

/// The format of a cell.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CellFormat {
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundColor"
    )]
    pub background_color: Option<Color>,
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundColorStyle"
    )]
    pub background_color_style: Option<ColorStyle>,
    /**
     * The format of a cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borders: Option<Borders>,
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "horizontalAlignment"
    )]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "hyperlinkDisplayType"
    )]
    pub hyperlink_display_type: Option<HyperlinkDisplayType>,
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "numberFormat"
    )]
    pub number_format: Option<NumberFormat>,
    /**
     * The format of a cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "textDirection"
    )]
    pub text_direction: Option<TextDirection>,
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "textFormat"
    )]
    pub text_format: Option<TextFormat>,
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "textRotation"
    )]
    pub text_rotation: Option<TextRotation>,
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verticalAlignment"
    )]
    pub vertical_alignment: Option<VerticalAlignment>,
    /**
     * The format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wrapStrategy"
    )]
    pub wrap_strategy: Option<WrapStrategy>,
}

/**
* The view window's mode.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ViewWindowMode {
    #[serde(rename = "DEFAULT_VIEW_WINDOW_MODE")]
    DefaultViewWindowMode,
    #[serde(rename = "EXPLICIT")]
    Explicit,
    #[serde(rename = "PRETTY")]
    Pretty,
    #[serde(rename = "VIEW_WINDOW_MODE_UNSUPPORTED")]
    ViewWindowModeUnsupported,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ViewWindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewWindowMode::DefaultViewWindowMode => "DEFAULT_VIEW_WINDOW_MODE",
            ViewWindowMode::Explicit => "EXPLICIT",
            ViewWindowMode::Pretty => "PRETTY",
            ViewWindowMode::ViewWindowModeUnsupported => "VIEW_WINDOW_MODE_UNSUPPORTED",
            ViewWindowMode::Noop => "",
            ViewWindowMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ViewWindowMode {
    fn default() -> ViewWindowMode {
        ViewWindowMode::Noop
    }
}
impl ViewWindowMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, ViewWindowMode::Noop)
    }
}

/// The options that define a "view window" for a chart (such as the visible values in an axis).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChartAxisViewWindowOptions {
    /**
     * The options that define a "view window" for a chart (such as the visible values in an axis).
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "viewWindowMax"
    )]
    pub view_window_max: f64,
    /**
     * The options that define a "view window" for a chart (such as the visible values in an axis).
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "viewWindowMin"
    )]
    pub view_window_min: f64,
    /**
     * The options that define a "view window" for a chart (such as the visible values in an axis).
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "viewWindowMode"
    )]
    pub view_window_mode: Option<ViewWindowMode>,
}

/// Custom number formatting options for chart attributes.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChartCustomNumberFormatOptions {
    /**
     * Custom number formatting options for chart attributes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub prefix: String,
    /**
     * Custom number formatting options for chart attributes.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub suffix: String,
}

/**
* The aggregation type for the series of a data source chart. Only supported for data source charts.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AggregateType {
    #[serde(rename = "AVERAGE")]
    Average,
    #[serde(rename = "CHART_AGGREGATE_TYPE_UNSPECIFIED")]
    ChartAggregateTypeUnspecified,
    #[serde(rename = "COUNT")]
    Count,
    #[serde(rename = "MAX")]
    Max,
    #[serde(rename = "MEDIAN")]
    Median,
    #[serde(rename = "MIN")]
    Min,
    #[serde(rename = "SUM")]
    Sum,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AggregateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregateType::Average => "AVERAGE",
            AggregateType::ChartAggregateTypeUnspecified => "CHART_AGGREGATE_TYPE_UNSPECIFIED",
            AggregateType::Count => "COUNT",
            AggregateType::Max => "MAX",
            AggregateType::Median => "MEDIAN",
            AggregateType::Min => "MIN",
            AggregateType::Sum => "SUM",
            AggregateType::Noop => "",
            AggregateType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AggregateType {
    fn default() -> AggregateType {
        AggregateType::Noop
    }
}
impl AggregateType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AggregateType::Noop)
    }
}

/// The data included in a domain or series.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChartData {
    /**
     * The data included in a domain or series.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "aggregateType"
    )]
    pub aggregate_type: Option<AggregateType>,
    /**
     * The data included in a domain or series.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "columnReference"
    )]
    pub column_reference: Option<DataSourceColumnReference>,
    /**
     * The data included in a domain or series.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupRule")]
    pub group_rule: Option<ChartGroupRule>,
    /**
     * The data included in a domain or series.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceRange"
    )]
    pub source_range: Option<ChartSourceRange>,
}

/**
* The type of date-time grouping to apply.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ChartDateTimeRuleType {
    #[serde(rename = "CHART_DATE_TIME_RULE_TYPE_UNSPECIFIED")]
    ChartDateTimeRuleTypeUnspecified,
    #[serde(rename = "DAY_MONTH")]
    DayMonth,
    #[serde(rename = "DAY_OF_MONTH")]
    DayOfMonth,
    #[serde(rename = "DAY_OF_WEEK")]
    DayOfWeek,
    #[serde(rename = "DAY_OF_YEAR")]
    DayOfYear,
    #[serde(rename = "HOUR")]
    Hour,
    #[serde(rename = "HOUR_MINUTE")]
    HourMinute,
    #[serde(rename = "HOUR_MINUTE_AMPM")]
    HourMinuteAmpm,
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "MONTH")]
    Month,
    #[serde(rename = "QUARTER")]
    Quarter,
    #[serde(rename = "SECOND")]
    Second,
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "YEAR_MONTH")]
    YearMonth,
    #[serde(rename = "YEAR_MONTH_DAY")]
    YearMonthDay,
    #[serde(rename = "YEAR_QUARTER")]
    YearQuarter,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ChartDateTimeRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartDateTimeRuleType::ChartDateTimeRuleTypeUnspecified => {
                "CHART_DATE_TIME_RULE_TYPE_UNSPECIFIED"
            }
            ChartDateTimeRuleType::DayMonth => "DAY_MONTH",
            ChartDateTimeRuleType::DayOfMonth => "DAY_OF_MONTH",
            ChartDateTimeRuleType::DayOfWeek => "DAY_OF_WEEK",
            ChartDateTimeRuleType::DayOfYear => "DAY_OF_YEAR",
            ChartDateTimeRuleType::Hour => "HOUR",
            ChartDateTimeRuleType::HourMinute => "HOUR_MINUTE",
            ChartDateTimeRuleType::HourMinuteAmpm => "HOUR_MINUTE_AMPM",
            ChartDateTimeRuleType::Minute => "MINUTE",
            ChartDateTimeRuleType::Month => "MONTH",
            ChartDateTimeRuleType::Quarter => "QUARTER",
            ChartDateTimeRuleType::Second => "SECOND",
            ChartDateTimeRuleType::Year => "YEAR",
            ChartDateTimeRuleType::YearMonth => "YEAR_MONTH",
            ChartDateTimeRuleType::YearMonthDay => "YEAR_MONTH_DAY",
            ChartDateTimeRuleType::YearQuarter => "YEAR_QUARTER",
            ChartDateTimeRuleType::Noop => "",
            ChartDateTimeRuleType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ChartDateTimeRuleType {
    fn default() -> ChartDateTimeRuleType {
        ChartDateTimeRuleType::Noop
    }
}
impl ChartDateTimeRuleType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ChartDateTimeRuleType::Noop)
    }
}

/// Allows you to organize the date-time values in a source data column into buckets based on selected parts of their date or time values.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChartDateTimeRule {
    /**
     * Allows you to organize the date-time values in a source data column into buckets based on selected parts of their date or time values.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<ChartDateTimeRuleType>,
}

/// An optional setting on the ChartData of the domain of a data source chart that defines buckets for the values in the domain rather than breaking out each individual value. For example, when plotting a data source chart, you can specify a histogram rule on the domain (it should only contain numeric values), grouping its values into buckets. Any values of a chart series that fall into the same bucket are aggregated based on the aggregate_type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChartGroupRule {
    /**
     * An optional setting on the ChartData of the domain of a data source chart that defines buckets for the values in the domain rather than breaking out each individual value. For example, when plotting a data source chart, you can specify a histogram rule on the domain (it should only contain numeric values), grouping its values into buckets. Any values of a chart series that fall into the same bucket are aggregated based on the aggregate_type.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dateTimeRule"
    )]
    pub date_time_rule: Option<ChartDateTimeRule>,
    /**
     * An optional setting on the ChartData of the domain of a data source chart that defines buckets for the values in the domain rather than breaking out each individual value. For example, when plotting a data source chart, you can specify a histogram rule on the domain (it should only contain numeric values), grouping its values into buckets. Any values of a chart series that fall into the same bucket are aggregated based on the aggregate_type.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "histogramRule"
    )]
    pub histogram_rule: Option<ChartHistogramRule>,
}

/// Allows you to organize numeric values in a source data column into buckets of constant size.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChartHistogramRule {
    /**
     * Allows you to organize numeric values in a source data column into buckets of constant size.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "intervalSize"
    )]
    pub interval_size: f64,
    /**
     * Allows you to organize numeric values in a source data column into buckets of constant size.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "maxValue"
    )]
    pub max_value: f64,
    /**
     * Allows you to organize numeric values in a source data column into buckets of constant size.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "minValue"
    )]
    pub min_value: f64,
}

/// Source ranges for a chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChartSourceRange {
    /**
     * Source ranges for a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub sources: Vec<GridRange>,
}

/**
* Determines how the charts will use hidden rows or columns.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum HiddenDimensionStrategy {
    #[serde(rename = "CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED")]
    ChartHiddenDimensionStrategyUnspecified,
    #[serde(rename = "SHOW_ALL")]
    ShowAll,
    #[serde(rename = "SKIP_HIDDEN_COLUMNS")]
    SkipHiddenColumns,
    #[serde(rename = "SKIP_HIDDEN_ROWS")]
    SkipHiddenRows,
    #[serde(rename = "SKIP_HIDDEN_ROWS_AND_COLUMNS")]
    SkipHiddenRowsAndColumns,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for HiddenDimensionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HiddenDimensionStrategy::ChartHiddenDimensionStrategyUnspecified => {
                "CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED"
            }
            HiddenDimensionStrategy::ShowAll => "SHOW_ALL",
            HiddenDimensionStrategy::SkipHiddenColumns => "SKIP_HIDDEN_COLUMNS",
            HiddenDimensionStrategy::SkipHiddenRows => "SKIP_HIDDEN_ROWS",
            HiddenDimensionStrategy::SkipHiddenRowsAndColumns => "SKIP_HIDDEN_ROWS_AND_COLUMNS",
            HiddenDimensionStrategy::Noop => "",
            HiddenDimensionStrategy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for HiddenDimensionStrategy {
    fn default() -> HiddenDimensionStrategy {
        HiddenDimensionStrategy::Noop
    }
}
impl HiddenDimensionStrategy {
    pub fn is_noop(&self) -> bool {
        matches!(self, HiddenDimensionStrategy::Noop)
    }
}

/// The specifications of a chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChartSpec {
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "altText"
    )]
    pub alt_text: String,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundColor"
    )]
    pub background_color: Option<Color>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundColorStyle"
    )]
    pub background_color_style: Option<ColorStyle>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "basicChart"
    )]
    pub basic_chart: Option<BasicChartSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bubbleChart"
    )]
    pub bubble_chart: Option<BubbleChartSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "candlestickChart"
    )]
    pub candlestick_chart: Option<CandlestickChartSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceChartProperties"
    )]
    pub data_source_chart_properties: Option<DataSourceChartProperties>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "filterSpecs"
    )]
    pub filter_specs: Vec<FilterSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "fontName"
    )]
    pub font_name: String,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "hiddenDimensionStrategy"
    )]
    pub hidden_dimension_strategy: Option<HiddenDimensionStrategy>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "histogramChart"
    )]
    pub histogram_chart: Option<HistogramChartSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub maximized: bool,
    /**
     * The specifications of a chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "orgChart")]
    pub org_chart: Option<OrgChartSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pieChart")]
    pub pie_chart: Option<PieChartSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "scorecardChart"
    )]
    pub scorecard_chart: Option<ScorecardChartSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "sortSpecs"
    )]
    pub sort_specs: Vec<SortSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subtitle: String,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subtitleTextFormat"
    )]
    pub subtitle_text_format: Option<TextFormat>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subtitleTextPosition"
    )]
    pub subtitle_text_position: Option<TextPosition>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "titleTextFormat"
    )]
    pub title_text_format: Option<TextFormat>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "titleTextPosition"
    )]
    pub title_text_position: Option<TextPosition>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "treemapChart"
    )]
    pub treemap_chart: Option<TreemapChartSpec>,
    /**
     * The specifications of a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "waterfallChart"
    )]
    pub waterfall_chart: Option<WaterfallChartSpec>,
}

/// Clears the basic filter, if any exists on the sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ClearBasicFilterRequest {
    /**
     * Clears the basic filter, if any exists on the sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/// The request for clearing a range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ClearValuesRequest {}

/// The response when clearing a range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ClearValuesResponse {
    /**
     * The response when clearing a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "clearedRange"
    )]
    pub cleared_range: String,
    /**
     * The response when clearing a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
}

/// Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Color {
    /**
     * Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor\* fromProto(Color\* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue\* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color\* toProto(UIColor\* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color\* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac \* 255); var green = Math.floor(greenFrac \* 255); var blue = Math.floor(blueFrac \* 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub alpha: f64,
    /**
     * Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor\* fromProto(Color\* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue\* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color\* toProto(UIColor\* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color\* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac \* 255); var green = Math.floor(greenFrac \* 255); var blue = Math.floor(blueFrac \* 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub blue: f64,
    /**
     * Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor\* fromProto(Color\* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue\* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color\* toProto(UIColor\* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color\* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac \* 255); var green = Math.floor(greenFrac \* 255); var blue = Math.floor(blueFrac \* 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub green: f64,
    /**
     * Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor\* fromProto(Color\* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue\* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color\* toProto(UIColor\* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color\* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac \* 255); var green = Math.floor(greenFrac \* 255); var blue = Math.floor(blueFrac \* 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub red: f64,
}

/**
* The type of the spreadsheet theme color.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ColorType {
    #[serde(rename = "ACCENT1")]
    Accent1,
    #[serde(rename = "ACCENT2")]
    Accent2,
    #[serde(rename = "ACCENT3")]
    Accent3,
    #[serde(rename = "ACCENT4")]
    Accent4,
    #[serde(rename = "ACCENT5")]
    Accent5,
    #[serde(rename = "ACCENT6")]
    Accent6,
    #[serde(rename = "BACKGROUND")]
    Background,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "THEME_COLOR_TYPE_UNSPECIFIED")]
    ThemeColorTypeUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorType::Accent1 => "ACCENT1",
            ColorType::Accent2 => "ACCENT2",
            ColorType::Accent3 => "ACCENT3",
            ColorType::Accent4 => "ACCENT4",
            ColorType::Accent5 => "ACCENT5",
            ColorType::Accent6 => "ACCENT6",
            ColorType::Background => "BACKGROUND",
            ColorType::Link => "LINK",
            ColorType::Text => "TEXT",
            ColorType::ThemeColorTypeUnspecified => "THEME_COLOR_TYPE_UNSPECIFIED",
            ColorType::Noop => "",
            ColorType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ColorType {
    fn default() -> ColorType {
        ColorType::Noop
    }
}
impl ColorType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ColorType::Noop)
    }
}

/// A color value.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ColorStyle {
    /**
     * A color value.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rgbColor")]
    pub rgb_color: Option<Color>,
    /**
     * A color value.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "themeColor"
    )]
    pub theme_color: Option<ColorType>,
}

/**
* A relative date (based on the current date). Valid only if the type is DATE_BEFORE, DATE_AFTER, DATE_ON_OR_BEFORE or DATE_ON_OR_AFTER. Relative dates are not supported in data validation. They are supported only in conditional formatting and conditional filters.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum RelativeDate {
    #[serde(rename = "PAST_MONTH")]
    PastMonth,
    #[serde(rename = "PAST_WEEK")]
    PastWeek,
    #[serde(rename = "PAST_YEAR")]
    PastYear,
    #[serde(rename = "RELATIVE_DATE_UNSPECIFIED")]
    RelativeDateUnspecified,
    #[serde(rename = "TODAY")]
    Today,
    #[serde(rename = "TOMORROW")]
    Tomorrow,
    #[serde(rename = "YESTERDAY")]
    Yesterday,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RelativeDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeDate::PastMonth => "PAST_MONTH",
            RelativeDate::PastWeek => "PAST_WEEK",
            RelativeDate::PastYear => "PAST_YEAR",
            RelativeDate::RelativeDateUnspecified => "RELATIVE_DATE_UNSPECIFIED",
            RelativeDate::Today => "TODAY",
            RelativeDate::Tomorrow => "TOMORROW",
            RelativeDate::Yesterday => "YESTERDAY",
            RelativeDate::Noop => "",
            RelativeDate::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RelativeDate {
    fn default() -> RelativeDate {
        RelativeDate::Noop
    }
}
impl RelativeDate {
    pub fn is_noop(&self) -> bool {
        matches!(self, RelativeDate::Noop)
    }
}

/// The value of the condition.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConditionValue {
    /**
     * The value of the condition.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "relativeDate"
    )]
    pub relative_date: Option<RelativeDate>,
    /**
     * The value of the condition.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "userEnteredValue"
    )]
    pub user_entered_value: String,
}

/// A rule describing a conditional format.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ConditionalFormatRule {
    /**
     * A rule describing a conditional format.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "booleanRule"
    )]
    pub boolean_rule: Option<BooleanRule>,
    /**
     * A rule describing a conditional format.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "gradientRule"
    )]
    pub gradient_rule: Option<GradientRule>,
    /**
     * A rule describing a conditional format.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub ranges: Vec<GridRange>,
}

/**
* How that data should be oriented when pasting.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PasteOrientation {
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "TRANSPOSE")]
    Transpose,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PasteOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasteOrientation::Normal => "NORMAL",
            PasteOrientation::Transpose => "TRANSPOSE",
            PasteOrientation::Noop => "",
            PasteOrientation::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PasteOrientation {
    fn default() -> PasteOrientation {
        PasteOrientation::Noop
    }
}
impl PasteOrientation {
    pub fn is_noop(&self) -> bool {
        matches!(self, PasteOrientation::Noop)
    }
}

/**
* What kind of data to paste.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PasteType {
    #[serde(rename = "PASTE_CONDITIONAL_FORMATTING")]
    PasteConditionalFormatting,
    #[serde(rename = "PASTE_DATA_VALIDATION")]
    PasteDataValidation,
    #[serde(rename = "PASTE_FORMAT")]
    PasteFormat,
    #[serde(rename = "PASTE_FORMULA")]
    PasteFormula,
    #[serde(rename = "PASTE_NORMAL")]
    PasteNormal,
    #[serde(rename = "PASTE_NO_BORDERS")]
    PasteNoBorders,
    #[serde(rename = "PASTE_VALUES")]
    PasteValues,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PasteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasteType::PasteConditionalFormatting => "PASTE_CONDITIONAL_FORMATTING",
            PasteType::PasteDataValidation => "PASTE_DATA_VALIDATION",
            PasteType::PasteFormat => "PASTE_FORMAT",
            PasteType::PasteFormula => "PASTE_FORMULA",
            PasteType::PasteNormal => "PASTE_NORMAL",
            PasteType::PasteNoBorders => "PASTE_NO_BORDERS",
            PasteType::PasteValues => "PASTE_VALUES",
            PasteType::Noop => "",
            PasteType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PasteType {
    fn default() -> PasteType {
        PasteType::Noop
    }
}
impl PasteType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PasteType::Noop)
    }
}

/// Copies data from the source to the destination.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CopyPasteRequest {
    /**
     * Copies data from the source to the destination.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<GridRange>,
    /**
     * Copies data from the source to the destination.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pasteOrientation"
    )]
    pub paste_orientation: Option<PasteOrientation>,
    /**
     * Copies data from the source to the destination.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pasteType")]
    pub paste_type: Option<PasteType>,
    /**
     * Copies data from the source to the destination.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GridRange>,
}

/// The request to copy a sheet across spreadsheets.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CopySheetAnotherSpreadsheetRequest {
    /**
     * The request to copy a sheet across spreadsheets.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "destinationSpreadsheetId"
    )]
    pub destination_spreadsheet_id: String,
}

/// A request to create developer metadata.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreateDeveloperMetadataRequest {
    /**
     * A request to create developer metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "developerMetadata"
    )]
    pub developer_metadata: Option<DeveloperMetadata>,
}

/// The response from creating developer metadata.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CreateDeveloperMetadataResponse {
    /**
     * The response from creating developer metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "developerMetadata"
    )]
    pub developer_metadata: Option<DeveloperMetadata>,
}

/// Moves data from the source to the destination.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct CutPasteRequest {
    /**
     * Moves data from the source to the destination.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<GridCoordinate>,
    /**
     * Moves data from the source to the destination.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pasteType")]
    pub paste_type: Option<PasteType>,
    /**
     * Moves data from the source to the destination.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GridRange>,
}

/**
* The error code.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ErrorCode {
    #[serde(rename = "CONCURRENT_QUERY")]
    ConcurrentQuery,
    #[serde(rename = "DATA_EXECUTION_ERROR_CODE_UNSPECIFIED")]
    DataExecutionErrorCodeUnspecified,
    #[serde(rename = "DATA_NOT_FOUND")]
    DataNotFound,
    #[serde(rename = "DUPLICATE_COLUMN_NAMES")]
    DuplicateColumnNames,
    #[serde(rename = "ENGINE")]
    Engine,
    #[serde(rename = "INTERRUPTED")]
    Interrupted,
    #[serde(rename = "MISSING_COLUMN_ALIAS")]
    MissingColumnAlias,
    #[serde(rename = "OBJECT_IN_ERROR_STATE")]
    ObjectInErrorState,
    #[serde(rename = "OBJECT_NOT_FOUND")]
    ObjectNotFound,
    #[serde(rename = "OBJECT_SPEC_INVALID")]
    ObjectSpecInvalid,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "PARAMETER_INVALID")]
    ParameterInvalid,
    #[serde(rename = "PERMISSION_DENIED")]
    PermissionDenied,
    #[serde(rename = "TIMED_OUT")]
    TimedOut,
    #[serde(rename = "TOO_MANY_CELLS")]
    TooManyCells,
    #[serde(rename = "TOO_MANY_CHARS_PER_CELL")]
    TooManyCharsPerCell,
    #[serde(rename = "TOO_MANY_ROWS")]
    TooManyRows,
    #[serde(rename = "UNSUPPORTED_DATA_TYPE")]
    UnsupportedDataType,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::ConcurrentQuery => "CONCURRENT_QUERY",
            ErrorCode::DataExecutionErrorCodeUnspecified => "DATA_EXECUTION_ERROR_CODE_UNSPECIFIED",
            ErrorCode::DataNotFound => "DATA_NOT_FOUND",
            ErrorCode::DuplicateColumnNames => "DUPLICATE_COLUMN_NAMES",
            ErrorCode::Engine => "ENGINE",
            ErrorCode::Interrupted => "INTERRUPTED",
            ErrorCode::MissingColumnAlias => "MISSING_COLUMN_ALIAS",
            ErrorCode::ObjectInErrorState => "OBJECT_IN_ERROR_STATE",
            ErrorCode::ObjectNotFound => "OBJECT_NOT_FOUND",
            ErrorCode::ObjectSpecInvalid => "OBJECT_SPEC_INVALID",
            ErrorCode::Other => "OTHER",
            ErrorCode::ParameterInvalid => "PARAMETER_INVALID",
            ErrorCode::PermissionDenied => "PERMISSION_DENIED",
            ErrorCode::TimedOut => "TIMED_OUT",
            ErrorCode::TooManyCells => "TOO_MANY_CELLS",
            ErrorCode::TooManyCharsPerCell => "TOO_MANY_CHARS_PER_CELL",
            ErrorCode::TooManyRows => "TOO_MANY_ROWS",
            ErrorCode::UnsupportedDataType => "UNSUPPORTED_DATA_TYPE",
            ErrorCode::Noop => "",
            ErrorCode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ErrorCode {
    fn default() -> ErrorCode {
        ErrorCode::Noop
    }
}
impl ErrorCode {
    pub fn is_noop(&self) -> bool {
        matches!(self, ErrorCode::Noop)
    }
}

/**
* The state of the data execution.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum State {
    #[serde(rename = "DATA_EXECUTION_STATE_UNSPECIFIED")]
    DataExecutionStateUnspecified,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::DataExecutionStateUnspecified => "DATA_EXECUTION_STATE_UNSPECIFIED",
            State::Failed => "FAILED",
            State::NotStarted => "NOT_STARTED",
            State::Running => "RUNNING",
            State::Succeeded => "SUCCEEDED",
            State::Noop => "",
            State::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for State {
    fn default() -> State {
        State::Noop
    }
}
impl State {
    pub fn is_noop(&self) -> bool {
        matches!(self, State::Noop)
    }
}

/// The data execution status. A data execution is created to sync a data source object with the latest data from a DataSource. It is usually scheduled to run at background, you can check its state to tell if an execution completes There are several scenarios where a data execution is triggered to run: * Adding a data source creates an associated data source sheet as well as a data execution to sync the data from the data source to the sheet. * Updating a data source creates a data execution to refresh the associated data source sheet similarly. * You can send refresh request to explicitly refresh one or multiple data source objects.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataExecutionStatus {
    /**
     * The data execution status. A data execution is created to sync a data source object with the latest data from a DataSource. It is usually scheduled to run at background, you can check its state to tell if an execution completes There are several scenarios where a data execution is triggered to run: \* Adding a data source creates an associated data source sheet as well as a data execution to sync the data from the data source to the sheet. \* Updating a data source creates a data execution to refresh the associated data source sheet similarly. \* You can send refresh request to explicitly refresh one or multiple data source objects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorCode")]
    pub error_code: Option<ErrorCode>,
    /**
     * The data execution status. A data execution is created to sync a data source object with the latest data from a DataSource. It is usually scheduled to run at background, you can check its state to tell if an execution completes There are several scenarios where a data execution is triggered to run: \* Adding a data source creates an associated data source sheet as well as a data execution to sync the data from the data source to the sheet. \* Updating a data source creates a data execution to refresh the associated data source sheet similarly. \* You can send refresh request to explicitly refresh one or multiple data source objects.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "errorMessage"
    )]
    pub error_message: String,
    /**
     * The data execution status. A data execution is created to sync a data source object with the latest data from a DataSource. It is usually scheduled to run at background, you can check its state to tell if an execution completes There are several scenarios where a data execution is triggered to run: \* Adding a data source creates an associated data source sheet as well as a data execution to sync the data from the data source to the sheet. \* Updating a data source creates a data execution to refresh the associated data source sheet similarly. \* You can send refresh request to explicitly refresh one or multiple data source objects.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "lastRefreshTime"
    )]
    pub last_refresh_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The data execution status. A data execution is created to sync a data source object with the latest data from a DataSource. It is usually scheduled to run at background, you can check its state to tell if an execution completes There are several scenarios where a data execution is triggered to run: \* Adding a data source creates an associated data source sheet as well as a data execution to sync the data from the data source to the sheet. \* Updating a data source creates a data execution to refresh the associated data source sheet similarly. \* You can send refresh request to explicitly refresh one or multiple data source objects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

/// Filter that describes what data should be selected or returned from a request.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataFilter {
    /**
     * Filter that describes what data should be selected or returned from a request.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "a1Range"
    )]
    pub a_1_range: String,
    /**
     * Filter that describes what data should be selected or returned from a request.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "developerMetadataLookup"
    )]
    pub developer_metadata_lookup: Option<DeveloperMetadataLookup>,
    /**
     * Filter that describes what data should be selected or returned from a request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gridRange")]
    pub grid_range: Option<GridRange>,
}

/// A range of values whose location is specified by a DataFilter.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataFilterValueRange {
    /**
     * A range of values whose location is specified by a DataFilter.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataFilter"
    )]
    pub data_filter: Option<DataFilter>,
    /**
     * A range of values whose location is specified by a DataFilter.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "majorDimension"
    )]
    pub major_dimension: Option<Dimension>,
    /**
     * A range of values whose location is specified by a DataFilter.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub values: Vec<Vec<String>>,
}

/**
* The placement of the data label relative to the labeled data.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Placement {
    #[serde(rename = "ABOVE")]
    Above,
    #[serde(rename = "BELOW")]
    Below,
    #[serde(rename = "CENTER")]
    Center,
    #[serde(rename = "DATA_LABEL_PLACEMENT_UNSPECIFIED")]
    DataLabelPlacementUnspecified,
    #[serde(rename = "INSIDE_BASE")]
    InsideBase,
    #[serde(rename = "INSIDE_END")]
    InsideEnd,
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "OUTSIDE_END")]
    OutsideEnd,
    #[serde(rename = "RIGHT")]
    Right,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Placement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Placement::Above => "ABOVE",
            Placement::Below => "BELOW",
            Placement::Center => "CENTER",
            Placement::DataLabelPlacementUnspecified => "DATA_LABEL_PLACEMENT_UNSPECIFIED",
            Placement::InsideBase => "INSIDE_BASE",
            Placement::InsideEnd => "INSIDE_END",
            Placement::Left => "LEFT",
            Placement::OutsideEnd => "OUTSIDE_END",
            Placement::Right => "RIGHT",
            Placement::Noop => "",
            Placement::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Placement {
    fn default() -> Placement {
        Placement::Noop
    }
}
impl Placement {
    pub fn is_noop(&self) -> bool {
        matches!(self, Placement::Noop)
    }
}

/**
* The type of the data label.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DataLabelType {
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "DATA")]
    Data,
    #[serde(rename = "DATA_LABEL_TYPE_UNSPECIFIED")]
    DataLabelTypeUnspecified,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DataLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataLabelType::Custom => "CUSTOM",
            DataLabelType::Data => "DATA",
            DataLabelType::DataLabelTypeUnspecified => "DATA_LABEL_TYPE_UNSPECIFIED",
            DataLabelType::None => "NONE",
            DataLabelType::Noop => "",
            DataLabelType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DataLabelType {
    fn default() -> DataLabelType {
        DataLabelType::Noop
    }
}
impl DataLabelType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DataLabelType::Noop)
    }
}

/// Settings for one set of data labels. Data labels are annotations that appear next to a set of data, such as the points on a line chart, and provide additional information about what the data represents, such as a text representation of the value behind that point on the graph.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataLabel {
    /**
     * Settings for one set of data labels. Data labels are annotations that appear next to a set of data, such as the points on a line chart, and provide additional information about what the data represents, such as a text representation of the value behind that point on the graph.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "customLabelData"
    )]
    pub custom_label_data: Option<ChartData>,
    /**
     * Settings for one set of data labels. Data labels are annotations that appear next to a set of data, such as the points on a line chart, and provide additional information about what the data represents, such as a text representation of the value behind that point on the graph.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
    /**
     * Settings for one set of data labels. Data labels are annotations that appear next to a set of data, such as the points on a line chart, and provide additional information about what the data represents, such as a text representation of the value behind that point on the graph.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "textFormat"
    )]
    pub text_format: Option<TextFormat>,
    /**
     * Settings for one set of data labels. Data labels are annotations that appear next to a set of data, such as the points on a line chart, and provide additional information about what the data represents, such as a text representation of the value behind that point on the graph.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<DataLabelType>,
}

/// Information about an external data source in the spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSource {
    /**
     * Information about an external data source in the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "calculatedColumns"
    )]
    pub calculated_columns: Vec<DataSourceColumn>,
    /**
     * Information about an external data source in the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dataSourceId"
    )]
    pub data_source_id: String,
    /**
     * Information about an external data source in the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
    /**
     * Information about an external data source in the spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<DataSourceSpec>,
}

/// Properties of a data source chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceChartProperties {
    /**
     * Properties of a data source chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataExecutionStatus"
    )]
    pub data_execution_status: Option<DataExecutionStatus>,
    /**
     * Properties of a data source chart.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dataSourceId"
    )]
    pub data_source_id: String,
}

/// A column in a data source.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceColumn {
    /**
     * A column in a data source.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub formula: String,
    /**
     * A column in a data source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<DataSourceColumnReference>,
}

/// An unique identifier that references a data source column.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceColumnReference {
    /**
     * An unique identifier that references a data source column.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// A data source formula.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceFormula {
    /**
     * A data source formula.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataExecutionStatus"
    )]
    pub data_execution_status: Option<DataExecutionStatus>,
    /**
     * A data source formula.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dataSourceId"
    )]
    pub data_source_id: String,
}

/// Reference to a data source object.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceObjectReference {
    /**
     * Reference to a data source object.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "chartId"
    )]
    pub chart_id: i64,
    /**
     * Reference to a data source object.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceFormulaCell"
    )]
    pub data_source_formula_cell: Option<GridCoordinate>,
    /**
     * Reference to a data source object.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourcePivotTableAnchorCell"
    )]
    pub data_source_pivot_table_anchor_cell: Option<GridCoordinate>,
    /**
     * Reference to a data source object.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceTableAnchorCell"
    )]
    pub data_source_table_anchor_cell: Option<GridCoordinate>,
    /**
     * Reference to a data source object.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: String,
}

/// A list of references to data source objects.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceObjectReferences {
    /**
     * A list of references to data source objects.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub references: Vec<DataSourceObjectReference>,
}

/// A parameter in a data source's query. The parameter allows the user to pass in values from the spreadsheet into a query.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceParameter {
    /**
     * A parameter in a data source's query. The parameter allows the user to pass in values from the spreadsheet into a query.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A parameter in a data source's query. The parameter allows the user to pass in values from the spreadsheet into a query.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "namedRangeId"
    )]
    pub named_range_id: String,
    /**
     * A parameter in a data source's query. The parameter allows the user to pass in values from the spreadsheet into a query.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
}

/// A schedule for data to refresh every day in a given time interval.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceRefreshDailySchedule {
    /**
     * A schedule for data to refresh every day in a given time interval.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<TimeOfDay>,
}

/// A monthly schedule for data to refresh on specific days in the month in a given time interval.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceRefreshMonthlySchedule {
    /**
     * A monthly schedule for data to refresh on specific days in the month in a given time interval.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "daysOfMonth"
    )]
    pub days_of_month: Vec<i64>,
    /**
     * A monthly schedule for data to refresh on specific days in the month in a given time interval.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<TimeOfDay>,
}

/**
* The scope of the refresh. Must be ALL_DATA_SOURCES.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum RefreshScope {
    #[serde(rename = "ALL_DATA_SOURCES")]
    AllDataSources,
    #[serde(rename = "DATA_SOURCE_REFRESH_SCOPE_UNSPECIFIED")]
    DataSourceRefreshScopeUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for RefreshScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefreshScope::AllDataSources => "ALL_DATA_SOURCES",
            RefreshScope::DataSourceRefreshScopeUnspecified => {
                "DATA_SOURCE_REFRESH_SCOPE_UNSPECIFIED"
            }
            RefreshScope::Noop => "",
            RefreshScope::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for RefreshScope {
    fn default() -> RefreshScope {
        RefreshScope::Noop
    }
}
impl RefreshScope {
    pub fn is_noop(&self) -> bool {
        matches!(self, RefreshScope::Noop)
    }
}

/// Schedule for refreshing the data source. Data sources in the spreadsheet are refreshed within a time interval. You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours. For example, if you specify a start time of 8am , the refresh will take place between 8am and 12pm every day.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceRefreshSchedule {
    /**
     * Schedule for refreshing the data source. Data sources in the spreadsheet are refreshed within a time interval. You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours. For example, if you specify a start time of 8am , the refresh will take place between 8am and 12pm every day.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dailySchedule"
    )]
    pub daily_schedule: Option<DataSourceRefreshDailySchedule>,
    /**
     * Schedule for refreshing the data source. Data sources in the spreadsheet are refreshed within a time interval. You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours. For example, if you specify a start time of 8am , the refresh will take place between 8am and 12pm every day.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    /**
     * Schedule for refreshing the data source. Data sources in the spreadsheet are refreshed within a time interval. You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours. For example, if you specify a start time of 8am , the refresh will take place between 8am and 12pm every day.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "monthlySchedule"
    )]
    pub monthly_schedule: Option<DataSourceRefreshMonthlySchedule>,
    /**
     * Schedule for refreshing the data source. Data sources in the spreadsheet are refreshed within a time interval. You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours. For example, if you specify a start time of 8am , the refresh will take place between 8am and 12pm every day.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextRun")]
    pub next_run: Option<Interval>,
    /**
     * Schedule for refreshing the data source. Data sources in the spreadsheet are refreshed within a time interval. You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours. For example, if you specify a start time of 8am , the refresh will take place between 8am and 12pm every day.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refreshScope"
    )]
    pub refresh_scope: Option<RefreshScope>,
    /**
     * Schedule for refreshing the data source. Data sources in the spreadsheet are refreshed within a time interval. You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours. For example, if you specify a start time of 8am , the refresh will take place between 8am and 12pm every day.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "weeklySchedule"
    )]
    pub weekly_schedule: Option<DataSourceRefreshWeeklySchedule>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DaysOfWeek {
    #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
    DayOfWeekUnspecified,
    #[serde(rename = "FRIDAY")]
    Friday,
    #[serde(rename = "MONDAY")]
    Monday,
    #[serde(rename = "SATURDAY")]
    Saturday,
    #[serde(rename = "SUNDAY")]
    Sunday,
    #[serde(rename = "THURSDAY")]
    Thursday,
    #[serde(rename = "TUESDAY")]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    Wednesday,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DaysOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DaysOfWeek::DayOfWeekUnspecified => "DAY_OF_WEEK_UNSPECIFIED",
            DaysOfWeek::Friday => "FRIDAY",
            DaysOfWeek::Monday => "MONDAY",
            DaysOfWeek::Saturday => "SATURDAY",
            DaysOfWeek::Sunday => "SUNDAY",
            DaysOfWeek::Thursday => "THURSDAY",
            DaysOfWeek::Tuesday => "TUESDAY",
            DaysOfWeek::Wednesday => "WEDNESDAY",
            DaysOfWeek::Noop => "",
            DaysOfWeek::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DaysOfWeek {
    fn default() -> DaysOfWeek {
        DaysOfWeek::Noop
    }
}
impl DaysOfWeek {
    pub fn is_noop(&self) -> bool {
        matches!(self, DaysOfWeek::Noop)
    }
}

/// A weekly schedule for data to refresh on specific days in a given time interval.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceRefreshWeeklySchedule {
    /**
     * A weekly schedule for data to refresh on specific days in a given time interval.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "daysOfWeek"
    )]
    pub days_of_week: Vec<DaysOfWeek>,
    /**
     * A weekly schedule for data to refresh on specific days in a given time interval.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<TimeOfDay>,
}

/// A range along a single dimension on a DATA_SOURCE sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceSheetDimensionRange {
    /**
     * A range along a single dimension on a DATA_SOURCE sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "columnReferences"
    )]
    pub column_references: Vec<DataSourceColumnReference>,
    /**
     * A range along a single dimension on a DATA_SOURCE sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/// Additional properties of a DATA_SOURCE sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceSheetProperties {
    /**
     * Additional properties of a DATA_SOURCE sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub columns: Vec<DataSourceColumn>,
    /**
     * Additional properties of a DATA_SOURCE sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataExecutionStatus"
    )]
    pub data_execution_status: Option<DataExecutionStatus>,
    /**
     * Additional properties of a DATA_SOURCE sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dataSourceId"
    )]
    pub data_source_id: String,
}

/// This specifies the details of the data source. For example, for BigQuery, this specifies information about the BigQuery source.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceSpec {
    /**
     * This specifies the details of the data source. For example, for BigQuery, this specifies information about the BigQuery source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bigQuery")]
    pub big_query: Option<BigQueryDataSourceSpec>,
    /**
     * This specifies the details of the data source. For example, for BigQuery, this specifies information about the BigQuery source.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub parameters: Vec<DataSourceParameter>,
}

/**
* The type to select columns for the data source table. Defaults to SELECTED.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ColumnSelectionType {
    #[serde(rename = "DATA_SOURCE_TABLE_COLUMN_SELECTION_TYPE_UNSPECIFIED")]
    DataSourceTableColumnSelectionTypeUnspecified,
    #[serde(rename = "SELECTED")]
    Selected,
    #[serde(rename = "SYNC_ALL")]
    SyncAll,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ColumnSelectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnSelectionType::DataSourceTableColumnSelectionTypeUnspecified => {
                "DATA_SOURCE_TABLE_COLUMN_SELECTION_TYPE_UNSPECIFIED"
            }
            ColumnSelectionType::Selected => "SELECTED",
            ColumnSelectionType::SyncAll => "SYNC_ALL",
            ColumnSelectionType::Noop => "",
            ColumnSelectionType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ColumnSelectionType {
    fn default() -> ColumnSelectionType {
        ColumnSelectionType::Noop
    }
}
impl ColumnSelectionType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ColumnSelectionType::Noop)
    }
}

/// A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataSourceTable {
    /**
     * A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "columnSelectionType"
    )]
    pub column_selection_type: Option<ColumnSelectionType>,
    /**
     * A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub columns: Vec<DataSourceColumnReference>,
    /**
     * A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataExecutionStatus"
    )]
    pub data_execution_status: Option<DataExecutionStatus>,
    /**
     * A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dataSourceId"
    )]
    pub data_source_id: String,
    /**
     * A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "filterSpecs"
    )]
    pub filter_specs: Vec<FilterSpec>,
    /**
     * A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "rowLimit"
    )]
    pub row_limit: i64,
    /**
     * A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "sortSpecs"
    )]
    pub sort_specs: Vec<SortSpec>,
}

/// A data validation rule.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DataValidationRule {
    /**
     * A data validation rule.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<BooleanCondition>,
    /**
     * A data validation rule.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "inputMessage"
    )]
    pub input_message: String,
    /**
     * A data validation rule.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "showCustomUi"
    )]
    pub show_custom_ui: bool,
    /**
     * A data validation rule.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub strict: bool,
}

/**
* The type of date-time grouping to apply.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DateTimeRuleType {
    #[serde(rename = "DATE_TIME_RULE_TYPE_UNSPECIFIED")]
    DateTimeRuleTypeUnspecified,
    #[serde(rename = "DAY_MONTH")]
    DayMonth,
    #[serde(rename = "DAY_OF_MONTH")]
    DayOfMonth,
    #[serde(rename = "DAY_OF_WEEK")]
    DayOfWeek,
    #[serde(rename = "DAY_OF_YEAR")]
    DayOfYear,
    #[serde(rename = "HOUR")]
    Hour,
    #[serde(rename = "HOUR_MINUTE")]
    HourMinute,
    #[serde(rename = "HOUR_MINUTE_AMPM")]
    HourMinuteAmpm,
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "MONTH")]
    Month,
    #[serde(rename = "QUARTER")]
    Quarter,
    #[serde(rename = "SECOND")]
    Second,
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "YEAR_MONTH")]
    YearMonth,
    #[serde(rename = "YEAR_MONTH_DAY")]
    YearMonthDay,
    #[serde(rename = "YEAR_QUARTER")]
    YearQuarter,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DateTimeRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTimeRuleType::DateTimeRuleTypeUnspecified => "DATE_TIME_RULE_TYPE_UNSPECIFIED",
            DateTimeRuleType::DayMonth => "DAY_MONTH",
            DateTimeRuleType::DayOfMonth => "DAY_OF_MONTH",
            DateTimeRuleType::DayOfWeek => "DAY_OF_WEEK",
            DateTimeRuleType::DayOfYear => "DAY_OF_YEAR",
            DateTimeRuleType::Hour => "HOUR",
            DateTimeRuleType::HourMinute => "HOUR_MINUTE",
            DateTimeRuleType::HourMinuteAmpm => "HOUR_MINUTE_AMPM",
            DateTimeRuleType::Minute => "MINUTE",
            DateTimeRuleType::Month => "MONTH",
            DateTimeRuleType::Quarter => "QUARTER",
            DateTimeRuleType::Second => "SECOND",
            DateTimeRuleType::Year => "YEAR",
            DateTimeRuleType::YearMonth => "YEAR_MONTH",
            DateTimeRuleType::YearMonthDay => "YEAR_MONTH_DAY",
            DateTimeRuleType::YearQuarter => "YEAR_QUARTER",
            DateTimeRuleType::Noop => "",
            DateTimeRuleType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DateTimeRuleType {
    fn default() -> DateTimeRuleType {
        DateTimeRuleType::Noop
    }
}
impl DateTimeRuleType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DateTimeRuleType::Noop)
    }
}

/// Allows you to organize the date-time values in a source data column into buckets based on selected parts of their date or time values. For example, consider a pivot table showing sales transactions by date: +----------+--------------+ | Date | SUM of Sales | +----------+--------------+ | 1/1/2017 | $621.14 | | 2/3/2017 | $708.84 | | 5/8/2017 | $326.84 | ... +----------+--------------+ Applying a date-time group rule with a DateTimeRuleType of YEAR_MONTH results in the following pivot table. +--------------+--------------+ | Grouped Date | SUM of Sales | +--------------+--------------+ | 2017-Jan | $53,731.78 | | 2017-Feb | $83,475.32 | | 2017-Mar | $94,385.05 | ... +--------------+--------------+
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DateTimeRule {
    /**
     * Allows you to organize the date-time values in a source data column into buckets based on selected parts of their date or time values. For example, consider a pivot table showing sales transactions by date: +----------+--------------+ | Date | SUM of Sales | +----------+--------------+ | 1/1/2017 | $621.14 | | 2/3/2017 | $708.84 | | 5/8/2017 | $326.84 | ... +----------+--------------+ Applying a date-time group rule with a DateTimeRuleType of YEAR_MONTH results in the following pivot table. +--------------+--------------+ | Grouped Date | SUM of Sales | +--------------+--------------+ | 2017-Jan | $53,731.78 | | 2017-Feb | $83,475.32 | | 2017-Mar | $94,385.05 | ... +--------------+--------------+
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<DateTimeRuleType>,
}

/// Removes the banded range with the given ID from the spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteBandingRequest {
    /**
     * Removes the banded range with the given ID from the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "bandedRangeId"
    )]
    pub banded_range_id: i64,
}

/// Deletes a conditional format rule at the given index. All subsequent rules' indexes are decremented.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteConditionalFormatRuleRequest {
    /**
     * Deletes a conditional format rule at the given index. All subsequent rules' indexes are decremented.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub index: i64,
    /**
     * Deletes a conditional format rule at the given index. All subsequent rules' indexes are decremented.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/// The result of deleting a conditional format rule.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteConditionalFormatRuleResponse {
    /**
     * The result of deleting a conditional format rule.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<ConditionalFormatRule>,
}

/// Deletes a data source. The request also deletes the associated data source sheet, and unlinks all associated data source objects.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteDataSourceRequest {
    /**
     * Deletes a data source. The request also deletes the associated data source sheet, and unlinks all associated data source objects.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dataSourceId"
    )]
    pub data_source_id: String,
}

/// A request to delete developer metadata.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteDeveloperMetadataRequest {
    /**
     * A request to delete developer metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataFilter"
    )]
    pub data_filter: Option<DataFilter>,
}

/// The response from deleting developer metadata.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteDeveloperMetadataResponse {
    /**
     * The response from deleting developer metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "deletedDeveloperMetadata"
    )]
    pub deleted_developer_metadata: Vec<DeveloperMetadata>,
}

/// Deletes a group over the specified range by decrementing the depth of the dimensions in the range. For example, assume the sheet has a depth-1 group over B:E and a depth-2 group over C:D. Deleting a group over D:E leaves the sheet with a depth-1 group over B:D and a depth-2 group over C:C.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteDimensionGroupRequest {
    /**
     * Deletes a group over the specified range by decrementing the depth of the dimensions in the range. For example, assume the sheet has a depth-1 group over B:E and a depth-2 group over C:D. Deleting a group over D:E leaves the sheet with a depth-1 group over B:D and a depth-2 group over C:C.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<DimensionRange>,
}

/// The result of deleting a group.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteDimensionGroupResponse {
    /**
     * The result of deleting a group.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dimensionGroups"
    )]
    pub dimension_groups: Vec<DimensionGroup>,
}

/// Deletes the dimensions from the sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteDimensionRequest {
    /**
     * Deletes the dimensions from the sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<DimensionRange>,
}

/// Removes rows within this range that contain values in the specified columns that are duplicates of values in any previous row. Rows with identical values but different letter cases, formatting, or formulas are considered to be duplicates. This request also removes duplicate rows hidden from view (for example, due to a filter). When removing duplicates, the first instance of each duplicate row scanning from the top downwards is kept in the resulting range. Content outside of the specified range isn't removed, and rows considered duplicates do not have to be adjacent to each other in the range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteDuplicatesRequest {
    /**
     * Removes rows within this range that contain values in the specified columns that are duplicates of values in any previous row. Rows with identical values but different letter cases, formatting, or formulas are considered to be duplicates. This request also removes duplicate rows hidden from view (for example, due to a filter). When removing duplicates, the first instance of each duplicate row scanning from the top downwards is kept in the resulting range. Content outside of the specified range isn't removed, and rows considered duplicates do not have to be adjacent to each other in the range.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "comparisonColumns"
    )]
    pub comparison_columns: Vec<DimensionRange>,
    /**
     * Removes rows within this range that contain values in the specified columns that are duplicates of values in any previous row. Rows with identical values but different letter cases, formatting, or formulas are considered to be duplicates. This request also removes duplicate rows hidden from view (for example, due to a filter). When removing duplicates, the first instance of each duplicate row scanning from the top downwards is kept in the resulting range. Content outside of the specified range isn't removed, and rows considered duplicates do not have to be adjacent to each other in the range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
}

/// The result of removing duplicates in a range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteDuplicatesResponse {
    /**
     * The result of removing duplicates in a range.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "duplicatesRemovedCount"
    )]
    pub duplicates_removed_count: i64,
}

/// Deletes the embedded object with the given ID.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteEmbeddedObjectRequest {
    /**
     * Deletes the embedded object with the given ID.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "objectId"
    )]
    pub object_id: i64,
}

/// Deletes a particular filter view.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteFilterViewRequest {
    /**
     * Deletes a particular filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "filterId"
    )]
    pub filter_id: i64,
}

/// Removes the named range with the given ID from the spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteNamedRangeRequest {
    /**
     * Removes the named range with the given ID from the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "namedRangeId"
    )]
    pub named_range_id: String,
}

/// Deletes the protected range with the given ID.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteProtectedRangeRequest {
    /**
     * Deletes the protected range with the given ID.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "protectedRangeId"
    )]
    pub protected_range_id: i64,
}

/// Deletes a range of cells, shifting other cells into the deleted area.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteRangeRequest {
    /**
     * Deletes a range of cells, shifting other cells into the deleted area.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * Deletes a range of cells, shifting other cells into the deleted area.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "shiftDimension"
    )]
    pub shift_dimension: Option<Dimension>,
}

/// Deletes the requested sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeleteSheetRequest {
    /**
     * Deletes the requested sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/**
* The metadata visibility. Developer metadata must always have a visibility specified.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Visibility {
    #[serde(rename = "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED")]
    DeveloperMetadataVisibilityUnspecified,
    #[serde(rename = "DOCUMENT")]
    Document,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::DeveloperMetadataVisibilityUnspecified => {
                "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED"
            }
            Visibility::Document => "DOCUMENT",
            Visibility::Project => "PROJECT",
            Visibility::Noop => "",
            Visibility::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Visibility {
    fn default() -> Visibility {
        Visibility::Noop
    }
}
impl Visibility {
    pub fn is_noop(&self) -> bool {
        matches!(self, Visibility::Noop)
    }
}

/// Developer metadata associated with a location or object in a spreadsheet. Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet and will remain associated at those locations as they move around and the spreadsheet is edited. For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, that original metadata will still be associated with the row it was first associated with (what is now row 6). If the associated object is deleted its metadata is deleted too.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeveloperMetadata {
    /**
     * Developer metadata associated with a location or object in a spreadsheet. Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet and will remain associated at those locations as they move around and the spreadsheet is edited. For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, that original metadata will still be associated with the row it was first associated with (what is now row 6). If the associated object is deleted its metadata is deleted too.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<DeveloperMetadataLocation>,
    /**
     * Developer metadata associated with a location or object in a spreadsheet. Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet and will remain associated at those locations as they move around and the spreadsheet is edited. For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, that original metadata will still be associated with the row it was first associated with (what is now row 6). If the associated object is deleted its metadata is deleted too.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "metadataId"
    )]
    pub metadata_id: i64,
    /**
     * Developer metadata associated with a location or object in a spreadsheet. Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet and will remain associated at those locations as they move around and the spreadsheet is edited. For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, that original metadata will still be associated with the row it was first associated with (what is now row 6). If the associated object is deleted its metadata is deleted too.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "metadataKey"
    )]
    pub metadata_key: String,
    /**
     * Developer metadata associated with a location or object in a spreadsheet. Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet and will remain associated at those locations as they move around and the spreadsheet is edited. For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, that original metadata will still be associated with the row it was first associated with (what is now row 6). If the associated object is deleted its metadata is deleted too.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "metadataValue"
    )]
    pub metadata_value: String,
    /**
     * Developer metadata associated with a location or object in a spreadsheet. Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet and will remain associated at those locations as they move around and the spreadsheet is edited. For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, that original metadata will still be associated with the row it was first associated with (what is now row 6). If the associated object is deleted its metadata is deleted too.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

/**
* The type of location this object represents. This field is read-only.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LocationType {
    #[serde(rename = "COLUMN")]
    Column,
    #[serde(rename = "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED")]
    DeveloperMetadataLocationTypeUnspecified,
    #[serde(rename = "ROW")]
    Row,
    #[serde(rename = "SHEET")]
    Sheet,
    #[serde(rename = "SPREADSHEET")]
    Spreadsheet,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationType::Column => "COLUMN",
            LocationType::DeveloperMetadataLocationTypeUnspecified => {
                "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED"
            }
            LocationType::Row => "ROW",
            LocationType::Sheet => "SHEET",
            LocationType::Spreadsheet => "SPREADSHEET",
            LocationType::Noop => "",
            LocationType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LocationType {
    fn default() -> LocationType {
        LocationType::Noop
    }
}
impl LocationType {
    pub fn is_noop(&self) -> bool {
        matches!(self, LocationType::Noop)
    }
}

/// A location where metadata may be associated in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeveloperMetadataLocation {
    /**
     * A location where metadata may be associated in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dimensionRange"
    )]
    pub dimension_range: Option<DimensionRange>,
    /**
     * A location where metadata may be associated in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "locationType"
    )]
    pub location_type: Option<LocationType>,
    /**
     * A location where metadata may be associated in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
    /**
     * A location where metadata may be associated in a spreadsheet.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub spreadsheet: bool,
}

/**
* Determines how this lookup matches the location. If this field is specified as EXACT, only developer metadata associated on the exact location specified is matched. If this field is specified to INTERSECTING, developer metadata associated on intersecting locations is also matched. If left unspecified, this field assumes a default value of INTERSECTING. If this field is specified, a metadataLocation must also be specified.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LocationMatchingStrategy {
    #[serde(rename = "DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED")]
    DeveloperMetadataLocationMatchingStrategyUnspecified,
    #[serde(rename = "EXACT_LOCATION")]
    ExactLocation,
    #[serde(rename = "INTERSECTING_LOCATION")]
    IntersectingLocation,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LocationMatchingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationMatchingStrategy::DeveloperMetadataLocationMatchingStrategyUnspecified => {
                "DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED"
            }
            LocationMatchingStrategy::ExactLocation => "EXACT_LOCATION",
            LocationMatchingStrategy::IntersectingLocation => "INTERSECTING_LOCATION",
            LocationMatchingStrategy::Noop => "",
            LocationMatchingStrategy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LocationMatchingStrategy {
    fn default() -> LocationMatchingStrategy {
        LocationMatchingStrategy::Noop
    }
}
impl LocationMatchingStrategy {
    pub fn is_noop(&self) -> bool {
        matches!(self, LocationMatchingStrategy::Noop)
    }
}

/// Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DeveloperMetadataLookup {
    /**
     * Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "locationMatchingStrategy"
    )]
    pub location_matching_strategy: Option<LocationMatchingStrategy>,
    /**
     * Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "locationType"
    )]
    pub location_type: Option<LocationType>,
    /**
     * Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "metadataId"
    )]
    pub metadata_id: i64,
    /**
     * Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "metadataKey"
    )]
    pub metadata_key: String,
    /**
     * Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "metadataLocation"
    )]
    pub metadata_location: Option<DeveloperMetadataLocation>,
    /**
     * Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "metadataValue"
    )]
    pub metadata_value: String,
    /**
     * Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

/// A group over an interval of rows or columns on a sheet, which can contain or be contained within other groups. A group can be collapsed or expanded as a unit on the sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DimensionGroup {
    /**
     * A group over an interval of rows or columns on a sheet, which can contain or be contained within other groups. A group can be collapsed or expanded as a unit on the sheet.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub collapsed: bool,
    /**
     * A group over an interval of rows or columns on a sheet, which can contain or be contained within other groups. A group can be collapsed or expanded as a unit on the sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub depth: i64,
    /**
     * A group over an interval of rows or columns on a sheet, which can contain or be contained within other groups. A group can be collapsed or expanded as a unit on the sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<DimensionRange>,
}

/// Properties about a dimension.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DimensionProperties {
    /**
     * Properties about a dimension.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceColumnReference"
    )]
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /**
     * Properties about a dimension.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "developerMetadata"
    )]
    pub developer_metadata: Vec<DeveloperMetadata>,
    /**
     * Properties about a dimension.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "hiddenByFilter"
    )]
    pub hidden_by_filter: bool,
    /**
     * Properties about a dimension.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "hiddenByUser"
    )]
    pub hidden_by_user: bool,
    /**
     * Properties about a dimension.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "pixelSize"
    )]
    pub pixel_size: i64,
}

/// A range along a single dimension on a sheet. All indexes are zero-based. Indexes are half open: the start index is inclusive and the end index is exclusive. Missing indexes indicate the range is unbounded on that side.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DimensionRange {
    /**
     * A range along a single dimension on a sheet. All indexes are zero-based. Indexes are half open: the start index is inclusive and the end index is exclusive. Missing indexes indicate the range is unbounded on that side.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Dimension>,
    /**
     * A range along a single dimension on a sheet. All indexes are zero-based. Indexes are half open: the start index is inclusive and the end index is exclusive. Missing indexes indicate the range is unbounded on that side.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "endIndex"
    )]
    pub end_index: i64,
    /**
     * A range along a single dimension on a sheet. All indexes are zero-based. Indexes are half open: the start index is inclusive and the end index is exclusive. Missing indexes indicate the range is unbounded on that side.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
    /**
     * A range along a single dimension on a sheet. All indexes are zero-based. Indexes are half open: the start index is inclusive and the end index is exclusive. Missing indexes indicate the range is unbounded on that side.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "startIndex"
    )]
    pub start_index: i64,
}

/// Duplicates a particular filter view.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DuplicateFilterViewRequest {
    /**
     * Duplicates a particular filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "filterId"
    )]
    pub filter_id: i64,
}

/// The result of a filter view being duplicated.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DuplicateFilterViewResponse {
    /**
     * The result of a filter view being duplicated.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterView>,
}

/// Duplicates the contents of a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DuplicateSheetRequest {
    /**
     * Duplicates the contents of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "insertSheetIndex"
    )]
    pub insert_sheet_index: i64,
    /**
     * Duplicates the contents of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "newSheetId"
    )]
    pub new_sheet_id: i64,
    /**
     * Duplicates the contents of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "newSheetName"
    )]
    pub new_sheet_name: String,
    /**
     * Duplicates the contents of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sourceSheetId"
    )]
    pub source_sheet_id: i64,
}

/// The result of duplicating a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct DuplicateSheetResponse {
    /**
     * The result of duplicating a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SheetProperties>,
}

/// The editors of a protected range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Editors {
    /**
     * The editors of a protected range.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "domainUsersCanEdit"
    )]
    pub domain_users_can_edit: bool,
    /**
     * The editors of a protected range.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub groups: Vec<String>,
    /**
     * The editors of a protected range.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub users: Vec<String>,
}

/// A chart embedded in a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmbeddedChart {
    /**
     * A chart embedded in a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<EmbeddedObjectBorder>,
    /**
     * A chart embedded in a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "chartId"
    )]
    pub chart_id: i64,
    /**
     * A chart embedded in a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<EmbeddedObjectPosition>,
    /**
     * A chart embedded in a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ChartSpec>,
}

/// A border along an embedded object.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmbeddedObjectBorder {
    /**
     * A border along an embedded object.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /**
     * A border along an embedded object.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "colorStyle"
    )]
    pub color_style: Option<ColorStyle>,
}

/// The position of an embedded object such as a chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct EmbeddedObjectPosition {
    /**
     * The position of an embedded object such as a chart.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "newSheet"
    )]
    pub new_sheet: bool,
    /**
     * The position of an embedded object such as a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "overlayPosition"
    )]
    pub overlay_position: Option<OverlayPosition>,
    /**
     * The position of an embedded object such as a chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/**
* The type of error.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ErrorValueType {
    #[serde(rename = "DIVIDE_BY_ZERO")]
    DivideByZero,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "ERROR_TYPE_UNSPECIFIED")]
    ErrorTypeUnspecified,
    #[serde(rename = "LOADING")]
    Loading,
    #[serde(rename = "NAME")]
    Name,
    #[serde(rename = "NULL_VALUE")]
    NullValue,
    #[serde(rename = "NUM")]
    Num,
    #[serde(rename = "N_A")]
    NA,
    #[serde(rename = "REF")]
    Ref,
    #[serde(rename = "VALUE")]
    Value,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ErrorValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorValueType::DivideByZero => "DIVIDE_BY_ZERO",
            ErrorValueType::Error => "ERROR",
            ErrorValueType::ErrorTypeUnspecified => "ERROR_TYPE_UNSPECIFIED",
            ErrorValueType::Loading => "LOADING",
            ErrorValueType::Name => "NAME",
            ErrorValueType::NullValue => "NULL_VALUE",
            ErrorValueType::Num => "NUM",
            ErrorValueType::NA => "N_A",
            ErrorValueType::Ref => "REF",
            ErrorValueType::Value => "VALUE",
            ErrorValueType::Noop => "",
            ErrorValueType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ErrorValueType {
    fn default() -> ErrorValueType {
        ErrorValueType::Noop
    }
}
impl ErrorValueType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ErrorValueType::Noop)
    }
}

/// An error in a cell.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ErrorValue {
    /**
     * An error in a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * An error in a cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<ErrorValueType>,
}

/// The kinds of value that a cell in a spreadsheet can have.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ExtendedValue {
    /**
     * The kinds of value that a cell in a spreadsheet can have.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "boolValue"
    )]
    pub bool_value: bool,
    /**
     * The kinds of value that a cell in a spreadsheet can have.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "errorValue"
    )]
    pub error_value: Option<ErrorValue>,
    /**
     * The kinds of value that a cell in a spreadsheet can have.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "formulaValue"
    )]
    pub formula_value: String,
    /**
     * The kinds of value that a cell in a spreadsheet can have.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "numberValue"
    )]
    pub number_value: f64,
    /**
     * The kinds of value that a cell in a spreadsheet can have.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "stringValue"
    )]
    pub string_value: String,
}

/// Criteria for showing/hiding rows in a filter or filter view.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FilterCriteria {
    /**
     * Criteria for showing/hiding rows in a filter or filter view.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<BooleanCondition>,
    /**
     * Criteria for showing/hiding rows in a filter or filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "hiddenValues"
    )]
    pub hidden_values: Vec<String>,
    /**
     * Criteria for showing/hiding rows in a filter or filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "visibleBackgroundColor"
    )]
    pub visible_background_color: Option<Color>,
    /**
     * Criteria for showing/hiding rows in a filter or filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "visibleBackgroundColorStyle"
    )]
    pub visible_background_color_style: Option<ColorStyle>,
    /**
     * Criteria for showing/hiding rows in a filter or filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "visibleForegroundColor"
    )]
    pub visible_foreground_color: Option<Color>,
    /**
     * Criteria for showing/hiding rows in a filter or filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "visibleForegroundColorStyle"
    )]
    pub visible_foreground_color_style: Option<ColorStyle>,
}

/// The filter criteria associated with a specific column.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FilterSpec {
    /**
     * The filter criteria associated with a specific column.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "columnIndex"
    )]
    pub column_index: i64,
    /**
     * The filter criteria associated with a specific column.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceColumnReference"
    )]
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /**
     * The filter criteria associated with a specific column.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "filterCriteria"
    )]
    pub filter_criteria: Option<FilterCriteria>,
}

/// A filter view.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FilterView {
    /**
     * A filter view.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<FilterCriteria>,
    /**
     * A filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "filterSpecs"
    )]
    pub filter_specs: Vec<FilterSpec>,
    /**
     * A filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "filterViewId"
    )]
    pub filter_view_id: i64,
    /**
     * A filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "namedRangeId"
    )]
    pub named_range_id: String,
    /**
     * A filter view.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * A filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "sortSpecs"
    )]
    pub sort_specs: Vec<SortSpec>,
    /**
     * A filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// Finds and replaces data in cells over a range, sheet, or all sheets.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FindReplaceRequest {
    /**
     * Finds and replaces data in cells over a range, sheet, or all sheets.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allSheets")]
    pub all_sheets: Option<bool>,
    /**
     * Finds and replaces data in cells over a range, sheet, or all sheets.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub find: String,
    /**
     * Finds and replaces data in cells over a range, sheet, or all sheets.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "includeFormulas"
    )]
    pub include_formulas: Option<bool>,
    /**
     * Finds and replaces data in cells over a range, sheet, or all sheets.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchCase")]
    pub match_case: Option<bool>,
    /**
     * Finds and replaces data in cells over a range, sheet, or all sheets.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchEntireCell"
    )]
    pub match_entire_cell: Option<bool>,
    /**
     * Finds and replaces data in cells over a range, sheet, or all sheets.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * Finds and replaces data in cells over a range, sheet, or all sheets.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub replacement: String,
    /**
     * Finds and replaces data in cells over a range, sheet, or all sheets.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "searchByRegex"
    )]
    pub search_by_regex: Option<bool>,
    /**
     * Finds and replaces data in cells over a range, sheet, or all sheets.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/// The result of the find/replace.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct FindReplaceResponse {
    /**
     * The result of the find/replace.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "formulasChanged"
    )]
    pub formulas_changed: i64,
    /**
     * The result of the find/replace.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "occurrencesChanged"
    )]
    pub occurrences_changed: i64,
    /**
     * The result of the find/replace.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "rowsChanged"
    )]
    pub rows_changed: i64,
    /**
     * The result of the find/replace.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetsChanged"
    )]
    pub sheets_changed: i64,
    /**
     * The result of the find/replace.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "valuesChanged"
    )]
    pub values_changed: i64,
}

/// The request for retrieving a Spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GetSpreadsheetByDataFilterRequest {
    /**
     * The request for retrieving a Spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dataFilters"
    )]
    pub data_filters: Vec<DataFilter>,
    /**
     * The request for retrieving a Spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "includeGridData"
    )]
    pub include_grid_data: Option<bool>,
}

/// A rule that applies a gradient color scale format, based on the interpolation points listed. The format of a cell will vary based on its contents as compared to the values of the interpolation points.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GradientRule {
    /**
     * A rule that applies a gradient color scale format, based on the interpolation points listed. The format of a cell will vary based on its contents as compared to the values of the interpolation points.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxpoint: Option<InterpolationPoint>,
    /**
     * A rule that applies a gradient color scale format, based on the interpolation points listed. The format of a cell will vary based on its contents as compared to the values of the interpolation points.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub midpoint: Option<InterpolationPoint>,
    /**
     * A rule that applies a gradient color scale format, based on the interpolation points listed. The format of a cell will vary based on its contents as compared to the values of the interpolation points.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minpoint: Option<InterpolationPoint>,
}

/// A coordinate in a sheet. All indexes are zero-based.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GridCoordinate {
    /**
     * A coordinate in a sheet. All indexes are zero-based.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "columnIndex"
    )]
    pub column_index: i64,
    /**
     * A coordinate in a sheet. All indexes are zero-based.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "rowIndex"
    )]
    pub row_index: i64,
    /**
     * A coordinate in a sheet. All indexes are zero-based.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/// Data in the grid, as well as metadata about the dimensions.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GridData {
    /**
     * Data in the grid, as well as metadata about the dimensions.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "columnMetadata"
    )]
    pub column_metadata: Vec<DimensionProperties>,
    /**
     * Data in the grid, as well as metadata about the dimensions.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "rowData"
    )]
    pub row_data: Vec<RowData>,
    /**
     * Data in the grid, as well as metadata about the dimensions.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "rowMetadata"
    )]
    pub row_metadata: Vec<DimensionProperties>,
    /**
     * Data in the grid, as well as metadata about the dimensions.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "startColumn"
    )]
    pub start_column: i64,
    /**
     * Data in the grid, as well as metadata about the dimensions.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "startRow"
    )]
    pub start_row: i64,
}

/// Properties of a grid.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GridProperties {
    /**
     * Properties of a grid.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "columnCount"
    )]
    pub column_count: i64,
    /**
     * Properties of a grid.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "columnGroupControlAfter"
    )]
    pub column_group_control_after: bool,
    /**
     * Properties of a grid.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "frozenColumnCount"
    )]
    pub frozen_column_count: i64,
    /**
     * Properties of a grid.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "frozenRowCount"
    )]
    pub frozen_row_count: i64,
    /**
     * Properties of a grid.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "hideGridlines"
    )]
    pub hide_gridlines: bool,
    /**
     * Properties of a grid.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "rowCount"
    )]
    pub row_count: i64,
    /**
     * Properties of a grid.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "rowGroupControlAfter"
    )]
    pub row_group_control_after: bool,
}

/// A range on a sheet. All indexes are zero-based. Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [start_index, end_index). Missing indexes indicate the range is unbounded on that side. For example, if `"Sheet1"` is sheet ID 0, then: `Sheet1!A1:A1 == sheet_id: 0, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1` `Sheet1!A3:B4 == sheet_id: 0, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1!A:B == sheet_id: 0, start_column_index: 0, end_column_index: 2` `Sheet1!A5:B == sheet_id: 0, start_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1 == sheet_id:0` The start index must always be less than or equal to the end index. If the start index equals the end index, then the range is empty. Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct GridRange {
    /**
     * A range on a sheet. All indexes are zero-based. Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [start_index, end_index). Missing indexes indicate the range is unbounded on that side. For example, if `"Sheet1"` is sheet ID 0, then: `Sheet1!A1:A1 == sheet_id: 0, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1` `Sheet1!A3:B4 == sheet_id: 0, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1!A:B == sheet_id: 0, start_column_index: 0, end_column_index: 2` `Sheet1!A5:B == sheet_id: 0, start_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1 == sheet_id:0` The start index must always be less than or equal to the end index. If the start index equals the end index, then the range is empty. Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "endColumnIndex"
    )]
    pub end_column_index: i64,
    /**
     * A range on a sheet. All indexes are zero-based. Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [start_index, end_index). Missing indexes indicate the range is unbounded on that side. For example, if `"Sheet1"` is sheet ID 0, then: `Sheet1!A1:A1 == sheet_id: 0, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1` `Sheet1!A3:B4 == sheet_id: 0, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1!A:B == sheet_id: 0, start_column_index: 0, end_column_index: 2` `Sheet1!A5:B == sheet_id: 0, start_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1 == sheet_id:0` The start index must always be less than or equal to the end index. If the start index equals the end index, then the range is empty. Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "endRowIndex"
    )]
    pub end_row_index: i64,
    /**
     * A range on a sheet. All indexes are zero-based. Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [start_index, end_index). Missing indexes indicate the range is unbounded on that side. For example, if `"Sheet1"` is sheet ID 0, then: `Sheet1!A1:A1 == sheet_id: 0, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1` `Sheet1!A3:B4 == sheet_id: 0, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1!A:B == sheet_id: 0, start_column_index: 0, end_column_index: 2` `Sheet1!A5:B == sheet_id: 0, start_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1 == sheet_id:0` The start index must always be less than or equal to the end index. If the start index equals the end index, then the range is empty. Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
    /**
     * A range on a sheet. All indexes are zero-based. Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [start_index, end_index). Missing indexes indicate the range is unbounded on that side. For example, if `"Sheet1"` is sheet ID 0, then: `Sheet1!A1:A1 == sheet_id: 0, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1` `Sheet1!A3:B4 == sheet_id: 0, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1!A:B == sheet_id: 0, start_column_index: 0, end_column_index: 2` `Sheet1!A5:B == sheet_id: 0, start_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1 == sheet_id:0` The start index must always be less than or equal to the end index. If the start index equals the end index, then the range is empty. Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "startColumnIndex"
    )]
    pub start_column_index: i64,
    /**
     * A range on a sheet. All indexes are zero-based. Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [start_index, end_index). Missing indexes indicate the range is unbounded on that side. For example, if `"Sheet1"` is sheet ID 0, then: `Sheet1!A1:A1 == sheet_id: 0, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1` `Sheet1!A3:B4 == sheet_id: 0, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1!A:B == sheet_id: 0, start_column_index: 0, end_column_index: 2` `Sheet1!A5:B == sheet_id: 0, start_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1 == sheet_id:0` The start index must always be less than or equal to the end index. If the start index equals the end index, then the range is empty. Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "startRowIndex"
    )]
    pub start_row_index: i64,
}

/**
* The position of the chart legend.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum HistogramChartSpecLegendPosition {
    #[serde(rename = "BOTTOM_LEGEND")]
    BottomLegend,
    #[serde(rename = "HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED")]
    HistogramChartLegendPositionUnspecified,
    #[serde(rename = "INSIDE_LEGEND")]
    InsideLegend,
    #[serde(rename = "LEFT_LEGEND")]
    LeftLegend,
    #[serde(rename = "NO_LEGEND")]
    NoLegend,
    #[serde(rename = "RIGHT_LEGEND")]
    RightLegend,
    #[serde(rename = "TOP_LEGEND")]
    TopLegend,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for HistogramChartSpecLegendPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistogramChartSpecLegendPosition::BottomLegend => "BOTTOM_LEGEND",
            HistogramChartSpecLegendPosition::HistogramChartLegendPositionUnspecified => {
                "HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED"
            }
            HistogramChartSpecLegendPosition::InsideLegend => "INSIDE_LEGEND",
            HistogramChartSpecLegendPosition::LeftLegend => "LEFT_LEGEND",
            HistogramChartSpecLegendPosition::NoLegend => "NO_LEGEND",
            HistogramChartSpecLegendPosition::RightLegend => "RIGHT_LEGEND",
            HistogramChartSpecLegendPosition::TopLegend => "TOP_LEGEND",
            HistogramChartSpecLegendPosition::Noop => "",
            HistogramChartSpecLegendPosition::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for HistogramChartSpecLegendPosition {
    fn default() -> HistogramChartSpecLegendPosition {
        HistogramChartSpecLegendPosition::Noop
    }
}
impl HistogramChartSpecLegendPosition {
    pub fn is_noop(&self) -> bool {
        matches!(self, HistogramChartSpecLegendPosition::Noop)
    }
}

/// A histogram chart. A histogram chart groups data items into bins, displaying each bin as a column of stacked items. Histograms are used to display the distribution of a dataset. Each column of items represents a range into which those items fall. The number of bins can be chosen automatically or specified explicitly.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct HistogramChartSpec {
    /**
     * A histogram chart. A histogram chart groups data items into bins, displaying each bin as a column of stacked items. Histograms are used to display the distribution of a dataset. Each column of items represents a range into which those items fall. The number of bins can be chosen automatically or specified explicitly.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "bucketSize"
    )]
    pub bucket_size: f64,
    /**
     * A histogram chart. A histogram chart groups data items into bins, displaying each bin as a column of stacked items. Histograms are used to display the distribution of a dataset. Each column of items represents a range into which those items fall. The number of bins can be chosen automatically or specified explicitly.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "legendPosition"
    )]
    pub legend_position: Option<HistogramChartSpecLegendPosition>,
    /**
     * A histogram chart. A histogram chart groups data items into bins, displaying each bin as a column of stacked items. Histograms are used to display the distribution of a dataset. Each column of items represents a range into which those items fall. The number of bins can be chosen automatically or specified explicitly.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "outlierPercentile"
    )]
    pub outlier_percentile: f64,
    /**
     * A histogram chart. A histogram chart groups data items into bins, displaying each bin as a column of stacked items. Histograms are used to display the distribution of a dataset. Each column of items represents a range into which those items fall. The number of bins can be chosen automatically or specified explicitly.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub series: Vec<HistogramSeries>,
    /**
     * A histogram chart. A histogram chart groups data items into bins, displaying each bin as a column of stacked items. Histograms are used to display the distribution of a dataset. Each column of items represents a range into which those items fall. The number of bins can be chosen automatically or specified explicitly.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "showItemDividers"
    )]
    pub show_item_dividers: bool,
}

/// Allows you to organize the numeric values in a source data column into buckets of a constant size. All values from HistogramRule.start to HistogramRule.end are placed into groups of size HistogramRule.interval. In addition, all values below HistogramRule.start are placed in one group, and all values above HistogramRule.end are placed in another. Only HistogramRule.interval is required, though if HistogramRule.start and HistogramRule.end are both provided, HistogramRule.start must be less than HistogramRule.end. For example, a pivot table showing average purchase amount by age that has 50+ rows: +-----+-------------------+ | Age | AVERAGE of Amount | +-----+-------------------+ | 16 | $27.13 | | 17 | $5.24 | | 18 | $20.15 | ... +-----+-------------------+ could be turned into a pivot table that looks like the one below by applying a histogram group rule with a HistogramRule.start of 25, an HistogramRule.interval of 20, and an HistogramRule.end of 65. +-------------+-------------------+ | Grouped Age | AVERAGE of Amount | +-------------+-------------------+ | < 25 | $19.34 | | 25-45 | $31.43 | | 45-65 | $35.87 | | > 65 | $27.55 | +-------------+-------------------+ | Grand Total | $29.12 | +-------------+-------------------+
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct HistogramRule {
    /**
     * Allows you to organize the numeric values in a source data column into buckets of a constant size. All values from HistogramRule.start to HistogramRule.end are placed into groups of size HistogramRule.interval. In addition, all values below HistogramRule.start are placed in one group, and all values above HistogramRule.end are placed in another. Only HistogramRule.interval is required, though if HistogramRule.start and HistogramRule.end are both provided, HistogramRule.start must be less than HistogramRule.end. For example, a pivot table showing average purchase amount by age that has 50+ rows: +-----+-------------------+ | Age | AVERAGE of Amount | +-----+-------------------+ | 16 | $27.13 | | 17 | $5.24 | | 18 | $20.15 | ... +-----+-------------------+ could be turned into a pivot table that looks like the one below by applying a histogram group rule with a HistogramRule.start of 25, an HistogramRule.interval of 20, and an HistogramRule.end of 65. +-------------+-------------------+ | Grouped Age | AVERAGE of Amount | +-------------+-------------------+ | < 25 | $19.34 | | 25-45 | $31.43 | | 45-65 | $35.87 | | > 65 | $27.55 | +-------------+-------------------+ | Grand Total | $29.12 | +-------------+-------------------+
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub end: f64,
    /**
     * Allows you to organize the numeric values in a source data column into buckets of a constant size. All values from HistogramRule.start to HistogramRule.end are placed into groups of size HistogramRule.interval. In addition, all values below HistogramRule.start are placed in one group, and all values above HistogramRule.end are placed in another. Only HistogramRule.interval is required, though if HistogramRule.start and HistogramRule.end are both provided, HistogramRule.start must be less than HistogramRule.end. For example, a pivot table showing average purchase amount by age that has 50+ rows: +-----+-------------------+ | Age | AVERAGE of Amount | +-----+-------------------+ | 16 | $27.13 | | 17 | $5.24 | | 18 | $20.15 | ... +-----+-------------------+ could be turned into a pivot table that looks like the one below by applying a histogram group rule with a HistogramRule.start of 25, an HistogramRule.interval of 20, and an HistogramRule.end of 65. +-------------+-------------------+ | Grouped Age | AVERAGE of Amount | +-------------+-------------------+ | < 25 | $19.34 | | 25-45 | $31.43 | | 45-65 | $35.87 | | > 65 | $27.55 | +-------------+-------------------+ | Grand Total | $29.12 | +-------------+-------------------+
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub interval: f64,
    /**
     * Allows you to organize the numeric values in a source data column into buckets of a constant size. All values from HistogramRule.start to HistogramRule.end are placed into groups of size HistogramRule.interval. In addition, all values below HistogramRule.start are placed in one group, and all values above HistogramRule.end are placed in another. Only HistogramRule.interval is required, though if HistogramRule.start and HistogramRule.end are both provided, HistogramRule.start must be less than HistogramRule.end. For example, a pivot table showing average purchase amount by age that has 50+ rows: +-----+-------------------+ | Age | AVERAGE of Amount | +-----+-------------------+ | 16 | $27.13 | | 17 | $5.24 | | 18 | $20.15 | ... +-----+-------------------+ could be turned into a pivot table that looks like the one below by applying a histogram group rule with a HistogramRule.start of 25, an HistogramRule.interval of 20, and an HistogramRule.end of 65. +-------------+-------------------+ | Grouped Age | AVERAGE of Amount | +-------------+-------------------+ | < 25 | $19.34 | | 25-45 | $31.43 | | 45-65 | $35.87 | | > 65 | $27.55 | +-------------+-------------------+ | Grand Total | $29.12 | +-------------+-------------------+
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub start: f64,
}

/// A histogram series containing the series color and data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct HistogramSeries {
    /**
     * A histogram series containing the series color and data.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "barColor")]
    pub bar_color: Option<Color>,
    /**
     * A histogram series containing the series color and data.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "barColorStyle"
    )]
    pub bar_color_style: Option<ColorStyle>,
    /**
     * A histogram series containing the series color and data.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ChartData>,
}

/// Inserts rows or columns in a sheet at a particular index.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InsertDimensionRequest {
    /**
     * Inserts rows or columns in a sheet at a particular index.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "inheritFromBefore"
    )]
    pub inherit_from_before: Option<bool>,
    /**
     * Inserts rows or columns in a sheet at a particular index.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<DimensionRange>,
}

/// Inserts cells into a range, shifting the existing cells over or down.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InsertRangeRequest {
    /**
     * Inserts cells into a range, shifting the existing cells over or down.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * Inserts cells into a range, shifting the existing cells over or down.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "shiftDimension"
    )]
    pub shift_dimension: Option<Dimension>,
}

/**
* How the value should be interpreted.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum InterpolationPointType {
    #[serde(rename = "INTERPOLATION_POINT_TYPE_UNSPECIFIED")]
    InterpolationPointTypeUnspecified,
    #[serde(rename = "MAX")]
    Max,
    #[serde(rename = "MIN")]
    Min,
    #[serde(rename = "NUMBER")]
    Number,
    #[serde(rename = "PERCENT")]
    Percent,
    #[serde(rename = "PERCENTILE")]
    Percentile,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for InterpolationPointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpolationPointType::InterpolationPointTypeUnspecified => {
                "INTERPOLATION_POINT_TYPE_UNSPECIFIED"
            }
            InterpolationPointType::Max => "MAX",
            InterpolationPointType::Min => "MIN",
            InterpolationPointType::Number => "NUMBER",
            InterpolationPointType::Percent => "PERCENT",
            InterpolationPointType::Percentile => "PERCENTILE",
            InterpolationPointType::Noop => "",
            InterpolationPointType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for InterpolationPointType {
    fn default() -> InterpolationPointType {
        InterpolationPointType::Noop
    }
}
impl InterpolationPointType {
    pub fn is_noop(&self) -> bool {
        matches!(self, InterpolationPointType::Noop)
    }
}

/// A single interpolation point on a gradient conditional format. These pin the gradient color scale according to the color, type and value chosen.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InterpolationPoint {
    /**
     * A single interpolation point on a gradient conditional format. These pin the gradient color scale according to the color, type and value chosen.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /**
     * A single interpolation point on a gradient conditional format. These pin the gradient color scale according to the color, type and value chosen.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "colorStyle"
    )]
    pub color_style: Option<ColorStyle>,
    /**
     * A single interpolation point on a gradient conditional format. These pin the gradient color scale according to the color, type and value chosen.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<InterpolationPointType>,
    /**
     * A single interpolation point on a gradient conditional format. These pin the gradient color scale according to the color, type and value chosen.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// Represents a time interval, encoded as a Timestamp start (inclusive) and a Timestamp end (exclusive). The start must be less than or equal to the end. When the start equals the end, the interval is empty (matches no time). When both start and end are unspecified, the interval matches any time.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Interval {
    /**
     * Represents a time interval, encoded as a Timestamp start (inclusive) and a Timestamp end (exclusive). The start must be less than or equal to the end. When the start equals the end, the interval is empty (matches no time). When both start and end are unspecified, the interval matches any time.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "endTime"
    )]
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Represents a time interval, encoded as a Timestamp start (inclusive) and a Timestamp end (exclusive). The start must be less than or equal to the end. When the start equals the end, the interval is empty (matches no time). When both start and end are unspecified, the interval matches any time.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize",
        rename = "startTime"
    )]
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Settings to control how circular dependencies are resolved with iterative calculation.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IterativeCalculationSettings {
    /**
     * Settings to control how circular dependencies are resolved with iterative calculation.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "convergenceThreshold"
    )]
    pub convergence_threshold: f64,
    /**
     * Settings to control how circular dependencies are resolved with iterative calculation.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxIterations"
    )]
    pub max_iterations: i64,
}

/// Formatting options for key value.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct KeyValueFormat {
    /**
     * Formatting options for key value.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<TextPosition>,
    /**
     * Formatting options for key value.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "textFormat"
    )]
    pub text_format: Option<TextFormat>,
}

/**
* The dash type of the line.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum LineStyleType {
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "DOTTED")]
    Dotted,
    #[serde(rename = "INVISIBLE")]
    Invisible,
    #[serde(rename = "LINE_DASH_TYPE_UNSPECIFIED")]
    LineDashTypeUnspecified,
    #[serde(rename = "LONG_DASHED")]
    LongDashed,
    #[serde(rename = "LONG_DASHED_DOTTED")]
    LongDashedDotted,
    #[serde(rename = "MEDIUM_DASHED")]
    MediumDashed,
    #[serde(rename = "MEDIUM_DASHED_DOTTED")]
    MediumDashedDotted,
    #[serde(rename = "SOLID")]
    Solid,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for LineStyleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineStyleType::Custom => "CUSTOM",
            LineStyleType::Dotted => "DOTTED",
            LineStyleType::Invisible => "INVISIBLE",
            LineStyleType::LineDashTypeUnspecified => "LINE_DASH_TYPE_UNSPECIFIED",
            LineStyleType::LongDashed => "LONG_DASHED",
            LineStyleType::LongDashedDotted => "LONG_DASHED_DOTTED",
            LineStyleType::MediumDashed => "MEDIUM_DASHED",
            LineStyleType::MediumDashedDotted => "MEDIUM_DASHED_DOTTED",
            LineStyleType::Solid => "SOLID",
            LineStyleType::Noop => "",
            LineStyleType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for LineStyleType {
    fn default() -> LineStyleType {
        LineStyleType::Noop
    }
}
impl LineStyleType {
    pub fn is_noop(&self) -> bool {
        matches!(self, LineStyleType::Noop)
    }
}

/// Properties that describe the style of a line.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct LineStyle {
    /**
     * Properties that describe the style of a line.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<LineStyleType>,
    /**
     * Properties that describe the style of a line.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub width: i64,
}

/// An external or local reference.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Link {
    /**
     * An external or local reference.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uri: String,
}

/// Allows you to manually organize the values in a source data column into buckets with names of your choosing. For example, a pivot table that aggregates population by state: +-------+-------------------+ | State | SUM of Population | +-------+-------------------+ | AK | 0.7 | | AL | 4.8 | | AR | 2.9 | ... +-------+-------------------+ could be turned into a pivot table that aggregates population by time zone by providing a list of groups (for example, groupName = 'Central', items = ['AL', 'AR', 'IA', ...]) to a manual group rule. Note that a similar effect could be achieved by adding a time zone column to the source data and adjusting the pivot table. +-----------+-------------------+ | Time Zone | SUM of Population | +-----------+-------------------+ | Central | 106.3 | | Eastern | 151.9 | | Mountain | 17.4 | ... +-----------+-------------------+
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ManualRule {
    /**
     * Allows you to manually organize the values in a source data column into buckets with names of your choosing. For example, a pivot table that aggregates population by state: +-------+-------------------+ | State | SUM of Population | +-------+-------------------+ | AK | 0.7 | | AL | 4.8 | | AR | 2.9 | ... +-------+-------------------+ could be turned into a pivot table that aggregates population by time zone by providing a list of groups (for example, groupName = 'Central', items = ['AL', 'AR', 'IA', ...]) to a manual group rule. Note that a similar effect could be achieved by adding a time zone column to the source data and adjusting the pivot table. +-----------+-------------------+ | Time Zone | SUM of Population | +-----------+-------------------+ | Central | 106.3 | | Eastern | 151.9 | | Mountain | 17.4 | ... +-----------+-------------------+
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub groups: Vec<ManualRuleGroup>,
}

/// A group name and a list of items from the source data that should be placed in the group with this name.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ManualRuleGroup {
    /**
     * A group name and a list of items from the source data that should be placed in the group with this name.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupName")]
    pub group_name: Option<ExtendedValue>,
    /**
     * A group name and a list of items from the source data that should be placed in the group with this name.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub items: Vec<ExtendedValue>,
}

/// A developer metadata entry and the data filters specified in the original request that matched it.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MatchedDeveloperMetadata {
    /**
     * A developer metadata entry and the data filters specified in the original request that matched it.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dataFilters"
    )]
    pub data_filters: Vec<DataFilter>,
    /**
     * A developer metadata entry and the data filters specified in the original request that matched it.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "developerMetadata"
    )]
    pub developer_metadata: Option<DeveloperMetadata>,
}

/// A value range that was matched by one or more data filers.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MatchedValueRange {
    /**
     * A value range that was matched by one or more data filers.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dataFilters"
    )]
    pub data_filters: Vec<DataFilter>,
    /**
     * A value range that was matched by one or more data filers.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "valueRange"
    )]
    pub value_range: Option<ValueRange>,
}

/**
* How the cells should be merged.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum MergeType {
    #[serde(rename = "MERGE_ALL")]
    MergeAll,
    #[serde(rename = "MERGE_COLUMNS")]
    MergeColumns,
    #[serde(rename = "MERGE_ROWS")]
    MergeRows,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for MergeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MergeType::MergeAll => "MERGE_ALL",
            MergeType::MergeColumns => "MERGE_COLUMNS",
            MergeType::MergeRows => "MERGE_ROWS",
            MergeType::Noop => "",
            MergeType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for MergeType {
    fn default() -> MergeType {
        MergeType::Noop
    }
}
impl MergeType {
    pub fn is_noop(&self) -> bool {
        matches!(self, MergeType::Noop)
    }
}

/// Merges all cells in the range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MergeCellsRequest {
    /**
     * Merges all cells in the range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mergeType")]
    pub merge_type: Option<MergeType>,
    /**
     * Merges all cells in the range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
}

/// Moves one or more rows or columns.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MoveDimensionRequest {
    /**
     * Moves one or more rows or columns.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "destinationIndex"
    )]
    pub destination_index: i64,
    /**
     * Moves one or more rows or columns.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<DimensionRange>,
}

/// A named range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NamedRange {
    /**
     * A named range.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A named range.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "namedRangeId"
    )]
    pub named_range_id: String,
    /**
     * A named range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
}

/**
* The type of the number format. When writing, this field must be set.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NumberFormatType {
    #[serde(rename = "CURRENCY")]
    Currency,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "DATE_TIME")]
    DateTime,
    #[serde(rename = "NUMBER")]
    Number,
    #[serde(rename = "NUMBER_FORMAT_TYPE_UNSPECIFIED")]
    NumberFormatTypeUnspecified,
    #[serde(rename = "PERCENT")]
    Percent,
    #[serde(rename = "SCIENTIFIC")]
    Scientific,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "TIME")]
    Time,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NumberFormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberFormatType::Currency => "CURRENCY",
            NumberFormatType::Date => "DATE",
            NumberFormatType::DateTime => "DATE_TIME",
            NumberFormatType::Number => "NUMBER",
            NumberFormatType::NumberFormatTypeUnspecified => "NUMBER_FORMAT_TYPE_UNSPECIFIED",
            NumberFormatType::Percent => "PERCENT",
            NumberFormatType::Scientific => "SCIENTIFIC",
            NumberFormatType::Text => "TEXT",
            NumberFormatType::Time => "TIME",
            NumberFormatType::Noop => "",
            NumberFormatType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NumberFormatType {
    fn default() -> NumberFormatType {
        NumberFormatType::Noop
    }
}
impl NumberFormatType {
    pub fn is_noop(&self) -> bool {
        matches!(self, NumberFormatType::Noop)
    }
}

/// The number format of a cell.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct NumberFormat {
    /**
     * The number format of a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pattern: String,
    /**
     * The number format of a cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<NumberFormatType>,
}

/**
* The size of the org chart nodes.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NodeSize {
    #[serde(rename = "LARGE")]
    Large,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "ORG_CHART_LABEL_SIZE_UNSPECIFIED")]
    OrgChartLabelSizeUnspecified,
    #[serde(rename = "SMALL")]
    Small,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NodeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeSize::Large => "LARGE",
            NodeSize::Medium => "MEDIUM",
            NodeSize::OrgChartLabelSizeUnspecified => "ORG_CHART_LABEL_SIZE_UNSPECIFIED",
            NodeSize::Small => "SMALL",
            NodeSize::Noop => "",
            NodeSize::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NodeSize {
    fn default() -> NodeSize {
        NodeSize::Noop
    }
}
impl NodeSize {
    pub fn is_noop(&self) -> bool {
        matches!(self, NodeSize::Noop)
    }
}

/// An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrgChartSpec {
    /**
     * An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<ChartData>,
    /**
     * An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeColor")]
    pub node_color: Option<Color>,
    /**
     * An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nodeColorStyle"
    )]
    pub node_color_style: Option<ColorStyle>,
    /**
     * An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSize")]
    pub node_size: Option<NodeSize>,
    /**
     * An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "parentLabels"
    )]
    pub parent_labels: Option<ChartData>,
    /**
     * An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "selectedNodeColor"
    )]
    pub selected_node_color: Option<Color>,
    /**
     * An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "selectedNodeColorStyle"
    )]
    pub selected_node_color_style: Option<ColorStyle>,
    /**
     * An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tooltips: Option<ChartData>,
}

/// The location an object is overlaid on top of a grid.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OverlayPosition {
    /**
     * The location an object is overlaid on top of a grid.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "anchorCell"
    )]
    pub anchor_cell: Option<GridCoordinate>,
    /**
     * The location an object is overlaid on top of a grid.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "heightPixels"
    )]
    pub height_pixels: i64,
    /**
     * The location an object is overlaid on top of a grid.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "offsetXPixels"
    )]
    pub offset_x_pixels: i64,
    /**
     * The location an object is overlaid on top of a grid.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "offsetYPixels"
    )]
    pub offset_y_pixels: i64,
    /**
     * The location an object is overlaid on top of a grid.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "widthPixels"
    )]
    pub width_pixels: i64,
}

/// The amount of padding around the cell, in pixels. When updating padding, every field must be specified.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Padding {
    /**
     * The amount of padding around the cell, in pixels. When updating padding, every field must be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub bottom: i64,
    /**
     * The amount of padding around the cell, in pixels. When updating padding, every field must be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub left: i64,
    /**
     * The amount of padding around the cell, in pixels. When updating padding, every field must be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub right: i64,
    /**
     * The amount of padding around the cell, in pixels. When updating padding, every field must be specified.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub top: i64,
}

/// Inserts data into the spreadsheet starting at the specified coordinate.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PasteDataRequest {
    /**
     * Inserts data into the spreadsheet starting at the specified coordinate.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinate: Option<GridCoordinate>,
    /**
     * Inserts data into the spreadsheet starting at the specified coordinate.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub data: String,
    /**
     * Inserts data into the spreadsheet starting at the specified coordinate.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub delimiter: String,
    /**
     * Inserts data into the spreadsheet starting at the specified coordinate.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<bool>,
    /**
     * Inserts data into the spreadsheet starting at the specified coordinate.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<PasteType>,
}

/**
* Where the legend of the pie chart should be drawn.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PieChartSpecLegendPosition {
    #[serde(rename = "BOTTOM_LEGEND")]
    BottomLegend,
    #[serde(rename = "LABELED_LEGEND")]
    LabeledLegend,
    #[serde(rename = "LEFT_LEGEND")]
    LeftLegend,
    #[serde(rename = "NO_LEGEND")]
    NoLegend,
    #[serde(rename = "PIE_CHART_LEGEND_POSITION_UNSPECIFIED")]
    PieChartLegendPositionUnspecified,
    #[serde(rename = "RIGHT_LEGEND")]
    RightLegend,
    #[serde(rename = "TOP_LEGEND")]
    TopLegend,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PieChartSpecLegendPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieChartSpecLegendPosition::BottomLegend => "BOTTOM_LEGEND",
            PieChartSpecLegendPosition::LabeledLegend => "LABELED_LEGEND",
            PieChartSpecLegendPosition::LeftLegend => "LEFT_LEGEND",
            PieChartSpecLegendPosition::NoLegend => "NO_LEGEND",
            PieChartSpecLegendPosition::PieChartLegendPositionUnspecified => {
                "PIE_CHART_LEGEND_POSITION_UNSPECIFIED"
            }
            PieChartSpecLegendPosition::RightLegend => "RIGHT_LEGEND",
            PieChartSpecLegendPosition::TopLegend => "TOP_LEGEND",
            PieChartSpecLegendPosition::Noop => "",
            PieChartSpecLegendPosition::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PieChartSpecLegendPosition {
    fn default() -> PieChartSpecLegendPosition {
        PieChartSpecLegendPosition::Noop
    }
}
impl PieChartSpecLegendPosition {
    pub fn is_noop(&self) -> bool {
        matches!(self, PieChartSpecLegendPosition::Noop)
    }
}

/// A pie chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PieChartSpec {
    /**
     * A pie chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<ChartData>,
    /**
     * A pie chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "legendPosition"
    )]
    pub legend_position: Option<PieChartSpecLegendPosition>,
    /**
     * A pie chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "pieHole"
    )]
    pub pie_hole: f64,
    /**
     * A pie chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<ChartData>,
    /**
     * A pie chart.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "threeDimensional"
    )]
    pub three_dimensional: bool,
}

/// Criteria for showing/hiding rows in a pivot table.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PivotFilterCriteria {
    /**
     * Criteria for showing/hiding rows in a pivot table.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<BooleanCondition>,
    /**
     * Criteria for showing/hiding rows in a pivot table.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "visibleByDefault"
    )]
    pub visible_by_default: bool,
    /**
     * Criteria for showing/hiding rows in a pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "visibleValues"
    )]
    pub visible_values: Vec<String>,
}

/// The pivot table filter criteria associated with a specific source column offset.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PivotFilterSpec {
    /**
     * The pivot table filter criteria associated with a specific source column offset.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "columnOffsetIndex"
    )]
    pub column_offset_index: i64,
    /**
     * The pivot table filter criteria associated with a specific source column offset.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceColumnReference"
    )]
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /**
     * The pivot table filter criteria associated with a specific source column offset.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "filterCriteria"
    )]
    pub filter_criteria: Option<PivotFilterCriteria>,
}

/**
* The order the values in this group should be sorted.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SortOrder {
    #[serde(rename = "ASCENDING")]
    Ascending,
    #[serde(rename = "DESCENDING")]
    Descending,
    #[serde(rename = "SORT_ORDER_UNSPECIFIED")]
    SortOrderUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Ascending => "ASCENDING",
            SortOrder::Descending => "DESCENDING",
            SortOrder::SortOrderUnspecified => "SORT_ORDER_UNSPECIFIED",
            SortOrder::Noop => "",
            SortOrder::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SortOrder {
    fn default() -> SortOrder {
        SortOrder::Noop
    }
}
impl SortOrder {
    pub fn is_noop(&self) -> bool {
        matches!(self, SortOrder::Noop)
    }
}

/// A single grouping (either row or column) in a pivot table.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PivotGroup {
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceColumnReference"
    )]
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "groupLimit"
    )]
    pub group_limit: Option<PivotGroupLimit>,
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupRule")]
    pub group_rule: Option<PivotGroupRule>,
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "repeatHeadings"
    )]
    pub repeat_headings: bool,
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "showTotals"
    )]
    pub show_totals: bool,
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sortOrder")]
    pub sort_order: Option<SortOrder>,
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sourceColumnOffset"
    )]
    pub source_column_offset: i64,
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "valueBucket"
    )]
    pub value_bucket: Option<PivotGroupSortValueBucket>,
    /**
     * A single grouping (either row or column) in a pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "valueMetadata"
    )]
    pub value_metadata: Vec<PivotGroupValueMetadata>,
}

/// The count limit on rows or columns in the pivot group.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PivotGroupLimit {
    /**
     * The count limit on rows or columns in the pivot group.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "applyOrder"
    )]
    pub apply_order: i64,
    /**
     * The count limit on rows or columns in the pivot group.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "countLimit"
    )]
    pub count_limit: i64,
}

/// An optional setting on a PivotGroup that defines buckets for the values in the source data column rather than breaking out each individual value. Only one PivotGroup with a group rule may be added for each column in the source data, though on any given column you may add both a PivotGroup that has a rule and a PivotGroup that does not.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PivotGroupRule {
    /**
     * An optional setting on a PivotGroup that defines buckets for the values in the source data column rather than breaking out each individual value. Only one PivotGroup with a group rule may be added for each column in the source data, though on any given column you may add both a PivotGroup that has a rule and a PivotGroup that does not.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dateTimeRule"
    )]
    pub date_time_rule: Option<DateTimeRule>,
    /**
     * An optional setting on a PivotGroup that defines buckets for the values in the source data column rather than breaking out each individual value. Only one PivotGroup with a group rule may be added for each column in the source data, though on any given column you may add both a PivotGroup that has a rule and a PivotGroup that does not.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "histogramRule"
    )]
    pub histogram_rule: Option<HistogramRule>,
    /**
     * An optional setting on a PivotGroup that defines buckets for the values in the source data column rather than breaking out each individual value. Only one PivotGroup with a group rule may be added for each column in the source data, though on any given column you may add both a PivotGroup that has a rule and a PivotGroup that does not.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "manualRule"
    )]
    pub manual_rule: Option<ManualRule>,
}

/// Information about which values in a pivot group should be used for sorting.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PivotGroupSortValueBucket {
    /**
     * Information about which values in a pivot group should be used for sorting.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub buckets: Vec<ExtendedValue>,
    /**
     * Information about which values in a pivot group should be used for sorting.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "valuesIndex"
    )]
    pub values_index: i64,
}

/// Metadata about a value in a pivot grouping.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PivotGroupValueMetadata {
    /**
     * Metadata about a value in a pivot grouping.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub collapsed: bool,
    /**
     * Metadata about a value in a pivot grouping.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ExtendedValue>,
}

/**
* Whether values should be listed horizontally (as columns) or vertically (as rows).
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ValueLayout {
    #[serde(rename = "HORIZONTAL")]
    Horizontal,
    #[serde(rename = "VERTICAL")]
    Vertical,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ValueLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueLayout::Horizontal => "HORIZONTAL",
            ValueLayout::Vertical => "VERTICAL",
            ValueLayout::Noop => "",
            ValueLayout::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ValueLayout {
    fn default() -> ValueLayout {
        ValueLayout::Noop
    }
}
impl ValueLayout {
    pub fn is_noop(&self) -> bool {
        matches!(self, ValueLayout::Noop)
    }
}

/// A pivot table.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PivotTable {
    /**
     * A pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub columns: Vec<PivotGroup>,
    /**
     * A pivot table.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<PivotFilterCriteria>,
    /**
     * A pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataExecutionStatus"
    )]
    pub data_execution_status: Option<DataExecutionStatus>,
    /**
     * A pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dataSourceId"
    )]
    pub data_source_id: String,
    /**
     * A pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "filterSpecs"
    )]
    pub filter_specs: Vec<PivotFilterSpec>,
    /**
     * A pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub rows: Vec<PivotGroup>,
    /**
     * A pivot table.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GridRange>,
    /**
     * A pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "valueLayout"
    )]
    pub value_layout: Option<ValueLayout>,
    /**
     * A pivot table.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub values: Vec<PivotValue>,
}

/**
* If specified, indicates that pivot values should be displayed as the result of a calculation with another pivot value. For example, if calculated_display_type is specified as PERCENT_OF_GRAND_TOTAL, all the pivot values are displayed as the percentage of the grand total. In the Sheets editor, this is referred to as "Show As" in the value section of a pivot table.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CalculatedDisplayType {
    #[serde(rename = "PERCENT_OF_COLUMN_TOTAL")]
    PercentOfColumnTotal,
    #[serde(rename = "PERCENT_OF_GRAND_TOTAL")]
    PercentOfGrandTotal,
    #[serde(rename = "PERCENT_OF_ROW_TOTAL")]
    PercentOfRowTotal,
    #[serde(rename = "PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED")]
    PivotValueCalculatedDisplayTypeUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CalculatedDisplayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculatedDisplayType::PercentOfColumnTotal => "PERCENT_OF_COLUMN_TOTAL",
            CalculatedDisplayType::PercentOfGrandTotal => "PERCENT_OF_GRAND_TOTAL",
            CalculatedDisplayType::PercentOfRowTotal => "PERCENT_OF_ROW_TOTAL",
            CalculatedDisplayType::PivotValueCalculatedDisplayTypeUnspecified => {
                "PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED"
            }
            CalculatedDisplayType::Noop => "",
            CalculatedDisplayType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CalculatedDisplayType {
    fn default() -> CalculatedDisplayType {
        CalculatedDisplayType::Noop
    }
}
impl CalculatedDisplayType {
    pub fn is_noop(&self) -> bool {
        matches!(self, CalculatedDisplayType::Noop)
    }
}

/**
* A function to summarize the value. If formula is set, the only supported values are SUM and CUSTOM. If sourceColumnOffset is set, then `CUSTOM` is not supported.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SummarizeFunction {
    #[serde(rename = "AVERAGE")]
    Average,
    #[serde(rename = "COUNT")]
    Count,
    #[serde(rename = "COUNTA")]
    Counta,
    #[serde(rename = "COUNTUNIQUE")]
    Countunique,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "MAX")]
    Max,
    #[serde(rename = "MEDIAN")]
    Median,
    #[serde(rename = "MIN")]
    Min,
    #[serde(rename = "PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED")]
    PivotStandardValueFunctionUnspecified,
    #[serde(rename = "PRODUCT")]
    Product,
    #[serde(rename = "STDEV")]
    Stdev,
    #[serde(rename = "STDEVP")]
    Stdevp,
    #[serde(rename = "SUM")]
    Sum,
    #[serde(rename = "VAR")]
    Var,
    #[serde(rename = "VARP")]
    Varp,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SummarizeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SummarizeFunction::Average => "AVERAGE",
            SummarizeFunction::Count => "COUNT",
            SummarizeFunction::Counta => "COUNTA",
            SummarizeFunction::Countunique => "COUNTUNIQUE",
            SummarizeFunction::Custom => "CUSTOM",
            SummarizeFunction::Max => "MAX",
            SummarizeFunction::Median => "MEDIAN",
            SummarizeFunction::Min => "MIN",
            SummarizeFunction::PivotStandardValueFunctionUnspecified => {
                "PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED"
            }
            SummarizeFunction::Product => "PRODUCT",
            SummarizeFunction::Stdev => "STDEV",
            SummarizeFunction::Stdevp => "STDEVP",
            SummarizeFunction::Sum => "SUM",
            SummarizeFunction::Var => "VAR",
            SummarizeFunction::Varp => "VARP",
            SummarizeFunction::Noop => "",
            SummarizeFunction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SummarizeFunction {
    fn default() -> SummarizeFunction {
        SummarizeFunction::Noop
    }
}
impl SummarizeFunction {
    pub fn is_noop(&self) -> bool {
        matches!(self, SummarizeFunction::Noop)
    }
}

/// The definition of how a value in a pivot table should be calculated.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PivotValue {
    /**
     * The definition of how a value in a pivot table should be calculated.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "calculatedDisplayType"
    )]
    pub calculated_display_type: Option<CalculatedDisplayType>,
    /**
     * The definition of how a value in a pivot table should be calculated.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceColumnReference"
    )]
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /**
     * The definition of how a value in a pivot table should be calculated.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub formula: String,
    /**
     * The definition of how a value in a pivot table should be calculated.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The definition of how a value in a pivot table should be calculated.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sourceColumnOffset"
    )]
    pub source_column_offset: i64,
    /**
     * The definition of how a value in a pivot table should be calculated.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "summarizeFunction"
    )]
    pub summarize_function: Option<SummarizeFunction>,
}

/**
* The point shape. If empty or unspecified, a default shape is used.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Shape {
    #[serde(rename = "CIRCLE")]
    Circle,
    #[serde(rename = "DIAMOND")]
    Diamond,
    #[serde(rename = "HEXAGON")]
    Hexagon,
    #[serde(rename = "PENTAGON")]
    Pentagon,
    #[serde(rename = "POINT_SHAPE_UNSPECIFIED")]
    PointShapeUnspecified,
    #[serde(rename = "SQUARE")]
    Square,
    #[serde(rename = "STAR")]
    Star,
    #[serde(rename = "TRIANGLE")]
    Triangle,
    #[serde(rename = "X_MARK")]
    XMark,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle => "CIRCLE",
            Shape::Diamond => "DIAMOND",
            Shape::Hexagon => "HEXAGON",
            Shape::Pentagon => "PENTAGON",
            Shape::PointShapeUnspecified => "POINT_SHAPE_UNSPECIFIED",
            Shape::Square => "SQUARE",
            Shape::Star => "STAR",
            Shape::Triangle => "TRIANGLE",
            Shape::XMark => "X_MARK",
            Shape::Noop => "",
            Shape::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Shape {
    fn default() -> Shape {
        Shape::Noop
    }
}
impl Shape {
    pub fn is_noop(&self) -> bool {
        matches!(self, Shape::Noop)
    }
}

/// The style of a point on the chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct PointStyle {
    /**
     * The style of a point on the chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<Shape>,
    /**
     * The style of a point on the chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub size: f64,
}

/// A protected range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProtectedRange {
    /**
     * A protected range.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * A protected range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editors: Option<Editors>,
    /**
     * A protected range.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "namedRangeId"
    )]
    pub named_range_id: String,
    /**
     * A protected range.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "protectedRangeId"
    )]
    pub protected_range_id: i64,
    /**
     * A protected range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * A protected range.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "requestingUserCanEdit"
    )]
    pub requesting_user_can_edit: bool,
    /**
     * A protected range.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "unprotectedRanges"
    )]
    pub unprotected_ranges: Vec<GridRange>,
    /**
     * A protected range.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "warningOnly"
    )]
    pub warning_only: bool,
}

/// Randomizes the order of the rows in a range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RandomizeRangeRequest {
    /**
     * Randomizes the order of the rows in a range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
}

/// The execution status of refreshing one data source object.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RefreshDataSourceObjectExecutionStatus {
    /**
     * The execution status of refreshing one data source object.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataExecutionStatus"
    )]
    pub data_execution_status: Option<DataExecutionStatus>,
    /**
     * The execution status of refreshing one data source object.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<DataSourceObjectReference>,
}

/// Refreshes one or multiple data source objects in the spreadsheet by the specified references. The request requires an additional `bigquery.readonly` OAuth scope. If there are multiple refresh requests referencing the same data source objects in one batch, only the last refresh request is processed, and all those requests will have the same response accordingly.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RefreshDataSourceRequest {
    /**
     * Refreshes one or multiple data source objects in the spreadsheet by the specified references. The request requires an additional `bigquery.readonly` OAuth scope. If there are multiple refresh requests referencing the same data source objects in one batch, only the last refresh request is processed, and all those requests will have the same response accordingly.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "dataSourceId"
    )]
    pub data_source_id: String,
    /**
     * Refreshes one or multiple data source objects in the spreadsheet by the specified references. The request requires an additional `bigquery.readonly` OAuth scope. If there are multiple refresh requests referencing the same data source objects in one batch, only the last refresh request is processed, and all those requests will have the same response accordingly.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /**
     * Refreshes one or multiple data source objects in the spreadsheet by the specified references. The request requires an additional `bigquery.readonly` OAuth scope. If there are multiple refresh requests referencing the same data source objects in one batch, only the last refresh request is processed, and all those requests will have the same response accordingly.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isAll")]
    pub is_all: Option<bool>,
    /**
     * Refreshes one or multiple data source objects in the spreadsheet by the specified references. The request requires an additional `bigquery.readonly` OAuth scope. If there are multiple refresh requests referencing the same data source objects in one batch, only the last refresh request is processed, and all those requests will have the same response accordingly.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references: Option<DataSourceObjectReferences>,
}

/// The response from refreshing one or multiple data source objects.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RefreshDataSourceResponse {
    /**
     * The response from refreshing one or multiple data source objects.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub statuses: Vec<RefreshDataSourceObjectExecutionStatus>,
}

/// Updates all cells in the range to the values in the given Cell object. Only the fields listed in the fields field are updated; others are unchanged. If writing a cell with a formula, the formula's ranges will automatically increment for each field in the range. For example, if writing a cell with formula `=A1` into range B2:C4, B2 would be `=A1`, B3 would be `=A2`, B4 would be `=A3`, C2 would be `=B1`, C3 would be `=B2`, C4 would be `=B3`. To keep the formula's ranges static, use the `$` indicator. For example, use the formula `=$A$1` to prevent both the row and the column from incrementing.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RepeatCellRequest {
    /**
     * Updates all cells in the range to the values in the given Cell object. Only the fields listed in the fields field are updated; others are unchanged. If writing a cell with a formula, the formula's ranges will automatically increment for each field in the range. For example, if writing a cell with formula `=A1` into range B2:C4, B2 would be `=A1`, B3 would be `=A2`, B4 would be `=A3`, C2 would be `=B1`, C3 would be `=B2`, C4 would be `=B3`. To keep the formula's ranges static, use the `$` indicator. For example, use the formula `=$A$1` to prevent both the row and the column from incrementing.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell: Option<CellData>,
    /**
     * Updates all cells in the range to the values in the given Cell object. Only the fields listed in the fields field are updated; others are unchanged. If writing a cell with a formula, the formula's ranges will automatically increment for each field in the range. For example, if writing a cell with formula `=A1` into range B2:C4, B2 would be `=A1`, B3 would be `=A2`, B4 would be `=A3`, C2 would be `=B1`, C3 would be `=B2`, C4 would be `=B3`. To keep the formula's ranges static, use the `$` indicator. For example, use the formula `=$A$1` to prevent both the row and the column from incrementing.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates all cells in the range to the values in the given Cell object. Only the fields listed in the fields field are updated; others are unchanged. If writing a cell with a formula, the formula's ranges will automatically increment for each field in the range. For example, if writing a cell with formula `=A1` into range B2:C4, B2 would be `=A1`, B3 would be `=A2`, B4 would be `=A3`, C2 would be `=B1`, C3 would be `=B2`, C4 would be `=B3`. To keep the formula's ranges static, use the `$` indicator. For example, use the formula `=$A$1` to prevent both the row and the column from incrementing.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
}

/// A single kind of update to apply to a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Request {
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addBanding"
    )]
    pub add_banding: Option<AddBandingRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addChart")]
    pub add_chart: Option<AddChartRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addConditionalFormatRule"
    )]
    pub add_conditional_format_rule: Option<AddConditionalFormatRuleRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addDataSource"
    )]
    pub add_data_source: Option<AddDataSourceRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addDimensionGroup"
    )]
    pub add_dimension_group: Option<AddDimensionGroupRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addFilterView"
    )]
    pub add_filter_view: Option<AddFilterViewRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addNamedRange"
    )]
    pub add_named_range: Option<AddNamedRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addProtectedRange"
    )]
    pub add_protected_range: Option<AddProtectedRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addSheet")]
    pub add_sheet: Option<AddSheetRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addSlicer")]
    pub add_slicer: Option<AddSlicerRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "appendCells"
    )]
    pub append_cells: Option<AppendCellsRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "appendDimension"
    )]
    pub append_dimension: Option<AppendDimensionRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoFill")]
    pub auto_fill: Option<AutoFillRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "autoResizeDimensions"
    )]
    pub auto_resize_dimensions: Option<AutoResizeDimensionsRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "clearBasicFilter"
    )]
    pub clear_basic_filter: Option<ClearBasicFilterRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "copyPaste")]
    pub copy_paste: Option<CopyPasteRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "createDeveloperMetadata"
    )]
    pub create_developer_metadata: Option<CreateDeveloperMetadataRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cutPaste")]
    pub cut_paste: Option<CutPasteRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteBanding"
    )]
    pub delete_banding: Option<DeleteBandingRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteConditionalFormatRule"
    )]
    pub delete_conditional_format_rule: Option<DeleteConditionalFormatRuleRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteDataSource"
    )]
    pub delete_data_source: Option<DeleteDataSourceRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteDeveloperMetadata"
    )]
    pub delete_developer_metadata: Option<DeleteDeveloperMetadataRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteDimension"
    )]
    pub delete_dimension: Option<DeleteDimensionRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteDimensionGroup"
    )]
    pub delete_dimension_group: Option<DeleteDimensionGroupRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteDuplicates"
    )]
    pub delete_duplicates: Option<DeleteDuplicatesRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteEmbeddedObject"
    )]
    pub delete_embedded_object: Option<DeleteEmbeddedObjectRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteFilterView"
    )]
    pub delete_filter_view: Option<DeleteFilterViewRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteNamedRange"
    )]
    pub delete_named_range: Option<DeleteNamedRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteProtectedRange"
    )]
    pub delete_protected_range: Option<DeleteProtectedRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteRange"
    )]
    pub delete_range: Option<DeleteRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteSheet"
    )]
    pub delete_sheet: Option<DeleteSheetRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "duplicateFilterView"
    )]
    pub duplicate_filter_view: Option<DuplicateFilterViewRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "duplicateSheet"
    )]
    pub duplicate_sheet: Option<DuplicateSheetRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "findReplace"
    )]
    pub find_replace: Option<FindReplaceRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "insertDimension"
    )]
    pub insert_dimension: Option<InsertDimensionRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "insertRange"
    )]
    pub insert_range: Option<InsertRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mergeCells"
    )]
    pub merge_cells: Option<MergeCellsRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "moveDimension"
    )]
    pub move_dimension: Option<MoveDimensionRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pasteData")]
    pub paste_data: Option<PasteDataRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "randomizeRange"
    )]
    pub randomize_range: Option<RandomizeRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refreshDataSource"
    )]
    pub refresh_data_source: Option<RefreshDataSourceRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "repeatCell"
    )]
    pub repeat_cell: Option<RepeatCellRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "setBasicFilter"
    )]
    pub set_basic_filter: Option<SetBasicFilterRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "setDataValidation"
    )]
    pub set_data_validation: Option<SetDataValidationRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sortRange")]
    pub sort_range: Option<SortRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "textToColumns"
    )]
    pub text_to_columns: Option<TextColumnsRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trimWhitespace"
    )]
    pub trim_whitespace: Option<TrimWhitespaceRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "unmergeCells"
    )]
    pub unmerge_cells: Option<UnmergeCellsRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateBanding"
    )]
    pub update_banding: Option<UpdateBandingRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateBorders"
    )]
    pub update_borders: Option<UpdateBordersRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateCells"
    )]
    pub update_cells: Option<UpdateCellsRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateChartSpec"
    )]
    pub update_chart_spec: Option<UpdateChartSpecRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateConditionalFormatRule"
    )]
    pub update_conditional_format_rule: Option<UpdateConditionalFormatRuleRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateDataSource"
    )]
    pub update_data_source: Option<UpdateDataSourceRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateDeveloperMetadata"
    )]
    pub update_developer_metadata: Option<UpdateDeveloperMetadataRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateDimensionGroup"
    )]
    pub update_dimension_group: Option<UpdateDimensionGroupRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateDimensionProperties"
    )]
    pub update_dimension_properties: Option<UpdateDimensionPropertiesRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateEmbeddedObjectBorder"
    )]
    pub update_embedded_object_border: Option<UpdateEmbeddedObjectBorderRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateEmbeddedObjectPosition"
    )]
    pub update_embedded_object_position: Option<UpdateEmbeddedObjectPositionRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateFilterView"
    )]
    pub update_filter_view: Option<UpdateFilterViewRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateNamedRange"
    )]
    pub update_named_range: Option<UpdateNamedRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateProtectedRange"
    )]
    pub update_protected_range: Option<UpdateProtectedRangeRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateSheetProperties"
    )]
    pub update_sheet_properties: Option<UpdateSheetPropertiesRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateSlicerSpec"
    )]
    pub update_slicer_spec: Option<UpdateSlicerSpecRequest>,
    /**
     * A single kind of update to apply to a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateSpreadsheetProperties"
    )]
    pub update_spreadsheet_properties: Option<UpdateSpreadsheetPropertiesRequest>,
}

/// A single response from an update.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Response {
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addBanding"
    )]
    pub add_banding: Option<AddBandingResponse>,
    /**
     * A single response from an update.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addChart")]
    pub add_chart: Option<AddChartResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addDataSource"
    )]
    pub add_data_source: Option<AddDataSourceResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addDimensionGroup"
    )]
    pub add_dimension_group: Option<AddDimensionGroupResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addFilterView"
    )]
    pub add_filter_view: Option<AddFilterViewResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addNamedRange"
    )]
    pub add_named_range: Option<AddNamedRangeResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addProtectedRange"
    )]
    pub add_protected_range: Option<AddProtectedRangeResponse>,
    /**
     * A single response from an update.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addSheet")]
    pub add_sheet: Option<AddSheetResponse>,
    /**
     * A single response from an update.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addSlicer")]
    pub add_slicer: Option<AddSlicerResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "createDeveloperMetadata"
    )]
    pub create_developer_metadata: Option<CreateDeveloperMetadataResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteConditionalFormatRule"
    )]
    pub delete_conditional_format_rule: Option<DeleteConditionalFormatRuleResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteDeveloperMetadata"
    )]
    pub delete_developer_metadata: Option<DeleteDeveloperMetadataResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteDimensionGroup"
    )]
    pub delete_dimension_group: Option<DeleteDimensionGroupResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deleteDuplicates"
    )]
    pub delete_duplicates: Option<DeleteDuplicatesResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "duplicateFilterView"
    )]
    pub duplicate_filter_view: Option<DuplicateFilterViewResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "duplicateSheet"
    )]
    pub duplicate_sheet: Option<DuplicateSheetResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "findReplace"
    )]
    pub find_replace: Option<FindReplaceResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refreshDataSource"
    )]
    pub refresh_data_source: Option<RefreshDataSourceResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trimWhitespace"
    )]
    pub trim_whitespace: Option<TrimWhitespaceResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateConditionalFormatRule"
    )]
    pub update_conditional_format_rule: Option<UpdateConditionalFormatRuleResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateDataSource"
    )]
    pub update_data_source: Option<UpdateDataSourceResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateDeveloperMetadata"
    )]
    pub update_developer_metadata: Option<UpdateDeveloperMetadataResponse>,
    /**
     * A single response from an update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updateEmbeddedObjectPosition"
    )]
    pub update_embedded_object_position: Option<UpdateEmbeddedObjectPositionResponse>,
}

/// Data about each cell in a row.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct RowData {
    /**
     * Data about each cell in a row.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub values: Vec<CellData>,
}

/**
* The number format source used in the scorecard chart. This field is optional.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum NumberFormatSource {
    #[serde(rename = "CHART_NUMBER_FORMAT_SOURCE_UNDEFINED")]
    ChartNumberFormatSourceUndefined,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "FROM_DATA")]
    FromData,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for NumberFormatSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberFormatSource::ChartNumberFormatSourceUndefined => {
                "CHART_NUMBER_FORMAT_SOURCE_UNDEFINED"
            }
            NumberFormatSource::Custom => "CUSTOM",
            NumberFormatSource::FromData => "FROM_DATA",
            NumberFormatSource::Noop => "",
            NumberFormatSource::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for NumberFormatSource {
    fn default() -> NumberFormatSource {
        NumberFormatSource::Noop
    }
}
impl NumberFormatSource {
    pub fn is_noop(&self) -> bool {
        matches!(self, NumberFormatSource::Noop)
    }
}

/// A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ScorecardChartSpec {
    /**
     * A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "aggregateType"
    )]
    pub aggregate_type: Option<AggregateType>,
    /**
     * A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "baselineValueData"
    )]
    pub baseline_value_data: Option<ChartData>,
    /**
     * A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "baselineValueFormat"
    )]
    pub baseline_value_format: Option<BaselineValueFormat>,
    /**
     * A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "customFormatOptions"
    )]
    pub custom_format_options: Option<ChartCustomNumberFormatOptions>,
    /**
     * A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "keyValueData"
    )]
    pub key_value_data: Option<ChartData>,
    /**
     * A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "keyValueFormat"
    )]
    pub key_value_format: Option<KeyValueFormat>,
    /**
     * A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "numberFormatSource"
    )]
    pub number_format_source: Option<NumberFormatSource>,
    /**
     * A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "scaleFactor"
    )]
    pub scale_factor: f64,
}

/// A request to retrieve all developer metadata matching the set of specified criteria.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SearchDeveloperMetadataRequest {
    /**
     * A request to retrieve all developer metadata matching the set of specified criteria.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dataFilters"
    )]
    pub data_filters: Vec<DataFilter>,
}

/// A reply to a developer metadata search request.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SearchDeveloperMetadataResponse {
    /**
     * A reply to a developer metadata search request.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "matchedDeveloperMetadata"
    )]
    pub matched_developer_metadata: Vec<MatchedDeveloperMetadata>,
}

/// Sets the basic filter associated with a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SetBasicFilterRequest {
    /**
     * Sets the basic filter associated with a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<BasicFilter>,
}

/// Sets a data validation rule to every cell in the range. To clear validation in a range, call this with no rule specified.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SetDataValidationRequest {
    /**
     * Sets a data validation rule to every cell in the range. To clear validation in a range, call this with no rule specified.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * Sets a data validation rule to every cell in the range. To clear validation in a range, call this with no rule specified.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<DataValidationRule>,
}

/// A sheet in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Sheet {
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "bandedRanges"
    )]
    pub banded_ranges: Vec<BandedRange>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "basicFilter"
    )]
    pub basic_filter: Option<BasicFilter>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub charts: Vec<EmbeddedChart>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "columnGroups"
    )]
    pub column_groups: Vec<DimensionGroup>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "conditionalFormats"
    )]
    pub conditional_formats: Vec<ConditionalFormatRule>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub data: Vec<GridData>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "developerMetadata"
    )]
    pub developer_metadata: Vec<DeveloperMetadata>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "filterViews"
    )]
    pub filter_views: Vec<FilterView>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub merges: Vec<GridRange>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SheetProperties>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "protectedRanges"
    )]
    pub protected_ranges: Vec<ProtectedRange>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "rowGroups"
    )]
    pub row_groups: Vec<DimensionGroup>,
    /**
     * A sheet in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub slicers: Vec<Slicer>,
}

/**
* The type of sheet. Defaults to GRID. This field cannot be changed once set.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SheetType {
    #[serde(rename = "DATA_SOURCE")]
    DataSource,
    #[serde(rename = "GRID")]
    Grid,
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "SHEET_TYPE_UNSPECIFIED")]
    SheetTypeUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SheetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SheetType::DataSource => "DATA_SOURCE",
            SheetType::Grid => "GRID",
            SheetType::Object => "OBJECT",
            SheetType::SheetTypeUnspecified => "SHEET_TYPE_UNSPECIFIED",
            SheetType::Noop => "",
            SheetType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for SheetType {
    fn default() -> SheetType {
        SheetType::Noop
    }
}
impl SheetType {
    pub fn is_noop(&self) -> bool {
        matches!(self, SheetType::Noop)
    }
}

/// Properties of a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SheetProperties {
    /**
     * Properties of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceSheetProperties"
    )]
    pub data_source_sheet_properties: Option<DataSourceSheetProperties>,
    /**
     * Properties of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "gridProperties"
    )]
    pub grid_properties: Option<GridProperties>,
    /**
     * Properties of a sheet.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub hidden: bool,
    /**
     * Properties of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub index: i64,
    /**
     * Properties of a sheet.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "rightToLeft"
    )]
    pub right_to_left: bool,
    /**
     * Properties of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
    /**
     * Properties of a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sheetType")]
    pub sheet_type: Option<SheetType>,
    /**
     * Properties of a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tabColor")]
    pub tab_color: Option<Color>,
    /**
     * Properties of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "tabColorStyle"
    )]
    pub tab_color_style: Option<ColorStyle>,
    /**
     * Properties of a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// A slicer in a sheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Slicer {
    /**
     * A slicer in a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<EmbeddedObjectPosition>,
    /**
     * A slicer in a sheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "slicerId"
    )]
    pub slicer_id: i64,
    /**
     * A slicer in a sheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<SlicerSpec>,
}

/// The specifications of a slicer.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SlicerSpec {
    /**
     * The specifications of a slicer.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "applyToPivotTables"
    )]
    pub apply_to_pivot_tables: bool,
    /**
     * The specifications of a slicer.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundColor"
    )]
    pub background_color: Option<Color>,
    /**
     * The specifications of a slicer.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundColorStyle"
    )]
    pub background_color_style: Option<ColorStyle>,
    /**
     * The specifications of a slicer.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "columnIndex"
    )]
    pub column_index: i64,
    /**
     * The specifications of a slicer.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataRange")]
    pub data_range: Option<GridRange>,
    /**
     * The specifications of a slicer.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "filterCriteria"
    )]
    pub filter_criteria: Option<FilterCriteria>,
    /**
     * The specifications of a slicer.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "horizontalAlignment"
    )]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    /**
     * The specifications of a slicer.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "textFormat"
    )]
    pub text_format: Option<TextFormat>,
    /**
     * The specifications of a slicer.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// Sorts data in rows based on a sort order per column.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SortRangeRequest {
    /**
     * Sorts data in rows based on a sort order per column.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * Sorts data in rows based on a sort order per column.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "sortSpecs"
    )]
    pub sort_specs: Vec<SortSpec>,
}

/// A sort order associated with a specific column or row.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SortSpec {
    /**
     * A sort order associated with a specific column or row.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundColor"
    )]
    pub background_color: Option<Color>,
    /**
     * A sort order associated with a specific column or row.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backgroundColorStyle"
    )]
    pub background_color_style: Option<ColorStyle>,
    /**
     * A sort order associated with a specific column or row.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceColumnReference"
    )]
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /**
     * A sort order associated with a specific column or row.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "dimensionIndex"
    )]
    pub dimension_index: i64,
    /**
     * A sort order associated with a specific column or row.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "foregroundColor"
    )]
    pub foreground_color: Option<Color>,
    /**
     * A sort order associated with a specific column or row.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "foregroundColorStyle"
    )]
    pub foreground_color_style: Option<ColorStyle>,
    /**
     * A sort order associated with a specific column or row.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sortOrder")]
    pub sort_order: Option<SortOrder>,
}

/// A combination of a source range and how to extend that source.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SourceDestination {
    /**
     * A combination of a source range and how to extend that source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Dimension>,
    /**
     * A combination of a source range and how to extend that source.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "fillLength"
    )]
    pub fill_length: i64,
    /**
     * A combination of a source range and how to extend that source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GridRange>,
}

/// Resource that represents a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Spreadsheet {
    /**
     * Resource that represents a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dataSourceSchedules"
    )]
    pub data_source_schedules: Vec<DataSourceRefreshSchedule>,
    /**
     * Resource that represents a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dataSources"
    )]
    pub data_sources: Vec<DataSource>,
    /**
     * Resource that represents a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "developerMetadata"
    )]
    pub developer_metadata: Vec<DeveloperMetadata>,
    /**
     * Resource that represents a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "namedRanges"
    )]
    pub named_ranges: Vec<NamedRange>,
    /**
     * Resource that represents a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SpreadsheetProperties>,
    /**
     * Resource that represents a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub sheets: Vec<Sheet>,
    /**
     * Resource that represents a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
    /**
     * Resource that represents a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetUrl"
    )]
    pub spreadsheet_url: String,
}

/**
* The amount of time to wait before volatile functions are recalculated.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum AutoRecalc {
    #[serde(rename = "HOUR")]
    Hour,
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "ON_CHANGE")]
    OnChange,
    #[serde(rename = "RECALCULATION_INTERVAL_UNSPECIFIED")]
    RecalculationIntervalUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AutoRecalc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoRecalc::Hour => "HOUR",
            AutoRecalc::Minute => "MINUTE",
            AutoRecalc::OnChange => "ON_CHANGE",
            AutoRecalc::RecalculationIntervalUnspecified => "RECALCULATION_INTERVAL_UNSPECIFIED",
            AutoRecalc::Noop => "",
            AutoRecalc::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AutoRecalc {
    fn default() -> AutoRecalc {
        AutoRecalc::Noop
    }
}
impl AutoRecalc {
    pub fn is_noop(&self) -> bool {
        matches!(self, AutoRecalc::Noop)
    }
}

/// Properties of a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SpreadsheetProperties {
    /**
     * Properties of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "autoRecalc"
    )]
    pub auto_recalc: Option<AutoRecalc>,
    /**
     * Properties of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "defaultFormat"
    )]
    pub default_format: Option<CellFormat>,
    /**
     * Properties of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "iterativeCalculationSettings"
    )]
    pub iterative_calculation_settings: Option<IterativeCalculationSettings>,
    /**
     * Properties of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub locale: String,
    /**
     * Properties of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "spreadsheetTheme"
    )]
    pub spreadsheet_theme: Option<SpreadsheetTheme>,
    /**
     * Properties of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "timeZone"
    )]
    pub time_zone: String,
    /**
     * Properties of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// Represents spreadsheet theme
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SpreadsheetTheme {
    /**
     * Represents spreadsheet theme
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "primaryFontFamily"
    )]
    pub primary_font_family: String,
    /**
     * Represents spreadsheet theme
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "themeColors"
    )]
    pub theme_colors: Vec<ThemeColorPair>,
}

/// The format of a run of text in a cell. Absent values indicate that the field isn't specified.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TextFormat {
    /**
     * The format of a run of text in a cell. Absent values indicate that the field isn't specified.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub bold: bool,
    /**
     * The format of a run of text in a cell. Absent values indicate that the field isn't specified.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "fontFamily"
    )]
    pub font_family: String,
    /**
     * The format of a run of text in a cell. Absent values indicate that the field isn't specified.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "fontSize"
    )]
    pub font_size: i64,
    /**
     * The format of a run of text in a cell. Absent values indicate that the field isn't specified.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "foregroundColor"
    )]
    pub foreground_color: Option<Color>,
    /**
     * The format of a run of text in a cell. Absent values indicate that the field isn't specified.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "foregroundColorStyle"
    )]
    pub foreground_color_style: Option<ColorStyle>,
    /**
     * The format of a run of text in a cell. Absent values indicate that the field isn't specified.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub italic: bool,
    /**
     * The format of a run of text in a cell. Absent values indicate that the field isn't specified.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    /**
     * The format of a run of text in a cell. Absent values indicate that the field isn't specified.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub strikethrough: bool,
    /**
     * The format of a run of text in a cell. Absent values indicate that the field isn't specified.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub underline: bool,
}

/// A run of a text format. The format of this run continues until the start index of the next run. When updating, all fields must be set.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TextFormatRun {
    /**
     * A run of a text format. The format of this run continues until the start index of the next run. When updating, all fields must be set.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormat>,
    /**
     * A run of a text format. The format of this run continues until the start index of the next run. When updating, all fields must be set.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "startIndex"
    )]
    pub start_index: i64,
}

/// Position settings for text.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TextPosition {
    /**
     * Position settings for text.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "horizontalAlignment"
    )]
    pub horizontal_alignment: Option<HorizontalAlignment>,
}

/// The rotation applied to text in a cell.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TextRotation {
    /**
     * The rotation applied to text in a cell.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub angle: i64,
    /**
     * The rotation applied to text in a cell.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub vertical: bool,
}

/**
* The delimiter type to use.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum DelimiterType {
    #[serde(rename = "AUTODETECT")]
    Autodetect,
    #[serde(rename = "COMMA")]
    Comma,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "DELIMITER_TYPE_UNSPECIFIED")]
    DelimiterTypeUnspecified,
    #[serde(rename = "PERIOD")]
    Period,
    #[serde(rename = "SEMICOLON")]
    Semicolon,
    #[serde(rename = "SPACE")]
    Space,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for DelimiterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DelimiterType::Autodetect => "AUTODETECT",
            DelimiterType::Comma => "COMMA",
            DelimiterType::Custom => "CUSTOM",
            DelimiterType::DelimiterTypeUnspecified => "DELIMITER_TYPE_UNSPECIFIED",
            DelimiterType::Period => "PERIOD",
            DelimiterType::Semicolon => "SEMICOLON",
            DelimiterType::Space => "SPACE",
            DelimiterType::Noop => "",
            DelimiterType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for DelimiterType {
    fn default() -> DelimiterType {
        DelimiterType::Noop
    }
}
impl DelimiterType {
    pub fn is_noop(&self) -> bool {
        matches!(self, DelimiterType::Noop)
    }
}

/// Splits a column of text into multiple columns, based on a delimiter in each cell.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TextColumnsRequest {
    /**
     * Splits a column of text into multiple columns, based on a delimiter in each cell.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub delimiter: String,
    /**
     * Splits a column of text into multiple columns, based on a delimiter in each cell.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "delimiterType"
    )]
    pub delimiter_type: Option<DelimiterType>,
    /**
     * Splits a column of text into multiple columns, based on a delimiter in each cell.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GridRange>,
}

/// A pair mapping a spreadsheet theme color type to the concrete color it represents.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ThemeColorPair {
    /**
     * A pair mapping a spreadsheet theme color type to the concrete color it represents.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorStyle>,
    /**
     * A pair mapping a spreadsheet theme color type to the concrete color it represents.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "colorType")]
    pub color_type: Option<ColorType>,
}

/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TimeOfDay {
    /**
     * Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub hours: i64,
    /**
     * Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub minutes: i64,
    /**
     * Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub nanos: i64,
    /**
     * Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub seconds: i64,
}

/// A color scale for a treemap chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TreemapChartColorScale {
    /**
     * A color scale for a treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "maxValueColor"
    )]
    pub max_value_color: Option<Color>,
    /**
     * A color scale for a treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "maxValueColorStyle"
    )]
    pub max_value_color_style: Option<ColorStyle>,
    /**
     * A color scale for a treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "midValueColor"
    )]
    pub mid_value_color: Option<Color>,
    /**
     * A color scale for a treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "midValueColorStyle"
    )]
    pub mid_value_color_style: Option<ColorStyle>,
    /**
     * A color scale for a treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "minValueColor"
    )]
    pub min_value_color: Option<Color>,
    /**
     * A color scale for a treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "minValueColorStyle"
    )]
    pub min_value_color_style: Option<ColorStyle>,
    /**
     * A color scale for a treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "noDataColor"
    )]
    pub no_data_color: Option<Color>,
    /**
     * A color scale for a treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "noDataColorStyle"
    )]
    pub no_data_color_style: Option<ColorStyle>,
}

/// A Treemap chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TreemapChartSpec {
    /**
     * A Treemap chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "colorData")]
    pub color_data: Option<ChartData>,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "colorScale"
    )]
    pub color_scale: Option<TreemapChartColorScale>,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "headerColor"
    )]
    pub header_color: Option<Color>,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "headerColorStyle"
    )]
    pub header_color_style: Option<ColorStyle>,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "hideTooltips"
    )]
    pub hide_tooltips: bool,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "hintedLevels"
    )]
    pub hinted_levels: i64,
    /**
     * A Treemap chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<ChartData>,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub levels: i64,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "maxValue"
    )]
    pub max_value: f64,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize",
        rename = "minValue"
    )]
    pub min_value: f64,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "parentLabels"
    )]
    pub parent_labels: Option<ChartData>,
    /**
     * A Treemap chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sizeData")]
    pub size_data: Option<ChartData>,
    /**
     * A Treemap chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "textFormat"
    )]
    pub text_format: Option<TextFormat>,
}

/// Trims the whitespace (such as spaces, tabs, or new lines) in every cell in the specified range. This request removes all whitespace from the start and end of each cell's text, and reduces any subsequence of remaining whitespace characters to a single space. If the resulting trimmed text starts with a '+' or '=' character, the text remains as a string value and isn't interpreted as a formula.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TrimWhitespaceRequest {
    /**
     * Trims the whitespace (such as spaces, tabs, or new lines) in every cell in the specified range. This request removes all whitespace from the start and end of each cell's text, and reduces any subsequence of remaining whitespace characters to a single space. If the resulting trimmed text starts with a '+' or '=' character, the text remains as a string value and isn't interpreted as a formula.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
}

/// The result of trimming whitespace in cells.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct TrimWhitespaceResponse {
    /**
     * The result of trimming whitespace in cells.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "cellsChangedCount"
    )]
    pub cells_changed_count: i64,
}

/// Unmerges cells in the given range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UnmergeCellsRequest {
    /**
     * Unmerges cells in the given range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
}

/// Updates properties of the supplied banded range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateBandingRequest {
    /**
     * Updates properties of the supplied banded range.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bandedRange"
    )]
    pub banded_range: Option<BandedRange>,
    /**
     * Updates properties of the supplied banded range.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
}

/// Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateBordersRequest {
    /**
     * Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom: Option<Border>,
    /**
     * Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "innerHorizontal"
    )]
    pub inner_horizontal: Option<Border>,
    /**
     * Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "innerVertical"
    )]
    pub inner_vertical: Option<Border>,
    /**
     * Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<Border>,
    /**
     * Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<Border>,
    /**
     * Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<Border>,
}

/// Updates all cells in a range with new data.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateCellsRequest {
    /**
     * Updates all cells in a range with new data.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates all cells in a range with new data.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<GridRange>,
    /**
     * Updates all cells in a range with new data.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub rows: Vec<RowData>,
    /**
     * Updates all cells in a range with new data.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<GridCoordinate>,
}

/// Updates a chart's specifications. (This does not move or resize a chart. To move or resize a chart, use UpdateEmbeddedObjectPositionRequest.)
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateChartSpecRequest {
    /**
     * Updates a chart's specifications. (This does not move or resize a chart. To move or resize a chart, use UpdateEmbeddedObjectPositionRequest.)
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "chartId"
    )]
    pub chart_id: i64,
    /**
     * Updates a chart's specifications. (This does not move or resize a chart. To move or resize a chart, use UpdateEmbeddedObjectPositionRequest.)
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ChartSpec>,
}

/// Updates a conditional format rule at the given index, or moves a conditional format rule to another index.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateConditionalFormatRuleRequest {
    /**
     * Updates a conditional format rule at the given index, or moves a conditional format rule to another index.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub index: i64,
    /**
     * Updates a conditional format rule at the given index, or moves a conditional format rule to another index.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "newIndex"
    )]
    pub new_index: i64,
    /**
     * Updates a conditional format rule at the given index, or moves a conditional format rule to another index.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<ConditionalFormatRule>,
    /**
     * Updates a conditional format rule at the given index, or moves a conditional format rule to another index.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "sheetId"
    )]
    pub sheet_id: i64,
}

/// The result of updating a conditional format rule.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateConditionalFormatRuleResponse {
    /**
     * The result of updating a conditional format rule.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "newIndex"
    )]
    pub new_index: i64,
    /**
     * The result of updating a conditional format rule.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "newRule")]
    pub new_rule: Option<ConditionalFormatRule>,
    /**
     * The result of updating a conditional format rule.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "oldIndex"
    )]
    pub old_index: i64,
    /**
     * The result of updating a conditional format rule.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oldRule")]
    pub old_rule: Option<ConditionalFormatRule>,
}

/// Updates a data source. After the data source is updated successfully, an execution is triggered to refresh the associated DATA_SOURCE sheet to read data from the updated data source. The request requires an additional `bigquery.readonly` OAuth scope.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateDataSourceRequest {
    /**
     * Updates a data source. After the data source is updated successfully, an execution is triggered to refresh the associated DATA_SOURCE sheet to read data from the updated data source. The request requires an additional `bigquery.readonly` OAuth scope.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSource"
    )]
    pub data_source: Option<DataSource>,
    /**
     * Updates a data source. After the data source is updated successfully, an execution is triggered to refresh the associated DATA_SOURCE sheet to read data from the updated data source. The request requires an additional `bigquery.readonly` OAuth scope.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
}

/// The response from updating data source.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateDataSourceResponse {
    /**
     * The response from updating data source.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataExecutionStatus"
    )]
    pub data_execution_status: Option<DataExecutionStatus>,
    /**
     * The response from updating data source.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSource"
    )]
    pub data_source: Option<DataSource>,
}

/// A request to update properties of developer metadata. Updates the properties of the developer metadata selected by the filters to the values provided in the DeveloperMetadata resource. Callers must specify the properties they wish to update in the fields parameter, as well as specify at least one DataFilter matching the metadata they wish to update.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateDeveloperMetadataRequest {
    /**
     * A request to update properties of developer metadata. Updates the properties of the developer metadata selected by the filters to the values provided in the DeveloperMetadata resource. Callers must specify the properties they wish to update in the fields parameter, as well as specify at least one DataFilter matching the metadata they wish to update.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "dataFilters"
    )]
    pub data_filters: Vec<DataFilter>,
    /**
     * A request to update properties of developer metadata. Updates the properties of the developer metadata selected by the filters to the values provided in the DeveloperMetadata resource. Callers must specify the properties they wish to update in the fields parameter, as well as specify at least one DataFilter matching the metadata they wish to update.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "developerMetadata"
    )]
    pub developer_metadata: Option<DeveloperMetadata>,
    /**
     * A request to update properties of developer metadata. Updates the properties of the developer metadata selected by the filters to the values provided in the DeveloperMetadata resource. Callers must specify the properties they wish to update in the fields parameter, as well as specify at least one DataFilter matching the metadata they wish to update.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
}

/// The response from updating developer metadata.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateDeveloperMetadataResponse {
    /**
     * The response from updating developer metadata.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "developerMetadata"
    )]
    pub developer_metadata: Vec<DeveloperMetadata>,
}

/// Updates the state of the specified group.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateDimensionGroupRequest {
    /**
     * Updates the state of the specified group.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dimensionGroup"
    )]
    pub dimension_group: Option<DimensionGroup>,
    /**
     * Updates the state of the specified group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
}

/// Updates properties of dimensions within the specified range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateDimensionPropertiesRequest {
    /**
     * Updates properties of dimensions within the specified range.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataSourceSheetRange"
    )]
    pub data_source_sheet_range: Option<DataSourceSheetDimensionRange>,
    /**
     * Updates properties of dimensions within the specified range.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates properties of dimensions within the specified range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DimensionProperties>,
    /**
     * Updates properties of dimensions within the specified range.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<DimensionRange>,
}

/// Updates an embedded object's border property.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateEmbeddedObjectBorderRequest {
    /**
     * Updates an embedded object's border property.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<EmbeddedObjectBorder>,
    /**
     * Updates an embedded object's border property.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates an embedded object's border property.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "objectId"
    )]
    pub object_id: i64,
}

/// Update an embedded object's position (such as a moving or resizing a chart or image).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateEmbeddedObjectPositionRequest {
    /**
     * Update an embedded object's position (such as a moving or resizing a chart or image).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Update an embedded object's position (such as a moving or resizing a chart or image).
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "newPosition"
    )]
    pub new_position: Option<EmbeddedObjectPosition>,
    /**
     * Update an embedded object's position (such as a moving or resizing a chart or image).
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "objectId"
    )]
    pub object_id: i64,
}

/// The result of updating an embedded object's position.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateEmbeddedObjectPositionResponse {
    /**
     * The result of updating an embedded object's position.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<EmbeddedObjectPosition>,
}

/// Updates properties of the filter view.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateFilterViewRequest {
    /**
     * Updates properties of the filter view.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates properties of the filter view.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterView>,
}

/// Updates properties of the named range with the specified namedRangeId.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateNamedRangeRequest {
    /**
     * Updates properties of the named range with the specified namedRangeId.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates properties of the named range with the specified namedRangeId.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "namedRange"
    )]
    pub named_range: Option<NamedRange>,
}

/// Updates an existing protected range with the specified protectedRangeId.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateProtectedRangeRequest {
    /**
     * Updates an existing protected range with the specified protectedRangeId.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates an existing protected range with the specified protectedRangeId.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "protectedRange"
    )]
    pub protected_range: Option<ProtectedRange>,
}

/// Updates properties of the sheet with the specified sheetId.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateSheetPropertiesRequest {
    /**
     * Updates properties of the sheet with the specified sheetId.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates properties of the sheet with the specified sheetId.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SheetProperties>,
}

/// Updates a slicer's specifications. (This does not move or resize a slicer. To move or resize a slicer use UpdateEmbeddedObjectPositionRequest.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateSlicerSpecRequest {
    /**
     * Updates a slicer's specifications. (This does not move or resize a slicer. To move or resize a slicer use UpdateEmbeddedObjectPositionRequest.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates a slicer's specifications. (This does not move or resize a slicer. To move or resize a slicer use UpdateEmbeddedObjectPositionRequest.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "slicerId"
    )]
    pub slicer_id: i64,
    /**
     * Updates a slicer's specifications. (This does not move or resize a slicer. To move or resize a slicer use UpdateEmbeddedObjectPositionRequest.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<SlicerSpec>,
}

/// Updates properties of a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateSpreadsheetPropertiesRequest {
    /**
     * Updates properties of a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fields: String,
    /**
     * Updates properties of a spreadsheet.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SpreadsheetProperties>,
}

/// The response when updating a range of values by a data filter in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateValuesByDataFilterResponse {
    /**
     * The response when updating a range of values by a data filter in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dataFilter"
    )]
    pub data_filter: Option<DataFilter>,
    /**
     * The response when updating a range of values by a data filter in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "updatedCells"
    )]
    pub updated_cells: i64,
    /**
     * The response when updating a range of values by a data filter in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "updatedColumns"
    )]
    pub updated_columns: i64,
    /**
     * The response when updating a range of values by a data filter in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updatedData"
    )]
    pub updated_data: Option<ValueRange>,
    /**
     * The response when updating a range of values by a data filter in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "updatedRange"
    )]
    pub updated_range: String,
    /**
     * The response when updating a range of values by a data filter in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "updatedRows"
    )]
    pub updated_rows: i64,
}

/// The response when updating a range of values in a spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct UpdateValuesResponse {
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spreadsheetId"
    )]
    pub spreadsheet_id: String,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "updatedCells"
    )]
    pub updated_cells: i64,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "updatedColumns"
    )]
    pub updated_columns: i64,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "updatedData"
    )]
    pub updated_data: Option<ValueRange>,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "updatedRange"
    )]
    pub updated_range: String,
    /**
     * The response when updating a range of values in a spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "updatedRows"
    )]
    pub updated_rows: i64,
}

/// Data within a range of the spreadsheet.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ValueRange {
    /**
     * Data within a range of the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "majorDimension"
    )]
    pub major_dimension: Option<Dimension>,
    /**
     * Data within a range of the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub range: String,
    /**
     * Data within a range of the spreadsheet.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub values: Vec<Vec<String>>,
}

/// Styles for a waterfall chart column.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WaterfallChartColumnStyle {
    /**
     * Styles for a waterfall chart column.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /**
     * Styles for a waterfall chart column.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "colorStyle"
    )]
    pub color_style: Option<ColorStyle>,
    /**
     * Styles for a waterfall chart column.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
}

/// A custom subtotal column for a waterfall chart series.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WaterfallChartCustomSubtotal {
    /**
     * A custom subtotal column for a waterfall chart series.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "dataIsSubtotal"
    )]
    pub data_is_subtotal: bool,
    /**
     * A custom subtotal column for a waterfall chart series.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    /**
     * A custom subtotal column for a waterfall chart series.
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "subtotalIndex"
    )]
    pub subtotal_index: i64,
}

/// The domain of a waterfall chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WaterfallChartDomain {
    /**
     * The domain of a waterfall chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ChartData>,
    /**
     * The domain of a waterfall chart.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub reversed: bool,
}

/// A single series of data for a waterfall chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WaterfallChartSeries {
    /**
     * A single series of data for a waterfall chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize",
        rename = "customSubtotals"
    )]
    pub custom_subtotals: Vec<WaterfallChartCustomSubtotal>,
    /**
     * A single series of data for a waterfall chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ChartData>,
    /**
     * A single series of data for a waterfall chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataLabel")]
    pub data_label: Option<DataLabel>,
    /**
     * A single series of data for a waterfall chart.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "hideTrailingSubtotal"
    )]
    pub hide_trailing_subtotal: bool,
    /**
     * A single series of data for a waterfall chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "negativeColumnsStyle"
    )]
    pub negative_columns_style: Option<WaterfallChartColumnStyle>,
    /**
     * A single series of data for a waterfall chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "positiveColumnsStyle"
    )]
    pub positive_columns_style: Option<WaterfallChartColumnStyle>,
    /**
     * A single series of data for a waterfall chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subtotalColumnsStyle"
    )]
    pub subtotal_columns_style: Option<WaterfallChartColumnStyle>,
}

/**
* The stacked type.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum WaterfallChartSpecStackedType {
    #[serde(rename = "SEQUENTIAL")]
    Sequential,
    #[serde(rename = "STACKED")]
    Stacked,
    #[serde(rename = "WATERFALL_STACKED_TYPE_UNSPECIFIED")]
    WaterfallStackedTypeUnspecified,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for WaterfallChartSpecStackedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaterfallChartSpecStackedType::Sequential => "SEQUENTIAL",
            WaterfallChartSpecStackedType::Stacked => "STACKED",
            WaterfallChartSpecStackedType::WaterfallStackedTypeUnspecified => {
                "WATERFALL_STACKED_TYPE_UNSPECIFIED"
            }
            WaterfallChartSpecStackedType::Noop => "",
            WaterfallChartSpecStackedType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for WaterfallChartSpecStackedType {
    fn default() -> WaterfallChartSpecStackedType {
        WaterfallChartSpecStackedType::Noop
    }
}
impl WaterfallChartSpecStackedType {
    pub fn is_noop(&self) -> bool {
        matches!(self, WaterfallChartSpecStackedType::Noop)
    }
}

/// A waterfall chart.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WaterfallChartSpec {
    /**
     * A waterfall chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "connectorLineStyle"
    )]
    pub connector_line_style: Option<LineStyle>,
    /**
     * A waterfall chart.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<WaterfallChartDomain>,
    /**
     * A waterfall chart.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "firstValueIsTotal"
    )]
    pub first_value_is_total: bool,
    /**
     * A waterfall chart.
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize",
        rename = "hideConnectorLines"
    )]
    pub hide_connector_lines: bool,
    /**
     * A waterfall chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub series: Vec<WaterfallChartSeries>,
    /**
     * A waterfall chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stackedType"
    )]
    pub stacked_type: Option<WaterfallChartSpecStackedType>,
    /**
     * A waterfall chart.
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalDataLabel"
    )]
    pub total_data_label: Option<DataLabel>,
}

/**
* V1 error format.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Xgafv {
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Xgafv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Xgafv::One => "1",
            Xgafv::Two => "2",
            Xgafv::Noop => "",
            Xgafv::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Xgafv {
    fn default() -> Xgafv {
        Xgafv::Noop
    }
}
impl Xgafv {
    pub fn is_noop(&self) -> bool {
        matches!(self, Xgafv::Noop)
    }
}

/**
* Data format for response.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Alt {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "media")]
    Media,
    #[serde(rename = "proto")]
    Proto,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Alt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alt::Json => "json",
            Alt::Media => "media",
            Alt::Proto => "proto",
            Alt::Noop => "",
            Alt::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Alt {
    fn default() -> Alt {
        Alt::Noop
    }
}
impl Alt {
    pub fn is_noop(&self) -> bool {
        matches!(self, Alt::Noop)
    }
}

/**
* How the input data should be inserted.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum InsertDataOption {
    #[serde(rename = "INSERT_ROWS")]
    InsertRows,
    #[serde(rename = "OVERWRITE")]
    Overwrite,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for InsertDataOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertDataOption::InsertRows => "INSERT_ROWS",
            InsertDataOption::Overwrite => "OVERWRITE",
            InsertDataOption::Noop => "",
            InsertDataOption::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for InsertDataOption {
    fn default() -> InsertDataOption {
        InsertDataOption::Noop
    }
}
impl InsertDataOption {
    pub fn is_noop(&self) -> bool {
        matches!(self, InsertDataOption::Noop)
    }
}

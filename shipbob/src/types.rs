//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ChannelsChannelView {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub application_name: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Array of permissions granted for the channel
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub scopes: Vec<String>,
}

/// Information about an inventory item's dimensions
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InventoryDimensionView {
    /**
     * Information about an inventory item's dimensions
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub depth: f64,
    /**
     * Information about an inventory item's dimensions
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub length: f64,
    /**
     * Information about an inventory item's dimensions
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub weight: f64,
    /**
     * Information about an inventory item's dimensions
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub width: f64,
}

/// Break down of fulfillable quantity by fulfillment center
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InventoryFulfillmentCenterQuantityView {
    /**
     * Break down of fulfillable quantity by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub awaiting_quantity: i64,
    /**
     * Break down of fulfillable quantity by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub committed_quantity: i64,
    /**
     * Break down of fulfillable quantity by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub fulfillable_quantity: i64,
    /**
     * Break down of fulfillable quantity by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Break down of fulfillable quantity by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub internal_transfer_quantity: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Break down of fulfillable quantity by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub onhand_quantity: i64,
}

/// Break down of fulfillable quantity by lot
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InventoryLotQuantityView {
    /**
     * Break down of fulfillable quantity by lot
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub awaiting_quantity: i64,
    /**
     * Break down of fulfillable quantity by lot
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub committed_quantity: i64,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Break down of fulfillable quantity by lot
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub fulfillable_quantity: i64,
    /**
     * Break down of fulfillable quantity by lot
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_fulfillment_center: Vec<InventoryFulfillmentCenterQuantityView>,
    /**
     * Break down of fulfillable quantity by lot
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub internal_transfer_quantity: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lot_number: String,
    /**
     * Break down of fulfillable quantity by lot
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub onhand_quantity: i64,
}

/**
 * Attribute influencing the packaging requirements of this inventory item
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PackagingAttribute {
    #[serde(rename = "Book")]
    Book,
    #[serde(rename = "CustomDunnage")]
    CustomDunnage,
    #[serde(rename = "CustomPackaging")]
    CustomPackaging,
    #[serde(rename = "Foldable")]
    Foldable,
    #[serde(rename = "Fragile")]
    Fragile,
    #[serde(rename = "MarketingInsert")]
    MarketingInsert,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Poster")]
    Poster,
    #[serde(rename = "Stackable")]
    Stackable,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PackagingAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PackagingAttribute::Book => "Book",
            PackagingAttribute::CustomDunnage => "CustomDunnage",
            PackagingAttribute::CustomPackaging => "CustomPackaging",
            PackagingAttribute::Foldable => "Foldable",
            PackagingAttribute::Fragile => "Fragile",
            PackagingAttribute::MarketingInsert => "MarketingInsert",
            PackagingAttribute::None => "None",
            PackagingAttribute::Poster => "Poster",
            PackagingAttribute::Stackable => "Stackable",
            PackagingAttribute::Noop => "",
            PackagingAttribute::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PackagingAttribute {
    fn default() -> PackagingAttribute {
        PackagingAttribute::Noop
    }
}
impl PackagingAttribute {
    pub fn is_noop(&self) -> bool {
        matches!(self, PackagingAttribute::Noop)
    }
}

/// Information about an inventory item
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct InventoryView {
    /**
     * Information about an inventory item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<InventoryDimensionView>,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_fulfillment_center: Vec<InventoryFulfillmentCenterQuantityView>,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_lot: Vec<InventoryLotQuantityView>,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_active: bool,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_case_pick: bool,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_digital: bool,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_lot: bool,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Information about an inventory item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packaging_attribute: Option<PackagingAttribute>,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_awaiting_quantity: i64,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_backordered_quantity: i64,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_committed_quantity: i64,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_exception_quantity: i64,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_fulfillable_quantity: i64,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_internal_transfer_quantity: i64,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_onhand_quantity: i64,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_sellable_quantity: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsEstimationAddress {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip_code: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationModelsEstimateProductInfo {
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationModelsEstimateFulfillmentRequest {
    #[serde()]
    pub address: OrdersPresentationViewModelsEstimationAddress,
    /**
     * Products to be included in the order. Each product must include one of reference_id or id
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub products: Vec<OrdersPresentationModelsEstimateProductInfo>,
    /**
     * Array of permissions granted for the channel
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub shipping_methods: Vec<String>,
}

/// Information about a fulfillment center that a shipment can belong to
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsFulfillmentCenter {
    /**
     * Information about a fulfillment center that a shipment can belong to
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsEstimateDetail {
    /**
     * Weight in ounces of this inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub estimated_price: f64,
    /**
     * Information about a fulfillment center that a shipment can belong to
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_center: Option<OrdersPresentationViewModelsFulfillmentCenter>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub shipping_method: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsEstimate {
    /**
     * Array of estimates for each shipping method
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub estimates: Vec<OrdersPresentationViewModelsEstimateDetail>,
}

/// Created by channel metadata
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsChannelInfoView {
    /**
     * Created by channel metadata
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
 * Specifies the type of address:
 *   ShipFrom
 *   MarkFor
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Type {
    #[serde(rename = "MarkFor")]
    MarkFor,
    #[serde(rename = "ShipFrom")]
    ShipFrom,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Type::MarkFor => "MarkFor",
            Type::ShipFrom => "ShipFrom",
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

/// Address to used when creating a B2B/DropShip order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsRetailerProgramDataAddress {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company_name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Specifies the type of address:
     *  ShipFrom
     *  MarkFor
     */
    #[serde(default, skip_serializing_if = "Type::is_noop", rename = "type")]
    pub type_: Type,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip_code: String,
}

/// Information about the recipient of an order
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsRecipientInfo {
    /**
     * Address to used when creating a B2B/DropShip order.
     */
    #[serde()]
    pub address: OrdersPresentationViewModelsRetailerProgramDataAddress,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsProductInfo {
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sku: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsTag {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// Information about the recipient of a shipment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsRecipient {
    /**
     * Information about the recipient of a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<OrdersPresentationViewModelsRetailerProgramDataAddress>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsStatusDetail {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub exception_fulfillment_center_id: i64,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub inventory_id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Tracking information for a shipment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsTracking {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub carrier: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub carrier_service: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tracking_number: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tracking_url: String,
}

/// Information about inventory belonging to a store product
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsInventory {
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Information about inventory belonging to a store product
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Information about inventory belonging to a store product
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_dangerous_goods: bool,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lot: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Information about inventory belonging to a store product
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
    /**
     * Information about inventory belonging to a store product
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity_committed: i64,
    /**
     * Array of permissions granted for the channel
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub serial_numbers: Vec<String>,
}

/// Information about a store product belonging to a shipment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsShipmentProduct {
    /**
     * Information about a store product belonging to a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Information about a store product belonging to a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub inventory_items: Vec<OrdersPresentationViewModelsInventory>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sku: String,
}

/// Measurements of a shipment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsMeasurements {
    /**
     * Measurements of a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub depth_in: i64,
    /**
     * Measurements of a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub length_in: i64,
    /**
     * Measurements of a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_weight_oz: i64,
    /**
     * Measurements of a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub width_in: i64,
}

/**
 * The shipment status
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Status {
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "CleanSweeped")]
    CleanSweeped,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Exception")]
    Exception,
    #[serde(rename = "ImportReview")]
    ImportReview,
    #[serde(rename = "LabeledCreated")]
    LabeledCreated,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "OnHold")]
    OnHold,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Status::Cancelled => "Cancelled",
            Status::CleanSweeped => "CleanSweeped",
            Status::Completed => "Completed",
            Status::Exception => "Exception",
            Status::ImportReview => "ImportReview",
            Status::LabeledCreated => "LabeledCreated",
            Status::None => "None",
            Status::OnHold => "OnHold",
            Status::Pending => "Pending",
            Status::Processing => "Processing",
            Status::Noop => "",
            Status::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Status {
    fn default() -> Status {
        Status::Noop
    }
}
impl Status {
    pub fn is_noop(&self) -> bool {
        matches!(self, Status::Noop)
    }
}

/**
 * Container type for the shipment
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PackageMaterialType {
    #[serde(rename = "Bookfold")]
    Bookfold,
    #[serde(rename = "Box")]
    Box,
    #[serde(rename = "BubbleMailer")]
    BubbleMailer,
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "FragileBox")]
    FragileBox,
    #[serde(rename = "OwnContainer")]
    OwnContainer,
    #[serde(rename = "PolyMailer")]
    PolyMailer,
    #[serde(rename = "PosterTube")]
    PosterTube,
    #[serde(rename = "Undefined")]
    Undefined,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PackageMaterialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PackageMaterialType::Bookfold => "Bookfold",
            PackageMaterialType::Box => "Box",
            PackageMaterialType::BubbleMailer => "BubbleMailer",
            PackageMaterialType::Custom => "Custom",
            PackageMaterialType::FragileBox => "FragileBox",
            PackageMaterialType::OwnContainer => "OwnContainer",
            PackageMaterialType::PolyMailer => "PolyMailer",
            PackageMaterialType::PosterTube => "PosterTube",
            PackageMaterialType::Undefined => "Undefined",
            PackageMaterialType::Unknown => "Unknown",
            PackageMaterialType::Noop => "",
            PackageMaterialType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PackageMaterialType {
    fn default() -> PackageMaterialType {
        PackageMaterialType::Noop
    }
}
impl PackageMaterialType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PackageMaterialType::Noop)
    }
}

/**
 * Status of ShipBob’s completion of the fulfillment operation.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum EstimatedFulfillmentDateStatus {
    #[serde(rename = "AwaitingInventoryAllocation")]
    AwaitingInventoryAllocation,
    #[serde(rename = "AwaitingReset")]
    AwaitingReset,
    #[serde(rename = "FulfilledLate")]
    FulfilledLate,
    #[serde(rename = "FulfilledOnTime")]
    FulfilledOnTime,
    #[serde(rename = "PendingLate")]
    PendingLate,
    #[serde(rename = "PendingOnTime")]
    PendingOnTime,
    #[serde(rename = "Unavailable")]
    Unavailable,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for EstimatedFulfillmentDateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            EstimatedFulfillmentDateStatus::AwaitingInventoryAllocation => {
                "AwaitingInventoryAllocation"
            }
            EstimatedFulfillmentDateStatus::AwaitingReset => "AwaitingReset",
            EstimatedFulfillmentDateStatus::FulfilledLate => "FulfilledLate",
            EstimatedFulfillmentDateStatus::FulfilledOnTime => "FulfilledOnTime",
            EstimatedFulfillmentDateStatus::PendingLate => "PendingLate",
            EstimatedFulfillmentDateStatus::PendingOnTime => "PendingOnTime",
            EstimatedFulfillmentDateStatus::Unavailable => "Unavailable",
            EstimatedFulfillmentDateStatus::Noop => "",
            EstimatedFulfillmentDateStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for EstimatedFulfillmentDateStatus {
    fn default() -> EstimatedFulfillmentDateStatus {
        EstimatedFulfillmentDateStatus::Noop
    }
}
impl EstimatedFulfillmentDateStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, EstimatedFulfillmentDateStatus::Noop)
    }
}

/// Information about a shipment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsShipment {
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub actual_fulfillment_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub estimated_fulfillment_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_fulfillment_date_status: Option<EstimatedFulfillmentDateStatus>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gift_message: String,
    /**
     * Information about a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Information about a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub insurance_value: f64,
    /**
     * Information about a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub invoice_amount: f64,
    /**
     * Information about a shipment
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_tracking_uploaded: bool,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_update_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<OrdersPresentationViewModelsFulfillmentCenter>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurements: Option<OrdersPresentationViewModelsMeasurements>,
    /**
     * Information about a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub order_id: i64,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_material_type: Option<PackageMaterialType>,
    /**
     * Information about a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub products: Vec<OrdersPresentationViewModelsShipmentProduct>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<OrdersPresentationViewModelsRecipient>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
    /**
     * Information about a shipment
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub require_signature: bool,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ship_option: String,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /**
     * Information about a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub status_details: Vec<OrdersPresentationViewModelsStatusDetail>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<OrdersPresentationViewModelsTracking>,
}

/**
 * Identifies whether to ship parcel or freight.
 *   
 *   Parcel: Smaller, light weight boxes.
 *   
 *   Freight: Larger boxes, usually transported by truckload.
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum CarrierType {
    #[serde(rename = "Freight")]
    Freight,
    #[serde(rename = "Parcel")]
    Parcel,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CarrierType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CarrierType::Freight => "Freight",
            CarrierType::Parcel => "Parcel",
            CarrierType::Noop => "",
            CarrierType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CarrierType {
    fn default() -> CarrierType {
        CarrierType::Noop
    }
}
impl CarrierType {
    pub fn is_noop(&self) -> bool {
        matches!(self, CarrierType::Noop)
    }
}

/**
 * Identifies the party responsible for shipping charges.
 *   
 *   Collect: The person/entity receiving the product pays the shipping charges [freight only].
 *   
 *   ThirdParty: Another party pays for the shipping charges (not Shipbob) [parcel only].
 *   
 *   Prepaid: The shipper pays the shipping charges (Shipbob or merchant).
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum PaymentTerm {
    #[serde(rename = "Collect")]
    Collect,
    #[serde(rename = "Prepaid")]
    Prepaid,
    #[serde(rename = "ThirdParty")]
    ThirdParty,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PaymentTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PaymentTerm::Collect => "Collect",
            PaymentTerm::Prepaid => "Prepaid",
            PaymentTerm::ThirdParty => "ThirdParty",
            PaymentTerm::Noop => "",
            PaymentTerm::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PaymentTerm {
    fn default() -> PaymentTerm {
        PaymentTerm::Noop
    }
}
impl PaymentTerm {
    pub fn is_noop(&self) -> bool {
        matches!(self, PaymentTerm::Noop)
    }
}

/// Contains shipping properties that need to be used for fulfilling an order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsShippingTerms {
    /**
     * Identifies whether to ship parcel or freight.
     *  
     *  Parcel: Smaller, light weight boxes.
     *  
     *  Freight: Larger boxes, usually transported by truckload.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carrier_type: Option<CarrierType>,
    /**
     * Identifies the party responsible for shipping charges.
     *  
     *  Collect: The person/entity receiving the product pays the shipping charges [freight only].
     *  
     *  ThirdParty: Another party pays for the shipping charges (not Shipbob) [parcel only].
     *  
     *  Prepaid: The shipper pays the shipping charges (Shipbob or merchant).
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_term: Option<PaymentTerm>,
}

/// Contains properties that needs to be used for fulfilling B2B/Dropship orders.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsRetailerProgramData {
    /**
     * Contains properties that needs to be used for fulfilling B2B/Dropship orders.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub addresses: Vec<OrdersPresentationViewModelsRetailerProgramDataAddress>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub delivery_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mark_for_store: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub purchase_order_number: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub retailer_program_type: String,
}

/**
 * The order status
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OrdersPresentationViewModelsOrderStatus {
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Exception")]
    Exception,
    #[serde(rename = "Fulfilled")]
    Fulfilled,
    #[serde(rename = "ImportReview")]
    ImportReview,
    #[serde(rename = "PartiallyFulfilled")]
    PartiallyFulfilled,
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OrdersPresentationViewModelsOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrdersPresentationViewModelsOrderStatus::Cancelled => "Cancelled",
            OrdersPresentationViewModelsOrderStatus::Exception => "Exception",
            OrdersPresentationViewModelsOrderStatus::Fulfilled => "Fulfilled",
            OrdersPresentationViewModelsOrderStatus::ImportReview => "ImportReview",
            OrdersPresentationViewModelsOrderStatus::PartiallyFulfilled => "PartiallyFulfilled",
            OrdersPresentationViewModelsOrderStatus::Processing => "Processing",
            OrdersPresentationViewModelsOrderStatus::Noop => "",
            OrdersPresentationViewModelsOrderStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrdersPresentationViewModelsOrderStatus {
    fn default() -> OrdersPresentationViewModelsOrderStatus {
        OrdersPresentationViewModelsOrderStatus::Noop
    }
}
impl OrdersPresentationViewModelsOrderStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrdersPresentationViewModelsOrderStatus::Noop)
    }
}

/**
 * Shipment type of the order
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OrdersPresentationViewModelsOrderType {
    #[serde(rename = "B2B")]
    B2B,
    #[serde(rename = "DTC")]
    Dtc,
    #[serde(rename = "DropShip")]
    DropShip,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OrdersPresentationViewModelsOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrdersPresentationViewModelsOrderType::B2B => "B2B",
            OrdersPresentationViewModelsOrderType::Dtc => "DTC",
            OrdersPresentationViewModelsOrderType::DropShip => "DropShip",
            OrdersPresentationViewModelsOrderType::Noop => "",
            OrdersPresentationViewModelsOrderType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrdersPresentationViewModelsOrderType {
    fn default() -> OrdersPresentationViewModelsOrderType {
        OrdersPresentationViewModelsOrderType::Noop
    }
}
impl OrdersPresentationViewModelsOrderType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrdersPresentationViewModelsOrderType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsOrder {
    /**
     * Created by channel metadata
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<ReturnsChannelInfoView>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gift_message: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_number: String,
    /**
     * List of products included in the order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub products: Vec<OrdersPresentationViewModelsProductInfo>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub purchase_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Information about the recipient of an order
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<OrdersPresentationViewModelsRecipientInfo>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
    /**
     * Contains properties that needs to be used for fulfilling B2B/Dropship orders.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retailer_program_data: Option<OrdersPresentationViewModelsRetailerProgramData>,
    /**
     * Shipments affiliated with the order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub shipments: Vec<OrdersPresentationViewModelsShipment>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub shipping_method: String,
    /**
     * Contains shipping properties that need to be used for fulfilling an order.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_terms: Option<OrdersPresentationViewModelsShippingTerms>,
    /**
     * The order status
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OrdersPresentationViewModelsOrderStatus>,
    /**
     * Client-defined order tags
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<OrdersPresentationViewModelsTag>,
    /**
     * Shipment type of the order
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<OrdersPresentationViewModelsOrderType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationModelsAddProductOrderBy {
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationModelsAddProductOrderByReference {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
}

/// All of the following types:
///
/// - `OrdersPresentationModelsAddProductOrderBy`
/// - `OrdersPresentationModelsAddProductOrderByReference`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrdersPresentationModelsAddProductOrderOneOf {
    OrdersPresentationModelsAddProductOrderBy(OrdersPresentationModelsAddProductOrderBy),
    OrdersPresentationModelsAddProductOrderByReference(
        OrdersPresentationModelsAddProductOrderByReference,
    ),
}

impl OrdersPresentationModelsAddProductOrderOneOf {
    pub fn orders_presentation_models_add_product_order_by(
        &self,
    ) -> Option<&OrdersPresentationModelsAddProductOrderBy> {
        if let OrdersPresentationModelsAddProductOrderOneOf::OrdersPresentationModelsAddProductOrderBy(ref_) = self {
                                return Some(ref_);
                            }
        None
    }

    pub fn orders_presentation_models_add_product_order_by_reference(
        &self,
    ) -> Option<&OrdersPresentationModelsAddProductOrderByReference> {
        if let OrdersPresentationModelsAddProductOrderOneOf::OrdersPresentationModelsAddProductOrderByReference(ref_) = self {
                                return Some(ref_);
                            }
        None
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationModelsCreateOrder {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gift_message: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub location_id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_number: String,
    /**
     * Products included in the order. Products identified by reference_id must also include the product name if there is no matching ShipBob product.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub products: Vec<OrdersPresentationModelsAddProductOrderOneOf>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub purchase_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Information about the recipient of an order
     */
    #[serde()]
    pub recipient: OrdersPresentationViewModelsRecipientInfo,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
    /**
     * Contains properties that needs to be used for fulfilling B2B/Dropship orders.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retailer_program_data: Option<OrdersPresentationViewModelsRetailerProgramData>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub shipping_method: String,
    /**
     * Contains shipping properties that need to be used for fulfilling an order.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_terms: Option<OrdersPresentationViewModelsShippingTerms>,
    /**
     * Client-defined order tags
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<OrdersPresentationViewModelsTag>,
    /**
     * Shipment type of the order
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<OrdersPresentationViewModelsOrderType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Action {
    #[serde(rename = "Cancel")]
    Cancel,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Action::Cancel => "Cancel",
            Action::Noop => "",
            Action::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Action {
    fn default() -> Action {
        Action::Noop
    }
}
impl Action {
    pub fn is_noop(&self) -> bool {
        matches!(self, Action::Noop)
    }
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsCanceledShipment {
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /**
     *
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_success: bool,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub shipment_id: i64,
}

/**
 * The overall result of canceling the shipments associated with the order
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OrdersPresentationViewModelsCanceledOrderStatus {
    #[serde(rename = "Failure")]
    Failure,
    #[serde(rename = "PartialSuccess")]
    PartialSuccess,
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for OrdersPresentationViewModelsCanceledOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            OrdersPresentationViewModelsCanceledOrderStatus::Failure => "Failure",
            OrdersPresentationViewModelsCanceledOrderStatus::PartialSuccess => "PartialSuccess",
            OrdersPresentationViewModelsCanceledOrderStatus::Success => "Success",
            OrdersPresentationViewModelsCanceledOrderStatus::Noop => "",
            OrdersPresentationViewModelsCanceledOrderStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrdersPresentationViewModelsCanceledOrderStatus {
    fn default() -> OrdersPresentationViewModelsCanceledOrderStatus {
        OrdersPresentationViewModelsCanceledOrderStatus::Noop
    }
}
impl OrdersPresentationViewModelsCanceledOrderStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrdersPresentationViewModelsCanceledOrderStatus::Noop)
    }
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsCanceledOrder {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub canceled_shipment_results: Vec<OrdersPresentationViewModelsCanceledShipment>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<OrdersPresentationViewModelsOrder>,
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub order_id: i64,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OrdersPresentationViewModelsCanceledOrderStatus>,
}

/// Model for adding a Store Order Json to a ShipBob Order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationModelsAddStoreOrderJson {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub order_json: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsShipmentLog {
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub log_type_id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub log_type_name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub log_type_text: String,
    /**
     * Specifics data for the event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/// Model for cancel multiple shipments at once
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationModelsCancelShipments {
    /**
     * Model for cancel multiple shipments at once
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub shipment_ids: Vec<i64>,
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsCanceledShipments {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<OrdersPresentationViewModelsCanceledShipment>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProductsPublicBundleRootInformationView {
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersPresentationViewModelsShipMethodDetail {
    /**
     * True if the inventory item is marked as a digital item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub active: bool,
    /**
     * True if the inventory item is marked as a digital item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub default: bool,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<ProductsPublicBundleRootInformationView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProductsCommonModelsProductActiveStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Any")]
    Any,
    #[serde(rename = "Inactive")]
    Inactive,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProductsCommonModelsProductActiveStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ProductsCommonModelsProductActiveStatus::Active => "Active",
            ProductsCommonModelsProductActiveStatus::Any => "Any",
            ProductsCommonModelsProductActiveStatus::Inactive => "Inactive",
            ProductsCommonModelsProductActiveStatus::Noop => "",
            ProductsCommonModelsProductActiveStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProductsCommonModelsProductActiveStatus {
    fn default() -> ProductsCommonModelsProductActiveStatus {
        ProductsCommonModelsProductActiveStatus::Noop
    }
}
impl ProductsCommonModelsProductActiveStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProductsCommonModelsProductActiveStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProductsCommonModelsProductBundleStatus {
    #[serde(rename = "Any")]
    Any,
    #[serde(rename = "Bundle")]
    Bundle,
    #[serde(rename = "NotBundle")]
    NotBundle,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ProductsCommonModelsProductBundleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ProductsCommonModelsProductBundleStatus::Any => "Any",
            ProductsCommonModelsProductBundleStatus::Bundle => "Bundle",
            ProductsCommonModelsProductBundleStatus::NotBundle => "NotBundle",
            ProductsCommonModelsProductBundleStatus::Noop => "",
            ProductsCommonModelsProductBundleStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProductsCommonModelsProductBundleStatus {
    fn default() -> ProductsCommonModelsProductBundleStatus {
        ProductsCommonModelsProductBundleStatus::Noop
    }
}
impl ProductsCommonModelsProductBundleStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProductsCommonModelsProductBundleStatus::Noop)
    }
}

/// Information about a store channel
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProductsPublicChannelView {
    /**
     * Information about a store channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// The inventory that store products can resolve to when packing a shipment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProductsPublicInventoryItemView {
    /**
     * The inventory that store products can resolve to when packing a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The inventory that store products can resolve to when packing a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

/// Break down of quantities by fulfillment center
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProductsPublicFulfillmentCenterQuantityView {
    /**
     * Break down of quantities by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub committed_quantity: i64,
    /**
     * Break down of quantities by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub fulfillable_quantity: i64,
    /**
     * Break down of quantities by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Break down of quantities by fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub onhand_quantity: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProductsPublicProductView {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub barcode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_root_information: Option<ProductsPublicBundleRootInformationView>,
    /**
     * Information about a store channel
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<ProductsPublicChannelView>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The inventory that this product will resolve to when packing a shipment
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_inventory_items: Vec<ProductsPublicInventoryItemView>,
    /**
     * Fulfillable quantity of this product broken down by fulfillment center location
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_fulfillment_center:
        Vec<ProductsPublicFulfillmentCenterQuantityView>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gtin: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sku: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_committed_quantity: i64,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_fulfillable_quantity: i64,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_onhand_quantity: i64,
    /**
     * Weight in ounces of this inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unit_price: f64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub upc: String,
}

/// The product to create
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProductsApiModelsPublicCreateProduct {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub barcode: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gtin: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sku: String,
    /**
     * The product to create
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unit_price: f64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub upc: String,
}

/// Updates to an existing product product
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProductsApiModelsPublicUpdateProduct {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub barcode: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gtin: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sku: String,
    /**
     * Updates to an existing product product
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub unit_price: f64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub upc: String,
}

/// Information about a fulfillment center
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReceivingFulfillmentCenterView {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Information about a fulfillment center
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_number: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip_code: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReceivingPublicCommonModelsStatus {
    #[serde(rename = "Arrived")]
    Arrived,
    #[serde(rename = "Awaiting")]
    Awaiting,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Incomplete")]
    Incomplete,
    #[serde(rename = "PartiallyArrived")]
    PartiallyArrived,
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReceivingPublicCommonModelsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReceivingPublicCommonModelsStatus::Arrived => "Arrived",
            ReceivingPublicCommonModelsStatus::Awaiting => "Awaiting",
            ReceivingPublicCommonModelsStatus::Cancelled => "Cancelled",
            ReceivingPublicCommonModelsStatus::Completed => "Completed",
            ReceivingPublicCommonModelsStatus::Incomplete => "Incomplete",
            ReceivingPublicCommonModelsStatus::PartiallyArrived => "PartiallyArrived",
            ReceivingPublicCommonModelsStatus::Processing => "Processing",
            ReceivingPublicCommonModelsStatus::Noop => "",
            ReceivingPublicCommonModelsStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReceivingPublicCommonModelsStatus {
    fn default() -> ReceivingPublicCommonModelsStatus {
        ReceivingPublicCommonModelsStatus::Noop
    }
}
impl ReceivingPublicCommonModelsStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReceivingPublicCommonModelsStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReceivingPublicCommonModelsPackageType {
    #[serde(rename = "FloorLoadedContainer")]
    FloorLoadedContainer,
    #[serde(rename = "Package")]
    Package,
    #[serde(rename = "Pallet")]
    Pallet,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReceivingPublicCommonModelsPackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReceivingPublicCommonModelsPackageType::FloorLoadedContainer => "FloorLoadedContainer",
            ReceivingPublicCommonModelsPackageType::Package => "Package",
            ReceivingPublicCommonModelsPackageType::Pallet => "Pallet",
            ReceivingPublicCommonModelsPackageType::Noop => "",
            ReceivingPublicCommonModelsPackageType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReceivingPublicCommonModelsPackageType {
    fn default() -> ReceivingPublicCommonModelsPackageType {
        ReceivingPublicCommonModelsPackageType::Noop
    }
}
impl ReceivingPublicCommonModelsPackageType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReceivingPublicCommonModelsPackageType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReceivingPublicCommonModelsPackingType {
    #[serde(rename = "EverythingInOneBox")]
    EverythingInOneBox,
    #[serde(rename = "MultipleSkuPerBox")]
    MultipleSkuPerBox,
    #[serde(rename = "OneSkuPerBox")]
    OneSkuPerBox,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReceivingPublicCommonModelsPackingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReceivingPublicCommonModelsPackingType::EverythingInOneBox => "EverythingInOneBox",
            ReceivingPublicCommonModelsPackingType::MultipleSkuPerBox => "MultipleSkuPerBox",
            ReceivingPublicCommonModelsPackingType::OneSkuPerBox => "OneSkuPerBox",
            ReceivingPublicCommonModelsPackingType::Noop => "",
            ReceivingPublicCommonModelsPackingType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReceivingPublicCommonModelsPackingType {
    fn default() -> ReceivingPublicCommonModelsPackingType {
        ReceivingPublicCommonModelsPackingType::Noop
    }
}
impl ReceivingPublicCommonModelsPackingType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReceivingPublicCommonModelsPackingType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReceivingPublicCommonModelsBoxStatus {
    #[serde(rename = "Arrived")]
    Arrived,
    #[serde(rename = "Awaiting")]
    Awaiting,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Counting")]
    Counting,
    #[serde(rename = "Stowing")]
    Stowing,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReceivingPublicCommonModelsBoxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReceivingPublicCommonModelsBoxStatus::Arrived => "Arrived",
            ReceivingPublicCommonModelsBoxStatus::Awaiting => "Awaiting",
            ReceivingPublicCommonModelsBoxStatus::Cancelled => "Cancelled",
            ReceivingPublicCommonModelsBoxStatus::Completed => "Completed",
            ReceivingPublicCommonModelsBoxStatus::Counting => "Counting",
            ReceivingPublicCommonModelsBoxStatus::Stowing => "Stowing",
            ReceivingPublicCommonModelsBoxStatus::Noop => "",
            ReceivingPublicCommonModelsBoxStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReceivingPublicCommonModelsBoxStatus {
    fn default() -> ReceivingPublicCommonModelsBoxStatus {
        ReceivingPublicCommonModelsBoxStatus::Noop
    }
}
impl ReceivingPublicCommonModelsBoxStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReceivingPublicCommonModelsBoxStatus::Noop)
    }
}

/// Information about an item contained inside a box as part of a receiving order
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReceivingBoxItemView {
    /**
     * Information about an item contained inside a box as part of a receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub inventory_id: i64,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub lot_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lot_number: String,
    /**
     * Information about an item contained inside a box as part of a receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
    /**
     * Information about an item contained inside a box as part of a receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub received_quantity: i64,
}

/// Information about a box shipment included in a receiving order
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReceivingBoxView {
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub arrived_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Information about the items included in the box
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub box_items: Vec<ReceivingBoxItemView>,
    /**
     * Information about a box shipment included in a receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub box_number: i64,
    /**
     * Information about a box shipment included in a receiving order
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub box_status: Option<ReceivingPublicCommonModelsBoxStatus>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub counting_started_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub received_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tracking_number: String,
}

/// Information about a receiving order
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReceivingOrderView {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub box_labels_uri: String,
    /**
     * Information about a receiving order
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub box_packaging_type: Option<ReceivingPublicCommonModelsPackingType>,
    /**
     * Information about the boxes being shipped in this receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub boxes: Vec<ReceivingBoxView>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub expected_arrival_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Information about a receiving order
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_center: Option<ReceivingFulfillmentCenterView>,
    /**
     * Information about a receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub insert_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub last_updated_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Information about a receiving order
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_type: Option<ReceivingPublicCommonModelsPackageType>,
    /**
     * Information about a receiving order
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceivingPublicCommonModelsStatus>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Extensions {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MicrosoftAspNetCoreMvcValidationProblemDetails {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub detail: String,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub errors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instance: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub status: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// Model containing information that assigns a receiving order to a fulfillment center
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReceivingAssignOrderFulfillmentCenter {
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
}

/// Information about an inventory item contained inside a receiving order box
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReceivingAddBoxItemTo {
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub inventory_id: i64,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub lot_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lot_number: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

/// Information about a box shipment to be added to a receiving order
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReceivingAddBoxOrder {
    /**
     * Items contained in this box
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub box_items: Vec<ReceivingAddBoxItemTo>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tracking_number: String,
}

/// Information to create a new receiving order
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReceivingCreateOrder {
    #[serde()]
    pub box_packaging_type: ReceivingPublicCommonModelsPackingType,
    /**
     * Box shipments to be added to this receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub boxes: Vec<ReceivingAddBoxOrder>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub expected_arrival_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Model containing information that assigns a receiving order to a fulfillment center
     */
    #[serde()]
    pub fulfillment_center: ReceivingAssignOrderFulfillmentCenter,
    #[serde()]
    pub package_type: ReceivingPublicCommonModelsPackageType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReturnsPublicCommonReturnStatus {
    #[serde(rename = "Arrived")]
    Arrived,
    #[serde(rename = "AwaitingArrival")]
    AwaitingArrival,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReturnsPublicCommonReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReturnsPublicCommonReturnStatus::Arrived => "Arrived",
            ReturnsPublicCommonReturnStatus::AwaitingArrival => "AwaitingArrival",
            ReturnsPublicCommonReturnStatus::Cancelled => "Cancelled",
            ReturnsPublicCommonReturnStatus::Completed => "Completed",
            ReturnsPublicCommonReturnStatus::Processing => "Processing",
            ReturnsPublicCommonReturnStatus::Noop => "",
            ReturnsPublicCommonReturnStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnsPublicCommonReturnStatus {
    fn default() -> ReturnsPublicCommonReturnStatus {
        ReturnsPublicCommonReturnStatus::Noop
    }
}
impl ReturnsPublicCommonReturnStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnsPublicCommonReturnStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReturnsPublicCommonTransactionLogSource {
    #[serde(rename = "ReturnLabelInvoice")]
    ReturnLabelInvoice,
    #[serde(rename = "ReturnProcessingFee")]
    ReturnProcessingFee,
    #[serde(rename = "ReturnToSenderFee")]
    ReturnToSenderFee,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReturnsPublicCommonTransactionLogSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReturnsPublicCommonTransactionLogSource::ReturnLabelInvoice => "ReturnLabelInvoice",
            ReturnsPublicCommonTransactionLogSource::ReturnProcessingFee => "ReturnProcessingFee",
            ReturnsPublicCommonTransactionLogSource::ReturnToSenderFee => "ReturnToSenderFee",
            ReturnsPublicCommonTransactionLogSource::Noop => "",
            ReturnsPublicCommonTransactionLogSource::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnsPublicCommonTransactionLogSource {
    fn default() -> ReturnsPublicCommonTransactionLogSource {
        ReturnsPublicCommonTransactionLogSource::Noop
    }
}
impl ReturnsPublicCommonTransactionLogSource {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnsPublicCommonTransactionLogSource::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsTransactionView {
    /**
     * Weight in ounces of this inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub amount: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<ReturnsPublicCommonTransactionLogSource>,
}

/// Information about a fulfillment center
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsFulfillmentCenterView {
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReturnsPublicCommonReturnAction {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Dispose")]
    Dispose,
    #[serde(rename = "Quarantine")]
    Quarantine,
    #[serde(rename = "Restock")]
    Restock,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReturnsPublicCommonReturnAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReturnsPublicCommonReturnAction::Default => "Default",
            ReturnsPublicCommonReturnAction::Dispose => "Dispose",
            ReturnsPublicCommonReturnAction::Quarantine => "Quarantine",
            ReturnsPublicCommonReturnAction::Restock => "Restock",
            ReturnsPublicCommonReturnAction::Noop => "",
            ReturnsPublicCommonReturnAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnsPublicCommonReturnAction {
    fn default() -> ReturnsPublicCommonReturnAction {
        ReturnsPublicCommonReturnAction::Noop
    }
}
impl ReturnsPublicCommonReturnAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnsPublicCommonReturnAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReturnsPublicCommonReturnActionSource {
    #[serde(rename = "InventoryDefault")]
    InventoryDefault,
    #[serde(rename = "Override")]
    Override,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReturnsPublicCommonReturnActionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReturnsPublicCommonReturnActionSource::InventoryDefault => "InventoryDefault",
            ReturnsPublicCommonReturnActionSource::Override => "Override",
            ReturnsPublicCommonReturnActionSource::Noop => "",
            ReturnsPublicCommonReturnActionSource::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnsPublicCommonReturnActionSource {
    fn default() -> ReturnsPublicCommonReturnActionSource {
        ReturnsPublicCommonReturnActionSource::Noop
    }
}
impl ReturnsPublicCommonReturnActionSource {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnsPublicCommonReturnActionSource::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsReturnActionRequestedView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ReturnsPublicCommonReturnAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ReturnsPublicCommonReturnActionSource>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instructions: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsReturnActionTakenView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ReturnsPublicCommonReturnAction>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action_reason: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity_processed: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsInventoryItemView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_requested: Option<ReturnsReturnActionRequestedView>,
    /**
     * Action(s) taken when processing the return
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub action_taken: Vec<ReturnsReturnActionTakenView>,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReturnsPublicCommonReturnType {
    #[serde(rename = "Regular")]
    Regular,
    #[serde(rename = "ReturnToSender")]
    ReturnToSender,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReturnsPublicCommonReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ReturnsPublicCommonReturnType::Regular => "Regular",
            ReturnsPublicCommonReturnType::ReturnToSender => "ReturnToSender",
            ReturnsPublicCommonReturnType::Noop => "",
            ReturnsPublicCommonReturnType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnsPublicCommonReturnType {
    fn default() -> ReturnsPublicCommonReturnType {
        ReturnsPublicCommonReturnType::Noop
    }
}
impl ReturnsPublicCommonReturnType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnsPublicCommonReturnType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsReturnOrderView {
    /**
     * Created by channel metadata
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<ReturnsChannelInfoView>,
    /**
     * Information about a fulfillment center
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_center: Option<ReturnsFulfillmentCenterView>,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub insert_date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * List of inventory included in the return order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub inventory: Vec<ReturnsInventoryItemView>,
    /**
     * Weight in ounces of this inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub invoice_amount: f64,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub original_shipment_id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_type: Option<ReturnsPublicCommonReturnType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReturnsPublicCommonReturnStatus>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tracking_number: String,
    /**
     * Array of transactions affiliated with the return order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub transactions: Vec<ReturnsTransactionView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsReturnInventoryView {
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub quantity: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_action: Option<ReturnsPublicCommonReturnAction>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsCreateReturnView {
    /**
     * Information about a fulfillment center
     */
    #[serde()]
    pub fulfillment_center: ReturnsFulfillmentCenterView,
    /**
     * Array of inventory items being returned
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub inventory: Vec<ReturnsReturnInventoryView>,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub original_shipment_id: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reference_id: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tracking_number: String,
}

/**
 * Order to sort results in
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum SortOrder {
    #[serde(rename = "Newest")]
    Newest,
    #[serde(rename = "Oldest")]
    Oldest,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            SortOrder::Newest => "Newest",
            SortOrder::Oldest => "Oldest",
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsReturnOrderStatusHistoryView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReturnsPublicCommonReturnStatus>,
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub time_stamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum WebhooksPublicCommonTopics {
    #[serde(rename = "order_shipped")]
    OrderShipped,
    #[serde(rename = "shipment_delivered")]
    ShipmentDelivered,
    #[serde(rename = "shipment_exception")]
    ShipmentException,
    #[serde(rename = "shipment_onhold")]
    ShipmentOnhold,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for WebhooksPublicCommonTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            WebhooksPublicCommonTopics::OrderShipped => "order_shipped",
            WebhooksPublicCommonTopics::ShipmentDelivered => "shipment_delivered",
            WebhooksPublicCommonTopics::ShipmentException => "shipment_exception",
            WebhooksPublicCommonTopics::ShipmentOnhold => "shipment_onhold",
            WebhooksPublicCommonTopics::Noop => "",
            WebhooksPublicCommonTopics::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for WebhooksPublicCommonTopics {
    fn default() -> WebhooksPublicCommonTopics {
        WebhooksPublicCommonTopics::Noop
    }
}
impl WebhooksPublicCommonTopics {
    pub fn is_noop(&self) -> bool {
        matches!(self, WebhooksPublicCommonTopics::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebhooksWebhookView {
    /**
     * Expiration date for this lot
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::date_time_format::deserialize"
    )]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Unique id of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    /**
     * URL subscription events will be posted to
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::deserialize_empty_url::deserialize"
    )]
    pub subscription_url: Option<url::Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<WebhooksPublicCommonTopics>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebhooksCreateWebhookSubscription {
    /**
     * URL subscription events will be posted to
     */
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::deserialize_empty_url::deserialize"
    )]
    pub subscription_url: Option<url::Url>,
    #[serde()]
    pub topic: WebhooksPublicCommonTopics,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct MicrosoftAspNetCoreMvcProblemDetails {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "$type"
    )]
    pub type__: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub detail: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instance: String,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub status: i64,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum IntegrationsLocationPublicCommonServiceTypeEnum {
    #[serde(rename = "Receiving")]
    Receiving,
    #[serde(rename = "Returns")]
    Returns,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IntegrationsLocationPublicCommonServiceTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            IntegrationsLocationPublicCommonServiceTypeEnum::Receiving => "Receiving",
            IntegrationsLocationPublicCommonServiceTypeEnum::Returns => "Returns",
            IntegrationsLocationPublicCommonServiceTypeEnum::Noop => "",
            IntegrationsLocationPublicCommonServiceTypeEnum::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IntegrationsLocationPublicCommonServiceTypeEnum {
    fn default() -> IntegrationsLocationPublicCommonServiceTypeEnum {
        IntegrationsLocationPublicCommonServiceTypeEnum::Noop
    }
}
impl IntegrationsLocationPublicCommonServiceTypeEnum {
    pub fn is_noop(&self) -> bool {
        matches!(self, IntegrationsLocationPublicCommonServiceTypeEnum::Noop)
    }
}

/// The service-specific address of the location. Each object contains address type, address1, address2, city, state, country, zip code, phone number, and email
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IntegrationsLocationAddressView {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address1"
    )]
    pub address_1: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address2"
    )]
    pub address_2: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone_number: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip_code: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IntegrationsLocationServiceView {
    /**
     * The service-specific address of the location. Each object contains address type, address1, address2, city, state, country, zip code, phone number, and email
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<IntegrationsLocationAddressView>,
    /**
     * True if the inventory item is marked as a digital item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<IntegrationsLocationPublicCommonServiceTypeEnum>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IntegrationsLocationView {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "$type"
    )]
    pub type__: String,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub abbreviation: String,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub access_granted: bool,
    /**
     * Array of permissions granted for the channel
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub attributes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub id: i64,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_active: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_receiving_enabled: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_shipping_enabled: bool,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<ProductsPublicBundleRootInformationView>,
    /**
     * Services provided by the location
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub services: Vec<IntegrationsLocationServiceView>,
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timezone: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IntegrationsLocationInternalView {
    /**
     * True if the inventory item is marked as a digital item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_enabled_for_new_user: bool,
}

/// All of the following types are flattened into one object:
///
/// - `IntegrationsLocationView`
/// - `IntegrationsLocationInternalView`
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IntegrationsLocationInternalViewAllOf {
    #[serde(flatten)]
    pub integrations_location_view: IntegrationsLocationView,
    #[serde(flatten)]
    pub integrations_location_internal_view: IntegrationsLocationInternalView,
}

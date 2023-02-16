//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Channel {
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
pub struct InventoryDimension {
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
pub struct InventoryFulfillmentCenterQuantity {
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
pub struct InventoryLotQuantity {
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
    pub fulfillable_quantity_by_fulfillment_center: Vec<InventoryFulfillmentCenterQuantity>,
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
        match self {
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
pub struct Inventory {
    /**
     * Information about an inventory item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<InventoryDimension>,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_fulfillment_center: Vec<InventoryFulfillmentCenterQuantity>,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_lot: Vec<InventoryLotQuantity>,
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
pub struct OrdersEstimationAddress {
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
pub struct OrdersEstimateProductInfoModel {
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
pub struct OrdersEstimateFulfillmentRequestModel {
    #[serde()]
    pub address: OrdersEstimationAddress,
    /**
     * Products to be included in the order. Each product must include one of reference_id or id
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub products: Vec<OrdersEstimateProductInfoModel>,
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
pub struct OrdersFulfillmentCenter {
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
pub struct OrdersEstimateDetail {
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
    pub fulfillment_center: Option<OrdersFulfillmentCenter>,
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
pub struct OrdersEstimate {
    /**
     * Array of estimates for each shipping method
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub estimates: Vec<OrdersEstimateDetail>,
}

/// Created by channel metadata
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersChannelInfo {
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
        match self {
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
pub struct OrdersRetailerProgramDataAddress {
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
pub struct OrdersRecipientInfo {
    /**
     * Address to used when creating a B2B/DropShip order.
     */
    #[serde()]
    pub address: OrdersRetailerProgramDataAddress,
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
pub struct OrdersProductInfo {
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
pub struct OrdersTag {
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
pub struct OrdersRecipient {
    /**
     * Information about the recipient of a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<OrdersRetailerProgramDataAddress>,
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
pub struct OrdersStatusDetail {
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
pub struct OrdersTracking {
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
pub struct OrdersInventory {
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
pub struct OrdersShipmentProduct {
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
    pub inventory_items: Vec<OrdersInventory>,
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
pub struct OrdersMeasurements {
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
        match self {
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
        match self {
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
        match self {
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
pub struct OrdersShipment {
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
    pub location: Option<OrdersFulfillmentCenter>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurements: Option<OrdersMeasurements>,
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
    pub products: Vec<OrdersShipmentProduct>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<OrdersRecipient>,
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
    pub status_details: Vec<OrdersStatusDetail>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<OrdersTracking>,
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
        match self {
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
        match self {
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
pub struct OrdersShippingTerms {
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
pub struct OrdersRetailerProgramData {
    /**
     * Contains properties that needs to be used for fulfilling B2B/Dropship orders.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub addresses: Vec<OrdersRetailerProgramDataAddress>,
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
pub enum OrderStatus {
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

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Cancelled => "Cancelled",
            OrderStatus::Exception => "Exception",
            OrderStatus::Fulfilled => "Fulfilled",
            OrderStatus::ImportReview => "ImportReview",
            OrderStatus::PartiallyFulfilled => "PartiallyFulfilled",
            OrderStatus::Processing => "Processing",
            OrderStatus::Noop => "",
            OrderStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrderStatus {
    fn default() -> OrderStatus {
        OrderStatus::Noop
    }
}
impl OrderStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrderStatus::Noop)
    }
}

/**
* Shipment type of the order
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum OrderType {
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

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::B2B => "B2B",
            OrderType::Dtc => "DTC",
            OrderType::DropShip => "DropShip",
            OrderType::Noop => "",
            OrderType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrderType {
    fn default() -> OrderType {
        OrderType::Noop
    }
}
impl OrderType {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrderType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Order {
    /**
     * Created by channel metadata
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<OrdersChannelInfo>,
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
    pub products: Vec<OrdersProductInfo>,
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
    pub recipient: Option<OrdersRecipientInfo>,
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
    pub retailer_program_data: Option<OrdersRetailerProgramData>,
    /**
     * Shipments affiliated with the order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub shipments: Vec<OrdersShipment>,
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
    pub shipping_terms: Option<OrdersShippingTerms>,
    /**
     * The order status
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    /**
     * Client-defined order tags
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<OrdersTag>,
    /**
     * Shipment type of the order
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<OrderType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersAddProductOrderByModel {
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
pub struct OrdersAddProductOrderByReferenceModel {
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
/// - `OrdersAddProductOrderByModel`
/// - `OrdersAddProductOrderByReferenceModel`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum OrdersAddProductOrderModelOneOf {
    OrdersAddProductOrderByModel(OrdersAddProductOrderByModel),
    OrdersAddProductOrderByReferenceModel(OrdersAddProductOrderByReferenceModel),
}

impl OrdersAddProductOrderModelOneOf {
    pub fn orders_add_product_order_by_model(&self) -> Option<&OrdersAddProductOrderByModel> {
        if let OrdersAddProductOrderModelOneOf::OrdersAddProductOrderByModel(ref_) = self {
            return Some(ref_);
        }
        None
    }

    pub fn orders_add_product_order_by_reference_model(
        &self,
    ) -> Option<&OrdersAddProductOrderByReferenceModel> {
        if let OrdersAddProductOrderModelOneOf::OrdersAddProductOrderByReferenceModel(ref_) = self {
            return Some(ref_);
        }
        None
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersCreateOrderModel {
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
    pub products: Vec<OrdersAddProductOrderModelOneOf>,
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
    pub recipient: OrdersRecipientInfo,
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
    pub retailer_program_data: Option<OrdersRetailerProgramData>,
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
    pub shipping_terms: Option<OrdersShippingTerms>,
    /**
     * Client-defined order tags
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<OrdersTag>,
    /**
     * Shipment type of the order
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<OrderType>,
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
        match self {
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
pub struct OrdersCanceledShipment {
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
pub enum OrdersCanceledOrderStatus {
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

impl std::fmt::Display for OrdersCanceledOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrdersCanceledOrderStatus::Failure => "Failure",
            OrdersCanceledOrderStatus::PartialSuccess => "PartialSuccess",
            OrdersCanceledOrderStatus::Success => "Success",
            OrdersCanceledOrderStatus::Noop => "",
            OrdersCanceledOrderStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for OrdersCanceledOrderStatus {
    fn default() -> OrdersCanceledOrderStatus {
        OrdersCanceledOrderStatus::Noop
    }
}
impl OrdersCanceledOrderStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, OrdersCanceledOrderStatus::Noop)
    }
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersCanceledOrder {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub canceled_shipment_results: Vec<OrdersCanceledShipment>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
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
    pub status: Option<OrdersCanceledOrderStatus>,
}

/// Model for adding a Store Order Json to a ShipBob Order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersAddStoreOrderJsonModel {
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
pub struct OrdersShipmentLog {
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
pub struct OrdersCancelShipmentsModel {
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
pub struct OrdersCanceledShipments {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<OrdersCanceledShipment>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct OrdersServiceLevelDetail {
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
pub struct OrdersShipMethodDetail {
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
    pub service_level: Option<OrdersServiceLevelDetail>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProductActiveStatus {
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

impl std::fmt::Display for ProductActiveStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductActiveStatus::Active => "Active",
            ProductActiveStatus::Any => "Any",
            ProductActiveStatus::Inactive => "Inactive",
            ProductActiveStatus::Noop => "",
            ProductActiveStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProductActiveStatus {
    fn default() -> ProductActiveStatus {
        ProductActiveStatus::Noop
    }
}
impl ProductActiveStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProductActiveStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ProductBundleStatus {
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

impl std::fmt::Display for ProductBundleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductBundleStatus::Any => "Any",
            ProductBundleStatus::Bundle => "Bundle",
            ProductBundleStatus::NotBundle => "NotBundle",
            ProductBundleStatus::Noop => "",
            ProductBundleStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ProductBundleStatus {
    fn default() -> ProductBundleStatus {
        ProductBundleStatus::Noop
    }
}
impl ProductBundleStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ProductBundleStatus::Noop)
    }
}

/// Information about a store channel
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ProductsChannel {
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
pub struct ProductsInventoryItem {
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
pub struct ProductsFulfillmentCenterQuantity {
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
pub struct Product {
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
    pub bundle_root_information: Option<OrdersServiceLevelDetail>,
    /**
     * Information about a store channel
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<ProductsChannel>,
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
    pub fulfillable_inventory_items: Vec<ProductsInventoryItem>,
    /**
     * Fulfillable quantity of this product broken down by fulfillment center location
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_fulfillment_center: Vec<ProductsFulfillmentCenterQuantity>,
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
pub struct ProductsCreateProductModel {
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
pub struct ProductsUpdateProductModel {
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
pub struct ReceivingFulfillmentCenter {
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
pub enum ReceivingStatus {
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

impl std::fmt::Display for ReceivingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReceivingStatus::Arrived => "Arrived",
            ReceivingStatus::Awaiting => "Awaiting",
            ReceivingStatus::Cancelled => "Cancelled",
            ReceivingStatus::Completed => "Completed",
            ReceivingStatus::Incomplete => "Incomplete",
            ReceivingStatus::PartiallyArrived => "PartiallyArrived",
            ReceivingStatus::Processing => "Processing",
            ReceivingStatus::Noop => "",
            ReceivingStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReceivingStatus {
    fn default() -> ReceivingStatus {
        ReceivingStatus::Noop
    }
}
impl ReceivingStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReceivingStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReceivingPackageType {
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

impl std::fmt::Display for ReceivingPackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReceivingPackageType::FloorLoadedContainer => "FloorLoadedContainer",
            ReceivingPackageType::Package => "Package",
            ReceivingPackageType::Pallet => "Pallet",
            ReceivingPackageType::Noop => "",
            ReceivingPackageType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReceivingPackageType {
    fn default() -> ReceivingPackageType {
        ReceivingPackageType::Noop
    }
}
impl ReceivingPackageType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReceivingPackageType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReceivingPackingType {
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

impl std::fmt::Display for ReceivingPackingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReceivingPackingType::EverythingInOneBox => "EverythingInOneBox",
            ReceivingPackingType::MultipleSkuPerBox => "MultipleSkuPerBox",
            ReceivingPackingType::OneSkuPerBox => "OneSkuPerBox",
            ReceivingPackingType::Noop => "",
            ReceivingPackingType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReceivingPackingType {
    fn default() -> ReceivingPackingType {
        ReceivingPackingType::Noop
    }
}
impl ReceivingPackingType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReceivingPackingType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReceivingBoxStatus {
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

impl std::fmt::Display for ReceivingBoxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReceivingBoxStatus::Arrived => "Arrived",
            ReceivingBoxStatus::Awaiting => "Awaiting",
            ReceivingBoxStatus::Cancelled => "Cancelled",
            ReceivingBoxStatus::Completed => "Completed",
            ReceivingBoxStatus::Counting => "Counting",
            ReceivingBoxStatus::Stowing => "Stowing",
            ReceivingBoxStatus::Noop => "",
            ReceivingBoxStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReceivingBoxStatus {
    fn default() -> ReceivingBoxStatus {
        ReceivingBoxStatus::Noop
    }
}
impl ReceivingBoxStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReceivingBoxStatus::Noop)
    }
}

/// Information about an item contained inside a box as part of a receiving order
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReceivingBoxItem {
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
pub struct ReceivingBox {
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
    pub box_items: Vec<ReceivingBoxItem>,
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
    pub box_status: Option<ReceivingBoxStatus>,
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
pub struct ReceivingOrder {
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
    pub box_packaging_type: Option<ReceivingPackingType>,
    /**
     * Information about the boxes being shipped in this receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub boxes: Vec<ReceivingBox>,
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
    pub fulfillment_center: Option<ReceivingFulfillmentCenter>,
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
    pub package_type: Option<ReceivingPackageType>,
    /**
     * Information about a receiving order
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceivingStatus>,
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
pub struct ReceivingAssignOrderFulfillmentCenterModel {
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
pub struct ReceivingAddBoxItemModel {
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
pub struct ReceivingAddBoxOrderModel {
    /**
     * Items contained in this box
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub box_items: Vec<ReceivingAddBoxItemModel>,
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
pub struct ReceivingCreateOrderModel {
    #[serde()]
    pub box_packaging_type: ReceivingPackingType,
    /**
     * Box shipments to be added to this receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub boxes: Vec<ReceivingAddBoxOrderModel>,
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
    pub fulfillment_center: ReceivingAssignOrderFulfillmentCenterModel,
    #[serde()]
    pub package_type: ReceivingPackageType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReturnStatus {
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

impl std::fmt::Display for ReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnStatus::Arrived => "Arrived",
            ReturnStatus::AwaitingArrival => "AwaitingArrival",
            ReturnStatus::Cancelled => "Cancelled",
            ReturnStatus::Completed => "Completed",
            ReturnStatus::Processing => "Processing",
            ReturnStatus::Noop => "",
            ReturnStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnStatus {
    fn default() -> ReturnStatus {
        ReturnStatus::Noop
    }
}
impl ReturnStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReturnsTransactionLogSource {
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

impl std::fmt::Display for ReturnsTransactionLogSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnsTransactionLogSource::ReturnLabelInvoice => "ReturnLabelInvoice",
            ReturnsTransactionLogSource::ReturnProcessingFee => "ReturnProcessingFee",
            ReturnsTransactionLogSource::ReturnToSenderFee => "ReturnToSenderFee",
            ReturnsTransactionLogSource::Noop => "",
            ReturnsTransactionLogSource::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnsTransactionLogSource {
    fn default() -> ReturnsTransactionLogSource {
        ReturnsTransactionLogSource::Noop
    }
}
impl ReturnsTransactionLogSource {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnsTransactionLogSource::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsTransaction {
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
    pub transaction_type: Option<ReturnsTransactionLogSource>,
}

/// Information about a fulfillment center
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsFulfillmentCenter {
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
pub enum ReturnAction {
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

impl std::fmt::Display for ReturnAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnAction::Default => "Default",
            ReturnAction::Dispose => "Dispose",
            ReturnAction::Quarantine => "Quarantine",
            ReturnAction::Restock => "Restock",
            ReturnAction::Noop => "",
            ReturnAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnAction {
    fn default() -> ReturnAction {
        ReturnAction::Noop
    }
}
impl ReturnAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ReturnActionSource {
    #[serde(rename = "InventoryDefault")]
    InventoryDefault,
    #[serde(rename = "Override")]
    Override,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReturnActionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnActionSource::InventoryDefault => "InventoryDefault",
            ReturnActionSource::Override => "Override",
            ReturnActionSource::Noop => "",
            ReturnActionSource::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnActionSource {
    fn default() -> ReturnActionSource {
        ReturnActionSource::Noop
    }
}
impl ReturnActionSource {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnActionSource::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnActionRequested {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ReturnAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ReturnActionSource>,
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
pub struct ReturnActionTaken {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ReturnAction>,
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
pub struct ReturnsInventoryItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_requested: Option<ReturnActionRequested>,
    /**
     * Action(s) taken when processing the return
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub action_taken: Vec<ReturnActionTaken>,
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
pub enum ReturnType {
    #[serde(rename = "Regular")]
    Regular,
    #[serde(rename = "ReturnToSender")]
    ReturnToSender,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnType::Regular => "Regular",
            ReturnType::ReturnToSender => "ReturnToSender",
            ReturnType::Noop => "",
            ReturnType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ReturnType {
    fn default() -> ReturnType {
        ReturnType::Noop
    }
}
impl ReturnType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ReturnType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnOrder {
    /**
     * Created by channel metadata
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<OrdersChannelInfo>,
    /**
     * Information about a fulfillment center
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_center: Option<ReturnsFulfillmentCenter>,
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
    pub inventory: Vec<ReturnsInventoryItem>,
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
    pub return_type: Option<ReturnType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReturnStatus>,
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
    pub transactions: Vec<ReturnsTransaction>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnInventory {
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
    pub requested_action: Option<ReturnAction>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ReturnsCreateReturn {
    /**
     * Information about a fulfillment center
     */
    #[serde()]
    pub fulfillment_center: ReturnsFulfillmentCenter,
    /**
     * Array of inventory items being returned
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub inventory: Vec<ReturnInventory>,
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
        match self {
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
pub struct ReturnOrderStatusHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReturnStatus>,
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
pub enum WebhooksTopics {
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

impl std::fmt::Display for WebhooksTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhooksTopics::OrderShipped => "order_shipped",
            WebhooksTopics::ShipmentDelivered => "shipment_delivered",
            WebhooksTopics::ShipmentException => "shipment_exception",
            WebhooksTopics::ShipmentOnhold => "shipment_onhold",
            WebhooksTopics::Noop => "",
            WebhooksTopics::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for WebhooksTopics {
    fn default() -> WebhooksTopics {
        WebhooksTopics::Noop
    }
}
impl WebhooksTopics {
    pub fn is_noop(&self) -> bool {
        matches!(self, WebhooksTopics::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Webhook {
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
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<WebhooksTopics>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct WebhooksCreateWebhookSubscriptionModel {
    /**
     * Name of the channel
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde()]
    pub topic: WebhooksTopics,
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
pub enum IntegrationsLocationServiceTypeEnum {
    #[serde(rename = "Receiving")]
    Receiving,
    #[serde(rename = "Returns")]
    Returns,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for IntegrationsLocationServiceTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegrationsLocationServiceTypeEnum::Receiving => "Receiving",
            IntegrationsLocationServiceTypeEnum::Returns => "Returns",
            IntegrationsLocationServiceTypeEnum::Noop => "",
            IntegrationsLocationServiceTypeEnum::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for IntegrationsLocationServiceTypeEnum {
    fn default() -> IntegrationsLocationServiceTypeEnum {
        IntegrationsLocationServiceTypeEnum::Noop
    }
}
impl IntegrationsLocationServiceTypeEnum {
    pub fn is_noop(&self) -> bool {
        matches!(self, IntegrationsLocationServiceTypeEnum::Noop)
    }
}

/// The service-specific address of the location. Each object contains address type, address1, address2, city, state, country, zip code, phone number, and email
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IntegrationsLocationAddress {
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
pub struct IntegrationsLocationService {
    /**
     * The service-specific address of the location. Each object contains address type, address1, address2, city, state, country, zip code, phone number, and email
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<IntegrationsLocationAddress>,
    /**
     * True if the inventory item is marked as a digital item
     */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<IntegrationsLocationServiceTypeEnum>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IntegrationsLocation {
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
    pub region: Option<OrdersServiceLevelDetail>,
    /**
     * Services provided by the location
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub services: Vec<IntegrationsLocationService>,
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
pub struct IntegrationsLocationInternal {
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
/// - `IntegrationsLocation`
/// - `IntegrationsLocationInternal`
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IntegrationsLocationInternalAllOf {
    #[serde(flatten)]
    pub integrations_location: IntegrationsLocation,
    #[serde(flatten)]
    pub integrations_location_internal: IntegrationsLocationInternal,
}

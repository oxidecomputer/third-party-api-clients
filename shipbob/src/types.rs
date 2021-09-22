//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipBobChannelsApiViewModelsChannelModel {
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
pub struct ShipbobInventoryApiViewModelsDimensionModel {
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
pub struct ShipbobInventoryApiViewModelsFulfillmentCenterQuantityModel {
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
pub struct ShipbobInventoryApiViewModelsLotQuantityModel {
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
    pub fulfillable_quantity_by_fulfillment_center:
        Vec<ShipbobInventoryApiViewModelsFulfillmentCenterQuantityModel>,
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
pub struct ShipbobInventoryApiViewModelsModel {
    /**
     * Information about an inventory item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ShipbobInventoryApiViewModelsDimensionModel>,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_fulfillment_center:
        Vec<ShipbobInventoryApiViewModelsFulfillmentCenterQuantityModel>,
    /**
     * Information about an inventory item
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_lot: Vec<ShipbobInventoryApiViewModelsLotQuantityModel>,
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
pub struct ShipBobOrdersPresentationViewModelsEstimationAddressModel {
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
pub struct ShipBobOrdersPresentationModelsEstimateProductInfoModel {
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
pub struct ShipBobOrdersPresentationModelsEstimateFulfillmentRequestModel {
    #[serde()]
    pub address: ShipBobOrdersPresentationViewModelsEstimationAddressModel,
    /**
     * Products to be included in the order. Each product must include one of reference_id or id
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub products: Vec<ShipBobOrdersPresentationModelsEstimateProductInfoModel>,
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
pub struct ShipBobOrdersPresentationViewModelsFulfillmentCenterModel {
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
pub struct ShipBobOrdersPresentationViewModelsEstimateDetailModel {
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
    pub fulfillment_center: Option<ShipBobOrdersPresentationViewModelsFulfillmentCenterModel>,
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
pub struct ShipBobOrdersPresentationViewModelsEstimateModel {
    /**
     * Array of estimates for each shipping method
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub estimates: Vec<ShipBobOrdersPresentationViewModelsEstimateDetailModel>,
}

/// Created by channel metadata
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipbobReturnsPublicApiViewModelsChannelInfoModel {
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
pub struct ShipBobOrdersPresentationViewModelsRetailerProgramDataAddressModel {
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
pub struct ShipBobOrdersPresentationViewModelsRecipientInfoModel {
    /**
     * Address to used when creating a B2B/DropShip order.
     */
    #[serde()]
    pub address: ShipBobOrdersPresentationViewModelsRetailerProgramDataAddressModel,
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
pub struct ShipBobOrdersPresentationViewModelsProductInfoModel {
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
pub struct ShipBobOrdersPresentationViewModelsTagModel {
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
pub struct ShipBobOrdersPresentationViewModelsRecipientModel {
    /**
     * Information about the recipient of a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ShipBobOrdersPresentationViewModelsRetailerProgramDataAddressModel>,
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
pub struct ShipBobOrdersPresentationViewModelsStatusDetailModel {
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
pub struct ShipBobOrdersPresentationViewModelsTrackingModel {
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
pub struct ShipBobOrdersPresentationViewModelsInventoryModel {
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
pub struct ShipBobOrdersPresentationViewModelsShipmentProductModel {
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
    pub inventory_items: Vec<ShipBobOrdersPresentationViewModelsInventoryModel>,
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
pub struct ShipBobOrdersPresentationViewModelsMeasurementsModel {
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
pub struct ShipBobOrdersPresentationViewModelsShipmentModel {
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
    pub location: Option<ShipBobOrdersPresentationViewModelsFulfillmentCenterModel>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurements: Option<ShipBobOrdersPresentationViewModelsMeasurementsModel>,
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
    pub products: Vec<ShipBobOrdersPresentationViewModelsShipmentProductModel>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<ShipBobOrdersPresentationViewModelsRecipientModel>,
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
    pub status_details: Vec<ShipBobOrdersPresentationViewModelsStatusDetailModel>,
    /**
     * Information about a shipment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking: Option<ShipBobOrdersPresentationViewModelsTrackingModel>,
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
pub struct ShipBobOrdersPresentationViewModelsShippingTermsModel {
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
pub struct ShipBobOrdersPresentationViewModelsRetailerProgramDataModel {
    /**
     * Contains properties that needs to be used for fulfilling B2B/Dropship orders.
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub addresses: Vec<ShipBobOrdersPresentationViewModelsRetailerProgramDataAddressModel>,
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
pub enum ShipBobOrdersPresentationViewModelsOrderModelStatus {
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

impl std::fmt::Display for ShipBobOrdersPresentationViewModelsOrderModelStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipBobOrdersPresentationViewModelsOrderModelStatus::Cancelled => "Cancelled",
            ShipBobOrdersPresentationViewModelsOrderModelStatus::Exception => "Exception",
            ShipBobOrdersPresentationViewModelsOrderModelStatus::Fulfilled => "Fulfilled",
            ShipBobOrdersPresentationViewModelsOrderModelStatus::ImportReview => "ImportReview",
            ShipBobOrdersPresentationViewModelsOrderModelStatus::PartiallyFulfilled => {
                "PartiallyFulfilled"
            }
            ShipBobOrdersPresentationViewModelsOrderModelStatus::Processing => "Processing",
            ShipBobOrdersPresentationViewModelsOrderModelStatus::Noop => "",
            ShipBobOrdersPresentationViewModelsOrderModelStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipBobOrdersPresentationViewModelsOrderModelStatus {
    fn default() -> ShipBobOrdersPresentationViewModelsOrderModelStatus {
        ShipBobOrdersPresentationViewModelsOrderModelStatus::Noop
    }
}
impl ShipBobOrdersPresentationViewModelsOrderModelStatus {
    pub fn is_noop(&self) -> bool {
        matches!(
            self,
            ShipBobOrdersPresentationViewModelsOrderModelStatus::Noop
        )
    }
}

/**
 * Shipment type of the order
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ShipBobOrdersPresentationViewModelsOrderModelType {
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

impl std::fmt::Display for ShipBobOrdersPresentationViewModelsOrderModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipBobOrdersPresentationViewModelsOrderModelType::B2B => "B2B",
            ShipBobOrdersPresentationViewModelsOrderModelType::Dtc => "DTC",
            ShipBobOrdersPresentationViewModelsOrderModelType::DropShip => "DropShip",
            ShipBobOrdersPresentationViewModelsOrderModelType::Noop => "",
            ShipBobOrdersPresentationViewModelsOrderModelType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipBobOrdersPresentationViewModelsOrderModelType {
    fn default() -> ShipBobOrdersPresentationViewModelsOrderModelType {
        ShipBobOrdersPresentationViewModelsOrderModelType::Noop
    }
}
impl ShipBobOrdersPresentationViewModelsOrderModelType {
    pub fn is_noop(&self) -> bool {
        matches!(
            self,
            ShipBobOrdersPresentationViewModelsOrderModelType::Noop
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipBobOrdersPresentationViewModelsOrderModel {
    /**
     * Created by channel metadata
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<ShipbobReturnsPublicApiViewModelsChannelInfoModel>,
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
    pub products: Vec<ShipBobOrdersPresentationViewModelsProductInfoModel>,
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
    pub recipient: Option<ShipBobOrdersPresentationViewModelsRecipientInfoModel>,
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
    pub retailer_program_data: Option<ShipBobOrdersPresentationViewModelsRetailerProgramDataModel>,
    /**
     * Shipments affiliated with the order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub shipments: Vec<ShipBobOrdersPresentationViewModelsShipmentModel>,
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
    pub shipping_terms: Option<ShipBobOrdersPresentationViewModelsShippingTermsModel>,
    /**
     * The order status
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ShipBobOrdersPresentationViewModelsOrderModelStatus>,
    /**
     * Client-defined order tags
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<ShipBobOrdersPresentationViewModelsTagModel>,
    /**
     * Shipment type of the order
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<ShipBobOrdersPresentationViewModelsOrderModelType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipBobOrdersPresentationModelsAddProductOrderByModel {
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
pub struct ShipBobOrdersPresentationModelsAddProductOrderByReferenceModel {
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
/// - `ShipBobOrdersPresentationModelsAddProductOrderByModel`
/// - `ShipBobOrdersPresentationModelsAddProductOrderByReferenceModel`
///
/// You can easily convert this enum to the inner value with `From` and `Into`, as both are implemented for each type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ShipBobOrdersPresentationModelsAddProductOrderModelOneOf {
    ShipBobOrdersPresentationModelsAddProductOrderByModel(
        ShipBobOrdersPresentationModelsAddProductOrderByModel,
    ),
    ShipBobOrdersPresentationModelsAddProductOrderByReferenceModel(
        ShipBobOrdersPresentationModelsAddProductOrderByReferenceModel,
    ),
}

impl ShipBobOrdersPresentationModelsAddProductOrderModelOneOf {
    pub fn ship_bob_orders_presentation_models_add_product_order_by_model(
        &self,
    ) -> Option<&ShipBobOrdersPresentationModelsAddProductOrderByModel> {
        if let ShipBobOrdersPresentationModelsAddProductOrderModelOneOf::ShipBobOrdersPresentationModelsAddProductOrderByModel(ref_) = self {
                                return Some(ref_);
                            }
        None
    }

    pub fn ship_bob_orders_presentation_models_add_product_order_by_reference_model(
        &self,
    ) -> Option<&ShipBobOrdersPresentationModelsAddProductOrderByReferenceModel> {
        if let ShipBobOrdersPresentationModelsAddProductOrderModelOneOf::ShipBobOrdersPresentationModelsAddProductOrderByReferenceModel(ref_) = self {
                                return Some(ref_);
                            }
        None
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipBobOrdersPresentationModelsCreateOrderModel {
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
    pub products: Vec<ShipBobOrdersPresentationModelsAddProductOrderModelOneOf>,
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
    pub recipient: ShipBobOrdersPresentationViewModelsRecipientInfoModel,
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
    pub retailer_program_data: Option<ShipBobOrdersPresentationViewModelsRetailerProgramDataModel>,
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
    pub shipping_terms: Option<ShipBobOrdersPresentationViewModelsShippingTermsModel>,
    /**
     * Client-defined order tags
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub tags: Vec<ShipBobOrdersPresentationViewModelsTagModel>,
    /**
     * Shipment type of the order
     */
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<ShipBobOrdersPresentationViewModelsOrderModelType>,
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
pub struct ShipBobOrdersPresentationViewModelsCanceledShipmentModel {
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
pub enum ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus {
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

impl std::fmt::Display for ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus::Failure => "Failure",
            ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus::PartialSuccess => {
                "PartialSuccess"
            }
            ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus::Success => "Success",
            ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus::Noop => "",
            ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus {
    fn default() -> ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus {
        ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus::Noop
    }
}
impl ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus {
    pub fn is_noop(&self) -> bool {
        matches!(
            self,
            ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus::Noop
        )
    }
}

///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipBobOrdersPresentationViewModelsCanceledOrderModel {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub canceled_shipment_results: Vec<ShipBobOrdersPresentationViewModelsCanceledShipmentModel>,
    /**
     *
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<ShipBobOrdersPresentationViewModelsOrderModel>,
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
    pub status: Option<ShipBobOrdersPresentationViewModelsCanceledOrderModelStatus>,
}

/// Model for adding a Store Order Json to a ShipBob Order.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipBobOrdersPresentationModelsAddStoreOrderJsonModel {
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
pub struct ShipBobOrdersPresentationViewModelsShipmentLogModel {
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
pub struct ShipBobOrdersPresentationModelsCancelShipmentsModel {
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
pub struct ShipBobOrdersPresentationViewModelsCanceledShipmentsModel {
    /**
     *
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub results: Vec<ShipBobOrdersPresentationViewModelsCanceledShipmentModel>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipBobOrdersPresentationViewModelsServiceLevelDetailModel {
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
pub struct ShipBobOrdersPresentationViewModelsMethodDetailModel {
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
    pub service_level: Option<ShipBobOrdersPresentationViewModelsServiceLevelDetailModel>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ShipbobProductsCommonModelsProductActiveStatus {
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

impl std::fmt::Display for ShipbobProductsCommonModelsProductActiveStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobProductsCommonModelsProductActiveStatus::Active => "Active",
            ShipbobProductsCommonModelsProductActiveStatus::Any => "Any",
            ShipbobProductsCommonModelsProductActiveStatus::Inactive => "Inactive",
            ShipbobProductsCommonModelsProductActiveStatus::Noop => "",
            ShipbobProductsCommonModelsProductActiveStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobProductsCommonModelsProductActiveStatus {
    fn default() -> ShipbobProductsCommonModelsProductActiveStatus {
        ShipbobProductsCommonModelsProductActiveStatus::Noop
    }
}
impl ShipbobProductsCommonModelsProductActiveStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobProductsCommonModelsProductActiveStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ShipbobProductsCommonModelsProductBundleStatus {
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

impl std::fmt::Display for ShipbobProductsCommonModelsProductBundleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobProductsCommonModelsProductBundleStatus::Any => "Any",
            ShipbobProductsCommonModelsProductBundleStatus::Bundle => "Bundle",
            ShipbobProductsCommonModelsProductBundleStatus::NotBundle => "NotBundle",
            ShipbobProductsCommonModelsProductBundleStatus::Noop => "",
            ShipbobProductsCommonModelsProductBundleStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobProductsCommonModelsProductBundleStatus {
    fn default() -> ShipbobProductsCommonModelsProductBundleStatus {
        ShipbobProductsCommonModelsProductBundleStatus::Noop
    }
}
impl ShipbobProductsCommonModelsProductBundleStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobProductsCommonModelsProductBundleStatus::Noop)
    }
}

/// Information about a store channel
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipbobProductsApiViewModelsPublicChannelModel {
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
pub struct ShipbobProductsApiViewModelsPublicInventoryItemModel {
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
pub struct ShipbobProductsApiViewModelsPublicFulfillmentCenterQuantityModel {
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
pub struct ShipbobProductsApiViewModelsPublicProductModel {
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
    pub bundle_root_information: Option<ShipBobOrdersPresentationViewModelsServiceLevelDetailModel>,
    /**
     * Information about a store channel
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<ShipbobProductsApiViewModelsPublicChannelModel>,
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
    pub fulfillable_inventory_items: Vec<ShipbobProductsApiViewModelsPublicInventoryItemModel>,
    /**
     * Fulfillable quantity of this product broken down by fulfillment center location
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub fulfillable_quantity_by_fulfillment_center:
        Vec<ShipbobProductsApiViewModelsPublicFulfillmentCenterQuantityModel>,
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
pub struct ShipbobProductsApiModelsPublicCreateProductModel {
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
pub struct ShipbobProductsApiModelsPublicUpdateProductModel {
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
pub struct ShipbobReceivingPublicApiModelsFulfillmentCenterViewModel {
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
pub enum ShipbobReceivingPublicCommonModelsStatus {
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

impl std::fmt::Display for ShipbobReceivingPublicCommonModelsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobReceivingPublicCommonModelsStatus::Arrived => "Arrived",
            ShipbobReceivingPublicCommonModelsStatus::Awaiting => "Awaiting",
            ShipbobReceivingPublicCommonModelsStatus::Cancelled => "Cancelled",
            ShipbobReceivingPublicCommonModelsStatus::Completed => "Completed",
            ShipbobReceivingPublicCommonModelsStatus::Incomplete => "Incomplete",
            ShipbobReceivingPublicCommonModelsStatus::PartiallyArrived => "PartiallyArrived",
            ShipbobReceivingPublicCommonModelsStatus::Processing => "Processing",
            ShipbobReceivingPublicCommonModelsStatus::Noop => "",
            ShipbobReceivingPublicCommonModelsStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobReceivingPublicCommonModelsStatus {
    fn default() -> ShipbobReceivingPublicCommonModelsStatus {
        ShipbobReceivingPublicCommonModelsStatus::Noop
    }
}
impl ShipbobReceivingPublicCommonModelsStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobReceivingPublicCommonModelsStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ShipbobReceivingPublicCommonModelsPackageType {
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

impl std::fmt::Display for ShipbobReceivingPublicCommonModelsPackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobReceivingPublicCommonModelsPackageType::FloorLoadedContainer => {
                "FloorLoadedContainer"
            }
            ShipbobReceivingPublicCommonModelsPackageType::Package => "Package",
            ShipbobReceivingPublicCommonModelsPackageType::Pallet => "Pallet",
            ShipbobReceivingPublicCommonModelsPackageType::Noop => "",
            ShipbobReceivingPublicCommonModelsPackageType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobReceivingPublicCommonModelsPackageType {
    fn default() -> ShipbobReceivingPublicCommonModelsPackageType {
        ShipbobReceivingPublicCommonModelsPackageType::Noop
    }
}
impl ShipbobReceivingPublicCommonModelsPackageType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobReceivingPublicCommonModelsPackageType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ShipbobReceivingPublicCommonModelsPackingType {
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

impl std::fmt::Display for ShipbobReceivingPublicCommonModelsPackingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobReceivingPublicCommonModelsPackingType::EverythingInOneBox => {
                "EverythingInOneBox"
            }
            ShipbobReceivingPublicCommonModelsPackingType::MultipleSkuPerBox => "MultipleSkuPerBox",
            ShipbobReceivingPublicCommonModelsPackingType::OneSkuPerBox => "OneSkuPerBox",
            ShipbobReceivingPublicCommonModelsPackingType::Noop => "",
            ShipbobReceivingPublicCommonModelsPackingType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobReceivingPublicCommonModelsPackingType {
    fn default() -> ShipbobReceivingPublicCommonModelsPackingType {
        ShipbobReceivingPublicCommonModelsPackingType::Noop
    }
}
impl ShipbobReceivingPublicCommonModelsPackingType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobReceivingPublicCommonModelsPackingType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ShipbobReceivingPublicCommonModelsBoxStatus {
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

impl std::fmt::Display for ShipbobReceivingPublicCommonModelsBoxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobReceivingPublicCommonModelsBoxStatus::Arrived => "Arrived",
            ShipbobReceivingPublicCommonModelsBoxStatus::Awaiting => "Awaiting",
            ShipbobReceivingPublicCommonModelsBoxStatus::Cancelled => "Cancelled",
            ShipbobReceivingPublicCommonModelsBoxStatus::Completed => "Completed",
            ShipbobReceivingPublicCommonModelsBoxStatus::Counting => "Counting",
            ShipbobReceivingPublicCommonModelsBoxStatus::Stowing => "Stowing",
            ShipbobReceivingPublicCommonModelsBoxStatus::Noop => "",
            ShipbobReceivingPublicCommonModelsBoxStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobReceivingPublicCommonModelsBoxStatus {
    fn default() -> ShipbobReceivingPublicCommonModelsBoxStatus {
        ShipbobReceivingPublicCommonModelsBoxStatus::Noop
    }
}
impl ShipbobReceivingPublicCommonModelsBoxStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobReceivingPublicCommonModelsBoxStatus::Noop)
    }
}

/// Information about an item contained inside a box as part of a receiving order
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipbobReceivingPublicApiModelsBoxItemViewModel {
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
pub struct ShipbobReceivingPublicApiModelsBoxViewModel {
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
    pub box_items: Vec<ShipbobReceivingPublicApiModelsBoxItemViewModel>,
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
    pub box_status: Option<ShipbobReceivingPublicCommonModelsBoxStatus>,
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
pub struct ShipbobReceivingPublicApiModelsOrderViewModel {
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
    pub box_packaging_type: Option<ShipbobReceivingPublicCommonModelsPackingType>,
    /**
     * Information about the boxes being shipped in this receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub boxes: Vec<ShipbobReceivingPublicApiModelsBoxViewModel>,
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
    pub fulfillment_center: Option<ShipbobReceivingPublicApiModelsFulfillmentCenterViewModel>,
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
    pub package_type: Option<ShipbobReceivingPublicCommonModelsPackageType>,
    /**
     * Information about a receiving order
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ShipbobReceivingPublicCommonModelsStatus>,
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
pub struct ShipbobReceivingPublicApiModelsAssignOrderFulfillmentCenterModel {
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
pub struct ShipbobReceivingPublicApiModelsAddBoxItemModel {
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
pub struct ShipbobReceivingPublicApiModelsAddBoxOrderModel {
    /**
     * Items contained in this box
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub box_items: Vec<ShipbobReceivingPublicApiModelsAddBoxItemModel>,
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
pub struct ShipbobReceivingPublicApiModelsCreateOrderModel {
    #[serde()]
    pub box_packaging_type: ShipbobReceivingPublicCommonModelsPackingType,
    /**
     * Box shipments to be added to this receiving order
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub boxes: Vec<ShipbobReceivingPublicApiModelsAddBoxOrderModel>,
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
    pub fulfillment_center: ShipbobReceivingPublicApiModelsAssignOrderFulfillmentCenterModel,
    #[serde()]
    pub package_type: ShipbobReceivingPublicCommonModelsPackageType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ShipbobReturnsPublicCommonReturnStatus {
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

impl std::fmt::Display for ShipbobReturnsPublicCommonReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobReturnsPublicCommonReturnStatus::Arrived => "Arrived",
            ShipbobReturnsPublicCommonReturnStatus::AwaitingArrival => "AwaitingArrival",
            ShipbobReturnsPublicCommonReturnStatus::Cancelled => "Cancelled",
            ShipbobReturnsPublicCommonReturnStatus::Completed => "Completed",
            ShipbobReturnsPublicCommonReturnStatus::Processing => "Processing",
            ShipbobReturnsPublicCommonReturnStatus::Noop => "",
            ShipbobReturnsPublicCommonReturnStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobReturnsPublicCommonReturnStatus {
    fn default() -> ShipbobReturnsPublicCommonReturnStatus {
        ShipbobReturnsPublicCommonReturnStatus::Noop
    }
}
impl ShipbobReturnsPublicCommonReturnStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobReturnsPublicCommonReturnStatus::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ShipbobReturnsPublicCommonTransactionLogSource {
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

impl std::fmt::Display for ShipbobReturnsPublicCommonTransactionLogSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobReturnsPublicCommonTransactionLogSource::ReturnLabelInvoice => {
                "ReturnLabelInvoice"
            }
            ShipbobReturnsPublicCommonTransactionLogSource::ReturnProcessingFee => {
                "ReturnProcessingFee"
            }
            ShipbobReturnsPublicCommonTransactionLogSource::ReturnToSenderFee => {
                "ReturnToSenderFee"
            }
            ShipbobReturnsPublicCommonTransactionLogSource::Noop => "",
            ShipbobReturnsPublicCommonTransactionLogSource::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobReturnsPublicCommonTransactionLogSource {
    fn default() -> ShipbobReturnsPublicCommonTransactionLogSource {
        ShipbobReturnsPublicCommonTransactionLogSource::Noop
    }
}
impl ShipbobReturnsPublicCommonTransactionLogSource {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobReturnsPublicCommonTransactionLogSource::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipbobReturnsPublicApiViewModelsTransactionModel {
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
    pub transaction_type: Option<ShipbobReturnsPublicCommonTransactionLogSource>,
}

/// Information about a fulfillment center
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipbobReturnsPublicApiViewModelsFulfillmentCenterModel {
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
pub enum ShipbobReturnsPublicCommonReturnAction {
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

impl std::fmt::Display for ShipbobReturnsPublicCommonReturnAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobReturnsPublicCommonReturnAction::Default => "Default",
            ShipbobReturnsPublicCommonReturnAction::Dispose => "Dispose",
            ShipbobReturnsPublicCommonReturnAction::Quarantine => "Quarantine",
            ShipbobReturnsPublicCommonReturnAction::Restock => "Restock",
            ShipbobReturnsPublicCommonReturnAction::Noop => "",
            ShipbobReturnsPublicCommonReturnAction::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobReturnsPublicCommonReturnAction {
    fn default() -> ShipbobReturnsPublicCommonReturnAction {
        ShipbobReturnsPublicCommonReturnAction::Noop
    }
}
impl ShipbobReturnsPublicCommonReturnAction {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobReturnsPublicCommonReturnAction::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum ShipbobReturnsPublicCommonReturnActionSource {
    #[serde(rename = "InventoryDefault")]
    InventoryDefault,
    #[serde(rename = "Override")]
    Override,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ShipbobReturnsPublicCommonReturnActionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobReturnsPublicCommonReturnActionSource::InventoryDefault => "InventoryDefault",
            ShipbobReturnsPublicCommonReturnActionSource::Override => "Override",
            ShipbobReturnsPublicCommonReturnActionSource::Noop => "",
            ShipbobReturnsPublicCommonReturnActionSource::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobReturnsPublicCommonReturnActionSource {
    fn default() -> ShipbobReturnsPublicCommonReturnActionSource {
        ShipbobReturnsPublicCommonReturnActionSource::Noop
    }
}
impl ShipbobReturnsPublicCommonReturnActionSource {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobReturnsPublicCommonReturnActionSource::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipbobReturnsPublicApiViewModelsReturnActionRequestedModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ShipbobReturnsPublicCommonReturnAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ShipbobReturnsPublicCommonReturnActionSource>,
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
pub struct ShipbobReturnsPublicApiViewModelsReturnActionTakenModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ShipbobReturnsPublicCommonReturnAction>,
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
pub struct ShipbobReturnsPublicApiViewModelsInventoryItemModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_requested: Option<ShipbobReturnsPublicApiViewModelsReturnActionRequestedModel>,
    /**
     * Action(s) taken when processing the return
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub action_taken: Vec<ShipbobReturnsPublicApiViewModelsReturnActionTakenModel>,
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
pub enum ShipbobReturnsPublicCommonReturnType {
    #[serde(rename = "Regular")]
    Regular,
    #[serde(rename = "ReturnToSender")]
    ReturnToSender,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ShipbobReturnsPublicCommonReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipbobReturnsPublicCommonReturnType::Regular => "Regular",
            ShipbobReturnsPublicCommonReturnType::ReturnToSender => "ReturnToSender",
            ShipbobReturnsPublicCommonReturnType::Noop => "",
            ShipbobReturnsPublicCommonReturnType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipbobReturnsPublicCommonReturnType {
    fn default() -> ShipbobReturnsPublicCommonReturnType {
        ShipbobReturnsPublicCommonReturnType::Noop
    }
}
impl ShipbobReturnsPublicCommonReturnType {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipbobReturnsPublicCommonReturnType::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipbobReturnsPublicApiViewModelsReturnOrderModel {
    /**
     * Created by channel metadata
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<ShipbobReturnsPublicApiViewModelsChannelInfoModel>,
    /**
     * Information about a fulfillment center
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_center: Option<ShipbobReturnsPublicApiViewModelsFulfillmentCenterModel>,
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
    pub inventory: Vec<ShipbobReturnsPublicApiViewModelsInventoryItemModel>,
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
    pub return_type: Option<ShipbobReturnsPublicCommonReturnType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ShipbobReturnsPublicCommonReturnStatus>,
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
    pub transactions: Vec<ShipbobReturnsPublicApiViewModelsTransactionModel>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipbobReturnsPublicApiViewModelsReturnInventoryModel {
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
    pub requested_action: Option<ShipbobReturnsPublicCommonReturnAction>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipbobReturnsPublicApiViewModelsCreateReturnModel {
    /**
     * Information about a fulfillment center
     */
    #[serde()]
    pub fulfillment_center: ShipbobReturnsPublicApiViewModelsFulfillmentCenterModel,
    /**
     * Array of inventory items being returned
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub inventory: Vec<ShipbobReturnsPublicApiViewModelsReturnInventoryModel>,
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
pub struct ShipbobReturnsPublicApiViewModelsReturnOrderStatusHistoryModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ShipbobReturnsPublicCommonReturnStatus>,
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
pub enum ShipBobWebhooksPublicCommonTopics {
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

impl std::fmt::Display for ShipBobWebhooksPublicCommonTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ShipBobWebhooksPublicCommonTopics::OrderShipped => "order_shipped",
            ShipBobWebhooksPublicCommonTopics::ShipmentDelivered => "shipment_delivered",
            ShipBobWebhooksPublicCommonTopics::ShipmentException => "shipment_exception",
            ShipBobWebhooksPublicCommonTopics::ShipmentOnhold => "shipment_onhold",
            ShipBobWebhooksPublicCommonTopics::Noop => "",
            ShipBobWebhooksPublicCommonTopics::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ShipBobWebhooksPublicCommonTopics {
    fn default() -> ShipBobWebhooksPublicCommonTopics {
        ShipBobWebhooksPublicCommonTopics::Noop
    }
}
impl ShipBobWebhooksPublicCommonTopics {
    pub fn is_noop(&self) -> bool {
        matches!(self, ShipBobWebhooksPublicCommonTopics::Noop)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipBobWebhooksPublicApiModelsWebhookViewModel {
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
    pub topic: Option<ShipBobWebhooksPublicCommonTopics>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct ShipBobWebhooksPublicApiModelsCreateWebhookSubscriptionModel {
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
    pub topic: ShipBobWebhooksPublicCommonTopics,
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
pub struct IntegrationsLocationPublicApiViewModelsAddressModel {
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
pub struct IntegrationsLocationPublicApiViewModelsServiceModel {
    /**
     * The service-specific address of the location. Each object contains address type, address1, address2, city, state, country, zip code, phone number, and email
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<IntegrationsLocationPublicApiViewModelsAddressModel>,
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
pub struct IntegrationsLocationPublicApiViewModelsModel {
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
    pub region: Option<ShipBobOrdersPresentationViewModelsServiceLevelDetailModel>,
    /**
     * Services provided by the location
     */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub services: Vec<IntegrationsLocationPublicApiViewModelsServiceModel>,
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
pub struct IntegrationsLocationPublicApiViewModelsInternalModel {
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
/// - `IntegrationsLocationPublicApiViewModelsModel`
/// - `IntegrationsLocationPublicApiViewModelsInternalModel`
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct IntegrationsLocationPublicApiViewModelsInternalModelAllOf {
    #[serde(flatten)]
    pub integrations_location_public_api_view_models_model:
        IntegrationsLocationPublicApiViewModelsModel,
    #[serde(flatten)]
    pub integrations_location_public_api_view_models_internal_model:
        IntegrationsLocationPublicApiViewModelsInternalModel,
}

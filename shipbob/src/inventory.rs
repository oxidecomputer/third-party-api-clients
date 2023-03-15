use crate::Client;
use crate::ClientResult;

pub struct Inventory {
    pub client: Client,
}

impl Inventory {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Inventory { client }
    }

    /**
     * Get an inventory item.
     *
     * This function performs a `GET` to the `/inventory/{inventoryId}` endpoint.
     *
     * **Parameters:**
     *
     * * `inventory_id: i64` -- Unique id of the channel.
     */
    pub async fn get(&self, inventory_id: i64) -> ClientResult<crate::types::Inventory> {
        let url = self.client.url(
            &format!(
                "/inventory/{}",
                crate::progenitor_support::encode_path(&inventory_id.to_string()),
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
     * List inventory items.
     *
     * This function performs a `GET` to the `/inventory` endpoint.
     *
     * **Parameters:**
     *
     * * `page: i64` -- Page of inventory items to get.
     * * `limit: i64` -- Amount of inventory items per page to request.
     * * `is_active: bool` -- True if the inventory item is marked as a digital item.
     * * `is_digital: bool` -- True if the inventory item is marked as a digital item.
     * * `i_ds: &[String]` -- Comma separated inventory ids to filter by.
     * * `sort: &str` -- Sort will default to ascending order for each field.
     *   To sort in descending order please pass a "-" in front of the field name.
     *   For example, Sort=-onHand,name will sort by onHand descending.
     * * `search: &str` -- Search is available for 2 fields, Inventory ID and Name -
     *   1. Expected behavior for search by Inventory ID is exact match
     *   2. Expected behavior for search by Inventory Name is partial match, i.e. does not have to be start of word,
     *   but must be consecutive characters. This is not case sensitive.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_page(
        &self,
        page: i64,
        limit: i64,
        is_active: bool,
        is_digital: bool,
        ids: &[String],
        sort: &str,
        search: &str,
    ) -> ClientResult<Vec<crate::types::Inventory>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ids.is_empty() {
            query_args.push(("IDs".to_string(), ids.join(" ")));
        }
        if is_active {
            query_args.push(("IsActive".to_string(), is_active.to_string()));
        }
        if is_digital {
            query_args.push(("IsDigital".to_string(), is_digital.to_string()));
        }
        if limit > 0 {
            query_args.push(("Limit".to_string(), limit.to_string()));
        }
        if page > 0 {
            query_args.push(("Page".to_string(), page.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("Search".to_string(), search.to_string()));
        }
        if !sort.is_empty() {
            query_args.push(("Sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/inventory?{}", query_), None);
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
     * List inventory items.
     *
     * This function performs a `GET` to the `/inventory` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        is_active: bool,
        is_digital: bool,
        ids: &[String],
        sort: &str,
        search: &str,
    ) -> ClientResult<Vec<crate::types::Inventory>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ids.is_empty() {
            query_args.push(("IDs".to_string(), ids.join(" ")));
        }
        if is_active {
            query_args.push(("IsActive".to_string(), is_active.to_string()));
        }
        if is_digital {
            query_args.push(("IsDigital".to_string(), is_digital.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("Search".to_string(), search.to_string()));
        }
        if !sort.is_empty() {
            query_args.push(("Sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/inventory?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a list of inventory items by product id.
     *
     * This function performs a `GET` to the `/product/{productId}/inventory` endpoint.
     *
     * **Parameters:**
     *
     * * `product_id: i64` -- The product id to get inventory for.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_product(&self, product_id: i64) -> ClientResult<Vec<crate::types::Inventory>> {
        let url = self.client.url(
            &format!(
                "/product/{}/inventory",
                crate::progenitor_support::encode_path(&product_id.to_string()),
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
     * Get a list of inventory items by product id.
     *
     * This function performs a `GET` to the `/product/{productId}/inventory` endpoint.
     *
     * As opposed to `get_product`, this function returns all the pages of the request at once.
     */
    pub async fn get_all_product(
        &self,
        product_id: i64,
    ) -> ClientResult<Vec<crate::types::Inventory>> {
        let url = self.client.url(
            &format!(
                "/product/{}/inventory",
                crate::progenitor_support::encode_path(&product_id.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}

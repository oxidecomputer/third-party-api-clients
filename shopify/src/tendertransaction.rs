use crate::Client;
use crate::ClientResult;

pub struct Tendertransaction {
    pub client: Client,
}

impl Tendertransaction {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Tendertransaction { client }
    }

    /**
     * Retrieves a list of tender transactions. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/tender_transactions.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/tendertransaction#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Retrieve only transactions after the specified ID.
     * * `processed_at_min: &str` -- Show tender transactions processed_at or after the specified date.
     * * `processed_at_max: &str` -- Show tender transactions processed_at or before the specified date.
     * * `processed_at: &str` -- Show tender transactions processed at the specified date.
     * * `order: &str` -- Show tender transactions ordered by processed_at in ascending or descending order.
     */
    pub async fn deprecated_202001_get_tender_transaction(
        &self,
        limit: &str,
        since_id: &str,
        processed_at_min: &str,
        processed_at_max: &str,
        processed_at: &str,
        order: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !processed_at.is_empty() {
            query_args.push(("processed_at".to_string(), processed_at.to_string()));
        }
        if !processed_at_max.is_empty() {
            query_args.push(("processed_at_max".to_string(), processed_at_max.to_string()));
        }
        if !processed_at_min.is_empty() {
            query_args.push(("processed_at_min".to_string(), processed_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/tender_transactions.json?{}", query_),
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
     * Retrieves a list of tender transactions. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/tender_transactions.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/tendertransaction#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Retrieve only transactions after the specified ID.
     * * `processed_at_min: &str` -- Show tender transactions processed_at or after the specified date.
     * * `processed_at_max: &str` -- Show tender transactions processed_at or before the specified date.
     * * `processed_at: &str` -- Show tender transactions processed at the specified date.
     * * `order: &str` -- Show tender transactions ordered by processed_at in ascending or descending order.
     */
    pub async fn deprecated_202004_get_tender_transaction(
        &self,
        limit: &str,
        since_id: &str,
        processed_at_min: &str,
        processed_at_max: &str,
        processed_at: &str,
        order: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !processed_at.is_empty() {
            query_args.push(("processed_at".to_string(), processed_at.to_string()));
        }
        if !processed_at_max.is_empty() {
            query_args.push(("processed_at_max".to_string(), processed_at_max.to_string()));
        }
        if !processed_at_min.is_empty() {
            query_args.push(("processed_at_min".to_string(), processed_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/tender_transactions.json?{}", query_),
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
     * Retrieves a list of tender transactions. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/tender_transactions.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/tendertransaction#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Retrieve only transactions after the specified ID.
     * * `processed_at_min: &str` -- Show tender transactions processed_at or after the specified date.
     * * `processed_at_max: &str` -- Show tender transactions processed_at or before the specified date.
     * * `processed_at: &str` -- Show tender transactions processed at the specified date.
     * * `order: &str` -- Show tender transactions ordered by processed_at in ascending or descending order.
     */
    pub async fn deprecated_202007_get_tender_transaction(
        &self,
        limit: &str,
        since_id: &str,
        processed_at_min: &str,
        processed_at_max: &str,
        processed_at: &str,
        order: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !processed_at.is_empty() {
            query_args.push(("processed_at".to_string(), processed_at.to_string()));
        }
        if !processed_at_max.is_empty() {
            query_args.push(("processed_at_max".to_string(), processed_at_max.to_string()));
        }
        if !processed_at_min.is_empty() {
            query_args.push(("processed_at_min".to_string(), processed_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/tender_transactions.json?{}", query_),
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
     * Retrieves a list of tender transactions. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/tender_transactions.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/tendertransaction#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Retrieve only transactions after the specified ID.
     * * `processed_at_min: &str` -- Show tender transactions processed_at or after the specified date.
     * * `processed_at_max: &str` -- Show tender transactions processed_at or before the specified date.
     * * `processed_at: &str` -- Show tender transactions processed at the specified date.
     * * `order: &str` -- Show tender transactions ordered by processed_at in ascending or descending order.
     */
    pub async fn get_tender_transaction(
        &self,
        limit: &str,
        since_id: &str,
        processed_at_min: &str,
        processed_at_max: &str,
        processed_at: &str,
        order: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !processed_at.is_empty() {
            query_args.push(("processed_at".to_string(), processed_at.to_string()));
        }
        if !processed_at_max.is_empty() {
            query_args.push(("processed_at_max".to_string(), processed_at_max.to_string()));
        }
        if !processed_at_min.is_empty() {
            query_args.push(("processed_at_min".to_string(), processed_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/tender_transactions.json?{}", query_),
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
     * Retrieves a list of tender transactions. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/tender_transactions.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/tendertransaction#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Retrieve only transactions after the specified ID.
     * * `processed_at_min: &str` -- Show tender transactions processed_at or after the specified date.
     * * `processed_at_max: &str` -- Show tender transactions processed_at or before the specified date.
     * * `processed_at: &str` -- Show tender transactions processed at the specified date.
     * * `order: &str` -- Show tender transactions ordered by processed_at in ascending or descending order.
     */
    pub async fn deprecated_202101_get_tender_transaction(
        &self,
        limit: &str,
        since_id: &str,
        processed_at_min: &str,
        processed_at_max: &str,
        processed_at: &str,
        order: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !processed_at.is_empty() {
            query_args.push(("processed_at".to_string(), processed_at.to_string()));
        }
        if !processed_at_max.is_empty() {
            query_args.push(("processed_at_max".to_string(), processed_at_max.to_string()));
        }
        if !processed_at_min.is_empty() {
            query_args.push(("processed_at_min".to_string(), processed_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/tender_transactions.json?{}", query_),
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
     * Retrieves a list of tender transactions. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/tender_transactions.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/tendertransaction#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Retrieve only transactions after the specified ID.
     * * `processed_at_min: &str` -- Show tender transactions processed_at or after the specified date.
     * * `processed_at_max: &str` -- Show tender transactions processed_at or before the specified date.
     * * `processed_at: &str` -- Show tender transactions processed at the specified date.
     * * `order: &str` -- Show tender transactions ordered by processed_at in ascending or descending order.
     */
    pub async fn deprecated_unstable_get_tender_transaction(
        &self,
        limit: &str,
        since_id: &str,
        processed_at_min: &str,
        processed_at_max: &str,
        processed_at: &str,
        order: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !processed_at.is_empty() {
            query_args.push(("processed_at".to_string(), processed_at.to_string()));
        }
        if !processed_at_max.is_empty() {
            query_args.push(("processed_at_max".to_string(), processed_at_max.to_string()));
        }
        if !processed_at_min.is_empty() {
            query_args.push(("processed_at_min".to_string(), processed_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/tender_transactions.json?{}", query_),
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
}

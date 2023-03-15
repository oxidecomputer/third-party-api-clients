use crate::Client;
use crate::ClientResult;

pub struct Plus {
    pub client: Client,
}

impl Plus {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Plus { client }
    }

    /**
     * Retrieves a list of gift cards. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#index-2020-01
     *
     * **Parameters:**
     *
     * * `status: &str` -- Retrieve gift cards with a given status. Valid values:
     *                       
     *                           enabled: Restrict results to only enabled gift cards
     *                           disabled: Restrict results to only disabled gift cards.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_gift_card(
        &self,
        status: &str,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/gift_cards.json?{}", query_),
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
     * Creates a gift card.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#create-2020-01
     */
    pub async fn deprecated_202001_create_gift_cards(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/gift_cards.json", None);
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
     * Retrieves a single gift card by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/gift_cards/{gift_card_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#show-2020-01
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_gift_cards_param_card(
        &self,
        gift_card_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Updates an existing gift card.
              The gift card's balance can't be changed via the API. You can change only the expiry date, note, and template suffix.
    *
    * This function performs a `PUT` to the `/admin/api/2020-01/gift_cards/{gift_card_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#update-2020-01
    *
    * **Parameters:**
    *
    * * `gift_card_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_update_gift_cards_param_card(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
     * Retrieves a count of gift cards.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/gift_cards/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#count-2020-01
     *
     * **Parameters:**
     *
     * * `status: &str` -- Count gift cards with a given status. Valid values:
     *                       
     *                           enabled: Count only enabled gift cards
     *                           disabled: Count only disabled gift cards.
     */
    pub async fn deprecated_202001_get_gift_cards_count(&self, status: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/gift_cards/count.json?{}", query_),
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
     * Disables a gift card. Disabling a gift card can't be undone.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/gift_cards/{gift_card_id}/disable.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#disable-2020-01
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_gift_cards_param_card_disable(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/gift_cards/{}/disable.json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Searches for gift cards that match a supplied query. The following fields are indexed by search:

                created_at
                updated_at
                disabled_at
                balance
                initial_value
                amount_spent
                email
                last_characters

              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/gift_cards/search.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#search-2020-01
    *
    * **Parameters:**
    *
    * * `order: &str` -- The field and direction to order results by.
    *                     (default: disabled_at DESC).
    * * `query: &str` -- The text to search for.
    * * `limit: &str` -- The maximum number of results to retrieve.
    *                     (default: 50, maximum: 250).
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    */
    pub async fn deprecated_202001_get_gift_cards_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/gift_cards/search.json?{}", query_),
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
     * Retrieves a list of gift cards. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#index-2020-04
     *
     * **Parameters:**
     *
     * * `status: &str` -- Retrieve gift cards with a given status. Valid values:
     *                       
     *                           enabled: Restrict results to only enabled gift cards
     *                           disabled: Restrict results to only disabled gift cards.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_gift_card(
        &self,
        status: &str,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/gift_cards.json?{}", query_),
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
     * Creates a gift card.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#create-2020-04
     */
    pub async fn deprecated_202004_create_gift_cards(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/gift_cards.json", None);
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
     * Retrieves a single gift card by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/gift_cards/{gift_card_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#show-2020-04
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_gift_cards_param_card(
        &self,
        gift_card_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Updates an existing gift card.
              The gift card's balance can't be changed via the API. You can change only the expiry date, note, and template suffix.
    *
    * This function performs a `PUT` to the `/admin/api/2020-04/gift_cards/{gift_card_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#update-2020-04
    *
    * **Parameters:**
    *
    * * `gift_card_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_update_gift_cards_param_card(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
     * Retrieves a count of gift cards.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/gift_cards/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#count-2020-04
     *
     * **Parameters:**
     *
     * * `status: &str` -- Count gift cards with a given status. Valid values:
     *                       
     *                           enabled: Count only enabled gift cards
     *                           disabled: Count only disabled gift cards.
     */
    pub async fn deprecated_202004_get_gift_cards_count(&self, status: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/gift_cards/count.json?{}", query_),
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
     * Disables a gift card. Disabling a gift card can't be undone.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/gift_cards/{gift_card_id}/disable.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#disable-2020-04
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_gift_cards_param_card_disable(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/gift_cards/{}/disable.json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Searches for gift cards that match a supplied query. The following fields are indexed by search:

                created_at
                updated_at
                disabled_at
                balance
                initial_value
                amount_spent
                email
                last_characters

              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-04/gift_cards/search.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#search-2020-04
    *
    * **Parameters:**
    *
    * * `order: &str` -- The field and direction to order results by.
    *                     (default: disabled_at DESC).
    * * `query: &str` -- The text to search for.
    * * `limit: &str` -- The maximum number of results to retrieve.
    *                     (default: 50, maximum: 250).
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    */
    pub async fn deprecated_202004_get_gift_cards_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/gift_cards/search.json?{}", query_),
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
     * Retrieves a list of gift cards. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#index-2020-07
     *
     * **Parameters:**
     *
     * * `status: &str` -- Retrieve gift cards with a given status. Valid values:
     *                       
     *                           enabled: Restrict results to only enabled gift cards
     *                           disabled: Restrict results to only disabled gift cards.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_gift_card(
        &self,
        status: &str,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/gift_cards.json?{}", query_),
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
     * Creates a gift card.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#create-2020-07
     */
    pub async fn deprecated_202007_create_gift_cards(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/gift_cards.json", None);
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
     * Retrieves a single gift card by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/gift_cards/{gift_card_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#show-2020-07
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_gift_cards_param_card(
        &self,
        gift_card_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Updates an existing gift card.
              The gift card's balance can't be changed via the API. You can change only the expiry date, note, and template suffix.
    *
    * This function performs a `PUT` to the `/admin/api/2020-07/gift_cards/{gift_card_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#update-2020-07
    *
    * **Parameters:**
    *
    * * `gift_card_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_update_gift_cards_param_card(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
     * Retrieves a count of gift cards.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/gift_cards/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#count-2020-07
     *
     * **Parameters:**
     *
     * * `status: &str` -- Count gift cards with a given status. Valid values:
     *                       
     *                           enabled: Count only enabled gift cards
     *                           disabled: Count only disabled gift cards.
     */
    pub async fn deprecated_202007_get_gift_cards_count(&self, status: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/gift_cards/count.json?{}", query_),
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
     * Disables a gift card. Disabling a gift card can't be undone.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/gift_cards/{gift_card_id}/disable.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#disable-2020-07
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_gift_cards_param_card_disable(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/gift_cards/{}/disable.json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Searches for gift cards that match a supplied query. The following fields are indexed by search:

                created_at
                updated_at
                disabled_at
                balance
                initial_value
                amount_spent
                email
                last_characters

              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-07/gift_cards/search.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#search-2020-07
    *
    * **Parameters:**
    *
    * * `order: &str` -- The field and direction to order results by.
    *                     (default: disabled_at DESC).
    * * `query: &str` -- The text to search for.
    * * `limit: &str` -- The maximum number of results to retrieve.
    *                     (default: 50, maximum: 250).
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    */
    pub async fn deprecated_202007_get_gift_cards_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/gift_cards/search.json?{}", query_),
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
     * Retrieves a list of gift cards. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#index-2020-10
     *
     * **Parameters:**
     *
     * * `status: &str` -- Retrieve gift cards with a given status. Valid values:
     *                       
     *                           enabled: Restrict results to only enabled gift cards
     *                           disabled: Restrict results to only disabled gift cards.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_gift_card(
        &self,
        status: &str,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/gift_cards.json?{}", query_),
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
     * Creates a gift card.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#create-2020-10
     */
    pub async fn create_gift_cards(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/gift_cards.json", None);
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
     * Retrieves a single gift card by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/gift_cards/{gift_card_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#show-2020-10
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_gift_cards_param_card(&self, gift_card_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Updates an existing gift card.
              The gift card's balance can't be changed via the API. You can change only the expiry date, note, and template suffix.
    *
    * This function performs a `PUT` to the `/admin/api/2020-10/gift_cards/{gift_card_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#update-2020-10
    *
    * **Parameters:**
    *
    * * `gift_card_id: &str` -- storefront_access_token_id.
    */
    pub async fn update_gift_cards_param_card(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
     * Retrieves a count of gift cards.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/gift_cards/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#count-2020-10
     *
     * **Parameters:**
     *
     * * `status: &str` -- Count gift cards with a given status. Valid values:
     *                       
     *                           enabled: Count only enabled gift cards
     *                           disabled: Count only disabled gift cards.
     */
    pub async fn get_gift_cards_count(&self, status: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/gift_cards/count.json?{}", query_),
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
     * Disables a gift card. Disabling a gift card can't be undone.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/gift_cards/{gift_card_id}/disable.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#disable-2020-10
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_gift_cards_param_card_disable(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/gift_cards/{}/disable.json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Searches for gift cards that match a supplied query. The following fields are indexed by search:

                created_at
                updated_at
                disabled_at
                balance
                initial_value
                amount_spent
                email
                last_characters

              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-10/gift_cards/search.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#search-2020-10
    *
    * **Parameters:**
    *
    * * `order: &str` -- The field and direction to order results by.
    *                     (default: disabled_at DESC).
    * * `query: &str` -- The text to search for.
    * * `limit: &str` -- The maximum number of results to retrieve.
    *                     (default: 50, maximum: 250).
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    */
    pub async fn get_gift_cards_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/gift_cards/search.json?{}", query_),
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
     * Retrieves a list of gift cards. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#index-2021-01
     *
     * **Parameters:**
     *
     * * `status: &str` -- Retrieve gift cards with a given status. Valid values:
     *                       
     *                           enabled: Restrict results to only enabled gift cards
     *                           disabled: Restrict results to only disabled gift cards.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_gift_card(
        &self,
        status: &str,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/gift_cards.json?{}", query_),
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
     * Creates a gift card.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#create-2021-01
     */
    pub async fn deprecated_202101_create_gift_cards(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/gift_cards.json", None);
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
     * Retrieves a single gift card by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/gift_cards/{gift_card_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#show-2021-01
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_gift_cards_param_card(
        &self,
        gift_card_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Updates an existing gift card.
              The gift card's balance can't be changed via the API. You can change only the expiry date, note, and template suffix.
    *
    * This function performs a `PUT` to the `/admin/api/2021-01/gift_cards/{gift_card_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#update-2021-01
    *
    * **Parameters:**
    *
    * * `gift_card_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_update_gift_cards_param_card(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
     * Retrieves a count of gift cards.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/gift_cards/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#count-2021-01
     *
     * **Parameters:**
     *
     * * `status: &str` -- Count gift cards with a given status. Valid values:
     *                       
     *                           enabled: Count only enabled gift cards
     *                           disabled: Count only disabled gift cards.
     */
    pub async fn deprecated_202101_get_gift_cards_count(&self, status: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/gift_cards/count.json?{}", query_),
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
     * Disables a gift card. Disabling a gift card can't be undone.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/gift_cards/{gift_card_id}/disable.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#disable-2021-01
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_gift_cards_param_card_disable(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/gift_cards/{}/disable.json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Searches for gift cards that match a supplied query. The following fields are indexed by search:

                created_at
                updated_at
                disabled_at
                balance
                initial_value
                amount_spent
                email
                last_characters

              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2021-01/gift_cards/search.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#search-2021-01
    *
    * **Parameters:**
    *
    * * `order: &str` -- The field and direction to order results by.
    *                     (default: disabled_at DESC).
    * * `query: &str` -- The text to search for.
    * * `limit: &str` -- The maximum number of results to retrieve.
    *                     (default: 50, maximum: 250).
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    */
    pub async fn deprecated_202101_get_gift_cards_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/gift_cards/search.json?{}", query_),
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
     * Retrieves a list of gift cards. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#index-unstable
     *
     * **Parameters:**
     *
     * * `status: &str` -- Retrieve gift cards with a given status. Valid values:
     *                       
     *                           enabled: Restrict results to only enabled gift cards
     *                           disabled: Restrict results to only disabled gift cards.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_gift_card(
        &self,
        status: &str,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/gift_cards.json?{}", query_),
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
     * Creates a gift card.
     *
     * This function performs a `POST` to the `/admin/api/unstable/gift_cards.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#create-unstable
     */
    pub async fn deprecated_unstable_create_gift_cards(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/gift_cards.json", None);
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
     * Retrieves a single gift card by its ID.
     *
     * This function performs a `GET` to the `/admin/api/unstable/gift_cards/{gift_card_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#show-unstable
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_gift_cards_param_card(
        &self,
        gift_card_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Updates an existing gift card.
              The gift card's balance can't be changed via the API. You can change only the expiry date, note, and template suffix.
    *
    * This function performs a `PUT` to the `/admin/api/unstable/gift_cards/{gift_card_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#update-unstable
    *
    * **Parameters:**
    *
    * * `gift_card_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_update_gift_cards_param_card(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/gift_cards/{}/json",
                crate::progenitor_support::encode_path(gift_card_id),
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
     * Retrieves a count of gift cards.
     *
     * This function performs a `GET` to the `/admin/api/unstable/gift_cards/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#count-unstable
     *
     * **Parameters:**
     *
     * * `status: &str` -- Count gift cards with a given status. Valid values:
     *                       
     *                           enabled: Count only enabled gift cards
     *                           disabled: Count only disabled gift cards.
     */
    pub async fn deprecated_unstable_get_gift_cards_count(&self, status: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/gift_cards/count.json?{}", query_),
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
     * Disables a gift card. Disabling a gift card can't be undone.
     *
     * This function performs a `POST` to the `/admin/api/unstable/gift_cards/{gift_card_id}/disable.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#disable-unstable
     *
     * **Parameters:**
     *
     * * `gift_card_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_gift_cards_param_card_disable(
        &self,
        gift_card_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/gift_cards/{}/disable.json",
                crate::progenitor_support::encode_path(gift_card_id),
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
    * Searches for gift cards that match a supplied query. The following fields are indexed by search:

                created_at
                updated_at
                disabled_at
                balance
                initial_value
                amount_spent
                email
                last_characters

              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/unstable/gift_cards/search.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/plus/giftcard#search-unstable
    *
    * **Parameters:**
    *
    * * `order: &str` -- The field and direction to order results by.
    *                     (default: disabled_at DESC).
    * * `query: &str` -- The text to search for.
    * * `limit: &str` -- The maximum number of results to retrieve.
    *                     (default: 50, maximum: 250).
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    */
    pub async fn deprecated_unstable_get_gift_cards_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/gift_cards/search.json?{}", query_),
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
     * Retrieves a list of all users. Note: As of version 2021-01, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/users.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show on a page.
     *                     (default: 50, maximum: 250).
     * * `page_info: &str` -- A unique ID used to access a certain page of results.
     */
    pub async fn deprecated_202001_get_user(
        &self,
        limit: &str,
        page_info: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_info.is_empty() {
            query_args.push(("page_info".to_string(), page_info.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-01/users.json?{}", query_), None);
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
     * Retrieves a single user.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/users/{user_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#show-2020-01
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_users_param_user(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/users/{}/json",
                crate::progenitor_support::encode_path(user_id),
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
     * Retrieves information about the user account associated with the access token used to make this API request. This request works only when the access token was created for a specific user of the shop.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/users/current.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#current-2020-01
     */
    pub async fn deprecated_202001_get_users_current(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/users/current.json", None);
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
     * Retrieves a list of all users. Note: As of version 2021-01, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/users.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show on a page.
     *                     (default: 50, maximum: 250).
     * * `page_info: &str` -- A unique ID used to access a certain page of results.
     */
    pub async fn deprecated_202004_get_user(
        &self,
        limit: &str,
        page_info: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_info.is_empty() {
            query_args.push(("page_info".to_string(), page_info.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-04/users.json?{}", query_), None);
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
     * Retrieves a single user.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/users/{user_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#show-2020-04
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_users_param_user(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/users/{}/json",
                crate::progenitor_support::encode_path(user_id),
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
     * Retrieves information about the user account associated with the access token used to make this API request. This request works only when the access token was created for a specific user of the shop.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/users/current.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#current-2020-04
     */
    pub async fn deprecated_202004_get_users_current(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/users/current.json", None);
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
     * Retrieves a list of all users. Note: As of version 2021-01, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/users.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show on a page.
     *                     (default: 50, maximum: 250).
     * * `page_info: &str` -- A unique ID used to access a certain page of results.
     */
    pub async fn deprecated_202007_get_user(
        &self,
        limit: &str,
        page_info: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_info.is_empty() {
            query_args.push(("page_info".to_string(), page_info.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-07/users.json?{}", query_), None);
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
     * Retrieves a single user.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/users/{user_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#show-2020-07
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_users_param_user(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/users/{}/json",
                crate::progenitor_support::encode_path(user_id),
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
     * Retrieves information about the user account associated with the access token used to make this API request. This request works only when the access token was created for a specific user of the shop.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/users/current.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#current-2020-07
     */
    pub async fn deprecated_202007_get_users_current(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/users/current.json", None);
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
     * Retrieves a list of all users. Note: As of version 2021-01, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/users.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show on a page.
     *                     (default: 50, maximum: 250).
     * * `page_info: &str` -- A unique ID used to access a certain page of results.
     */
    pub async fn get_user(&self, limit: &str, page_info: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_info.is_empty() {
            query_args.push(("page_info".to_string(), page_info.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-10/users.json?{}", query_), None);
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
     * Retrieves a single user.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/users/{user_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#show-2020-10
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_users_param_user(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/users/{}/json",
                crate::progenitor_support::encode_path(user_id),
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
     * Retrieves information about the user account associated with the access token used to make this API request. This request works only when the access token was created for a specific user of the shop.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/users/current.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#current-2020-10
     */
    pub async fn get_users_current(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/users/current.json", None);
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
     * Retrieves a list of all users. Note: As of version 2021-01, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/users.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show on a page.
     *                     (default: 50, maximum: 250).
     * * `page_info: &str` -- A unique ID used to access a certain page of results.
     */
    pub async fn deprecated_202101_get_user(
        &self,
        limit: &str,
        page_info: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_info.is_empty() {
            query_args.push(("page_info".to_string(), page_info.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2021-01/users.json?{}", query_), None);
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
     * Retrieves a single user.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/users/{user_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#show-2021-01
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_users_param_user(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/users/{}/json",
                crate::progenitor_support::encode_path(user_id),
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
     * Retrieves information about the user account associated with the access token used to make this API request. This request works only when the access token was created for a specific user of the shop.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/users/current.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#current-2021-01
     */
    pub async fn deprecated_202101_get_users_current(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/users/current.json", None);
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
     * Retrieves a list of all users. Note: As of version 2021-01, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/users.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show on a page.
     *                     (default: 50, maximum: 250).
     * * `page_info: &str` -- A unique ID used to access a certain page of results.
     */
    pub async fn deprecated_unstable_get_user(
        &self,
        limit: &str,
        page_info: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_info.is_empty() {
            query_args.push(("page_info".to_string(), page_info.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/unstable/users.json?{}", query_), None);
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
     * Retrieves a single user.
     *
     * This function performs a `GET` to the `/admin/api/unstable/users/{user_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#show-unstable
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_users_param_user(
        &self,
        user_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/users/{}/json",
                crate::progenitor_support::encode_path(user_id),
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
     * Retrieves information about the user account associated with the access token used to make this API request. This request works only when the access token was created for a specific user of the shop.
     *
     * This function performs a `GET` to the `/admin/api/unstable/users/current.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/plus/user#current-unstable
     */
    pub async fn deprecated_unstable_get_users_current(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/users/current.json", None);
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

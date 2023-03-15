use crate::Client;
use crate::ClientResult;

pub struct Events {
    pub client: Client,
}

impl Events {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Events { client }
    }

    /**
     * Retrieves a list of events. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Show only results after the specified ID.
     * * `created_at_min: &str` -- Show events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `filter: &str` -- Show events specified in this filter.
     * * `verb: &str` -- Show events of a certain type.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        filter: &str,
        verb: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !verb.is_empty() {
            query_args.push(("verb".to_string(), verb.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-01/events.json?{}", query_), None);
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
     * Retrieves a single event by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#show-2020-01
     *
     * **Parameters:**
     *
     * * `event_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_param(
        &self,
        event_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/events/{}/json?{}",
                crate::progenitor_support::encode_path(event_id),
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
     * Retrieves a count of events.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/events/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#count-2020-01
     *
     * **Parameters:**
     *
     * * `created_at_min: &str` -- Count only events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count only events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202001_get_count(
        &self,
        created_at_min: &str,
        created_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/events/count.json?{}", query_),
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
     * Retrieves a list of events. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Show only results after the specified ID.
     * * `created_at_min: &str` -- Show events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `filter: &str` -- Show events specified in this filter.
     * * `verb: &str` -- Show events of a certain type.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        filter: &str,
        verb: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !verb.is_empty() {
            query_args.push(("verb".to_string(), verb.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-04/events.json?{}", query_), None);
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
     * Retrieves a single event by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#show-2020-04
     *
     * **Parameters:**
     *
     * * `event_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_param(
        &self,
        event_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/events/{}/json?{}",
                crate::progenitor_support::encode_path(event_id),
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
     * Retrieves a count of events.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/events/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#count-2020-04
     *
     * **Parameters:**
     *
     * * `created_at_min: &str` -- Count only events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count only events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202004_get_count(
        &self,
        created_at_min: &str,
        created_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/events/count.json?{}", query_),
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
     * Retrieves a list of events. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Show only results after the specified ID.
     * * `created_at_min: &str` -- Show events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `filter: &str` -- Show events specified in this filter.
     * * `verb: &str` -- Show events of a certain type.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        filter: &str,
        verb: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !verb.is_empty() {
            query_args.push(("verb".to_string(), verb.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-07/events.json?{}", query_), None);
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
     * Retrieves a single event by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#show-2020-07
     *
     * **Parameters:**
     *
     * * `event_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_param(
        &self,
        event_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/events/{}/json?{}",
                crate::progenitor_support::encode_path(event_id),
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
     * Retrieves a count of events.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/events/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#count-2020-07
     *
     * **Parameters:**
     *
     * * `created_at_min: &str` -- Count only events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count only events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202007_get_count(
        &self,
        created_at_min: &str,
        created_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/events/count.json?{}", query_),
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
     * Retrieves a list of events. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Show only results after the specified ID.
     * * `created_at_min: &str` -- Show events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `filter: &str` -- Show events specified in this filter.
     * * `verb: &str` -- Show events of a certain type.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        filter: &str,
        verb: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !verb.is_empty() {
            query_args.push(("verb".to_string(), verb.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-10/events.json?{}", query_), None);
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
     * Retrieves a single event by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#show-2020-10
     *
     * **Parameters:**
     *
     * * `event_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_param(&self, event_id: &str, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/events/{}/json?{}",
                crate::progenitor_support::encode_path(event_id),
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
     * Retrieves a count of events.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/events/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#count-2020-10
     *
     * **Parameters:**
     *
     * * `created_at_min: &str` -- Count only events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count only events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn get_count(&self, created_at_min: &str, created_at_max: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/events/count.json?{}", query_),
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
     * Retrieves a list of events. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Show only results after the specified ID.
     * * `created_at_min: &str` -- Show events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `filter: &str` -- Show events specified in this filter.
     * * `verb: &str` -- Show events of a certain type.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        filter: &str,
        verb: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !verb.is_empty() {
            query_args.push(("verb".to_string(), verb.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2021-01/events.json?{}", query_), None);
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
     * Retrieves a single event by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#show-2021-01
     *
     * **Parameters:**
     *
     * * `event_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_param(
        &self,
        event_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/events/{}/json?{}",
                crate::progenitor_support::encode_path(event_id),
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
     * Retrieves a count of events.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/events/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#count-2021-01
     *
     * **Parameters:**
     *
     * * `created_at_min: &str` -- Count only events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count only events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202101_get_count(
        &self,
        created_at_min: &str,
        created_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/events/count.json?{}", query_),
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
     * Retrieves a list of events. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Show only results after the specified ID.
     * * `created_at_min: &str` -- Show events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `filter: &str` -- Show events specified in this filter.
     * * `verb: &str` -- Show events of a certain type.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        filter: &str,
        verb: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !verb.is_empty() {
            query_args.push(("verb".to_string(), verb.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/unstable/events.json?{}", query_), None);
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
     * Retrieves a single event by its ID.
     *
     * This function performs a `GET` to the `/admin/api/unstable/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#show-unstable
     *
     * **Parameters:**
     *
     * * `event_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_param(
        &self,
        event_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/events/{}/json?{}",
                crate::progenitor_support::encode_path(event_id),
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
     * Retrieves a count of events.
     *
     * This function performs a `GET` to the `/admin/api/unstable/events/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/event#count-unstable
     *
     * **Parameters:**
     *
     * * `created_at_min: &str` -- Count only events created at or after this date and time. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count only events created at or before this date and time. (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_unstable_get_count(
        &self,
        created_at_min: &str,
        created_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/events/count.json?{}", query_),
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
     * Retrieves a list of webhooks. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#index-2020-01
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `created_at_max: &str` -- Retrieve webhook subscriptions that were created before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Retrieve webhook subscriptions that were created after a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     * * `limit: &str` -- Maximum number of webhook subscriptions that should be returned. Setting this parameter outside the maximum range will return an error.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict the returned list to webhook subscriptions whose id is greater than the specified since_id.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     * * `updated_at_min: &str` -- Retrieve webhooks that were updated before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Retrieve webhooks that were updated after a given date and time (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202001_get_webhook(
        &self,
        address: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        topic: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/webhooks.json?{}", query_),
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
     * Create a new webhook subscription by specifying both an address and a topic.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#create-2020-01
     *
     * **Parameters:**
     *
     * * `format: &str` -- Use this parameter to select the data format for the payload. Valid values are json and xml.
     *                     (default: json).
     */
    pub async fn deprecated_202001_create_webhooks(
        &self,
        format: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !format.is_empty() {
            query_args.push(("format".to_string(), format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/webhooks.json?{}", query_),
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
     * Retrieves a count of existing webhook subscriptions.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/webhooks/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#count-2020-01
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     */
    pub async fn deprecated_202001_get_webhooks_count(
        &self,
        address: &str,
        topic: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/webhooks/count.json?{}", query_),
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
     * Retrieves a single webhook subscription.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#show-2020-01
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     */
    pub async fn deprecated_202001_get_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/webhooks/{}/json?{}",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Update a webhook subscription's topic or address URIs.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#update-2020-01
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Delete a webhook subscription.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_webhooks_param_webhook(
        &self,
        webhook_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of webhooks. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#index-2020-04
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `created_at_max: &str` -- Retrieve webhook subscriptions that were created before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Retrieve webhook subscriptions that were created after a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     * * `limit: &str` -- Maximum number of webhook subscriptions that should be returned. Setting this parameter outside the maximum range will return an error.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict the returned list to webhook subscriptions whose id is greater than the specified since_id.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     * * `updated_at_min: &str` -- Retrieve webhooks that were updated before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Retrieve webhooks that were updated after a given date and time (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202004_get_webhook(
        &self,
        address: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        topic: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/webhooks.json?{}", query_),
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
     * Create a new webhook subscription by specifying both an address and a topic.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#create-2020-04
     *
     * **Parameters:**
     *
     * * `format: &str` -- Use this parameter to select the data format for the payload. Valid values are json and xml.
     *                     (default: json).
     */
    pub async fn deprecated_202004_create_webhooks(
        &self,
        format: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !format.is_empty() {
            query_args.push(("format".to_string(), format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/webhooks.json?{}", query_),
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
     * Retrieves a count of existing webhook subscriptions.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/webhooks/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#count-2020-04
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     */
    pub async fn deprecated_202004_get_webhooks_count(
        &self,
        address: &str,
        topic: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/webhooks/count.json?{}", query_),
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
     * Retrieves a single webhook subscription.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#show-2020-04
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     */
    pub async fn deprecated_202004_get_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/webhooks/{}/json?{}",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Update a webhook subscription's topic or address URIs.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#update-2020-04
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Delete a webhook subscription.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_webhooks_param_webhook(
        &self,
        webhook_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of webhooks. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#index-2020-07
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `created_at_max: &str` -- Retrieve webhook subscriptions that were created before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Retrieve webhook subscriptions that were created after a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     * * `limit: &str` -- Maximum number of webhook subscriptions that should be returned. Setting this parameter outside the maximum range will return an error.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict the returned list to webhook subscriptions whose id is greater than the specified since_id.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     * * `updated_at_min: &str` -- Retrieve webhooks that were updated before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Retrieve webhooks that were updated after a given date and time (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202007_get_webhook(
        &self,
        address: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        topic: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/webhooks.json?{}", query_),
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
     * Create a new webhook subscription by specifying both an address and a topic.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#create-2020-07
     *
     * **Parameters:**
     *
     * * `format: &str` -- Use this parameter to select the data format for the payload. Valid values are json and xml.
     *                     (default: json).
     */
    pub async fn deprecated_202007_create_webhooks(
        &self,
        format: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !format.is_empty() {
            query_args.push(("format".to_string(), format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/webhooks.json?{}", query_),
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
     * Retrieves a count of existing webhook subscriptions.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/webhooks/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#count-2020-07
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     */
    pub async fn deprecated_202007_get_webhooks_count(
        &self,
        address: &str,
        topic: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/webhooks/count.json?{}", query_),
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
     * Retrieves a single webhook subscription.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#show-2020-07
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     */
    pub async fn deprecated_202007_get_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/webhooks/{}/json?{}",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Update a webhook subscription's topic or address URIs.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#update-2020-07
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Delete a webhook subscription.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_webhooks_param_webhook(
        &self,
        webhook_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of webhooks. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#index-2020-10
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `created_at_max: &str` -- Retrieve webhook subscriptions that were created before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Retrieve webhook subscriptions that were created after a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     * * `limit: &str` -- Maximum number of webhook subscriptions that should be returned. Setting this parameter outside the maximum range will return an error.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict the returned list to webhook subscriptions whose id is greater than the specified since_id.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     * * `updated_at_min: &str` -- Retrieve webhooks that were updated before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Retrieve webhooks that were updated after a given date and time (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn get_webhook(
        &self,
        address: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        topic: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/webhooks.json?{}", query_),
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
     * Create a new webhook subscription by specifying both an address and a topic.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#create-2020-10
     *
     * **Parameters:**
     *
     * * `format: &str` -- Use this parameter to select the data format for the payload. Valid values are json and xml.
     *                     (default: json).
     */
    pub async fn create_webhooks(
        &self,
        format: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !format.is_empty() {
            query_args.push(("format".to_string(), format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/webhooks.json?{}", query_),
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
     * Retrieves a count of existing webhook subscriptions.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/webhooks/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#count-2020-10
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     */
    pub async fn get_webhooks_count(&self, address: &str, topic: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/webhooks/count.json?{}", query_),
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
     * Retrieves a single webhook subscription.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#show-2020-10
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     */
    pub async fn get_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/webhooks/{}/json?{}",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Update a webhook subscription's topic or address URIs.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#update-2020-10
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Delete a webhook subscription.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_webhooks_param_webhook(&self, webhook_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of webhooks. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#index-2021-01
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `created_at_max: &str` -- Retrieve webhook subscriptions that were created before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Retrieve webhook subscriptions that were created after a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     * * `limit: &str` -- Maximum number of webhook subscriptions that should be returned. Setting this parameter outside the maximum range will return an error.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict the returned list to webhook subscriptions whose id is greater than the specified since_id.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     * * `updated_at_min: &str` -- Retrieve webhooks that were updated before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Retrieve webhooks that were updated after a given date and time (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202101_get_webhook(
        &self,
        address: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        topic: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/webhooks.json?{}", query_),
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
     * Create a new webhook subscription by specifying both an address and a topic.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#create-2021-01
     *
     * **Parameters:**
     *
     * * `format: &str` -- Use this parameter to select the data format for the payload. Valid values are json and xml.
     *                     (default: json).
     */
    pub async fn deprecated_202101_create_webhooks(
        &self,
        format: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !format.is_empty() {
            query_args.push(("format".to_string(), format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/webhooks.json?{}", query_),
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
     * Retrieves a count of existing webhook subscriptions.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/webhooks/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#count-2021-01
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     */
    pub async fn deprecated_202101_get_webhooks_count(
        &self,
        address: &str,
        topic: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/webhooks/count.json?{}", query_),
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
     * Retrieves a single webhook subscription.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#show-2021-01
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     */
    pub async fn deprecated_202101_get_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/webhooks/{}/json?{}",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Update a webhook subscription's topic or address URIs.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#update-2021-01
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Delete a webhook subscription.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_webhooks_param_webhook(
        &self,
        webhook_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of webhooks. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#index-unstable
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `created_at_max: &str` -- Retrieve webhook subscriptions that were created before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Retrieve webhook subscriptions that were created after a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     * * `limit: &str` -- Maximum number of webhook subscriptions that should be returned. Setting this parameter outside the maximum range will return an error.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict the returned list to webhook subscriptions whose id is greater than the specified since_id.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     * * `updated_at_min: &str` -- Retrieve webhooks that were updated before a given date and time (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Retrieve webhooks that were updated after a given date and time (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_unstable_get_webhook(
        &self,
        address: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        topic: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/webhooks.json?{}", query_),
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
     * Create a new webhook subscription by specifying both an address and a topic.
     *
     * This function performs a `POST` to the `/admin/api/unstable/webhooks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#create-unstable
     *
     * **Parameters:**
     *
     * * `format: &str` -- Use this parameter to select the data format for the payload. Valid values are json and xml.
     *                     (default: json).
     */
    pub async fn deprecated_unstable_create_webhooks(
        &self,
        format: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !format.is_empty() {
            query_args.push(("format".to_string(), format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/webhooks.json?{}", query_),
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
     * Retrieves a count of existing webhook subscriptions.
     *
     * This function performs a `GET` to the `/admin/api/unstable/webhooks/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#count-unstable
     *
     * **Parameters:**
     *
     * * `address: &str` -- Retrieve webhook subscriptions that send the POST request to this URI.
     * * `topic: &str` -- Show webhook subscriptions with a given topic.
     *   For a list of valid values, refer to the topic property.>.
     */
    pub async fn deprecated_unstable_get_webhooks_count(
        &self,
        address: &str,
        topic: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !address.is_empty() {
            query_args.push(("address".to_string(), address.to_string()));
        }
        if !topic.is_empty() {
            query_args.push(("topic".to_string(), topic.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/webhooks/count.json?{}", query_),
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
     * Retrieves a single webhook subscription.
     *
     * This function performs a `GET` to the `/admin/api/unstable/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#show-unstable
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of the properties you want returned for each item in the result list. Use this parameter to restrict the returned list of items to only those properties you specify.
     */
    pub async fn deprecated_unstable_get_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/webhooks/{}/json?{}",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Update a webhook subscription's topic or address URIs.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#update-unstable
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_webhooks_param_webhook(
        &self,
        webhook_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
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
     * Delete a webhook subscription.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/webhooks/{webhook_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/events/webhook#destroy-unstable
     *
     * **Parameters:**
     *
     * * `webhook_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_webhooks_param_webhook(
        &self,
        webhook_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/webhooks/{}/json",
                crate::progenitor_support::encode_path(webhook_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}

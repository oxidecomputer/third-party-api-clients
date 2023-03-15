use crate::Client;
use crate::ClientResult;

pub struct Analytics {
    pub client: Client,
}

impl Analytics {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Analytics { client }
    }

    /**
     * Retrieves a list of reports. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#index-2020-01
     *
     * **Parameters:**
     *
     * * `ids: &str` -- A comma-separated list of report IDs.
     * * `limit: &str` -- The amount of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_min: &str` -- Show reports last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show reports last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_report(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-01/reports.json?{}", query_), None);
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
     * Creates a new report.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#create-2020-01
     *
     * **Parameters:**
     *
     * * `name: &str` -- The name of the report. Maximum length: 255 characters.
     * * `shopify_ql: &str` -- The ShopifyQL the report will query.
     */
    pub async fn deprecated_202001_create_reports(
        &self,
        name: &str,
        shopify_ql: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        if !shopify_ql.is_empty() {
            query_args.push(("shopify_ql".to_string(), shopify_ql.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-01/reports.json?{}", query_), None);
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
     * Retrieves a single report created by your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#show-2020-01
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_reports_param_report(
        &self,
        report_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/reports/{}/json?{}",
                crate::progenitor_support::encode_path(report_id),
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
     * Updates a report.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#update-2020-01
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_reports_param_report(
        &self,
        report_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Deletes a report.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_reports_param_report(
        &self,
        report_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Retrieves a list of reports. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#index-2020-04
     *
     * **Parameters:**
     *
     * * `ids: &str` -- A comma-separated list of report IDs.
     * * `limit: &str` -- The amount of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_min: &str` -- Show reports last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show reports last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_report(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-04/reports.json?{}", query_), None);
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
     * Creates a new report.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#create-2020-04
     *
     * **Parameters:**
     *
     * * `name: &str` -- The name of the report. Maximum length: 255 characters.
     * * `shopify_ql: &str` -- The ShopifyQL the report will query.
     */
    pub async fn deprecated_202004_create_reports(
        &self,
        name: &str,
        shopify_ql: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        if !shopify_ql.is_empty() {
            query_args.push(("shopify_ql".to_string(), shopify_ql.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-04/reports.json?{}", query_), None);
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
     * Retrieves a single report created by your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#show-2020-04
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_reports_param_report(
        &self,
        report_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/reports/{}/json?{}",
                crate::progenitor_support::encode_path(report_id),
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
     * Updates a report.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#update-2020-04
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_reports_param_report(
        &self,
        report_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Deletes a report.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_reports_param_report(
        &self,
        report_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Retrieves a list of reports. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#index-2020-07
     *
     * **Parameters:**
     *
     * * `ids: &str` -- A comma-separated list of report IDs.
     * * `limit: &str` -- The amount of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_min: &str` -- Show reports last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show reports last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_report(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-07/reports.json?{}", query_), None);
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
     * Creates a new report.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#create-2020-07
     *
     * **Parameters:**
     *
     * * `name: &str` -- The name of the report. Maximum length: 255 characters.
     * * `shopify_ql: &str` -- The ShopifyQL the report will query.
     */
    pub async fn deprecated_202007_create_reports(
        &self,
        name: &str,
        shopify_ql: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        if !shopify_ql.is_empty() {
            query_args.push(("shopify_ql".to_string(), shopify_ql.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-07/reports.json?{}", query_), None);
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
     * Retrieves a single report created by your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#show-2020-07
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_reports_param_report(
        &self,
        report_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/reports/{}/json?{}",
                crate::progenitor_support::encode_path(report_id),
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
     * Updates a report.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#update-2020-07
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_reports_param_report(
        &self,
        report_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Deletes a report.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_reports_param_report(
        &self,
        report_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Retrieves a list of reports. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#index-2020-10
     *
     * **Parameters:**
     *
     * * `ids: &str` -- A comma-separated list of report IDs.
     * * `limit: &str` -- The amount of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_min: &str` -- Show reports last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show reports last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_report(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-10/reports.json?{}", query_), None);
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
     * Creates a new report.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#create-2020-10
     *
     * **Parameters:**
     *
     * * `name: &str` -- The name of the report. Maximum length: 255 characters.
     * * `shopify_ql: &str` -- The ShopifyQL the report will query.
     */
    pub async fn create_reports(
        &self,
        name: &str,
        shopify_ql: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        if !shopify_ql.is_empty() {
            query_args.push(("shopify_ql".to_string(), shopify_ql.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-10/reports.json?{}", query_), None);
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
     * Retrieves a single report created by your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#show-2020-10
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_reports_param_report(
        &self,
        report_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/reports/{}/json?{}",
                crate::progenitor_support::encode_path(report_id),
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
     * Updates a report.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#update-2020-10
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_reports_param_report(
        &self,
        report_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Deletes a report.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_reports_param_report(&self, report_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Retrieves a list of reports. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#index-2021-01
     *
     * **Parameters:**
     *
     * * `ids: &str` -- A comma-separated list of report IDs.
     * * `limit: &str` -- The amount of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_min: &str` -- Show reports last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show reports last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_report(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2021-01/reports.json?{}", query_), None);
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
     * Creates a new report.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#create-2021-01
     *
     * **Parameters:**
     *
     * * `name: &str` -- The name of the report. Maximum length: 255 characters.
     * * `shopify_ql: &str` -- The ShopifyQL the report will query.
     */
    pub async fn deprecated_202101_create_reports(
        &self,
        name: &str,
        shopify_ql: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        if !shopify_ql.is_empty() {
            query_args.push(("shopify_ql".to_string(), shopify_ql.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2021-01/reports.json?{}", query_), None);
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
     * Retrieves a single report created by your app.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#show-2021-01
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_reports_param_report(
        &self,
        report_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/reports/{}/json?{}",
                crate::progenitor_support::encode_path(report_id),
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
     * Updates a report.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#update-2021-01
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_reports_param_report(
        &self,
        report_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Deletes a report.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_reports_param_report(
        &self,
        report_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Retrieves a list of reports. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#index-unstable
     *
     * **Parameters:**
     *
     * * `ids: &str` -- A comma-separated list of report IDs.
     * * `limit: &str` -- The amount of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_min: &str` -- Show reports last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show reports last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_report(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/reports.json?{}", query_),
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
     * Creates a new report.
     *
     * This function performs a `POST` to the `/admin/api/unstable/reports.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#create-unstable
     *
     * **Parameters:**
     *
     * * `name: &str` -- The name of the report. Maximum length: 255 characters.
     * * `shopify_ql: &str` -- The ShopifyQL the report will query.
     */
    pub async fn deprecated_unstable_create_reports(
        &self,
        name: &str,
        shopify_ql: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        if !shopify_ql.is_empty() {
            query_args.push(("shopify_ql".to_string(), shopify_ql.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/reports.json?{}", query_),
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
     * Retrieves a single report created by your app.
     *
     * This function performs a `GET` to the `/admin/api/unstable/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#show-unstable
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_reports_param_report(
        &self,
        report_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/reports/{}/json?{}",
                crate::progenitor_support::encode_path(report_id),
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
     * Updates a report.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#update-unstable
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_reports_param_report(
        &self,
        report_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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
     * Deletes a report.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/reports/{report_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/analytics/report#destroy-unstable
     *
     * **Parameters:**
     *
     * * `report_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_reports_param_report(
        &self,
        report_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/reports/{}/json",
                crate::progenitor_support::encode_path(report_id),
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

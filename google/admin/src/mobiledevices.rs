use crate::Client;
use crate::ClientResult;

pub struct Mobiledevices {
    pub client: Client,
}

impl Mobiledevices {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Mobiledevices { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/devices/mobile` endpoint.
     *
     * Retrieves a paginated list of all mobile devices for an account.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `max_results: i64` -- Maximum number of results to return. Max allowed value is 100.
     * * `order_by: crate::types::DirectoryMobiledevicesListOrderBy` -- Device property to use for sorting results.
     * * `page_token: &str` -- Token to specify next page in the list.
     * * `projection: crate::types::Projection` -- Restrict information returned to a set of selected fields.
     * * `query: &str` -- Search string in the format given at https://developers.google.com/admin-sdk/directory/v1/search-operators.
     * * `sort_order: crate::types::SortOrder` -- Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
     */
    pub async fn list(
        &self,
        customer_id: &str,
        max_results: i64,
        order_by: crate::types::DirectoryMobiledevicesListOrderBy,
        page_token: &str,
        projection: crate::types::Projection,
        query: &str,
        sort_order: crate::types::SortOrder,
    ) -> ClientResult<Vec<crate::types::MobileDevice>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        if !sort_order.to_string().is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/mobile?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        let resp: crate::types::MobileDevices = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.mobiledevices.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/devices/mobile` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Retrieves a paginated list of all mobile devices for an account.
     */
    pub async fn list_all(
        &self,
        customer_id: &str,
        order_by: crate::types::DirectoryMobiledevicesListOrderBy,
        projection: crate::types::Projection,
        query: &str,
        sort_order: crate::types::SortOrder,
    ) -> ClientResult<Vec<crate::types::MobileDevice>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        if !sort_order.to_string().is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/mobile?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        let mut resp: crate::types::MobileDevices = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut mobiledevices = resp.mobiledevices;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                resp = self
                    .client
                    .get(
                        &format!("{}&pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            mobiledevices.append(&mut resp.mobiledevices);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(mobiledevices)
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/devices/mobile/{resourceId}` endpoint.
     *
     * Retrieves a mobile device's properties.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `resource_id: &str` -- The unique ID the API service uses to identify the mobile device.
     * * `projection: crate::types::Projection` -- Restrict information returned to a set of selected fields.
     */
    pub async fn get(
        &self,
        customer_id: &str,
        resource_id: &str,
        projection: crate::types::Projection,
    ) -> ClientResult<crate::types::MobileDevice> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/mobile/{}?{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(resource_id),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customerId}/devices/mobile/{resourceId}` endpoint.
     *
     * Removes a mobile device.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `resource_id: &str` -- The unique ID the API service uses to identify the mobile device.
     */
    pub async fn delete(&self, customer_id: &str, resource_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/mobile/{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(resource_id),
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
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customerId}/devices/mobile/{resourceId}/action` endpoint.
     *
     * Takes an action that affects a mobile device. For example, remotely wiping a device.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `resource_id: &str` -- The unique ID the API service uses to identify the mobile device.
     */
    pub async fn action(
        &self,
        customer_id: &str,
        resource_id: &str,
        body: &crate::types::MobileDeviceAction,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/mobile/{}/action",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(resource_id),
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

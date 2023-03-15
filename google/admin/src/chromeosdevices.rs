use crate::Client;
use crate::ClientResult;

pub struct Chromeosdevices {
    pub client: Client,
}

impl Chromeosdevices {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Chromeosdevices { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/devices/chromeos` endpoint.
     *
     * Retrieves a paginated list of Chrome OS devices within an account.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `max_results: i64` -- Maximum number of results to return.
     * * `order_by: crate::types::OrderBy` -- Device property to use for sorting results.
     * * `org_unit_path: &str` -- The full path of the organizational unit or its unique ID.
     * * `page_token: &str` -- The `pageToken` query parameter is used to request the next page of query results. The follow-on request's `pageToken` query parameter is the `nextPageToken` from your previous response.
     * * `projection: crate::types::Projection` -- Restrict information returned to a set of selected fields.
     * * `query: &str` -- Search string in the format given at http://support.google.com/chromeos/a/bin/answer.py?answer=1698333.
     * * `sort_order: crate::types::SortOrder` -- Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
     */
    pub async fn list(
        &self,
        customer_id: &str,
        max_results: i64,
        order_by: crate::types::OrderBy,
        org_unit_path: &str,
        page_token: &str,
        projection: crate::types::Projection,
        query: &str,
        sort_order: crate::types::SortOrder,
    ) -> ClientResult<Vec<crate::types::ChromeOsDevice>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !org_unit_path.is_empty() {
            query_args.push(("orgUnitPath".to_string(), org_unit_path.to_string()));
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
                "/admin/directory/v1/customer/{}/devices/chromeos?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        let resp: crate::types::ChromeOsDevices = self
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
        Ok(resp.chromeosdevices.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/devices/chromeos` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Retrieves a paginated list of Chrome OS devices within an account.
     */
    pub async fn list_all(
        &self,
        customer_id: &str,
        order_by: crate::types::OrderBy,
        org_unit_path: &str,
        projection: crate::types::Projection,
        query: &str,
        sort_order: crate::types::SortOrder,
    ) -> ClientResult<Vec<crate::types::ChromeOsDevice>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !org_unit_path.is_empty() {
            query_args.push(("orgUnitPath".to_string(), org_unit_path.to_string()));
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
                "/admin/directory/v1/customer/{}/devices/chromeos?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        let mut resp: crate::types::ChromeOsDevices = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut chromeosdevices = resp.chromeosdevices;
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

            chromeosdevices.append(&mut resp.chromeosdevices);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(chromeosdevices)
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customerId}/devices/chromeos/moveDevicesToOu` endpoint.
     *
     * Moves or inserts multiple Chrome OS devices to an organizational unit. You can move up to 50 devices at once.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     * * `org_unit_path: &str` -- Full path of the target organizational unit or its ID.
     */
    pub async fn move_devices_ou(
        &self,
        customer_id: &str,
        org_unit_path: &str,
        body: &crate::types::ChromeOsMoveDevicesOu,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !org_unit_path.is_empty() {
            query_args.push(("orgUnitPath".to_string(), org_unit_path.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/chromeos/moveDevicesToOu?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
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
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/devices/chromeos/{deviceId}` endpoint.
     *
     * Retrieves a Chrome OS device's properties.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `device_id: &str` -- The unique ID of the device. The `deviceId`s are returned in the response from the [chromeosdevices.list](/admin-sdk/directory/v1/reference/chromeosdevices/list) method.
     * * `projection: crate::types::Projection` -- Determines whether the response contains the full list of properties or only a subset.
     */
    pub async fn get(
        &self,
        customer_id: &str,
        device_id: &str,
        projection: crate::types::Projection,
    ) -> ClientResult<crate::types::ChromeOsDevice> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/chromeos/{}?{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(device_id),
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
     * This function performs a `PUT` to the `/admin/directory/v1/customer/{customerId}/devices/chromeos/{deviceId}` endpoint.
     *
     * Updates a device's updatable properties, such as `annotatedUser`, `annotatedLocation`, `notes`, `orgUnitPath`, or `annotatedAssetId`.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `device_id: &str` -- The unique ID of the device. The `deviceId`s are returned in the response from the [chromeosdevices.list](/admin-sdk/v1/reference/chromeosdevices/list) method.
     * * `projection: crate::types::Projection` -- Restrict information returned to a set of selected fields.
     */
    pub async fn update(
        &self,
        customer_id: &str,
        device_id: &str,
        projection: crate::types::Projection,
        body: &crate::types::ChromeOsDevice,
    ) -> ClientResult<crate::types::ChromeOsDevice> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/chromeos/{}?{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(device_id),
                query_
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
     * This function performs a `PATCH` to the `/admin/directory/v1/customer/{customerId}/devices/chromeos/{deviceId}` endpoint.
     *
     * Updates a device's updatable properties, such as `annotatedUser`, `annotatedLocation`, `notes`, `orgUnitPath`, or `annotatedAssetId`. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `device_id: &str` -- The unique ID of the device. The `deviceId`s are returned in the response from the [chromeosdevices.list](/admin-sdk/v1/reference/chromeosdevices/list) method.
     * * `projection: crate::types::Projection` -- Restrict information returned to a set of selected fields.
     */
    pub async fn patch(
        &self,
        customer_id: &str,
        device_id: &str,
        projection: crate::types::Projection,
        body: &crate::types::ChromeOsDevice,
    ) -> ClientResult<crate::types::ChromeOsDevice> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/chromeos/{}?{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(device_id),
                query_
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customerId}/devices/chromeos/{resourceId}/action` endpoint.
     *
     * Takes an action that affects a Chrome OS Device. This includes deprovisioning, disabling, and re-enabling devices. *Warning:* * Deprovisioning a device will stop device policy syncing and remove device-level printers. After a device is deprovisioned, it must be wiped before it can be re-enrolled. * Lost or stolen devices should use the disable action. * Re-enabling a disabled device will consume a device license. If you do not have sufficient licenses available when completing the re-enable action, you will receive an error. For more information about deprovisioning and disabling devices, visit the [help center](https://support.google.com/chrome/a/answer/3523633).
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
     * * `resource_id: &str` -- The unique ID of the device. The `resourceId`s are returned in the response from the [chromeosdevices.list](/admin-sdk/directory/v1/reference/chromeosdevices/list) method.
     */
    pub async fn action(
        &self,
        customer_id: &str,
        resource_id: &str,
        body: &crate::types::ChromeOsDeviceAction,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/chromeos/{}/action",
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

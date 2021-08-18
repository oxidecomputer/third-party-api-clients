use anyhow::Result;

use crate::Client;

pub struct PhoneDevices {
    pub client: Client,
}

impl PhoneDevices {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PhoneDevices { client }
    }

    /**
     * List devices.
     *
     * This function performs a `GET` to the `/phone/devices` endpoint.
     *
     * List all the [desk phone devices](https://support.zoom.us/hc/en-us/articles/360021119092) that are configured with Zoom Phone on an account.
     *
     * **Scopes:** `phone:read:admin`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**<br>
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `type_: crate::types::ListPhoneDevicesType` -- State of the device. The value should be either `assigned` to list devices that have been assigned to user(s) or `unassigned` to list devices that have not yet been assigned to any user in the Zoom account.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_size: i64` -- The number of records returned within a single API call.
     */
    pub async fn list(
        &self,
        type_: crate::types::ListPhoneDevicesType,
        next_page_token: &str,
        page_size: i64,
    ) -> Result<Vec<crate::types::ListPhoneDevicesResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/devices?{}", query_);

        let resp: crate::types::ListPhoneDevicesResponseData = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.devices)
    }

    /**
     * List devices.
     *
     * This function performs a `GET` to the `/phone/devices` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * List all the [desk phone devices](https://support.zoom.us/hc/en-us/articles/360021119092) that are configured with Zoom Phone on an account.
     *
     * **Scopes:** `phone:read:admin`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**<br>
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     */
    pub async fn list_all(
        &self,
        type_: crate::types::ListPhoneDevicesType,
    ) -> Result<Vec<crate::types::ListPhoneDevicesResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/devices?{}", query_);

        let mut resp: crate::types::ListPhoneDevicesResponseData =
            self.client.get(&url, None).await?;

        let mut devices = resp.devices;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?next_page_token={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&next_page_token={}", url, page), None)
                    .await?;
            }

            devices.append(&mut resp.devices);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(devices)
    }

    /**
     * Add a device.
     *
     * This function performs a `POST` to the `/phone/devices` endpoint.
     *
     * By default, all Zoom Phone users can make and receive calls using the Zoom desktop and mobile applications. Additionally, if a desk phone is required, use this API to [add a desk phone and assign it](https://support.zoom.us/hc/en-us/articles/360021119092#h_5ca07504-68a8-4c3d-ad0e-c1d3594436da) to a user.
     *
     * **Prerequisites:**<br>
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     * * [Supported device](https://support.zoom.us/hc/en-us/articles/360001299063-Zoom-Voice-Supported-Devices)<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn add(&self, body: &crate::types::AddPhoneDeviceRequest) -> Result<()> {
        let url = "/phone/devices".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get device details.
     *
     * This function performs a `GET` to the `/phone/devices/{deviceId}` endpoint.
     *
     * Get detailed information about a specific [desk phone device](https://support.zoom.us/hc/en-us/articles/360021119092).
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `device_id: &str` -- Unique Identifier of the device.
     */
    pub async fn get_device(&self, device_id: &str) -> Result<crate::types::GetDeviceResponse> {
        let url = format!(
            "/phone/devices/{}",
            crate::progenitor_support::encode_path(&device_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a device.
     *
     * This function performs a `DELETE` to the `/phone/devices/{deviceId}` endpoint.
     *
     * Remove a [desk phone device](https://support.zoom.us/hc/en-us/articles/360021119092) from the Zoom Phone System Management.<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     * * Device must not have been assigned to a user.<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `device_id: &str` -- Unique Identifier of the device.
     */
    pub async fn delete_device(&self, device_id: &str) -> Result<()> {
        let url = format!(
            "/phone/devices/{}",
            crate::progenitor_support::encode_path(&device_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a device.
     *
     * This function performs a `PATCH` to the `/phone/devices/{deviceId}` endpoint.
     *
     * Update information of a [desk phone device](https://support.zoom.us/hc/en-us/articles/360021119092).<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `device_id: &str` -- Unique Identifier of the Device.
     */
    pub async fn update_device(
        &self,
        device_id: &str,
        body: &crate::types::UpdateDeviceRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/devices/{}",
            crate::progenitor_support::encode_path(&device_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

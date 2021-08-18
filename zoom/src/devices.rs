use anyhow::Result;

use crate::Client;

pub struct Devices {
    pub client: Client,
}

impl Devices {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Devices { client }
    }

    /**
     * List H.323/SIP devices.
     *
     * This function performs a `GET` to the `/h323/devices` endpoint.
     *
     * A H.323 or SIP device can make a video call to a [Room Connector](https://support.zoom.us/hc/en-us/articles/201363273-Getting-Started-With-H-323-SIP-Room-Connector) to join a Zoom cloud meeting. A Room Connector can also call out to a H.323 or SIP device to join a Zoom cloud meeting. Use this API to list all H.323/SIP Devices on a Zoom account.<br><br>
     * **Scopes:** `h323:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list(
        &self,
        page_size: i64,
        page_number: i64,
        next_page_token: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_number > 0 {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/h323/devices?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Create a H.323/SIP device.
     *
     * This function performs a `POST` to the `/h323/devices` endpoint.
     *
     * A H.323 or SIP device can make a video call to a [Room Connector](https://support.zoom.us/hc/en-us/articles/201363273-Getting-Started-With-H-323-SIP-Room-Connector) to join a Zoom cloud meeting. A Room Connector can also call out to a H.323 or SIP device to join a Zoom cloud meeting. Use this API to add a H.323/SIP device to your Zoom account<br><br>
     * **Scopes:** `h323:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light` <br>
     */
    pub async fn create(&self, body: &crate::types::Device) -> Result<()> {
        let url = "/h323/devices".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a H.323/SIP device.
     *
     * This function performs a `DELETE` to the `/h323/devices/{deviceId}` endpoint.
     *
     * A H.323 or SIP device can make a video call to a [Room Connector](https://support.zoom.us/hc/en-us/articles/201363273-Getting-Started-With-H-323-SIP-Room-Connector) to join a Zoom cloud meeting. A Room Connector can also call out to a H.323 or SIP device to join a Zoom cloud meeting. Use this API to delete a H.323/SIP device from your Zoom account.<br><br>
     * **Scopes:** `h323:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     * **Parameters:**
     *
     * * `device_id: &str` -- User's first name.
     */
    pub async fn delete(&self, device_id: &str) -> Result<()> {
        let url = format!(
            "/h323/devices/{}",
            crate::progenitor_support::encode_path(&device_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a H.323/SIP device.
     *
     * This function performs a `PATCH` to the `/h323/devices/{deviceId}` endpoint.
     *
     * A H.323 or SIP device can make a video call to a [Room Connector](https://support.zoom.us/hc/en-us/articles/201363273-Getting-Started-With-H-323-SIP-Room-Connector) to join a Zoom cloud meeting. A Room Connector can also call out to a H.323 or SIP device to join a Zoom cloud meeting. Use this API to edit information of a H.323/SIP device from your Zoom account.<br><br>
     * **Scopes:** `h323:write:admin`<br>
     *  <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `device_id: &str` -- User's first name.
     */
    pub async fn update(&self, device_id: &str, body: &crate::types::Device) -> Result<()> {
        let url = format!(
            "/h323/devices/{}",
            crate::progenitor_support::encode_path(&device_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

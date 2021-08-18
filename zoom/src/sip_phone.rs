use anyhow::Result;

use crate::Client;

pub struct SipPhone {
    pub client: Client,
}

impl SipPhone {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SipPhone { client }
    }

    /**
     * List SIP phones.
     *
     * This function performs a `GET` to the `/sip_phones` endpoint.
     *
     * Zoom’s Phone System Integration (PSI), also referred as SIP phones, enables an organization to leverage the Zoom client to complete a softphone registration to supported premise based PBX system. End users will have the ability to have softphone functionality within a single client while maintaining a comparable interface to Zoom Phone. Use this API to list SIP phones on an account.<br><br>
     * **Prerequisites**:
     * * Currently only supported on Cisco and Avaya PBX systems.
     * * User must enable SIP Phone Integration by contacting the [Sales](https://zoom.us/contactsales) team.<br> **Scope:** `sip_phone:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     * **Parameters:**
     *
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     * * `search_key: &str` -- User name or email address of a user. If this parameter is provided, only the SIP phone system integration enabled for that specific user will be returned. Otherwise, all SIP phones on an account will be returned.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list(
        &self,
        page_number: i64,
        search_key: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::Phones>> {
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
        if !search_key.is_empty() {
            query_args.push(("search_key".to_string(), search_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/sip_phones?{}", query_);

        let resp: crate::types::ListSipPhonesResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.phones)
    }

    /**
     * List SIP phones.
     *
     * This function performs a `GET` to the `/sip_phones` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Zoom’s Phone System Integration (PSI), also referred as SIP phones, enables an organization to leverage the Zoom client to complete a softphone registration to supported premise based PBX system. End users will have the ability to have softphone functionality within a single client while maintaining a comparable interface to Zoom Phone. Use this API to list SIP phones on an account.<br><br>
     * **Prerequisites**:
     * * Currently only supported on Cisco and Avaya PBX systems.
     * * User must enable SIP Phone Integration by contacting the [Sales](https://zoom.us/contactsales) team.<br> **Scope:** `sip_phone:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     */
    pub async fn list_all(&self, search_key: &str) -> Result<Vec<crate::types::Phones>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !search_key.is_empty() {
            query_args.push(("search_key".to_string(), search_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/sip_phones?{}", query_);

        let mut resp: crate::types::ListSipPhonesResponse = self.client.get(&url, None).await?;

        let mut phones = resp.phones;
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

            phones.append(&mut resp.phones);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(phones)
    }

    /**
     * Enable SIP phone.
     *
     * This function performs a `POST` to the `/sip_phones` endpoint.
     *
     * Zoom’s Phone System Integration (PSI), also referred as SIP phones, enables an organization to leverage the Zoom client to complete a softphone registration to supported premise based PBX system. End users will have the ability to have softphone functionality within a single client while maintaining a comparable interface to Zoom Phone. Use this API to enable a user to use SIP phone.<br><br>
     * **Prerequisites**:
     * * Currently only supported on Cisco and Avaya PBX systems.
     * * The account owner or account admin must first enable SIP Phone Integration by contacting the [Sales](https://zoom.us/contactsales) team.<br> **Scope:** `sip_phone:write:admin`
     * <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     */
    pub async fn create(&self, body: &crate::types::CreateSipPhoneRequest) -> Result<()> {
        let url = "/sip_phones".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete SIP phone.
     *
     * This function performs a `DELETE` to the `/sip_phones/{phoneId}` endpoint.
     *
     * Zoom’s Phone System Integration (PSI), also referred as SIP phones, enables an organization to leverage the Zoom client to complete a softphone registration to supported premise based PBX system. End users will have the ability to have softphone functionality within a single client while maintaining a comparable interface to Zoom Phone. Use this API to delete a specific SIP phone on a Zoom account.<br><br>
     * **Prerequisites**:
     * * Currently only supported on Cisco and Avaya PBX systems.
     * * User must enable SIP Phone Integration by contacting the [Sales](https://zoom.us/contactsales) team.<br> **Scope:** `sip_phone:read:admin`
     * <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `phone_id: &str` -- Unique Identifier of the SIP Phone. It can be retrieved from the List SIP Phones API.
     */
    pub async fn delete(&self, phone_id: &str) -> Result<()> {
        let url = format!(
            "/sip_phones/{}",
            crate::progenitor_support::encode_path(&phone_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update SIP phone.
     *
     * This function performs a `PATCH` to the `/sip_phones/{phoneId}` endpoint.
     *
     * Zoom’s Phone System Integration (PSI), also referred as SIP phones, enables an organization to leverage the Zoom client to complete a softphone registration to supported premise based PBX system. End users will have the ability to have softphone functionality within a single client while maintaining a comparable interface to Zoom Phone. Use this API to update information of a specific SIP Phone on a Zoom account.<br><br>
     * **Prerequisites**:
     * * Currently only supported on Cisco and Avaya PBX systems.
     * * The account owner or account admin must first enable SIP Phone Integration by contacting the [Sales](https://zoom.us/contactsales) team.<br> **Scope:** `sip_phone:write:admin`
     * <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `phone_id: &str` -- Unique Identifier of the SIP Phone. This can be retrieved from the List SIP Phones API.
     */
    pub async fn update(
        &self,
        phone_id: &str,
        body: &crate::types::UpdateSipPhoneRequest,
    ) -> Result<()> {
        let url = format!(
            "/sip_phones/{}",
            crate::progenitor_support::encode_path(&phone_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

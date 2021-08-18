use anyhow::Result;

use crate::Client;

pub struct CommonAreaPhones {
    pub client: Client,
}

impl CommonAreaPhones {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CommonAreaPhones { client }
    }

    /**
     * List common area phones.
     *
     * This function performs a `GET` to the `/phone/common_area_phones` endpoint.
     *
     * Use this API to list all of an account's [common area phones](https://support.zoom.us/hc/en-us/articles/360028516231-Managing-Common-Area-Phones).
     *
     * A common area phone can be provisioned by a Zoom account owner or a Zoom admin so that anyone in an organization can use it. For example, if your office has shared desks that don't belong to a specific employees, you could add a common area phone so that any person can use it.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     *
     * * Pro or a higher account with Zoom Phone license.
     * * Account owner or admin permissions.
     * * A [supported device](https://support.zoom.us/hc/en-us/articles/360001299063-Zoom-Voice-Supported-Devices)
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The total number of records returned from a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list(
        &self,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::CommonAreaPhones>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/common_area_phones?{}", query_);

        let resp: crate::types::ListCommonAreaPhonesResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.common_area_phones)
    }

    /**
     * List common area phones.
     *
     * This function performs a `GET` to the `/phone/common_area_phones` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Use this API to list all of an account's [common area phones](https://support.zoom.us/hc/en-us/articles/360028516231-Managing-Common-Area-Phones).
     *
     * A common area phone can be provisioned by a Zoom account owner or a Zoom admin so that anyone in an organization can use it. For example, if your office has shared desks that don't belong to a specific employees, you could add a common area phone so that any person can use it.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     *
     * * Pro or a higher account with Zoom Phone license.
     * * Account owner or admin permissions.
     * * A [supported device](https://support.zoom.us/hc/en-us/articles/360001299063-Zoom-Voice-Supported-Devices)
     */
    pub async fn list_all(&self) -> Result<Vec<crate::types::CommonAreaPhones>> {
        let url = "/phone/common_area_phones".to_string();
        let mut resp: crate::types::ListCommonAreaPhonesResponse =
            self.client.get(&url, None).await?;

        let mut common_area_phones = resp.common_area_phones;
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

            common_area_phones.append(&mut resp.common_area_phones);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(common_area_phones)
    }

    /**
     * Add a common area phone.
     *
     * This function performs a `POST` to the `/phone/common_area_phones` endpoint.
     *
     * Use this API to [add a common area phone](https://support.zoom.us/hc/en-us/articles/360028516231-Managing-Common-Area-Phones#h_2d0da347-c35a-4993-9771-e21aaa568deb).
     *
     * A common area phone can be provisioned by a Zoom account owner or a Zoom admin so that anyone in an organization can use it. For example, if your office has shared desks that don't belong to a specific employees, you could add a common area phone so that any person can use it.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     *
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     * * A [supported device](https://support.zoom.us/hc/en-us/articles/360001299063-Zoom-Voice-Supported-Devices)
     */
    pub async fn add(
        &self,
        body: &crate::types::AddCommonAreaPhoneRequest,
    ) -> Result<crate::types::AddCommonAreaPhoneResponse> {
        let url = "/phone/common_area_phones".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get common area phone details.
     *
     * This function performs a `GET` to the `/phone/common_area_phones/{commonAreaPhoneId}` endpoint.
     *
     * Use this API to get details on a specific [common area phone](https://support.zoom.us/hc/en-us/articles/360028516231-Managing-Common-Area-Phones) in an account.
     *
     * A common area phone can be provisioned by a Zoom account owner or a Zoom admin so that anyone in an organization can use it. For example, if your office has shared desks that don't belong to a specific employees, you could add a common area phone so that any person can use it.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**<br>
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     * * A [supported device](https://support.zoom.us/hc/en-us/articles/360001299063-Zoom-Voice-Supported-Devices)
     *
     * **Parameters:**
     *
     * * `common_area_phone_id: &str` -- Unique Identifier of the Common Area Phone. Use the unique identifier or the Mac address of the common area phone. The Mac address can be in hyphenated (`00-04-f2-5e-ec-3c`) or not hyphenated (`0004f25eec3c`) format. You can get this value from the [List Common Area Phones API](https://marketplace.zoom.us/docs/api-reference/zoom-api/common-area-phones/listcommonareaphones).
     */
    pub async fn get(
        &self,
        common_area_phone_id: &str,
    ) -> Result<crate::types::GetCommonAreaPhoneResponse> {
        let url = format!(
            "/phone/common_area_phones/{}",
            crate::progenitor_support::encode_path(&common_area_phone_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a common area phone.
     *
     * This function performs a `DELETE` to the `/phone/common_area_phones/{commonAreaPhoneId}` endpoint.
     *
     * A common area phone can be provisioned by a Zoom account owner or a Zoom admin so that anyone in an organization can use it. For example, if your office has shared desks that don't belong to a specific employees, you could add a common area phone so that any person can use it.<br> Use this API to remove the [common area phone](https://support.zoom.us/hc/en-us/articles/360028516231-Managing-Common-Area-Phones) from Zoom Phone System in an account.<br><br>**Prerequisites:**<br>
     * * Pro or a higher account with Zoom Phone license.
     * * Account owner or admin permissions.
     * * [Supported device](https://support.zoom.us/hc/en-us/articles/360001299063-Zoom-Voice-Supported-Devices)<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `common_area_phone_id: &str` -- Unique Identifier of the common area phone.
     */
    pub async fn delete(&self, common_area_phone_id: &str) -> Result<()> {
        let url = format!(
            "/phone/common_area_phones/{}",
            crate::progenitor_support::encode_path(&common_area_phone_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update common area phone.
     *
     * This function performs a `PATCH` to the `/phone/common_area_phones/{commonAreaPhoneId}` endpoint.
     *
     * Use this API to update details on a specific [common area phone](https://support.zoom.us/hc/en-us/articles/360028516231-Managing-Common-Area-Phones) in an account.
     *
     * A common area phone can be provisioned by a Zoom account owner or a Zoom admin so that anyone in an organization can use it. For example, if your office has shared desks that don't belong to a specific employees, you could add a common area phone so that any person can use it.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     *
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     * * A [supported device](https://support.zoom.us/hc/en-us/articles/360001299063-Zoom-Voice-Supported-Devices)
     */
    pub async fn update(
        &self,
        common_area_phone_id: &str,
        body: &crate::types::UpdateCommonAreaPhoneRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/common_area_phones/{}",
            crate::progenitor_support::encode_path(&common_area_phone_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Assign phone numbers to common area phone.
     *
     * This function performs a `POST` to the `/phone/common_area_phones/{commonAreaPhoneId}/phone_numbers` endpoint.
     *
     * Assign phone numbers to common area phone.<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher account with Zoom Phone license.
     * * Account owner or admin permissions.
     * **Scope:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn assign_phone_numbers_common_area(
        &self,
        common_area_phone_id: &str,
        body: &crate::types::AssignPhoneNumbersCommonAreaRequest,
    ) -> Result<crate::types::AssignPhoneNumbersCommonAreaResponseData> {
        let url = format!(
            "/phone/common_area_phones/{}/phone_numbers",
            crate::progenitor_support::encode_path(&common_area_phone_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign phone numbers from a common area phone.
     *
     * This function performs a `DELETE` to the `/phone/common_area_phones/{commonAreaPhoneId}/phone_numbers/{phoneNumberId}` endpoint.
     *
     * Use this API to unassign a phone number from a common Area phone.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**<br>
     * * A Pro or a higher account with a Zoom Phone license
     * * An account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `common_area_phone_id: &str` -- The common area phone's unique ID.
     * * `phone_number_id: &str` -- The phone number or the phone number's unique ID.
     */
    pub async fn unassign_phone_numbers_from_common_area(
        &self,
        common_area_phone_id: &str,
        phone_number_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/phone/common_area_phones/{}/phone_numbers/{}",
            crate::progenitor_support::encode_path(&common_area_phone_id.to_string()),
            crate::progenitor_support::encode_path(&phone_number_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Assign calling plans to common area phone.
     *
     * This function performs a `POST` to the `/phone/common_area_phones/{commonAreaPhoneId}/calling_plans` endpoint.
     *
     * Assign calling plans to common area phone.<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher account with Zoom Phone license.
     * * Account owner or admin permissions.
     * **Scope:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn assign_calling_plans(
        &self,
        common_area_phone_id: &str,
        body: &crate::types::AssignCallingPlansCommonAreaPhoneRequestData,
    ) -> Result<crate::types::AssignCallingPlansCommonAreaPhoneResponseData> {
        let url = format!(
            "/phone/common_area_phones/{}/calling_plans",
            crate::progenitor_support::encode_path(&common_area_phone_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign calling plan from a common area phone.
     *
     * This function performs a `DELETE` to the `/phone/common_area_phones/{commonAreaPhoneId}/calling_plans/{type}` endpoint.
     *
     * Use this API to unassign a calling plan from a common area phone.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Pro or higher account with a Zoom Phone license
     * * An account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `common_area_phone_id: &str` -- The common area phone's unique ID.
     * * `type_: &str` -- The [calling plan](https://marketplace.zoom.us/docs/api-reference/other-references/plans#zoom-phone-calling-plans) to remove.
     */
    pub async fn unassign_calling_plans_from(
        &self,
        common_area_phone_id: &str,
        type_: &str,
    ) -> Result<()> {
        let url = format!(
            "/phone/common_area_phones/{}/calling_plans/{}",
            crate::progenitor_support::encode_path(&common_area_phone_id.to_string()),
            crate::progenitor_support::encode_path(&type_.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

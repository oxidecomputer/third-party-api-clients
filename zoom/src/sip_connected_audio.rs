use anyhow::Result;

use crate::Client;

pub struct SipConnectedAudio {
    pub client: Client,
}

impl SipConnectedAudio {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SipConnectedAudio { client }
    }

    /**
     * List SIP trunk numbers.
     *
     * This function performs a `GET` to the `/sip_trunk/numbers` endpoint.
     *
     * With SIP-connected audio, Zoom establishes a SIP trunk (a network connection specifically designed to make and deliver phone calls) over a direct and private connection between the customer’s network and the Zoom cloud. Meeting participants that dial into a meeting or have the meeting call them, and are On-Net from the perspective of the customers' IP telephony network, will be connected over this trunk rather than over the PSTN. <br><br>Use this API to list all the numbers that are configured for SIP Connected Audio in a Zoom Account.
     *
     * **Prerequisites:**<br>
     * * Pro or a higher account with SIP Connected Audio plan enabled.
     * * The account must be a master account<br>
     * **Scopes:** `sip_trunk:master`
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn list_sip_trunk_numbers(
        &self,
    ) -> Result<crate::types::ListSipTrunkNumbersResponse> {
        let url = "/sip_trunk/numbers".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Assign SIP trunk configuration.
     *
     * This function performs a `PATCH` to the `/accounts/{accountId}/sip_trunk/settings` endpoint.
     *
     * Use this API to copy the Session Initiation Protocol (SIP) Connected Audio configurations applied on the Master account and enable or disable those configurations on a subaccount.
     *
     * With SIP-connected audio, Zoom establishes a [SIP trunk](https://en.wikipedia.org/wiki/SIP_trunking) (a network connection specifically designed to make and deliver phone calls) over a direct and private connection between the customer’s network and the Zoom cloud. Meeting participants that dial into a meeting or have the meeting call them, and are On-Net from the perspective of the customers' IP telephony network, will be connected over this trunk rather than over the PSTN.
     *
     * **Scopes:** `sip_trunk:master`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with SIP Connected Audio plan enabled.
     * * A Master account owner
     */
    pub async fn assign_sip_config(
        &self,
        account_id: &str,
        body: &crate::types::AssignSipConfigRequest,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/sip_trunk/settings",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Assign numbers.
     *
     * This function performs a `POST` to the `/accounts/{accountId}/sip_trunk/numbers` endpoint.
     *
     * With SIP-connected audio, Zoom establishes a SIP trunk (a network connection specifically designed to make and deliver phone calls) over a direct and private connection between the customer’s network and the Zoom cloud. Meeting participants that dial into a meeting or have the meeting call them, and are On-Net from the perspective of the customers' IP telephony network, will be connected over this trunk rather than over the PSTN. <br><br>Use this API to assign internal numbers to a sub account.
     *
     * **Prerequisites:**<br>
     * * Pro or a higher account with SIP Connected Audio plan enabled.
     * * The account must be a master account<br>
     * **Scopes:** `sip_trunk:master`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique Identifier of the sub account.
     */
    pub async fn assign_sip_trunk_numbers(
        &self,
        account_id: &str,
        body: &crate::types::AssignSipTrunkNumbersRequest,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/sip_trunk/numbers",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete all numbers.
     *
     * This function performs a `DELETE` to the `/accounts/{accountId}/sip_trunk/numbers` endpoint.
     *
     * With SIP-connected audio, Zoom establishes a SIP trunk (a network connection specifically designed to make and deliver phone calls) over a direct and private connection between the customer’s network and the Zoom cloud. Meeting participants that dial into a meeting or have the meeting call them, and are On-Net from the perspective of the customers' IP telephony network, will be connected over this trunk rather than over the PSTN. <br><br>Use this API to delete all internal numbers assigned to a sub account.
     * **Prerequisites:**<br>
     *
     * * Pro or a higher account with SIP Connected Audio plan enabled.
     * * The account must be a master account<br>
     * **Scopes:** `sip_trunk:master`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Account ID of the sub account from which the numbers are to be deleted. This can be retrieved from [List sub accounts](https://marketplace.zoom.us/docs/api-reference/zoom-api/accounts/account) API.
     */
    pub async fn delete_all_sip_numbers(&self, account_id: &str) -> Result<()> {
        let url = format!(
            "/accounts/{}/sip_trunk/numbers",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List SIP trunks.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/sip_trunk/trunks` endpoint.
     *
     * With SIP-connected audio, Zoom establishes a SIP trunk (a network connection specifically designed to make and deliver phone calls) over a direct and private connection between the customer’s network and the Zoom cloud. Meeting participants that dial into a meeting or have the meeting call them, and are On-Net from the perspective of the customers’ IP telephony network, will be connected over this trunk rather than over the PSTN.<br><br>
     * Use this API to list all the SIP trunks assigned to a master account or a sub account of the master account. To retrieve SIP trunks assigned to a sub account, provide the account ID of the sub account in the `accountId` path parameter. To retrieve SIP trunks of a master account, provide `me` as the value of the `accountId` path parameter. <br><br> **Scope:** `sip_trunk:read:admin`
     * <br><b>Prerequisites:</b><br>
     * * The account must either be a master account or a sub account with [API Partner Plan](https://zoom.us/plan/api) and SIP Connected Audio Plan.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique Identifier of the Account. To retrieve SIP trunks assigned to a sub account, provide the account ID of the sub account in the as the value of this field. To retrieve SIP trunks of a master account, provide `me` as the value of this field.
     */
    pub async fn list_sip_trunk(
        &self,
        account_id: &str,
    ) -> Result<crate::types::ListSipTrunksResponseData> {
        let url = format!(
            "/accounts/{}/sip_trunk/trunks",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Assign SIP trunks.
     *
     * This function performs a `POST` to the `/accounts/{accountId}/sip_trunk/trunks` endpoint.
     *
     * With SIP-connected audio, Zoom establishes a SIP trunk (a network connection specifically designed to make and deliver phone calls) over a direct and private connection between the customer’s network and the Zoom cloud. Meeting participants that dial into a meeting or have the meeting call them, and are On-Net from the perspective of the customers’ IP telephony network, will be connected over this trunk rather than over the PSTN.<br><br>Use this API to assign SIP trunk(s) that are available on a master account to a sub account. <br><b>Prerequisites:</b><br>
     * * The account making this API request must be a master account with [API Partner Plan](https://zoom.us/plan/api) and SIP Connected Audio Plan.<br><br>
     * **Scope:** `sip_trunk:master`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique Identifier of the sub account.
     */
    pub async fn assign_sip_trunks(
        &self,
        account_id: &str,
        body: &crate::types::AssignSipTrunksRequestData,
    ) -> Result<crate::types::AssignSipTrunksResponseData> {
        let url = format!(
            "/accounts/{}/sip_trunk/trunks",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a SIP trunk.
     *
     * This function performs a `DELETE` to the `/accounts/{accountId}/sip_trunk/trunks/{trunkId}` endpoint.
     *
     * Use this API to remove existing SIP trunk of a sub account.<br>
     * <br><b>Prerequisites:</b><br>
     * * The account making this API request must be a master account with [API Partner Plan](https://zoom.us/plan/api) and SIP Connected Audio Plan.<br><br>
     * **Scope:** `sip_trunk:master`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique identifier of the sub account.
     * * `trunk_id: &str` -- Unique identifier of the SIP Trunk that was previously assigned to a sub account. To retrieve the value of this field, use the List SIP Trunks API.
     */
    pub async fn delete_sip_trunk(
        &self,
        account_id: &str,
        trunk_id: &str,
    ) -> Result<crate::types::Domains> {
        let url = format!(
            "/accounts/{}/sip_trunk/trunks/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&trunk_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List internal call-out countries.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/sip_trunk/callout_countries` endpoint.
     *
     * Retrieve the list of internal [call-out](https://support.zoom.us/hc/en-us/articles/200942859-How-To-Use-Telephone-Call-Out-) countries of a master account or a sub account. To list call-out enabled countries of a sub account, provide the account ID of the sub account in the `accountId` path parameter. To list call-out enabled countries of a master account, provide `me` as the value of the `accountId` path parameter.
     * <br><b>Prerequisites:</b><br>
     * * The account making this API request must be a [master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis) with SIP Connected Audio Plan.<br><br>
     * **Scope:** `sip_trunk:master`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique identifier of the account. To list Call-out enabled countries to a sub account, provide the account ID of the sub account in the `accountId` path parameter. To list Call-out enabled countries of a sub account, provide the account ID of the sub account in the `accountId` path parameter. To list Call-out enabled countries of a master account, provide `me` as the value of the `accountId` path parameter.
     */
    pub async fn list_internal_callout_countries(
        &self,
        account_id: &str,
    ) -> Result<crate::types::ListInternalCalloutCountriesResponse> {
        let url = format!(
            "/accounts/{}/sip_trunk/callout_countries",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Add internal call-out countries.
     *
     * This function performs a `POST` to the `/accounts/{accountId}/sip_trunk/callout_countries` endpoint.
     *
     * Specify the list of [call-out](https://support.zoom.us/hc/en-us/articles/200942859-How-To-Use-Telephone-Call-Out-) countries for a master account or a sub account. To add call-out enabled countries to a sub account, provide the account ID of the sub account in the `accountId` path parameter. To add call-out enabled countries to a master account, provide `me` as the value of the `accountId` path parameter.
     * <br><b>Prerequisites:</b><br>
     * * The account making this API request must be a [master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis) with SIP Connected Audio Plan.<br><br>
     * **Scope:** `sip_trunk:master`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique identifier of the account. To add Call-out enabled countries to a sub account, provide the account ID of the sub account in the `accountId` path parameter. To add Call-out enabled countries to a master account, provide `me` as the value of the `accountId` path parameter.
     */
    pub async fn add_callout_countries(
        &self,
        account_id: &str,
        body: &crate::types::AddCalloutCountriesRequestData,
    ) -> Result<crate::types::AddCalloutCountriesResponse> {
        let url = format!(
            "/accounts/{}/sip_trunk/callout_countries",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete internal call-out country.
     *
     * This function performs a `DELETE` to the `/accounts/{accountId}/sip_trunk/callout_countries/{countryId}` endpoint.
     *
     * Delete a previously assigned [call-out](https://support.zoom.us/hc/en-us/articles/200942859-How-To-Use-Telephone-Call-Out-) country from a master account or a sub account. To remove call-out country from a sub account, provide the account ID of the sub account in the `accountId` path parameter. To remove call-out country from a master account, provide `me` as the value of the `accountId` path parameter.
     * <br><b>Prerequisites:</b><br>
     * * The account making this API request must be a [master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis) with SIP Connected Audio Plan.<br><br>
     * **Scope:** `sip_trunk:master`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique Identifier of the Account.<br>
     *   To remove Call-out country from a sub account, provide the account ID of the sub account in the `accountId` path parameter. To remove Call-out country from a master account, provide `me` as the value of the `accountId` path parameter.
     * * `country_id: &str` -- Two lettered Id of the country.
     */
    pub async fn delete_internal_call_out_country(
        &self,
        account_id: &str,
        country_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/sip_trunk/callout_countries/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&country_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List internal numbers.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/sip_trunk/internal_numbers` endpoint.
     *
     * This API allows a master account with SIP Connected Audio plan to list internal phone numbers (i.e., numbers that are not provided by Zoom but are owned by the organization consuming the API) assigned to a master account or a sub account.<br><br>To list internal numbers of a sub account, provide the account ID of the sub account in the `accountId` path parameter. To list internal numbers of a  master account, provide `me` as the value of the `accountId` path parameter.
     * <br><b>Prerequisites:</b><br>
     * * The account making this API request must be a [master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis) with SIP Connected Audio Plan.<br><br>
     * **Scope:** `sip_trunk:master`
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique identifier of the account. To list internal numbers of a sub account, provide the account ID of the sub account in the `accountId` path parameter. To list internal numbers of a  master account, provide `me` as the value of the `accountId` path parameter.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list_internal_numbers(
        &self,
        account_id: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::InternalNumbers>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/sip_trunk/internal_numbers?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        let resp: crate::types::ListInternalNumbersResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.internal_numbers)
    }

    /**
     * List internal numbers.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/sip_trunk/internal_numbers` endpoint.
     *
     * As opposed to `list_internal_numbers`, this function returns all the pages of the request at once.
     *
     * This API allows a master account with SIP Connected Audio plan to list internal phone numbers (i.e., numbers that are not provided by Zoom but are owned by the organization consuming the API) assigned to a master account or a sub account.<br><br>To list internal numbers of a sub account, provide the account ID of the sub account in the `accountId` path parameter. To list internal numbers of a  master account, provide `me` as the value of the `accountId` path parameter.
     * <br><b>Prerequisites:</b><br>
     * * The account making this API request must be a [master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis) with SIP Connected Audio Plan.<br><br>
     * **Scope:** `sip_trunk:master`
     *
     *
     *
     */
    pub async fn list_all_internal_numbers(
        &self,
        account_id: &str,
    ) -> Result<Vec<crate::types::InternalNumbers>> {
        let url = format!(
            "/accounts/{}/sip_trunk/internal_numbers",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        let mut resp: crate::types::ListInternalNumbersResponse =
            self.client.get(&url, None).await?;

        let mut internal_numbers = resp.internal_numbers;
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

            internal_numbers.append(&mut resp.internal_numbers);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(internal_numbers)
    }

    /**
     * Add internal numbers.
     *
     * This function performs a `POST` to the `/accounts/{accountId}/sip_trunk/internal_numbers` endpoint.
     *
     * This API allows a master account with SIP Connected Audio plan to assign internal phone numbers (i.e., numbers that are not provided by Zoom but are owned by the organization consuming the API) to a master account or a sub account.<br><br>To add internal numbers to a sub account, provide the account ID of the sub account in the `accountId` path parameter. To add internal numbers to a master account, provide `me` as the value of the `accountId` path parameter.
     * <br><b>Prerequisites:</b><br>
     * * The account making this API request must be a [master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis) with SIP Connected Audio Plan.<br><br>
     * **Scope:** `sip_trunk:master`
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique identifier of the account.<br>To add internal numbers to a sub account, provide the account ID of the sub account in the `accountId` path parameter. To add internal numbers to a master account, provide `me` as the value of the `accountId` path parameter.
     */
    pub async fn add_internal_numbers(
        &self,
        account_id: &str,
        body: &crate::types::AddInternalNumbersRequest,
    ) -> Result<crate::types::AddInternalNumbersResponseData> {
        let url = format!(
            "/accounts/{}/sip_trunk/internal_numbers",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete an internal number.
     *
     * This function performs a `DELETE` to the `/accounts/{accountId}/sip_trunk/internal_numbers/{numberId}` endpoint.
     *
     * This API allows a master account with SIP Connected Audio plan to delete a previously assigned internal phone number from a master account or a sub account.<br><br>To delete an internal number from a sub account, provide the account ID of the sub account in the `accountId` path parameter. To delete an internal number from a master account, provide `me` as the value of the `accountId` path parameter.
     * <br><b>Prerequisites:</b><br>
     * * The account making this API request must be a [master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis) with SIP Connected Audio Plan.<br><br>
     * **Scope:** `sip_trunk:master`
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique Identifier of the account. To delete an internal number from a sub account, provide the account ID of the sub account in the `accountId` path parameter. To delete an internal number from a master account, provide `me` as the value of the `accountId` path parameter.
     * * `number_id: &str` -- Unique identifier of the phone number. This value can be retrieved by calling the List Internal Numbers API.
     */
    pub async fn delete_internal_number(&self, account_id: &str, number_id: &str) -> Result<()> {
        let url = format!(
            "/accounts/{}/sip_trunk/internal_numbers/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&number_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

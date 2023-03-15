use crate::Client;
use crate::ClientResult;

pub struct BillingPlans {
    pub client: Client,
}

impl BillingPlans {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        BillingPlans { client }
    }

    /**
     * Get Account Billing Plan.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_plan` endpoint.
     *
     * Retrieves the billing plan information for the specified account, including the current billing plan, successor plans, billing address, and billing credit card.
     *
     * By default the successor plan and credit card information is included in the response. You can exclude this information from the response by adding the appropriate optional query string and setting it to **false**.
     *
     * Response
     *
     * The response returns the billing plan information, including the currency code, for the plan. The `billingPlan` and `succesorPlans` property values are the same as those shown in the [Billing::getBillingPlan](https://developers.docusign.com/docs/esign-rest-api/reference/Billing/BillingPlans/get) reference. the `billingAddress` and `creditCardInformation` property values are the same as those shown in the [Billing::updatePlan](https://developers.docusign.com/docs/esign-rest-api/reference/Billing/BillingPlans/update) reference.
     *
     * **Note**: When credit card number information displays, a mask is applied to the response so that only the last 4 digits of the card number are visible.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_credit_card_information: &str` -- When set to **true**, payment information including credit card information will show in the return.
     * * `include_metadata: &str` -- When set to **true**, the `canUpgrade` and `renewalStatus` properities are included the response and an array of `supportedCountries` is added to the `billingAddress` information. .
     * * `include_successor_plans: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(
        &self,
        account_id: &str,
        include_credit_card_information: &str,
        include_metadata: &str,
        include_successor_plans: &str,
    ) -> ClientResult<crate::types::AccountBillingPlanResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_credit_card_information.is_empty() {
            query_args.push((
                "include_credit_card_information".to_string(),
                include_credit_card_information.to_string(),
            ));
        }
        if !include_metadata.is_empty() {
            query_args.push(("include_metadata".to_string(), include_metadata.to_string()));
        }
        if !include_successor_plans.is_empty() {
            query_args.push((
                "include_successor_plans".to_string(),
                include_successor_plans.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_plan?{}",
                crate::progenitor_support::encode_path(account_id),
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
     * Updates an account billing plan.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/billing_plan` endpoint.
     *
     * Updates the billing plan information, billing address, and credit card information for the specified account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `preview_billing_plan: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        preview_billing_plan: &str,
        body: &crate::types::BillingPlanInformation,
    ) -> ClientResult<crate::types::BillingPlanUpdateResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !preview_billing_plan.is_empty() {
            query_args.push((
                "preview_billing_plan".to_string(),
                preview_billing_plan.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_plan?{}",
                crate::progenitor_support::encode_path(account_id),
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
     * Get credit card information.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_plan/credit_card` endpoint.
     *
     * This method returns information about a credit card associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get_credit_card_info(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::CreditCardInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_plan/credit_card",
                crate::progenitor_support::encode_path(account_id),
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
     * Returns downgrade plan information for the specified account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_plan/downgrade` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get_downgrade_request_info(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::DowngradRequestBillingInfoResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_plan/downgrade",
                crate::progenitor_support::encode_path(account_id),
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
     * Queues downgrade billing plan request for an account.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/billing_plan/downgrade` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put_downgrade_account(
        &self,
        account_id: &str,
        body: &crate::types::DowngradeBillingPlanInformation,
    ) -> ClientResult<crate::types::DowngradePlanUpdateResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_plan/downgrade",
                crate::progenitor_support::encode_path(account_id),
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
     * Reserverd: Purchase additional envelopes.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/billing_plan/purchased_envelopes` endpoint.
     *
     * Reserved: At this time, this endpoint is limited to DocuSign internal use only. Completes the purchase of envelopes for your account. The actual purchase is done as part of an internal workflow interaction with an envelope vendor.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn purchased_envelopes_put(
        &self,
        account_id: &str,
        body: &crate::types::PurchasedEnvelopesInformation,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_plan/purchased_envelopes",
                crate::progenitor_support::encode_path(account_id),
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
     * Gets a list of available billing plans.
     *
     * This function performs a `GET` to the `/v2.1/billing_plans` endpoint.
     *
     * Retrieves a list of the billing plans associated with a distributor.
     */
    pub async fn get_billing_plans(&self) -> ClientResult<crate::types::BillingPlansResponse> {
        let url = self.client.url("/v2.1/billing_plans", None);
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
     * Gets billing plan details.
     *
     * This function performs a `GET` to the `/v2.1/billing_plans/{billingPlanId}` endpoint.
     *
     * Retrieves the billing plan details for the specified billing plan ID.
     *
     * **Parameters:**
     *
     * * `billing_plan_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get_plan(
        &self,
        billing_plan_id: &str,
    ) -> ClientResult<crate::types::BillingPlanResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/billing_plans/{}",
                crate::progenitor_support::encode_path(billing_plan_id),
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
}

use crate::Client;
use crate::ClientResult;

pub struct Payments {
    pub client: Client,
}

impl Payments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Payments { client }
    }

    /**
     * Gets payment information for one or more payments.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_payments` endpoint.
     *
     * Retrieves a list containing information about one or more payments. If the from date or to date queries are not used, the response returns payment information for the last 365 days.
     *
     * Privileges required: account administrator
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `from_date: &str` -- Specifies the date/time of the earliest payment in the account to retrieve.
     * * `to_date: &str` -- Specifies the date/time of the latest payment in the account to retrieve.
     */
    pub async fn billing_get_list(
        &self,
        account_id: &str,
        from_date: &str,
        to_date: &str,
    ) -> ClientResult<crate::types::BillingPaymentsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from_date.is_empty() {
            query_args.push(("from_date".to_string(), from_date.to_string()));
        }
        if !to_date.is_empty() {
            query_args.push(("to_date".to_string(), to_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_payments?{}",
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
     * Posts a payment to a past due invoice.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/billing_payments` endpoint.
     *
     * Posts a payment to a past due invoice.
     *
     * This method can only be used if the `paymentAllowed` value for a past due invoice is true. This can be determined calling [Billing::listInvoicesPastDue](https://developers.docusign.com/docs/esign-rest-api/reference/Billing/Invoices/listPastDue).
     *
     * The response returns information for a single payment, if a payment ID was used in the endpoint, or a list of payments. If the from date or to date queries or payment ID are not used, the response returns payment information for the last 365 days. If the request was for a single payment ID, the `nextUri` and `previousUri` properties are not returned.
     *
     * Privileges required: account administrator
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn billing_post(
        &self,
        account_id: &str,
        body: &crate::types::BillingPaymentRequest,
    ) -> ClientResult<crate::types::BillingPaymentResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_payments",
                crate::progenitor_support::encode_path(account_id),
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
     * Gets billing payment information for a specific payment.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_payments/{paymentId}` endpoint.
     *
     * Retrieves the information for a specified payment.
     *
     * Privileges required: account administrator
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `payment_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn billing_get(
        &self,
        account_id: &str,
        payment_id: &str,
    ) -> ClientResult<crate::types::BillingPaymentItem> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_payments/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(payment_id),
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

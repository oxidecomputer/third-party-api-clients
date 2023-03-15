use crate::Client;
use crate::ClientResult;

pub struct Invoices {
    pub client: Client,
}

impl Invoices {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Invoices { client }
    }

    /**
     * Get a List of Billing Invoices.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_invoices` endpoint.
     *
     * Retrieves a list of invoices for the account. If the from date or to date queries are not specified, the response returns invoices for the last 365 days.
     *
     * Privileges required: account administrator
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `from_date: &str` -- Specifies the date/time of the earliest invoice in the account to retrieve.
     * * `to_date: &str` -- Specifies the date/time of the latest invoice in the account to retrieve.
     */
    pub async fn billing_get(
        &self,
        account_id: &str,
        from_date: &str,
        to_date: &str,
    ) -> ClientResult<crate::types::BillingInvoicesResponse> {
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
                "/v2.1/accounts/{}/billing_invoices?{}",
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
     * Retrieves a billing invoice.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_invoices/{invoiceId}` endpoint.
     *
     * Retrieves the specified invoice.
     *
     * **Note**: If the `pdfAvailable` property in the response is set to *true*, you can download a PDF version of the invoice. To download the PDF, make the call again and change the value of the `Accept` property in the header to `Accept: application/pdf`.
     *
     * Privileges required: account administrator
     *
     * The response returns a list of charges and information about the charges. Quantities are usually shown as 'unlimited' or an integer. Amounts are shown in the currency set for the account.
     *
     * **Response**
     * The following table provides a description of the different `chargeName` property values. The information will grow as more chargeable items are added to the system.
     *
     * | chargeName | Description |
     * | --- | --- |
     * | id_check | ID Check Charge |
     * | in_person_signing | In Person Signing charge |
     * | envelopes Included | Sent Envelopes for the account |
     * | age_verify | Age verification check |
     * | ofac | OFAC Check |
     * | id_confirm | ID confirmation check |
     * | student_authentication | STAN PIN authentication check |
     * | wet_sign_fax | Pages for returning signed documents by fax |
     * | attachment_fax | Pages for returning attachments by fax |
     * | phone_authentication | Phone authentication charge |
     * | powerforms | PowerForm envelopes sent |
     * | signer_payments | Payment processing charge |
     * | outbound_fax | Send by fax charge |
     * | bulk_recipient_envelopes | Bulk Recipient Envelopes sent |
     * | sms_authentications | SMS authentication charge |
     * | saml_authentications | SAML authentication charge |
     * | express_signer_certificate | DocuSign Express Certificate charge |
     * | personal_signer_certificate | Personal Signer Certificate charge |
     * | safe_certificate | SAFE BioPharma Signer Certificate charge |
     * | seats | Included active seats charge |
     * | open_trust_certificate | OpenTrust Signer Certificate charge |
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `invoice_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn billing_get_invoices(
        &self,
        account_id: &str,
        invoice_id: &str,
    ) -> ClientResult<crate::types::BillingInvoice> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_invoices/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(invoice_id),
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
     * Get a list of past due invoices.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_invoices_past_due` endpoint.
     *
     * Returns a list past due invoices for the account and notes if payment can be made through the REST API.
     *
     * Privileges Required: account administrator
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn billing_get_past_due(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::BillingInvoicesSummary> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_invoices_past_due",
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
}

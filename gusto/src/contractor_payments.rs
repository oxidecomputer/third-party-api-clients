use crate::Client;
use crate::ClientResult;

pub struct ContractorPayments {
    pub client: Client,
}

impl ContractorPayments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ContractorPayments { client }
    }

    /**
     * Get contractor payments for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/contractor_payments` endpoint.
     *
     * Returns an object containing individual contractor payments, within a given time period, including totals.
     *
     * **Parameters:**
     *
     * * `start_date: &str` -- The time period for which to retrieve contractor payments.
     * * `end_date: &str` -- The time period for which to retrieve contractor payments.
     */
    pub async fn get_company(
        &self,
        company_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<crate::types::ContractorPaymentSummary> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/contractor_payments?{}",
                crate::progenitor_support::encode_path(company_id),
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
     * Create a contractor payment (Beta).
     *
     * This function performs a `POST` to the `/v1/companies/{company_id}/contractor_payments` endpoint.
     *
     * Returns an object containing individual contractor payments, within a given time period, including totals.
     *
     * This endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.
     *
     * **Parameters:**
     *
     * * `date: &str` -- A unique identifier of the employee in Gusto.
     * * `contractor_id: f64` -- The contractor receiving the payment.
     * * `wage: f64` -- If the contractor is on a fixed wage, this is the fixed wage payment for the contractor, regardless of hours worked.
     * * `hours: f64` -- If the contractor is on an hourly wage, this is the number of hours that the contractor worked for the payment.
     * * `bonus: f64` -- If the contractor is on an hourly wage, this is the bonus the contractor earned.
     * * `reimbursement: f64` -- Reimbursed wages for the contractor .
     */
    pub async fn post_company(
        &self,
        company_id: &str,
        date: &str,
        contractor_id: f64,
        wage: f64,
        hours: f64,
        bonus: f64,
        reimbursement: f64,
    ) -> ClientResult<crate::types::ContractorPayment> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !bonus.to_string().is_empty() {
            query_args.push(("bonus".to_string(), bonus.to_string()));
        }
        if !contractor_id.to_string().is_empty() {
            query_args.push(("contractor_id".to_string(), contractor_id.to_string()));
        }
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !hours.to_string().is_empty() {
            query_args.push(("hours".to_string(), hours.to_string()));
        }
        if !reimbursement.to_string().is_empty() {
            query_args.push(("reimbursement".to_string(), reimbursement.to_string()));
        }
        if !wage.to_string().is_empty() {
            query_args.push(("wage".to_string(), wage.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/contractor_payments?{}",
                crate::progenitor_support::encode_path(company_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a single contractor payment.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/contractor_payments/{contractor_payment_id_or_uuid}` endpoint.
     *
     * Returns a single contractor payments
     */
    pub async fn get_company_contractor_payments(
        &self,
        company_id: &str,
        contractor_payment_id_or_uuid: &str,
    ) -> ClientResult<crate::types::ContractorPayment> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/contractor_payments/{}",
                crate::progenitor_support::encode_path(company_id),
                crate::progenitor_support::encode_path(contractor_payment_id_or_uuid),
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
     * Cancel a contractor payment (Beta).
     *
     * This function performs a `DELETE` to the `/v1/companies/{company_id}/contractor_payments/{contractor_payment_id_or_uuid}` endpoint.
     *
     * Cancels and deletes a contractor payment. If the contractor payment has already started processing, the payment cannot be cancelled.
     *
     * This endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.
     */
    pub async fn delete_company(
        &self,
        company_id: &str,
        contractor_payment_id_or_uuid: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/contractor_payments/{}",
                crate::progenitor_support::encode_path(company_id),
                crate::progenitor_support::encode_path(contractor_payment_id_or_uuid),
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
}

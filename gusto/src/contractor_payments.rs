use anyhow::Result;

use crate::Client;

pub struct ContractorPayments {
    client: Client,
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
    ) -> Result<crate::types::ContractorPaymentSummary> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !end_date.is_empty() {
            query_args.push(format!("end_date={}", end_date));
        }
        if !start_date.is_empty() {
            query_args.push(format!("start_date={}", start_date));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v1/companies/{}/contractor_payments?{}",
            crate::progenitor_support::encode_path(&company_id.to_string()),
            query
        );

        self.client.get(&url, None).await
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
    pub async fn post_company_contractor_payment(
        &self,
        company_id: &str,
        date: &str,
        contractor_id: f64,
        wage: f64,
        hours: f64,
        bonus: f64,
        reimbursement: f64,
    ) -> Result<crate::types::ContractorPayment> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("bonus={}", bonus));
        query_args.push(format!("contractor_id={}", contractor_id));
        if !date.is_empty() {
            query_args.push(format!("date={}", date));
        }
        query_args.push(format!("hours={}", hours));
        query_args.push(format!("reimbursement={}", reimbursement));
        query_args.push(format!("wage={}", wage));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v1/companies/{}/contractor_payments?{}",
            crate::progenitor_support::encode_path(&company_id.to_string()),
            query
        );

        self.client.post(&url, None).await
    }

    /**
     * Get a single contractor payment.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/contractor_payments/{contractor_payment_id_or_uuid}` endpoint.
     *
     * Returns a single contractor payments
     */
    pub async fn get_company_contractor_payment(
        &self,
        company_id: &str,
        contractor_payment_id_or_uuid: &str,
    ) -> Result<crate::types::ContractorPayment> {
        let url = format!(
            "/v1/companies/{}/contractor_payments/{}",
            crate::progenitor_support::encode_path(&company_id.to_string()),
            crate::progenitor_support::encode_path(&contractor_payment_id_or_uuid.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn delete_company_contractor_payment(
        &self,
        company_id: &str,
        contractor_payment_id_or_uuid: &str,
    ) -> Result<()> {
        let url = format!(
            "/v1/companies/{}/contractor_payments/{}",
            crate::progenitor_support::encode_path(&company_id.to_string()),
            crate::progenitor_support::encode_path(&contractor_payment_id_or_uuid.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

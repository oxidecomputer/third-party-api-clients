use anyhow::Result;

use crate::Client;

pub struct Billing {
    pub client: Client,
}

impl Billing {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Billing { client }
    }

    /**
     * Get billing information.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/billing` endpoint.
     *
     * Get [billing information](https://support.zoom.us/hc/en-us/articles/201363263-About-Billing) of a sub account.<br><br>Only master accounts can use this API. Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage sub accounts' billing information. Email the partner programs team at **partner-success@zoom.us** for more details.<br>
     *
     * **Prerequisites:**
     * * Pro or a higher paid account with master account option enabled. <br>
     *
     * **Scope**:`billing:master`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn account(&self, account_id: &str) -> Result<crate::types::Contact> {
        let url = format!(
            "/accounts/{}/billing",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update billing information.
     *
     * This function performs a `PATCH` to the `/accounts/{accountId}/billing` endpoint.
     *
     * Update [billing information](https://support.zoom.us/hc/en-us/articles/201363263-About-Billing) of a sub account.<br><br>
     * This API can only be used by master accounts that pay all billing charges of their associated sub accounts. Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage sub accounts' billing information. Email the partner programs team at **partner-success@zoom.us** for more details.<br><br>
     *
     * **Prerequisites:**
     * * Pro or a higher paid account with master account option enabled. <br>
     *
     * **Scope**:`billing:master`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn account_update(
        &self,
        account_id: &str,
        body: &crate::types::BillingContact,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/billing",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get plan Information.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/plans` endpoint.
     *
     * Get plan information of a sub account that is managed by a master account. <br><br>This API can only be used by master accounts that pay all billing charges of their associated Pro or higher sub accounts. Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage sub accounts' billing information. Email the partner programs team at **partner-success@zoom.us** for more details.<br><br>
     * **Scopes:** `billing:master`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *  
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn account_plans(
        &self,
        account_id: &str,
    ) -> Result<crate::types::AccountPlansResponse> {
        let url = format!(
            "/accounts/{}/plans",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Subscribe plans.
     *
     * This function performs a `POST` to the `/accounts/{accountId}/plans` endpoint.
     *
     * Subscribe a sub account to a Zoom plan using your master account. This API can only be used by master accounts that pay all billing charges of their associated Pro or higher sub accounts. Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage sub accounts' subscriptions. Email the partner programs team at **partner-success@zoom.us** for more details.<br><br>
     * **Scopes**: `billing:master`<br>
     *  
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn account_plan_create(
        &self,
        account_id: &str,
        body: &crate::types::AccountPlanCreateRequestAllOf,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/plans",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update a base plan.
     *
     * This function performs a `PUT` to the `/accounts/{accountId}/plans/base` endpoint.
     *
     * Update a base plan of a sub account.
     *
     * This API can only be used by master accounts that pay all billing charges of their associated Pro or higher subaccounts. Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage subaccounts' subscriptions. Email the partner programs team at **partner-success@zoom.us** for more details.
     *
     * **Scopes:** `billing:master`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Prerequisites:**<br>
     * * The subaccount must have a Pro or a higher plan.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn account_plan_base_update(
        &self,
        account_id: &str,
        body: &crate::types::PlanBase,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/plans/base",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update an additional plan.
     *
     * This function performs a `PUT` to the `/accounts/{accountId}/plans/addons` endpoint.
     *
     * Update an additional plan for a sub account.
     *
     * This API can only be used by master accounts that pay all billing charges of their associated Pro or higher sub accounts. Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage sub accounts' subscriptions. Email the partner programs team at **partner-success@zoom.us** for more details.<br><br>
     * <br>**Prerequisites:**<br>
     * * Pro or a higher plan with master account enabled.
     * * The sub account must be a paid account. The billing charges for the sub account must be paid by the master account.<br><br>
     *
     * **Scopes**: `billing:master`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     *
     *
     *  
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn account_plan_addon_update(
        &self,
        account_id: &str,
        body: &crate::types::PlanBase,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/plans/addons",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Subscribe additional plan.
     *
     * This function performs a `POST` to the `/accounts/{accountId}/plans/addons` endpoint.
     *
     * Subscribe a subaccount to a Zoom addon plan. This API can **only** be used by master accounts that pay all billing charges of their associated Pro or higher subaccounts. Zoom allows only approved partners to use [master APIs](https://marketplace.zoom.us/docs/api-reference/master-account-apis) and manage subaccounts' subscriptions. For more information, email the partner programs team at **partner-success@zoom.us**.
     *
     * **Scopes**: `billing:master`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Prerequisites:**
     * * Pro or a higher plan with master account option enabled
     * * The subaccount must be a paid account whose billing charges are paid by its master account
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn account_plan_addon_create(
        &self,
        account_id: &str,
        body: &crate::types::AccountPlanAddonCreateRequestOneOf,
    ) -> Result<crate::types::AccountPlans> {
        let url = format!(
            "/accounts/{}/plans/addons",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Cancel a base plan.
     *
     * This function performs a `PATCH` to the `/accounts/{accountId}/plans/base/status` endpoint.
     *
     * [Cancel a base plan](https://support.zoom.us/hc/en-us/articles/203634215-How-Do-I-Cancel-My-Subscription-) for a sub account.
     *
     * This API can only be used by master accounts that pay all billing charges of their associated Pro or higher sub accounts. Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage sub accounts' subscriptions. Email the partner programs team at **partner-success@zoom.us** for more details.<br><br>
     *
     * **Scopes**: `billing:master`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * The sub account must have a Pro or a higher plan.
     *  
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn account_plan_base_delete(
        &self,
        account_id: &str,
        body: &crate::types::AccountPlanBaseDeleteRequest,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/plans/base/status",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Cancel additional plans.
     *
     * This function performs a `PATCH` to the `/accounts/{accountId}/plans/addons/status` endpoint.
     *
     * [Cancel additional plan](https://support.zoom.us/hc/en-us/articles/203634215-How-Do-I-Cancel-My-Subscription-) of a sub account. The cancellation does not provide refund for the current subscription. The service remains active for the current session.
     *
     * This API can only be used by master accounts that pay all billing charges of their associated Pro or higher sub accounts. Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage sub accounts' subscriptions. Email the partner programs team at **partner-success@zoom.us** for more details.<br><br>
     *
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     *
     * **Prerequisites:**<br>
     * * Pro or a higher plan with master account option enabled.
     * * The sub account must be a paid account.<br>
     * **Scope:** `billing:master`<br>
     *  
     */
    pub async fn account_plan_addon_cancel(
        &self,
        account_id: &str,
        body: &crate::types::AccountPlanAddonCancelRequest,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/plans/addons/status",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get plan usage.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/plans/usage` endpoint.
     *
     * Get information on usage of [plans](https://marketplace.zoom.us/docs/api-reference/other-references/plans) of an account. This API supports regular accounts as well as master and sub accounts. To get plan usage of a regular account, use the `account:read:admin` scope and provide “me” as the value of the  `accountId` path parameter.To get plan usage of a master account, provide the keyword "me" as the value of the `accountId` path parameter and use the `billing:master` scope. To get plan usage of a sub account, provide the actual account Id of the sub account as the value of the `accountId` path parameter and use the `billing:master` scope.
     *
     * **Prerequisite**:<br>
     * Account type: master account on a paid Pro, Business or Enterprise plan.<br>
     * **Scope:** `billing:master` for master and sub accounts. `account:read:admin` for regular Zoom accounts.<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     */
    pub async fn get_plan_usage(
        &self,
        account_id: &str,
    ) -> Result<crate::types::GetPlanUsageResponse> {
        let url = format!(
            "/accounts/{}/plans/usage",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * List billing invoices.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/billing/invoices` endpoint.
     *
     * List [invoices](https://support.zoom.us/hc/en-us/articles/207276556-Viewing-your-invoice-history#h_6710542f-23cc-4059-9cc7-ff02bec7314e) of a Zoom account.
     *
     * To list a regular Zoom account's invoices or a master account's invoices, provide `me` as the value of the `accountId` path parameter. To list a sub account's invoices, provide the account ID of the sub account in the `accountId` path parameter.
     *
     * **Prerequisites:**
     * * Account must be enrolled in Pro or a higher plan.<br>
     *
     * **Scope**:`billing:master`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Additional Rate Limit:** You can make **one** API request per account(`accountId`) every **five** minutes until the daily limit is reached. This API has a daily limit of **6** requests per account(`accountId`).
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date for the invoice query in `yyyy-mm-dd` format. The date range defined by the “from” and “to” parameters should not exceed one year. The range defined should fall within the past three years.
     *   .
     * * `to: chrono::NaiveDate` -- End date for the invoice query in `yyyy-mm-dd` format.
     */
    pub async fn account_invoice(
        &self,
        account_id: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<crate::types::AccountBillingInvoicesResponseData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/billing/invoices?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get invoice details.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/billing/invoices/{invoiceId}` endpoint.
     *
     * Get detailed information about a specific [invoice](https://support.zoom.us/hc/en-us/articles/207276556-Viewing-your-invoice-history#h_6710542f-23cc-4059-9cc7-ff02bec7314e). <br>To retrieve a regular Zoom account's invoice details or a master account's invoice details, provide `me` as the value of `accountId` path parameter. To list a sub account's invoice details, provide the account ID of the sub account in the `accountId` path parameter.
     *
     * **Prerequisites:**
     * * Account must be enrolled in Pro or a higher plan. <br>
     *
     * **Scope**:`billing:master`<br>
     * <br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Additional Rate Limit:** You can make **one** API request every **thirty** minutes until the daily limit is reached. This API has a daily limit of **100** requests per **account**.
     */
    pub async fn get_account_invoice(
        &self,
        account_id: &str,
        invoice_id: &str,
    ) -> Result<crate::types::GetAccountBillingInvoiceResponse> {
        let url = format!(
            "/accounts/{}/billing/invoices/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&invoice_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Download an invoice file.
     *
     * This function performs a `GET` to the `/api/download/billing/invoices/{invoiceId}` endpoint.
     *
     * Use this API to download a Zoom account’s [billed](https://support.zoom.us/hc/en-us/articles/201363263-About-Billing) invoice file, in PDF format. To get an account’s invoice ID, use the **[List billing invoices](https://marketplace.zoom.us/docs/api-reference/zoom-api/billing/accountbillinginvoices)** API.
     *
     * **Scopes:** `billing:master`<br>**Rate Limits:**
     * * You can make **one** request to this API every **30 minutes** until the daily limit is reached.
     * * This API has a daily limit of **100 requests per account**.
     */
    pub async fn download_invoice_pdf(&self, invoice_id: &str) -> Result<()> {
        let url = format!(
            "/api/download/billing/invoices/{}",
            crate::progenitor_support::encode_path(&invoice_id.to_string()),
        );

        self.client.get(&url, None).await
    }
}

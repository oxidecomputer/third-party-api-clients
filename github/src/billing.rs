use crate::Client;
use crate::ClientResult;

pub struct Billing {
    pub client: Client,
}

impl Billing {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Billing { client }
    }

    /**
     * Get all budgets for an organization.
     *
     * This function performs a `GET` to the `/organizations/{org}/settings/billing/budgets` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Gets all budgets for an organization. The authenticated user must be an organization admin or billing manager.
     * Each page returns up to 10 budgets.
     *
     * FROM: <https://docs.github.com/rest/billing/budgets#get-all-budgets-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `page: i64` -- The page number of the results to fetch.
     * * `per_page: i64` -- The number of results per page (max 10).
     * * `scope: crate::types::BudgetScope` -- The type of scope for the budget.
     */
    pub async fn get_all_budgets_org(
        &self,
        org: &str,
        page: i64,
        per_page: i64,
        scope: crate::types::BudgetScope,
    ) -> ClientResult<crate::Response<crate::types::GetAllBudgets>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !scope.to_string().is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/organizations/{}/settings/billing/budgets?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get a budget by ID for an organization.
     *
     * This function performs a `GET` to the `/organizations/{org}/settings/billing/budgets/{budget_id}` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Gets a budget by ID. The authenticated user must be an organization admin or billing manager.
     *
     * FROM: <https://docs.github.com/rest/billing/budgets#get-a-budget-by-id-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `budget_id: &str` -- The ID corresponding to the budget.
     */
    pub async fn get_budget_org(
        &self,
        org: &str,
        budget_id: &str,
    ) -> ClientResult<crate::Response<crate::types::GetBudget>> {
        let url = self.client.url(
            &format!(
                "/organizations/{}/settings/billing/budgets/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&budget_id.to_string()),
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
     * Delete a budget for an organization.
     *
     * This function performs a `DELETE` to the `/organizations/{org}/settings/billing/budgets/{budget_id}` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Deletes a budget by ID for an organization. The authenticated user must be an organization admin or billing manager.
     *
     * FROM: <https://docs.github.com/rest/billing/budgets#delete-a-budget-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `budget_id: &str` -- The ID corresponding to the budget.
     */
    pub async fn delete_budget_org(
        &self,
        org: &str,
        budget_id: &str,
    ) -> ClientResult<crate::Response<crate::types::DeleteBudget>> {
        let url = self.client.url(
            &format!(
                "/organizations/{}/settings/billing/budgets/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&budget_id.to_string()),
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
    /**
     * Update a budget for an organization.
     *
     * This function performs a `PATCH` to the `/organizations/{org}/settings/billing/budgets/{budget_id}` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Updates an existing budget for an organization. The authenticated user must be an organization admin or billing manager.
     *
     * FROM: <https://docs.github.com/rest/billing/budgets#update-a-budget-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `budget_id: &str` -- The ID corresponding to the budget.
     */
    pub async fn update_budget_org(
        &self,
        org: &str,
        budget_id: &str,
        body: &crate::types::BillingUpdateBudgetOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::BillingUpdateBudgetOrgResponseData>> {
        let url = self.client.url(
            &format!(
                "/organizations/{}/settings/billing/budgets/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&budget_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get billing premium request usage report for an organization.
     *
     * This function performs a `GET` to the `/organizations/{org}/settings/billing/premium_request/usage` endpoint.
     *
     * Gets a report of premium request usage for an organization. To use this endpoint, you must be an administrator of an organization within an enterprise or an organization account.
     *
     * **Note:** Only data from the past 24 months is accessible via this endpoint.
     *
     * FROM: <https://docs.github.com/rest/billing/usage#get-billing-premium-request-usage-report-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `year: i64` -- If specified, only return results for a single year. The value of `year` is an integer with four digits representing a year. For example, `2025`. Default value is the current year.
     * * `month: i64` -- If specified, only return results for a single month. The value of `month` is an integer between `1` and `12`. Default value is the current month. If no year is specified the default `year` is used.
     * * `day: i64` -- If specified, only return results for a single day. The value of `day` is an integer between `1` and `31`. If no `year` or `month` is specified, the default `year` and `month` are used.
     * * `user: &str` -- The user name to query usage for. The name is not case sensitive.
     * * `model: &str` -- The model name to query usage for. The name is not case sensitive.
     * * `product: &str` -- The product name to query usage for. The name is not case sensitive.
     */
    pub async fn get_github_billing_premium_request_usage_report_org(
        &self,
        org: &str,
        year: i64,
        month: i64,
        day: i64,
        user: &str,
        model: &str,
        product: &str,
    ) -> ClientResult<crate::Response<crate::types::BillingPremiumRequestUsageReportOrg>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if day > 0 {
            query_args.push(("day".to_string(), day.to_string()));
        }
        if !model.is_empty() {
            query_args.push(("model".to_string(), model.to_string()));
        }
        if month > 0 {
            query_args.push(("month".to_string(), month.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        if year > 0 {
            query_args.push(("year".to_string(), year.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/organizations/{}/settings/billing/premium_request/usage?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get billing usage report for an organization.
     *
     * This function performs a `GET` to the `/organizations/{org}/settings/billing/usage` endpoint.
     *
     * Gets a report of the total usage for an organization. To use this endpoint, you must be an administrator of an organization within an enterprise or an organization account.
     *
     * **Note:** This endpoint is only available to organizations with access to the enhanced billing platform. For more information, see "[About the enhanced billing platform](https://docs.github.com/billing/using-the-new-billing-platform)."
     *
     * FROM: <https://docs.github.com/rest/billing/usage#get-billing-usage-report-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `year: i64` -- If specified, only return results for a single year. The value of `year` is an integer with four digits representing a year. For example, `2025`. Default value is the current year.
     * * `month: i64` -- If specified, only return results for a single month. The value of `month` is an integer between `1` and `12`. If no year is specified the default `year` is used.
     * * `day: i64` -- If specified, only return results for a single day. The value of `day` is an integer between `1` and `31`. If no `year` or `month` is specified, the default `year` and `month` are used.
     */
    pub async fn get_github_billing_usage_report_org(
        &self,
        org: &str,
        year: i64,
        month: i64,
        day: i64,
    ) -> ClientResult<crate::Response<crate::types::BillingUsageReport>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if day > 0 {
            query_args.push(("day".to_string(), day.to_string()));
        }
        if month > 0 {
            query_args.push(("month".to_string(), month.to_string()));
        }
        if year > 0 {
            query_args.push(("year".to_string(), year.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/organizations/{}/settings/billing/usage?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get billing usage summary for an organization.
     *
     * This function performs a `GET` to the `/organizations/{org}/settings/billing/usage/summary` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Gets a summary report of usage for an organization. To use this endpoint, you must be an administrator of an organization within an enterprise or an organization account.
     *
     * **Note:** Only data from the past 24 months is accessible via this endpoint.
     *
     * FROM: <https://docs.github.com/rest/billing/usage#get-billing-usage-summary-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `year: i64` -- If specified, only return results for a single year. The value of `year` is an integer with four digits representing a year. For example, `2025`. Default value is the current year.
     * * `month: i64` -- If specified, only return results for a single month. The value of `month` is an integer between `1` and `12`. Default value is the current month. If no year is specified the default `year` is used.
     * * `day: i64` -- If specified, only return results for a single day. The value of `day` is an integer between `1` and `31`. If no `year` or `month` is specified, the default `year` and `month` are used.
     * * `repository: &str` -- The repository name to query for usage in the format owner/repository.
     * * `product: &str` -- The product name to query usage for. The name is not case sensitive.
     * * `sku: &str` -- The SKU to query for usage.
     */
    pub async fn get_github_billing_usage_summary_report_org(
        &self,
        org: &str,
        year: i64,
        month: i64,
        day: i64,
        repository: &str,
        product: &str,
        sku: &str,
    ) -> ClientResult<crate::Response<crate::types::BillingUsageSummaryReportOrg>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if day > 0 {
            query_args.push(("day".to_string(), day.to_string()));
        }
        if month > 0 {
            query_args.push(("month".to_string(), month.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if !repository.is_empty() {
            query_args.push(("repository".to_string(), repository.to_string()));
        }
        if !sku.is_empty() {
            query_args.push(("sku".to_string(), sku.to_string()));
        }
        if year > 0 {
            query_args.push(("year".to_string(), year.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/organizations/{}/settings/billing/usage/summary?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get billing premium request usage report for a user.
     *
     * This function performs a `GET` to the `/users/{username}/settings/billing/premium_request/usage` endpoint.
     *
     * Gets a report of premium request usage for a user.
     *
     * **Note:** Only data from the past 24 months is accessible via this endpoint.
     *
     * FROM: <https://docs.github.com/rest/billing/usage#get-billing-premium-request-usage-report-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `year: i64` -- If specified, only return results for a single year. The value of `year` is an integer with four digits representing a year. For example, `2025`. Default value is the current year.
     * * `month: i64` -- If specified, only return results for a single month. The value of `month` is an integer between `1` and `12`. Default value is the current month. If no year is specified the default `year` is used.
     * * `day: i64` -- If specified, only return results for a single day. The value of `day` is an integer between `1` and `31`. If no `year` or `month` is specified, the default `year` and `month` are used.
     * * `model: &str` -- The model name to query usage for. The name is not case sensitive.
     * * `product: &str` -- The product name to query usage for. The name is not case sensitive.
     */
    pub async fn get_github_billing_premium_request_usage_report_user(
        &self,
        username: &str,
        year: i64,
        month: i64,
        day: i64,
        model: &str,
        product: &str,
    ) -> ClientResult<crate::Response<crate::types::BillingPremiumRequestUsageReportUser>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if day > 0 {
            query_args.push(("day".to_string(), day.to_string()));
        }
        if !model.is_empty() {
            query_args.push(("model".to_string(), model.to_string()));
        }
        if month > 0 {
            query_args.push(("month".to_string(), month.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if year > 0 {
            query_args.push(("year".to_string(), year.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/settings/billing/premium_request/usage?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Get billing usage report for a user.
     *
     * This function performs a `GET` to the `/users/{username}/settings/billing/usage` endpoint.
     *
     * Gets a report of the total usage for a user.
     *
     * **Note:** This endpoint is only available to users with access to the enhanced billing platform.
     *
     * FROM: <https://docs.github.com/rest/billing/usage#get-billing-usage-report-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `year: i64` -- If specified, only return results for a single year. The value of `year` is an integer with four digits representing a year. For example, `2025`. Default value is the current year.
     * * `month: i64` -- If specified, only return results for a single month. The value of `month` is an integer between `1` and `12`. If no year is specified the default `year` is used.
     * * `day: i64` -- If specified, only return results for a single day. The value of `day` is an integer between `1` and `31`. If no `year` or `month` is specified, the default `year` and `month` are used.
     */
    pub async fn get_github_billing_usage_report_user(
        &self,
        username: &str,
        year: i64,
        month: i64,
        day: i64,
    ) -> ClientResult<crate::Response<crate::types::BillingUsageReportUser>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if day > 0 {
            query_args.push(("day".to_string(), day.to_string()));
        }
        if month > 0 {
            query_args.push(("month".to_string(), month.to_string()));
        }
        if year > 0 {
            query_args.push(("year".to_string(), year.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/settings/billing/usage?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Get billing usage summary for a user.
     *
     * This function performs a `GET` to the `/users/{username}/settings/billing/usage/summary` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Gets a summary report of usage for a user.
     *
     * **Note:** Only data from the past 24 months is accessible via this endpoint.
     *
     * FROM: <https://docs.github.com/rest/billing/usage#get-billing-usage-summary-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `year: i64` -- If specified, only return results for a single year. The value of `year` is an integer with four digits representing a year. For example, `2025`. Default value is the current year.
     * * `month: i64` -- If specified, only return results for a single month. The value of `month` is an integer between `1` and `12`. Default value is the current month. If no year is specified the default `year` is used.
     * * `day: i64` -- If specified, only return results for a single day. The value of `day` is an integer between `1` and `31`. If no `year` or `month` is specified, the default `year` and `month` are used.
     * * `repository: &str` -- The repository name to query for usage in the format owner/repository.
     * * `product: &str` -- The product name to query usage for. The name is not case sensitive.
     * * `sku: &str` -- The SKU to query for usage.
     */
    pub async fn get_github_billing_usage_summary_report_user(
        &self,
        username: &str,
        year: i64,
        month: i64,
        day: i64,
        repository: &str,
        product: &str,
        sku: &str,
    ) -> ClientResult<crate::Response<crate::types::BillingUsageSummaryReportUser>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if day > 0 {
            query_args.push(("day".to_string(), day.to_string()));
        }
        if month > 0 {
            query_args.push(("month".to_string(), month.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if !repository.is_empty() {
            query_args.push(("repository".to_string(), repository.to_string()));
        }
        if !sku.is_empty() {
            query_args.push(("sku".to_string(), sku.to_string()));
        }
        if year > 0 {
            query_args.push(("year".to_string(), year.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/settings/billing/usage/summary?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
}

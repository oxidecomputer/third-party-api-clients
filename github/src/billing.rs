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
     * Get GitHub Actions billing for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/settings/billing/actions` endpoint.
     *
     * Gets the summary of the free and paid GitHub Actions minutes used.
     *
     * Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
     *
     * The authenticated user must be an enterprise admin.
     *
     * FROM: <https://docs.github.com/rest/reference/billing#get-github-actions-billing-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn get_github_actions_billing_ghe(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsBillingUsage>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/settings/billing/actions",
                crate::progenitor_support::encode_path(enterprise),
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
     * Get GitHub Packages billing for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/settings/billing/packages` endpoint.
     *
     * Gets the free and paid storage used for GitHub Packages in gigabytes.
     *
     * Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
     *
     * The authenticated user must be an enterprise admin.
     *
     * FROM: <https://docs.github.com/rest/reference/billing#get-github-packages-billing-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn get_github_packages_billing_ghe(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<crate::types::PackagesBillingUsage>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/settings/billing/packages",
                crate::progenitor_support::encode_path(enterprise),
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
     * Get shared storage billing for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/settings/billing/shared-storage` endpoint.
     *
     * Gets the estimated paid and estimated total storage used for GitHub Actions and Github Packages.
     *
     * Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
     *
     * The authenticated user must be an enterprise admin.
     *
     * FROM: <https://docs.github.com/rest/reference/billing#get-shared-storage-billing-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn get_shared_storage_billing_ghe(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<crate::types::CombinedBillingUsage>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/settings/billing/shared-storage",
                crate::progenitor_support::encode_path(enterprise),
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
     * Get GitHub Actions billing for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/settings/billing/actions` endpoint.
     *
     * Gets the summary of the free and paid GitHub Actions minutes used.
     *
     * Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
     *
     * Access tokens must have the `repo` or `admin:org` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/billing#get-github-actions-billing-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn get_github_actions_billing_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsBillingUsage>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/billing/actions",
                crate::progenitor_support::encode_path(org),
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
     * Get GitHub Packages billing for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/settings/billing/packages` endpoint.
     *
     * Gets the free and paid storage used for GitHub Packages in gigabytes.
     *
     * Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
     *
     * Access tokens must have the `repo` or `admin:org` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/billing#get-github-packages-billing-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn get_github_packages_billing_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::PackagesBillingUsage>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/billing/packages",
                crate::progenitor_support::encode_path(org),
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
     * Get shared storage billing for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/settings/billing/shared-storage` endpoint.
     *
     * Gets the estimated paid and estimated total storage used for GitHub Actions and Github Packages.
     *
     * Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
     *
     * Access tokens must have the `repo` or `admin:org` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/billing#get-shared-storage-billing-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn get_shared_storage_billing_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::CombinedBillingUsage>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/billing/shared-storage",
                crate::progenitor_support::encode_path(org),
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
     * Get GitHub Actions billing for a user.
     *
     * This function performs a `GET` to the `/users/{username}/settings/billing/actions` endpoint.
     *
     * Gets the summary of the free and paid GitHub Actions minutes used.
     *
     * Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
     *
     * Access tokens must have the `user` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/billing#get-github-actions-billing-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn get_github_actions_billing_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsBillingUsage>> {
        let url = self.client.url(
            &format!(
                "/users/{}/settings/billing/actions",
                crate::progenitor_support::encode_path(username),
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
     * Get GitHub Packages billing for a user.
     *
     * This function performs a `GET` to the `/users/{username}/settings/billing/packages` endpoint.
     *
     * Gets the free and paid storage used for GitHub Packages in gigabytes.
     *
     * Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
     *
     * Access tokens must have the `user` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/billing#get-github-packages-billing-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn get_github_packages_billing_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::PackagesBillingUsage>> {
        let url = self.client.url(
            &format!(
                "/users/{}/settings/billing/packages",
                crate::progenitor_support::encode_path(username),
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
     * Get shared storage billing for a user.
     *
     * This function performs a `GET` to the `/users/{username}/settings/billing/shared-storage` endpoint.
     *
     * Gets the estimated paid and estimated total storage used for GitHub Actions and Github Packages.
     *
     * Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
     *
     * Access tokens must have the `user` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/billing#get-shared-storage-billing-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn get_shared_storage_billing_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::CombinedBillingUsage>> {
        let url = self.client.url(
            &format!(
                "/users/{}/settings/billing/shared-storage",
                crate::progenitor_support::encode_path(username),
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

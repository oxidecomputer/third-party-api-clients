use anyhow::Result;

use crate::Client;

pub struct Billing {
    client: Client,
}

impl Billing {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Billing { client }
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
    ) -> Result<crate::types::CombinedBillingUsage> {
        let url = format!(
            "/users/{}/settings/billing/shared-storage",
            crate::progenitor_support::encode_path(&username.to_string()),
        );

        self.client.get(&url).await
    }
}

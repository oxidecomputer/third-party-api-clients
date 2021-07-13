use anyhow::Result;

use crate::Client;

pub struct SecretScanning {
    client: Client,
}

impl SecretScanning {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SecretScanning { client }
    }

    /**
     * List secret scanning alerts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/secret-scanning/alerts` endpoint.
     *
     * Lists all secret scanning alerts for a private repository, from newest to oldest. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
     *
     * GitHub Apps must have the `secret_scanning_alerts` read permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/secret-scanning#list-secret-scanning-alerts-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `state: crate::types::SecretScanningAlertState` -- Set to `open` or `resolved` to only list secret scanning alerts in a specific state.
     * * `secret_type: &str` -- A comma separated list of secret types to return. By default all secret types are returned.
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     */
    pub async fn list_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::SecretScanningAlertState,
        secret_type: &str,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<crate::types::SecretScanningAlert>> {
        let url = format!(
            "/repos/{}/{}/secret-scanning/alerts?page={}&per_page={}&secret_type={}&state={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            format!("{}", page),
            format!("{}", per_page),
            secret_type.to_string(),
            state,
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Get a secret scanning alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}` endpoint.
     *
     * Gets a single secret scanning alert detected in a private repository. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
     *
     * GitHub Apps must have the `secret_scanning_alerts` read permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/secret-scanning#get-a-secret-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     */
    pub async fn get_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
    ) -> Result<crate::types::SecretScanningAlert> {
        let url = format!(
            "/repos/{}/{}/secret-scanning/alerts/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&alert_number.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Update a secret scanning alert.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}` endpoint.
     *
     * Updates the status of a secret scanning alert in a private repository. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
     *
     * GitHub Apps must have the `secret_scanning_alerts` write permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/secret-scanning#update-a-secret-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     */
    pub async fn update_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        body: &crate::types::SecretScanningUpdateAlertRequest,
    ) -> Result<crate::types::SecretScanningAlert> {
        let url = format!(
            "/repos/{}/{}/secret-scanning/alerts/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&alert_number.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}

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

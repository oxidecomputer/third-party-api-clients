use crate::Client;
use crate::ClientResult;

pub struct SecretScanning {
    pub client: Client,
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
     * * `state: crate::types::SecretScanningAlertState` -- Sets the state of the secret scanning alert. Can be either `open` or `resolved`. You must provide `resolution` when you set the state to `resolved`.
     * * `secret_type: &str` -- A comma separated list of secret types to return. By default all secret types are returned. See "[About secret scanning for private repositories](https://docs.github.com/code-security/secret-security/about-secret-scanning#about-secret-scanning-for-private-repositories)" for a complete list of secret types (API slug).
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
    ) -> ClientResult<crate::Response<Vec<crate::types::SecretScanningAlert>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !secret_type.is_empty() {
            query_args.push(("secret_type".to_string(), secret_type.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/alerts?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List secret scanning alerts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/secret-scanning/alerts` endpoint.
     *
     * As opposed to `list_alerts_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists all secret scanning alerts for a private repository, from newest to oldest. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
     *
     * GitHub Apps must have the `secret_scanning_alerts` read permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/secret-scanning#list-secret-scanning-alerts-for-a-repository>
     */
    pub async fn list_all_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::SecretScanningAlertState,
        secret_type: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SecretScanningAlert>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !secret_type.is_empty() {
            query_args.push(("secret_type".to_string(), secret_type.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/alerts?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                query_
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
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
    ) -> ClientResult<crate::Response<crate::types::SecretScanningAlert>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/alerts/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
    ) -> ClientResult<crate::Response<crate::types::SecretScanningAlert>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/alerts/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
}

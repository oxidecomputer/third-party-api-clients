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
     * List secret scanning alerts for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/secret-scanning/alerts` endpoint.
     *
     * Lists secret scanning alerts for eligible repositories in an organization, from newest to oldest.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `state: crate::types::SecretScanningAlertState` -- Sets the state of the secret scanning alert. You must provide `resolution` when you set the state to `resolved`.
     * * `secret_type: &str` -- A comma-separated list of secret types to return. All default secret patterns are returned. To return generic patterns, pass the token name(s) in the parameter. See "[Supported secret scanning patterns](https://docs.github.com/code-security/secret-scanning/introduction/supported-secret-scanning-patterns#supported-secrets)" for a complete list of secret types.
     * * `resolution: &str` -- A comma-separated list of resolutions. Only secret scanning alerts with one of these resolutions are listed. Valid resolutions are `false_positive`, `wont_fix`, `revoked`, `pattern_edited`, `pattern_deleted` or `used_in_tests`.
     * * `assignee: &str` -- Filters alerts by assignee. Use `*` to get all assigned alerts, `none` to get all unassigned alerts, or a GitHub username to get alerts assigned to a specific user.
     * * `sort: crate::types::SortData` -- The property to sort the results by. `created` means when the alert was created. `updated` means when the alert was updated or resolved.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events before this cursor. To receive an initial cursor on your first request, include an empty "before" query string.
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events after this cursor.  To receive an initial cursor on your first request, include an empty "after" query string.
     * * `validity: &str` -- A comma-separated list of validities that, when present, will return alerts that match the validities in this list. Valid options are `active`, `inactive`, and `unknown`.
     * * `is_publicly_leaked: bool` -- A boolean value representing whether or not to filter alerts by the publicly-leaked tag being present.
     * * `is_multi_repo: bool` -- A boolean value representing whether or not to filter alerts by the multi-repo tag being present.
     * * `hide_secret: bool` -- A boolean value representing whether or not to hide literal secrets in the results.
     */
    pub async fn list_alerts_for_org(
        &self,
        org: &str,
        state: crate::types::SecretScanningAlertState,
        secret_type: &str,
        resolution: &str,
        assignee: &str,
        sort: crate::types::SortData,
        direction: crate::types::Order,
        page: i64,
        per_page: i64,
        before: &str,
        after: &str,
        validity: &str,
        is_publicly_leaked: bool,
        is_multi_repo: bool,
        hide_secret: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSecretScanningAlert>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if hide_secret {
            query_args.push(("hide_secret".to_string(), hide_secret.to_string()));
        }
        if is_multi_repo {
            query_args.push(("is_multi_repo".to_string(), is_multi_repo.to_string()));
        }
        if is_publicly_leaked {
            query_args.push((
                "is_publicly_leaked".to_string(),
                is_publicly_leaked.to_string(),
            ));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !resolution.is_empty() {
            query_args.push(("resolution".to_string(), resolution.to_string()));
        }
        if !secret_type.is_empty() {
            query_args.push(("secret_type".to_string(), secret_type.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !validity.is_empty() {
            query_args.push(("validity".to_string(), validity.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/secret-scanning/alerts?{}",
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
     * List secret scanning alerts for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/secret-scanning/alerts` endpoint.
     *
     * As opposed to `list_alerts_for_org`, this function returns all the pages of the request at once.
     *
     * Lists secret scanning alerts for eligible repositories in an organization, from newest to oldest.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-an-organization>
     */
    pub async fn list_all_alerts_for_org(
        &self,
        org: &str,
        state: crate::types::SecretScanningAlertState,
        secret_type: &str,
        resolution: &str,
        assignee: &str,
        sort: crate::types::SortData,
        direction: crate::types::Order,
        before: &str,
        after: &str,
        validity: &str,
        is_publicly_leaked: bool,
        is_multi_repo: bool,
        hide_secret: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSecretScanningAlert>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if hide_secret {
            query_args.push(("hide_secret".to_string(), hide_secret.to_string()));
        }
        if is_multi_repo {
            query_args.push(("is_multi_repo".to_string(), is_multi_repo.to_string()));
        }
        if is_publicly_leaked {
            query_args.push((
                "is_publicly_leaked".to_string(),
                is_publicly_leaked.to_string(),
            ));
        }
        if !resolution.is_empty() {
            query_args.push(("resolution".to_string(), resolution.to_string()));
        }
        if !secret_type.is_empty() {
            query_args.push(("secret_type".to_string(), secret_type.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !validity.is_empty() {
            query_args.push(("validity".to_string(), validity.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/secret-scanning/alerts?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List organization pattern configurations.
     *
     * This function performs a `GET` to the `/orgs/{org}/secret-scanning/pattern-configurations` endpoint.
     *
     * Lists the secret scanning pattern configurations for an organization.
     *
     * Personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/push-protection#list-organization-pattern-configurations>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_org_pattern_configs(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::SecretScanningPatternConfiguration>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/secret-scanning/pattern-configurations",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Update organization pattern configurations.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/secret-scanning/pattern-configurations` endpoint.
     *
     * Updates the secret scanning pattern configurations for an organization.
     *
     * Personal access tokens (classic) need the `write:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/push-protection#update-organization-pattern-configurations>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn update_org_pattern_configs(
        &self,
        org: &str,
        body: &crate::types::SecretScanningUpdateOrgPatternConfigsRequest,
    ) -> ClientResult<crate::Response<crate::types::SecretScanningUpdateOrgPatternConfigsResponse>>
    {
        let url = self.client.url(
            &format!(
                "/orgs/{}/secret-scanning/pattern-configurations",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List secret scanning alerts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/secret-scanning/alerts` endpoint.
     *
     * Lists secret scanning alerts for an eligible repository, from newest to oldest.
     *
     * The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `state: crate::types::SecretScanningAlertState` -- Sets the state of the secret scanning alert. You must provide `resolution` when you set the state to `resolved`.
     * * `secret_type: &str` -- A comma-separated list of secret types to return. All default secret patterns are returned. To return generic patterns, pass the token name(s) in the parameter. See "[Supported secret scanning patterns](https://docs.github.com/code-security/secret-scanning/introduction/supported-secret-scanning-patterns#supported-secrets)" for a complete list of secret types.
     * * `resolution: &str` -- A comma-separated list of resolutions. Only secret scanning alerts with one of these resolutions are listed. Valid resolutions are `false_positive`, `wont_fix`, `revoked`, `pattern_edited`, `pattern_deleted` or `used_in_tests`.
     * * `assignee: &str` -- Filters alerts by assignee. Use `*` to get all assigned alerts, `none` to get all unassigned alerts, or a GitHub username to get alerts assigned to a specific user.
     * * `sort: crate::types::SortData` -- The property to sort the results by. `created` means when the alert was created. `updated` means when the alert was updated or resolved.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events before this cursor. To receive an initial cursor on your first request, include an empty "before" query string.
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events after this cursor.  To receive an initial cursor on your first request, include an empty "after" query string.
     * * `validity: &str` -- A comma-separated list of validities that, when present, will return alerts that match the validities in this list. Valid options are `active`, `inactive`, and `unknown`.
     * * `is_publicly_leaked: bool` -- A boolean value representing whether or not to filter alerts by the publicly-leaked tag being present.
     * * `is_multi_repo: bool` -- A boolean value representing whether or not to filter alerts by the multi-repo tag being present.
     * * `hide_secret: bool` -- A boolean value representing whether or not to hide literal secrets in the results.
     */
    pub async fn list_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::SecretScanningAlertState,
        secret_type: &str,
        resolution: &str,
        assignee: &str,
        sort: crate::types::SortData,
        direction: crate::types::Order,
        page: i64,
        per_page: i64,
        before: &str,
        after: &str,
        validity: &str,
        is_publicly_leaked: bool,
        is_multi_repo: bool,
        hide_secret: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::SecretScanningAlert>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if hide_secret {
            query_args.push(("hide_secret".to_string(), hide_secret.to_string()));
        }
        if is_multi_repo {
            query_args.push(("is_multi_repo".to_string(), is_multi_repo.to_string()));
        }
        if is_publicly_leaked {
            query_args.push((
                "is_publicly_leaked".to_string(),
                is_publicly_leaked.to_string(),
            ));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !resolution.is_empty() {
            query_args.push(("resolution".to_string(), resolution.to_string()));
        }
        if !secret_type.is_empty() {
            query_args.push(("secret_type".to_string(), secret_type.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !validity.is_empty() {
            query_args.push(("validity".to_string(), validity.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/alerts?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists secret scanning alerts for an eligible repository, from newest to oldest.
     *
     * The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#list-secret-scanning-alerts-for-a-repository>
     */
    pub async fn list_all_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::SecretScanningAlertState,
        secret_type: &str,
        resolution: &str,
        assignee: &str,
        sort: crate::types::SortData,
        direction: crate::types::Order,
        before: &str,
        after: &str,
        validity: &str,
        is_publicly_leaked: bool,
        is_multi_repo: bool,
        hide_secret: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::SecretScanningAlert>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if hide_secret {
            query_args.push(("hide_secret".to_string(), hide_secret.to_string()));
        }
        if is_multi_repo {
            query_args.push(("is_multi_repo".to_string(), is_multi_repo.to_string()));
        }
        if is_publicly_leaked {
            query_args.push((
                "is_publicly_leaked".to_string(),
                is_publicly_leaked.to_string(),
            ));
        }
        if !resolution.is_empty() {
            query_args.push(("resolution".to_string(), resolution.to_string()));
        }
        if !secret_type.is_empty() {
            query_args.push(("secret_type".to_string(), secret_type.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !validity.is_empty() {
            query_args.push(("validity".to_string(), validity.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/alerts?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Gets a single secret scanning alert detected in an eligible repository.
     *
     * The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#get-a-secret-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     * * `hide_secret: bool` -- A boolean value representing whether or not to hide literal secrets in the results.
     */
    pub async fn get_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        hide_secret: bool,
    ) -> ClientResult<crate::Response<crate::types::SecretScanningAlert>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if hide_secret {
            query_args.push(("hide_secret".to_string(), hide_secret.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/alerts/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * Update a secret scanning alert.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}` endpoint.
     *
     * Updates the status of a secret scanning alert in an eligible repository.
     *
     * You can also use this endpoint to assign or unassign an alert to a user who has write access to the repository.
     *
     * The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#update-a-secret-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
    /**
     * List locations for a secret scanning alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}/locations` endpoint.
     *
     * Lists all locations for a given secret scanning alert for an eligible repository.
     *
     * The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#list-locations-for-a-secret-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_locations_for_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SecretScanningLocation>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/alerts/{}/locations?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * List locations for a secret scanning alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}/locations` endpoint.
     *
     * As opposed to `list_locations_for_alert`, this function returns all the pages of the request at once.
     *
     * Lists all locations for a given secret scanning alert for an eligible repository.
     *
     * The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#list-locations-for-a-secret-scanning-alert>
     */
    pub async fn list_all_locations_for_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SecretScanningLocation>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/alerts/{}/locations",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * Create a push protection bypass.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/secret-scanning/push-protection-bypasses` endpoint.
     *
     * Creates a bypass for a previously push protected secret.
     *
     * The authenticated user must be the original author of the committed secret.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#create-a-push-protection-bypass>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_push_protection_bypass(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::SecretScanningCreatePushProtectionBypassRequest,
    ) -> ClientResult<crate::Response<crate::types::SecretScanningPushProtectionBypass>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/push-protection-bypasses",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get secret scanning scan history for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/secret-scanning/scan-history` endpoint.
     *
     * Lists the latest default incremental and backfill scans by type for a repository. Scans from Copilot Secret Scanning are not included.
     *
     * > [!NOTE]
     * > This endpoint requires [GitHub Advanced Security](https://docs.github.com/get-started/learning-about-github/about-github-advanced-security)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/secret-scanning/secret-scanning#get-secret-scanning-scan-history-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_scan_history(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/secret-scanning/scan-history",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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

use crate::Client;
use crate::ClientResult;

pub struct Copilot {
    pub client: Client,
}

impl Copilot {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Copilot { client }
    }

    /**
     * Get Copilot seat information and settings for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/copilot/billing` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Gets information about an organization's Copilot subscription, including seat breakdown
     * and feature policies. To configure these settings, go to your organization's settings on GitHub.com.
     * For more information, see "[Managing policies for Copilot in your organization](https://docs.github.com/copilot/managing-copilot/managing-policies-for-copilot-business-in-your-organization)."
     *
     * Only organization owners can view details about the organization's Copilot Business or Copilot Enterprise subscription.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot` or `read:org` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-user-management#get-copilot-seat-information-and-settings-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_copilot_organization_details(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::CopilotOrganizationDetails>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/billing",
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
     * List all Copilot seat assignments for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/copilot/billing/seats` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Lists all Copilot seats for which an organization with a Copilot Business or Copilot Enterprise subscription is currently being billed.
     * Only organization owners can view assigned seats.
     *
     * Each seat object contains information about the assigned user's most recent Copilot activity. Users must have telemetry enabled in their IDE for Copilot in the IDE activity to be reflected in `last_activity_at`.
     * For more information about activity data, see [Metrics data properties for GitHub Copilot](https://docs.github.com/copilot/reference/metrics-data).
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot` or `read:org` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-user-management#list-all-copilot-seat-assignments-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_copilot_seats(
        &self,
        org: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<crate::types::CopilotListSeatsResponse>> {
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
                "/orgs/{}/copilot/billing/seats?{}",
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
     * Add teams to the Copilot subscription for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/copilot/billing/selected_teams` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Purchases a GitHub Copilot seat for all users within each specified team.
     * The organization will be billed for each seat based on the organization's Copilot plan. For more information about Copilot pricing, see "[About billing for GitHub Copilot in your organization](https://docs.github.com/copilot/managing-copilot/managing-github-copilot-in-your-organization/managing-the-copilot-subscription-for-your-organization/about-billing-for-github-copilot-in-your-organization)."
     *
     * Only organization owners can purchase Copilot seats for their organization members. The organization must have a Copilot Business or Copilot Enterprise subscription and a configured suggestion matching policy.
     * For more information about setting up a Copilot subscription, see "[Subscribing to Copilot for your organization](https://docs.github.com/copilot/managing-copilot/managing-github-copilot-in-your-organization/managing-the-copilot-subscription-for-your-organization/subscribing-to-copilot-for-your-organization)."
     * For more information about setting a suggestion matching policy, see "[Managing policies for Copilot in your organization](https://docs.github.com/copilot/managing-copilot/managing-github-copilot-in-your-organization/setting-policies-for-copilot-in-your-organization/managing-policies-for-copilot-in-your-organization#policies-for-suggestion-matching)."
     *
     * The response contains the total number of new seats that were created and existing seats that were refreshed.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot` or `admin:org` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-user-management#add-teams-to-the-copilot-subscription-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn add_copilot_seats_for_teams(
        &self,
        org: &str,
        body: &crate::types::CopilotAddSeatsTeamsRequest,
    ) -> ClientResult<crate::Response<crate::types::CopilotAddSeatsTeamsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/billing/selected_teams",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Remove teams from the Copilot subscription for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/copilot/billing/selected_teams` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Sets seats for all members of each team specified to "pending cancellation".
     * This will cause the members of the specified team(s) to lose access to GitHub Copilot at the end of the current billing cycle unless they retain access through another team.
     * For more information about disabling access to Copilot, see "[Revoking access to Copilot for members of your organization](https://docs.github.com/copilot/managing-copilot/managing-github-copilot-in-your-organization/managing-access-to-github-copilot-in-your-organization/revoking-access-to-copilot-for-members-of-your-organization)."
     *
     * Only organization owners can cancel Copilot seats for their organization members.
     *
     * The response contains the total number of seats set to "pending cancellation".
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot` or `admin:org` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-user-management#remove-teams-from-the-copilot-subscription-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn cancel_copilot_seat_assignment_for_teams(
        &self,
        org: &str,
        body: &crate::types::CopilotAddSeatsTeamsRequest,
    ) -> ClientResult<crate::Response<crate::types::CopilotCancelSeatAssignmentTeamsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/billing/selected_teams",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Add users to the Copilot subscription for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/copilot/billing/selected_users` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Purchases a GitHub Copilot seat for each user specified.
     * The organization will be billed for each seat based on the organization's Copilot plan. For more information about Copilot pricing, see "[About billing for GitHub Copilot in your organization](https://docs.github.com/copilot/managing-copilot/managing-github-copilot-in-your-organization/managing-the-copilot-subscription-for-your-organization/about-billing-for-github-copilot-in-your-organization)."
     *
     * Only organization owners can purchase Copilot seats for their organization members. The organization must have a Copilot Business or Copilot Enterprise subscription and a configured suggestion matching policy.
     * For more information about setting up a Copilot subscription, see "[Subscribing to Copilot for your organization](https://docs.github.com/copilot/managing-copilot/managing-github-copilot-in-your-organization/managing-the-copilot-subscription-for-your-organization/subscribing-to-copilot-for-your-organization)."
     * For more information about setting a suggestion matching policy, see "[Managing policies for Copilot in your organization](https://docs.github.com/copilot/managing-copilot/managing-github-copilot-in-your-organization/setting-policies-for-copilot-in-your-organization/managing-policies-for-copilot-in-your-organization#policies-for-suggestion-matching)."
     *
     * The response contains the total number of new seats that were created and existing seats that were refreshed.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot` or `admin:org` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-user-management#add-users-to-the-copilot-subscription-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn add_copilot_seats_for_users(
        &self,
        org: &str,
        body: &crate::types::CopilotAddSeatsUsersRequest,
    ) -> ClientResult<crate::Response<crate::types::CopilotAddSeatsTeamsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/billing/selected_users",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Remove users from the Copilot subscription for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/copilot/billing/selected_users` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Sets seats for all users specified to "pending cancellation".
     * This will cause the specified users to lose access to GitHub Copilot at the end of the current billing cycle unless they retain access through team membership.
     * For more information about disabling access to Copilot, see "[Revoking access to Copilot for members of your organization](https://docs.github.com/copilot/managing-copilot/managing-github-copilot-in-your-organization/managing-access-to-github-copilot-in-your-organization/revoking-access-to-copilot-for-members-of-your-organization)."
     *
     * Only organization owners can cancel Copilot seats for their organization members.
     *
     * The response contains the total number of seats set to "pending cancellation".
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot` or `admin:org` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-user-management#remove-users-from-the-copilot-subscription-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn cancel_copilot_seat_assignment_for_users(
        &self,
        org: &str,
        body: &crate::types::CopilotAddSeatsUsersRequest,
    ) -> ClientResult<crate::Response<crate::types::CopilotCancelSeatAssignmentTeamsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/billing/selected_users",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get Copilot content exclusion rules for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/copilot/content_exclusion` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Gets information about an organization's Copilot content exclusion path rules.
     * To configure these settings, go to the organization's settings on GitHub.
     * For more information, see "[Excluding content from GitHub Copilot](https://docs.github.com/copilot/managing-copilot/configuring-and-auditing-content-exclusion/excluding-content-from-github-copilot#configuring-content-exclusions-for-your-organization)."
     *
     * Organization owners can view details about Copilot content exclusion rules for the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `copilot` or `read:org` scopes to use this endpoint.
     *
     * > [!CAUTION]
     * > * At this time, the API does not support comments. This endpoint will not return any comments in the existing rules.
     * > * At this time, the API does not support duplicate keys. If your content exclusion configuration contains duplicate keys, the API will return only the last occurrence of that key. For example, if duplicate entries are present, only the final value will be included in the response.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-content-exclusion-management#get-copilot-content-exclusion-rules-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn copilot_content_exclusion_for_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<String>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/content_exclusion",
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
     * Get Copilot content exclusion rules for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/copilot/content_exclusion` endpoint.
     *
     * As opposed to `copilot_content_exclusion_for_organization`, this function returns all the pages of the request at once.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Gets information about an organization's Copilot content exclusion path rules.
     * To configure these settings, go to the organization's settings on GitHub.
     * For more information, see "[Excluding content from GitHub Copilot](https://docs.github.com/copilot/managing-copilot/configuring-and-auditing-content-exclusion/excluding-content-from-github-copilot#configuring-content-exclusions-for-your-organization)."
     *
     * Organization owners can view details about Copilot content exclusion rules for the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `copilot` or `read:org` scopes to use this endpoint.
     *
     * > [!CAUTION]
     * > * At this time, the API does not support comments. This endpoint will not return any comments in the existing rules.
     * > * At this time, the API does not support duplicate keys. If your content exclusion configuration contains duplicate keys, the API will return only the last occurrence of that key. For example, if duplicate entries are present, only the final value will be included in the response.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-content-exclusion-management#get-copilot-content-exclusion-rules-for-an-organization>
     */
    pub async fn get_all_copilot_content_exclusion_for_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<String>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/content_exclusion",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Set Copilot content exclusion rules for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/copilot/content_exclusion` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Sets Copilot content exclusion path rules for an organization.
     * To configure these settings, go to the organization's settings on GitHub.
     * For more information, see "[Excluding content from GitHub Copilot](https://docs.github.com/copilot/managing-copilot/configuring-and-auditing-content-exclusion/excluding-content-from-github-copilot#configuring-content-exclusions-for-your-organization)."
     *
     * Organization owners can set Copilot content exclusion rules for the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `copilot` scope to use this endpoint.
     *
     * > [!CAUTION]
     * > * At this time, the API does not support comments. When using this endpoint, any existing comments in your rules will be deleted.
     * > * At this time, the API does not support duplicate keys. If you submit content exclusions through the API with duplicate keys, only the last occurrence will be saved. Earlier entries with the same key will be overwritten.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-content-exclusion-management#set-copilot-content-exclusion-rules-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_copilot_content_exclusion_for_organization(
        &self,
        org: &str,
        body: &[crate::types::CopilotSetContentExclusionOrganizationRequestAnyOf],
    ) -> ClientResult<crate::Response<crate::types::ReposCreateDeploymentResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/content_exclusion",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get Copilot metrics for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/copilot/metrics` endpoint.
     *
     * Use this endpoint to see a breakdown of aggregated metrics for various GitHub Copilot features. See the response schema tab for detailed metrics definitions.
     *
     * > [!NOTE]
     * > This endpoint will only return results for a given day if the organization contained **five or more members with active Copilot licenses** on that day, as evaluated at the end of that day.
     *
     * The response contains metrics for up to 100 days prior. Metrics are processed once per day for the previous day,
     * and the response will only include data up until yesterday. In order for an end user to be counted towards these metrics,
     * they must have telemetry enabled in their IDE.
     *
     * To access this endpoint, the Copilot Metrics API access policy must be enabled for the organization.
     * Only organization owners and owners and billing managers of the parent enterprise can view Copilot metrics.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot`, `read:org`, or `read:enterprise` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-metrics#get-copilot-metrics-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `since: &str` -- Show usage metrics since this date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:MM:SSZ`). Maximum value is 100 days ago.
     * * `until: &str` -- Show usage metrics until this date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:MM:SSZ`) and should not preceed the `since` date if it is passed.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of days of metrics to display per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn copilot_metrics_for_organization(
        &self,
        org: &str,
        since: &str,
        until: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CopilotUsageMetrics>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !since.is_empty() {
            query_args.push(("since".to_string(), since.to_string()));
        }
        if !until.is_empty() {
            query_args.push(("until".to_string(), until.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/metrics?{}",
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
     * Get Copilot metrics for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/copilot/metrics` endpoint.
     *
     * As opposed to `copilot_metrics_for_organization`, this function returns all the pages of the request at once.
     *
     * Use this endpoint to see a breakdown of aggregated metrics for various GitHub Copilot features. See the response schema tab for detailed metrics definitions.
     *
     * > [!NOTE]
     * > This endpoint will only return results for a given day if the organization contained **five or more members with active Copilot licenses** on that day, as evaluated at the end of that day.
     *
     * The response contains metrics for up to 100 days prior. Metrics are processed once per day for the previous day,
     * and the response will only include data up until yesterday. In order for an end user to be counted towards these metrics,
     * they must have telemetry enabled in their IDE.
     *
     * To access this endpoint, the Copilot Metrics API access policy must be enabled for the organization.
     * Only organization owners and owners and billing managers of the parent enterprise can view Copilot metrics.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot`, `read:org`, or `read:enterprise` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-metrics#get-copilot-metrics-for-an-organization>
     */
    pub async fn get_all_copilot_metrics_for_organization(
        &self,
        org: &str,
        since: &str,
        until: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CopilotUsageMetrics>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since.is_empty() {
            query_args.push(("since".to_string(), since.to_string()));
        }
        if !until.is_empty() {
            query_args.push(("until".to_string(), until.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/copilot/metrics?{}",
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
     * Get Copilot seat assignment details for a user.
     *
     * This function performs a `GET` to the `/orgs/{org}/members/{username}/copilot` endpoint.
     *
     * > [!NOTE]
     * > This endpoint is in public preview and is subject to change.
     *
     * Gets the GitHub Copilot seat details for a member of an organization who currently has access to GitHub Copilot.
     *
     * The seat object contains information about the user's most recent Copilot activity. Users must have telemetry enabled in their IDE for Copilot in the IDE activity to be reflected in `last_activity_at`.
     * For more information about activity data, see [Metrics data properties for GitHub Copilot](https://docs.github.com/copilot/reference/metrics-data).
     *
     * Only organization owners can view Copilot seat assignment details for members of their organization.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot` or `read:org` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-user-management#get-copilot-seat-assignment-details-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_copilot_seat_details_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::CopilotSeatDetails>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/members/{}/copilot",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Get Copilot metrics for a team.
     *
     * This function performs a `GET` to the `/orgs/{org}/team/{team_slug}/copilot/metrics` endpoint.
     *
     * Use this endpoint to see a breakdown of aggregated metrics for various GitHub Copilot features. See the response schema tab for detailed metrics definitions.
     *
     * > [!NOTE]
     * > This endpoint will only return results for a given day if the team had **five or more members with active Copilot licenses** on that day, as evaluated at the end of that day.
     *
     * The response contains metrics for up to 100 days prior. Metrics are processed once per day for the previous day,
     * and the response will only include data up until yesterday. In order for an end user to be counted towards these metrics,
     * they must have telemetry enabled in their IDE.
     *
     * To access this endpoint, the Copilot Metrics API access policy must be enabled for the organization containing the team within GitHub settings.
     * Only organization owners for the organization that contains this team and owners and billing managers of the parent enterprise can view Copilot metrics for a team.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot`, `read:org`, or `read:enterprise` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-metrics#get-copilot-metrics-for-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `since: &str` -- Show usage metrics since this date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:MM:SSZ`). Maximum value is 100 days ago.
     * * `until: &str` -- Show usage metrics until this date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:MM:SSZ`) and should not preceed the `since` date if it is passed.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of days of metrics to display per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn copilot_metrics_for_team(
        &self,
        org: &str,
        team_slug: &str,
        since: &str,
        until: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CopilotUsageMetrics>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !since.is_empty() {
            query_args.push(("since".to_string(), since.to_string()));
        }
        if !until.is_empty() {
            query_args.push(("until".to_string(), until.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/team/{}/copilot/metrics?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * Get Copilot metrics for a team.
     *
     * This function performs a `GET` to the `/orgs/{org}/team/{team_slug}/copilot/metrics` endpoint.
     *
     * As opposed to `copilot_metrics_for_team`, this function returns all the pages of the request at once.
     *
     * Use this endpoint to see a breakdown of aggregated metrics for various GitHub Copilot features. See the response schema tab for detailed metrics definitions.
     *
     * > [!NOTE]
     * > This endpoint will only return results for a given day if the team had **five or more members with active Copilot licenses** on that day, as evaluated at the end of that day.
     *
     * The response contains metrics for up to 100 days prior. Metrics are processed once per day for the previous day,
     * and the response will only include data up until yesterday. In order for an end user to be counted towards these metrics,
     * they must have telemetry enabled in their IDE.
     *
     * To access this endpoint, the Copilot Metrics API access policy must be enabled for the organization containing the team within GitHub settings.
     * Only organization owners for the organization that contains this team and owners and billing managers of the parent enterprise can view Copilot metrics for a team.
     *
     * OAuth app tokens and personal access tokens (classic) need either the `manage_billing:copilot`, `read:org`, or `read:enterprise` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/copilot/copilot-metrics#get-copilot-metrics-for-a-team>
     */
    pub async fn get_all_copilot_metrics_for_team(
        &self,
        org: &str,
        team_slug: &str,
        since: &str,
        until: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CopilotUsageMetrics>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since.is_empty() {
            query_args.push(("since".to_string(), since.to_string()));
        }
        if !until.is_empty() {
            query_args.push(("until".to_string(), until.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/team/{}/copilot/metrics?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
}

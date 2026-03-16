use crate::Client;
use crate::ClientResult;

pub struct Campaigns {
    pub client: Client,
}

impl Campaigns {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Campaigns { client }
    }

    /**
     * List campaigns for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/campaigns` endpoint.
     *
     * Lists campaigns in an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/campaigns/campaigns#list-campaigns-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `state: crate::types::CampaignState` -- If specified, only campaigns with this state will be returned.
     * * `sort: crate::types::CampaignsListOrgSort` -- The property by which to sort the results.
     */
    pub async fn list_org_campaigns(
        &self,
        org: &str,
        page: i64,
        per_page: i64,
        direction: crate::types::Order,
        state: crate::types::CampaignState,
        sort: crate::types::CampaignsListOrgSort,
    ) -> ClientResult<crate::Response<Vec<crate::types::CampaignSummary>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/campaigns?{}",
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
     * List campaigns for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/campaigns` endpoint.
     *
     * As opposed to `list_org_campaigns`, this function returns all the pages of the request at once.
     *
     * Lists campaigns in an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/campaigns/campaigns#list-campaigns-for-an-organization>
     */
    pub async fn list_all_org_campaigns(
        &self,
        org: &str,
        direction: crate::types::Order,
        state: crate::types::CampaignState,
        sort: crate::types::CampaignsListOrgSort,
    ) -> ClientResult<crate::Response<Vec<crate::types::CampaignSummary>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/campaigns?{}",
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
     * Create a campaign for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/campaigns` endpoint.
     *
     * Create a campaign for an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
     *
     * Fine-grained tokens must have the "Code scanning alerts" repository permissions (read) on all repositories included
     * in the campaign.
     *
     * FROM: <https://docs.github.com/rest/campaigns/campaigns#create-a-campaign-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_campaign(
        &self,
        org: &str,
        body: &crate::types::CampaignsCreateCampaignRequest,
    ) -> ClientResult<crate::Response<crate::types::CampaignSummary>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/campaigns",
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
     * Get a campaign for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/campaigns/{campaign_number}` endpoint.
     *
     * Gets a campaign for an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/campaigns/campaigns#get-a-campaign-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `campaign_number: i64` -- The campaign number.
     */
    pub async fn get_campaign_summary(
        &self,
        org: &str,
        campaign_number: i64,
    ) -> ClientResult<crate::Response<crate::types::CampaignSummary>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/campaigns/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&campaign_number.to_string()),
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
     * Delete a campaign for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/campaigns/{campaign_number}` endpoint.
     *
     * Deletes a campaign in an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/campaigns/campaigns#delete-a-campaign-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `campaign_number: i64` -- The campaign number.
     */
    pub async fn delete_campaign(
        &self,
        org: &str,
        campaign_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/campaigns/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&campaign_number.to_string()),
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
     * Update a campaign.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/campaigns/{campaign_number}` endpoint.
     *
     * Updates a campaign in an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/campaigns/campaigns#update-a-campaign>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `campaign_number: i64` -- The campaign number.
     */
    pub async fn update_campaign(
        &self,
        org: &str,
        campaign_number: i64,
        body: &crate::types::CampaignsUpdateCampaignRequest,
    ) -> ClientResult<crate::Response<crate::types::CampaignSummary>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/campaigns/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&campaign_number.to_string()),
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

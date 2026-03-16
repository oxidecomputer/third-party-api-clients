use crate::Client;
use crate::ClientResult;

pub struct HostedCompute {
    pub client: Client,
}

impl HostedCompute {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        HostedCompute { client }
    }

    /**
     * List hosted compute network configurations for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/settings/network-configurations` endpoint.
     *
     * Lists all hosted compute network configurations configured in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:network_configurations` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/network-configurations#list-hosted-compute-network-configurations-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_network_configurations_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<
        crate::Response<crate::types::HostedComputeListNetworkConfigurationsOrgResponse>,
    > {
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
                "/orgs/{}/settings/network-configurations?{}",
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
     * Create a hosted compute network configuration for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/settings/network-configurations` endpoint.
     *
     * Creates a hosted compute network configuration for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:network_configurations` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/network-configurations#create-a-hosted-compute-network-configuration-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_network_configuration_for_org(
        &self,
        org: &str,
        body: &crate::types::HostedComputeCreateNetworkConfigurationOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::NetworkConfiguration>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/network-configurations",
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
     * Get a hosted compute network configuration for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/settings/network-configurations/{network_configuration_id}` endpoint.
     *
     * Gets a hosted compute network configuration configured in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:network_configurations` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/network-configurations#get-a-hosted-compute-network-configuration-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `network_configuration_id: &str` -- Unique identifier of the hosted compute network configuration.
     */
    pub async fn get_network_configuration_for_org(
        &self,
        org: &str,
        network_configuration_id: &str,
    ) -> ClientResult<crate::Response<crate::types::NetworkConfiguration>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/network-configurations/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&network_configuration_id.to_string()),
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
     * Delete a hosted compute network configuration from an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/settings/network-configurations/{network_configuration_id}` endpoint.
     *
     * Deletes a hosted compute network configuration from an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:network_configurations` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/network-configurations#delete-a-hosted-compute-network-configuration-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `network_configuration_id: &str` -- Unique identifier of the hosted compute network configuration.
     */
    pub async fn delete_network_configuration_from_org(
        &self,
        org: &str,
        network_configuration_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/network-configurations/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&network_configuration_id.to_string()),
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
     * Update a hosted compute network configuration for an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/settings/network-configurations/{network_configuration_id}` endpoint.
     *
     * Updates a hosted compute network configuration for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:network_configurations` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/network-configurations#update-a-hosted-compute-network-configuration-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `network_configuration_id: &str` -- Unique identifier of the hosted compute network configuration.
     */
    pub async fn update_network_configuration_for_org(
        &self,
        org: &str,
        network_configuration_id: &str,
        body: &crate::types::HostedComputeUpdateNetworkConfigurationOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::NetworkConfiguration>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/network-configurations/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&network_configuration_id.to_string()),
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
     * Get a hosted compute network settings resource for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/settings/network-settings/{network_settings_id}` endpoint.
     *
     * Gets a hosted compute network settings resource configured for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:network_configurations` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/network-configurations#get-a-hosted-compute-network-settings-resource-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `network_settings_id: &str` -- Unique identifier of the hosted compute network settings.
     */
    pub async fn get_network_settings_for_org(
        &self,
        org: &str,
        network_settings_id: &str,
    ) -> ClientResult<crate::Response<crate::types::NetworkSettings>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/network-settings/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&network_settings_id.to_string()),
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

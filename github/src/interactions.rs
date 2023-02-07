use anyhow::Result;

use crate::Client;

pub struct Interactions {
    pub client: Client,
}

impl Interactions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Interactions { client }
    }

    /**
    * Get interaction restrictions for an organization.
    *
    * This function performs a `GET` to the `/orgs/{org}/interaction-limits` endpoint.
    *
    * Shows which type of GitHub user can interact with this organization and when the restriction expires. If there is no restrictions, you will see an empty response.
    *
    * FROM: <https://docs.github.com/rest/reference/interactions#get-interaction-restrictions-for-an-organization>
    *
    * **Parameters:**
    *
    * * `org: &str`
    */
    pub async fn get_restrictions_for_org(
        &self,
        org: &str,
    ) -> Result<crate::types::InteractionsGetRestrictionsResponseAnyOf> {
        let url = format!(
            "/orgs/{}/interaction-limits",
            crate::progenitor_support::encode_path(&org.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Set interaction restrictions for an organization.
    *
    * This function performs a `PUT` to the `/orgs/{org}/interaction-limits` endpoint.
    *
    * Temporarily restricts interactions to a certain type of GitHub user in any public repository in the given organization. You must be an organization owner to set these restrictions. Setting the interaction limit at the organization level will overwrite any interaction limits that are set for individual repositories owned by the organization.
    *
    * FROM: <https://docs.github.com/rest/reference/interactions#set-interaction-restrictions-for-an-organization>
    *
    * **Parameters:**
    *
    * * `org: &str`
    */
    pub async fn set_restrictions_for_org(
        &self,
        org: &str,
        body: &crate::types::InteractionLimit,
    ) -> Result<crate::types::InteractionLimits> {
        let url = format!(
            "/orgs/{}/interaction-limits",
            crate::progenitor_support::encode_path(&org.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Remove interaction restrictions for an organization.
    *
    * This function performs a `DELETE` to the `/orgs/{org}/interaction-limits` endpoint.
    *
    * Removes all interaction restrictions from public repositories in the given organization. You must be an organization owner to remove restrictions.
    *
    * FROM: <https://docs.github.com/rest/reference/interactions#remove-interaction-restrictions-for-an-organization>
    *
    * **Parameters:**
    *
    * * `org: &str`
    */
    pub async fn remove_restrictions_for_org(&self, org: &str) -> Result<()> {
        let url = format!(
            "/orgs/{}/interaction-limits",
            crate::progenitor_support::encode_path(&org.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Get interaction restrictions for a repository.
    *
    * This function performs a `GET` to the `/repos/{owner}/{repo}/interaction-limits` endpoint.
    *
    * Shows which type of GitHub user can interact with this repository and when the restriction expires. If there are no restrictions, you will see an empty response.
    *
    * FROM: <https://docs.github.com/rest/reference/interactions#get-interaction-restrictions-for-a-repository>
    *
    * **Parameters:**
    *
    * * `owner: &str`
    * * `repo: &str`
    */
    pub async fn get_restrictions_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<crate::types::InteractionsGetRestrictionsResponseAnyOf> {
        let url = format!(
            "/repos/{}/{}/interaction-limits",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Set interaction restrictions for a repository.
    *
    * This function performs a `PUT` to the `/repos/{owner}/{repo}/interaction-limits` endpoint.
    *
    * Temporarily restricts interactions to a certain type of GitHub user within the given repository. You must have owner or admin access to set these restrictions. If an interaction limit is set for the user or organization that owns this repository, you will receive a `409 Conflict` response and will not be able to use this endpoint to change the interaction limit for a single repository.
    *
    * FROM: <https://docs.github.com/rest/reference/interactions#set-interaction-restrictions-for-a-repository>
    *
    * **Parameters:**
    *
    * * `owner: &str`
    * * `repo: &str`
    */
    pub async fn set_restrictions_for_repo(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::InteractionLimit,
    ) -> Result<crate::types::InteractionLimits> {
        let url = format!(
            "/repos/{}/{}/interaction-limits",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Remove interaction restrictions for a repository.
    *
    * This function performs a `DELETE` to the `/repos/{owner}/{repo}/interaction-limits` endpoint.
    *
    * Removes all interaction restrictions from the given repository. You must have owner or admin access to remove restrictions. If the interaction limit is set for the user or organization that owns this repository, you will receive a `409 Conflict` response and will not be able to use this endpoint to change the interaction limit for a single repository.
    *
    * FROM: <https://docs.github.com/rest/reference/interactions#remove-interaction-restrictions-for-a-repository>
    *
    * **Parameters:**
    *
    * * `owner: &str`
    * * `repo: &str`
    */
    pub async fn remove_restrictions_for_repo(&self, owner: &str, repo: &str) -> Result<()> {
        let url = format!(
            "/repos/{}/{}/interaction-limits",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Get interaction restrictions for your public repositories.
    *
    * This function performs a `GET` to the `/user/interaction-limits` endpoint.
    *
    * Shows which type of GitHub user can interact with your public repositories and when the restriction expires.
    *
    * FROM: <https://docs.github.com/rest/reference/interactions#get-interaction-restrictions-for-your-public-repositories>
    */
    pub async fn get_restrictions_for_authenticated_user(
        &self,
    ) -> Result<crate::types::InteractionsGetRestrictionsResponseAnyOf> {
        let url = "/user/interaction-limits".to_string();
        self.client.get(&url, None).await
    }

    /**
    * Set interaction restrictions for your public repositories.
    *
    * This function performs a `PUT` to the `/user/interaction-limits` endpoint.
    *
    * Temporarily restricts which type of GitHub user can interact with your public repositories. Setting the interaction limit at the user level will overwrite any interaction limits that are set for individual repositories owned by the user.
    *
    * FROM: <https://docs.github.com/rest/reference/interactions#set-interaction-restrictions-for-your-public-repositories>
    */
    pub async fn set_restrictions_for_authenticated_user(
        &self,
        body: &crate::types::InteractionLimit,
    ) -> Result<crate::types::InteractionLimits> {
        let url = "/user/interaction-limits".to_string();
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Remove interaction restrictions from your public repositories.
    *
    * This function performs a `DELETE` to the `/user/interaction-limits` endpoint.
    *
    * Removes any interaction restrictions from your public repositories.
    *
    * FROM: <https://docs.github.com/rest/reference/interactions#remove-interaction-restrictions-from-your-public-repositories>
    */
    pub async fn remove_restrictions_for_authenticated_user(&self) -> Result<()> {
        let url = "/user/interaction-limits".to_string();
        self.client.delete(&url, None).await
    }
}

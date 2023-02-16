use anyhow::Result;

use crate::Client;

pub struct SingleSignOnTeammates {
    pub client: Client,
}

impl SingleSignOnTeammates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SingleSignOnTeammates { client }
    }

    /**
     * Create SSO Teammate.
     *
     * This function performs a `POST` to the `/sso/teammates` endpoint.
     *
     * **This endpoint allows you to create an SSO Teammate.**
     *
     * The email provided for this user will also function as the Teammate’s username.
     */
    pub async fn post_sso_teammate(
        &self,
        body: &crate::types::SsoTeammateRequestAllOf,
    ) -> Result<crate::types::SsoTeammateResponseAllOf> {
        let url = "/sso/teammates".to_string();
        let url = self.client.url(&url, None);
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                Some("application/json"),
            )
            .await
    }
    /**
     * Edit an SSO Teammate.
     *
     * This function performs a `PATCH` to the `/sso/teammates/{username}` endpoint.
     *
     * **This endpoint allows you to modify an existing SSO Teammate.**
     *
     * To turn a teammate into an admin, the request body should contain the `is_admin` field set to `true`. Otherwise, set `is_admin` to false and pass in all the scopes that a teammate should have.
     *
     * Only the parent user and Teammates with admin permissions can update another Teammate’s permissions. Admin users can only update permissions.
     */
    pub async fn patch_sso_teammates_username(
        &self,
        username: &str,
        body: &crate::types::PatchSsoTeammatesUsernameRequest,
    ) -> Result<crate::types::SsoTeammatesPatchResponseAllOf> {
        let url = format!(
            "/sso/teammates/{}",
            crate::progenitor_support::encode_path(username),
        );
        let url = self.client.url(&url, None);
        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                Some("application/json"),
            )
            .await
    }
}

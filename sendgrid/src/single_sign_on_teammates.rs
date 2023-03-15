use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<crate::types::SsoTeammateResponseAllOf> {
        let url = self.client.url("/sso/teammates", None);
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
    ) -> ClientResult<crate::types::SsoTeammatesPatchResponseAllOf> {
        let url = self.client.url(
            &format!(
                "/sso/teammates/{}",
                crate::progenitor_support::encode_path(username),
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

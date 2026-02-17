use crate::Client;
use crate::ClientResult;

pub struct Credentials {
    pub client: Client,
}

impl Credentials {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Credentials { client }
    }

    /**
     * Revoke a list of credentials.
     *
     * This function performs a `POST` to the `/credentials/revoke` endpoint.
     *
     * Submit a list of credentials to be revoked. This endpoint is intended to revoke credentials the caller does not own and may have found exposed on GitHub.com or elsewhere. It can also be used for credentials associated with an old user account that you no longer have access to. Credential owners will be notified of the revocation.
     *
     * This endpoint currently accepts the following credential types:
     * - Personal access tokens (classic)
     * - Fine-grained personal access tokens
     *
     * Revoked credentials may impact users on GitHub Free, Pro, & Team and GitHub Enterprise Cloud, and GitHub Enterprise Cloud with Enterprise Managed Users.
     * GitHub cannot reactivate any credentials that have been revoked; new credentials will need to be generated.
     *
     * To prevent abuse, this API is limited to only 60 unauthenticated requests per hour and a max of 1000 tokens per API request.
     *
     * > [!NOTE]
     * > Any authenticated requests will return a 403.
     *
     * FROM: <https://docs.github.com/rest/credentials/revoke#revoke-a-list-of-credentials>
     */
    pub async fn revoke(
        &self,
        body: &crate::types::CredentialsRevokeRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(&"/credentials/revoke".to_string(), None);
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
}

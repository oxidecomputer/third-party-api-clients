use anyhow::Result;

use crate::Client;

pub struct Actions {
    client: Client,
}

impl Actions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Actions { client }
    }

    /**
     * Delete an environment secret.
     *
     * This function performs a `DELETE` to the `/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}` endpoint.
     *
     * Deletes a secret in an environment using the secret name. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-an-environment-secret>
     *
     * **Parameters:**
     *
     * * `repository_id: i64`
     * * `environment_name: &str` -- The name of the environment.
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn delete_environment_secret(
        &self,
        repository_id: i64,
        environment_name: &str,
        secret_name: &str,
    ) -> Result<()> {
        let url = format!(
            "/repositories/{}/environments/{}/secrets/{}",
            crate::progenitor_support::encode_path(&repository_id.to_string()),
            crate::progenitor_support::encode_path(&environment_name.to_string()),
            crate::progenitor_support::encode_path(&secret_name.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

use crate::Client;
use crate::ClientResult;

pub struct Asps {
    pub client: Client,
}

impl Asps {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Asps { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/asps` endpoint.
     *
     * Lists the ASPs issued by a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn list(&self, user_key: &str) -> ClientResult<crate::types::Asps> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/asps",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/asps/{codeId}` endpoint.
     *
     * Gets information about an ASP issued by a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `code_id: i64` -- The unique ID of the ASP.
     */
    pub async fn get(&self, user_key: &str, code_id: i64) -> ClientResult<crate::types::Asp> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/asps/{}",
                crate::progenitor_support::encode_path(user_key),
                crate::progenitor_support::encode_path(&code_id.to_string()),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/users/{userKey}/asps/{codeId}` endpoint.
     *
     * Deletes an ASP issued by a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `code_id: i64` -- The unique ID of the ASP to be deleted.
     */
    pub async fn delete(&self, user_key: &str, code_id: i64) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/asps/{}",
                crate::progenitor_support::encode_path(user_key),
                crate::progenitor_support::encode_path(&code_id.to_string()),
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
}

use crate::Client;
use crate::ClientResult;

pub struct ApiKeys {
    pub client: Client,
}

impl ApiKeys {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ApiKeys { client }
    }

    /**
     * Retrieve all API Keys belonging to the authenticated user.
     *
     * This function performs a `GET` to the `/api_keys` endpoint.
     *
     * **This endpoint allows you to retrieve all API Keys that belong to the authenticated user.**
     *
     * A successful response from this API will include all available API keys' names and IDs.
     *
     * For security reasons, there is not a way to retrieve the key itself after it's created. If you lose your API key, you must create a new one. Only the "Create API keys" endpoint will return a key to you and only at the time of creation.
     *
     * An `api_key_id` can be used to update or delete the key, as well as retrieve the key's details, such as its scopes.
     *
     * **Parameters:**
     *
     * * `limit: i64`
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get(&self, limit: i64) -> ClientResult<crate::types::GetApiKeysResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api_keys?{}", query_), None);
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
     * Create API keys.
     *
     * This function performs a `POST` to the `/api_keys` endpoint.
     *
     * **This endpoint allows you to create a new API Key for the user.**
     *
     * To create your initial SendGrid API Key, you should [use the SendGrid App](https://app.sendgrid.com/settings/api_keys). Once you have created a first key with scopes to manage additional API keys, you can use this API for all other key management.
     *
     * > There is a limit of 100 API Keys on your account.
     *
     * A JSON request body containing a `name` property is required when making requests to this endpoint. If the number of maximum keys, 100, is reached, a `403` status will be returned.
     *
     * Though the `name` field is required, it does not need to be unique. A unique API key ID will be generated for each key you create and returned in the response body.
     *
     * It is not necessary to pass a `scopes` field to the API when creating a key, but you should be aware that omitting the `scopes` field from your request will create a key with "Full Access" permissions by default.
     *
     * See the [API Key Permissions List](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization) for all available scopes. An API key's scopes can be updated after creation using the "Update API keys" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn create(
        &self,
        body: &crate::types::CreateApiKeysRequest,
    ) -> ClientResult<crate::types::CreateApiKeysResponse> {
        let url = self.client.url("/api_keys", None);
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
     * Retrieve an existing API Key.
     *
     * This function performs a `GET` to the `/api_keys/{api_key_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a single API key using an `api_key_id`.**
     *
     * The endpoint will return a key's name, ID, and scopes. If the API Key ID does not, exist a `404` status will be returned.
     *
     * See the [API Key Permissions List](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization) for all available scopes. An API key's scopes can be updated after creation using the "Update API keys" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_key(
        &self,
        api_key_id: &str,
    ) -> ClientResult<crate::types::GetApiKeysKeyResponse> {
        let url = self.client.url(
            &format!(
                "/api_keys/{}",
                crate::progenitor_support::encode_path(api_key_id),
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
     * Update API key name and scopes.
     *
     * This function performs a `PUT` to the `/api_keys/{api_key_id}` endpoint.
     *
     * **This endpoint allows you to update the name and scopes of a given API key.**
     *
     * You must pass this endpoint a JSON request body with a `name` field and a `scopes` array containing at least one scope. The `name` and `scopes` fields will be used to update the key associated with the `api_key_id` in the request URL.
     *
     * If you need to update a key's scopes only, pass the `name` field with the key's existing name; the `name` will not be modified. If you need to update a key's name only, use the "Update API key name" endpoint.
     *
     * See the [API Key Permissions List](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization) for all available scopes.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn put_key(
        &self,
        api_key_id: &str,
        body: &crate::types::PutApiKeysKeyRequest,
    ) -> ClientResult<crate::types::ApiKeyNameScopesAllOf> {
        let url = self.client.url(
            &format!(
                "/api_keys/{}",
                crate::progenitor_support::encode_path(api_key_id),
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
     * Delete API keys.
     *
     * This function performs a `DELETE` to the `/api_keys/{api_key_id}` endpoint.
     *
     * **This endpoint allows you to revoke an existing API Key using an `api_key_id`**
     *
     * Authentications using a revoked API Key will fail after after some small propogation delay. If the API Key ID does not exist, a `404` status will be returned.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_key(&self, api_key_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api_keys/{}",
                crate::progenitor_support::encode_path(api_key_id),
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
     * Update API key name.
     *
     * This function performs a `PATCH` to the `/api_keys/{api_key_id}` endpoint.
     *
     * **This endpoint allows you to update the name of an existing API Key.**
     *
     * You must pass this endpoint a JSON request body with a `name` property, which will be used to rename the key associated with the `api_key_id` passed in the URL.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_key(
        &self,
        api_key_id: &str,
        body: &crate::types::IpPool,
    ) -> ClientResult<crate::types::ApiKeyNameId> {
        let url = self.client.url(
            &format!(
                "/api_keys/{}",
                crate::progenitor_support::encode_path(api_key_id),
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

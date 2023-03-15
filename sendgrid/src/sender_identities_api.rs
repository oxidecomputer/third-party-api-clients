use crate::Client;
use crate::ClientResult;

pub struct SenderIdentitiesApi {
    pub client: Client,
}

impl SenderIdentitiesApi {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SenderIdentitiesApi { client }
    }

    /**
     * Get all Sender Identities.
     *
     * This function performs a `GET` to the `/senders` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all sender identities that have been created for your account.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_senders(&self) -> ClientResult<crate::types::GetSendersResponse> {
        let url = self.client.url("/senders", None);
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
     * Create a Sender Identity.
     *
     * This function performs a `POST` to the `/senders` endpoint.
     *
     * **This endpoint allows you to create a new sender identity.**
     *
     * You may create up to 100 unique sender identities.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_sender(
        &self,
        body: &crate::types::PostSendersRequestAllOf,
    ) -> ClientResult<crate::types::SenderAllOf> {
        let url = self.client.url("/senders", None);
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
     * View a Sender Identity.
     *
     * This function performs a `GET` to the `/senders/{sender_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific sender identity.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_senders_sender(
        &self,
        sender_id: i64,
    ) -> ClientResult<crate::types::SenderAllOf> {
        let url = self.client.url(
            &format!(
                "/senders/{}",
                crate::progenitor_support::encode_path(&sender_id.to_string()),
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
     * Delete a Sender Identity.
     *
     * This function performs a `DELETE` to the `/senders/{sender_id}` endpoint.
     *
     * **This endoint allows you to delete one of your sender identities.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_senders_sender(&self, sender_id: i64) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/senders/{}",
                crate::progenitor_support::encode_path(&sender_id.to_string()),
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
     * Update a Sender Identity.
     *
     * This function performs a `PATCH` to the `/senders/{sender_id}` endpoint.
     *
     * **This endpoint allows you to update a sender identity.**
     *
     * Updates to `from.email` require re-verification.
     *
     * Partial updates are allowed, but fields that are marked as "required" in the POST (create) endpoint must not be nil if that field is included in the PATCH request.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_senders_sender(
        &self,
        sender_id: i64,
        body: &crate::types::SenderRequest,
    ) -> ClientResult<crate::types::SenderAllOf> {
        let url = self.client.url(
            &format!(
                "/senders/{}",
                crate::progenitor_support::encode_path(&sender_id.to_string()),
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
     * Resend Sender Identity Verification.
     *
     * This function performs a `POST` to the `/senders/{sender_id}/resend_verification` endpoint.
     *
     * **This enpdoint allows you to resend a sender identity verification email.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_senders_sender_resend_verification(
        &self,
        sender_id: i64,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/senders/{}/resend_verification",
                crate::progenitor_support::encode_path(&sender_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}

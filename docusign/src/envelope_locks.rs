use crate::Client;
use crate::ClientResult;

pub struct EnvelopeLocks {
    pub client: Client,
}

impl EnvelopeLocks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeLocks { client }
    }

    /**
     * Gets envelope lock information.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/lock` endpoint.
     *
     * Retrieves general information about an envelope lock.
     *
     * The user requesting the information must be the same user
     * who locked the envelope.
     *
     * You can use this method to recover the lock information,
     * including the `lockToken`,
     * for a locked envelope.
     * The `X-DocuSign-Edit` header is included in the response.
     *
     * See [EnvelopeLocks: create](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/envelopelocks/create/)
     * for a description of the `X-DocuSign-Edit` header.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn lock_get_envelope(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::EnvelopeLocks> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/lock",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Updates an envelope lock.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/lock` endpoint.
     *
     * Updates the lock information for a locked envelope.
     *
     * You must include the `X-DocuSign-Edit` header
     * as described in
     * [EnvelopeLocks: create](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/envelopelocks/create/).
     *
     *
     * Use this method to change the duration
     * of the lock (`lockDurationInSeconds`)
     * or the `lockedByApp` string.
     *
     * The request body is a full `lockRequest` object,
     * but you only need to specify the
     * properties that you are updating. For example:
     *
     * ```
     * {
     *   "lockDurationInSeconds": "3600",
     *   "lockedByApp": "My Application"
     * }
     * ```
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn lock_put_envelope(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::LockRequest,
    ) -> ClientResult<crate::types::EnvelopeLocks> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/lock",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Locks an envelope.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/lock` endpoint.
     *
     * This method locks the specified envelope and sets the time until
     * the lock expires to prevent other users or recipients from
     * changing the envelope.
     *
     * **Note**: To use this method, the envelope locking
     * capability must be enabled for the user; that is, the user setting
     * `canLockEnvelopes` must be set to **true**.
     *
     * The response to this request includes a `lockToken` parameter
     * that you must use in the `X-DocuSign-Edit` header for
     * every PUT method (typically a method that updates an envelope)
     * while the envelope is locked.
     *
     *
     * If you do not provide the `lockToken` when accessing
     * a locked envelope, you will get the following
     * error:
     *
     * ```
     * {
     *    "errorCode": "EDIT_LOCK_NOT_LOCK_OWNER",
     *    "message": "The user is not the owner of the lock. The template is locked by another user or in another application"
     * }
     * ```
     *
     *
     * ### The X-DocuSign-Edit header
     *
     * The `X-DocuSign-Edit` header looks like this
     * and can be specified in either JSON or XML.
     *
     * **JSON**
     * ```
     * {
     *   "LockToken": "token-from-response",
     *   "LockDurationInSeconds": "600"
     * }
     * ```
     *
     * **XML**
     * ```
     * <DocuSignEdit>
     *   <LockToken>token-from-response</LockToken>
     *   <LockDurationInSeconds>600</LockDurationInSeconds>
     * </DocuSignEdit>
     * ```
     *
     * In the actual HTTP header, you would remove the linebreaks:
     *
     * ```
     * X-DocuSign-Edit: {"LockToken": "token-from-response", "LockDurationInSeconds": "600" }
     *     or
     * X-DocuSign-Edit:<DocuSignEdit><LockToken>token-from-response</LockToken><LockDurationInSeconds>600</LockDurationInSeconds></DocuSignEdit>
     * ```
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn lock_post_envelope(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::LockRequest,
    ) -> ClientResult<crate::types::EnvelopeLocks> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/lock",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes an envelope lock.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/lock` endpoint.
     *
     * Deletes the lock from the specified envelope.
     * The user deleting the lock must be the same user
     * who locked the envelope.
     *
     * You must include the `X-DocuSign-Edit` header
     * as described in
     * [EnvelopeLocks: create](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/envelopelocks/create/).
     *
     * This method takes an optional query paramter
     * that lets you specify whether
     * changes made while the envelope was locked
     * are kept or discarded.
     *
     *
     * | Query Parameter | Description                                                                                                                                                                         |
     * | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
     * | `save_changes`  | (Optional) When set to **true** (the default), any changes made while the lock was active are saved. When set to **false**, any changes made while the envelope was locked are discarded. |
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn lock_delete_envelope(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::EnvelopeLocks> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/lock",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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

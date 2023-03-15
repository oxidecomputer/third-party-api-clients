use crate::Client;
use crate::ClientResult;

pub struct SenderVerification {
    pub client: Client,
}

impl SenderVerification {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SenderVerification { client }
    }

    /**
     * Domain Warn List.
     *
     * This function performs a `GET` to the `/verified_senders/domains` endpoint.
     *
     * **This endpoint returns a list of domains known to implement DMARC and categorizes them by failure type — hard failure or soft failure**.
     *
     * Domains listed as hard failures will not deliver mail when used as a [Sender Identity](https://sendgrid.com/docs/for-developers/sending-email/sender-identity/) due to the domain's DMARC policy settings.
     *
     * For example, using a `yahoo.com` email address as a Sender Identity will likely result in the rejection of your mail. For more information about DMARC, see [Everything about DMARC](https://sendgrid.com/docs/ui/sending-email/dmarc/).
     */
    pub async fn get_verified_senders_domains(
        &self,
    ) -> ClientResult<crate::types::GetVerifiedSendersDomainsResponse> {
        let url = self.client.url("/verified_senders/domains", None);
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
     * Completed Steps.
     *
     * This function performs a `GET` to the `/verified_senders/steps_completed` endpoint.
     *
     * **This endpoint allows you to determine which of SendGrid’s verification processes have been completed for an account**.
     *
     * This endpoint returns boolean values, `true` and `false`, for [Domain Authentication](https://sendgrid.com/docs/for-developers/sending-email/sender-identity/#domain-authentication), `domain_verified`, and [Single Sender Verification](https://sendgrid.com/docs/for-developers/sending-email/sender-identity/#single-sender-verification), `sender_verified`, for the account.
     *
     * An account may have one, both, or neither verification steps completed. If you need to authenticate a domain rather than a Single Sender, see the "Authenticate a domain" endpoint.
     */
    pub async fn get_verified_senders_steps_completed(
        &self,
    ) -> ClientResult<crate::types::GetVerifiedSendersStepsCompletedResponse> {
        let url = self.client.url("/verified_senders/steps_completed", None);
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
     * Get All Verified Senders.
     *
     * This function performs a `GET` to the `/verified_senders` endpoint.
     *
     * **This endpoint allows you to retrieve all the Sender Identities associated with an account.**
     *
     * This endpoint will return both verified and unverified senders.
     *
     * You can limit the number of results returned using the `limit`, `lastSeenID`, and `id` query string parameters.
     *
     * * `limit` allows you to specify an exact number of Sender Identities to return.
     * * `lastSeenID` will return senders with an ID number occuring after the passed in ID. In other words, the `lastSeenID` provides a starting point from which SendGrid will iterate to find Sender Identities associated with your account.
     * * `id` will return information about only the Sender Identity passed in the request.
     *
     * **Parameters:**
     *
     * * `limit: f64` -- The number of errors found while adding recipients.
     * * `last_seen_id: f64` -- The number of errors found while adding recipients.
     * * `id: i64`
     */
    pub async fn get_verified_senders(
        &self,
        limit: f64,
        last_seen_id: f64,
        id: i64,
    ) -> ClientResult<crate::types::GetVerifiedSendersResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if id > 0 {
            query_args.push(("id".to_string(), id.to_string()));
        }
        if !last_seen_id.to_string().is_empty() {
            query_args.push(("lastSeenID".to_string(), last_seen_id.to_string()));
        }
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/verified_senders?{}", query_), None);
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
     * Create Verified Sender Request.
     *
     * This function performs a `POST` to the `/verified_senders` endpoint.
     *
     * **This endpoint allows you to create a new Sender Identify**.
     *
     * Upon successful submission of a `POST` request to this endpoint, an identity will be created, and a verification email will be sent to the address assigned to the `from_email` field. You must complete the verification process using the sent email to fully verify the sender.
     *
     * If you need to resend the verification email, you can do so with the Resend Verified Sender Request, `/resend/{id}`, endpoint.
     *
     * If you need to authenticate a domain rather than a Single Sender, see the [Domain Authentication API](https://sendgrid.api-docs.io/v3.0/domain-authentication/authenticate-a-domain).
     */
    pub async fn post_verified_sender(
        &self,
        body: &crate::types::VerifiedSenderRequestSchema,
    ) -> ClientResult<crate::types::VerifiedSenderResponseSchema> {
        let url = self.client.url("/verified_senders", None);
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
     * Verify Sender Request.
     *
     * This function performs a `GET` to the `/verified_senders/verify/{token}` endpoint.
     *
     * **This endpoint allows you to verify a sender requests.**
     *
     * The token is generated by SendGrid and included in a verification email delivered to the address that's pending verification.
     */
    pub async fn get_verified_senders_verify_token(&self, token: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/verified_senders/verify/{}",
                crate::progenitor_support::encode_path(token),
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
     * Delete Verified Sender.
     *
     * This function performs a `DELETE` to the `/verified_senders/{id}` endpoint.
     *
     * **This endpoint allows you to delete a Sender Identity**.
     *
     * Pass the `id` assigned to a Sender Identity to this endpoint to delete the Sender Identity from your account.
     *
     * You can retrieve the IDs associated with Sender Identities using the "Get All Verified Senders" endpoint.
     */
    pub async fn delete_verified_senders(&self, id: &str) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/verified_senders/{}",
                crate::progenitor_support::encode_path(id),
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
     * Edit Verified Sender.
     *
     * This function performs a `PATCH` to the `/verified_senders/{id}` endpoint.
     *
     * **This endpoint allows you to update an existing Sender Identity**.
     *
     * Pass the `id` assigned to a Sender Identity to this endpoint as a path parameter. Include any fields you wish to update in the request body in JSON format.
     *
     * You can retrieve the IDs associated with Sender Identities by passing a `GET` request to the Get All Verified Senders endpoint, `/verified_senders`.
     *
     * **Note:** Unlike a `PUT` request, `PATCH` allows you to update only the fields you wish to edit. Fields that are not passed as part of a request will remain unaltered.
     */
    pub async fn patch_verified_senders(
        &self,
        id: &str,
        body: &crate::types::VerifiedSenderRequestSchema,
    ) -> ClientResult<crate::types::VerifiedSenderResponseSchema> {
        let url = self.client.url(
            &format!(
                "/verified_senders/{}",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Resend Verified Sender Request.
     *
     * This function performs a `POST` to the `/verified_senders/resend/{id}` endpoint.
     *
     * **This endpoint allows you to resend a verification email to a specified Sender Identity**.
     *
     * Passing the `id` assigned to a Sender Identity to this endpoint will resend a verification email to the `from_address` associated with the Sender Identity. This can be useful if someone loses their verification email or needs to have it resent for any other reason.
     *
     * You can retrieve the IDs associated with Sender Identities by passing a "Get All Verified Senders" endpoint.
     */
    pub async fn post_verified_senders_resend(&self, id: &str) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/verified_senders/resend/{}",
                crate::progenitor_support::encode_path(id),
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

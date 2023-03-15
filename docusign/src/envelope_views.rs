use crate::Client;
use crate::ClientResult;

pub struct EnvelopeViews {
    pub client: Client,
}

impl EnvelopeViews {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeViews { client }
    }

    /**
     * Returns a URL to the envelope correction UI.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/views/correct` endpoint.
     *
     * Returns a URL that allows you to embed the envelope correction view of the DocuSign UI in your applications.
     *
     * **Important**: Due to screen space issues, iFrames should not be used for embedded operations on mobile devices. For iOS devices, DocuSign recommends using a WebView.
     *
     * **Note**: You can revoke this URL by making the DELETE call to the same URL with no request body.
     *
     * <blockquote>
     * <p><b>Information Security notice</b>: This method provides full access to the sending account. When you use this view, the current user has full access to the account. If the account has administrative privileges, then this method also provides administrator access.</p>
     *
     * <p>If your use case needs to enable a sender to update a draft envelope before it is sent or make other changes, take one of the following steps:</p>
     *
     * <ul>
     * <li>Configure each sender to have their own individual user account to use this API method.</li>
     * <li>Enhance your API integration so that this method is not needed. Your integration can create the tabs, recipients, and other envelope settings as needed.</li>
     * </ul>
     * </blockquote>
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_post_envelope_correct_view(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::CorrectViewRequest,
    ) -> ClientResult<crate::types::EnvelopeViews> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/views/correct",
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
     * Revokes the correction view URL to the Envelope UI.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/views/correct` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_delete_envelope_correct_view(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::CorrectViewRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/views/correct",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Returns a URL to the edit view UI.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/views/edit` endpoint.
     *
     * Returns a URL that enables you to embed the edit view of the DocuSign UI in your applications. This is a one-time use login token that allows the user to be placed into the DocuSign editing view.
     *
     * Upon sending completion, the user is returned to the return URL provided by the API application.
     *
     * **Important**: Due to screen space issues, iFrames should not be used for embedded operations on mobile devices. For iOS devices, DocuSign recommends using a WebView.
     *
     * **Note**: You can revoke this URL by making the DELETE call to the same URL with no request body.
     *
     * <blockquote>
     * <p><b>Information Security notice</b>: This method provides full access to the sending account. When you use this view, the current user has full access to the account. If the account has administrative privileges, then this method also provides administrator access.</p>
     *
     * <p>If your use case needs to enable a sender to update a draft envelope before it is sent or make other changes, take one of the following steps:</p>
     *
     * <ul>
     * <li>Configure each sender to have their own individual user account to use this API method.</li>
     * <li>Enhance your API integration so that this method is not needed. Your integration can create the tabs, recipients, and other envelope settings as needed.</li>
     * </ul>
     * </blockquote>
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_post_envelope_edit_view(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::ReturnUrlRequest,
    ) -> ClientResult<crate::types::EnvelopeViews> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/views/edit",
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
     * Returns a URL to the recipient view UI.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/views/recipient` endpoint.
     *
     * Returns a URL that enables you to embed the recipient view of the DocuSign UI in your applications. If the recipient is a signer, then the view will provide the signing ceremony.
     *
     * **Note**: Please redirect the client to the URL. iFrames should not be used, especially if the recipient is using a mobile or tablet.
     *
     * This method is only used with envelopes in the `sent` status.
     *
     * Your application is responsible for authenticating the identity of the recipient or signer when you use this method. Use the parameters `assertionId`, `authenticationInstant`, `authenticationMethod`, `clientUserId`, and `securityDomain` to record information on how the recipient was authenticated. At a minimum, `authenticationMethod` and `clientUserId` are required. The information that you provide is included in the envelope's certificate of completion.
     *
     * ## Redirects
     * After the signer completes or ends the signing ceremony, DocuSign will redirect the user's browser back to your app via the `returnUrl` that you supply. (The user is redirected through a different subdomain, depending on the region of the account sending the envelope. Please [verify your domain whitelist](https://www.docusign.com/trust/security/whitelists).)
     *
     * ### The event status parameter
     * DocuSign appends an `event` query parameter to the URL with the outcome of the signing ceremony. Your app can use this event parameter to determine the next step for the envelope. Do not fetch the envelope status by using Envelopes::get or a similar method because doing so could break the DocuSign rule against polling.
     *
     * **Note**: Because a user can cancel redirection, close their browser after signing, or spoof the landing URL. Hitting the `ReturnUrl` should not be the single source of truth for envelope status for your integration.
     *
     * ## The URL is time-limited
     * The URL returned by this method is valid for one use, and you must use or display it within a couple of minutes after it is generated. AFter the recipient is redirected to the recipient view, they must interact with the DocuSign system periodically or their session will time out.
     *
     * Because the URL is time-limited, it should not be stored or sent through email. After you receive it, immediately redirect the user's browser to the URL.
     *
     * If you want to invite someone to an embedded signing session via email, the email invitation's URL must be to your application. When invoked, your app should request a recipientView URL from DocuSign and then redirect the signer to that URL.
     *
     * ## Maintaining State
     * After the recipient completes the recipient view (or signing ceremony), they are redirected to your application. Your application can recover state information about the transaction by storing information in a cookie, or by including query parameters in the `returnUrl` field. Eg, `https://myapp.eg.com/docusign_return?myState=12345` When the user is redirected to your app, the `event` query parameter will be appended. In this example, prevent spoofing by not using a guessable value as the state value.
     *
     * **Note**: You can revoke the URL by making the DELETE call to the same URL with no request body.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_post_envelope_recipient_view(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::RecipientViewRequest,
    ) -> ClientResult<crate::types::EnvelopeViews> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/views/recipient",
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
     * Returns a URL to the sender view UI.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/views/sender` endpoint.
     *
     * Returns a URL that enables you to embed the sender view of the DocuSign UI in your applications.
     *
     * The returned URL can only be redirected to immediately after it is generated. It can only be used once.
     * Therefore, request the URL immediately before you redirect your user to it.
     *
     * For the best user experience, don't use an iFrame. For iOS devices DocuSign recommends using a WebView.
     *
     * Multiple solutions are available for maintaining your
     * client state. See the **Maintaining State** section of the [Embedded Signing introduction.](https://developers.docusign.com/esign-rest-api/guides/embedded-signing)
     *
     * After the user has completed the sending view, the browser is redirected to the `returnUrl` supplied.
     *
     * By default, if the envelope already contains one or more documents, DocuSign will initially show the document tagging view when you redirect to the URL.
     *
     * To start with the envelope's recipients and documents view instead, examine the URL in the method's response.
     * Then change the query parameter from `send=1` to `send=0` to start with the recipients/documents view.
     *
     * **Note**: You can revoke the URL by making the DELETE call to the same URL with no request body.
     *
     * <blockquote>
     * <p><b>Information Security notice</b>: This method provides full access to the sending account. When you use this view, the current user has full access to the account. If the account has administrative privileges, then this method also provides administrator access.</p>
     *
     * <p>If your use case needs to enable a sender to update a draft envelope before it is sent or make other changes, take one of the following steps:</p>
     *
     * <ul>
     * <li>Configure each sender to have their own individual user account to use this API method.</li>
     * <li>Enhance your API integration so that this method is not needed. Your integration can create the tabs, recipients, and other envelope settings as needed.</li>
     * </ul>
     * </blockquote>
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_post_envelope_sender_view(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::ReturnUrlRequest,
    ) -> ClientResult<crate::types::EnvelopeViews> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/views/sender",
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
     * Returns a URL to the shared recipient view UI for an envelope.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/views/shared` endpoint.
     *
     * Returns a URL that enables you to embed the DocuSign UI recipient view of a [shared envelope](https://support.docusign.com/en/guides/ndse-admin-guide-share-envelopes) in your applications. This is the view that a user sees of an envelope that a recipient on the same account has shared with them.
     *
     * **Important**: Due to screen space issues, iFrames should not be used for embedded operations on mobile devices. For iOS devices, DocuSign recommends using a WebView.
     *
     * **Note**: You can revoke this URL by making the DELETE call to the same URL with no request body.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_post_envelope_recipient_shared_view(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::RecipientViewRequest,
    ) -> ClientResult<crate::types::ViewUrl> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/views/shared",
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
     * Returns a URL to the authentication view UI.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/views/console` endpoint.
     *
     * Returns a URL that enables you to embed the authentication view of the DocuSign UI in your applications.
     *
     * **Note**: You can revoke this URL by making the DELETE call to the same URL with no request body.
     *
     * <blockquote>
     * <p><b>Information Security notice</b>: This method provides full administrator access to the account.</p>
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_post_account_console_view(
        &self,
        account_id: &str,
        body: &crate::types::ConsoleViewRequest,
    ) -> ClientResult<crate::types::EnvelopeViews> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/views/console",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
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

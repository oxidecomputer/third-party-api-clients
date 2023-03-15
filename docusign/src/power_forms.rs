use crate::Client;
use crate::ClientResult;

pub struct PowerForms {
    pub client: Client,
}

impl PowerForms {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PowerForms { client }
    }

    /**
     * Returns a list of PowerForms.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/powerforms` endpoint.
     *
     * This method returns a list of PowerForms that are available to the user.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `from_date: &str` -- (Optional) The start date for a date range.
     *   
     *   **Note**: If no value is provided, no date filtering is applied.
     * * `order: &str` -- (Optional) The order in which to sort the results.
     *   
     *   Valid values are:
     *   
     *   
     *   * `asc`: Ascending order.
     *   * `desc`: Descending order.
     * * `order_by: &str` -- (Optional) The file attribute to use to sort the results.
     *   
     *   Valid values are:
     *   
     *   * `modified`
     *   * `name`.
     * * `to_date: &str` -- (Optional) The end date for a date range.
     *   
     *   **Note**: If no value is provided, this property defaults to the current date.
     */
    pub async fn get_list(
        &self,
        account_id: &str,
        from_date: &str,
        order: &str,
        order_by: &str,
        to_date: &str,
    ) -> ClientResult<crate::types::PowerFormsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from_date.is_empty() {
            query_args.push(("from_date".to_string(), from_date.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !order_by.is_empty() {
            query_args.push(("order_by".to_string(), order_by.to_string()));
        }
        if !to_date.is_empty() {
            query_args.push(("to_date".to_string(), to_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/powerforms?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
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
     * Creates a new PowerForm.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/powerforms` endpoint.
     *
     * This method creates a new PowerForm.
     *
     * You create a PowerForm from an existing DocuSign [template](https://developers.docusign.com/docs/esign-rest-api/reference/templates/templates/create/), based on the `templateId` in the request body.
     *
     *  PowerForms that you create from a template are referred to as *web PowerForms*.
     *
     * **Note**: The RADmin console also supports creating a PowerForm by uploading a PDF file that has active form fields (referred to as a *PDF PowerForm*). However, PDF PowerForms are deprecated and are not supported in the API.
     *
     * **Note**: A PowerForm can have only one sender. (Because PowerForms are not necessarily sent by email, this user is also referred to as the PowerForm *initiator*.) If you need to associate multiple senders with a PowerForm, create multiple copies of the PowerForm by using the same template (one copy for each sender). By default, the sender is the PowerForm Administrator who creates the PowerForm.
     *
     *
     * ### Signing modes
     *
     * You can use one of the following signing modes for a PowerForm:
     *
     * **`email`**
     *
     * This mode verifies the recipient's identity by using email authentication before the recipient can sign a document. The recipient enters their email address on the landing page and then clicks **Begin Signing** to begin the signing process. The system then sends an email message with a validation code to the recipient. If the recipient does not provide a valid email address, they do not receive the email message containing the access code and are not able to open and sign the document.
     *
     * Alternatively, you can make the process easier for signers by using email authentication only and omitting the access code. To do this, you append the `activateonly` flag to the PowerForm URL and set it to true by passing in the value `1`.  When the flag is active, the first recipient receives an email with a link that initiates the signing session without having to enter access code.
     *
     * Example: `activateonly=1`
     *
     * **`direct`**
     *
     * This mode does not require any verification. After a recipient enters their email address on the landing page and clicks **Begin Signing**, a new browser tab opens and the recipient can immediately begin the signing process.
     *
     * Because the `direct` signing mode does not verify the recipient's identity by using email authentication, we strongly recommend that you use this mode only when the PowerForm is accessible behind a secure portal where the recipient's identity is already authenticated, or where another form of authentication is specified for the recipient in the DocuSign template (for example, an access code, phone authentication, or ID check).
     *
     * **Note**: In the account settings, `enablePowerFormDirect` must be **true** to use `direct` as the `signingMode`.
     *
     * ### Redirect URLs
     *
     * You can control the URL to which signers are redirected after signing your PowerForm. However, the URL is specified elsewhere, outside of the PowerForm creation process. For details, see [How do I specify a URL to redirect to when a PowerForm is completed?](https://support.docusign.com/en/articles/How-do-I-specify-a-URL-to-redirect-to-when-a-Powerform-is-completed).
     *
     * ### More information
     *
     * For more information about creating PowerForms, see [Create a PowerForm](https://support.docusign.com/en/guides/ndse-user-guide-create-a-powerform).
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn post_form(
        &self,
        account_id: &str,
        body: &crate::types::PowerForm,
    ) -> ClientResult<crate::types::PowerForm> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/powerforms",
                crate::progenitor_support::encode_path(account_id),
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
     * Deletes one or more PowerForms.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/powerforms` endpoint.
     *
     * This method deletes one or more PowerForms. The request body takes an array of PowerForm objects that are deleted based on the `powerFormId`.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete_list(
        &self,
        account_id: &str,
        body: &crate::types::PowerFormsRequest,
    ) -> ClientResult<crate::types::PowerFormsResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/powerforms",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Gets PowerForm senders.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/powerforms/senders` endpoint.
     *
     * This method returns a list of users who have sent PowerForms.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- The position within the total result set from which to start returning values. The value **thumbnail** may be used to return the page image.
     */
    pub async fn get_senders(
        &self,
        account_id: &str,
        start_position: &str,
    ) -> ClientResult<crate::types::PowerFormSendersResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/powerforms/senders?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
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
     * Returns a single PowerForm.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/powerforms/{powerFormId}` endpoint.
     *
     * This method returns detailed information about a specific PowerForm.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `power_form_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get_form(
        &self,
        account_id: &str,
        power_form_id: &str,
    ) -> ClientResult<crate::types::PowerForm> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/powerforms/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(power_form_id),
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
     * Updates an existing PowerForm.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/powerforms/{powerFormId}` endpoint.
     *
     * This method updates an existing PowerForm.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `power_form_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put_form(
        &self,
        account_id: &str,
        power_form_id: &str,
        body: &crate::types::PowerForm,
    ) -> ClientResult<crate::types::PowerForm> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/powerforms/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(power_form_id),
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
     * Deletes a PowerForm.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/powerforms/{powerFormId}` endpoint.
     *
     * This method deletes a PowerForm.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `power_form_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete_form(&self, account_id: &str, power_form_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/powerforms/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(power_form_id),
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

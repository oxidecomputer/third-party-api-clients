use crate::Client;
use crate::ClientResult;

pub struct Contacts {
    pub client: Client,
}

impl Contacts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Contacts { client }
    }

    /**
     * Updates one or more contacts.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/contacts` endpoint.
     *
     * This method updates one or more contacts associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        body: &crate::types::ContactModRequest,
    ) -> ClientResult<crate::types::ContactUpdateResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/contacts",
                crate::progenitor_support::encode_path(account_id),
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
     * Imports new contacts into a contacts list.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/contacts` endpoint.
     *
     * This method imports multiple new contacts into a contact list from a CSV, JSON, or XML file.
     *
     * To use this method, you must provide a request body in one of the supported formats and include a `content-type` header with the appropriate value.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn post(
        &self,
        account_id: &str,
        body: &crate::types::ContactModRequest,
    ) -> ClientResult<crate::types::ContactUpdateResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/contacts",
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
     * Deletes multiple contacts from an account.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/contacts` endpoint.
     *
     * This method deletes multiple contacts associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete(
        &self,
        account_id: &str,
        body: &crate::types::ContactModRequest,
    ) -> ClientResult<crate::types::ContactUpdateResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/contacts",
                crate::progenitor_support::encode_path(account_id),
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
     * Gets one or more contacts.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/contacts/{contactId}` endpoint.
     *
     * This method returns one or more contacts
     * associated with a DocuSign account. You can also
     * retrieve contacts from connected [cloud storage][CloudStorage] providers by using the
     * `cloud_provider` query parameter. By default,
     * contacts are retrieved from the DocuSign account's
     * default address book.
     *
     * To return a specific contact, use the `contactId`
     * query parameter. To return all contacts associated
     * with an account, omit this parameter.
     *
     * [CloudStorage]: https://developers.docusign.com/docs/esign-rest-api/reference/CloudStorage
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `contact_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `cloud_provider: &str` -- (Optional) The cloud provider from which to retrieve the contacts. Valid values are:
     *   
     *   - `rooms`
     *   - `docusignCore` (default).
     */
    pub async fn get(
        &self,
        account_id: &str,
        contact_id: &str,
        cloud_provider: &str,
    ) -> ClientResult<crate::types::ContactGetResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cloud_provider.is_empty() {
            query_args.push(("cloud_provider".to_string(), cloud_provider.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/contacts/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(contact_id),
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
     * Deletes a contact.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/contacts/{contactId}` endpoint.
     *
     * This method deletes a contact associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `contact_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete_contacts(
        &self,
        account_id: &str,
        contact_id: &str,
    ) -> ClientResult<crate::types::ContactUpdateResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/contacts/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(contact_id),
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

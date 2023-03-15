use crate::Client;
use crate::ClientResult;

pub struct Folders {
    pub client: Client,
}

impl Folders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Folders { client }
    }

    /**
     * Gets a list of the folders for the account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/folders` endpoint.
     *
     * Retrieves a list of the folders for the account, including the folder hierarchy. You can specify whether to return just the template folder or template folder and normal folders by setting the `template` query string parameter.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include: &str` -- A comma-separated list of folder types to include in the response.
     *   Valid values are:
     *   
     *   - `envelope_folders`: Returns a list of envelope folders. (Default)
     *   - `template_folders`: Returns a list of template folders.
     *   - `shared_template_folders`: Returns a list of shared template folders.
     *   .
     * * `include_items: &str` -- Indicates whether folder items are included in the response. If this parameter is omitted, the default is false.
     * * `start_position: &str` -- The position within the total result set from which to start returning values.
     * * `template: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_filter: &str` -- Narrows down the resulting folder list by the following values:
     *   
     *   - `all`: Returns all templates owned or shared with the user. (default)
     *   - `owned_by_me`: Returns only  templates the user owns.
     *   - `shared_with_me`: Returns only templates that are shared with the user.
     *   .
     */
    pub async fn get(
        &self,
        account_id: &str,
        include: &str,
        include_items: &str,
        start_position: &str,
        template: &str,
        user_filter: &str,
    ) -> ClientResult<crate::types::FoldersResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        if !include_items.is_empty() {
            query_args.push(("include_items".to_string(), include_items.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        if !template.is_empty() {
            query_args.push(("template".to_string(), template.to_string()));
        }
        if !user_filter.is_empty() {
            query_args.push(("user_filter".to_string(), user_filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/folders?{}",
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
     * Gets a list of the envelopes in the specified folder.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/folders/{folderId}` endpoint.
     *
     * Retrieves a list of the envelopes in the specified folder. You can narrow the query by specifying search criteria in the query string parameters.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `folder_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `from_date: &str` -- The billing period end date in UTC timedate format.
     * * `include_items: &str` -- Indicates whether folder items are included in the response. If this parameter is omitted, the default is false.
     * * `owner_email: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `owner_name: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `search_text: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `status: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `to_date: &str` -- The billing period end date in UTC timedate format.
     */
    pub async fn get_items(
        &self,
        account_id: &str,
        folder_id: &str,
        from_date: &str,
        include_items: &str,
        owner_email: &str,
        owner_name: &str,
        search_text: &str,
        start_position: &str,
        status: &str,
        to_date: &str,
    ) -> ClientResult<crate::types::FolderItemsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from_date.is_empty() {
            query_args.push(("from_date".to_string(), from_date.to_string()));
        }
        if !include_items.is_empty() {
            query_args.push(("include_items".to_string(), include_items.to_string()));
        }
        if !owner_email.is_empty() {
            query_args.push(("owner_email".to_string(), owner_email.to_string()));
        }
        if !owner_name.is_empty() {
            query_args.push(("owner_name".to_string(), owner_name.to_string()));
        }
        if !search_text.is_empty() {
            query_args.push(("search_text".to_string(), search_text.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !to_date.is_empty() {
            query_args.push(("to_date".to_string(), to_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/folders/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(folder_id),
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
     * Moves an envelope from its current folder to the specified folder.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/folders/{folderId}` endpoint.
     *
     * Moves an envelope from its current folder to the specified folder.
     *
     * You can use this method to delete envelopes by specifying `recyclebin` in the `folderId` parameter.
     * Placing an in-process envelope (envelope status of `sent` or `delivered`) in the recycle bin voids the envelope.
     *
     * You can also use this method to delete templates by specifying a template ID instead of an envelope ID in the `envelopeIds` property and specifying `recyclebin` in the `folderId` parameter.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `folder_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        folder_id: &str,
        body: &crate::types::FoldersRequest,
    ) -> ClientResult<crate::types::FoldersResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/folders/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(folder_id),
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
     * Gets a list of envelopes in folders matching the specified criteria.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/search_folders/{searchFolderId}` endpoint.
     *
     * **This method is deprecated in API v2.1.**
     *
     * Use  [Envelopes::listStatusChanges](https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/listStatusChanges) instead.
     *
     * Retrieves a list of items that match the criteria specified in the query.
     *
     * If the user ID of the user making the call is the same as the user ID for any returned recipient, then the userId property is added to the returned information for those recipients.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `search_folder_id: &str` -- Specifies the envelope group that is searched by the request. These are logical groupings, not actual folder names. Valid values are: drafts, awaiting_my_signature, completed, out_for_signature.
     * * `all: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- Specifies the number of records returned in the cache. The number must be greater than 0 and less than or equal to 100.
     * * `from_date: &str` -- Specifies the start of the date range to return. If no value is provided, the default search is the previous 30 days.
     * * `include_recipients: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `order: &str` -- Specifies the order in which the list is returned. Valid values are: `asc` for ascending order, and `desc` for descending order.
     * * `order_by: &str` -- Specifies the property used to sort the list. Valid values are: `action_required`, `created`, `completed`, `sent`, `signer_list`, `status`, or `subject`.
     * * `start_position: &str` -- Specifies the the starting location in the result set of the items that are returned.
     * * `to_date: &str` -- The billing period end date in UTC timedate format.
     */
    pub async fn search_get_contents(
        &self,
        account_id: &str,
        search_folder_id: &str,
        all: &str,
        count: &str,
        from_date: &str,
        include_recipients: &str,
        order: &str,
        order_by: &str,
        start_position: &str,
        to_date: &str,
    ) -> ClientResult<crate::types::FolderItemResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !all.is_empty() {
            query_args.push(("all".to_string(), all.to_string()));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !from_date.is_empty() {
            query_args.push(("from_date".to_string(), from_date.to_string()));
        }
        if !include_recipients.is_empty() {
            query_args.push((
                "include_recipients".to_string(),
                include_recipients.to_string(),
            ));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !order_by.is_empty() {
            query_args.push(("order_by".to_string(), order_by.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        if !to_date.is_empty() {
            query_args.push(("to_date".to_string(), to_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/search_folders/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(search_folder_id),
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
}

use crate::Client;
use crate::ClientResult;

pub struct ContactsApiLists {
    pub client: Client,
}

impl ContactsApiLists {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ContactsApiLists { client }
    }

    /**
     * Retrieve all lists.
     *
     * This function performs a `GET` to the `/contactdb/lists` endpoint.
     *
     * **This endpoint allows you to retrieve all of your recipient lists. If you don't have any lists, an empty array will be returned.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_lists(&self) -> ClientResult<crate::types::ListAllListsResponse> {
        let url = self.client.url("/contactdb/lists", None);
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
     * Create a List.
     *
     * This function performs a `POST` to the `/contactdb/lists` endpoint.
     *
     * **This endpoint allows you to create a list for your recipients.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_contactdb_list(
        &self,
        body: &crate::types::IpPool,
    ) -> ClientResult<crate::types::ContactdbList> {
        let url = self.client.url("/contactdb/lists", None);
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
     * Delete Multiple lists.
     *
     * This function performs a `DELETE` to the `/contactdb/lists` endpoint.
     *
     * **This endpoint allows you to delete multiple recipient lists.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_contactdb_lists(&self, body: &[i64]) -> ClientResult<()> {
        let url = self.client.url("/contactdb/lists", None);
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
     * Retrieve a single list.
     *
     * This function performs a `GET` to the `/contactdb/lists/{list_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a single recipient list.**
     *
     * **Parameters:**
     *
     * * `list_id: i64` -- The ID of the list to retrieve.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_lists_list(
        &self,
        list_id: &str,
    ) -> ClientResult<crate::types::ContactdbList> {
        let url = self.client.url(
            &format!(
                "/contactdb/lists/{}",
                crate::progenitor_support::encode_path(list_id),
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
     * Delete a List.
     *
     * This function performs a `DELETE` to the `/contactdb/lists/{list_id}` endpoint.
     *
     * **This endpoint allows you to delete a specific recipient list with the given ID.**
     *
     * **Parameters:**
     *
     * * `delete_contacts: bool` -- Adds the ability to delete all contacts on the list in addition to deleting the list.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_contactdb_lists_list(
        &self,
        list_id: &str,
        delete_contacts: bool,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if delete_contacts {
            query_args.push(("delete_contacts".to_string(), delete_contacts.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/contactdb/lists/{}?{}",
                crate::progenitor_support::encode_path(list_id),
                query_
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
     * Update a List.
     *
     * This function performs a `PATCH` to the `/contactdb/lists/{list_id}` endpoint.
     *
     * **This endpoint allows you to update the name of one of your recipient lists.**
     *
     * **Parameters:**
     *
     * * `list_id: i64` -- The ID of the list you are updating.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_contactdb_lists_list(
        &self,
        list_id: &str,
        body: &crate::types::IpPool,
    ) -> ClientResult<crate::types::PatchContactdbListsListResponse> {
        let url = self.client.url(
            &format!(
                "/contactdb/lists/{}",
                crate::progenitor_support::encode_path(list_id),
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
     * Retrieve all recipients on a List.
     *
     * This function performs a `GET` to the `/contactdb/lists/{list_id}/recipients` endpoint.
     *
     * **This endpoint allows you to retrieve all recipients on the list with the given ID.**
     *
     * **Parameters:**
     *
     * * `page: i64` -- Page index of first recipient to return (must be a positive integer).
     * * `page_size: i64` -- Number of recipients to return at a time (must be a positive integer between 1 and 1000).
     * * `list_id: i64` -- The ID of the list whose recipients you are requesting.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_lists_list_recipients(
        &self,
        list_id: i64,
        page: i64,
        page_size: i64,
    ) -> ClientResult<crate::types::GetContactdbRecipientsSearchResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/contactdb/lists/{}/recipients?{}",
                crate::progenitor_support::encode_path(&list_id.to_string()),
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
     * Add Multiple Recipients to a List.
     *
     * This function performs a `POST` to the `/contactdb/lists/{list_id}/recipients` endpoint.
     *
     * **This endpoint allows you to add multiple recipients to a list.**
     *
     * Adds existing recipients to a list, passing in the recipient IDs to add. Recipient IDs should be passed exactly as they are returned from recipient endpoints.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_contactdb_lists_list_recipient(
        &self,
        list_id: i64,
        body: &[i64],
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/contactdb/lists/{}/recipients",
                crate::progenitor_support::encode_path(&list_id.to_string()),
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
    /**
     * Add a Single Recipient to a List.
     *
     * This function performs a `POST` to the `/contactdb/lists/{list_id}/recipients/{recipient_id}` endpoint.
     *
     * **This endpoint allows you to add a single recipient to a list.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_contactdb_lists_list_recipients_recipient(
        &self,
        list_id: i64,
        recipient_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/contactdb/lists/{}/recipients/{}",
                crate::progenitor_support::encode_path(&list_id.to_string()),
                crate::progenitor_support::encode_path(recipient_id),
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
    /**
     * Delete a Single Recipient from a Single List.
     *
     * This function performs a `DELETE` to the `/contactdb/lists/{list_id}/recipients/{recipient_id}` endpoint.
     *
     * **This endpoint allows you to delete a single recipient from a list.**
     *
     * **Parameters:**
     *
     * * `list_id: i64` -- The ID of the list you are taking this recipient away from.
     * * `recipient_id: i64` -- The ID of the recipient to take off the list.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_contactdb_lists_list_recipients_recipient(
        &self,
        list_id: i64,
        recipient_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/contactdb/lists/{}/recipients/{}",
                crate::progenitor_support::encode_path(&list_id.to_string()),
                crate::progenitor_support::encode_path(recipient_id),
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
}

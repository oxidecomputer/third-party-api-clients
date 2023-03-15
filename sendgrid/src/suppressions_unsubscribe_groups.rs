use crate::Client;
use crate::ClientResult;

pub struct SuppressionsUnsubscribeGroups {
    pub client: Client,
}

impl SuppressionsUnsubscribeGroups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SuppressionsUnsubscribeGroups { client }
    }

    /**
     * Retrieve all suppression groups associated with the user.
     *
     * This function performs a `GET` to the `/asm/groups` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all suppression groups created by this user.**
     *
     * This endpoint can also return information for multiple group IDs that you include in your request. To add a group ID to your request, simply append `?id=123456&id=123456`, with the appropriate group IDs.
     *
     * **Parameters:**
     *
     * * `id: i64`
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_asm_groups(
        &self,
        id: i64,
    ) -> ClientResult<Vec<crate::types::SuppressionGroup>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if id > 0 {
            query_args.push(("id".to_string(), id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/asm/groups?{}", query_), None);
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
     * Retrieve all suppression groups associated with the user.
     *
     * This function performs a `GET` to the `/asm/groups` endpoint.
     *
     * As opposed to `get_asm_groups`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a list of all suppression groups created by this user.**
     *
     * This endpoint can also return information for multiple group IDs that you include in your request. To add a group ID to your request, simply append `?id=123456&id=123456`, with the appropriate group IDs.
     */
    pub async fn get_all_asm_groups(
        &self,
        id: i64,
    ) -> ClientResult<Vec<crate::types::SuppressionGroup>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if id > 0 {
            query_args.push(("id".to_string(), id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/asm/groups?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create a new suppression group.
     *
     * This function performs a `POST` to the `/asm/groups` endpoint.
     *
     * **This endpoint allows you to create a new suppression group.**
     *
     * To add an email address to the suppression group, [create a Suppression](https://sendgrid.api-docs.io/v3.0/suppressions-suppressions/add-suppressions-to-a-suppression-group).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_asm_group(
        &self,
        body: &crate::types::SuppressionGroupRequestBase,
    ) -> ClientResult<crate::types::PostAsmGroupsResponse> {
        let url = self.client.url("/asm/groups", None);
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
     * Get information on a single suppression group.
     *
     * This function performs a `GET` to the `/asm/groups/{group_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a single suppression group.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_asm_groups_group(
        &self,
        group_id: &str,
    ) -> ClientResult<crate::types::GetAsmGroupsGroupResponseAllOf> {
        let url = self.client.url(
            &format!(
                "/asm/groups/{}",
                crate::progenitor_support::encode_path(group_id),
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
     * Delete a Suppression Group.
     *
     * This function performs a `DELETE` to the `/asm/groups/{group_id}` endpoint.
     *
     * **This endpoint allows you to delete a suppression group.**
     *
     * If a recipient uses the "one-click unsubscribe" option on an email associated with a deleted group, that recipient will be added to the global suppression list.
     *
     * Deleting a suppression group will remove the suppression, meaning email will once again be sent to the previously suppressed addresses. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_asm_groups_group(
        &self,
        group_id: &str,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/asm/groups/{}",
                crate::progenitor_support::encode_path(group_id),
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
     * Update a suppression group.
     *
     * This function performs a `PATCH` to the `/asm/groups/{group_id}` endpoint.
     *
     * **This endpoint allows you to update or change a suppression group.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_asm_groups_group(
        &self,
        group_id: &str,
        body: &crate::types::SuppressionGroupRequestBase,
    ) -> ClientResult<crate::types::SuppressionGroup> {
        let url = self.client.url(
            &format!(
                "/asm/groups/{}",
                crate::progenitor_support::encode_path(group_id),
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

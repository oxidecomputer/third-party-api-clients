use crate::Client;
use crate::ClientResult;

pub struct Suppressions {
    pub client: Client,
}

impl Suppressions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Suppressions { client }
    }

    /**
     * Retrieve all suppressions for a suppression group.
     *
     * This function performs a `GET` to the `/asm/groups/{group_id}/suppressions` endpoint.
     *
     * **This endpoint allows you to retrieve all suppressed email addresses belonging to the given group.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_asm_groups_group(&self, group_id: &str) -> ClientResult<Vec<String>> {
        let url = self.client.url(
            &format!(
                "/asm/groups/{}/suppressions",
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
     * Retrieve all suppressions for a suppression group.
     *
     * This function performs a `GET` to the `/asm/groups/{group_id}/suppressions` endpoint.
     *
     * As opposed to `get_asm_groups_group`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all suppressed email addresses belonging to the given group.**
     */
    pub async fn get_all_asm_groups_group(&self, group_id: &str) -> ClientResult<Vec<String>> {
        let url = self.client.url(
            &format!(
                "/asm/groups/{}/suppressions",
                crate::progenitor_support::encode_path(group_id),
            ),
            None,
        );
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
     * Add suppressions to a suppression group.
     *
     * This function performs a `POST` to the `/asm/groups/{group_id}/suppressions` endpoint.
     *
     * **This endpoint allows you to add email addresses to an unsubscribe group.**
     *
     * If you attempt to add suppressions to a group that has been deleted or does not exist, the suppressions will be added to the global suppressions list.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_asm_groups_group(
        &self,
        group_id: &str,
        body: &crate::types::SuppressionsRequestBody,
    ) -> ClientResult<crate::types::PostAsmGroupsGroupSuppressionsResponse> {
        let url = self.client.url(
            &format!(
                "/asm/groups/{}/suppressions",
                crate::progenitor_support::encode_path(group_id),
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
     * Search for suppressions within a group.
     *
     * This function performs a `POST` to the `/asm/groups/{group_id}/suppressions/search` endpoint.
     *
     * **This endpoint allows you to search a suppression group for multiple suppressions.**
     *
     * When given a list of email addresses and a group ID, this endpoint will only return the email addresses that have been unsubscribed from the given group.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_asm_groups_group_search(
        &self,
        group_id: &str,
        body: &crate::types::SuppressionsRequestBody,
    ) -> ClientResult<Vec<String>> {
        let url = self.client.url(
            &format!(
                "/asm/groups/{}/suppressions/search",
                crate::progenitor_support::encode_path(group_id),
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
     * Retrieve all suppressions.
     *
     * This function performs a `GET` to the `/asm/suppressions` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all suppressions.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_asm(&self) -> ClientResult<Vec<crate::types::GetAsmSuppressionsResponse>> {
        let url = self.client.url("/asm/suppressions", None);
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
     * Retrieve all suppressions.
     *
     * This function performs a `GET` to the `/asm/suppressions` endpoint.
     *
     * As opposed to `get_asm`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a list of all suppressions.**
     */
    pub async fn get_all_asm(&self) -> ClientResult<Vec<crate::types::GetAsmSuppressionsResponse>> {
        let url = self.client.url("/asm/suppressions", None);
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
     * Retrieve all suppression groups for an email address.
     *
     * This function performs a `GET` to the `/asm/suppressions/{email}` endpoint.
     *
     * **This endpoint returns a list of all groups from which the given email address has been unsubscribed.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_asm_email(
        &self,
        email: &str,
    ) -> ClientResult<crate::types::GetAsmSuppressionsEmailResponse> {
        let url = self.client.url(
            &format!(
                "/asm/suppressions/{}",
                crate::progenitor_support::encode_path(email),
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
     * Delete a suppression from a suppression group.
     *
     * This function performs a `DELETE` to the `/asm/groups/{group_id}/suppressions/{email}` endpoint.
     *
     * **This endpoint allows you to remove a suppressed email address from the given suppression group.**
     *
     * Removing an address will remove the suppression, meaning email will once again be sent to the previously suppressed addresses. This should be avoided unless a recipient indicates they wish to receive email from you again. You can use our [bypass filters](https://sendgrid.com/docs/ui/sending-email/index-suppressions/#bypass-suppressions) to deliver messages to otherwise suppressed addresses when exceptions are required.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_asm_groups_group_email(
        &self,
        group_id: &str,
        email: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/asm/groups/{}/suppressions/{}",
                crate::progenitor_support::encode_path(group_id),
                crate::progenitor_support::encode_path(email),
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

use crate::Client;
use crate::ClientResult;

pub struct SigningGroupUsers {
    pub client: Client,
}

impl SigningGroupUsers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SigningGroupUsers { client }
    }

    /**
     * Gets a list of members in a Signing Group.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/signing_groups/{signingGroupId}/users` endpoint.
     *
     * Retrieves the list of members in the specified Signing Group.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signing_group_id: &str` -- Optional. The ID of the [signing group](https://support.docusign.com/en/guides/ndse-user-guide-signing-groups).
     *   
     *   **Note**: When you send an envelope to a signing group, anyone in the group can open it and sign it with their own signature. For this reason, we recommend that you do not include non-signer recipients (such as carbon copy recipients) in the same signing group as signer recipients. However, you could create a second signing group for the non-signer recipients and change the default action of Needs to Sign to a different value, such as Receives a Copy.
     */
    pub async fn signing_groups_get_group_user(
        &self,
        account_id: &str,
        signing_group_id: &str,
    ) -> ClientResult<crate::types::SigningGroupUsersData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signing_groups/{}/users",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(signing_group_id),
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
     * Adds members to a signing group. .
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/signing_groups/{signingGroupId}/users` endpoint.
     *
     * Adds one or more new members to a signing group. A signing group can have a maximum of 50 members.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signing_group_id: &str` -- Optional. The ID of the [signing group](https://support.docusign.com/en/guides/ndse-user-guide-signing-groups).
     *   
     *   **Note**: When you send an envelope to a signing group, anyone in the group can open it and sign it with their own signature. For this reason, we recommend that you do not include non-signer recipients (such as carbon copy recipients) in the same signing group as signer recipients. However, you could create a second signing group for the non-signer recipients and change the default action of Needs to Sign to a different value, such as Receives a Copy.
     */
    pub async fn signing_groups_put_group_users(
        &self,
        account_id: &str,
        signing_group_id: &str,
        body: &crate::types::SigningGroupUsersData,
    ) -> ClientResult<crate::types::SigningGroupUsersData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signing_groups/{}/users",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(signing_group_id),
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
     * Deletes  one or more members from a signing group.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/signing_groups/{signingGroupId}/users` endpoint.
     *
     * Deletes  one or more members from the specified signing group.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signing_group_id: &str` -- Optional. The ID of the [signing group](https://support.docusign.com/en/guides/ndse-user-guide-signing-groups).
     *   
     *   **Note**: When you send an envelope to a signing group, anyone in the group can open it and sign it with their own signature. For this reason, we recommend that you do not include non-signer recipients (such as carbon copy recipients) in the same signing group as signer recipients. However, you could create a second signing group for the non-signer recipients and change the default action of Needs to Sign to a different value, such as Receives a Copy.
     */
    pub async fn signing_groups_delete_group_users(
        &self,
        account_id: &str,
        signing_group_id: &str,
        body: &crate::types::SigningGroupUsersData,
    ) -> ClientResult<crate::types::SigningGroupUsersData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signing_groups/{}/users",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(signing_group_id),
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

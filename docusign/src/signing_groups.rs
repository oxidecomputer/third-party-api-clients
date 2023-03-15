use crate::Client;
use crate::ClientResult;

pub struct SigningGroups {
    pub client: Client,
}

impl SigningGroups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SigningGroups { client }
    }

    /**
     * Gets a list of the Signing Groups in an account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/signing_groups` endpoint.
     *
     * Retrieves a list of all signing groups in the specified account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `group_type: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_users: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(
        &self,
        account_id: &str,
        group_type: &str,
        include_users: &str,
    ) -> ClientResult<crate::types::SigningGroupInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !group_type.is_empty() {
            query_args.push(("group_type".to_string(), group_type.to_string()));
        }
        if !include_users.is_empty() {
            query_args.push(("include_users".to_string(), include_users.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signing_groups?{}",
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
     * Updates signing group names.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/signing_groups` endpoint.
     *
     * Updates the name of one or more existing signing groups.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        body: &crate::types::SigningGroupInformation,
    ) -> ClientResult<crate::types::SigningGroupInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signing_groups",
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
     * Creates a signing group. .
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/signing_groups` endpoint.
     *
     * Creates one or more signing groups.
     *
     * Multiple signing groups can be created in one call. Only users with account administrator privileges can create signing groups.
     *
     * An account can have a maximum of 50 signing groups. Each signing group can have a maximum of 50 group members.
     *
     * Signing groups can be used by any account user.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn post(
        &self,
        account_id: &str,
        body: &crate::types::SigningGroupInformation,
    ) -> ClientResult<crate::types::SigningGroupInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signing_groups",
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
     * Deletes one or more signing groups.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/signing_groups` endpoint.
     *
     * Deletes one or more signing groups in the specified account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete(
        &self,
        account_id: &str,
        body: &crate::types::SigningGroupInformation,
    ) -> ClientResult<crate::types::SigningGroupInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signing_groups",
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
     * Gets information about a signing group. .
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/signing_groups/{signingGroupId}` endpoint.
     *
     * Retrieves information, including group member information, for the specified signing group.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signing_group_id: &str` -- Optional. The ID of the [signing group](https://support.docusign.com/en/guides/ndse-user-guide-signing-groups).
     *   
     *   **Note**: When you send an envelope to a signing group, anyone in the group can open it and sign it with their own signature. For this reason, we recommend that you do not include non-signer recipients (such as carbon copy recipients) in the same signing group as signer recipients. However, you could create a second signing group for the non-signer recipients and change the default action of Needs to Sign to a different value, such as Receives a Copy.
     */
    pub async fn get_group(
        &self,
        account_id: &str,
        signing_group_id: &str,
    ) -> ClientResult<crate::types::SigningGroup> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signing_groups/{}",
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
     * Updates a signing group. .
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/signing_groups/{signingGroupId}` endpoint.
     *
     * Updates signing group name and member information. You can also add new members to the signing group. A signing group can have a maximum of 50 members.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signing_group_id: &str` -- Optional. The ID of the [signing group](https://support.docusign.com/en/guides/ndse-user-guide-signing-groups).
     *   
     *   **Note**: When you send an envelope to a signing group, anyone in the group can open it and sign it with their own signature. For this reason, we recommend that you do not include non-signer recipients (such as carbon copy recipients) in the same signing group as signer recipients. However, you could create a second signing group for the non-signer recipients and change the default action of Needs to Sign to a different value, such as Receives a Copy.
     */
    pub async fn put_group(
        &self,
        account_id: &str,
        signing_group_id: &str,
        body: &crate::types::SigningGroup,
    ) -> ClientResult<crate::types::SigningGroup> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signing_groups/{}",
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
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}

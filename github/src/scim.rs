use crate::Client;
use crate::ClientResult;

pub struct Scim {
    pub client: Client,
}

impl Scim {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Scim { client }
    }

    /**
     * List SCIM provisioned identities.
     *
     * This function performs a `GET` to the `/scim/v2/organizations/{org}/Users` endpoint.
     *
     * Retrieves a paginated list of all provisioned organization members, including pending invitations. If you provide the `filter` parameter, the resources for all matching provisions members are returned.
     *
     * When a user with a SAML-provisioned external identity leaves (or is removed from) an organization, the account's metadata is immediately removed. However, the returned list of user accounts might not always match the organization or enterprise member list you see on GitHub. This can happen in certain cases where an external identity associated with an organization will not match an organization member:
     *   - When a user with a SCIM-provisioned external identity is removed from an organization, the account's metadata is preserved to allow the user to re-join the organization in the future.
     *   - When inviting a user to join an organization, you can expect to see their external identity in the results before they accept the invitation, or if the invitation is cancelled (or never accepted).
     *   - When a user is invited over SCIM, an external identity is created that matches with the invitee's email address. However, this identity is only linked to a user account when the user accepts the invitation by going through SAML SSO.
     *
     * The returned list of external identities can include an entry for a `null` user. These are unlinked SAML identities that are created when a user goes through the following Single Sign-On (SSO) process but does not sign in to their GitHub account after completing SSO:
     *
     * 1. The user is granted access by the IdP and is not a member of the GitHub organization.
     *
     * 1. The user attempts to access the GitHub organization and initiates the SAML SSO process, and is not currently signed in to their GitHub account.
     *
     * 1. After successfully authenticating with the SAML SSO IdP, the `null` external identity entry is created and the user is prompted to sign in to their GitHub account:
     *    - If the user signs in, their GitHub account is linked to this entry.
     *    - If the user does not sign in (or does not create a new account when prompted), they are not added to the GitHub organization, and the external identity `null` entry remains in place.
     *
     * FROM: <https://docs.github.com/rest/reference/scim#list-scim-provisioned-identities>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `start_index: i64` -- Used for pagination: the index of the first result to return.
     * * `count: i64` -- Used for pagination: the number of results to return.
     * * `filter: &str` -- Filters results using the equals query parameter operator (`eq`). You can filter results that are equal to `id`, `userName`, `emails`, and `external_id`. For example, to search for an identity with the `userName` Octocat, you would use this query:
     *   
     *   `?filter=userName%20eq%20\"Octocat\"`.
     *   
     *   To filter results for the identity with the email `octocat@github.com`, you would use this query:
     *   
     *   `?filter=emails%20eq%20\"octocat@github.com\"`.
     */
    pub async fn list_provisioned_identities(
        &self,
        org: &str,
        start_index: i64,
        count: i64,
        filter: &str,
    ) -> ClientResult<crate::Response<crate::types::ScimUserList>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if start_index > 0 {
            query_args.push(("startIndex".to_string(), start_index.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/scim/v2/organizations/{}/Users?{}",
                crate::progenitor_support::encode_path(org),
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
     * Provision and invite a SCIM user.
     *
     * This function performs a `POST` to the `/scim/v2/organizations/{org}/Users` endpoint.
     *
     * Provision organization membership for a user, and send an activation email to the email address.
     *
     * FROM: <https://docs.github.com/rest/reference/scim#provision-and-invite-a-scim-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn provision_and_invite_user(
        &self,
        org: &str,
        body: &crate::types::ScimProvisionInviteUserRequest,
    ) -> ClientResult<crate::Response<crate::types::ScimUser>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/organizations/{}/Users",
                crate::progenitor_support::encode_path(org),
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
     * Get SCIM provisioning information for a user.
     *
     * This function performs a `GET` to the `/scim/v2/organizations/{org}/Users/{scim_user_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/scim#get-scim-provisioning-information-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `scim_user_id: &str` -- scim_user_id parameter.
     */
    pub async fn get_provisioning_information_for_user(
        &self,
        org: &str,
        scim_user_id: &str,
    ) -> ClientResult<crate::Response<crate::types::ScimUser>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/organizations/{}/Users/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(scim_user_id),
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
     * Update a provisioned organization membership.
     *
     * This function performs a `PUT` to the `/scim/v2/organizations/{org}/Users/{scim_user_id}` endpoint.
     *
     * Replaces an existing provisioned user's information. You must provide all the information required for the user as if you were provisioning them for the first time. Any existing user information that you don't provide will be removed. If you want to only update a specific attribute, use the [Update an attribute for a SCIM user](https://docs.github.com/rest/reference/scim#update-an-attribute-for-a-scim-user) endpoint instead.
     *
     * You must at least provide the required values for the user: `userName`, `name`, and `emails`.
     *
     * **Warning:** Setting `active: false` removes the user from the organization, deletes the external identity, and deletes the associated `{scim_user_id}`.
     *
     * FROM: <https://docs.github.com/rest/reference/scim#set-scim-information-for-a-provisioned-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `scim_user_id: &str` -- scim_user_id parameter.
     */
    pub async fn set_information_for_provisioned_user(
        &self,
        org: &str,
        scim_user_id: &str,
        body: &crate::types::ScimProvisionInviteUserRequest,
    ) -> ClientResult<crate::Response<crate::types::ScimUser>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/organizations/{}/Users/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(scim_user_id),
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
     * Delete a SCIM user from an organization.
     *
     * This function performs a `DELETE` to the `/scim/v2/organizations/{org}/Users/{scim_user_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/scim#delete-a-scim-user-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `scim_user_id: &str` -- scim_user_id parameter.
     */
    pub async fn delete_user_from_org(
        &self,
        org: &str,
        scim_user_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/organizations/{}/Users/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(scim_user_id),
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
     * Update an attribute for a SCIM user.
     *
     * This function performs a `PATCH` to the `/scim/v2/organizations/{org}/Users/{scim_user_id}` endpoint.
     *
     * Allows you to change a provisioned user's individual attributes. To change a user's values, you must provide a specific `Operations` JSON format that contains at least one of the `add`, `remove`, or `replace` operations. For examples and more information on the SCIM operations format, see the [SCIM specification](https://tools.ietf.org/html/rfc7644#section-3.5.2).
     *
     * **Note:** Complicated SCIM `path` selectors that include filters are not supported. For example, a `path` selector defined as `"path": "emails[type eq \"work\"]"` will not work.
     *
     * **Warning:** If you set `active:false` using the `replace` operation (as shown in the JSON example below), it removes the user from the organization, deletes the external identity, and deletes the associated `:scim_user_id`.
     *
     * ```
     * {
     *   "Operations":[{
     *     "op":"replace",
     *     "value":{
     *       "active":false
     *     }
     *   }]
     * }
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/scim#update-an-attribute-for-a-scim-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `scim_user_id: &str` -- scim_user_id parameter.
     */
    pub async fn update_attribute_for_user(
        &self,
        org: &str,
        scim_user_id: &str,
        body: &crate::types::ScimUpdateAttributeUserRequest,
    ) -> ClientResult<crate::Response<crate::types::ScimUser>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/organizations/{}/Users/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(scim_user_id),
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

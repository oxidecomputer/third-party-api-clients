use crate::Client;
use crate::ClientResult;

pub struct Teammates {
    pub client: Client,
}

impl Teammates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Teammates { client }
    }

    /**
     * Retrieve all teammates.
     *
     * This function performs a `GET` to the `/teammates` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all current Teammates.**
     *
     * You can limit the number of results returned using the `limit` query paramater. To return results from a specific Teammate, use the `offset` paramter. The Response Headers will include pagination info.
     *
     * **Parameters:**
     *
     * * `limit: u64` -- Number of items to return.
     * * `offset: u64` -- Paging offset.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get(
        &self,
        limit: u64,
        offset: u64,
    ) -> ClientResult<crate::types::GetTeammatesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !offset.to_string().is_empty() {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/teammates?{}", query_), None);
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
     * Invite teammate.
     *
     * This function performs a `POST` to the `/teammates` endpoint.
     *
     * **This endpoint allows you to invite a Teammate to your account via email.**
     *
     * You can set a Teammate's initial permissions using the `scopes` array in the request body. Teammate's will receive a minimum set of scopes from Twilio SendGrid that are necessary for the Teammate to function.
     *
     * **Note:** A teammate invite will expire after 7 days, but you may resend the invitation at any time to reset the expiration date.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post(
        &self,
        body: &crate::types::PostTeammatesRequest,
    ) -> ClientResult<crate::types::PostTeammatesResponse> {
        let url = self.client.url("/teammates", None);
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
     * Resend teammate invite.
     *
     * This function performs a `POST` to the `/teammates/pending/{token}/resend` endpoint.
     *
     * **This endpoint allows you to resend a Teammate invitation.**
     *
     * Teammate invitations will expire after 7 days. Resending an invitation will reset the expiration date.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_pending_token_resend(
        &self,
        token: &str,
    ) -> ClientResult<crate::types::PostTeammatesResponse> {
        let url = self.client.url(
            &format!(
                "/teammates/pending/{}/resend",
                crate::progenitor_support::encode_path(token),
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
     * Retrieve access requests.
     *
     * This function performs a `GET` to the `/scopes/requests` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all recent access requests.**
     *
     * The Response Header's `link` parameter will include pagination info.
     *
     * **Parameters:**
     *
     * * `limit: i64` -- Optional field to limit the number of results returned.
     * * `offset: i64` -- Optional beginning point in the list to retrieve from.
     */
    pub async fn get_scopes_requests(
        &self,
        limit: i64,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::GetScopesRequestsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/scopes/requests?{}", query_), None);
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
     * Retrieve access requests.
     *
     * This function performs a `GET` to the `/scopes/requests` endpoint.
     *
     * As opposed to `get_scopes_requests`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a list of all recent access requests.**
     *
     * The Response Header's `link` parameter will include pagination info.
     */
    pub async fn get_all_scopes_requests(
        &self,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::GetScopesRequestsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/scopes/requests?{}", query_), None);
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
     * Retrieve all pending teammates.
     *
     * This function performs a `GET` to the `/teammates/pending` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all pending Teammate invitations.**
     *
     * Each teammate invitation is valid for 7 days. Users may resend the invitation to refresh the expiration date.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_pending(&self) -> ClientResult<crate::types::GetTeammatesPendingResponse> {
        let url = self.client.url("/teammates/pending", None);
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
     * Retrieve specific teammate.
     *
     * This function performs a `GET` to the `/teammates/{username}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific Teammate by username.**
     *
     * You can retrieve the username's for each of your Teammates using the "Retrieve all Teammates" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_username(
        &self,
        username: &str,
    ) -> ClientResult<crate::types::GetTeammatesUsernameResponse> {
        let url = self.client.url(
            &format!(
                "/teammates/{}",
                crate::progenitor_support::encode_path(username),
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
     * Delete teammate.
     *
     * This function performs a `DELETE` to the `/teammates/{username}` endpoint.
     *
     * **This endpoint allows you to delete a teammate.**
     *
     * **Only the parent user or an admin teammate can delete another teammate.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_username(
        &self,
        username: &str,
    ) -> ClientResult<crate::types::PostSendersResponse> {
        let url = self.client.url(
            &format!(
                "/teammates/{}",
                crate::progenitor_support::encode_path(username),
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
     * Update teammate's permissions.
     *
     * This function performs a `PATCH` to the `/teammates/{username}` endpoint.
     *
     * **This endpoint allows you to update a teammate’s permissions.**
     *
     * To turn a teammate into an admin, the request body should contain an `is_admin` set to `true`. Otherwise, set `is_admin` to `false` and pass in all the scopes that a teammate should have.
     *
     * **Only the parent user or other admin teammates can update another teammate’s permissions.**
     *
     * **Admin users can only update permissions.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_username(
        &self,
        username: &str,
        body: &crate::types::PatchTeammatesUsernameRequest,
    ) -> ClientResult<crate::types::GetTeammatesUsernameResponse> {
        let url = self.client.url(
            &format!(
                "/teammates/{}",
                crate::progenitor_support::encode_path(username),
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
     * Approve access request.
     *
     * This function performs a `PATCH` to the `/scopes/requests/{request_id}/approve` endpoint.
     *
     * **This endpoint allows you to approve an access attempt.**
     *
     * **Note:** Only teammate admins may approve another teammate’s access request.
     */
    pub async fn patch_scopes_requests_approve(
        &self,
        request_id: &str,
    ) -> ClientResult<crate::types::PatchScopesRequestsApproveResponse> {
        let url = self.client.url(
            &format!(
                "/scopes/requests/{}/approve",
                crate::progenitor_support::encode_path(request_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deny access request.
     *
     * This function performs a `DELETE` to the `/scopes/requests/{request_id}` endpoint.
     *
     * **This endpoint allows you to deny an attempt to access your account.**
     *
     * **Note:** Only teammate admins may delete a teammate's access request.
     */
    pub async fn delete_scopes_requests_request(&self, request_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/scopes/requests/{}",
                crate::progenitor_support::encode_path(request_id),
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
     * Delete pending teammate.
     *
     * This function performs a `DELETE` to the `/teammates/pending/{token}` endpoint.
     *
     * **This endpoint allows you to delete a pending teammate invite.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_pending_token(&self, token: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/teammates/pending/{}",
                crate::progenitor_support::encode_path(token),
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

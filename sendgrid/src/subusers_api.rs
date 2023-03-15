use crate::Client;
use crate::ClientResult;

pub struct SubusersApi {
    pub client: Client,
}

impl SubusersApi {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SubusersApi { client }
    }

    /**
     * List all Subusers.
     *
     * This function performs a `GET` to the `/subusers` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all of your subusers.**
     *
     * You can choose to retrieve specific subusers as well as limit the results that come back from the API.
     *
     * **Parameters:**
     *
     * * `username: &str` -- The license key provided with your New Relic account.
     * * `limit: i64` -- The number of results you would like to get in each request.
     * * `offset: i64` -- The number of subusers to skip.
     */
    pub async fn get_subusers(
        &self,
        username: &str,
        limit: i64,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::Subuser>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !username.is_empty() {
            query_args.push(("username".to_string(), username.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/subusers?{}", query_), None);
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
     * List all Subusers.
     *
     * This function performs a `GET` to the `/subusers` endpoint.
     *
     * As opposed to `get_subusers`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a list of all of your subusers.**
     *
     * You can choose to retrieve specific subusers as well as limit the results that come back from the API.
     */
    pub async fn get_all_subusers(
        &self,
        username: &str,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::Subuser>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !username.is_empty() {
            query_args.push(("username".to_string(), username.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/subusers?{}", query_), None);
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
     * Create Subuser.
     *
     * This function performs a `POST` to the `/subusers` endpoint.
     *
     * **This endpoint allows you to create a new subuser.**
     */
    pub async fn post_subuser(
        &self,
        body: &crate::types::PostSubusersRequest,
    ) -> ClientResult<crate::types::SubuserPost> {
        let url = self.client.url("/subusers", None);
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
     * Delete a subuser.
     *
     * This function performs a `DELETE` to the `/subusers/{subuser_name}` endpoint.
     *
     * **This endpoint allows you to delete a subuser.**
     *
     * This is a permanent action. Once deleted, a subuser cannot be retrieved.
     */
    pub async fn delete_subusers_subuser_name(
        &self,
        subuser_name: &str,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/subusers/{}",
                crate::progenitor_support::encode_path(subuser_name),
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
     * Enable/disable a subuser.
     *
     * This function performs a `PATCH` to the `/subusers/{subuser_name}` endpoint.
     *
     * **This endpoint allows you to enable or disable a subuser.**
     */
    pub async fn patch_subusers_subuser_name(
        &self,
        subuser_name: &str,
        body: &crate::types::PatchSubusersSubuserNameRequest,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/subusers/{}",
                crate::progenitor_support::encode_path(subuser_name),
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
     * Retrieve Subuser Reputations.
     *
     * This function performs a `GET` to the `/subusers/reputations` endpoint.
     *
     * **This endpoint allows you to request the reputations for your subusers.**
     *
     * Subuser sender reputations give a good idea how well a sender is doing with regards to how recipients and recipient servers react to the mail that is being received. When a bounce, spam report, or other negative action happens on a sent email, it will affect your sender rating.
     *
     * **Parameters:**
     *
     * * `usernames: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_subusers_reputations(
        &self,
        usernames: &str,
    ) -> ClientResult<Vec<crate::types::GetSubusersReputationsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !usernames.is_empty() {
            query_args.push(("usernames".to_string(), usernames.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/subusers/reputations?{}", query_), None);
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
     * Retrieve Subuser Reputations.
     *
     * This function performs a `GET` to the `/subusers/reputations` endpoint.
     *
     * As opposed to `get_subusers_reputations`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to request the reputations for your subusers.**
     *
     * Subuser sender reputations give a good idea how well a sender is doing with regards to how recipients and recipient servers react to the mail that is being received. When a bounce, spam report, or other negative action happens on a sent email, it will affect your sender rating.
     */
    pub async fn get_all_subusers_reputations(
        &self,
        usernames: &str,
    ) -> ClientResult<Vec<crate::types::GetSubusersReputationsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !usernames.is_empty() {
            query_args.push(("usernames".to_string(), usernames.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/subusers/reputations?{}", query_), None);
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
     * Update IPs assigned to a subuser.
     *
     * This function performs a `PUT` to the `/subusers/{subuser_name}/ips` endpoint.
     *
     * **This endpoint allows you update your subusers' assigned IP.**
     *
     * Each subuser should be assigned to an IP address from which all of this subuser's mail will be sent. Often, this is the same IP as the parent account, but each subuser can have one or more of their own IP addresses as well.
     *
     * More information:
     *
     * * [How to request more IPs](https://sendgrid.com/docs/ui/account-and-settings/dedicated-ip-addresses/)
     * * [Setup Reverse DNS](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-reverse-dns/)
     */
    pub async fn put_subusers_subuser_name_ips(
        &self,
        subuser_name: &str,
        body: &[std::net::Ipv4Addr],
    ) -> ClientResult<crate::types::PutSubusersSubuserNameIpsResponse> {
        let url = self.client.url(
            &format!(
                "/subusers/{}/ips",
                crate::progenitor_support::encode_path(subuser_name),
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

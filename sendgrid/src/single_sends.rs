use crate::Client;
use crate::ClientResult;

pub struct SingleSends {
    pub client: Client,
}

impl SingleSends {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SingleSends { client }
    }

    /**
     * Get All Single Sends.
     *
     * This function performs a `GET` to the `/marketing/singlesends` endpoint.
     *
     * **This endpoint allows you to retrieve all your Single Sends.**
     *
     * Returns all of your Single Sends with condensed details about each, including the Single Sends' IDs. For more details about an individual Single Send, pass the Single Send's ID to the `/marketing/singlesends/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `page_size: i64`
     * * `page_token: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_marketing_singlesends(
        &self,
        page_size: i64,
        page_token: &str,
    ) -> ClientResult<crate::types::GetMarketingSinglesendsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/marketing/singlesends?{}", query_), None);
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
     * Create Single Send.
     *
     * This function performs a `POST` to the `/marketing/singlesends` endpoint.
     *
     * **This endpoint allows you to create a new Single Send.**
     *
     * Please note that if you are migrating from the previous version of Single Sends, you no longer need to pass a template ID with your request to this endpoint. Instead, you will pass all template data in the `email_config` object.
     */
    pub async fn post_marketing_singlesend(
        &self,
        body: &crate::types::SinglesendRequest,
    ) -> ClientResult<crate::types::SinglesendResponseAllOf> {
        let url = self.client.url("/marketing/singlesends", None);
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
     * Bulk Delete Single Sends.
     *
     * This function performs a `DELETE` to the `/marketing/singlesends` endpoint.
     *
     * **This endpoint allows you to delete multiple Single Sends using an array of Single Sends IDs.**
     *
     * To first retrieve all your Single Sends' IDs, you can make a GET request to the `/marketing/singlensends` endpoint.
     *
     * Please note that a DELETE request is permanent, and your Single Sends will not be recoverable after deletion.
     *
     * **Parameters:**
     *
     * * `ids: &[String]` -- The recipient IDs of the recipients that already existed from this request.
     */
    pub async fn delete_marketing_singlesends(&self, ids: &[String]) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/marketing/singlesends?{}", query_), None);
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
     * Get Single Send by ID.
     *
     * This function performs a `GET` to the `/marketing/singlesends/{id}` endpoint.
     *
     * **This endpoint allows you to retrieve details about one Single Send using a Single Send ID.**
     *
     * You can retrieve all of your Single Sends by making a GET request to the `/marketing/singlesends` endpoint.
     */
    pub async fn get_marketing_singlesend(
        &self,
        id: &str,
    ) -> ClientResult<crate::types::SinglesendResponseAllOf> {
        let url = self.client.url(
            &format!(
                "/marketing/singlesends/{}",
                crate::progenitor_support::encode_path(id),
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
     * Duplicate Single Send.
     *
     * This function performs a `POST` to the `/marketing/singlesends/{id}` endpoint.
     *
     * **This endpoint allows you to duplicate an existing Single Send using its Single Send ID.**
     *
     * Duplicating a Single Send is useful when you want to create a Single Send but don't want to start from scratch. Once duplicated, you can update or edit the Single Send by making a PATCH request to the `/marketing/singlesends/{id}` endpoint.
     *  
     * If you leave the `name` field blank, your duplicate will be assigned the name of the Single Send it was copied from with the text “Copy of ” prepended to it. The `name` field length is limited to 100 characters, so the end of the new Single Send name, including “Copy of ”, will be trimmed if the name exceeds this limit.
     */
    pub async fn post_marketing_singlesend_single_sends(
        &self,
        id: &str,
        body: &crate::types::PostMarketingSinglesendsRequest,
    ) -> ClientResult<crate::types::SinglesendResponseAllOf> {
        let url = self.client.url(
            &format!(
                "/marketing/singlesends/{}",
                crate::progenitor_support::encode_path(id),
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
     * Delete Single Send by ID.
     *
     * This function performs a `DELETE` to the `/marketing/singlesends/{id}` endpoint.
     *
     * **This endpoint allows you to delete one Single Send using a Single Send ID.**
     *
     * To first retrieve all your Single Sends' IDs, you can make a GET request to the `/marketing/singlensends` endpoint.
     *
     * Please note that a `DELETE` request is permanent, and your Single Send will not be recoverable after deletion.
     */
    pub async fn delete_marketing_singlesends_single_sends(&self, id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/marketing/singlesends/{}",
                crate::progenitor_support::encode_path(id),
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
     * Update Single Send.
     *
     * This function performs a `PATCH` to the `/marketing/singlesends/{id}` endpoint.
     *
     * **This endpoint allows you to update a Single Send using a Single Send ID.**
     *
     * You only need to pass the fields you want to update. Any blank/missing fields will remain unaltered.
     */
    pub async fn patch_marketing_singlesends(
        &self,
        id: &str,
        body: &crate::types::SinglesendRequest,
    ) -> ClientResult<crate::types::SinglesendResponseAllOf> {
        let url = self.client.url(
            &format!(
                "/marketing/singlesends/{}",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get Single Sends Search.
     *
     * This function performs a `POST` to the `/marketing/singlesends/search` endpoint.
     *
     * **This endpoint allows you to search for Single Sends based on specified criteria.**
     *
     * You can search for Single Sends by passing a combination of values using the `name`, `status`, and `categories` request body fields.
     *
     * For example, if you want to search for all Single Sends that are "drafts" or "scheduled" and also associated with the category "shoes," your request body may look like the example below.
     *
     * ```javascript
     * {
     *   "status": [
     *     "draft",
     *     "scheduled"
     *   ],
     *   "categories": [
     *     "shoes"
     *   ],
     * }
     * ```
     *
     * **Parameters:**
     *
     * * `page_size: i64`
     * * `page_token: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_marketing_singlesends_search(
        &self,
        page_size: i64,
        page_token: &str,
        body: &crate::types::SinglesendSearch,
    ) -> ClientResult<crate::types::GetMarketingSinglesendsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/marketing/singlesends/search?{}", query_), None);
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
     * Schedule Single Send.
     *
     * This function performs a `PUT` to the `/marketing/singlesends/{id}/schedule` endpoint.
     *
     * **This endpoint allows you to schedule a Single Send for future delivery using a Single Send ID.**
     *
     * To schedule a Single Send, you must pass a date string in ISO 8601 time format (yyyy-MM-ddTHH:mm:ssZ)  using the required `send_at` field. For example, the ISO 8601 format for 9:00 AM UTC on May 6, 2020 would be `2020-05-06T09:00:00Z`. You may also pass the string `"now"` to send the Single Send immediately.
     */
    pub async fn put_marketing_singlesends_schedule(
        &self,
        id: &str,
        body: &crate::types::PutMarketingSinglesendsScheduleRequest,
    ) -> ClientResult<crate::types::PutMarketingSinglesendsScheduleResponse> {
        let url = self.client.url(
            &format!(
                "/marketing/singlesends/{}/schedule",
                crate::progenitor_support::encode_path(id),
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
     * Delete Single Send Schedule.
     *
     * This function performs a `DELETE` to the `/marketing/singlesends/{id}/schedule` endpoint.
     *
     * **This endpoint allows you to cancel a scheduled Single Send using a Single Send ID.**
     *
     * Making a DELETE request to this endpoint will cancel the scheduled sending of a Single Send. The request will not delete the Single Send itself. Deleting a Single Send can be done by passing a DELETE request to `/marketing/singlesends/{id}`.
     */
    pub async fn delete_marketing_singlesends_schedule(
        &self,
        id: &str,
    ) -> ClientResult<crate::types::SinglesendSchedule> {
        let url = self.client.url(
            &format!(
                "/marketing/singlesends/{}/schedule",
                crate::progenitor_support::encode_path(id),
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
     * Get All Categories.
     *
     * This function performs a `GET` to the `/marketing/singlesends/categories` endpoint.
     *
     * **This endpoint allows you to retrieve all the categories associated with your Single Sends.**
     *
     * This endpoint will return your latest 1,000 categories.
     */
    pub async fn get_marketing_singlesends_categories(
        &self,
    ) -> ClientResult<crate::types::GetMarketingSinglesendsCategoriesResponse> {
        let url = self.client.url("/marketing/singlesends/categories", None);
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

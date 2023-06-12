use crate::Client;
use crate::ClientResult;

pub struct Activity {
    pub client: Client,
}

impl Activity {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Activity { client }
    }

    /**
     * List public events.
     *
     * This function performs a `GET` to the `/events` endpoint.
     *
     * We delay the public events feed by five minutes, which means the most recent event returned by the public events API actually occurred at least five minutes ago.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-events>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public_events(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/events?{}", query_), None);
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
     * List public events.
     *
     * This function performs a `GET` to the `/events` endpoint.
     *
     * As opposed to `list_public_events`, this function returns all the pages of the request at once.
     *
     * We delay the public events feed by five minutes, which means the most recent event returned by the public events API actually occurred at least five minutes ago.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-events>
     */
    pub async fn list_all_public_events(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let url = self.client.url("/events", None);
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
     * Get feeds.
     *
     * This function performs a `GET` to the `/feeds` endpoint.
     *
     * GitHub provides several timeline resources in [Atom](http://en.wikipedia.org/wiki/Atom_(standard)) format. The Feeds API lists all the feeds available to the authenticated user:
     *
     * *   **Timeline**: The GitHub global public timeline
     * *   **User**: The public timeline for any user, using [URI template](https://docs.github.com/rest/overview/resources-in-the-rest-api#hypermedia)
     * *   **Current user public**: The public timeline for the authenticated user
     * *   **Current user**: The private timeline for the authenticated user
     * *   **Current user actor**: The private timeline for activity created by the authenticated user
     * *   **Current user organizations**: The private timeline for the organizations the authenticated user is a member of.
     * *   **Security advisories**: A collection of public announcements that provide information about security-related vulnerabilities in software on GitHub.
     *
     * **Note**: Private feeds are only returned when [authenticating via Basic Auth](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) since current feed URIs use the older, non revocable auth tokens.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#get-feeds>
     */
    pub async fn get_feeds(&self) -> ClientResult<crate::Response<crate::types::Feed>> {
        let url = self.client.url("/feeds", None);
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
     * List public events for a network of repositories.
     *
     * This function performs a `GET` to the `/networks/{owner}/{repo}/events` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-events-for-a-network-of-repositories>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public_events_for_repo_network(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/networks/{}/{}/events?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List public events for a network of repositories.
     *
     * This function performs a `GET` to the `/networks/{owner}/{repo}/events` endpoint.
     *
     * As opposed to `list_public_events_for_repo_network`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-events-for-a-network-of-repositories>
     */
    pub async fn list_all_public_events_for_repo_network(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let url = self.client.url(
            &format!(
                "/networks/{}/{}/events",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List notifications for the authenticated user.
     *
     * This function performs a `GET` to the `/notifications` endpoint.
     *
     * List all notifications for the current user, sorted by most recently updated.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-notifications-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `all: bool` -- If `true`, show notifications marked as read.
     * * `participating: bool` -- If `true`, only shows notifications in which the user is directly participating or mentioned.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `before: chrono::DateTime<chrono::Utc>` -- Only show notifications updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_notifications_for_authenticated_user(
        &self,
        all: bool,
        participating: bool,
        since: Option<chrono::DateTime<chrono::Utc>>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Thread>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if all {
            query_args.push(("all".to_string(), all.to_string()));
        }
        if let Some(date) = before {
            query_args.push(("before".to_string(), date.to_rfc3339()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if participating {
            query_args.push(("participating".to_string(), participating.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/notifications?{}", query_), None);
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
     * List notifications for the authenticated user.
     *
     * This function performs a `GET` to the `/notifications` endpoint.
     *
     * As opposed to `list_notifications_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * List all notifications for the current user, sorted by most recently updated.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-notifications-for-the-authenticated-user>
     */
    pub async fn list_all_notifications_for_authenticated_user(
        &self,
        all: bool,
        participating: bool,
        since: Option<chrono::DateTime<chrono::Utc>>,
        before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::Thread>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if all {
            query_args.push(("all".to_string(), all.to_string()));
        }
        if let Some(date) = before {
            query_args.push(("before".to_string(), date.to_rfc3339()));
        }
        if participating {
            query_args.push(("participating".to_string(), participating.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/notifications?{}", query_), None);
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
     * Mark notifications as read.
     *
     * This function performs a `PUT` to the `/notifications` endpoint.
     *
     * Marks all notifications as "read" removes it from the [default view on GitHub](https://github.com/notifications). If the number of notifications is too large to complete in one request, you will receive a `202 Accepted` status and GitHub will run an asynchronous process to mark notifications as "read." To check whether any "unread" notifications remain, you can use the [List notifications for the authenticated user](https://docs.github.com/rest/reference/activity#list-notifications-for-the-authenticated-user) endpoint and pass the query parameter `all=false`.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#mark-notifications-as-read>
     */
    pub async fn mark_notifications_as_read(
        &self,
        body: &crate::types::ActivityMarkNotificationsAsReadRequest,
    ) -> ClientResult<crate::Response<crate::types::Error>> {
        let url = self.client.url("/notifications", None);
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
     * Get a thread.
     *
     * This function performs a `GET` to the `/notifications/threads/{thread_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#get-a-thread>
     *
     * **Parameters:**
     *
     * * `thread_id: i64` -- thread_id parameter.
     */
    pub async fn get_thread(
        &self,
        thread_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Thread>> {
        let url = self.client.url(
            &format!(
                "/notifications/threads/{}",
                crate::progenitor_support::encode_path(&thread_id.to_string()),
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
     * Mark a thread as read.
     *
     * This function performs a `PATCH` to the `/notifications/threads/{thread_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#mark-a-thread-as-read>
     *
     * **Parameters:**
     *
     * * `thread_id: i64` -- thread_id parameter.
     */
    pub async fn mark_thread_as_read(&self, thread_id: i64) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/notifications/threads/{}",
                crate::progenitor_support::encode_path(&thread_id.to_string()),
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
     * Get a thread subscription for the authenticated user.
     *
     * This function performs a `GET` to the `/notifications/threads/{thread_id}/subscription` endpoint.
     *
     * This checks to see if the current user is subscribed to a thread. You can also [get a repository subscription](https://docs.github.com/rest/reference/activity#get-a-repository-subscription).
     *
     * Note that subscriptions are only generated if a user is participating in a conversation--for example, they've replied to the thread, were **@mentioned**, or manually subscribe to a thread.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#get-a-thread-subscription-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `thread_id: i64` -- thread_id parameter.
     */
    pub async fn get_thread_subscription_for_authenticated_user(
        &self,
        thread_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ThreadSubscription>> {
        let url = self.client.url(
            &format!(
                "/notifications/threads/{}/subscription",
                crate::progenitor_support::encode_path(&thread_id.to_string()),
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
     * Set a thread subscription.
     *
     * This function performs a `PUT` to the `/notifications/threads/{thread_id}/subscription` endpoint.
     *
     * If you are watching a repository, you receive notifications for all threads by default. Use this endpoint to ignore future notifications for threads until you comment on the thread or get an **@mention**.
     *
     * You can also use this endpoint to subscribe to threads that you are currently not receiving notifications for or to subscribed to threads that you have previously ignored.
     *
     * Unsubscribing from a conversation in a repository that you are not watching is functionally equivalent to the [Delete a thread subscription](https://docs.github.com/rest/reference/activity#delete-a-thread-subscription) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#set-a-thread-subscription>
     *
     * **Parameters:**
     *
     * * `thread_id: i64` -- thread_id parameter.
     */
    pub async fn set_thread_subscription(
        &self,
        thread_id: i64,
        body: &crate::types::ActivitySetThreadSubscriptionRequest,
    ) -> ClientResult<crate::Response<crate::types::ThreadSubscription>> {
        let url = self.client.url(
            &format!(
                "/notifications/threads/{}/subscription",
                crate::progenitor_support::encode_path(&thread_id.to_string()),
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
     * Delete a thread subscription.
     *
     * This function performs a `DELETE` to the `/notifications/threads/{thread_id}/subscription` endpoint.
     *
     * Mutes all future notifications for a conversation until you comment on the thread or get an **@mention**. If you are watching the repository of the thread, you will still receive notifications. To ignore future notifications for a repository you are watching, use the [Set a thread subscription](https://docs.github.com/rest/reference/activity#set-a-thread-subscription) endpoint and set `ignore` to `true`.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#delete-a-thread-subscription>
     *
     * **Parameters:**
     *
     * * `thread_id: i64` -- thread_id parameter.
     */
    pub async fn delete_thread_subscription(
        &self,
        thread_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/notifications/threads/{}/subscription",
                crate::progenitor_support::encode_path(&thread_id.to_string()),
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
     * List public organization events.
     *
     * This function performs a `GET` to the `/orgs/{org}/events` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-organization-events>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public_org_events(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/events?{}",
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
     * List public organization events.
     *
     * This function performs a `GET` to the `/orgs/{org}/events` endpoint.
     *
     * As opposed to `list_public_org_events`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-organization-events>
     */
    pub async fn list_all_public_org_events(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/events",
                crate::progenitor_support::encode_path(org),
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
     * List repository events.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/events` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repository-events>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repo_events(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/events?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List repository events.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/events` endpoint.
     *
     * As opposed to `list_repo_events`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repository-events>
     */
    pub async fn list_all_repo_events(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/events",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List repository notifications for the authenticated user.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/notifications` endpoint.
     *
     * List all notifications for the current user.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repository-notifications-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `all: bool` -- If `true`, show notifications marked as read.
     * * `participating: bool` -- If `true`, only shows notifications in which the user is directly participating or mentioned.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `before: chrono::DateTime<chrono::Utc>` -- Only show notifications updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repo_notifications_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
        all: bool,
        participating: bool,
        since: Option<chrono::DateTime<chrono::Utc>>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Thread>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if all {
            query_args.push(("all".to_string(), all.to_string()));
        }
        if let Some(date) = before {
            query_args.push(("before".to_string(), date.to_rfc3339()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if participating {
            query_args.push(("participating".to_string(), participating.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/notifications?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List repository notifications for the authenticated user.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/notifications` endpoint.
     *
     * As opposed to `list_repo_notifications_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * List all notifications for the current user.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repository-notifications-for-the-authenticated-user>
     */
    pub async fn list_all_repo_notifications_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
        all: bool,
        participating: bool,
        since: Option<chrono::DateTime<chrono::Utc>>,
        before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::Thread>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if all {
            query_args.push(("all".to_string(), all.to_string()));
        }
        if let Some(date) = before {
            query_args.push(("before".to_string(), date.to_rfc3339()));
        }
        if participating {
            query_args.push(("participating".to_string(), participating.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/notifications?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                query_
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
     * Mark repository notifications as read.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/notifications` endpoint.
     *
     * Marks all notifications in a repository as "read" removes them from the [default view on GitHub](https://github.com/notifications). If the number of notifications is too large to complete in one request, you will receive a `202 Accepted` status and GitHub will run an asynchronous process to mark notifications as "read." To check whether any "unread" notifications remain, you can use the [List repository notifications for the authenticated user](https://docs.github.com/rest/reference/activity#list-repository-notifications-for-the-authenticated-user) endpoint and pass the query parameter `all=false`.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#mark-repository-notifications-as-read>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn mark_repo_notifications_as_read(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActivityMarkRepoNotificationsAsReadRequest,
    ) -> ClientResult<crate::Response<crate::types::PullsUpdateBranchResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/notifications",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List stargazers.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stargazers` endpoint.
     *
     * Lists the people that have starred the repository.
     *
     * You can also find out _when_ stars were created by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-stargazers>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_stargazers_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActivityListStargazersRepoResponseAnyOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stargazers?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List watchers.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/subscribers` endpoint.
     *
     * Lists the people watching the specified repository.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-watchers>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_watchers_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/subscribers?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List watchers.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/subscribers` endpoint.
     *
     * As opposed to `list_watchers_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists the people watching the specified repository.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-watchers>
     */
    pub async fn list_all_watchers_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/subscribers",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a repository subscription.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/subscription` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#get-a-repository-subscription>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_repo_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::RepositorySubscription>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/subscription",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Set a repository subscription.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/subscription` endpoint.
     *
     * If you would like to watch a repository, set `subscribed` to `true`. If you would like to ignore notifications made within a repository, set `ignored` to `true`. If you would like to stop watching a repository, [delete the repository's subscription](https://docs.github.com/rest/reference/activity#delete-a-repository-subscription) completely.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#set-a-repository-subscription>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn set_repo_subscription(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActivitySetRepoSubscriptionRequest,
    ) -> ClientResult<crate::Response<crate::types::RepositorySubscription>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/subscription",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Delete a repository subscription.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/subscription` endpoint.
     *
     * This endpoint should only be used to stop watching a repository. To control whether or not you wish to receive notifications from a repository, [set the repository's subscription manually](https://docs.github.com/rest/reference/activity#set-a-repository-subscription).
     *
     * FROM: <https://docs.github.com/rest/reference/activity#delete-a-repository-subscription>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn delete_repo_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/subscription",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List repositories starred by the authenticated user.
     *
     * This function performs a `GET` to the `/user/starred` endpoint.
     *
     * Lists repositories the authenticated user has starred.
     *
     * You can also find out _when_ stars were created by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repositories-starred-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `sort: crate::types::Sort` -- One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repos_starred_by_authenticated_user(
        &self,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Repository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/starred?{}", query_), None);
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
     * List repositories starred by the authenticated user.
     *
     * This function performs a `GET` to the `/user/starred` endpoint.
     *
     * As opposed to `list_repos_starred_by_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists repositories the authenticated user has starred.
     *
     * You can also find out _when_ stars were created by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repositories-starred-by-the-authenticated-user>
     */
    pub async fn list_all_repos_starred_by_authenticated_user(
        &self,
        sort: crate::types::Sort,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::Repository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/starred?{}", query_), None);
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
     * Check if a repository is starred by the authenticated user.
     *
     * This function performs a `GET` to the `/user/starred/{owner}/{repo}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#check-if-a-repository-is-starred-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn check_repo_is_starred_by_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/starred/{}/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Star a repository for the authenticated user.
     *
     * This function performs a `PUT` to the `/user/starred/{owner}/{repo}` endpoint.
     *
     * Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * FROM: <https://docs.github.com/rest/reference/activity#star-a-repository-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn star_repo_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/starred/{}/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Unstar a repository for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/starred/{owner}/{repo}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#unstar-a-repository-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn unstar_repo_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/starred/{}/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List repositories watched by the authenticated user.
     *
     * This function performs a `GET` to the `/user/subscriptions` endpoint.
     *
     * Lists repositories the authenticated user is watching.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repositories-watched-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_watched_repos_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/subscriptions?{}", query_), None);
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
     * List repositories watched by the authenticated user.
     *
     * This function performs a `GET` to the `/user/subscriptions` endpoint.
     *
     * As opposed to `list_watched_repos_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists repositories the authenticated user is watching.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repositories-watched-by-the-authenticated-user>
     */
    pub async fn list_all_watched_repos_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let url = self.client.url("/user/subscriptions", None);
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
     * List events for the authenticated user.
     *
     * This function performs a `GET` to the `/users/{username}/events` endpoint.
     *
     * If you are authenticated as the given user, you will see your private events. Otherwise, you'll only see public events.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-events-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_events_for_authenticated_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events?{}",
                crate::progenitor_support::encode_path(username),
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
     * List events for the authenticated user.
     *
     * This function performs a `GET` to the `/users/{username}/events` endpoint.
     *
     * As opposed to `list_events_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * If you are authenticated as the given user, you will see your private events. Otherwise, you'll only see public events.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-events-for-the-authenticated-user>
     */
    pub async fn list_all_events_for_authenticated_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/events",
                crate::progenitor_support::encode_path(username),
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
     * List organization events for the authenticated user.
     *
     * This function performs a `GET` to the `/users/{username}/events/orgs/{org}` endpoint.
     *
     * This is the user's organization dashboard. You must be authenticated as the user to view this.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-organization-events-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_org_events_for_authenticated_user(
        &self,
        username: &str,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/orgs/{}?{}",
                crate::progenitor_support::encode_path(username),
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
     * List organization events for the authenticated user.
     *
     * This function performs a `GET` to the `/users/{username}/events/orgs/{org}` endpoint.
     *
     * As opposed to `list_org_events_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * This is the user's organization dashboard. You must be authenticated as the user to view this.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-organization-events-for-the-authenticated-user>
     */
    pub async fn list_all_org_events_for_authenticated_user(
        &self,
        username: &str,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/orgs/{}",
                crate::progenitor_support::encode_path(username),
                crate::progenitor_support::encode_path(org),
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
     * List public events for a user.
     *
     * This function performs a `GET` to the `/users/{username}/events/public` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-events-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public_events_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/public?{}",
                crate::progenitor_support::encode_path(username),
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
     * List public events for a user.
     *
     * This function performs a `GET` to the `/users/{username}/events/public` endpoint.
     *
     * As opposed to `list_public_events_for_user`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-events-for-a-user>
     */
    pub async fn list_all_public_events_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/public",
                crate::progenitor_support::encode_path(username),
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
     * List events received by the authenticated user.
     *
     * This function performs a `GET` to the `/users/{username}/received_events` endpoint.
     *
     * These are events that you've received by watching repos and following users. If you are authenticated as the given user, you will see private events. Otherwise, you'll only see public events.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-events-received-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_received_events_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/received_events?{}",
                crate::progenitor_support::encode_path(username),
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
     * List events received by the authenticated user.
     *
     * This function performs a `GET` to the `/users/{username}/received_events` endpoint.
     *
     * As opposed to `list_received_events_for_user`, this function returns all the pages of the request at once.
     *
     * These are events that you've received by watching repos and following users. If you are authenticated as the given user, you will see private events. Otherwise, you'll only see public events.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-events-received-by-the-authenticated-user>
     */
    pub async fn list_all_received_events_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/received_events",
                crate::progenitor_support::encode_path(username),
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
     * List public events received by a user.
     *
     * This function performs a `GET` to the `/users/{username}/received_events/public` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-events-received-by-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_received_public_events_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/received_events/public?{}",
                crate::progenitor_support::encode_path(username),
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
     * List public events received by a user.
     *
     * This function performs a `GET` to the `/users/{username}/received_events/public` endpoint.
     *
     * As opposed to `list_received_public_events_for_user`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-public-events-received-by-a-user>
     */
    pub async fn list_all_received_public_events_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/received_events/public",
                crate::progenitor_support::encode_path(username),
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
     * List repositories starred by a user.
     *
     * This function performs a `GET` to the `/users/{username}/starred` endpoint.
     *
     * Lists repositories a user has starred.
     *
     * You can also find out _when_ stars were created by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repositories-starred-by-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `sort: crate::types::Sort` -- One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repos_starred_by_user(
        &self,
        username: &str,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActivityListReposStarredByUserResponseAnyOf>>
    {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/starred?{}",
                crate::progenitor_support::encode_path(username),
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
     * List repositories watched by a user.
     *
     * This function performs a `GET` to the `/users/{username}/subscriptions` endpoint.
     *
     * Lists repositories a user is watching.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repositories-watched-by-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repos_watched_by_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/subscriptions?{}",
                crate::progenitor_support::encode_path(username),
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
     * List repositories watched by a user.
     *
     * This function performs a `GET` to the `/users/{username}/subscriptions` endpoint.
     *
     * As opposed to `list_repos_watched_by_user`, this function returns all the pages of the request at once.
     *
     * Lists repositories a user is watching.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repositories-watched-by-a-user>
     */
    pub async fn list_all_repos_watched_by_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/subscriptions",
                crate::progenitor_support::encode_path(username),
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
}

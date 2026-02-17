use crate::Client;
use crate::ClientResult;

pub struct Issues {
    pub client: Client,
}

impl Issues {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Issues { client }
    }

    /**
     * List issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/issues` endpoint.
     *
     * List issues assigned to the authenticated user across all visible repositories including owned repositories, member
     * repositories, and organization repositories. You can use the `filter` query parameter to fetch issues that are not
     * necessarily assigned to you.
     *
     * > [!NOTE]
     * > GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#list-issues-assigned-to-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `filter: crate::types::Filter` -- Indicates which sorts of issues to return. `assigned` means issues assigned to you. `created` means issues created by you. `mentioned` means issues mentioning you. `subscribed` means issues you're subscribed to updates for. `all` or `repos` means all issues you can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `collab: bool`
     * * `orgs: bool`
     * * `owned: bool`
     * * `pulls: bool`
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list(
        &self,
        filter: crate::types::Filter,
        state: crate::types::IssuesListState,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        collab: bool,
        orgs: bool,
        owned: bool,
        pulls: bool,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if collab {
            query_args.push(("collab".to_string(), collab.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !labels.is_empty() {
            query_args.push(("labels".to_string(), labels.to_string()));
        }
        if orgs {
            query_args.push(("orgs".to_string(), orgs.to_string()));
        }
        if owned {
            query_args.push(("owned".to_string(), owned.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if pulls {
            query_args.push(("pulls".to_string(), pulls.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/issues?{}", query_), None);
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
     * List issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/issues` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * List issues assigned to the authenticated user across all visible repositories including owned repositories, member
     * repositories, and organization repositories. You can use the `filter` query parameter to fetch issues that are not
     * necessarily assigned to you.
     *
     * > [!NOTE]
     * > GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#list-issues-assigned-to-the-authenticated-user>
     */
    pub async fn list_all(
        &self,
        filter: crate::types::Filter,
        state: crate::types::IssuesListState,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        collab: bool,
        orgs: bool,
        owned: bool,
        pulls: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if collab {
            query_args.push(("collab".to_string(), collab.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !labels.is_empty() {
            query_args.push(("labels".to_string(), labels.to_string()));
        }
        if orgs {
            query_args.push(("orgs".to_string(), orgs.to_string()));
        }
        if owned {
            query_args.push(("owned".to_string(), owned.to_string()));
        }
        if pulls {
            query_args.push(("pulls".to_string(), pulls.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/issues?{}", query_), None);
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
     * List organization issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/orgs/{org}/issues` endpoint.
     *
     * List issues in an organization assigned to the authenticated user.
     *
     * > [!NOTE]
     * > GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#list-organization-issues-assigned-to-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `filter: crate::types::Filter` -- Indicates which sorts of issues to return. `assigned` means issues assigned to you. `created` means issues created by you. `mentioned` means issues mentioning you. `subscribed` means issues you're subscribed to updates for. `all` or `repos` means all issues you can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `type_: &str` -- Can be the name of an issue type.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_org(
        &self,
        org: &str,
        filter: crate::types::Filter,
        state: crate::types::IssuesListState,
        labels: &str,
        type_: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !labels.is_empty() {
            query_args.push(("labels".to_string(), labels.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/issues?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List organization issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/orgs/{org}/issues` endpoint.
     *
     * As opposed to `list_for_org`, this function returns all the pages of the request at once.
     *
     * List issues in an organization assigned to the authenticated user.
     *
     * > [!NOTE]
     * > GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#list-organization-issues-assigned-to-the-authenticated-user>
     */
    pub async fn list_all_for_org(
        &self,
        org: &str,
        filter: crate::types::Filter,
        state: crate::types::IssuesListState,
        labels: &str,
        type_: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !labels.is_empty() {
            query_args.push(("labels".to_string(), labels.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/issues?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List assignees.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/assignees` endpoint.
     *
     * Lists the [available assignees](https://docs.github.com/articles/assigning-issues-and-pull-requests-to-other-github-users/) for issues in a repository.
     *
     * FROM: <https://docs.github.com/rest/issues/assignees#list-assignees>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_assignees(
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
                "/repos/{}/{}/assignees?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List assignees.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/assignees` endpoint.
     *
     * As opposed to `list_assignees`, this function returns all the pages of the request at once.
     *
     * Lists the [available assignees](https://docs.github.com/articles/assigning-issues-and-pull-requests-to-other-github-users/) for issues in a repository.
     *
     * FROM: <https://docs.github.com/rest/issues/assignees#list-assignees>
     */
    pub async fn list_all_assignees(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/assignees",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Check if a user can be assigned.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/assignees/{assignee}` endpoint.
     *
     * Checks if a user has permission to be assigned to an issue in this repository.
     *
     * If the `assignee` can be assigned to issues in the repository, a `204` header with no content is returned.
     *
     * Otherwise a `404` status code is returned.
     *
     * FROM: <https://docs.github.com/rest/issues/assignees#check-if-a-user-can-be-assigned>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `assignee: &str`
     */
    pub async fn check_user_can_be_assigned(
        &self,
        owner: &str,
        repo: &str,
        assignee: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/assignees/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&assignee.to_string()),
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
     * List repository issues.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues` endpoint.
     *
     * List issues in a repository. Only open issues will be listed.
     *
     * > [!NOTE]
     * > GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#list-repository-issues>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `milestone: &str` -- If an `integer` is passed, it should refer to a milestone by its `number` field. If the string `*` is passed, issues with any milestone are accepted. If the string `none` is passed, issues without milestones are returned.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return.
     * * `assignee: &str` -- Can be the name of a user. Pass in `none` for issues with no assigned user, and `*` for issues assigned to any user.
     * * `type_: &str` -- Can be the name of an issue type. If the string `*` is passed, issues with any type are accepted. If the string `none` is passed, issues without type are returned.
     * * `creator: &str` -- The user that created the issue.
     * * `mentioned: &str` -- A user that's mentioned in the issue.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_repo(
        &self,
        owner: &str,
        repo: &str,
        milestone: &str,
        state: crate::types::IssuesListState,
        assignee: &str,
        type_: &str,
        creator: &str,
        mentioned: &str,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !creator.is_empty() {
            query_args.push(("creator".to_string(), creator.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !labels.is_empty() {
            query_args.push(("labels".to_string(), labels.to_string()));
        }
        if !mentioned.is_empty() {
            query_args.push(("mentioned".to_string(), mentioned.to_string()));
        }
        if !milestone.is_empty() {
            query_args.push(("milestone".to_string(), milestone.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List repository issues.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues` endpoint.
     *
     * As opposed to `list_for_repo`, this function returns all the pages of the request at once.
     *
     * List issues in a repository. Only open issues will be listed.
     *
     * > [!NOTE]
     * > GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#list-repository-issues>
     */
    pub async fn list_all_for_repo(
        &self,
        owner: &str,
        repo: &str,
        milestone: &str,
        state: crate::types::IssuesListState,
        assignee: &str,
        type_: &str,
        creator: &str,
        mentioned: &str,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !creator.is_empty() {
            query_args.push(("creator".to_string(), creator.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !labels.is_empty() {
            query_args.push(("labels".to_string(), labels.to_string()));
        }
        if !mentioned.is_empty() {
            query_args.push(("mentioned".to_string(), mentioned.to_string()));
        }
        if !milestone.is_empty() {
            query_args.push(("milestone".to_string(), milestone.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Create an issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues` endpoint.
     *
     * Any user with pull access to a repository can create an issue. If [issues are disabled in the repository](https://docs.github.com/articles/disabling-issues/), the API returns a `410 Gone` status.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#create-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::IssuesCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List issue comments for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments` endpoint.
     *
     * You can use the REST API to list comments on issues and pull requests for a repository. Every pull request is an issue, but not every issue is a pull request.
     *
     * By default, issue comments are ordered by ascending ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#list-issue-comments-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `sort: crate::types::SortData` -- The property to sort the results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::SortData,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueCommentData>>> {
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
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List issue comments for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments` endpoint.
     *
     * As opposed to `list_comments_for_repo`, this function returns all the pages of the request at once.
     *
     * You can use the REST API to list comments on issues and pull requests for a repository. Every pull request is an issue, but not every issue is a pull request.
     *
     * By default, issue comments are ordered by ascending ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#list-issue-comments-for-a-repository>
     */
    pub async fn list_all_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::SortData,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueCommentData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get an issue comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}` endpoint.
     *
     * You can use the REST API to get comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#get-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn get_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<crate::types::IssueCommentData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Delete an issue comment.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}` endpoint.
     *
     * You can use the REST API to delete comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#delete-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn delete_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Update an issue comment.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}` endpoint.
     *
     * You can use the REST API to update comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#update-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn update_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::IssueCommentData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Pin an issue comment.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/pin` endpoint.
     *
     * You can use the REST API to pin comments on issues.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#pin-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn pin_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<crate::types::IssueCommentData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/pin",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Unpin an issue comment.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/pin` endpoint.
     *
     * You can use the REST API to unpin comments on issues.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#unpin-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn unpin_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/pin",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * List issue events for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/events` endpoint.
     *
     * Lists events for a repository.
     *
     * FROM: <https://docs.github.com/rest/issues/events#list-issue-events-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_events_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueEvent>>> {
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
                "/repos/{}/{}/issues/events?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List issue events for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/events` endpoint.
     *
     * As opposed to `list_events_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists events for a repository.
     *
     * FROM: <https://docs.github.com/rest/issues/events#list-issue-events-for-a-repository>
     */
    pub async fn list_all_events_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueEvent>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/events",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get an issue event.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/events/{event_id}` endpoint.
     *
     * Gets a single event by the event id.
     *
     * FROM: <https://docs.github.com/rest/issues/events#get-an-issue-event>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `event_id: i64`
     */
    pub async fn get_event(
        &self,
        owner: &str,
        repo: &str,
        event_id: i64,
    ) -> ClientResult<crate::Response<crate::types::IssueEvent>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/events/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
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
     * Get an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}` endpoint.
     *
     * The API returns a [`301 Moved Permanently` status](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api#follow-redirects) if the issue was
     * [transferred](https://docs.github.com/articles/transferring-an-issue-to-another-repository/) to another repository. If
     * the issue was transferred to or deleted from a repository where the authenticated user lacks read access, the API
     * returns a `404 Not Found` status. If the issue was deleted from a repository where the authenticated user has read
     * access, the API returns a `410 Gone` status. To receive webhook events for transferred and deleted issues, subscribe
     * to the [`issues`](https://docs.github.com/webhooks/event-payloads/#issues) webhook.
     *
     * > [!NOTE]
     * > GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#get-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn get(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Update an issue.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/issues/{issue_number}` endpoint.
     *
     * Issue owners and users with push access or Triage role can edit an issue.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#update-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn update(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Add assignees to an issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/assignees` endpoint.
     *
     * Adds up to 10 assignees to an issue. Users already assigned to an issue are not replaced.
     *
     * FROM: <https://docs.github.com/rest/issues/assignees#add-assignees-to-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn add_assignees(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddAssigneesRequest,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/assignees",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Remove assignees from an issue.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/assignees` endpoint.
     *
     * Removes one or more assignees from an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/assignees#remove-assignees-from-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn remove_assignees(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddAssigneesRequest,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/assignees",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
            ),
            None,
        );
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
     * Check if a user can be assigned to a issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/assignees/{assignee}` endpoint.
     *
     * Checks if a user has permission to be assigned to a specific issue.
     *
     * If the `assignee` can be assigned to this issue, a `204` status code with no content is returned.
     *
     * Otherwise a `404` status code is returned.
     *
     * FROM: <https://docs.github.com/rest/issues/assignees#check-if-a-user-can-be-assigned-to-a-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `assignee: &str`
     */
    pub async fn check_user_can_be_assigned_to_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        assignee: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/assignees/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
                crate::progenitor_support::encode_path(&assignee.to_string()),
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
     * List issue comments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/comments` endpoint.
     *
     * You can use the REST API to list comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
     *
     * Issue comments are ordered by ascending ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#list-issue-comments>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_comments(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueCommentData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
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
                "/repos/{}/{}/issues/{}/comments?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List issue comments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/comments` endpoint.
     *
     * As opposed to `list_comments`, this function returns all the pages of the request at once.
     *
     * You can use the REST API to list comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
     *
     * Issue comments are ordered by ascending ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#list-issue-comments>
     */
    pub async fn list_all_comments(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueCommentData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/comments?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Create an issue comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/comments` endpoint.
     *
     * You can use the REST API to create comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications).
     * Creating content too quickly using this endpoint may result in secondary rate limiting.
     * For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/comments#create-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn create_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::IssueCommentData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/comments",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List dependencies an issue is blocked by.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/dependencies/blocked_by` endpoint.
     *
     * You can use the REST API to list the dependencies an issue is blocked by.
     *
     * This endpoint supports the following custom media types. For more information, see [Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw Markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the Markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's Markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issue-dependencies#list-dependencies-an-issue-is-blocked-by>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_dependencies_blocked_by(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
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
                "/repos/{}/{}/issues/{}/dependencies/blocked_by?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List dependencies an issue is blocked by.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/dependencies/blocked_by` endpoint.
     *
     * As opposed to `list_dependencies_blocked_by`, this function returns all the pages of the request at once.
     *
     * You can use the REST API to list the dependencies an issue is blocked by.
     *
     * This endpoint supports the following custom media types. For more information, see [Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw Markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the Markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's Markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issue-dependencies#list-dependencies-an-issue-is-blocked-by>
     */
    pub async fn list_all_dependencies_blocked_by(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/dependencies/blocked_by",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Add a dependency an issue is blocked by.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/dependencies/blocked_by` endpoint.
     *
     * You can use the REST API to add a 'blocked by' relationship to an issue.
     *
     * Creating content too quickly using this endpoint may result in secondary rate limiting.
     * For more information, see [Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)
     * and [Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).
     *
     * This endpoint supports the following custom media types. For more information, see [Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw Markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the Markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's Markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issue-dependencies#add-a-dependency-an-issue-is-blocked-by>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn add_blocked_by_dependency(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddBlockedByDependencyRequest,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/dependencies/blocked_by",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Remove dependency an issue is blocked by.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/dependencies/blocked_by/{issue_id}` endpoint.
     *
     * You can use the REST API to remove a dependency that an issue is blocked by.
     *
     * Removing content too quickly using this endpoint may result in secondary rate limiting.
     * For more information, see [Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)
     * and [Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).
     *
     * This endpoint supports the following custom media types. For more information, see [Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     * - **`application/vnd.github.raw+json`**: Returns the raw Markdown body. Response will include `body`. This is the default if you do not pass a specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the Markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's Markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issue-dependencies#remove-dependency-an-issue-is-blocked-by>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `issue_id: i64` -- The id of the blocking issue to remove as a dependency.
     */
    pub async fn remove_dependency_blocked_by(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        issue_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/dependencies/blocked_by/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
                crate::progenitor_support::encode_path(&issue_id.to_string()),
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
     * List dependencies an issue is blocking.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/dependencies/blocking` endpoint.
     *
     * You can use the REST API to list the dependencies an issue is blocking.
     *
     * This endpoint supports the following custom media types. For more information, see [Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw Markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the Markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's Markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issue-dependencies#list-dependencies-an-issue-is-blocking>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_dependencies_blocking(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
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
                "/repos/{}/{}/issues/{}/dependencies/blocking?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List dependencies an issue is blocking.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/dependencies/blocking` endpoint.
     *
     * As opposed to `list_dependencies_blocking`, this function returns all the pages of the request at once.
     *
     * You can use the REST API to list the dependencies an issue is blocking.
     *
     * This endpoint supports the following custom media types. For more information, see [Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw Markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the Markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's Markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issue-dependencies#list-dependencies-an-issue-is-blocking>
     */
    pub async fn list_all_dependencies_blocking(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/dependencies/blocking",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List issue events.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/events` endpoint.
     *
     * Lists all events for an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/events#list-issue-events>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_events(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueEventAnyOf>>> {
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
                "/repos/{}/{}/issues/{}/events?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List issue events.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/events` endpoint.
     *
     * As opposed to `list_events`, this function returns all the pages of the request at once.
     *
     * Lists all events for an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/events#list-issue-events>
     */
    pub async fn list_all_events(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueEventAnyOf>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/events",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List issue field values for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/issue-field-values` endpoint.
     *
     * Lists all issue field values for an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/issue-field-values#list-issue-field-values-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_issue_field_values_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueFieldValue>>> {
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
                "/repos/{}/{}/issues/{}/issue-field-values?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List issue field values for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/issue-field-values` endpoint.
     *
     * As opposed to `list_issue_field_values_for_issue`, this function returns all the pages of the request at once.
     *
     * Lists all issue field values for an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/issue-field-values#list-issue-field-values-for-an-issue>
     */
    pub async fn list_all_issue_field_values_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueFieldValue>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/issue-field-values",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List labels for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels` endpoint.
     *
     * Lists all labels for an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#list-labels-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_labels_on_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
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
                "/repos/{}/{}/issues/{}/labels?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List labels for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels` endpoint.
     *
     * As opposed to `list_labels_on_issue`, this function returns all the pages of the request at once.
     *
     * Lists all labels for an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#list-labels-for-an-issue>
     */
    pub async fn list_all_labels_on_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Set labels for an issue.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels` endpoint.
     *
     * Removes any previous labels and sets the new labels for an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#set-labels-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn set_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesSetLabelsRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Add labels to an issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels` endpoint.
     *
     * Adds labels to an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#add-labels-to-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn add_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddLabelsRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Remove all labels from an issue.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels` endpoint.
     *
     * Removes all labels from an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#remove-all-labels-from-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn remove_all_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Remove a label from an issue.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/labels/{name}` endpoint.
     *
     * Removes the specified label from the issue, and returns the remaining labels on the issue. This endpoint returns a `404 Not Found` status if the label does not exist.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#remove-a-label-from-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `name: &str`
     */
    pub async fn remove_label(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        name: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/labels/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Lock an issue.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/issues/{issue_number}/lock` endpoint.
     *
     * Users with push access can lock an issue or pull request's conversation.
     *
     * Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
     *
     * FROM: <https://docs.github.com/rest/issues/issues#lock-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn lock(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesLockRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/lock",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Unlock an issue.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/lock` endpoint.
     *
     * Users with push access can unlock an issue's conversation.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#unlock-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn unlock(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/lock",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Get parent issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/parent` endpoint.
     *
     * You can use the REST API to get the parent issue of a sub-issue.
     *
     * This endpoint supports the following custom media types. For more information, see [Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/sub-issues#get-parent-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn get_parent(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/parent",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Remove sub-issue.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/sub_issue` endpoint.
     *
     * You can use the REST API to remove a sub-issue from an issue.
     * Removing content too quickly using this endpoint may result in secondary rate limiting.
     * For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass a specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/sub-issues#remove-sub-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn remove_sub_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesRemoveSubIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/sub_issue",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
            ),
            None,
        );
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
     * List sub-issues.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/sub_issues` endpoint.
     *
     * You can use the REST API to list the sub-issues on an issue.
     *
     * This endpoint supports the following custom media types. For more information, see [Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw Markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the Markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's Markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/sub-issues#list-sub-issues>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_sub_issues(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
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
                "/repos/{}/{}/issues/{}/sub_issues?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List sub-issues.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/sub_issues` endpoint.
     *
     * As opposed to `list_sub_issues`, this function returns all the pages of the request at once.
     *
     * You can use the REST API to list the sub-issues on an issue.
     *
     * This endpoint supports the following custom media types. For more information, see [Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw Markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the Markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's Markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/sub-issues#list-sub-issues>
     */
    pub async fn list_all_sub_issues(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/sub_issues",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Add sub-issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/sub_issues` endpoint.
     *
     * You can use the REST API to add sub-issues to issues.
     *
     * Creating content too quickly using this endpoint may result in secondary rate limiting.
     * For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/sub-issues#add-sub-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn add_sub_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddSubIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/sub_issues",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Reprioritize sub-issue.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/issues/{issue_number}/sub_issues/priority` endpoint.
     *
     * You can use the REST API to reprioritize a sub-issue to a different position in the parent list.
     *
     * FROM: <https://docs.github.com/rest/issues/sub-issues#reprioritize-sub-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn reprioritize_sub_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesReprioritizeSubIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Issue>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/sub_issues/priority",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List timeline events for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/timeline` endpoint.
     *
     * List all timeline events for an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/timeline#list-timeline-events-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_events_for_timeline(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgRules>>> {
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
                "/repos/{}/{}/issues/{}/timeline?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List timeline events for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/timeline` endpoint.
     *
     * As opposed to `list_events_for_timeline`, this function returns all the pages of the request at once.
     *
     * List all timeline events for an issue.
     *
     * FROM: <https://docs.github.com/rest/issues/timeline#list-timeline-events-for-an-issue>
     */
    pub async fn list_all_events_for_timeline(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgRules>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/timeline",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List labels for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/labels` endpoint.
     *
     * Lists all labels for a repository.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#list-labels-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_labels_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
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
                "/repos/{}/{}/labels?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List labels for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/labels` endpoint.
     *
     * As opposed to `list_labels_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists all labels for a repository.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#list-labels-for-a-repository>
     */
    pub async fn list_all_labels_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Create a label.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/labels` endpoint.
     *
     * Creates a label for the specified repository with the given name and color. The name and color parameters are required. The color must be a valid [hexadecimal color code](http://www.color-hex.com/).
     *
     * FROM: <https://docs.github.com/rest/issues/labels#create-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_label(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::IssuesCreateLabelRequest,
    ) -> ClientResult<crate::Response<crate::types::Label>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get a label.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/labels/{name}` endpoint.
     *
     * Gets a label using the given name.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#get-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `name: &str`
     */
    pub async fn get_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> ClientResult<crate::Response<crate::types::Label>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/labels/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Delete a label.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/labels/{name}` endpoint.
     *
     * Deletes a label using the given label name.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#delete-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `name: &str`
     */
    pub async fn delete_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/labels/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Update a label.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/labels/{name}` endpoint.
     *
     * Updates a label using the given label name.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#update-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `name: &str`
     */
    pub async fn update_label(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        body: &crate::types::IssuesUpdateLabelRequest,
    ) -> ClientResult<crate::Response<crate::types::Label>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/labels/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * List milestones.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/milestones` endpoint.
     *
     * Lists milestones for a repository.
     *
     * FROM: <https://docs.github.com/rest/issues/milestones#list-milestones>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `state: crate::types::IssuesListState` -- The state of the milestone. Either `open`, `closed`, or `all`.
     * * `sort: crate::types::IssuesListMilestonesSort` -- What to sort results by. Either `due_on` or `completeness`.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_milestones(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::IssuesListState,
        sort: crate::types::IssuesListMilestonesSort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Milestone>>> {
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
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/milestones?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List milestones.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/milestones` endpoint.
     *
     * As opposed to `list_milestones`, this function returns all the pages of the request at once.
     *
     * Lists milestones for a repository.
     *
     * FROM: <https://docs.github.com/rest/issues/milestones#list-milestones>
     */
    pub async fn list_all_milestones(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::IssuesListState,
        sort: crate::types::IssuesListMilestonesSort,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::Milestone>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/milestones?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Create a milestone.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/milestones` endpoint.
     *
     * Creates a milestone.
     *
     * FROM: <https://docs.github.com/rest/issues/milestones#create-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_milestone(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::IssuesCreateMilestoneRequest,
    ) -> ClientResult<crate::Response<crate::types::Milestone>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/milestones",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get a milestone.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/milestones/{milestone_number}` endpoint.
     *
     * Gets a milestone using the given milestone number.
     *
     * FROM: <https://docs.github.com/rest/issues/milestones#get-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `milestone_number: i64` -- The number that identifies the milestone.
     */
    pub async fn get_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
    ) -> ClientResult<crate::Response<crate::types::Milestone>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/milestones/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&milestone_number.to_string()),
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
     * Delete a milestone.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/milestones/{milestone_number}` endpoint.
     *
     * Deletes a milestone using the given milestone number.
     *
     * FROM: <https://docs.github.com/rest/issues/milestones#delete-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `milestone_number: i64` -- The number that identifies the milestone.
     */
    pub async fn delete_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/milestones/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&milestone_number.to_string()),
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
     * Update a milestone.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/milestones/{milestone_number}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/issues/milestones#update-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `milestone_number: i64` -- The number that identifies the milestone.
     */
    pub async fn update_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
        body: &crate::types::IssuesUpdateMilestoneRequest,
    ) -> ClientResult<crate::Response<crate::types::Milestone>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/milestones/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&milestone_number.to_string()),
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
     * List labels for issues in a milestone.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/milestones/{milestone_number}/labels` endpoint.
     *
     * Lists labels for issues in a milestone.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#list-labels-for-issues-in-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `milestone_number: i64` -- The number that identifies the milestone.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_labels_for_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
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
                "/repos/{}/{}/milestones/{}/labels?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&milestone_number.to_string()),
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
     * List labels for issues in a milestone.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/milestones/{milestone_number}/labels` endpoint.
     *
     * As opposed to `list_labels_for_milestone`, this function returns all the pages of the request at once.
     *
     * Lists labels for issues in a milestone.
     *
     * FROM: <https://docs.github.com/rest/issues/labels#list-labels-for-issues-in-a-milestone>
     */
    pub async fn list_all_labels_for_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/milestones/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&milestone_number.to_string()),
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
     * Set issue field values for an issue.
     *
     * This function performs a `PUT` to the `/repositories/{repository_id}/issues/{issue_number}/issue-field-values` endpoint.
     *
     * Set custom field values for an issue, replacing any existing values. You can set values for organization-level issue fields that have been defined for the repository's organization.
     *
     * This endpoint supports the following field data types:
     * - **`text`**: String values for text fields
     * - **`single_select`**: Option names for single-select fields (must match an existing option name)
     * - **`number`**: Numeric values for number fields
     * - **`date`**: ISO 8601 date strings for date fields
     *
     * This operation will replace all existing field values with the provided ones. If you want to add field values without replacing existing ones, use the `POST` endpoint instead.
     *
     * Only users with push access to the repository can set issue field values. If you don't have the proper permissions, you'll receive a `403 Forbidden` response.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * FROM: <https://docs.github.com/rest/issues/issue-field-values#set-issue-field-values-for-an-issue>
     *
     * **Parameters:**
     *
     * * `repository_id: i64` -- The unique identifier of the repository.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn set_issue_field_values(
        &self,
        repository_id: i64,
        issue_number: i64,
        body: &crate::types::IssuesAddIssueFieldValuesRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueFieldValue>>> {
        let url = self.client.url(
            &format!(
                "/repositories/{}/issues/{}/issue-field-values",
                crate::progenitor_support::encode_path(&repository_id.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Add issue field values to an issue.
     *
     * This function performs a `POST` to the `/repositories/{repository_id}/issues/{issue_number}/issue-field-values` endpoint.
     *
     * Add custom field values to an issue. You can set values for organization-level issue fields that have been defined for the repository's organization.
     * Adding an empty array will clear all existing field values for the issue.
     *
     * This endpoint supports the following field data types:
     * - **`text`**: String values for text fields
     * - **`single_select`**: Option names for single-select fields (must match an existing option name)
     * - **`number`**: Numeric values for number fields
     * - **`date`**: ISO 8601 date strings for date fields
     *
     * Only users with push access to the repository can add issue field values. If you don't have the proper permissions, you'll receive a `403 Forbidden` response.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * FROM: <https://docs.github.com/rest/issues/issue-field-values#add-issue-field-values-to-an-issue>
     *
     * **Parameters:**
     *
     * * `repository_id: i64` -- The unique identifier of the repository.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn add_issue_field_values(
        &self,
        repository_id: i64,
        issue_number: i64,
        body: &crate::types::IssuesAddIssueFieldValuesRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueFieldValue>>> {
        let url = self.client.url(
            &format!(
                "/repositories/{}/issues/{}/issue-field-values",
                crate::progenitor_support::encode_path(&repository_id.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Delete an issue field value from an issue.
     *
     * This function performs a `DELETE` to the `/repositories/{repository_id}/issues/{issue_number}/issue-field-values/{issue_field_id}` endpoint.
     *
     * Remove a specific custom field value from an issue.
     *
     * Only users with push access to the repository can delete issue field values. If you don't have the proper permissions, you'll receive a `403 Forbidden` response.
     *
     * If the specified field does not have a value set on the issue, this operation will return a `404` error.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * FROM: <https://docs.github.com/rest/issues/issue-field-values#delete-an-issue-field-value-from-an-issue>
     *
     * **Parameters:**
     *
     * * `repository_id: i64` -- The unique identifier of the repository.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `issue_field_id: i64` -- The unique identifier of the issue field.
     */
    pub async fn delete_issue_field_value(
        &self,
        repository_id: i64,
        issue_number: i64,
        issue_field_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repositories/{}/issues/{}/issue-field-values/{}",
                crate::progenitor_support::encode_path(&repository_id.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
                crate::progenitor_support::encode_path(&issue_field_id.to_string()),
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
     * List user account issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/user/issues` endpoint.
     *
     * List issues across owned and member repositories assigned to the authenticated user.
     *
     * > [!NOTE]
     * > GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#list-user-account-issues-assigned-to-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `filter: crate::types::Filter` -- Indicates which sorts of issues to return. `assigned` means issues assigned to you. `created` means issues created by you. `mentioned` means issues mentioning you. `subscribed` means issues you're subscribed to updates for. `all` or `repos` means all issues you can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_authenticated_user(
        &self,
        filter: crate::types::Filter,
        state: crate::types::IssuesListState,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !labels.is_empty() {
            query_args.push(("labels".to_string(), labels.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/issues?{}", query_), None);
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
     * List user account issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/user/issues` endpoint.
     *
     * As opposed to `list_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * List issues across owned and member repositories assigned to the authenticated user.
     *
     * > [!NOTE]
     * > GitHub's REST API considers every pull request an issue, but not every issue is a pull request. For this reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/pulls/pulls#list-pull-requests)" endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/issues/issues#list-user-account-issues-assigned-to-the-authenticated-user>
     */
    pub async fn list_all_for_authenticated_user(
        &self,
        filter: crate::types::Filter,
        state: crate::types::IssuesListState,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::Issue>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !labels.is_empty() {
            query_args.push(("labels".to_string(), labels.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/issues?{}", query_), None);
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

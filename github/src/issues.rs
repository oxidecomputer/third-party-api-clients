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
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issues-assigned-to-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `filter: crate::types::Filter` -- Indicates which sorts of issues to return. Can be one of:  
     *  \\* `assigned`: Issues assigned to you  
     *  \\* `created`: Issues created by you  
     *  \\* `mentioned`: Issues mentioning you  
     *  \\* `subscribed`: Issues you're subscribed to updates for  
     *  \\* `all` or `repos`: All issues the authenticated user can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by. Can be either `created`, `updated`, `comments`.
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `collab: bool`
     * * `orgs: bool`
     * * `owned: bool`
     * * `pulls: bool`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issues-assigned-to-the-authenticated-user>
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
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-organization-issues-assigned-to-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `filter: crate::types::Filter` -- Indicates which sorts of issues to return. Can be one of:  
     *  \\* `assigned`: Issues assigned to you  
     *  \\* `created`: Issues created by you  
     *  \\* `mentioned`: Issues mentioning you  
     *  \\* `subscribed`: Issues you're subscribed to updates for  
     *  \\* `all` or `repos`: All issues the authenticated user can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by. Can be either `created`, `updated`, `comments`.
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_org(
        &self,
        org: &str,
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
        let url = self.client.url(
            &format!(
                "/orgs/{}/issues?{}",
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
     * List organization issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/orgs/{org}/issues` endpoint.
     *
     * As opposed to `list_for_org`, this function returns all the pages of the request at once.
     *
     * List issues in an organization assigned to the authenticated user.
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-organization-issues-assigned-to-the-authenticated-user>
     */
    pub async fn list_all_for_org(
        &self,
        org: &str,
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
        let url = self.client.url(
            &format!(
                "/orgs/{}/issues?{}",
                crate::progenitor_support::encode_path(org),
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
     * Lists the [available assignees](https://help.github.com/articles/assigning-issues-and-pull-requests-to-other-github-users/) for issues in a repository.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-assignees>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
     * List assignees.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/assignees` endpoint.
     *
     * As opposed to `list_assignees`, this function returns all the pages of the request at once.
     *
     * Lists the [available assignees](https://help.github.com/articles/assigning-issues-and-pull-requests-to-other-github-users/) for issues in a repository.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-assignees>
     */
    pub async fn list_all_assignees(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/assignees",
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
     * FROM: <https://docs.github.com/rest/reference/issues#check-if-a-user-can-be-assigned>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(assignee),
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
     * List issues in a repository.
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-repository-issues>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone: &str` -- If an `integer` is passed, it should refer to a milestone by its `number` field. If the string `*` is passed, issues with any milestone are accepted. If the string `none` is passed, issues without milestones are returned.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `assignee: &str` -- Can be the name of a user. Pass in `none` for issues with no assigned user, and `*` for issues assigned to any user.
     * * `creator: &str` -- The user that created the issue.
     * * `mentioned: &str` -- A user that's mentioned in the issue.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by. Can be either `created`, `updated`, `comments`.
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_repo(
        &self,
        owner: &str,
        repo: &str,
        milestone: &str,
        state: crate::types::IssuesListState,
        assignee: &str,
        creator: &str,
        mentioned: &str,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueSimple>>> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues?{}",
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
     * List repository issues.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues` endpoint.
     *
     * As opposed to `list_for_repo`, this function returns all the pages of the request at once.
     *
     * List issues in a repository.
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-repository-issues>
     */
    pub async fn list_all_for_repo(
        &self,
        owner: &str,
        repo: &str,
        milestone: &str,
        state: crate::types::IssuesListState,
        assignee: &str,
        creator: &str,
        mentioned: &str,
        labels: &str,
        sort: crate::types::IssuesListSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueSimple>>> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues?{}",
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
     * Create an issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues` endpoint.
     *
     * Any user with pull access to a repository can create an issue. If [issues are disabled in the repository](https://help.github.com/articles/disabling-issues/), the API returns a `410 Gone` status.
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#create-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * By default, Issue Comments are ordered by ascending ID.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-comments-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `sort: crate::types::Sort` -- One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueComment>>> {
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
     * List issue comments for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments` endpoint.
     *
     * As opposed to `list_comments_for_repo`, this function returns all the pages of the request at once.
     *
     * By default, Issue Comments are ordered by ascending ID.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-comments-for-a-repository>
     */
    pub async fn list_all_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueComment>>> {
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
     * Get an issue comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn get_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<crate::types::IssueComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#delete-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#update-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn update_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::IssueComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List issue events for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/events` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-events-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
     * List issue events for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/events` endpoint.
     *
     * As opposed to `list_events_for_repo`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-events-for-a-repository>
     */
    pub async fn list_all_events_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueEvent>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/events",
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
     * Get an issue event.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/events/{event_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-an-issue-event>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * The API returns a [`301 Moved Permanently` status](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-redirects-redirects) if the issue was
     * [transferred](https://help.github.com/articles/transferring-an-issue-to-another-repository/) to another repository. If
     * the issue was transferred to or deleted from a repository where the authenticated user lacks read access, the API
     * returns a `404 Not Found` status. If the issue was deleted from a repository where the authenticated user has read
     * access, the API returns a `410 Gone` status. To receive webhook events for transferred and deleted issues, subscribe
     * to the [`issues`](https://docs.github.com/webhooks/event-payloads/#issues) webhook.
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Issue owners and users with push access can edit an issue.
     *
     * FROM: <https://docs.github.com/rest/reference/issues/#update-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/issues#add-assignees-to-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn add_assignees(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddAssigneesRequest,
    ) -> ClientResult<crate::Response<crate::types::IssueSimple>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/assignees",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/issues#remove-assignees-from-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn remove_assignees(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesAddAssigneesRequest,
    ) -> ClientResult<crate::Response<crate::types::IssueSimple>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/assignees",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List issue comments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/comments` endpoint.
     *
     * Issue Comments are ordered by ascending ID.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-comments>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_comments(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueComment>>> {
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Issue Comments are ordered by ascending ID.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-comments>
     */
    pub async fn list_all_comments(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/comments?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#create-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn create_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::IssueComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/comments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List issue events.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/events` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-events>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-issue-events>
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-labels-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-labels-for-an-issue>
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/issues#set-labels-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn set_labels(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::IssuesSetLabelsRequestAnyOf,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/labels",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#add-labels-to-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#remove-all-labels-from-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/issues#remove-a-label-from-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
                crate::progenitor_support::encode_path(name),
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
     * Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * FROM: <https://docs.github.com/rest/reference/issues#lock-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/issues#unlock-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List timeline events for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/timeline` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-timeline-events-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_events_for_timeline(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Data>>> {
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-timeline-events-for-an-issue>
     */
    pub async fn list_all_events_for_timeline(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Data>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/timeline",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-labels-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
     * List labels for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/labels` endpoint.
     *
     * As opposed to `list_labels_for_repo`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-labels-for-a-repository>
     */
    pub async fn list_all_labels_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Label>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/labels",
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
     * Create a label.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/labels` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#create-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(name),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#delete-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(name),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#update-a-label>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(name),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-milestones>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `sort: crate::types::IssuesListMilestonesSort` -- What to sort results by. Either `due_on` or `completeness`.
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
     * List milestones.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/milestones` endpoint.
     *
     * As opposed to `list_milestones`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-milestones>
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
     * Create a milestone.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/milestones` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#create-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#get-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone_number: i64` -- milestone_number parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#delete-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone_number: i64` -- milestone_number parameter.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/issues#update-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone_number: i64` -- milestone_number parameter.
     */
    pub async fn update_milestone(
        &self,
        owner: &str,
        repo: &str,
        milestone_number: i64,
        body: &crate::types::IssuesCreateMilestoneRequest,
    ) -> ClientResult<crate::Response<crate::types::Milestone>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/milestones/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-labels-for-issues-in-a-milestone>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `milestone_number: i64` -- milestone_number parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-labels-for-issues-in-a-milestone>
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List user account issues assigned to the authenticated user.
     *
     * This function performs a `GET` to the `/user/issues` endpoint.
     *
     * List issues across owned and member repositories assigned to the authenticated user.
     *
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-user-account-issues-assigned-to-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `filter: crate::types::Filter` -- Indicates which sorts of issues to return. Can be one of:  
     *  \\* `assigned`: Issues assigned to you  
     *  \\* `created`: Issues created by you  
     *  \\* `mentioned`: Issues mentioning you  
     *  \\* `subscribed`: Issues you're subscribed to updates for  
     *  \\* `all` or `repos`: All issues the authenticated user can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by. Can be either `created`, `updated`, `comments`.
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
     * **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
     * reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
     * the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
     * request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/issues#list-user-account-issues-assigned-to-the-authenticated-user>
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

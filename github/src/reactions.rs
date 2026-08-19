use crate::Client;
use crate::ClientResult;

pub struct Reactions {
    pub client: Client,
}

impl Reactions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reactions { client }
    }

    /**
     * List reactions for a team discussion comment.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions` endpoint.
     *
     * List the reactions to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments/). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion-comment>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     * * `comment_number: i64`
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a team discussion comment.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_team_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * List reactions for a team discussion comment.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions` endpoint.
     *
     * As opposed to `list_for_team_discussion_comment_in_org`, this function returns all the pages of the request at once.
     *
     * List the reactions to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments/). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion-comment>
     */
    pub async fn list_all_for_team_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * Create reaction for a team discussion comment.
     *
     * This function performs a `POST` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions` endpoint.
     *
     * Create a reaction to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion comment.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion-comment>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     * * `comment_number: i64`
     */
    pub async fn create_for_team_discussion_comment_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/comments/{}/reactions",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * Delete team discussion comment reaction.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions/{reaction_id}` endpoint.
     *
     * **Note:** You can also specify a team or organization with `team_id` and `org_id` using the route `DELETE /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions/:reaction_id`.
     *
     * Delete a reaction to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#delete-team-discussion-comment-reaction>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     * * `comment_number: i64`
     * * `reaction_id: i64`
     */
    pub async fn delete_for_team_discussion_comment(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        comment_number: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/comments/{}/reactions/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * List reactions for a team discussion.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions` endpoint.
     *
     * List the reactions to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a team discussion.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_team_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/reactions?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * List reactions for a team discussion.
     *
     * This function performs a `GET` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions` endpoint.
     *
     * As opposed to `list_for_team_discussion_in_org`, this function returns all the pages of the request at once.
     *
     * List the reactions to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion>
     */
    pub async fn list_all_for_team_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/reactions?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Create reaction for a team discussion.
     *
     * This function performs a `POST` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions` endpoint.
     *
     * Create a reaction to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion.
     *
     * **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     */
    pub async fn create_for_team_discussion_in_org(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/reactions",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Delete team discussion reaction.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions/{reaction_id}` endpoint.
     *
     * **Note:** You can also specify a team or organization with `team_id` and `org_id` using the route `DELETE /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions/:reaction_id`.
     *
     * Delete a reaction to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#delete-team-discussion-reaction>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `team_slug: &str` -- team_slug parameter.
     * * `discussion_number: i64`
     * * `reaction_id: i64`
     */
    pub async fn delete_for_team_discussion(
        &self,
        org: &str,
        team_slug: &str,
        discussion_number: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/teams/{}/discussions/{}/reactions/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(team_slug),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * Delete a reaction (Legacy).
     *
     * This function performs a `DELETE` to the `/reactions/{reaction_id}` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Reactions API. We recommend migrating your existing code to use the new delete reactions endpoints. For more information, see this [blog post](https://developer.github.com/changes/2020-02-26-new-delete-reactions-endpoints/).
     *
     * OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/), when deleting a [team discussion](https://docs.github.com/rest/reference/teams#discussions) or [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions/#delete-a-reaction-legacy>
     *
     * **Parameters:**
     *
     * * `reaction_id: i64`
     */
    pub async fn delete_legacy(&self, reaction_id: i64) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/reactions/{}",
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * List reactions for a commit comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/comments/{comment_id}/reactions` endpoint.
     *
     * List the reactions to a [commit comment](https://docs.github.com/rest/reference/repos#comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a commit comment.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * List reactions for a commit comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/comments/{comment_id}/reactions` endpoint.
     *
     * As opposed to `list_for_commit_comment`, this function returns all the pages of the request at once.
     *
     * List the reactions to a [commit comment](https://docs.github.com/rest/reference/repos#comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-a-commit-comment>
     */
    pub async fn list_all_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Create reaction for a commit comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/comments/{comment_id}/reactions` endpoint.
     *
     * Create a reaction to a [commit comment](https://docs.github.com/rest/reference/repos#comments). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn create_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}/reactions",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Delete a commit comment reaction.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/comments/{comment_id}/reactions/{reaction_id}` endpoint.
     *
     * **Note:** You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/comments/:comment_id/reactions/:reaction_id`.
     *
     * Delete a reaction to a [commit comment](https://docs.github.com/rest/reference/repos#comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#delete-a-commit-comment-reaction>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     * * `reaction_id: i64`
     */
    pub async fn delete_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}/reactions/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * List reactions for an issue comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions` endpoint.
     *
     * List the reactions to an [issue comment](https://docs.github.com/rest/reference/issues#comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to an issue comment.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * List reactions for an issue comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions` endpoint.
     *
     * As opposed to `list_for_issue_comment`, this function returns all the pages of the request at once.
     *
     * List the reactions to an [issue comment](https://docs.github.com/rest/reference/issues#comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-an-issue-comment>
     */
    pub async fn list_all_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Create reaction for an issue comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions` endpoint.
     *
     * Create a reaction to an [issue comment](https://docs.github.com/rest/reference/issues#comments). A response with an HTTP `200` status means that you already added the reaction type to this issue comment.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#create-reaction-for-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn create_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/reactions",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Delete an issue comment reaction.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions/{reaction_id}` endpoint.
     *
     * **Note:** You can also specify a repository by `repository_id` using the route `DELETE delete /repositories/:repository_id/issues/comments/:comment_id/reactions/:reaction_id`.
     *
     * Delete a reaction to an [issue comment](https://docs.github.com/rest/reference/issues#comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#delete-an-issue-comment-reaction>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     * * `reaction_id: i64`
     */
    pub async fn delete_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/reactions/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * List reactions for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/reactions` endpoint.
     *
     * List the reactions to an [issue](https://docs.github.com/rest/reference/issues).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to an issue.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/reactions?{}",
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
     * List reactions for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/reactions` endpoint.
     *
     * As opposed to `list_for_issue`, this function returns all the pages of the request at once.
     *
     * List the reactions to an [issue](https://docs.github.com/rest/reference/issues).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-an-issue>
     */
    pub async fn list_all_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/reactions?{}",
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
     * Create reaction for an issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/reactions` endpoint.
     *
     * Create a reaction to an [issue](https://docs.github.com/rest/reference/issues/). A response with an HTTP `200` status means that you already added the reaction type to this issue.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#create-reaction-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     */
    pub async fn create_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/reactions",
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
     * Delete an issue reaction.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/reactions/{reaction_id}` endpoint.
     *
     * **Note:** You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/issues/:issue_number/reactions/:reaction_id`.
     *
     * Delete a reaction to an [issue](https://docs.github.com/rest/reference/issues/).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#delete-an-issue-reaction>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `issue_number: i64` -- issue_number parameter.
     * * `reaction_id: i64`
     */
    pub async fn delete_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/reactions/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * List reactions for a pull request review comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions` endpoint.
     *
     * List the reactions to a [pull request review comment](https://docs.github.com/rest/reference/pulls#review-comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-a-pull-request-review-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a pull request review comment.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_pull_request_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * List reactions for a pull request review comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions` endpoint.
     *
     * As opposed to `list_for_pull_request_review_comment`, this function returns all the pages of the request at once.
     *
     * List the reactions to a [pull request review comment](https://docs.github.com/rest/reference/pulls#review-comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#list-reactions-for-a-pull-request-review-comment>
     */
    pub async fn list_all_for_pull_request_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Create reaction for a pull request review comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions` endpoint.
     *
     * Create a reaction to a [pull request review comment](https://docs.github.com/rest/reference/pulls#comments). A response with an HTTP `200` status means that you already added the reaction type to this pull request review comment.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-pull-request-review-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn create_for_pull_request_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}/reactions",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Delete a pull request comment reaction.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions/{reaction_id}` endpoint.
     *
     * **Note:** You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/pulls/comments/:comment_id/reactions/:reaction_id.`
     *
     * Delete a reaction to a [pull request review comment](https://docs.github.com/rest/reference/pulls#review-comments).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions#delete-a-pull-request-comment-reaction>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     * * `reaction_id: i64`
     */
    pub async fn delete_for_pull_request_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}/reactions/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * Create reaction for a release.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/releases/{release_id}/reactions` endpoint.
     *
     * Create a reaction to a [release](https://docs.github.com/rest/reference/repos#releases). A response with a `Status: 200 OK` means that you already added the reaction type to this release.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions/#create-reaction-for-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `release_id: i64` -- release_id parameter.
     */
    pub async fn create_for_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        body: &crate::types::ReactionsCreateReleaseRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}/reactions",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&release_id.to_string()),
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
     * List reactions for a team discussion comment (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion comment`](https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion-comment) endpoint.
     *
     * List the reactions to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions/#list-reactions-for-a-team-discussion-comment-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     * * `comment_number: i64`
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a team discussion comment.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_team_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * List reactions for a team discussion comment (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions` endpoint.
     *
     * As opposed to `list_for_team_discussion_comment_legacy`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion comment`](https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion-comment) endpoint.
     *
     * List the reactions to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions/#list-reactions-for-a-team-discussion-comment-legacy>
     */
    pub async fn list_all_for_team_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * Create reaction for a team discussion comment (Legacy).
     *
     * This function performs a `POST` to the `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new "[Create reaction for a team discussion comment](https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion-comment)" endpoint.
     *
     * Create a reaction to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion comment.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions/#create-reaction-for-a-team-discussion-comment-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     * * `comment_number: i64`
     */
    pub async fn create_for_team_discussion_comment_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        comment_number: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/comments/{}/reactions",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
                crate::progenitor_support::encode_path(&comment_number.to_string()),
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
     * List reactions for a team discussion (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions/{discussion_number}/reactions` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion`](https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion) endpoint.
     *
     * List the reactions to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions/#list-reactions-for-a-team-discussion-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a team discussion.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_team_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/reactions?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * List reactions for a team discussion (Legacy).
     *
     * This function performs a `GET` to the `/teams/{team_id}/discussions/{discussion_number}/reactions` endpoint.
     *
     * As opposed to `list_for_team_discussion_legacy`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion`](https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion) endpoint.
     *
     * List the reactions to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/reactions/#list-reactions-for-a-team-discussion-legacy>
     */
    pub async fn list_all_for_team_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/reactions?{}",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
     * Create reaction for a team discussion (Legacy).
     *
     * This function performs a `POST` to the `/teams/{team_id}/discussions/{discussion_number}/reactions` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create reaction for a team discussion`](https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion) endpoint.
     *
     * Create a reaction to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions/#create-reaction-for-a-team-discussion-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     */
    pub async fn create_for_team_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/teams/{}/discussions/{}/reactions",
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&discussion_number.to_string()),
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
}

use anyhow::Result;

use crate::Client;

pub struct Issues {
    client: Client,
}

impl Issues {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Issues { client }
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
     *  \* `assigned`: Issues assigned to you  
     *  \* `created`: Issues created by you  
     *  \* `mentioned`: Issues mentioning you  
     *  \* `subscribed`: Issues you're subscribed to updates for  
     *  \* `all`: All issues the authenticated user can see, regardless of participation or creation.
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `labels: &str` -- A list of comma separated label names. Example: `bug,ui,@high`.
     * * `sort: crate::types::IssuesListSort` -- What to sort results by. Can be either `created`, `updated`, `comments`.
     * * `direction: crate::types::Direction` -- One of `asc` (ascending) or `desc` (descending).
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
        direction: crate::types::Direction,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::Issue>> {
        let url = format!(
            "/user/issues?direction={}&filter={}&labels={}&page={}&per_page={}&since={}&sort={}&\
             state={}",
            direction,
            filter,
            labels.to_string(),
            format!("{}", page),
            format!("{}", per_page),
            since,
            sort,
            state,
        );

        self.client.get_all_pages(&url).await
    }
}

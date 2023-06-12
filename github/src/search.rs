use crate::Client;
use crate::ClientResult;

pub struct Search {
    pub client: Client,
}

impl Search {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Search { client }
    }

    /**
     * Search code.
     *
     * This function performs a `GET` to the `/search/code` endpoint.
     *
     * Searches for query terms inside of a file. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
     *
     * When searching for code, you can get text match metadata for the file **content** and file **path** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
     *
     * For example, if you want to find the definition of the `addClass` function inside [jQuery](https://github.com/jquery/jquery) repository, your query would look something like this:
     *
     * `q=addClass+in:file+language:js+repo:jquery/jquery`
     *
     * This query searches for the keyword `addClass` within a file's contents. The query limits the search to files where the language is JavaScript in the `jquery/jquery` repository.
     *
     * #### Considerations for code search
     *
     * Due to the complexity of searching code, there are a few restrictions on how searches are performed:
     *
     * *   Only the _default branch_ is considered. In most cases, this will be the `master` branch.
     * *   Only files smaller than 384 KB are searchable.
     * *   You must always include at least one search term when searching source code. For example, searching for [`language:go`](https://github.com/search?utf8=%E2%9C%93&q=language%3Ago&type=Code) is not valid, while [`amazing
     * language:go`](https://github.com/search?utf8=%E2%9C%93&q=amazing+language%3Ago&type=Code) is.
     *
     * FROM: <https://docs.github.com/rest/reference/search#search-code>
     *
     * **Parameters:**
     *
     * * `q: &str` -- The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as GitHub.com. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/reference/search#constructing-a-search-query). See "[Searching code](https://help.github.com/articles/searching-code/)" for a detailed list of qualifiers.
     * * `sort: crate::types::SearchCodeSort` -- Sorts the results of your query. Can only be `indexed`, which indicates how recently a file has been indexed by the GitHub search infrastructure. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results).
     * * `order: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn code(
        &self,
        q: &str,
        sort: crate::types::SearchCodeSort,
        order: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::SearchCodeResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/search/code?{}", query_), None);
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
     * Search commits.
     *
     * This function performs a `GET` to the `/search/commits` endpoint.
     *
     * Find commits via various criteria on the default branch (usually `master`). This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
     *
     * When searching for commits, you can get text match metadata for the **message** field when you provide the `text-match` media type. For more details about how to receive highlighted search results, see [Text match
     * metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
     *
     * For example, if you want to find commits related to CSS in the [octocat/Spoon-Knife](https://github.com/octocat/Spoon-Knife) repository. Your query would look something like this:
     *
     * `q=repo:octocat/Spoon-Knife+css`
     *
     * FROM: <https://docs.github.com/rest/reference/search#search-commits>
     *
     * **Parameters:**
     *
     * * `q: &str` -- The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as GitHub.com. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/reference/search#constructing-a-search-query). See "[Searching commits](https://help.github.com/articles/searching-commits/)" for a detailed list of qualifiers.
     * * `sort: crate::types::SearchCommitsSort` -- Sorts the results of your query by `author-date` or `committer-date`. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results).
     * * `order: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn commits(
        &self,
        q: &str,
        sort: crate::types::SearchCommitsSort,
        order: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::SearchCommitsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/search/commits?{}", query_), None);
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
     * Search issues and pull requests.
     *
     * This function performs a `GET` to the `/search/issues` endpoint.
     *
     * Find issues by state and keyword. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
     *
     * When searching for issues, you can get text match metadata for the issue **title**, issue **body**, and issue **comment body** fields when you pass the `text-match` media type. For more details about how to receive highlighted
     * search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
     *
     * For example, if you want to find the oldest unresolved Python bugs on Windows. Your query might look something like this.
     *
     * `q=windows+label:bug+language:python+state:open&sort=created&order=asc`
     *
     * This query searches for the keyword `windows`, within any open issue that is labeled as `bug`. The search runs across repositories whose primary language is Python. The results are sorted by creation date in ascending order, which means the oldest issues appear first in the search results.
     *
     * **Note:** For [user-to-server](https://docs.github.com/developers/apps/identifying-and-authorizing-users-for-github-apps#user-to-server-requests) GitHub App requests, you can't retrieve a combination of issues and pull requests in a single query. Requests that don't include the `is:issue` or `is:pull-request` qualifier will receive an HTTP `422 Unprocessable Entity` response. To get results for both issues and pull requests, you must send separate queries for issues and pull requests. For more information about the `is` qualifier, see "[Searching only issues or pull requests](https://docs.github.com/github/searching-for-information-on-github/searching-issues-and-pull-requests#search-only-issues-or-pull-requests)."
     *
     * FROM: <https://docs.github.com/rest/reference/search#search-issues-and-pull-requests>
     *
     * **Parameters:**
     *
     * * `q: &str` -- The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as GitHub.com. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/reference/search#constructing-a-search-query). See "[Searching issues and pull requests](https://help.github.com/articles/searching-issues-and-pull-requests/)" for a detailed list of qualifiers.
     * * `sort: crate::types::SearchIssuesPullRequestsSort` -- Sorts the results of your query by the number of `comments`, `reactions`, `reactions-+1`, `reactions--1`, `reactions-smile`, `reactions-thinking_face`, `reactions-heart`, `reactions-tada`, or `interactions`. You can also sort results by how recently the items were `created` or `updated`, Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results).
     * * `order: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn issues_and_pull_requests(
        &self,
        q: &str,
        sort: crate::types::SearchIssuesPullRequestsSort,
        order: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::SearchIssuesPullRequestsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/search/issues?{}", query_), None);
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
     * Search labels.
     *
     * This function performs a `GET` to the `/search/labels` endpoint.
     *
     * Find labels in a repository with names or descriptions that match search keywords. Returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
     *
     * When searching for labels, you can get text match metadata for the label **name** and **description** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
     *
     * For example, if you want to find labels in the `linguist` repository that match `bug`, `defect`, or `enhancement`. Your query might look like this:
     *
     * `q=bug+defect+enhancement&repository_id=64778136`
     *
     * The labels that best match the query appear first in the search results.
     *
     * FROM: <https://docs.github.com/rest/reference/search#search-labels>
     *
     * **Parameters:**
     *
     * * `repository_id: i64` -- The id of the repository.
     * * `q: &str` -- The search keywords. This endpoint does not accept qualifiers in the query. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/reference/search#constructing-a-search-query).
     * * `sort: crate::types::Sort` -- Sorts the results of your query by when the label was `created` or `updated`. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results).
     * * `order: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn labels(
        &self,
        repository_id: i64,
        q: &str,
        sort: crate::types::Sort,
        order: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::SearchLabelsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if repository_id > 0 {
            query_args.push(("repository_id".to_string(), repository_id.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/search/labels?{}", query_), None);
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
     * Search repositories.
     *
     * This function performs a `GET` to the `/search/repositories` endpoint.
     *
     * Find repositories via various criteria. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
     *
     * When searching for repositories, you can get text match metadata for the **name** and **description** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
     *
     * For example, if you want to search for popular Tetris repositories written in assembly code, your query might look like this:
     *
     * `q=tetris+language:assembly&sort=stars&order=desc`
     *
     * This query searches for repositories with the word `tetris` in the name, the description, or the README. The results are limited to repositories where the primary language is assembly. The results are sorted by stars in descending order, so that the most popular repositories appear first in the search results.
     *
     * When you include the `mercy` preview header, you can also search for multiple topics by adding more `topic:` instances. For example, your query might look like this:
     *
     * `q=topic:ruby+topic:rails`
     *
     * FROM: <https://docs.github.com/rest/reference/search#search-repositories>
     *
     * **Parameters:**
     *
     * * `q: &str` -- The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as GitHub.com. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/reference/search#constructing-a-search-query). See "[Searching for repositories](https://help.github.com/articles/searching-for-repositories/)" for a detailed list of qualifiers.
     * * `sort: crate::types::SearchReposSort` -- Sorts the results of your query by number of `stars`, `forks`, or `help-wanted-issues` or how recently the items were `updated`. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results).
     * * `order: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn repos(
        &self,
        q: &str,
        sort: crate::types::SearchReposSort,
        order: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::SearchReposResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/search/repositories?{}", query_), None);
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
     * Search topics.
     *
     * This function performs a `GET` to the `/search/topics` endpoint.
     *
     * Find topics via various criteria. Results are sorted by best match. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination). See "[Searching topics](https://help.github.com/articles/searching-topics/)" for a detailed list of qualifiers.
     *
     * When searching for topics, you can get text match metadata for the topic's **short\_description**, **description**, **name**, or **display\_name** field when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
     *
     * For example, if you want to search for topics related to Ruby that are featured on https://github.com/topics. Your query might look like this:
     *
     * `q=ruby+is:featured`
     *
     * This query searches for topics with the keyword `ruby` and limits the results to find only topics that are featured. The topics that are the best match for the query appear first in the search results.
     *
     * FROM: <https://docs.github.com/rest/reference/search#search-topics>
     *
     * **Parameters:**
     *
     * * `q: &str` -- The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as GitHub.com. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/reference/search#constructing-a-search-query).
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn topics(
        &self,
        q: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::SearchTopicsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/search/topics?{}", query_), None);
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
     * Search users.
     *
     * This function performs a `GET` to the `/search/users` endpoint.
     *
     * Find users via various criteria. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
     *
     * When searching for users, you can get text match metadata for the issue **login**, **email**, and **name** fields when you pass the `text-match` media type. For more details about highlighting search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata). For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
     *
     * For example, if you're looking for a list of popular users, you might try this query:
     *
     * `q=tom+repos:%3E42+followers:%3E1000`
     *
     * This query searches for users with the name `tom`. The results are restricted to users with more than 42 repositories and over 1,000 followers.
     *
     * FROM: <https://docs.github.com/rest/reference/search#search-users>
     *
     * **Parameters:**
     *
     * * `q: &str` -- The query contains one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as GitHub.com. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/reference/search#constructing-a-search-query). See "[Searching users](https://help.github.com/articles/searching-users/)" for a detailed list of qualifiers.
     * * `sort: crate::types::SearchUsersSort` -- Sorts the results of your query by number of `followers` or `repositories`, or when the person `joined` GitHub. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results).
     * * `order: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn users(
        &self,
        q: &str,
        sort: crate::types::SearchUsersSort,
        order: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::SearchUsersResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/search/users?{}", query_), None);
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

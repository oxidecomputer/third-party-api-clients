use anyhow::Result;

use crate::Client;

pub struct Search {
    client: Client,
}

impl Search {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Search { client }
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
    ) -> Result<crate::types::GetSearchUsersOkResponse> {
        let url = format!(
            "/search/users?order={}&page={}&per_page={}&q={}&sort={}",
            order,
            format!("{}", page),
            format!("{}", per_page),
            q.to_string(),
            sort,
        );

        self.client.get(&url).await
    }
}

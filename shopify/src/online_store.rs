use crate::Client;
use crate::ClientResult;

pub struct OnlineStore {
    pub client: Client,
}

impl OnlineStore {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        OnlineStore { client }
    }

    /**
     * Retrieves a list of all articles from a blog. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#index-2020-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Retrieve results based on their published status.
     *                     (default: any)
     *                       
     *                           published: Show only published articles.
     *                           unpublished: Show only unpublished articles.
     *                           any: Show articles of any published status.
     * * `handle: &str` -- Retrieve an article with a specific handle.
     * * `tag: &str` -- Filter articles with a specific tag.
     * * `author: &str` -- Filter articles by article author.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_blogs_param_blog_article(
        &self,
        blog_id: &str,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        handle: &str,
        tag: &str,
        author: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !tag.is_empty() {
            query_args.push(("tag".to_string(), tag.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/blogs/{}/articles.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Creates an article for a blog.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#create-2020-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_blogs_param_blog_articles(
        &self,
        blog_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/blogs/{}/articles.json",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a count of all articles from a blog.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/blogs/{blog_id}/articles/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#count-2020-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count articles with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published articles.
     *                           unpublished: Count only unpublished articles.
     *                           any: Count all articles.
     */
    pub async fn deprecated_202001_get_blogs_param_blog_articles_count(
        &self,
        blog_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/blogs/{}/articles/count.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a single article.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#show-2020-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specifed by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/blogs/{}/articles/{}/json?{}",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Updates an article.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#update-2020-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Deletes an article.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Retrieves a list all of article authors.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/articles/authors.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#authors-2020-01
     */
    pub async fn deprecated_202001_get_articles_author(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/articles/authors.json", None);
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
     * Retrieves a list of all the tags.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/articles/tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#tags-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of tags to retrieve.
     * * `popular: &str` -- A flag for ordering retrieved tags. If present in the request, then the results will be ordered by popularity, starting with the most popular tag.
     */
    pub async fn deprecated_202001_get_articles_tag(
        &self,
        limit: &str,
        popular: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !popular.is_empty() {
            query_args.push(("popular".to_string(), popular.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/articles/tags.json?{}", query_),
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
     * Retrieves a list of all articles from a blog. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#index-2020-04
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Retrieve results based on their published status.
     *                     (default: any)
     *                       
     *                           published: Show only published articles.
     *                           unpublished: Show only unpublished articles.
     *                           any: Show articles of any published status.
     * * `handle: &str` -- Retrieve an article with a specific handle.
     * * `tag: &str` -- Filter articles with a specific tag.
     * * `author: &str` -- Filter articles by article author.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_blogs_param_blog_article(
        &self,
        blog_id: &str,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        handle: &str,
        tag: &str,
        author: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !tag.is_empty() {
            query_args.push(("tag".to_string(), tag.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/blogs/{}/articles.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Creates an article for a blog.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#create-2020-04
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_blogs_param_blog_articles(
        &self,
        blog_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/blogs/{}/articles.json",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a count of all articles from a blog.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/blogs/{blog_id}/articles/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#count-2020-04
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count articles with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published articles.
     *                           unpublished: Count only unpublished articles.
     *                           any: Count all articles.
     */
    pub async fn deprecated_202004_get_blogs_param_blog_articles_count(
        &self,
        blog_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/blogs/{}/articles/count.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a single article.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#show-2020-04
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specifed by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/blogs/{}/articles/{}/json?{}",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Updates an article.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#update-2020-04
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Deletes an article.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Retrieves a list all of article authors.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/articles/authors.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#authors-2020-04
     */
    pub async fn deprecated_202004_get_articles_author(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/articles/authors.json", None);
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
     * Retrieves a list of all the tags.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/articles/tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#tags-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of tags to retrieve.
     * * `popular: &str` -- A flag for ordering retrieved tags. If present in the request, then the results will be ordered by popularity, starting with the most popular tag.
     */
    pub async fn deprecated_202004_get_articles_tag(
        &self,
        limit: &str,
        popular: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !popular.is_empty() {
            query_args.push(("popular".to_string(), popular.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/articles/tags.json?{}", query_),
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
     * Retrieves a list of all articles from a blog. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#index-2020-07
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Retrieve results based on their published status.
     *                     (default: any)
     *                       
     *                           published: Show only published articles.
     *                           unpublished: Show only unpublished articles.
     *                           any: Show articles of any published status.
     * * `handle: &str` -- Retrieve an article with a specific handle.
     * * `tag: &str` -- Filter articles with a specific tag.
     * * `author: &str` -- Filter articles by article author.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_blogs_param_blog_article(
        &self,
        blog_id: &str,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        handle: &str,
        tag: &str,
        author: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !tag.is_empty() {
            query_args.push(("tag".to_string(), tag.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/blogs/{}/articles.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Creates an article for a blog.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#create-2020-07
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_blogs_param_blog_articles(
        &self,
        blog_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/blogs/{}/articles.json",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a count of all articles from a blog.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/blogs/{blog_id}/articles/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#count-2020-07
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count articles with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published articles.
     *                           unpublished: Count only unpublished articles.
     *                           any: Count all articles.
     */
    pub async fn deprecated_202007_get_blogs_param_blog_articles_count(
        &self,
        blog_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/blogs/{}/articles/count.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a single article.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#show-2020-07
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specifed by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/blogs/{}/articles/{}/json?{}",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Updates an article.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#update-2020-07
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Deletes an article.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Retrieves a list all of article authors.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/articles/authors.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#authors-2020-07
     */
    pub async fn deprecated_202007_get_articles_author(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/articles/authors.json", None);
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
     * Retrieves a list of all the tags.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/articles/tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#tags-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of tags to retrieve.
     * * `popular: &str` -- A flag for ordering retrieved tags. If present in the request, then the results will be ordered by popularity, starting with the most popular tag.
     */
    pub async fn deprecated_202007_get_articles_tag(
        &self,
        limit: &str,
        popular: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !popular.is_empty() {
            query_args.push(("popular".to_string(), popular.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/articles/tags.json?{}", query_),
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
     * Retrieves a list of all articles from a blog. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#index-2020-10
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Retrieve results based on their published status.
     *                     (default: any)
     *                       
     *                           published: Show only published articles.
     *                           unpublished: Show only unpublished articles.
     *                           any: Show articles of any published status.
     * * `handle: &str` -- Retrieve an article with a specific handle.
     * * `tag: &str` -- Filter articles with a specific tag.
     * * `author: &str` -- Filter articles by article author.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_blogs_param_blog_article(
        &self,
        blog_id: &str,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        handle: &str,
        tag: &str,
        author: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !tag.is_empty() {
            query_args.push(("tag".to_string(), tag.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/blogs/{}/articles.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Creates an article for a blog.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#create-2020-10
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_blogs_param_blog_articles(
        &self,
        blog_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/blogs/{}/articles.json",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a count of all articles from a blog.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/blogs/{blog_id}/articles/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#count-2020-10
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count articles with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published articles.
     *                           unpublished: Count only unpublished articles.
     *                           any: Count all articles.
     */
    pub async fn get_blogs_param_blog_articles_count(
        &self,
        blog_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/blogs/{}/articles/count.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a single article.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#show-2020-10
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specifed by a comma-separated list of field names.
     */
    pub async fn get_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/blogs/{}/articles/{}/json?{}",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Updates an article.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#update-2020-10
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Deletes an article.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Retrieves a list all of article authors.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/articles/authors.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#authors-2020-10
     */
    pub async fn get_articles_author(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/articles/authors.json", None);
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
     * Retrieves a list of all the tags.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/articles/tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#tags-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of tags to retrieve.
     * * `popular: &str` -- A flag for ordering retrieved tags. If present in the request, then the results will be ordered by popularity, starting with the most popular tag.
     */
    pub async fn get_articles_tag(&self, limit: &str, popular: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !popular.is_empty() {
            query_args.push(("popular".to_string(), popular.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/articles/tags.json?{}", query_),
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
     * Retrieves a list of all articles from a blog. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#index-2021-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Retrieve results based on their published status.
     *                     (default: any)
     *                       
     *                           published: Show only published articles.
     *                           unpublished: Show only unpublished articles.
     *                           any: Show articles of any published status.
     * * `handle: &str` -- Retrieve an article with a specific handle.
     * * `tag: &str` -- Filter articles with a specific tag.
     * * `author: &str` -- Filter articles by article author.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_blogs_param_blog_article(
        &self,
        blog_id: &str,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        handle: &str,
        tag: &str,
        author: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !tag.is_empty() {
            query_args.push(("tag".to_string(), tag.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/blogs/{}/articles.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Creates an article for a blog.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#create-2021-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_blogs_param_blog_articles(
        &self,
        blog_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/blogs/{}/articles.json",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a count of all articles from a blog.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/blogs/{blog_id}/articles/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#count-2021-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count articles with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published articles.
     *                           unpublished: Count only unpublished articles.
     *                           any: Count all articles.
     */
    pub async fn deprecated_202101_get_blogs_param_blog_articles_count(
        &self,
        blog_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/blogs/{}/articles/count.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a single article.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#show-2021-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specifed by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/blogs/{}/articles/{}/json?{}",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Updates an article.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#update-2021-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Deletes an article.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Retrieves a list all of article authors.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/articles/authors.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#authors-2021-01
     */
    pub async fn deprecated_202101_get_articles_author(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/articles/authors.json", None);
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
     * Retrieves a list of all the tags.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/articles/tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#tags-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of tags to retrieve.
     * * `popular: &str` -- A flag for ordering retrieved tags. If present in the request, then the results will be ordered by popularity, starting with the most popular tag.
     */
    pub async fn deprecated_202101_get_articles_tag(
        &self,
        limit: &str,
        popular: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !popular.is_empty() {
            query_args.push(("popular".to_string(), popular.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/articles/tags.json?{}", query_),
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
     * Retrieves a list of all articles from a blog. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#index-unstable
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Retrieve results based on their published status.
     *                     (default: any)
     *                       
     *                           published: Show only published articles.
     *                           unpublished: Show only unpublished articles.
     *                           any: Show articles of any published status.
     * * `handle: &str` -- Retrieve an article with a specific handle.
     * * `tag: &str` -- Filter articles with a specific tag.
     * * `author: &str` -- Filter articles by article author.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_blogs_param_blog_article(
        &self,
        blog_id: &str,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        handle: &str,
        tag: &str,
        author: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !tag.is_empty() {
            query_args.push(("tag".to_string(), tag.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/blogs/{}/articles.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Creates an article for a blog.
     *
     * This function performs a `POST` to the `/admin/api/unstable/blogs/{blog_id}/articles.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#create-unstable
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_blogs_param_blog_articles(
        &self,
        blog_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/blogs/{}/articles.json",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a count of all articles from a blog.
     *
     * This function performs a `GET` to the `/admin/api/unstable/blogs/{blog_id}/articles/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#count-unstable
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count articles created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count articles created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count articles last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count articles last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count articles published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count articles published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count articles with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published articles.
     *                           unpublished: Count only unpublished articles.
     *                           any: Count all articles.
     */
    pub async fn deprecated_unstable_get_blogs_param_blog_articles_count(
        &self,
        blog_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/blogs/{}/articles/count.json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a single article.
     *
     * This function performs a `GET` to the `/admin/api/unstable/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#show-unstable
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specifed by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/blogs/{}/articles/{}/json?{}",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Updates an article.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#update-unstable
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Deletes an article.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/blogs/{blog_id}/articles/{article_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#destroy-unstable
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `article_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_blogs_param_blog_articles_article(
        &self,
        blog_id: &str,
        article_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/blogs/{}/articles/{}/json",
                crate::progenitor_support::encode_path(blog_id),
                crate::progenitor_support::encode_path(article_id),
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
     * Retrieves a list all of article authors.
     *
     * This function performs a `GET` to the `/admin/api/unstable/articles/authors.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#authors-unstable
     */
    pub async fn deprecated_unstable_get_articles_author(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/articles/authors.json", None);
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
     * Retrieves a list of all the tags.
     *
     * This function performs a `GET` to the `/admin/api/unstable/articles/tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/article#tags-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of tags to retrieve.
     * * `popular: &str` -- A flag for ordering retrieved tags. If present in the request, then the results will be ordered by popularity, starting with the most popular tag.
     */
    pub async fn deprecated_unstable_get_articles_tag(
        &self,
        limit: &str,
        popular: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !popular.is_empty() {
            query_args.push(("popular".to_string(), popular.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/articles/tags.json?{}", query_),
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
    * Retrieves a single asset for a theme by its key.
              To retrieve a single asset, include asset[key]=#{asset_key} as a request parameter. For example, to retrieve the asset with a key of templates/index.liquid, the request might be /admin/themes/828155753/assets.json?asset[key]=templates/index.liquid.
              For more information on the key property, see Asset properties.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#show-2020-01
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    * * `asset_key: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_get_themes_param_theme_asset(
        &self,
        theme_id: &str,
        fields: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
    * Creates or updates an asset for a theme.
              In the PUT request, you can include the src or source_key property to create the asset from an existing file.
    *
    * This function performs a `PUT` to the `/admin/api/2020-01/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#update-2020-01
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `src: &str` -- The source URL of an image. Include in the body of the PUT request to upload the image to Shopify.
    * * `source_key: &str` -- The path within the theme to an existing asset. Include in the body of the PUT request to create a duplicate asset.
    */
    pub async fn deprecated_202001_update_themes_param_theme_assets(
        &self,
        theme_id: &str,
        src: &str,
        source_key: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !source_key.is_empty() {
            query_args.push(("source_key".to_string(), source_key.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
     * Deletes an asset from a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/themes/{theme_id}/assets.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `asset_key: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_themes_param_theme_assets(
        &self,
        theme_id: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
    * Retrieves a single asset for a theme by its key.
              To retrieve a single asset, include asset[key]=#{asset_key} as a request parameter. For example, to retrieve the asset with a key of templates/index.liquid, the request might be /admin/themes/828155753/assets.json?asset[key]=templates/index.liquid.
              For more information on the key property, see Asset properties.
    *
    * This function performs a `GET` to the `/admin/api/2020-04/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#show-2020-04
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    * * `asset_key: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_get_themes_param_theme_asset(
        &self,
        theme_id: &str,
        fields: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
    * Creates or updates an asset for a theme.
              In the PUT request, you can include the src or source_key property to create the asset from an existing file.
    *
    * This function performs a `PUT` to the `/admin/api/2020-04/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#update-2020-04
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `src: &str` -- The source URL of an image. Include in the body of the PUT request to upload the image to Shopify.
    * * `source_key: &str` -- The path within the theme to an existing asset. Include in the body of the PUT request to create a duplicate asset.
    */
    pub async fn deprecated_202004_update_themes_param_theme_assets(
        &self,
        theme_id: &str,
        src: &str,
        source_key: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !source_key.is_empty() {
            query_args.push(("source_key".to_string(), source_key.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
     * Deletes an asset from a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/themes/{theme_id}/assets.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `asset_key: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_themes_param_theme_assets(
        &self,
        theme_id: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
    * Retrieves a single asset for a theme by its key.
              To retrieve a single asset, include asset[key]=#{asset_key} as a request parameter. For example, to retrieve the asset with a key of templates/index.liquid, the request might be /admin/themes/828155753/assets.json?asset[key]=templates/index.liquid.
              For more information on the key property, see Asset properties.
    *
    * This function performs a `GET` to the `/admin/api/2020-07/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#show-2020-07
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    * * `asset_key: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_get_themes_param_theme_asset(
        &self,
        theme_id: &str,
        fields: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
    * Creates or updates an asset for a theme.
              In the PUT request, you can include the src or source_key property to create the asset from an existing file.
    *
    * This function performs a `PUT` to the `/admin/api/2020-07/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#update-2020-07
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `src: &str` -- The source URL of an image. Include in the body of the PUT request to upload the image to Shopify.
    * * `source_key: &str` -- The path within the theme to an existing asset. Include in the body of the PUT request to create a duplicate asset.
    */
    pub async fn deprecated_202007_update_themes_param_theme_assets(
        &self,
        theme_id: &str,
        src: &str,
        source_key: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !source_key.is_empty() {
            query_args.push(("source_key".to_string(), source_key.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
     * Deletes an asset from a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/themes/{theme_id}/assets.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `asset_key: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_themes_param_theme_assets(
        &self,
        theme_id: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
    * Retrieves a single asset for a theme by its key.
              To retrieve a single asset, include asset[key]=#{asset_key} as a request parameter. For example, to retrieve the asset with a key of templates/index.liquid, the request might be /admin/themes/828155753/assets.json?asset[key]=templates/index.liquid.
              For more information on the key property, see Asset properties.
    *
    * This function performs a `GET` to the `/admin/api/2020-10/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#show-2020-10
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    * * `asset_key: &str` -- storefront_access_token_id.
    */
    pub async fn get_themes_param_theme_asset(
        &self,
        theme_id: &str,
        fields: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
    * Creates or updates an asset for a theme.
              In the PUT request, you can include the src or source_key property to create the asset from an existing file.
    *
    * This function performs a `PUT` to the `/admin/api/2020-10/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#update-2020-10
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `src: &str` -- The source URL of an image. Include in the body of the PUT request to upload the image to Shopify.
    * * `source_key: &str` -- The path within the theme to an existing asset. Include in the body of the PUT request to create a duplicate asset.
    */
    pub async fn update_themes_param_theme_assets(
        &self,
        theme_id: &str,
        src: &str,
        source_key: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !source_key.is_empty() {
            query_args.push(("source_key".to_string(), source_key.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
     * Deletes an asset from a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/themes/{theme_id}/assets.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `asset_key: &str` -- storefront_access_token_id.
     */
    pub async fn delete_themes_param_theme_assets(
        &self,
        theme_id: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
    * Retrieves a single asset for a theme by its key.
              To retrieve a single asset, include asset[key]=#{asset_key} as a request parameter. For example, to retrieve the asset with a key of templates/index.liquid, the request might be /admin/themes/828155753/assets.json?asset[key]=templates/index.liquid.
              For more information on the key property, see Asset properties.
    *
    * This function performs a `GET` to the `/admin/api/2021-01/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#show-2021-01
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    * * `asset_key: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_get_themes_param_theme_asset(
        &self,
        theme_id: &str,
        fields: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
    * Creates or updates an asset for a theme.
              In the PUT request, you can include the src or source_key property to create the asset from an existing file.
    *
    * This function performs a `PUT` to the `/admin/api/2021-01/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#update-2021-01
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `src: &str` -- The source URL of an image. Include in the body of the PUT request to upload the image to Shopify.
    * * `source_key: &str` -- The path within the theme to an existing asset. Include in the body of the PUT request to create a duplicate asset.
    */
    pub async fn deprecated_202101_update_themes_param_theme_assets(
        &self,
        theme_id: &str,
        src: &str,
        source_key: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !source_key.is_empty() {
            query_args.push(("source_key".to_string(), source_key.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
     * Deletes an asset from a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/themes/{theme_id}/assets.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `asset_key: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_themes_param_theme_assets(
        &self,
        theme_id: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
    * Retrieves a single asset for a theme by its key.
              To retrieve a single asset, include asset[key]=#{asset_key} as a request parameter. For example, to retrieve the asset with a key of templates/index.liquid, the request might be /admin/themes/828155753/assets.json?asset[key]=templates/index.liquid.
              For more information on the key property, see Asset properties.
    *
    * This function performs a `GET` to the `/admin/api/unstable/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#show-unstable
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
    * * `asset_key: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_get_themes_param_theme_asset(
        &self,
        theme_id: &str,
        fields: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
    * Creates or updates an asset for a theme.
              In the PUT request, you can include the src or source_key property to create the asset from an existing file.
    *
    * This function performs a `PUT` to the `/admin/api/unstable/themes/{theme_id}/assets.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#update-unstable
    *
    * **Parameters:**
    *
    * * `theme_id: &str` -- storefront_access_token_id.
    * * `src: &str` -- The source URL of an image. Include in the body of the PUT request to upload the image to Shopify.
    * * `source_key: &str` -- The path within the theme to an existing asset. Include in the body of the PUT request to create a duplicate asset.
    */
    pub async fn deprecated_unstable_update_themes_param_theme_assets(
        &self,
        theme_id: &str,
        src: &str,
        source_key: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !source_key.is_empty() {
            query_args.push(("source_key".to_string(), source_key.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
     * Deletes an asset from a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/themes/{theme_id}/assets.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/asset#destroy-unstable
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `asset_key: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_themes_param_theme_assets(
        &self,
        theme_id: &str,
        asset_key: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !asset_key.is_empty() {
            query_args.push(("asset[key]".to_string(), asset_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/themes/{}/assets.json?{}",
                crate::progenitor_support::encode_path(theme_id),
                query_
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
     * Retrieve a list of all blogs. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/blogs.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/blog#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `handle: &str` -- Filter by blog handle.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_blog(
        &self,
        limit: &str,
        since_id: &str,
        handle: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-01/blogs.json?{}", query_), None);
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
     * Create a new blog.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/blogs.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/blog#create-2020-01
     */
    pub async fn deprecated_202001_create_blogs(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/blogs.json", None);
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
     * Get a count of all blogs.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/blogs/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/blog#count-2020-01
     */
    pub async fn deprecated_202001_get_blogs_count(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/blogs/count.json", None);
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
     * Get a single blog by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/blogs/{blog_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/blog#show-2020-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_blogs_param_blog(
        &self,
        blog_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/blogs/{}/json?{}",
                crate::progenitor_support::encode_path(blog_id),
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
     * Update a blog.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/blogs/{blog_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/blog#update-2020-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_blogs_param_blog(
        &self,
        blog_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/blogs/{}/json",
                crate::progenitor_support::encode_path(blog_id),
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
     * Delete a blog.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/blogs/{blog_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/blog#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `blog_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_blogs_param_blog(
        &self,
        blog_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/blogs/{}/json",
                crate::progenitor_support::encode_path(blog_id),
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
     * Retrieves a list of comments. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/comments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show comments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show comments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show comments last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show comments last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show comments published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show comments published before date (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `published_status: &str` -- Filter results by their published status.
     *                     (default: any)
     *                       
     *                           published: Show only published comments.
     *                           unpublished: Show only unpublished comments.
     *                           any: Show comments of any published status.
     * * `status: &str` -- Filter results by their status.
     *                       
     *                           pending: Show only pending comments.
     *                           published: Show only published comments.
     *                           unapproved: Show only unapproved comments.
     * * `article_id: i64` -- recurring_application_charge[capped_amount].
     * * `blog_id: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202001_get_comment(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        fields: &str,
        published_status: &str,
        status: &str,
        article_id: i64,
        blog_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if article_id > 0 {
            query_args.push(("article_id".to_string(), article_id.to_string()));
        }
        if blog_id > 0 {
            query_args.push(("blog_id".to_string(), blog_id.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/comments.json?{}", query_),
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
     * Creates a comment for an article.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/comments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#create-2020-01
     */
    pub async fn deprecated_202001_create_comments(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/comments.json", None);
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
     * Retrieves a count of comments.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/comments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#count-2020-01
     *
     * **Parameters:**
     *
     * * `created_at_min: &str` -- Count comments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count comments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count comments last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count comments last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count comments published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count comments published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Retrieve a count of comments with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published comments.
     *                           unpublished: Count only unpublished comments.
     *                           any: Count comments of any published status.
     * * `status: &str` -- Retrieve a count of comments with a given status.
     *                       
     *                           pending: Count pending comments.
     *                           published: Count published comments.
     *                           unapproved: Count unapproved comments.
     * * `article_id: i64` -- recurring_application_charge[capped_amount].
     * * `blog_id: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202001_get_comments_count(
        &self,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        status: &str,
        article_id: i64,
        blog_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if article_id > 0 {
            query_args.push(("article_id".to_string(), article_id.to_string()));
        }
        if blog_id > 0 {
            query_args.push(("blog_id".to_string(), blog_id.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/comments/count.json?{}", query_),
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
     * Retrieves a single comment by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/comments/{comment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#show-2020-01
     *
     * **Parameters:**
     *
     * * `comment_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_comments_param_comment(
        &self,
        comment_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/comments/{}/json?{}",
                crate::progenitor_support::encode_path(comment_id),
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
     * Updates a comment of an article.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/comments/{comment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#update-2020-01
     *
     * **Parameters:**
     *
     * * `comment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_comments_param_comment(
        &self,
        comment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/comments/{}/json",
                crate::progenitor_support::encode_path(comment_id),
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
     * Marks a comment as spam.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/comments/{comment_id}/spam.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#spam-2020-01
     *
     * **Parameters:**
     *
     * * `comment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_comments_param_comment_spam(
        &self,
        comment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/comments/{}/spam.json",
                crate::progenitor_support::encode_path(comment_id),
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
     * Marks a comment as not spam.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/comments/{comment_id}/not_spam.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#not_spam-2020-01
     *
     * **Parameters:**
     *
     * * `comment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_comments_param_comment_not_spam(
        &self,
        comment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/comments/{}/not_spam.json",
                crate::progenitor_support::encode_path(comment_id),
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
     * Approves a comment.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/comments/{comment_id}/approve.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#approve-2020-01
     *
     * **Parameters:**
     *
     * * `comment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_comments_param_comment_approve(
        &self,
        comment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/comments/{}/approve.json",
                crate::progenitor_support::encode_path(comment_id),
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
     * Removes a comment.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/comments/{comment_id}/remove.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#remove-2020-01
     *
     * **Parameters:**
     *
     * * `comment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_comments_param_comment_remove(
        &self,
        comment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/comments/{}/remove.json",
                crate::progenitor_support::encode_path(comment_id),
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
     * Restores a previously removed comment.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/comments/{comment_id}/restore.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/comment#restore-2020-01
     *
     * **Parameters:**
     *
     * * `comment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_comments_param_comment_restore(
        &self,
        comment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/comments/{}/restore.json",
                crate::progenitor_support::encode_path(comment_id),
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
     * Retrieves a list of URL redirects. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/redirects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `path: &str` -- Show redirects with a given path.
     * * `target: &str` -- Show redirects with a given target.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_redirect(
        &self,
        limit: &str,
        since_id: &str,
        path: &str,
        target: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/redirects.json?{}", query_),
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
    * Creates a redirect. When you provide a full URL as the value of the path property, it will be saved as an absolute path without the domain.
              For example, "path": "http://www.johns-apparel.com/springwear" will be saved as "path": "springwear".
    *
    * This function performs a `POST` to the `/admin/api/2020-01/redirects.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#create-2020-01
    */
    pub async fn deprecated_202001_create_redirects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/redirects.json", None);
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
     * Retrieves a count of URL redirects.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/redirects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#count-2020-01
     *
     * **Parameters:**
     *
     * * `path: &str` -- Count redirects with given path.
     * * `target: &str` -- Count redirects with given target.
     */
    pub async fn deprecated_202001_get_redirects_count(
        &self,
        path: &str,
        target: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/redirects/count.json?{}", query_),
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
     * Retrieves a single redirect.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#show-2020-01
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_redirects_param_redirect(
        &self,
        redirect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/redirects/{}/json?{}",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Updates an existing redirect.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#update-2020-01
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_redirects_param_redirect(
        &self,
        redirect_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Deletes a redirect.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_redirects_param_redirect(
        &self,
        redirect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Retrieves a list of URL redirects. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/redirects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `path: &str` -- Show redirects with a given path.
     * * `target: &str` -- Show redirects with a given target.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_redirect(
        &self,
        limit: &str,
        since_id: &str,
        path: &str,
        target: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/redirects.json?{}", query_),
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
    * Creates a redirect. When you provide a full URL as the value of the path property, it will be saved as an absolute path without the domain.
              For example, "path": "http://www.johns-apparel.com/springwear" will be saved as "path": "springwear".
    *
    * This function performs a `POST` to the `/admin/api/2020-04/redirects.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#create-2020-04
    */
    pub async fn deprecated_202004_create_redirects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/redirects.json", None);
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
     * Retrieves a count of URL redirects.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/redirects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#count-2020-04
     *
     * **Parameters:**
     *
     * * `path: &str` -- Count redirects with given path.
     * * `target: &str` -- Count redirects with given target.
     */
    pub async fn deprecated_202004_get_redirects_count(
        &self,
        path: &str,
        target: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/redirects/count.json?{}", query_),
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
     * Retrieves a single redirect.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#show-2020-04
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_redirects_param_redirect(
        &self,
        redirect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/redirects/{}/json?{}",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Updates an existing redirect.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#update-2020-04
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_redirects_param_redirect(
        &self,
        redirect_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Deletes a redirect.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_redirects_param_redirect(
        &self,
        redirect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Retrieves a list of URL redirects. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/redirects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `path: &str` -- Show redirects with a given path.
     * * `target: &str` -- Show redirects with a given target.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_redirect(
        &self,
        limit: &str,
        since_id: &str,
        path: &str,
        target: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/redirects.json?{}", query_),
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
    * Creates a redirect. When you provide a full URL as the value of the path property, it will be saved as an absolute path without the domain.
              For example, "path": "http://www.johns-apparel.com/springwear" will be saved as "path": "springwear".
    *
    * This function performs a `POST` to the `/admin/api/2020-07/redirects.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#create-2020-07
    */
    pub async fn deprecated_202007_create_redirects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/redirects.json", None);
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
     * Retrieves a count of URL redirects.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/redirects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#count-2020-07
     *
     * **Parameters:**
     *
     * * `path: &str` -- Count redirects with given path.
     * * `target: &str` -- Count redirects with given target.
     */
    pub async fn deprecated_202007_get_redirects_count(
        &self,
        path: &str,
        target: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/redirects/count.json?{}", query_),
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
     * Retrieves a single redirect.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#show-2020-07
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_redirects_param_redirect(
        &self,
        redirect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/redirects/{}/json?{}",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Updates an existing redirect.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#update-2020-07
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_redirects_param_redirect(
        &self,
        redirect_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Deletes a redirect.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_redirects_param_redirect(
        &self,
        redirect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Retrieves a list of URL redirects. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/redirects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `path: &str` -- Show redirects with a given path.
     * * `target: &str` -- Show redirects with a given target.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_redirect(
        &self,
        limit: &str,
        since_id: &str,
        path: &str,
        target: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/redirects.json?{}", query_),
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
    * Creates a redirect. When you provide a full URL as the value of the path property, it will be saved as an absolute path without the domain.
              For example, "path": "http://www.johns-apparel.com/springwear" will be saved as "path": "springwear".
    *
    * This function performs a `POST` to the `/admin/api/2020-10/redirects.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#create-2020-10
    */
    pub async fn create_redirects(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/redirects.json", None);
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
     * Retrieves a count of URL redirects.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/redirects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#count-2020-10
     *
     * **Parameters:**
     *
     * * `path: &str` -- Count redirects with given path.
     * * `target: &str` -- Count redirects with given target.
     */
    pub async fn get_redirects_count(&self, path: &str, target: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/redirects/count.json?{}", query_),
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
     * Retrieves a single redirect.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#show-2020-10
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_redirects_param_redirect(
        &self,
        redirect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/redirects/{}/json?{}",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Updates an existing redirect.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#update-2020-10
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_redirects_param_redirect(
        &self,
        redirect_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Deletes a redirect.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_redirects_param_redirect(&self, redirect_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Retrieves a list of URL redirects. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/redirects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `path: &str` -- Show redirects with a given path.
     * * `target: &str` -- Show redirects with a given target.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_redirect(
        &self,
        limit: &str,
        since_id: &str,
        path: &str,
        target: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/redirects.json?{}", query_),
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
    * Creates a redirect. When you provide a full URL as the value of the path property, it will be saved as an absolute path without the domain.
              For example, "path": "http://www.johns-apparel.com/springwear" will be saved as "path": "springwear".
    *
    * This function performs a `POST` to the `/admin/api/2021-01/redirects.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#create-2021-01
    */
    pub async fn deprecated_202101_create_redirects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/redirects.json", None);
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
     * Retrieves a count of URL redirects.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/redirects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#count-2021-01
     *
     * **Parameters:**
     *
     * * `path: &str` -- Count redirects with given path.
     * * `target: &str` -- Count redirects with given target.
     */
    pub async fn deprecated_202101_get_redirects_count(
        &self,
        path: &str,
        target: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/redirects/count.json?{}", query_),
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
     * Retrieves a single redirect.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#show-2021-01
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_redirects_param_redirect(
        &self,
        redirect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/redirects/{}/json?{}",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Updates an existing redirect.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#update-2021-01
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_redirects_param_redirect(
        &self,
        redirect_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Deletes a redirect.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_redirects_param_redirect(
        &self,
        redirect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Retrieves a list of URL redirects. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/redirects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `path: &str` -- Show redirects with a given path.
     * * `target: &str` -- Show redirects with a given target.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_redirect(
        &self,
        limit: &str,
        since_id: &str,
        path: &str,
        target: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/redirects.json?{}", query_),
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
    * Creates a redirect. When you provide a full URL as the value of the path property, it will be saved as an absolute path without the domain.
              For example, "path": "http://www.johns-apparel.com/springwear" will be saved as "path": "springwear".
    *
    * This function performs a `POST` to the `/admin/api/unstable/redirects.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#create-unstable
    */
    pub async fn deprecated_unstable_create_redirects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/redirects.json", None);
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
     * Retrieves a count of URL redirects.
     *
     * This function performs a `GET` to the `/admin/api/unstable/redirects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#count-unstable
     *
     * **Parameters:**
     *
     * * `path: &str` -- Count redirects with given path.
     * * `target: &str` -- Count redirects with given target.
     */
    pub async fn deprecated_unstable_get_redirects_count(
        &self,
        path: &str,
        target: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !target.is_empty() {
            query_args.push(("target".to_string(), target.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/redirects/count.json?{}", query_),
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
     * Retrieves a single redirect.
     *
     * This function performs a `GET` to the `/admin/api/unstable/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#show-unstable
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_redirects_param_redirect(
        &self,
        redirect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/redirects/{}/json?{}",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Updates an existing redirect.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#update-unstable
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_redirects_param_redirect(
        &self,
        redirect_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Deletes a redirect.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/redirects/{redirect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/redirect#destroy-unstable
     *
     * **Parameters:**
     *
     * * `redirect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_redirects_param_redirect(
        &self,
        redirect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/redirects/{}/json",
                crate::progenitor_support::encode_path(redirect_id),
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
     * Retrieves a list of all script tags. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show script tags created after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show script tags created before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show script tags last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show script tags last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `src: &str` -- Show script tags with this URL.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_script_tag(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        src: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/script_tags.json?{}", query_),
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
     * Creates a new script tag.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#create-2020-01
     */
    pub async fn deprecated_202001_create_script_tags(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/script_tags.json", None);
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
     * Retrieves a count of all script tags.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/script_tags/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#count-2020-01
     *
     * **Parameters:**
     *
     * * `src: &str` -- Count only script tags with a given URL.
     */
    pub async fn deprecated_202001_get_script_tags_count(&self, src: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/script_tags/count.json?{}", query_),
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
     * Retrieves a single script tag.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#show-2020-01
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/script_tags/{}/json?{}",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Updates a script tag.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#update-2020-01
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Deletes a script tag.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_script_tags_param_tag(
        &self,
        script_tag_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Retrieves a list of all script tags. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show script tags created after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show script tags created before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show script tags last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show script tags last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `src: &str` -- Show script tags with this URL.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_script_tag(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        src: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/script_tags.json?{}", query_),
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
     * Creates a new script tag.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#create-2020-04
     */
    pub async fn deprecated_202004_create_script_tags(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/script_tags.json", None);
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
     * Retrieves a count of all script tags.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/script_tags/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#count-2020-04
     *
     * **Parameters:**
     *
     * * `src: &str` -- Count only script tags with a given URL.
     */
    pub async fn deprecated_202004_get_script_tags_count(&self, src: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/script_tags/count.json?{}", query_),
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
     * Retrieves a single script tag.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#show-2020-04
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/script_tags/{}/json?{}",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Updates a script tag.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#update-2020-04
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Deletes a script tag.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_script_tags_param_tag(
        &self,
        script_tag_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Retrieves a list of all script tags. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show script tags created after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show script tags created before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show script tags last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show script tags last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `src: &str` -- Show script tags with this URL.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_script_tag(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        src: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/script_tags.json?{}", query_),
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
     * Creates a new script tag.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#create-2020-07
     */
    pub async fn deprecated_202007_create_script_tags(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/script_tags.json", None);
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
     * Retrieves a count of all script tags.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/script_tags/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#count-2020-07
     *
     * **Parameters:**
     *
     * * `src: &str` -- Count only script tags with a given URL.
     */
    pub async fn deprecated_202007_get_script_tags_count(&self, src: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/script_tags/count.json?{}", query_),
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
     * Retrieves a single script tag.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#show-2020-07
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/script_tags/{}/json?{}",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Updates a script tag.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#update-2020-07
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Deletes a script tag.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_script_tags_param_tag(
        &self,
        script_tag_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Retrieves a list of all script tags. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show script tags created after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show script tags created before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show script tags last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show script tags last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `src: &str` -- Show script tags with this URL.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_script_tag(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        src: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/script_tags.json?{}", query_),
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
     * Creates a new script tag.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#create-2020-10
     */
    pub async fn create_script_tags(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/script_tags.json", None);
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
     * Retrieves a count of all script tags.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/script_tags/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#count-2020-10
     *
     * **Parameters:**
     *
     * * `src: &str` -- Count only script tags with a given URL.
     */
    pub async fn get_script_tags_count(&self, src: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/script_tags/count.json?{}", query_),
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
     * Retrieves a single script tag.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#show-2020-10
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/script_tags/{}/json?{}",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Updates a script tag.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#update-2020-10
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Deletes a script tag.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_script_tags_param_tag(&self, script_tag_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Retrieves a list of all script tags. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show script tags created after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show script tags created before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show script tags last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show script tags last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `src: &str` -- Show script tags with this URL.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_script_tag(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        src: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/script_tags.json?{}", query_),
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
     * Creates a new script tag.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#create-2021-01
     */
    pub async fn deprecated_202101_create_script_tags(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/script_tags.json", None);
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
     * Retrieves a count of all script tags.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/script_tags/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#count-2021-01
     *
     * **Parameters:**
     *
     * * `src: &str` -- Count only script tags with a given URL.
     */
    pub async fn deprecated_202101_get_script_tags_count(&self, src: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/script_tags/count.json?{}", query_),
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
     * Retrieves a single script tag.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#show-2021-01
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/script_tags/{}/json?{}",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Updates a script tag.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#update-2021-01
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Deletes a script tag.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_script_tags_param_tag(
        &self,
        script_tag_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Retrieves a list of all script tags. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to return.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Show script tags created after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show script tags created before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show script tags last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show script tags last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `src: &str` -- Show script tags with this URL.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_script_tag(
        &self,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        src: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/script_tags.json?{}", query_),
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
     * Creates a new script tag.
     *
     * This function performs a `POST` to the `/admin/api/unstable/script_tags.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#create-unstable
     */
    pub async fn deprecated_unstable_create_script_tags(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/script_tags.json", None);
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
     * Retrieves a count of all script tags.
     *
     * This function performs a `GET` to the `/admin/api/unstable/script_tags/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#count-unstable
     *
     * **Parameters:**
     *
     * * `src: &str` -- Count only script tags with a given URL.
     */
    pub async fn deprecated_unstable_get_script_tags_count(&self, src: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !src.is_empty() {
            query_args.push(("src".to_string(), src.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/script_tags/count.json?{}", query_),
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
     * Retrieves a single script tag.
     *
     * This function performs a `GET` to the `/admin/api/unstable/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#show-unstable
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/script_tags/{}/json?{}",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Updates a script tag.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#update-unstable
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_script_tags_param_tag(
        &self,
        script_tag_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Deletes a script tag.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/script_tags/{script_tag_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/scripttag#destroy-unstable
     *
     * **Parameters:**
     *
     * * `script_tag_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_script_tags_param_tag(
        &self,
        script_tag_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/script_tags/{}/json",
                crate::progenitor_support::encode_path(script_tag_id),
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
     * Retrieves a list of themes.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/themes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#index-2020-01
     *
     * **Parameters:**
     *
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_theme(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-01/themes.json?{}", query_), None);
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
    * Creates a theme by providing the public URL of a ZIP file that contains the theme.
              A new theme is always unpublished by default. To publish a theme when you create it, include
              "role": "main" in the POST request. The theme will be published only after all
              of its files have been extracted and stored by Shopify, which might take a couple of minutes.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/themes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#create-2020-01
    */
    pub async fn deprecated_202001_create_themes(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/themes.json", None);
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
     * Retrieves a single theme.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#show-2020-01
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_themes_param_theme(
        &self,
        theme_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/themes/{}/json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
     * Updates an existing theme.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#update-2020-01
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_themes_param_theme(
        &self,
        theme_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Deletes a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_themes_param_theme(
        &self,
        theme_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Retrieves a list of themes.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/themes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#index-2020-04
     *
     * **Parameters:**
     *
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_theme(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-04/themes.json?{}", query_), None);
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
    * Creates a theme by providing the public URL of a ZIP file that contains the theme.
              A new theme is always unpublished by default. To publish a theme when you create it, include
              "role": "main" in the POST request. The theme will be published only after all
              of its files have been extracted and stored by Shopify, which might take a couple of minutes.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/themes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#create-2020-04
    */
    pub async fn deprecated_202004_create_themes(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/themes.json", None);
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
     * Retrieves a single theme.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#show-2020-04
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_themes_param_theme(
        &self,
        theme_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/themes/{}/json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
     * Updates an existing theme.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#update-2020-04
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_themes_param_theme(
        &self,
        theme_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Deletes a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_themes_param_theme(
        &self,
        theme_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Retrieves a list of themes.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/themes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#index-2020-07
     *
     * **Parameters:**
     *
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_theme(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-07/themes.json?{}", query_), None);
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
    * Creates a theme by providing the public URL of a ZIP file that contains the theme.
              A new theme is always unpublished by default. To publish a theme when you create it, include
              "role": "main" in the POST request. The theme will be published only after all
              of its files have been extracted and stored by Shopify, which might take a couple of minutes.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/themes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#create-2020-07
    */
    pub async fn deprecated_202007_create_themes(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/themes.json", None);
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
     * Retrieves a single theme.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#show-2020-07
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_themes_param_theme(
        &self,
        theme_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/themes/{}/json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
     * Updates an existing theme.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#update-2020-07
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_themes_param_theme(
        &self,
        theme_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Deletes a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_themes_param_theme(
        &self,
        theme_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Retrieves a list of themes.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/themes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#index-2020-10
     *
     * **Parameters:**
     *
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_theme(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-10/themes.json?{}", query_), None);
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
    * Creates a theme by providing the public URL of a ZIP file that contains the theme.
              A new theme is always unpublished by default. To publish a theme when you create it, include
              "role": "main" in the POST request. The theme will be published only after all
              of its files have been extracted and stored by Shopify, which might take a couple of minutes.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/themes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#create-2020-10
    */
    pub async fn create_themes(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/themes.json", None);
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
     * Retrieves a single theme.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#show-2020-10
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_themes_param_theme(&self, theme_id: &str, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/themes/{}/json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
     * Updates an existing theme.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#update-2020-10
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_themes_param_theme(
        &self,
        theme_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Deletes a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_themes_param_theme(&self, theme_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Retrieves a list of themes.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/themes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#index-2021-01
     *
     * **Parameters:**
     *
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_theme(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2021-01/themes.json?{}", query_), None);
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
    * Creates a theme by providing the public URL of a ZIP file that contains the theme.
              A new theme is always unpublished by default. To publish a theme when you create it, include
              "role": "main" in the POST request. The theme will be published only after all
              of its files have been extracted and stored by Shopify, which might take a couple of minutes.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/themes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#create-2021-01
    */
    pub async fn deprecated_202101_create_themes(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/themes.json", None);
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
     * Retrieves a single theme.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#show-2021-01
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_themes_param_theme(
        &self,
        theme_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/themes/{}/json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
     * Updates an existing theme.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#update-2021-01
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_themes_param_theme(
        &self,
        theme_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Deletes a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_themes_param_theme(
        &self,
        theme_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Retrieves a list of themes.
     *
     * This function performs a `GET` to the `/admin/api/unstable/themes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#index-unstable
     *
     * **Parameters:**
     *
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_theme(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/unstable/themes.json?{}", query_), None);
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
    * Creates a theme by providing the public URL of a ZIP file that contains the theme.
              A new theme is always unpublished by default. To publish a theme when you create it, include
              "role": "main" in the POST request. The theme will be published only after all
              of its files have been extracted and stored by Shopify, which might take a couple of minutes.
    *
    * This function performs a `POST` to the `/admin/api/unstable/themes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#create-unstable
    */
    pub async fn deprecated_unstable_create_themes(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/themes.json", None);
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
     * Retrieves a single theme.
     *
     * This function performs a `GET` to the `/admin/api/unstable/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#show-unstable
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_themes_param_theme(
        &self,
        theme_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/themes/{}/json?{}",
                crate::progenitor_support::encode_path(theme_id),
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
     * Updates an existing theme.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#update-unstable
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_themes_param_theme(
        &self,
        theme_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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
     * Deletes a theme.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/themes/{theme_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/online-store/theme#destroy-unstable
     *
     * **Parameters:**
     *
     * * `theme_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_themes_param_theme(
        &self,
        theme_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/themes/{}/json",
                crate::progenitor_support::encode_path(theme_id),
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

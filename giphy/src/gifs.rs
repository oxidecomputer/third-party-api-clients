use crate::Client;
use crate::ClientResult;

pub struct Gifs {
    pub client: Client,
}

impl Gifs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Gifs { client }
    }

    /**
     * Get GIFs by ID.
     *
     * This function performs a `GET` to the `/gifs` endpoint.
     *
     * A multiget version of the get GIF by ID endpoint.
     *
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Filters results by specified GIF IDs, separated by commas.
     */
    pub async fn get(&self, ids: &str) -> ClientResult<crate::types::GetGifsByResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gifs?{}", query_), None);
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
     * Random GIF.
     *
     * This function performs a `GET` to the `/gifs/random` endpoint.
     *
     * Returns a random GIF, limited by tag. Excluding the tag parameter will return a random GIF from the GIPHY catalog.
     *
     *
     * **Parameters:**
     *
     * * `tag: &str` -- The unique bit.ly URL for this GIF.
     * * `rating: &str` -- The unique bit.ly URL for this GIF.
     */
    pub async fn random(
        &self,
        tag: &str,
        rating: &str,
    ) -> ClientResult<crate::types::RandomGifResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !rating.is_empty() {
            query_args.push(("rating".to_string(), rating.to_string()));
        }
        if !tag.is_empty() {
            query_args.push(("tag".to_string(), tag.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gifs/random?{}", query_), None);
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
     * Search GIFs.
     *
     * This function performs a `GET` to the `/gifs/search` endpoint.
     *
     * Search all GIPHY GIFs for a word or phrase. Punctuation will be stripped and ignored.  Use a plus or url encode for phrases. Example paul+rudd, ryan+gosling or american+psycho.
     *
     *
     * **Parameters:**
     *
     * * `q: &str` -- The unique bit.ly URL for this GIF.
     * * `limit: i64` -- The maximum number of records to return.
     * * `offset: i64` -- An optional results offset.
     * * `rating: &str` -- The unique bit.ly URL for this GIF.
     * * `lang: &str` -- Specify default language for regional content; use a 2-letter ISO 639-1 language code.
     */
    pub async fn search(
        &self,
        q: &str,
        limit: i64,
        offset: i64,
        rating: &str,
        lang: &str,
    ) -> ClientResult<crate::types::GetGifsByResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !lang.is_empty() {
            query_args.push(("lang".to_string(), lang.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !rating.is_empty() {
            query_args.push(("rating".to_string(), rating.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gifs/search?{}", query_), None);
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
     * Translate phrase to GIF.
     *
     * This function performs a `GET` to the `/gifs/translate` endpoint.
     *
     * The translate API draws on search, but uses the GIPHY `special sauce` to handle translating from one vocabulary to another. In this case, words and phrases to GIF
     *
     *
     * **Parameters:**
     *
     * * `s: &str` -- The unique bit.ly URL for this GIF.
     */
    pub async fn translate(&self, s: &str) -> ClientResult<crate::types::RandomGifResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !s.is_empty() {
            query_args.push(("s".to_string(), s.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/gifs/translate?{}", query_), None);
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
     * Trending GIFs.
     *
     * This function performs a `GET` to the `/gifs/trending` endpoint.
     *
     * Fetch GIFs currently trending online. Hand curated by the GIPHY editorial team.  The data returned mirrors the GIFs showcased on the GIPHY homepage. Returns 25 results by default.
     *
     *
     * **Parameters:**
     *
     * * `limit: i64` -- The maximum number of records to return.
     * * `offset: i64` -- An optional results offset.
     * * `rating: &str` -- The unique bit.ly URL for this GIF.
     */
    pub async fn trending(
        &self,
        limit: i64,
        offset: i64,
        rating: &str,
    ) -> ClientResult<crate::types::GetGifsByResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !rating.is_empty() {
            query_args.push(("rating".to_string(), rating.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gifs/trending?{}", query_), None);
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
     * Get GIF by Id.
     *
     * This function performs a `GET` to the `/gifs/{gifId}` endpoint.
     *
     * Returns a GIF given that GIF's unique ID
     *
     *
     * **Parameters:**
     *
     * * `gif_id: i64` -- Filters results by specified GIF ID.
     */
    pub async fn get_gifs(&self, gif_id: i64) -> ClientResult<crate::types::RandomGifResponse> {
        let url = self.client.url(
            &format!(
                "/gifs/{}",
                crate::progenitor_support::encode_path(&gif_id.to_string()),
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
}

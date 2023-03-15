use crate::Client;
use crate::ClientResult;

pub struct Stickers {
    pub client: Client,
}

impl Stickers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Stickers { client }
    }

    /**
     * Random Sticker.
     *
     * This function performs a `GET` to the `/stickers/random` endpoint.
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
        let url = self
            .client
            .url(&format!("/stickers/random?{}", query_), None);
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
     * Search Stickers.
     *
     * This function performs a `GET` to the `/stickers/search` endpoint.
     *
     * Replicates the functionality and requirements of the classic GIPHY search, but returns animated stickers rather than GIFs.
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
        let url = self
            .client
            .url(&format!("/stickers/search?{}", query_), None);
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
     * Translate phrase to Sticker.
     *
     * This function performs a `GET` to the `/stickers/translate` endpoint.
     *
     * The translate API draws on search, but uses the GIPHY `special sauce` to handle translating from one vocabulary to another. In this case, words and phrases to GIFs.
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
            .url(&format!("/stickers/translate?{}", query_), None);
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
     * Trending Stickers.
     *
     * This function performs a `GET` to the `/stickers/trending` endpoint.
     *
     * Fetch Stickers currently trending online. Hand curated by the GIPHY editorial team. Returns 25 results by default.
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
        let url = self
            .client
            .url(&format!("/stickers/trending?{}", query_), None);
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

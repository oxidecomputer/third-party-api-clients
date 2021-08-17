use anyhow::Result;

use crate::Client;

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
    pub async fn random_sticker(
        &self,
        tag: &str,
        rating: &str,
    ) -> Result<crate::types::RandomGifResponse> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !rating.is_empty() {
            query_args.push(format!("rating={}", rating));
        }
        if !tag.is_empty() {
            query_args.push(format!("tag={}", tag));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/stickers/random?{}", query_);

        self.client.get(&url, None).await
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
    ) -> Result<crate::types::GetGifsByResponse> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !lang.is_empty() {
            query_args.push(format!("lang={}", lang));
        }
        if limit > 0 {
            query_args.push(format!("limit={}", limit));
        }
        if offset > 0 {
            query_args.push(format!("offset={}", offset));
        }
        if !q.is_empty() {
            query_args.push(format!("q={}", q));
        }
        if !rating.is_empty() {
            query_args.push(format!("rating={}", rating));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/stickers/search?{}", query_);

        self.client.get(&url, None).await
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
    pub async fn translate_sticker(&self, s: &str) -> Result<crate::types::RandomGifResponse> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !s.is_empty() {
            query_args.push(format!("s={}", s));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/stickers/translate?{}", query_);

        self.client.get(&url, None).await
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
    ) -> Result<crate::types::GetGifsByResponse> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if limit > 0 {
            query_args.push(format!("limit={}", limit));
        }
        if offset > 0 {
            query_args.push(format!("offset={}", offset));
        }
        if !rating.is_empty() {
            query_args.push(format!("rating={}", rating));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/stickers/trending?{}", query_);

        self.client.get(&url, None).await
    }
}

use anyhow::Result;

use crate::Client;

pub struct Gifs {
    client: Client,
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
    pub async fn get_by(&self, ids: &str) -> Result<crate::types::GetGifsByResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !ids.is_empty() {
            query_args.push(format!("ids={}", ids));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/gifs?{}", query);

        self.client.get(&url, None).await
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
    pub async fn random_gif(
        &self,
        tag: &str,
        rating: &str,
    ) -> Result<crate::types::RandomGifResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !rating.is_empty() {
            query_args.push(format!("rating={}", rating));
        }
        if !tag.is_empty() {
            query_args.push(format!("tag={}", tag));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/gifs/random?{}", query);

        self.client.get(&url, None).await
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
    ) -> Result<crate::types::GetGifsByResponse> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/gifs/search?{}", query);

        self.client.get(&url, None).await
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
    pub async fn translate_gif(&self, s: &str) -> Result<crate::types::RandomGifResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !s.is_empty() {
            query_args.push(format!("s={}", s));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/gifs/translate?{}", query);

        self.client.get(&url, None).await
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
    ) -> Result<crate::types::GetGifsByResponse> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/gifs/trending?{}", query);

        self.client.get(&url, None).await
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
    pub async fn get_gif_by(&self, gif_id: i64) -> Result<crate::types::RandomGifResponse> {
        let url = format!(
            "/gifs/{}",
            crate::progenitor_support::encode_path(&gif_id.to_string()),
        );

        self.client.get(&url, None).await
    }
}

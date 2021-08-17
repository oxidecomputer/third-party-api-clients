use anyhow::Result;

use crate::Client;

pub struct NotaryJournals {
    pub client: Client,
}

impl NotaryJournals {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        NotaryJournals { client }
    }

    /**
     * Gets notary jurisdictions for a user.
     *
     * This function performs a `GET` to the `/v2.1/current_user/notary/journals` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `search_text: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- The position within the total result set from which to start returning values. The value **thumbnail** may be used to return the page image.
     */
    pub async fn get(
        &self,
        count: &str,
        search_text: &str,
        start_position: &str,
    ) -> Result<crate::types::NotaryJournalList> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !count.is_empty() {
            query_args.push(format!("count={}", count));
        }
        if !search_text.is_empty() {
            query_args.push(format!("search_text={}", search_text));
        }
        if !start_position.is_empty() {
            query_args.push(format!("start_position={}", start_position));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/v2.1/current_user/notary/journals?{}", query_);

        self.client.get(&url, None).await
    }
}

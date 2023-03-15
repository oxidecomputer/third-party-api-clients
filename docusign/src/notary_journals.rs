use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<crate::types::NotaryJournalList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !search_text.is_empty() {
            query_args.push(("search_text".to_string(), search_text.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/v2.1/current_user/notary/journals?{}", query_),
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

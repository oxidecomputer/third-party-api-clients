use anyhow::Result;

use crate::Client;

pub struct CardProgram {
    client: Client,
}

impl CardProgram {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CardProgram { client }
    }

    /**
     * List card programs.
     *
     * This function performs a `GET` to the `/card-programs` endpoint.
     *
     * Retrieve all card programs.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_card_programs(
        &self,
        start: &str,
        page_size: f64,
    ) -> Result<crate::types::GetCardProgramsResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("page_size={}", page_size));
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/card-programs?{}", query);

        self.client.get(&url, None).await
    }

    /**
     * Create a card program.
     *
     * This function performs a `POST` to the `/card-programs` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn post_resources_card_program(
        &self,
        body: &crate::types::PostResourcesCardProgramRequest,
    ) -> Result<crate::types::CardPrograms> {
        let url = "/card-programs".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * GET a card program.
     *
     * This function performs a `GET` to the `/card-programs/<id>` endpoint.
     *
     * Retrieve a single card program.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_card_programs_card_program_id(&self) -> Result<crate::types::CardPrograms> {
        let url = "/card-programs/<id>".to_string();
        self.client.get(&url, None).await
    }
}

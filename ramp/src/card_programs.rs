use anyhow::Result;

use crate::Client;

pub struct CardPrograms {
    pub client: Client,
}

impl CardPrograms {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CardPrograms { client }
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
    pub async fn get_page(
        &self,
        start: &str,
        page_size: f64,
    ) -> Result<Vec<crate::types::CardProgram>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !page_size.to_string().is_empty() {
            query_args.push(format!("page_size={}", page_size.to_string()));
        }
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/card-programs?{}", query_);

        let resp: crate::types::GetCardProgramsResponse =
            self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.card_programs)
    }

    /**
     * List card programs.
     *
     * This function performs a `GET` to the `/card-programs` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Retrieve all card programs.
     */
    pub async fn get_all(&self) -> Result<Vec<crate::types::CardProgram>> {
        let url = "/card-programs".to_string();
        let mut resp: crate::types::GetCardProgramsResponse =
            self.client.get(&url, None).await.unwrap();

        let mut card_programs = resp.card_programs;
        let mut page = resp.page.next;

        // Paginate if we should.
        while !page.is_empty() {
            resp = self
                .client
                .get(page.trim_start_matches(crate::DEFAULT_HOST), None)
                .await
                .unwrap();

            card_programs.append(&mut resp.card_programs);

            if !resp.page.next.is_empty() && resp.page.next != page {
                page = resp.page.next.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(card_programs)
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
    ) -> Result<crate::types::CardProgram> {
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
     * This function performs a `GET` to the `/card-programs/{id}` endpoint.
     *
     * Retrieve a single card program.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_program(&self, id: &str) -> Result<crate::types::CardProgram> {
        let url = format!(
            "/card-programs/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }
}

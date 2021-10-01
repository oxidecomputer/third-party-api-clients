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
     * * `start: Option<uuid::Uuid>` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_page(
        &self,
        start: Option<uuid::Uuid>,
        page_size: f64,
    ) -> Result<Vec<crate::types::CardProgram>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if let Some(u) = start {
            query_args.push(("start".to_string(), u.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/card-programs?{}", query_);

        let resp: crate::types::GetCardProgramsResponse = self.client.get(&url, None).await?;

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
        let resp: crate::types::GetCardProgramsResponse = self.client.get(&url, None).await?;

        let mut card_programs = resp.card_programs;
        let mut page = resp.page.next.to_string();

        // Paginate if we should.
        while !page.is_empty() {
            match self
                .client
                .get::<crate::types::GetCardProgramsResponse>(
                    page.trim_start_matches(crate::DEFAULT_HOST),
                    None,
                )
                .await
            {
                Ok(mut resp) => {
                    card_programs.append(&mut resp.card_programs);

                    page = if resp.page.next != page {
                        resp.page.next.to_string()
                    } else {
                        "".to_string()
                    };
                }
                Err(e) => {
                    if e.to_string().contains("404 Not Found") {
                        page = "".to_string();
                    } else {
                        anyhow::bail!(e);
                    }
                }
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
    pub async fn post_resources(
        &self,
        body: &crate::types::PostResourcesCardProgramRequest,
    ) -> Result<crate::types::CardProgram> {
        let url = "/card-programs".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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

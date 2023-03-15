use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<Vec<crate::types::CardProgram>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !start.is_empty() {
            query_args.push(("start".to_string(), start.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/card-programs?{}", query_), None);
        let resp: crate::types::GetCardProgramsResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.card_programs.to_vec())
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
    pub async fn get_all(&self) -> ClientResult<Vec<crate::types::CardProgram>> {
        let url = self.client.url("/card-programs", None);
        let resp: crate::types::GetCardProgramsResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut card_programs = resp.card_programs;
        let mut page = resp.page.next.to_string();

        // Paginate if we should.
        while !page.is_empty() {
            match self
                .client
                .get::<crate::types::GetCardProgramsResponse>(
                    page.trim_start_matches(&self.client.host),
                    crate::Message {
                        body: None,
                        content_type: None,
                    },
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
                        return Err(e);
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
    ) -> ClientResult<crate::types::CardProgram> {
        let url = self.client.url("/card-programs", None);
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
    pub async fn get_program(&self, id: &str) -> ClientResult<crate::types::CardProgram> {
        let url = self.client.url(
            &format!(
                "/card-programs/{}",
                crate::progenitor_support::encode_path(id),
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

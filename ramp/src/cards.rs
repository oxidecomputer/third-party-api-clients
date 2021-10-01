use anyhow::Result;

use crate::Client;

pub struct Cards {
    pub client: Client,
}

impl Cards {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Cards { client }
    }

    /**
     * List cards.
     *
     * This function performs a `GET` to the `/cards` endpoint.
     *
     * Retrieve all cards.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     * * `start: uuid::Uuid` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     * * `user_id: uuid::Uuid` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `card_program_id: uuid::Uuid` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     */
    pub async fn get_page(
        &self,
        start: uuid::Uuid,
        page_size: f64,
        user_id: uuid::Uuid,
        card_program_id: uuid::Uuid,
    ) -> Result<Vec<crate::types::Card>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if card_program_id.to_string() != uuid::Uuid::nil().to_string() {
            query_args.push(("card_program_id".to_string(), card_program_id.to_string()));
        }
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if start.to_string() != uuid::Uuid::nil().to_string() {
            query_args.push(("start".to_string(), start.to_string()));
        }
        if user_id.to_string() != uuid::Uuid::nil().to_string() {
            query_args.push(("user_id".to_string(), user_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/cards?{}", query_);

        let resp: crate::types::GetCardsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.cards)
    }

    /**
     * List cards.
     *
     * This function performs a `GET` to the `/cards` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Retrieve all cards.
     */
    pub async fn get_all(
        &self,
        user_id: uuid::Uuid,
        card_program_id: uuid::Uuid,
    ) -> Result<Vec<crate::types::Card>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if card_program_id.to_string() != uuid::Uuid::nil().to_string() {
            query_args.push(("card_program_id".to_string(), card_program_id.to_string()));
        }
        if user_id.to_string() != uuid::Uuid::nil().to_string() {
            query_args.push(("user_id".to_string(), user_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/cards?{}", query_);

        let resp: crate::types::GetCardsResponse = self.client.get(&url, None).await?;

        let mut cards = resp.cards;
        let mut page = resp.page.next.to_string();

        // Paginate if we should.
        while !page.is_empty() {
            match self
                .client
                .get::<crate::types::GetCardsResponse>(
                    page.trim_start_matches(crate::DEFAULT_HOST),
                    None,
                )
                .await
            {
                Ok(mut resp) => {
                    cards.append(&mut resp.cards);

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
        Ok(cards)
    }

    /**
     * GET a card.
     *
     * This function performs a `GET` to the `/cards/{id}` endpoint.
     *
     * Retrieve a single card.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get(&self, id: &str) -> Result<crate::types::Card> {
        let url = format!(
            "/cards/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update card.
     *
     * This function performs a `PATCH` to the `/cards/{id}` endpoint.
     *
     * Update card details
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn patch_resources(
        &self,
        id: &str,
        body: &crate::types::PatchResourcesCardsCardRequest,
    ) -> Result<()> {
        let url = format!(
            "/cards/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Create a physical card.
     *
     * This function performs a `POST` to the `/cards/deferred/physical` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn post_resources_physical(
        &self,
        body: &crate::types::PostResourcesCardPhysicalRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = "/cards/deferred/physical".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Create a virtual card.
     *
     * This function performs a `POST` to the `/cards/deferred/virtual` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn post_resources_virtual(
        &self,
        body: &crate::types::PostResourcesCardVirtualRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = "/cards/deferred/virtual".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a card.
     *
     * This function performs a `POST` to the `/cards/{id}/deferred/termination` endpoint.
     *
     * Terminates a card permanently.
     */
    pub async fn post_resources_termination(
        &self,
        id: &str,
        body: &crate::types::PostResourcesCardsCardSuspensionRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = format!(
            "/cards/{}/deferred/termination",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Suspend a card.
     *
     * This function performs a `POST` to the `/cards/{id}/deferred/suspension` endpoint.
     *
     * Suspends a card so that it is locked from use. The suspension is revertable.
     */
    pub async fn post_resources_suspension(
        &self,
        id: &str,
        body: &crate::types::PostResourcesCardsCardSuspensionRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = format!(
            "/cards/{}/deferred/suspension",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Removes a card's suspension.
     *
     * This function performs a `POST` to the `/cards/{id}/deferred/unsuspension` endpoint.
     *
     * Removes a card's suspension so that it may be used again.
     */
    pub async fn post_resources_unsuspension(
        &self,
        id: &str,
        body: &crate::types::PostResourcesCardsCardSuspensionRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = format!(
            "/cards/{}/deferred/unsuspension",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get status of a deferred card task.
     *
     * This function performs a `GET` to the `/cards/deferred/status/{id}` endpoint.
     *
     * Gets status of a deferred task for cards
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_resources_deferred(
        &self,
        id: &str,
    ) -> Result<crate::types::GetResourcesCardsDeferredResponse> {
        let url = format!(
            "/cards/deferred/status/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }
}

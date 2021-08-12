use anyhow::Result;

use crate::Client;

pub struct Cards {
    client: Client,
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
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     * * `user_id: &str`
     * * `card_program_id: &str`
     */
    pub async fn get_page(
        &self,
        start: &str,
        page_size: f64,
        user_id: &str,
        card_program_id: &str,
    ) -> Result<Vec<crate::types::Card>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !card_program_id.is_empty() {
            query_args.push(format!("card_program_id={}", card_program_id));
        }
        query_args.push(format!("page_size={}", page_size));
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        if !user_id.is_empty() {
            query_args.push(format!("user_id={}", user_id));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/cards?{}", query);

        let resp: crate::types::GetCardsResponse = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.data)
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
        user_id: &str,
        card_program_id: &str,
    ) -> Result<Vec<crate::types::Card>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !card_program_id.is_empty() {
            query_args.push(format!("card_program_id={}", card_program_id));
        }
        if !user_id.is_empty() {
            query_args.push(format!("user_id={}", user_id));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/cards?{}", query);

        let mut resp: crate::types::GetCardsResponse = self.client.get(&url, None).await.unwrap();

        let mut cards = resp.cards;
        let mut page = resp.page.next;

        // Paginate if we should.
        while !page.is_empty() {
            resp = self
                .client
                .get(page.trim_start_matches(crate::DEFAULT_HOST), None)
                .await
                .unwrap();

            cards.append(&mut resp.cards);

            if !resp.page.next.is_empty() && resp.page.next != page {
                page = resp.page.next.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(data)
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
    pub async fn get_card(&self, id: &str) -> Result<crate::types::Card> {
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
    pub async fn patch_resources_card(
        &self,
        id: &str,
        body: &crate::types::PatchResourcesCardsCardRequest,
    ) -> Result<()> {
        let url = format!(
            "/cards/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
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
    pub async fn post_resources_card_physical(
        &self,
        body: &crate::types::PostResourcesCardPhysicalRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = "/cards/deferred/physical".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
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
    pub async fn post_resources_card_virtual(
        &self,
        body: &crate::types::PostResourcesCardVirtualRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = "/cards/deferred/virtual".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Delete a card.
     *
     * This function performs a `POST` to the `/cards/{id}/deferred/termination` endpoint.
     *
     * Terminates a card permanently.
     */
    pub async fn post_resources_card_termination(
        &self,
        id: &str,
        body: &crate::types::PostResourcesCardsCardSuspensionRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = format!(
            "/cards/{}/deferred/termination",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Suspend a card.
     *
     * This function performs a `POST` to the `/cards/{id}/deferred/suspension` endpoint.
     *
     * Suspends a card so that it is locked from use. The suspension is revertable.
     */
    pub async fn post_resources_card_suspension(
        &self,
        id: &str,
        body: &crate::types::PostResourcesCardsCardSuspensionRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = format!(
            "/cards/{}/deferred/suspension",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Removes a card's suspension.
     *
     * This function performs a `POST` to the `/cards/{id}/deferred/unsuspension` endpoint.
     *
     * Removes a card's suspension so that it may be used again.
     */
    pub async fn post_resources_card_unsuspension(
        &self,
        id: &str,
        body: &crate::types::PostResourcesCardsCardSuspensionRequest,
    ) -> Result<crate::types::TaskResponse> {
        let url = format!(
            "/cards/{}/deferred/unsuspension",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
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

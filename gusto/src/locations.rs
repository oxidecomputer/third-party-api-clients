use crate::Client;
use crate::ClientResult;

pub struct Locations {
    pub client: Client,
}

impl Locations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Locations { client }
    }

    /**
     * Get company locations.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/locations` endpoint.
     *
     * Company locations represent all addresses associated with a company. These can be filing addesses, mailing addresses, and/or work locations; one address may serve multiple, or all, purposes.
     *
     * Since all company locations are subsets of locations, retrieving or updating an individual record should be done via the locations endpoints.
     */
    pub async fn get_company(
        &self,
        company_id_or_uuid: &str,
    ) -> ClientResult<Vec<crate::types::Location>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/locations",
                crate::progenitor_support::encode_path(company_id_or_uuid),
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
    /**
     * Get company locations.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/locations` endpoint.
     *
     * As opposed to `get_company`, this function returns all the pages of the request at once.
     *
     * Company locations represent all addresses associated with a company. These can be filing addesses, mailing addresses, and/or work locations; one address may serve multiple, or all, purposes.
     *
     * Since all company locations are subsets of locations, retrieving or updating an individual record should be done via the locations endpoints.
     */
    pub async fn get_all_company(
        &self,
        company_id_or_uuid: &str,
    ) -> ClientResult<Vec<crate::types::Location>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/locations",
                crate::progenitor_support::encode_path(company_id_or_uuid),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create a company location.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id_or_uuid}/locations` endpoint.
     *
     * Company locations represent all addresses associated with a company. These can be filing addesses, mailing addresses, and/or work locations; one address may serve multiple, or all, purposes.
     *
     * Since all company locations are subsets of locations, retrieving or updating an individual record should be done via the locations endpoints.
     */
    pub async fn post_company(
        &self,
        company_id_or_uuid: &str,
        body: &crate::types::PostCompanyLocationsRequest,
    ) -> ClientResult<crate::types::Location> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/locations",
                crate::progenitor_support::encode_path(company_id_or_uuid),
            ),
            None,
        );
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
     * Get a location.
     *
     * This function performs a `GET` to the `/v1/locations/{location_id}` endpoint.
     *
     * Get a location.
     */
    pub async fn get(&self, location_id: &str) -> ClientResult<crate::types::Location> {
        let url = self.client.url(
            &format!(
                "/v1/locations/{}",
                crate::progenitor_support::encode_path(location_id),
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
    /**
     * Update a location.
     *
     * This function performs a `PUT` to the `/v1/locations/{location_id}` endpoint.
     *
     * Update a location.
     */
    pub async fn put(
        &self,
        location_id: &str,
        body: &crate::types::PutLocationRequest,
    ) -> ClientResult<crate::types::Location> {
        let url = self.client.url(
            &format!(
                "/v1/locations/{}",
                crate::progenitor_support::encode_path(location_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}

use anyhow::Result;

use crate::Client;

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
    ) -> Result<Vec<crate::types::Location>> {
        let url = format!(
            "/v1/companies/{}/locations",
            crate::progenitor_support::encode_path(company_id_or_uuid),
        );

        self.client.get(&url, None).await
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
    ) -> Result<Vec<crate::types::Location>> {
        let url = format!(
            "/v1/companies/{}/locations",
            crate::progenitor_support::encode_path(company_id_or_uuid),
        );

        self.client.get_all_pages(&url, None).await
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
    ) -> Result<crate::types::Location> {
        let url = format!(
            "/v1/companies/{}/locations",
            crate::progenitor_support::encode_path(company_id_or_uuid),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get a location.
    *
    * This function performs a `GET` to the `/v1/locations/{location_id}` endpoint.
    *
    * Get a location.
    */
    pub async fn get(&self, location_id: &str) -> Result<crate::types::Location> {
        let url = format!(
            "/v1/locations/{}",
            crate::progenitor_support::encode_path(location_id),
        );

        self.client.get(&url, None).await
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
    ) -> Result<crate::types::Location> {
        let url = format!(
            "/v1/locations/{}",
            crate::progenitor_support::encode_path(location_id),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

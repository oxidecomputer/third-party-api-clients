use anyhow::Result;

use crate::Client;

pub struct Roles {
    pub client: Client,
}

impl Roles {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Roles { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/roles` endpoint.
     *
     * Retrieves a paginated list of all the roles in a domain.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `max_results: i64` -- Maximum number of results to return.
     * * `page_token: &str` -- Token to specify the next page in the list.
     */
    pub async fn list(
        &self,
        customer: &str,
        max_results: i64,
        page_token: &str,
    ) -> Result<Vec<crate::types::Role>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/admin/directory/v1/customer/{}/roles?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        let resp: crate::types::Roles = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/roles` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Retrieves a paginated list of all the roles in a domain.
     */
    pub async fn list_all(&self, customer: &str) -> Result<Vec<crate::types::Role>> {
        let url = format!(
            "/admin/directory/v1/customer/{}/roles",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        let mut resp: crate::types::Roles = self.client.get(&url, None).await.unwrap();

        let mut items = resp.items;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?pageToken={}", url, page), None)
                    .await
                    .unwrap();
            } else {
                resp = self
                    .client
                    .get(&format!("{}&pageToken={}", url, page), None)
                    .await
                    .unwrap();
            }

            items.append(&mut resp.items);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(items)
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/roles` endpoint.
     *
     * Creates a role.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn insert(
        &self,
        customer: &str,
        body: &crate::types::Role,
    ) -> Result<crate::types::Role> {
        let url = format!(
            "/admin/directory/v1/customer/{}/roles",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/roles/{roleId}` endpoint.
     *
     * Retrieves a role.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `role_id: &str` -- Immutable ID of the role.
     */
    pub async fn get(&self, customer: &str, role_id: &str) -> Result<crate::types::Role> {
        let url = format!(
            "/admin/directory/v1/customer/{}/roles/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&role_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/admin/directory/v1/customer/{customer}/roles/{roleId}` endpoint.
     *
     * Updates a role.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `role_id: &str` -- Immutable ID of the role.
     */
    pub async fn update(
        &self,
        customer: &str,
        role_id: &str,
        body: &crate::types::Role,
    ) -> Result<crate::types::Role> {
        let url = format!(
            "/admin/directory/v1/customer/{}/roles/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&role_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/roles/{roleId}` endpoint.
     *
     * Deletes a role.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `role_id: &str` -- Immutable ID of the role.
     */
    pub async fn delete(&self, customer: &str, role_id: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/customer/{}/roles/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&role_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/admin/directory/v1/customer/{customer}/roles/{roleId}` endpoint.
     *
     * Patches a role.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `role_id: &str` -- Immutable ID of the role.
     */
    pub async fn patch(
        &self,
        customer: &str,
        role_id: &str,
        body: &crate::types::Role,
    ) -> Result<crate::types::Role> {
        let url = format!(
            "/admin/directory/v1/customer/{}/roles/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&role_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}

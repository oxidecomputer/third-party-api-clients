use anyhow::Result;

use crate::Client;

pub struct Privileges {
    pub client: Client,
}

impl Privileges {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Privileges { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/roles/ALL/privileges` endpoint.
     *
     * Retrieves a paginated list of all privileges for a customer.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn list(&self, customer: &str) -> Result<crate::types::Privileges> {
        let url = format!(
            "/admin/directory/v1/customer/{}/roles/ALL/privileges",
            crate::progenitor_support::encode_path(customer),
        );

        self.client.get(&url, None).await
    }
}

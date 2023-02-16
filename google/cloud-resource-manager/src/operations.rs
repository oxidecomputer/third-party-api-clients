use anyhow::Result;

use crate::Client;

pub struct Operations {
    pub client: Client,
}

impl Operations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Operations { client }
    }

    /**
     * This function performs a `GET` to the `/v1/{name}` endpoint.
     *
     * Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
     *
     * **Parameters:**
     *
     * * `name: &str` -- Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
     */
    pub async fn get(&self, name: &str) -> Result<crate::types::Operation> {
        let url = format!("/v1/{}", crate::progenitor_support::encode_path(name),);
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
}

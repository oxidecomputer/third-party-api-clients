use crate::Client;
use crate::ClientResult;

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
    pub async fn get(&self, name: &str) -> ClientResult<crate::types::Operation> {
        let url = self.client.url(
            &format!("/v1/{}", crate::progenitor_support::encode_path(name),),
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

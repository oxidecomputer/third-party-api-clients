use crate::Client;
use crate::ClientResult;

pub struct Services {
    pub client: Client,
}

impl Services {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Services { client }
    }

    /**
     * Retrieves the available REST API versions.
     *
     * This function performs a `GET` to the `/service_information` endpoint.
     *
     * Retrieves the available REST API versions.
     *
     * DocuSign Production system: https://www.docusign.net/restapi/service_information
     * DocuSign Demo system: https://demo.docusign.net/restapi/service_information
     *
     * You do not need an integrator key to view the REST API versions and resources.
     */
    pub async fn information_get(&self) -> ClientResult<crate::types::ServiceInformation> {
        let url = self.client.url("/service_information", None);
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

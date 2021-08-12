use anyhow::Result;

use crate::Client;

pub struct Resources {
    client: Client,
}

impl Resources {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Resources { client }
    }

    /**
     * Lists resources for REST version specified.
     *
     * This function performs a `GET` to the `/v2.1` endpoint.
     *
     * Retrieves the base resources available for the DocuSign REST APIs.
     *
     * You do not need an integrator key to view the REST API versions and resources.
     *
     * Example: https://demo.docusign.net/restapi/v2 lists all of the base resources available in version 2 of the REST API on the DocuSign Demo system.
     *
     * To view descriptions and samples of the service operations for all versions, remove the version number and add /help to the URL.
     *
     * Example: https://demo.docusign.net/restapi/help lists the REST API operations on the DocuSign Demo system with XML and JSON request and response samples.
     */
    pub async fn service_information_get_resource(
        &self,
    ) -> Result<crate::types::ResourceInformation> {
        let url = "/v2.1".to_string();
        self.client.get(&url, None).await
    }
}

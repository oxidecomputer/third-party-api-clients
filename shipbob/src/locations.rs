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
     * Get locations.
     *
     * This function performs a `GET` to the `/location` endpoint.
     *
     * **Parameters:**
     *
     * * `include_inactive: bool` -- True if the inventory item is marked as a digital item.
     * * `receiving_enabled: bool` -- True if the inventory item is marked as a digital item.
     * * `access_granted: bool` -- True if the inventory item is marked as a digital item.
     */
    pub async fn get_page(
        &self,
        include_inactive: bool,
        receiving_enabled: bool,
        access_granted: bool,
    ) -> ClientResult<Vec<crate::types::IntegrationsLocationInternalAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if access_granted {
            query_args.push(("AccessGranted".to_string(), access_granted.to_string()));
        }
        if include_inactive {
            query_args.push(("IncludeInactive".to_string(), include_inactive.to_string()));
        }
        if receiving_enabled {
            query_args.push((
                "ReceivingEnabled".to_string(),
                receiving_enabled.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/location?{}", query_), None);
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
     * Get locations.
     *
     * This function performs a `GET` to the `/location` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        include_inactive: bool,
        receiving_enabled: bool,
        access_granted: bool,
    ) -> ClientResult<Vec<crate::types::IntegrationsLocationInternalAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if access_granted {
            query_args.push(("AccessGranted".to_string(), access_granted.to_string()));
        }
        if include_inactive {
            query_args.push(("IncludeInactive".to_string(), include_inactive.to_string()));
        }
        if receiving_enabled {
            query_args.push((
                "ReceivingEnabled".to_string(),
                receiving_enabled.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/location?{}", query_), None);
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
}

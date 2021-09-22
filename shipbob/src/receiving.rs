use anyhow::Result;

use crate::Client;

pub struct Receiving {
    pub client: Client,
}

impl Receiving {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Receiving { client }
    }

    /**
     * Get Fulfillment Centers.
     *
     * This function performs a `GET` to the `/fulfillmentCenter` endpoint.
     */
    pub async fn get_fulfillment_center(
        &self,
    ) -> Result<Vec<crate::types::ShipbobReceivingPublicApiModelsFulfillmentCenterViewModel>> {
        let url = "/fulfillmentCenter".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Get Fulfillment Centers.
     *
     * This function performs a `GET` to the `/fulfillmentCenter` endpoint.
     *
     * As opposed to `get_fulfillment_center`, this function returns all the pages of the request at once.
     */
    pub async fn get_all_fulfillment_center(
        &self,
    ) -> Result<Vec<crate::types::ShipbobReceivingPublicApiModelsFulfillmentCenterViewModel>> {
        let url = "/fulfillmentCenter".to_string();
        self.client.get_all_pages(&url, None).await
    }

    /**
     * Get Warehouse Receiving Order.
     *
     * This function performs a `GET` to the `/receiving/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `id: i64` -- Unique id of the channel.
     */
    pub async fn get(
        &self,
        id: i64,
    ) -> Result<crate::types::ShipbobReceivingPublicApiModelsOrderViewModel> {
        let url = format!(
            "/receiving/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get Warehouse Receiving Order Box Labels.
     *
     * This function performs a `GET` to the `/receiving/{id}/labels` endpoint.
     *
     * **Parameters:**
     *
     * * `id: i64` -- Unique id of the channel.
     */
    pub async fn get_label(&self, id: i64) -> Result<bytes::Bytes> {
        let url = format!(
            "/receiving/{}/labels",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Create Warehouse Receiving Order.
     *
     * This function performs a `POST` to the `/receiving` endpoint.
     */
    pub async fn post<T: Into<reqwest::Body>>(
        &self,
        body: crate::types::ShipbobReceivingPublicApiModelsCreateOrderModel,
    ) -> Result<crate::types::ShipbobReceivingPublicApiModelsOrderViewModel> {
        let url = "/receiving".to_string();
        self.client.post(&url, Some(body.into())).await
    }

    /**
     * Cancel Warehouse Receiving Order.
     *
     * This function performs a `POST` to the `/receiving/{id}/cancel` endpoint.
     *
     * **Parameters:**
     *
     * * `id: i64` -- Id of the receiving order to cancel.
     */
    pub async fn post_cancel(&self, id: i64) -> Result<()> {
        let url = format!(
            "/receiving/{}/cancel",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }
}

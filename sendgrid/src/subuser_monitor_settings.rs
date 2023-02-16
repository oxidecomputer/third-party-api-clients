use anyhow::Result;

use crate::Client;

pub struct SubuserMonitorSettings {
    pub client: Client,
}

impl SubuserMonitorSettings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SubuserMonitorSettings { client }
    }

    /**
     * Retrieve monitor settings for a subuser.
     *
     * This function performs a `GET` to the `/subusers/{subuser_name}/monitor` endpoint.
     */
    pub async fn get_subusers_subuser_name_monitor(
        &self,
        subuser_name: &str,
    ) -> Result<crate::types::Monitor> {
        let url = format!(
            "/subusers/{}/monitor",
            crate::progenitor_support::encode_path(subuser_name),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
    /**
     * Update Monitor Settings for a subuser.
     *
     * This function performs a `PUT` to the `/subusers/{subuser_name}/monitor` endpoint.
     */
    pub async fn put_subusers_subuser_name_monitor(
        &self,
        subuser_name: &str,
        body: &crate::types::Monitor,
    ) -> Result<crate::types::Monitor> {
        let url = format!(
            "/subusers/{}/monitor",
            crate::progenitor_support::encode_path(subuser_name),
        );
        let url = self.client.url(&url, None);
        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                None,
            )
            .await
    }
    /**
     * Create monitor settings.
     *
     * This function performs a `POST` to the `/subusers/{subuser_name}/monitor` endpoint.
     */
    pub async fn post_subusers_subuser_name_monitor(
        &self,
        subuser_name: &str,
        body: &crate::types::Monitor,
    ) -> Result<crate::types::Monitor> {
        let url = format!(
            "/subusers/{}/monitor",
            crate::progenitor_support::encode_path(subuser_name),
        );
        let url = self.client.url(&url, None);
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                None,
            )
            .await
    }
    /**
     * Delete monitor settings.
     *
     * This function performs a `DELETE` to the `/subusers/{subuser_name}/monitor` endpoint.
     */
    pub async fn delete_subusers_subuser_name_monitor(
        &self,
        subuser_name: &str,
    ) -> Result<crate::types::Help> {
        let url = format!(
            "/subusers/{}/monitor",
            crate::progenitor_support::encode_path(subuser_name),
        );
        let url = self.client.url(&url, None);
        self.client.delete(&url, None, None).await
    }
}

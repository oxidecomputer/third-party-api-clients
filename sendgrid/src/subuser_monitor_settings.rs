use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<crate::types::Monitor> {
        let url = self.client.url(
            &format!(
                "/subusers/{}/monitor",
                crate::progenitor_support::encode_path(subuser_name),
            ),
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
    /**
     * Update Monitor Settings for a subuser.
     *
     * This function performs a `PUT` to the `/subusers/{subuser_name}/monitor` endpoint.
     */
    pub async fn put_subusers_subuser_name_monitor(
        &self,
        subuser_name: &str,
        body: &crate::types::Monitor,
    ) -> ClientResult<crate::types::Monitor> {
        let url = self.client.url(
            &format!(
                "/subusers/{}/monitor",
                crate::progenitor_support::encode_path(subuser_name),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
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
    ) -> ClientResult<crate::types::Monitor> {
        let url = self.client.url(
            &format!(
                "/subusers/{}/monitor",
                crate::progenitor_support::encode_path(subuser_name),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
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
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/subusers/{}/monitor",
                crate::progenitor_support::encode_path(subuser_name),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}

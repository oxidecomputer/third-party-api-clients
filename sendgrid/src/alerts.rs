use crate::Client;
use crate::ClientResult;

pub struct Alerts {
    pub client: Client,
}

impl Alerts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Alerts { client }
    }

    /**
     * Retrieve all alerts.
     *
     * This function performs a `GET` to the `/alerts` endpoint.
     *
     * **This endpoint allows you to retrieve all of your alerts.**
     *
     * Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.
     * * Usage alerts allow you to set the threshold at which an alert will be sent.
     * * Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".
     *
     * For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The license key provided with your New Relic account.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_page(&self) -> ClientResult<Vec<crate::types::GetAlertsResponse>> {
        let url = self.client.url("/alerts", None);
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
     * Retrieve all alerts.
     *
     * This function performs a `GET` to the `/alerts` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all of your alerts.**
     *
     * Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.
     * * Usage alerts allow you to set the threshold at which an alert will be sent.
     * * Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".
     *
     * For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).
     */
    pub async fn get_all(&self) -> ClientResult<Vec<crate::types::GetAlertsResponse>> {
        let url = self.client.url("/alerts", None);
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
    /**
     * Create a new Alert.
     *
     * This function performs a `POST` to the `/alerts` endpoint.
     *
     * **This endpoint allows you to create a new alert.**
     *
     * Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics. There are two types of alerts that can be created with this endpoint:
     *
     * * `usage_limit` allows you to set the threshold at which an alert will be sent.
     * * `stats_notification` allows you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".
     *
     * For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The license key provided with your New Relic account.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post(
        &self,
        body: &crate::types::PostAlertsRequest,
    ) -> ClientResult<crate::types::PostAlertsResponse> {
        let url = self.client.url("/alerts", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieve a specific alert.
     *
     * This function performs a `GET` to the `/alerts/{alert_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific alert.**
     *
     * Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.
     * * Usage alerts allow you to set the threshold at which an alert will be sent.
     * * Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".
     *
     * For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The license key provided with your New Relic account.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get(&self, alert_id: i64) -> ClientResult<crate::types::GetAlertsAlertResponse> {
        let url = self.client.url(
            &format!(
                "/alerts/{}",
                crate::progenitor_support::encode_path(&alert_id.to_string()),
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
     * Delete an alert.
     *
     * This function performs a `DELETE` to the `/alerts/{alert_id}` endpoint.
     *
     * **This endpoint allows you to delete an alert.**
     *
     * Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.
     * * Usage alerts allow you to set the threshold at which an alert will be sent.
     * * Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".
     *
     * For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete(&self, alert_id: i64) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/alerts/{}",
                crate::progenitor_support::encode_path(&alert_id.to_string()),
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
    /**
     * Update an alert.
     *
     * This function performs a `PATCH` to the `/alerts/{alert_id}` endpoint.
     *
     * **This endpoint allows you to update an alert.**
     *
     * Alerts allow you to specify an email address to receive notifications regarding your email usage or statistics.
     * * Usage alerts allow you to set the threshold at which an alert will be sent.
     * * Stats notifications allow you to set how frequently you would like to receive email statistics reports. For example, "daily", "weekly", or "monthly".
     *
     * For more information about alerts, please see our [Alerts documentation](https://sendgrid.com/docs/ui/account-and-settings/alerts/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch(
        &self,
        alert_id: i64,
        body: &crate::types::PatchAlertsAlertRequest,
    ) -> ClientResult<crate::types::GetAlertsAlertResponse> {
        let url = self.client.url(
            &format!(
                "/alerts/{}",
                crate::progenitor_support::encode_path(&alert_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}

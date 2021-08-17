use anyhow::Result;

use crate::Client;

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
    pub async fn get_page(
        &self,
        on_behalf_of: &str,
    ) -> Result<Vec<crate::types::GetAlertsResponse>> {
        let url = "/alerts".to_string();
        self.client.get(&url, None).await
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
    pub async fn get_all(
        &self,
        on_behalf_of: &str,
    ) -> Result<Vec<crate::types::GetAlertsResponse>> {
        let url = "/alerts".to_string();
        self.client.get_all_pages(&url, None).await
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
    pub async fn post_alert(
        &self,
        on_behalf_of: &str,
        body: &crate::types::PostAlertsRequest,
    ) -> Result<crate::types::PostAlertsResponse> {
        let url = "/alerts".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn get_alert(
        &self,
        alert_id: i64,
        on_behalf_of: &str,
    ) -> Result<crate::types::GetAlertsAlertResponse> {
        let url = format!(
            "/alerts/{}",
            crate::progenitor_support::encode_path(&alert_id.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn delete_alert(
        &self,
        alert_id: i64,
        on_behalf_of: &str,
    ) -> Result<crate::types::Help> {
        let url = format!(
            "/alerts/{}",
            crate::progenitor_support::encode_path(&alert_id.to_string()),
        );

        self.client.delete(&url, None).await
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
    pub async fn patch_alert(
        &self,
        alert_id: i64,
        on_behalf_of: &str,
        body: &crate::types::PatchAlertsAlertRequest,
    ) -> Result<crate::types::GetAlertsAlertResponse> {
        let url = format!(
            "/alerts/{}",
            crate::progenitor_support::encode_path(&alert_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}

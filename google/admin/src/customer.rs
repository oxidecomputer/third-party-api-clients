use crate::Client;
use crate::ClientResult;

pub struct Customer {
    pub client: Client,
}

impl Customer {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Customer { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/devices/chromeos/{deviceId}/commands/{commandId}` endpoint.
     *
     * Gets command data a specific command issued to the device.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable. Immutable ID of the Google Workspace account.
     * * `device_id: &str` -- Immutable. Immutable ID of Chrome OS Device.
     * * `command_id: &str` -- Immutable. Immutable ID of Chrome OS Device Command.
     */
    pub async fn admin_devices_chromeos_commands_get(
        &self,
        customer_id: &str,
        device_id: &str,
        command_id: &str,
    ) -> ClientResult<crate::types::DirectoryChromeosdevicesCommand> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/chromeos/{}/commands/{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(device_id),
                crate::progenitor_support::encode_path(command_id),
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
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customerId}/devices/chromeos/{deviceId}:issueCommand` endpoint.
     *
     * Issues a command for the device to execute.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable. Immutable ID of the Google Workspace account.
     * * `device_id: &str` -- Immutable. Immutable ID of Chrome OS Device.
     */
    pub async fn admin_devices_chromeos_issue_command(
        &self,
        customer_id: &str,
        device_id: &str,
        body: &crate::types::DirectoryChromeosdevicesIssueCommandRequest,
    ) -> ClientResult<crate::types::DirectoryChromeosdevicesIssueCommandResponse> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/devices/chromeos/{}/issueCommand",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(device_id),
            ),
            None,
        );
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
}

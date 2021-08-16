use anyhow::Result;

use crate::Client;

pub struct Customer {
    client: Client,
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
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer_id: &str,
        device_id: &str,
        command_id: &str,
    ) -> Result<crate::types::DirectoryChromeosdevicesCommand> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/devices/chromeos/{}/commands/{}?{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&device_id.to_string()),
            crate::progenitor_support::encode_path(&command_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
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
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer_id: &str,
        device_id: &str,
        body: &crate::types::DirectoryChromeosdevicesIssueCommandRequest,
    ) -> Result<crate::types::DirectoryChromeosdevicesIssueCommandResponse> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/devices/chromeos/{}/issueCommand?{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&device_id.to_string()),
            query_
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}

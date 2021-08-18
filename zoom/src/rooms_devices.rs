use anyhow::Result;

use crate::Client;

pub struct RoomsDevices {
    pub client: Client,
}

impl RoomsDevices {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        RoomsDevices { client }
    }

    /**
     * Change Zoom Rooms' app version.
     *
     * This function performs a `PUT` to the `/rooms/{roomId}/devices/{deviceId}/app_version` endpoint.
     *
     * [Upgrade](https://support.zoom.us/hc/en-us/articles/204675449-Upgrade-or-Downgrade-Zoom-Rooms-Software#h_1751c48a-644e-4a60-b96a-31ec77c616e6) or [downgrade](https://support.zoom.us/hc/en-us/articles/204675449-Upgrade-or-Downgrade-Zoom-Rooms-Software#h_d97349d6-9253-484c-af80-350475026524) the version of Zoom Rooms App installed in your Mac or Windows device.
     *
     * **Prerequisites:**<br>
     * * Pro or a higher account with Zoom Rooms.
     * * Zoom Rooms software must be installed either on a Mac or a Windows device. This API does not support other devices.
     *
     * **Parameters:**
     *
     * * `room_id: &str` -- Unique Identifier of the Zoom Room.
     * * `device_id: &str` -- Unique Identifier of the Mac or the Windows device. The value of this field can be retrieved from the [List Zoom Room Devices API](https://marketplace.zoom.us/docs/api-reference/zoom-api/rooms/listzrdevices).
     */
    pub async fn change_zoom_rooms_app_version(
        &self,
        room_id: &str,
        device_id: &str,
        body: &crate::types::ChangeZoomRoomsAppVersionRequest,
    ) -> Result<()> {
        let url = format!(
            "/rooms/{}/devices/{}/app_version",
            crate::progenitor_support::encode_path(&room_id.to_string()),
            crate::progenitor_support::encode_path(&device_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

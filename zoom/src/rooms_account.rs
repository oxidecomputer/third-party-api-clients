use anyhow::Result;

use crate::Client;

pub struct RoomsAccount {
    pub client: Client,
}

impl RoomsAccount {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        RoomsAccount { client }
    }

    /**
     * Get Zoom Room account profile.
     *
     * This function performs a `GET` to the `/rooms/account_profile` endpoint.
     *
     * Get details on the account profile of a Zoom Room. This information can only by accessed either by the Zoom Room Account Owner or a user with Zoom Rooms admin permission. To get information on an individual Room Profile, use [Get Zoom Room Profile API](https://marketplace.zoom.us/docs/api-reference/zoom-api/rooms/getzrprofile) instead.
     *
     * **Prerequisites:**<br>
     * * Zoom account owner or Zoom Rooms admin permissions<br>
     *
     * **Scopes:** `room:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn get_zr_account_profile(&self) -> Result<crate::types::UpdateZrAccProfileRequest> {
        let url = "/rooms/account_profile".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Update Zoom Room account profile.
     *
     * This function performs a `PATCH` to the `/rooms/account_profile` endpoint.
     *
     * Update information on the account profile of a Zoom Room. This information can only by accessed either by the Zoom Room Account Owner or a user with Zoom Rooms admin permission. To update information on an individual Room Profile, use [Update Zoom Room Profile API](https://marketplace.zoom.us/docs/api-reference/zoom-api/rooms/updatezrprofile) instead.
     *
     * **Prerequisites:**<br>
     * * Zoom account owner or Zoom Rooms admin permissions<br>
     *
     * **Scopes:** `room:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn update_zr_acc_profile(
        &self,
        body: &crate::types::UpdateZrAccProfileRequest,
    ) -> Result<crate::types::Domains> {
        let url = "/rooms/account_profile".to_string();
        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get Zoom Room account settings.
     *
     * This function performs a `GET` to the `/rooms/account_settings` endpoint.
     *
     * Get details on Account Settings of a Zoom Room. With this API, you can view either the **Account Meeting Settings** or the **Alert Settings** (Client Alert Settings and Notfication Settings) of the Zoom Rooms account. By default, only **Account Meeting Settings** are returned. To view only **Alert Settings**, specify `alert` as the value of the `setting_type` query parameter.<br><br>
     * **Prerequisites:**<br>
     * * Zoom Room licenses
     * * Owner or Admin privileges on the Zoom Account.<br>
     * **Scopes:** `room:read:admin`<br><br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `setting_type: crate::types::SettingType` -- The type of setting that you would like to retrieve.<br> `alert`: Alert Settings applied on the Zoom Rooms Account.<br>
     *  `meeting`: Meeting settings of the Zoom Rooms Account. <br>
     *  `signage`: View digital signage settings of the Zoom Rooms Account.
     */
    pub async fn get_zr_account_setting(
        &self,
        setting_type: crate::types::SettingType,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !setting_type.to_string().is_empty() {
            query_args.push(("setting_type".to_string(), setting_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/rooms/account_settings?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Update Zoom Room account settings.
     *
     * This function performs a `PATCH` to the `/rooms/account_settings` endpoint.
     *
     * Update account settings applied for Zoom Rooms in a Zoom account. With this API, you can update either the **Account Meeting Settings** or the **Alert Settings** (Client Alert Settings and Notfication Settings) of the Zoom Rooms account by specifying the required setting type in the `setting_type` parameter. To update only **Alert Settings**, specify `alert` as the value of the `setting_type` query parameter and to update only **Account Meeting Settings**, specify `meeting` as the value of the `setting_type` query parameter.<br><br>
     * **Prerequisites:**<br>
     * * Zoom Room licenses
     * * Owner or Admin privileges on the Zoom Account.<br>
     * **Scopes:** `room:write:admin`<br>
     *
     *   **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `setting_type: &str` -- The type of setting that you would like to update.<br> `alert`: Alert Settings applied on the Zoom Rooms Account.<br>
     *   `meeting`: Meeting settings of the Zoom Rooms Account.<br>
     *   `signage`: View digital signage settings of the Zoom Rooms Account.
     */
    pub async fn update_zoom_room_acc_settings(
        &self,
        setting_type: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !setting_type.is_empty() {
            query_args.push(("setting_type".to_string(), setting_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/rooms/account_settings?{}", query_);

        self.client.patch(&url, None).await
    }
}

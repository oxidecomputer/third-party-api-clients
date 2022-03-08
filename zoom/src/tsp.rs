use anyhow::Result;

use crate::Client;

pub struct Tsp {
    pub client: Client,
}

impl Tsp {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Tsp { client }
    }

    /**
     * Get account's TSP information.
     *
     * This function performs a `GET` to the `/tsp` endpoint.
     *
     * Get information on Telephony Service Provider on an account level.<br><br>
     * **Scopes:** `tsp:read:admin` <br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**<br>
     * * A Pro or a higher plan.
     */
    pub async fn get(&self) -> Result<crate::types::TspResponse> {
        let url = "/tsp".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Update account's TSP information.
     *
     * This function performs a `PATCH` to the `/tsp` endpoint.
     *
     * Update information of the Telephony Service Provider set up on an account.<br>
     * **Prerequisites**:<br>
     * TSP account option should be enabled.<br>
     * **Scopes:** `tsp:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     */
    pub async fn update(&self, body: &crate::types::TspUpdateRequest) -> Result<()> {
        let url = "/tsp".to_string();
        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List user's TSP accounts.
     *
     * This function performs a `GET` to the `/users/{userId}/tsp` endpoint.
     *
     * A user can have a maximum of two TSP accounts. Use this API to list all TSP accounts of a user.<br><br>
     * **Scopes:** `tsp:read:admin` `tsp:read`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn user_ts_ps(&self, user_id: &str) -> Result<crate::types::UserTsPsResponse> {
        let url = format!(
            "/users/{}/tsp",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Add a user's TSP account.
     *
     * This function performs a `POST` to the `/users/{userId}/tsp` endpoint.
     *
     * Add a user's TSP account.<br><br>
     * **Scopes:** `tsp:write:admin` `tsp:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn user_create(
        &self,
        user_id: &str,
        body: &crate::types::TspAccountsList,
    ) -> Result<crate::types::TspAccountsList> {
        let url = format!(
            "/users/{}/tsp",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a user's TSP account.
     *
     * This function performs a `GET` to the `/users/{userId}/tsp/{tspId}` endpoint.
     *
     * Each user can have a maximum of two TSP accounts. Use this API to retrieve details of a specific TSP account enabled for a specific user.<br><br>
     * **Scopes:** `tsp:read:admin` `tsp:read`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `tsp_id: &str` -- Audio types:<br>`1` - Toll-free Call-in & Call-out.<br>`2` - Toll <br>
     *  `3` - SIP Connected Audio.
     */
    pub async fn user(&self, user_id: &str, tsp_id: &str) -> Result<crate::types::TspAccount> {
        let url = format!(
            "/users/{}/tsp/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&tsp_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a user's TSP account.
     *
     * This function performs a `DELETE` to the `/users/{userId}/tsp/{tspId}` endpoint.
     *
     * Delete a user's TSP account.<br><br>
     * **Scopes:** `tsp:write:admin` `tsp:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `tsp_id: &str` -- Audio types:<br>`1` - Toll-free Call-in & Call-out.<br>`2` - Toll <br>
     *  `3` - SIP Connected Audio.
     */
    pub async fn user_delete(&self, user_id: &str, tsp_id: &str) -> Result<()> {
        let url = format!(
            "/users/{}/tsp/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&tsp_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a TSP account.
     *
     * This function performs a `PATCH` to the `/users/{userId}/tsp/{tspId}` endpoint.
     *
     * Update a user's TSP account.<br><br>
     * **Scopes:** `tsp:write:admin` `tsp:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `tsp_id: &str` -- Audio types:<br>`1` - Toll-free Call-in & Call-out.<br>`2` - Toll <br>
     *  `3` - SIP Connected Audio.
     */
    pub async fn user_update(
        &self,
        user_id: &str,
        tsp_id: &str,
        body: &crate::types::TspAccountData,
    ) -> Result<()> {
        let url = format!(
            "/users/{}/tsp/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&tsp_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Set global dial-in URL for a TSP user.
     *
     * This function performs a `PATCH` to the `/users/{userId}/tsp/settings` endpoint.
     *
     * A global dial-in page can provide a list of global access numbers using which audio conferencing can be conducted. By calling this API, you can set the url for the global dial-in page of a user whose Zoom account has TSP and special TSP with third-party audio conferencing options enabled. <p></p>
     * **Scopes:**`tsp:write:admin` `tsp:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The userId or email address of the user.
     */
    pub async fn url_update(
        &self,
        user_id: &str,
        body: &crate::types::TspGlobalDialIn,
    ) -> Result<()> {
        let url = format!(
            "/users/{}/tsp/settings",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

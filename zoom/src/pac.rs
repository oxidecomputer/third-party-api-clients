use anyhow::Result;

use crate::Client;

pub struct Pac {
    pub client: Client,
}

impl Pac {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Pac { client }
    }

    /**
     * List a user's PAC accounts.
     *
     * This function performs a `GET` to the `/users/{userId}/pac` endpoint.
     *
     * Use this API to list a user's [Personal Audio Conference](https://support.zoom.us/hc/en-us/articles/204517069-Getting-Started-with-Personal-Audio-Conference) accounts. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * PAC allows Pro or higher account holders to host meetings through PSTN (phone dial-in) only.
     *
     * **Scopes:** `pac:read:admin`, `pac:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Pro or higher plan with [Premium Audio Conferencing](https://support.zoom.us/hc/en-us/articles/204517069-Getting-Started-with-Personal-Audio-Conference) add-on.
     * * Personal Audio Conference must be enabled in the user's profile.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn user_pa_cs(&self, user_id: &str) -> Result<crate::types::UserPaCsResponse> {
        let url = format!(
            "/users/{}/pac",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }
}

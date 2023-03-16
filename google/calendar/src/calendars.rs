use crate::Client;
use crate::ClientResult;

pub struct Calendars {
    pub client: Client,
}

impl Calendars {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Calendars { client }
    }

    /**
     * This function performs a `POST` to the `/calendars` endpoint.
     *
     * Creates a secondary calendar.
     */
    pub async fn insert(
        &self,
        body: &crate::types::Calendar,
    ) -> ClientResult<crate::Response<crate::types::Calendar>> {
        let url = self.client.url("/calendars", None);
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
     * This function performs a `GET` to the `/calendars/{calendarId}` endpoint.
     *
     * Returns metadata for a calendar.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     */
    pub async fn get(
        &self,
        calendar_id: &str,
    ) -> ClientResult<crate::Response<crate::types::Calendar>> {
        let url = self.client.url(
            &format!(
                "/calendars/{}",
                crate::progenitor_support::encode_path(calendar_id),
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
     * This function performs a `PUT` to the `/calendars/{calendarId}` endpoint.
     *
     * Updates metadata for a calendar.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     */
    pub async fn update(
        &self,
        calendar_id: &str,
        body: &crate::types::Calendar,
    ) -> ClientResult<crate::Response<crate::types::Calendar>> {
        let url = self.client.url(
            &format!(
                "/calendars/{}",
                crate::progenitor_support::encode_path(calendar_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `DELETE` to the `/calendars/{calendarId}` endpoint.
     *
     * Deletes a secondary calendar. Use calendars.clear for clearing all events on primary calendars.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     */
    pub async fn delete(&self, calendar_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/calendars/{}",
                crate::progenitor_support::encode_path(calendar_id),
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
     * This function performs a `PATCH` to the `/calendars/{calendarId}` endpoint.
     *
     * Updates metadata for a calendar. This method supports patch semantics.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     */
    pub async fn patch(
        &self,
        calendar_id: &str,
        body: &crate::types::Calendar,
    ) -> ClientResult<crate::Response<crate::types::Calendar>> {
        let url = self.client.url(
            &format!(
                "/calendars/{}",
                crate::progenitor_support::encode_path(calendar_id),
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
    /**
     * This function performs a `POST` to the `/calendars/{calendarId}/clear` endpoint.
     *
     * Clears a primary calendar. This operation deletes all events associated with the primary calendar of an account.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     */
    pub async fn clear(&self, calendar_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/calendars/{}/clear",
                crate::progenitor_support::encode_path(calendar_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}

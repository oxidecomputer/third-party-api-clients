use anyhow::Result;

use crate::Client;

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
    pub async fn insert(&self, body: &crate::types::Calendar) -> Result<crate::types::Calendar> {
        let url = "/calendars".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn get(&self, calendar_id: &str) -> Result<crate::types::Calendar> {
        let url = format!(
            "/calendars/{}",
            crate::progenitor_support::encode_path(&calendar_id.to_string()),
        );

        self.client.get(&url, None).await
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
    ) -> Result<crate::types::Calendar> {
        let url = format!(
            "/calendars/{}",
            crate::progenitor_support::encode_path(&calendar_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn delete(&self, calendar_id: &str) -> Result<()> {
        let url = format!(
            "/calendars/{}",
            crate::progenitor_support::encode_path(&calendar_id.to_string()),
        );

        self.client.delete(&url, None).await
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
    ) -> Result<crate::types::Calendar> {
        let url = format!(
            "/calendars/{}",
            crate::progenitor_support::encode_path(&calendar_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn clear(&self, calendar_id: &str) -> Result<()> {
        let url = format!(
            "/calendars/{}/clear",
            crate::progenitor_support::encode_path(&calendar_id.to_string()),
        );

        self.client.post(&url, None).await
    }
}

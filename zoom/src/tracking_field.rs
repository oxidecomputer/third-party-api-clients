use anyhow::Result;

use crate::Client;

pub struct TrackingField {
    pub client: Client,
}

impl TrackingField {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TrackingField { client }
    }

    /**
     * List tracking fields.
     *
     * This function performs a `GET` to the `/tracking_fields` endpoint.
     *
     * [Tracking fields](https://support.zoom.us/hc/en-us/articles/115000293426-Scheduling-Tracking-Fields) allow you to analyze usage by various fields within an organization.<br> Use this API to list all the tracking fields on your Zoom account.<br><br>
     * **Scopes:** `trackingfield:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     * **Prerequisites:**
     * * Business, Education, API or higher plan
     */
    pub async fn trackingfield_list(&self) -> Result<crate::types::Domains> {
        let url = "/tracking_fields".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Create a tracking field.
     *
     * This function performs a `POST` to the `/tracking_fields` endpoint.
     *
     * [Tracking fields](https://support.zoom.us/hc/en-us/articles/115000293426-Scheduling-Tracking-Fields) allow you to analyze usage by various fields within an organization.<br> Use this API to create a new tracking field.<br><br>
     * **Scope:** `trackingfield:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**
     * * Business, Education, API or higher plan
     */
    pub async fn trackingfield_create(
        &self,
        body: &crate::types::TrackingField,
    ) -> Result<crate::types::TrackingfieldGetResponseAllOf> {
        let url = "/tracking_fields".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a tracking field.
     *
     * This function performs a `GET` to the `/tracking_fields/{fieldId}` endpoint.
     *
     * [Tracking fields](https://support.zoom.us/hc/en-us/articles/115000293426-Scheduling-Tracking-Fields) allow you to analyze usage by various fields within an organization.<br><br> When scheduling a meeting, the tracking field will be included in the meeting options.<br>Use this API to get information on a tracking field.<br><br>
     * **Scopes:** `trackingfield:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**
     * * Business, Education, API or higher plan
     *
     *
     * **Parameters:**
     *
     * * `field_id: &str` -- The Tracking Field ID.
     */
    pub async fn trackingfield_get(
        &self,
        field_id: &str,
    ) -> Result<crate::types::TrackingfieldGetResponseAllOf> {
        let url = format!(
            "/tracking_fields/{}",
            crate::progenitor_support::encode_path(&field_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a tracking field.
     *
     * This function performs a `DELETE` to the `/tracking_fields/{fieldId}` endpoint.
     *
     * [Tracking fields](https://support.zoom.us/hc/en-us/articles/115000293426-Scheduling-Tracking-Fields) allow you to analyze usage by various fields within an organization.<br> Use this API to delete a tracking field.<br><br>
     * **Scope:** `trackingfield:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**
     * * Business, Education, API or higher plan
     *
     * **Parameters:**
     *
     * * `field_id: &str` -- The Tracking Field ID.
     */
    pub async fn trackingfield_delete(&self, field_id: &str) -> Result<()> {
        let url = format!(
            "/tracking_fields/{}",
            crate::progenitor_support::encode_path(&field_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a tracking field.
     *
     * This function performs a `PATCH` to the `/tracking_fields/{fieldId}` endpoint.
     *
     * [Tracking fields](https://support.zoom.us/hc/en-us/articles/115000293426-Scheduling-Tracking-Fields) allow you to analyze usage by various fields within an organization.<br> Use this API to update a tracking field.<br><br>
     * **Scope:** `trackingfield:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**
     * * Business, Education, API or higher plan
     *
     * **Parameters:**
     *
     * * `field_id: &str` -- The Tracking Field ID.
     */
    pub async fn trackingfield_update(
        &self,
        field_id: &str,
        body: &crate::types::TrackingField,
    ) -> Result<()> {
        let url = format!(
            "/tracking_fields/{}",
            crate::progenitor_support::encode_path(&field_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

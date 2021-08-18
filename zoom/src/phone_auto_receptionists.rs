use anyhow::Result;

use crate::Client;

pub struct PhoneAutoReceptionists {
    pub client: Client,
}

impl PhoneAutoReceptionists {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PhoneAutoReceptionists { client }
    }

    /**
     * Update auto receptionist details.
     *
     * This function performs a `PATCH` to the `/phone/auto_receptionists/{autoReceptionistId}` endpoint.
     *
     * An auto receptionist answers calls with a personalized recording and routes calls to a phone user, call queue, common area phone, or voicemail. An auto receptionist can also be set up so that it routes calls to an interactive voice response (IVR) system to allow callers to select the routing options.<br>
     * Use this API to [change information](https://support.zoom.us/hc/en-us/articles/360021121312-Managing-Auto-Receptionists-and-Interactive-Voice-Response-IVR-#h_1d5ffc56-6ba3-4ce5-9d86-4a1a1ee743f3) such as display name and extension number assigned to the main auto receptionist.<br><br>
     * **Prerequisites:**<br>
     * * Pro or higher account with Zoom Phone license.<br>
     * **Scopes:** `phone:write:admin` <br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `auto_receptionist_id: &str` -- Unique Identifier of the Auto Receptionist. It can be retrieved from the [List Sites API](https://marketplace.zoom.us/docs/api-reference/zoom-api/phone-site/listphonesites).
     */
    pub async fn update_auto_receptionist(
        &self,
        auto_receptionist_id: &str,
        body: &crate::types::UpdateAutoReceptionistRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/auto_receptionists/{}",
            crate::progenitor_support::encode_path(&auto_receptionist_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Assign phone numbers.
     *
     * This function performs a `POST` to the `/phone/auto_receptionists/{autoReceptionistId}/phone_numbers` endpoint.
     *
     * Assign available phone numbers to an [auto receptionist](https://support.zoom.us/hc/en-us/articles/360021121312-Managing-Auto-Receptionists-and-Interactive-Voice-Response-IVR-). The available numbers can be retrieved using the List Phone Numbers API with `type` query parameter set to "unassigned".
     *
     * **Prerequisites:**
     * * Pro or higher account plan with Zoom Phone License
     * * Account owner or admin permissions<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `auto_receptionist_id: &str` -- Unique Identifier of the Auto Receptionist. It can be retrieved from the [List Sites API](https://marketplace.zoom.us/docs/api-reference/zoom-api/phone-site/listphonesites).
     */
    pub async fn assign_phone_numbers_auto_receptionist(
        &self,
        auto_receptionist_id: &str,
        body: &crate::types::AddByocNumberResponse,
    ) -> Result<()> {
        let url = format!(
            "/phone/auto_receptionists/{}/phone_numbers",
            crate::progenitor_support::encode_path(&auto_receptionist_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign all phone numbers.
     *
     * This function performs a `DELETE` to the `/phone/auto_receptionists/{autoReceptionistId}/phone_numbers` endpoint.
     *
     * Unassign all phone numbers that were previously assigned to an [auto receptionist](https://support.zoom.us/hc/en-us/articles/360021121312-Managing-Auto-Receptionists-and-Interactive-Voice-Response-IVR-).
     *
     * **Prerequisites:**
     * * Pro or higher account plan with Zoom Phone License
     * * Account owner or admin permissions<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     */
    pub async fn unassign_all_phone_nums_auto_receptionist(
        &self,
        auto_receptionist_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/phone/auto_receptionists/{}/phone_numbers",
            crate::progenitor_support::encode_path(&auto_receptionist_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Unassign a phone number.
     *
     * This function performs a `DELETE` to the `/phone/auto_receptionists/{autoReceptionistId}/phone_numbers/{phoneNumberId}` endpoint.
     *
     * Unassign a specific phone number that was previously assigned to an [auto receptionist](https://support.zoom.us/hc/en-us/articles/360021121312-Managing-Auto-Receptionists-and-Interactive-Voice-Response-IVR-).
     *
     * **Prerequisites:**
     * * Pro or higher account plan with Zoom Phone License
     * * Account owner or admin permissions<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `auto_receptionist_id: &str` -- Unique identifier of the auto receptionist. This can be retrieved from the List Phone Sites API.
     * * `phone_number_id: &str` -- Unique Identifier of the phone number or provide the actual phone number in e164 format (example: +19995550123).
     */
    pub async fn unassign_phone_num_auto_receptionist(
        &self,
        auto_receptionist_id: &str,
        phone_number_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/phone/auto_receptionists/{}/phone_numbers/{}",
            crate::progenitor_support::encode_path(&auto_receptionist_id.to_string()),
            crate::progenitor_support::encode_path(&phone_number_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Add an auto receptionist.
     *
     * This function performs a `POST` to the `/phone/auto_receptionists` endpoint.
     *
     * Auto receptionists answer calls with a personalized recording and routes calls to a phone user, call queue, common area phone, voicemail or an IVR system. Use this API to add an [auto receptionist](https://support.zoom.us/hc/en-us/articles/360021121312-Managing-Auto-Receptionists-and-Interactive-Voice-Response-IVR-) to a Zoom Phone.<br>
     *
     * **Prerequisites:**<br>
     * * Pro or higher account with Zoom Phone license.<br>
     * **Scopes:** `phone:write:admin` <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     */
    pub async fn add_auto_receptionist(
        &self,
        body: &crate::types::AddAutoReceptionistRequest,
    ) -> Result<crate::types::AddAutoReceptionistResponse> {
        let url = "/phone/auto_receptionists".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

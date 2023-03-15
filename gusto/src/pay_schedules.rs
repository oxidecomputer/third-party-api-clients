use crate::Client;
use crate::ClientResult;

pub struct PaySchedules {
    pub client: Client,
}

impl PaySchedules {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PaySchedules { client }
    }

    /**
     * Get the pay schedules for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/pay_schedules` endpoint.
     *
     * The pay schedule object in Gusto captures the details of when employees work and when they should be paid. A company can have multiple pay schedules.
     */
    pub async fn get_company(
        &self,
        company_id: &str,
    ) -> ClientResult<Vec<crate::types::PaySchedule>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/pay_schedules",
                crate::progenitor_support::encode_path(company_id),
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
     * Get the pay schedules for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/pay_schedules` endpoint.
     *
     * As opposed to `get_company`, this function returns all the pages of the request at once.
     *
     * The pay schedule object in Gusto captures the details of when employees work and when they should be paid. A company can have multiple pay schedules.
     */
    pub async fn get_all_company(
        &self,
        company_id: &str,
    ) -> ClientResult<Vec<crate::types::PaySchedule>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/pay_schedules",
                crate::progenitor_support::encode_path(company_id),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a pay schedule.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/pay_schedules/{pay_schedule_id_or_uuid}` endpoint.
     *
     * The pay schedule object in Gusto captures the details of when employees work and when they should be paid. A company can have multiple pay schedules.
     */
    pub async fn get_company_schedule(
        &self,
        company_id_or_uuid: &str,
        pay_schedule_id_or_uuid: &str,
    ) -> ClientResult<crate::types::PaySchedule> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/pay_schedules/{}",
                crate::progenitor_support::encode_path(company_id_or_uuid),
                crate::progenitor_support::encode_path(pay_schedule_id_or_uuid),
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
     * Update a pay schedule.
     *
     * This function performs a `PUT` to the `/v1/companies/{company_id_or_uuid}/pay_schedules/{pay_schedule_id_or_uuid}` endpoint.
     *
     * Updates a pay schedule.
     *
     * This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto
     */
    pub async fn put_company_schedule(
        &self,
        company_id_or_uuid: &str,
        pay_schedule_id_or_uuid: &str,
        body: &crate::types::PutCompanyPaySchedulesScheduleRequest,
    ) -> ClientResult<crate::types::PaySchedule> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/pay_schedules/{}",
                crate::progenitor_support::encode_path(company_id_or_uuid),
                crate::progenitor_support::encode_path(pay_schedule_id_or_uuid),
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
}

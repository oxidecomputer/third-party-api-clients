use crate::Client;
use crate::ClientResult;

pub struct PowerFormData {
    pub client: Client,
}

impl PowerFormData {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PowerFormData { client }
    }

    /**
     * Returns the data that users entered in a PowerForm.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/powerforms/{powerFormId}/form_data` endpoint.
     *
     * This method enables Powerform Administrators or the sender of a PowerForm to download the data that recipients have entered into a PowerForm.
     *
     * You specify the format in which you want to retrieve the data in the `Accept` header. This header accepts the following values:
     *
     *
     * - `application/json`: JSON format
     * - `application/xml`: XML format
     * - `text/csv`: Comma-separated value (CSV) format
     *
     * **Note**: Only PowerForm Administrators or the PowerForm Sender can download the data associated with a PowerForm.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `power_form_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `data_layout: &str` -- The layout in which to return the PowerForm data. Valid values are:
     *   
     *   - `Native`
     *   - `Csv_Classic`
     *   - `Csv_One_Envelope_Per_Line`
     *   - `Xml_Classic`.
     * * `from_date: &str` -- The start date for a date range in UTC DateTime format.
     *   
     *   **Note**: If this property is null, no date filtering is applied.
     * * `to_date: &str` -- The end date of a date range in UTC DateTime format. The default value is `UtcNow`.
     */
    pub async fn power_forms_get_form_data(
        &self,
        account_id: &str,
        power_form_id: &str,
        data_layout: &str,
        from_date: &str,
        to_date: &str,
    ) -> ClientResult<crate::types::PowerFormsFormDataResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !data_layout.is_empty() {
            query_args.push(("data_layout".to_string(), data_layout.to_string()));
        }
        if !from_date.is_empty() {
            query_args.push(("from_date".to_string(), from_date.to_string()));
        }
        if !to_date.is_empty() {
            query_args.push(("to_date".to_string(), to_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/powerforms/{}/form_data?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(power_form_id),
                query_
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
}

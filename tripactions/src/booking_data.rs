use crate::Client;
use crate::ClientResult;

pub struct BookingData {
    pub client: Client,
}

impl BookingData {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        BookingData { client }
    }

    /**
     * Your company's bookings.
     *
     * This function performs a `GET` to the `/v1/bookings` endpoint.
     *
     * Return booking rows filtered by the parameters you select.
     *
     * **Parameters:**
     *
     * * `created_from: &str` -- Filter based on booking created date in epoch seconds.
     * * `created_to: &str` -- Filter based on booking created date in epoch seconds.
     * * `start_date_from: &str` -- Filter based on travel start date in epoch seconds.
     * * `start_date_to: &str` -- Filter based on travel end date in epoch seconds.
     * * `booking_status: crate::types::BookingStatus` -- Filter based on booking status.
     * * `page: u64` -- Page cursor for use in pagination.
     * * `size: i64` -- Number of records returned per page.
     * * `booking_type: crate::types::BookingType` -- Filter based on Booking type.
     */
    pub async fn get_booking_report(
        &self,
        created_from: &str,
        created_to: &str,
        start_date_from: &str,
        start_date_to: &str,
        booking_status: crate::types::BookingStatus,
        page: u64,
        size: i64,
        booking_type: crate::types::BookingType,
    ) -> ClientResult<Vec<crate::types::BookingReport>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !booking_status.to_string().is_empty() {
            query_args.push(("bookingStatus".to_string(), booking_status.to_string()));
        }
        if !booking_type.to_string().is_empty() {
            query_args.push(("bookingType".to_string(), booking_type.to_string()));
        }
        if !created_from.is_empty() {
            query_args.push(("createdFrom".to_string(), created_from.to_string()));
        }
        if !created_to.is_empty() {
            query_args.push(("createdTo".to_string(), created_to.to_string()));
        }
        if !page.to_string().is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if size > 0 {
            query_args.push(("size".to_string(), size.to_string()));
        }
        if !start_date_from.is_empty() {
            query_args.push(("startDateFrom".to_string(), start_date_from.to_string()));
        }
        if !start_date_to.is_empty() {
            query_args.push(("startDateTo".to_string(), start_date_to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/bookings?{}", query_), None);
        let resp: crate::types::BookingReportResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }
    /**
     * Your company's bookings.
     *
     * This function performs a `GET` to the `/v1/bookings` endpoint.
     *
     * As opposed to `get_booking_report`, this function returns all the pages of the request at once.
     *
     * Return booking rows filtered by the parameters you select.
     */
    pub async fn get_all_booking_report(
        &self,
        created_from: &str,
        created_to: &str,
        start_date_from: &str,
        start_date_to: &str,
        booking_status: crate::types::BookingStatus,
        booking_type: crate::types::BookingType,
    ) -> ClientResult<Vec<crate::types::BookingReport>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !booking_status.to_string().is_empty() {
            query_args.push(("bookingStatus".to_string(), booking_status.to_string()));
        }
        if !booking_type.to_string().is_empty() {
            query_args.push(("bookingType".to_string(), booking_type.to_string()));
        }
        if !created_from.is_empty() {
            query_args.push(("createdFrom".to_string(), created_from.to_string()));
        }
        if !created_to.is_empty() {
            query_args.push(("createdTo".to_string(), created_to.to_string()));
        }
        if !start_date_from.is_empty() {
            query_args.push(("startDateFrom".to_string(), start_date_from.to_string()));
        }
        if !start_date_to.is_empty() {
            query_args.push(("startDateTo".to_string(), start_date_to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/bookings?{}", query_), None);

        let mut resp: crate::types::BookingReportResponse = if !url.contains('?') {
            self.client
                .get(
                    &format!("{}?page=0&size=100", url),
                    crate::Message {
                        body: None,
                        content_type: None,
                    },
                )
                .await?
        } else {
            self.client
                .get(
                    &format!("{}&page=0&size=100", url),
                    crate::Message {
                        body: None,
                        content_type: None,
                    },
                )
                .await?
        };

        let mut data = resp.data;
        let mut page = resp.page.current_page + 1;

        // Paginate if we should.
        while page <= (resp.page.total_pages - 1) {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?page={}&size=100", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                resp = self
                    .client
                    .get(
                        &format!("{}&page={}&size=100", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            data.append(&mut resp.data);

            page = resp.page.current_page + 1;
        }

        // Return our response data.
        Ok(data)
    }
}

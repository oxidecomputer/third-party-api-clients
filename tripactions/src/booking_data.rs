use anyhow::Result;

use crate::Client;

pub struct BookingData {
    client: Client,
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
    ) -> Result<Vec<crate::types::BookingReport>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("booking_status={}", booking_status));
        query_args.push(format!("booking_type={}", booking_type));
        if !created_from.is_empty() {
            query_args.push(format!("created_from={}", created_from));
        }
        if !created_to.is_empty() {
            query_args.push(format!("created_to={}", created_to));
        }
        query_args.push(format!("page={}", page));
        if size > 0 {
            query_args.push(format!("size={}", size));
        }
        if !start_date_from.is_empty() {
            query_args.push(format!("start_date_from={}", start_date_from));
        }
        if !start_date_to.is_empty() {
            query_args.push(format!("start_date_to={}", start_date_to));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/v1/bookings?{}", query_);

        let resp: crate::types::BookingReportResponse = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.data)
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
    ) -> Result<Vec<crate::types::BookingReport>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("booking_status={}", booking_status));
        query_args.push(format!("booking_type={}", booking_type));
        if !created_from.is_empty() {
            query_args.push(format!("created_from={}", created_from));
        }
        if !created_to.is_empty() {
            query_args.push(format!("created_to={}", created_to));
        }
        if !start_date_from.is_empty() {
            query_args.push(format!("start_date_from={}", start_date_from));
        }
        if !start_date_to.is_empty() {
            query_args.push(format!("start_date_to={}", start_date_to));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/v1/bookings?{}", query_);

        self.client.get_all_pages(&url, None).await
    }
}

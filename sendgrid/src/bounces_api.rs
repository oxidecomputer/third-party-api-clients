use crate::Client;
use crate::ClientResult;

pub struct BouncesApi {
    pub client: Client,
}

impl BouncesApi {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        BouncesApi { client }
    }

    /**
     * Retrieve all bounces.
     *
     * This function performs a `GET` to the `/suppression/bounces` endpoint.
     *
     * **This endpoint allows you to retrieve all of your bounces.**
     *
     * **Parameters:**
     *
     * * `start_time: i64` -- Refers start of the time range in unix timestamp when a bounce was created (inclusive).
     * * `end_time: i64` -- Refers end of the time range in unix timestamp when a bounce was created (inclusive).
     * * `accept: &str` -- The license key provided with your New Relic account.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_suppression_bounces(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> ClientResult<Vec<crate::types::BounceResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if end_time > 0 {
            query_args.push(("end_time".to_string(), end_time.to_string()));
        }
        if start_time > 0 {
            query_args.push(("start_time".to_string(), start_time.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/suppression/bounces?{}", query_), None);
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
     * Retrieve all bounces.
     *
     * This function performs a `GET` to the `/suppression/bounces` endpoint.
     *
     * As opposed to `get_suppression_bounces`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all of your bounces.**
     */
    pub async fn get_all_suppression_bounces(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> ClientResult<Vec<crate::types::BounceResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if end_time > 0 {
            query_args.push(("end_time".to_string(), end_time.to_string()));
        }
        if start_time > 0 {
            query_args.push(("start_time".to_string(), start_time.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/suppression/bounces?{}", query_), None);
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
     * Delete bounces.
     *
     * This function performs a `DELETE` to the `/suppression/bounces` endpoint.
     *
     * **This endpoint allows you to delete all emails on your bounces list.**
     *
     * There are two options for deleting bounced emails:
     *
     * 1. You can delete all bounced emails by setting `delete_all` to `true` in the request body.
     * 2. You can delete a selection of bounced emails by specifying the email addresses in the `emails` array of the request body.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_suppression_bounces(
        &self,
        body: &crate::types::DeleteSuppressionBouncesRequest,
    ) -> ClientResult<()> {
        let url = self.client.url("/suppression/bounces", None);
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieve a Bounce.
     *
     * This function performs a `GET` to the `/suppression/bounces/{email}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific bounce by email address.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_suppression_bounces_email(
        &self,
        email: &str,
    ) -> ClientResult<Vec<crate::types::BounceResponse>> {
        let url = self.client.url(
            &format!(
                "/suppression/bounces/{}",
                crate::progenitor_support::encode_path(email),
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
     * Retrieve a Bounce.
     *
     * This function performs a `GET` to the `/suppression/bounces/{email}` endpoint.
     *
     * As opposed to `get_suppression_bounces_email`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a specific bounce by email address.**
     */
    pub async fn get_all_suppression_bounces_email(
        &self,
        email: &str,
    ) -> ClientResult<Vec<crate::types::BounceResponse>> {
        let url = self.client.url(
            &format!(
                "/suppression/bounces/{}",
                crate::progenitor_support::encode_path(email),
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
     * Delete a bounce.
     *
     * This function performs a `DELETE` to the `/suppression/bounces/{email}` endpoint.
     *
     * **This endpoint allows you to remove an email address from your bounce list.**
     *
     * **Parameters:**
     *
     * * `email_address: &str` -- The email address you would like to remove from the bounce list.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_suppression_bounces_email(
        &self,
        email: &str,
        email_address: &str,
        body: &serde_json::Value,
    ) -> ClientResult<crate::types::Help> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email_address.is_empty() {
            query_args.push(("email_address".to_string(), email_address.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/suppression/bounces/{}?{}",
                crate::progenitor_support::encode_path(email),
                query_
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
}

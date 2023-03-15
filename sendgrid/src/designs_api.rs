use crate::Client;
use crate::ClientResult;

pub struct DesignsApi {
    pub client: Client,
}

impl DesignsApi {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        DesignsApi { client }
    }

    /**
     * Get Design.
     *
     * This function performs a `GET` to the `/designs/{id}` endpoint.
     *
     * **This endpoint allows you to retrieve a single design**.
     *
     * A GET request to `/designs/{id}` will retrieve details about a specific design in your Design Library.
     *
     * This endpoint is valuable when retrieving information stored in a field that you wish to update using a PATCH request.
     */
    pub async fn get_design(&self, id: &str) -> ClientResult<crate::types::DesignOutputAllOf> {
        let url = self.client.url(
            &format!("/designs/{}", crate::progenitor_support::encode_path(id),),
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
     * Duplicate Design.
     *
     * This function performs a `POST` to the `/designs/{id}` endpoint.
     *
     * **This endpoint allows you to duplicate one of your existing designs**.
     *
     * Modifying an existing design is often the easiest way to create something new.
     *
     * You are not required to pass any data in the body of a request to this endpoint. If you choose to leave the `name` field blank, your duplicate will be assigned the name of the design it was copied from with the text "Duplicate: " prepended to it. This name change is only a convenience, as the duplicate will be assigned a unique ID that differentiates it from your other designs.
     *
     * You can modify your duplicate’s name at the time of creation by passing an updated value to the `name` field when making the initial request.
     * More on retrieving design IDs can be found below.
     */
    pub async fn post_duplicate_design(
        &self,
        id: &str,
        body: &crate::types::DesignDuplicateInput,
    ) -> ClientResult<crate::types::DesignOutputAllOf> {
        let url = self.client.url(
            &format!("/designs/{}", crate::progenitor_support::encode_path(id),),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete Design.
     *
     * This function performs a `DELETE` to the `/designs/{id}` endpoint.
     *
     * **This endpoint allows you to delete a single design**.
     *
     * Be sure to check the ID of the design you intend to delete before making this request; deleting a design is a permanent action.
     */
    pub async fn delete_design(&self, id: &str) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!("/designs/{}", crate::progenitor_support::encode_path(id),),
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
     * Update Design.
     *
     * This function performs a `PATCH` to the `/designs/{id}` endpoint.
     *
     * **This endpoint allows you to edit a design**.
     *
     * The Design API supports PATCH requests, which allow you to make partial updates to a single design. Passing data to a specific field will update only the data stored in that field; all other fields will be unaltered.
     *
     * For example, updating a design's name requires that you make a PATCH request to this endpoint with data specified for the `name` field only.
     *
     * ```
     * {
     *     "name": "<Updated Name>"
     * }
     * ```
     */
    pub async fn put_design(
        &self,
        id: &str,
        body: &crate::types::PutDesignRequest,
    ) -> ClientResult<crate::types::DesignOutputAllOf> {
        let url = self.client.url(
            &format!("/designs/{}", crate::progenitor_support::encode_path(id),),
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
     * List Designs.
     *
     * This function performs a `GET` to the `/designs` endpoint.
     *
     * **This endpoint allows you to retrieve a list of designs already stored in your Design Library**.
     *
     * A GET request to `/designs` will return a list of your existing designs. This endpoint will not return the pre-built Twilio SendGrid designs. Pre-built designs can be retrieved using the `/designs/pre-builts` endpoint, which is detailed below.
     *
     * By default, you will receive 100 results per request; however, you can modify the number of results returned by passing an integer to the `page_size` query parameter.
     *
     * **Parameters:**
     *
     * * `page_size: u64` -- number of results to return.
     * * `page_token: &str` -- token corresponding to a specific page of results, as provided by metadata.
     * * `summary: bool` -- Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    pub async fn list_designs(
        &self,
        page_size: u64,
        page_token: &str,
        summary: bool,
    ) -> ClientResult<crate::types::ListDesignsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if summary {
            query_args.push(("summary".to_string(), summary.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/designs?{}", query_), None);
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
     * Create Design.
     *
     * This function performs a `POST` to the `/designs` endpoint.
     *
     * **This endpoint allows you to create a new design**.
     *
     * You can add a new design by passing data, including a string of HTML email content, to `/designs`. When creating designs from scratch, be aware of the styling constraints inherent to many email clients. For a list of best practices, see our guide to [Cross-Platform Email Design](https://sendgrid.com/docs/ui/sending-email/cross-platform-html-design/).
     *
     * The Design Library can also convert your design’s HTML elements into drag and drop modules that are editable in the Designs Library user interface. For more, visit the [Design and Code Editor documentation](https://sendgrid.com/docs/ui/sending-email/editor/#drag--drop-markup).
     *
     * Because the `/designs` endpoint makes it easy to add designs, you can create a design with your preferred tooling or migrate designs you already own without relying on the Design Library UI.
     */
    pub async fn post_design(
        &self,
        body: &crate::types::DesignInputAllOf,
    ) -> ClientResult<crate::types::DesignOutputAllOf> {
        let url = self.client.url("/designs", None);
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
     * Get SendGrid Pre-built Design.
     *
     * This function performs a `GET` to the `/designs/pre-builts/{id}` endpoint.
     *
     * **This endpoint allows you to retrieve a single pre-built design**.
     *
     * A GET request to `/designs/pre-builts/{id}` will retrieve details about a specific pre-built design.
     *
     * This endpoint is valuable when retrieving details about a pre-built design that you wish to duplicate and modify.
     */
    pub async fn get_sendgrid_pre_built_design(
        &self,
        id: &str,
    ) -> ClientResult<crate::types::DesignOutputAllOf> {
        let url = self.client.url(
            &format!(
                "/designs/pre-builts/{}",
                crate::progenitor_support::encode_path(id),
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
     * Duplicate SendGrid Pre-built Design.
     *
     * This function performs a `POST` to the `/designs/pre-builts/{id}` endpoint.
     *
     * **This endpoint allows you to duplicate one of the pre-built Twilio SendGrid designs**.
     *
     * Like duplicating one of your existing designs, you are not required to pass any data in the body of a request to this endpoint. If you choose to leave the `name` field blank, your duplicate will be assigned the name of the design it was copied from with the text "Duplicate: " prepended to it. This name change is only a convenience, as the duplicate design will be assigned a unique ID that differentiates it from your other designs. You can retrieve the IDs for Twilio SendGrid pre-built designs using the "List SendGrid Pre-built Designs" endpoint.
     *
     * You can modify your duplicate’s name at the time of creation by passing an updated value to the `name` field when making the initial request.
     * More on retrieving design IDs can be found above.
     */
    pub async fn post_sendgrid_pre_built_design(
        &self,
        id: &str,
        body: &crate::types::DesignDuplicateInput,
    ) -> ClientResult<crate::types::DesignOutputAllOf> {
        let url = self.client.url(
            &format!(
                "/designs/pre-builts/{}",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List SendGrid Pre-built Designs.
     *
     * This function performs a `GET` to the `/designs/pre-builts` endpoint.
     *
     * **This endpoint allows you to retrieve a list of pre-built designs provided by Twilio SendGrid**.
     *
     * Unlike the `/designs` endpoint where *your* designs are stored, a GET request made to `designs/pre-builts` will retrieve a list of the pre-built Twilio SendGrid designs. This endpoint will not return the designs stored in your Design Library.
     *
     * By default, you will receive 100 results per request; however, you can modify the number of results returned by passing an integer to the `page_size` query parameter.
     *
     * This endpoint is useful for retrieving the IDs of Twilio SendGrid designs that you want to duplicate and modify.
     *
     * **Parameters:**
     *
     * * `page_size: u64` -- number of results to return.
     * * `page_token: &str` -- token corresponding to a specific page of results, as provided by metadata.
     * * `summary: bool` -- Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     */
    pub async fn list_sendgrid_pre_built_designs(
        &self,
        page_size: u64,
        page_token: &str,
        summary: bool,
    ) -> ClientResult<crate::types::ListDesignsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if summary {
            query_args.push(("summary".to_string(), summary.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/designs/pre-builts?{}", query_), None);
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

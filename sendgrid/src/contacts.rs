use crate::Client;
use crate::ClientResult;

pub struct Contacts {
    pub client: Client,
}

impl Contacts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Contacts { client }
    }

    /**
     * Get Sample Contacts.
     *
     * This function performs a `GET` to the `/marketing/contacts` endpoint.
     *
     * **This endpoint will return up to 50 of the most recent contacts uploaded or attached to a list**.
     *
     * This list will then be sorted by email address.
     *
     * The full contact count is also returned.
     *
     * Please note that pagination of the contacts has been deprecated.
     */
    pub async fn get_mc_contats(&self) -> ClientResult<crate::types::GetMcContatsResponse> {
        let url = self.client.url("/marketing/contacts", None);
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
     * Add or Update a Contact.
     *
     * This function performs a `PUT` to the `/marketing/contacts` endpoint.
     *
     * **This endpoint allows the [upsert](https://en.wiktionary.org/wiki/upsert) (insert or update) of up to 30,000 contacts, or 6MB of data, whichever is lower**.
     *
     * Because the creation and update of contacts is an asynchronous process, the response will not contain immediate feedback on the processing of your upserted contacts. Rather, it will contain an HTTP 202 response indicating the contacts are queued for processing or an HTTP 4XX error containing validation errors. Should you wish to get the resulting contact's ID or confirm your contacts have been updated or added, you can use the "Get Contacts by Emails" endpoint.
     *
     * Please note that custom fields need to have been already created if you wish to set their values for the contacts being upserted. To do this, please use the "Create Custom Field Definition" endpoint.
     *
     * You will see a `job_id` in the response to your request. This can be used to check the status of your upsert job. To do so, please use the "Import Contacts Status" endpoint.
     *
     * If the contact already exists in the system, any entries submitted via this endpoint will update the existing contact. The contact to update will be determined only by the `email` field and any fields omitted from the request will remain as they were. A contact's ID cannot be used to update the contact.
     *
     * The email field will be changed to all lower-case. If a contact is added with an email that exists but contains capital letters, the existing contact with the all lower-case email will be updated.
     */
    pub async fn put_mc(
        &self,
        body: &crate::types::PutMcContactsRequest,
    ) -> ClientResult<crate::types::PutMcContactsResponse> {
        let url = self.client.url("/marketing/contacts", None);
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
     * Delete Contacts.
     *
     * This function performs a `DELETE` to the `/marketing/contacts` endpoint.
     *
     * **This endpoint can be used to delete one or more contacts**.
     *
     * The query parameter `ids` must set to a comma-separated list of contact IDs for bulk contact deletion.
     *
     * The query parameter `delete_all_contacts` must be set to `"true"` to delete **all** contacts.
     *
     * You must set either `ids` or `delete_all_contacts`.
     *
     * Deletion jobs are processed asynchronously.
     *
     * **Parameters:**
     *
     * * `delete_all_contacts: &str` -- The license key provided with your New Relic account.
     * * `ids: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_mc(
        &self,
        delete_all_contacts: &str,
        ids: &str,
    ) -> ClientResult<crate::types::DeleteMcContactsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !delete_all_contacts.is_empty() {
            query_args.push((
                "delete_all_contacts".to_string(),
                delete_all_contacts.to_string(),
            ));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/marketing/contacts?{}", query_), None);
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
     * Get Total Contact Count.
     *
     * This function performs a `GET` to the `/marketing/contacts/count` endpoint.
     *
     * **This endpoint returns the total number of contacts you have stored.**
     */
    pub async fn get_mc_count(&self) -> ClientResult<crate::types::GetMcContactsCountResponse> {
        let url = self.client.url("/marketing/contacts/count", None);
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
     * Get All Existing Exports.
     *
     * This function performs a `GET` to the `/marketing/contacts/exports` endpoint.
     *
     * **Use this endpoint to retrieve details of all current exported jobs**.
     *
     * It will return an array of objects, each of which records an export job in flight or recently completed.
     *
     * Each object's `export_type` field will tell you which kind of export it is and its `status` field will indicate what stage of processing it has reached. Exports which are `ready` will be accompanied by a `urls` field which lists the URLs of the export's downloadable files — there will be more than one if you specified a maximum file size in your initial export request.
     *
     * Use this endpoint if you have exports in flight but do not know their IDs, which are required for the "Export Contacts Status" endpoint.
     */
    pub async fn get_marketing_exports(
        &self,
    ) -> ClientResult<crate::types::GetMarketingContactsExportsResponse> {
        let url = self.client.url("/marketing/contacts/exports", None);
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
     * Export Contacts.
     *
     * This function performs a `POST` to the `/marketing/contacts/exports` endpoint.
     *
     * **Use this endpoint to export lists or segments of contacts**.
     *
     * If you would just like to have a link to the exported list sent to your email set the `notifications.email` option to `true` in the `POST` payload.
     *
     * If you would like to download the list, take the `id` that is returned and use the "Export Contacts Status" endpoint to get the `urls`. Once you have the list of URLs, make a `GET` request to each URL provided to download your CSV file(s).
     *
     * You specify the segements and or/contact lists you wish to export by providing the relevant IDs in, respectively, the `segment_ids` and `list_ids` fields in the request body.
     *
     * The lists will be provided in either JSON or CSV files. To specify which of these you would required, set the request body `file_type` field to `json` or `csv`.
     *
     * You can also specify a maximum file size (in MB). If the export file is larger than this, it will be split into multiple files.
     */
    pub async fn post_mc_export(
        &self,
        body: &crate::types::PostMcContactsExportsRequest,
    ) -> ClientResult<crate::types::PostMcContactsExportsResponse> {
        let url = self.client.url("/marketing/contacts/exports", None);
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
     * Get a Contact by ID.
     *
     * This function performs a `GET` to the `/marketing/contacts/{id}` endpoint.
     *
     * **This endpoint returns the full details and all fields for the specified contact**.
     *
     * The "Get Contacts by Emails" endpoint can be used to get the ID of a contact.
     */
    pub async fn get_mc(&self, id: &str) -> ClientResult<crate::types::ContactDetails3> {
        let url = self.client.url(
            &format!(
                "/marketing/contacts/{}",
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
     * Search Contacts.
     *
     * This function performs a `POST` to the `/marketing/contacts/search` endpoint.
     *
     * **Use this endpoint to locate contacts**.
     *
     * The request body's `query` field accepts valid [SGQL](https://sendgrid.com/docs/for-developers/sending-email/segmentation-query-language/) for searching for a contact.
     *
     * Because contact emails are stored in lower case, using SGQL to search by email address requires the provided email address to be in lower case. The SGQL `lower()` function can be used for this.
     *
     * Only the first 50 contacts that meet the search criteria will be returned.
     *
     * If the query takes longer than 20 seconds, a `408 Request Timeout` status will be returned.
     *
     * Formatting the `created_at` and `updated_at` values as Unix timestamps is deprecated. Instead they are returned as ISO format as string.
     */
    pub async fn post_mc_search(
        &self,
        body: &crate::types::PostMcContactsSearchRequest,
    ) -> ClientResult<crate::types::PostMcContactsSearchResponse> {
        let url = self.client.url("/marketing/contacts/search", None);
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
     * Import Contacts.
     *
     * This function performs a `PUT` to the `/marketing/contacts/imports` endpoint.
     *
     * **This endpoint allows a CSV upload containing up to one million contacts or 5GB of data, whichever is smaller.**
     *
     * Imports take place asynchronously: the endpoint returns a URL (`upload_uri`) and HTTP headers (`upload_headers`) which can subsequently be used to `PUT` a file of contacts to be  imported into our system.
     *
     * Uploaded CSV files may also be [gzip-compressed](https://en.wikipedia.org/wiki/Gzip).
     *
     * In either case, you must include the field `file_type` with the value `csv` in your request body.
     *
     * The `field_mappings` paramter is a respective list of field definition IDs to map the uploaded CSV columns to. It allows you to use CSVs where one or more columns are skipped (`null`) or remapped to the contact field.
     *
     * For example, if `field_mappings` is set to `[null, "w1", "_rf1"]`, this means skip column 0, map column 1 to the custom field with the ID `w1`, and map column 2 to the reserved field with the ID `_rf1`. See the "Get All Field Definitions" endpoint to fetch your custom and reserved field IDs to use with `field_mappings`.
     *
     * Once you recieve the response body you can then initiate a **second** API call where you use the supplied URL and HTTP header to upload your file. For example:
     *
     * `curl --upload-file "file/path.csv" "URL_GIVEN" -H 'HEADER_GIVEN'`
     *
     * If you'd like to monitor the status of your import job, use the `job_id` and the "Import Contacts Status" endpoint.
     */
    pub async fn put_mc_imports(
        &self,
        body: &crate::types::PutMcContactsImportsRequest,
    ) -> ClientResult<crate::types::PutMcContactsImportsResponse> {
        let url = self.client.url("/marketing/contacts/imports", None);
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
     * Import Contacts Status.
     *
     * This function performs a `GET` to the `/marketing/contacts/imports/{id}` endpoint.
     *
     * **This endpoint can be used to check the status of a contact import job**.
     *
     * Use the `job_id` from the "Improt Contacts," "Add or Update a Contact," or "Delete Contacts" endpoints as the `id` in the path parameter.
     *
     * If there is an error with your `PUT` request, download the `errors_url` file and open it to view more details.
     *
     * The job `status` field indicates whether the job is `pending`, `completed`, `errored`, or `failed`.
     *
     * Pending means not started. Completed means finished without any errors. Errored means finished with some errors. Failed means finshed with all errors, or the job was entirely unprocessable: for example, if you attempt to import file format we do not support.
     *
     * The `results` object will have fields depending on the job type.
     */
    pub async fn get_marketing_import(
        &self,
        id: &str,
    ) -> ClientResult<crate::types::ContactImport> {
        let url = self.client.url(
            &format!(
                "/marketing/contacts/imports/{}",
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
     * Export Contacts Status.
     *
     * This function performs a `GET` to the `/marketing/contacts/exports/{id}` endpoint.
     *
     * **This endpoint can be used to check the status of a contact export job**.
     *
     * To use this call, you will need the `id` from the "Export Contacts" call.
     *
     * If you would like to download a list, take the `id` that is returned from the "Export Contacts" endpoint and make an API request here to get the `urls`. Once you have the list of URLs, make a `GET` request on each URL to download your CSV file(s).
     */
    pub async fn get_mc_export(&self, id: &str) -> ClientResult<crate::types::ContactExport> {
        let url = self.client.url(
            &format!(
                "/marketing/contacts/exports/{}",
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
     * Get Batched Contacts by IDs.
     *
     * This function performs a `POST` to the `/marketing/contacts/batch` endpoint.
     *
     * **This endpoint is used to retrieve a set of contacts identified by their IDs.**
     *
     * This can be more efficient endpoint to get contacts than making a series of individual `GET` requests to the "Get a Contact by ID" endpoint.
     *
     * You can supply up to 100 IDs. Pass them into the `ids` field in your request body as an array or one or more strings.
     */
    pub async fn post_marketing_batch(
        &self,
        body: &crate::types::PostMarketingContactsBatchRequest,
    ) -> ClientResult<crate::types::PostMarketingContactsBatchResponse> {
        let url = self.client.url("/marketing/contacts/batch", None);
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
     * Get Contacts by Emails.
     *
     * This function performs a `POST` to the `/marketing/contacts/search/emails` endpoint.
     *
     * **This endpoint allows you to retrieve up to 100 contacts matching the searched `email` address(es), including any `alternate_emails`.**
     *
     * Email addresses are unique to a contact, meaning this endpoint can treat an email address as a primary key to search by. The contact object associated with the address, whether it is their `email` or one of their `alternate_emails` will be returned if matched.
     *
     * Email addresses in the search request do not need to match the case in which they're stored, but the email addresses in the result will be all lower case. Empty strings are excluded from the search and will not be returned.
     *
     * This endpoint should be used in place of the "Search Contacts" endpoint when you can provide exact email addresses and do not need to include other [Segmentation Query Language (SGQL)](https://sendgrid.com/docs/for-developers/sending-email/segmentation-query-language/) filters when searching.
     *
     * If you need to access a large percentage of your contacts, we recommend exporting your contacts with the "Export Contacts" endpoint and filtering the results client side.
     *
     * This endpoint returns a `200` status code when any contacts match the address(es) you supplied. When searching multiple addresses in a single request, it is possible that some addresses will match a contact while others will not. When a partially successful search like this is made, the matching contacts are returned in an object and an error message is returned for the email address(es) that are not found.
     *
     * This endpoint returns a `404` status code when no contacts are found for the provided email address(es).
     *
     * A `400` status code is returned if any searched addresses are invalid.
     */
    pub async fn post_marketing_search_email(
        &self,
        body: &crate::types::PostMarketingContactsSearchEmailsRequest,
    ) -> ClientResult<crate::types::PostMarketingContactsSearchEmailsResponse> {
        let url = self.client.url("/marketing/contacts/search/emails", None);
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
}

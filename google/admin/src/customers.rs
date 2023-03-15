use crate::Client;
use crate::ClientResult;

pub struct Customers {
    pub client: Client,
}

impl Customers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Customers { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customers/{customerKey}` endpoint.
     *
     * Retrieves a customer.
     *
     * **Parameters:**
     *
     * * `customer_key: &str` -- Id of the customer to be retrieved.
     */
    pub async fn get(&self, customer_key: &str) -> ClientResult<crate::types::Customer> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customers/{}",
                crate::progenitor_support::encode_path(customer_key),
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
     * This function performs a `PUT` to the `/admin/directory/v1/customers/{customerKey}` endpoint.
     *
     * Updates a customer.
     *
     * **Parameters:**
     *
     * * `customer_key: &str` -- Id of the customer to be updated.
     */
    pub async fn update(
        &self,
        customer_key: &str,
        body: &crate::types::Customer,
    ) -> ClientResult<crate::types::Customer> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customers/{}",
                crate::progenitor_support::encode_path(customer_key),
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
    /**
     * This function performs a `PATCH` to the `/admin/directory/v1/customers/{customerKey}` endpoint.
     *
     * Patches a customer.
     *
     * **Parameters:**
     *
     * * `customer_key: &str` -- Id of the customer to be updated.
     */
    pub async fn patch(
        &self,
        customer_key: &str,
        body: &crate::types::Customer,
    ) -> ClientResult<crate::types::Customer> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customers/{}",
                crate::progenitor_support::encode_path(customer_key),
            ),
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
     * This function performs a `GET` to the `/admin/directory/v1/{name}` endpoint.
     *
     * Returns a `Printer` resource (printer's config).
     *
     * **Parameters:**
     *
     * * `name: &str` -- Required. The name of the printer to retrieve. Format: customers/{customer_id}/chrome/printers/{printer_id}.
     */
    pub async fn admin_chrome_printers_get(
        &self,
        name: &str,
    ) -> ClientResult<crate::types::Printer> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}",
                crate::progenitor_support::encode_path(name),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/{name}` endpoint.
     *
     * Deletes a `Printer`.
     *
     * **Parameters:**
     *
     * * `name: &str` -- Required. The name of the printer to be updated. Format: customers/{customer_id}/chrome/printers/{printer_id}.
     */
    pub async fn admin_chrome_printers_delete(
        &self,
        name: &str,
    ) -> ClientResult<crate::types::Empty> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}",
                crate::progenitor_support::encode_path(name),
            ),
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
     * This function performs a `PATCH` to the `/admin/directory/v1/{name}` endpoint.
     *
     * Updates a `Printer` resource.
     *
     * **Parameters:**
     *
     * * `name: &str` -- The resource name of the Printer object, in the format customers/{customer-id}/printers/{printer-id} (During printer creation leave empty).
     * * `clear_mask: &str` -- The list of fields to be cleared. Note, some of the fields are read only and cannot be updated. Values for not specified fields will be patched.
     * * `update_mask: &str` -- The list of fields to be updated. Note, some of the fields are read only and cannot be updated. Values for not specified fields will be patched.
     */
    pub async fn admin_chrome_printers_patch(
        &self,
        name: &str,
        clear_mask: &str,
        update_mask: &str,
        body: &crate::types::Printer,
    ) -> ClientResult<crate::types::Printer> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !clear_mask.is_empty() {
            query_args.push(("clearMask".to_string(), clear_mask.to_string()));
        }
        if !update_mask.is_empty() {
            query_args.push(("updateMask".to_string(), update_mask.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}?{}",
                crate::progenitor_support::encode_path(name),
                query_
            ),
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
     * This function performs a `GET` to the `/admin/directory/v1/{parent}/chrome/printers` endpoint.
     *
     * List printers configs.
     *
     * **Parameters:**
     *
     * * `parent: &str` -- Required. The name of the customer who owns this collection of printers. Format: customers/{customer_id}.
     * * `filter: &str` -- Search query. Search syntax is shared between this api and Admin Console printers pages.
     * * `org_unit_id: &str` -- Organization Unit that we want to list the printers for. When org_unit is not present in the request then all printers of the customer are returned (or filtered). When org_unit is present in the request then only printers available to this OU will be returned (owned or inherited). You may see if printer is owned or inherited for this OU by looking at Printer.org_unit_id.
     * * `page_size: i64` -- The maximum number of objects to return. The service may return fewer than this value.
     * * `page_token: &str` -- A page token, received from a previous call.
     */
    pub async fn admin_chrome_printers_list(
        &self,
        parent: &str,
        filter: &str,
        org_unit_id: &str,
        page_size: i64,
        page_token: &str,
    ) -> ClientResult<Vec<crate::types::Printer>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !org_unit_id.is_empty() {
            query_args.push(("orgUnitId".to_string(), org_unit_id.to_string()));
        }
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}/chrome/printers?{}",
                crate::progenitor_support::encode_path(parent),
                query_
            ),
            None,
        );
        let resp: crate::types::ListPrintersResponse = self
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
        Ok(resp.printers.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/{parent}/chrome/printers` endpoint.
     *
     * As opposed to `admin_chrome_printers_list`, this function returns all the pages of the request at once.
     *
     * List printers configs.
     */
    pub async fn admin_chrome_printers_list_all(
        &self,
        parent: &str,
        filter: &str,
        org_unit_id: &str,
    ) -> ClientResult<Vec<crate::types::Printer>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !org_unit_id.is_empty() {
            query_args.push(("orgUnitId".to_string(), org_unit_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}/chrome/printers?{}",
                crate::progenitor_support::encode_path(parent),
                query_
            ),
            None,
        );
        let mut resp: crate::types::ListPrintersResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut printers = resp.printers;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?pageToken={}", url, page),
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
                        &format!("{}&pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            printers.append(&mut resp.printers);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(printers)
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/{parent}/chrome/printers` endpoint.
     *
     * Creates a printer under given Organization Unit.
     *
     * **Parameters:**
     *
     * * `parent: &str` -- Required. The name of the customer. Format: customers/{customer_id}.
     */
    pub async fn admin_chrome_printers_create(
        &self,
        parent: &str,
        body: &crate::types::Printer,
    ) -> ClientResult<crate::types::Printer> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}/chrome/printers",
                crate::progenitor_support::encode_path(parent),
            ),
            None,
        );
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
     * This function performs a `POST` to the `/admin/directory/v1/{parent}/chrome/printers:batchCreatePrinters` endpoint.
     *
     * Creates printers under given Organization Unit.
     *
     * **Parameters:**
     *
     * * `parent: &str` -- Required. The name of the customer. Format: customers/{customer_id}.
     */
    pub async fn admin_chrome_printers_batch_create(
        &self,
        parent: &str,
        body: &crate::types::BatchCreatePrintersRequest,
    ) -> ClientResult<crate::types::BatchCreatePrintersResponse> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}/chrome/printers:batchCreatePrinters",
                crate::progenitor_support::encode_path(parent),
            ),
            None,
        );
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
     * This function performs a `POST` to the `/admin/directory/v1/{parent}/chrome/printers:batchDeletePrinters` endpoint.
     *
     * Deletes printers in batch.
     *
     * **Parameters:**
     *
     * * `parent: &str` -- Required. The name of the customer. Format: customers/{customer_id}.
     */
    pub async fn admin_chrome_printers_batch_delete(
        &self,
        parent: &str,
        body: &crate::types::BatchDeletePrintersRequest,
    ) -> ClientResult<crate::types::BatchDeletePrintersResponse> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}/chrome/printers:batchDeletePrinters",
                crate::progenitor_support::encode_path(parent),
            ),
            None,
        );
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
     * This function performs a `GET` to the `/admin/directory/v1/{parent}/chrome/printers:listPrinterModels` endpoint.
     *
     * Lists the supported printer models.
     *
     * **Parameters:**
     *
     * * `parent: &str` -- Required. The name of the customer who owns this collection of printers. Format: customers/{customer_id}.
     * * `filter: &str` -- Filer to list only models by a given manufacturer in format: "manufacturer:Brother". Search syntax is shared between this api and Admin Console printers pages.
     * * `page_size: i64` -- The maximum number of objects to return. The service may return fewer than this value.
     * * `page_token: &str` -- A page token, received from a previous call.
     */
    pub async fn admin_chrome_printers_list_printer_models(
        &self,
        parent: &str,
        filter: &str,
        page_size: i64,
        page_token: &str,
    ) -> ClientResult<Vec<crate::types::PrinterModel>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}/chrome/printers:listPrinterModels?{}",
                crate::progenitor_support::encode_path(parent),
                query_
            ),
            None,
        );
        let resp: crate::types::ListPrinterModelsResponse = self
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
        Ok(resp.printer_models.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/{parent}/chrome/printers:listPrinterModels` endpoint.
     *
     * As opposed to `admin_chrome_printers_list_printer_models`, this function returns all the pages of the request at once.
     *
     * Lists the supported printer models.
     */
    pub async fn admin_chrome_printers_list_all_printer_models(
        &self,
        parent: &str,
        filter: &str,
    ) -> ClientResult<Vec<crate::types::PrinterModel>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/{}/chrome/printers:listPrinterModels?{}",
                crate::progenitor_support::encode_path(parent),
                query_
            ),
            None,
        );
        let mut resp: crate::types::ListPrinterModelsResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut printer_models = resp.printer_models;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?pageToken={}", url, page),
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
                        &format!("{}&pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            printer_models.append(&mut resp.printer_models);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(printer_models)
    }
}

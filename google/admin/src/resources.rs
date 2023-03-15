use crate::Client;
use crate::ClientResult;

pub struct Resources {
    pub client: Client,
}

impl Resources {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Resources { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/resources/buildings` endpoint.
     *
     * Retrieves a list of buildings for an account.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `max_results: i64` -- Maximum number of results to return.
     * * `page_token: &str` -- Token to specify the next page in the list.
     */
    pub async fn buildings_list(
        &self,
        customer: &str,
        max_results: i64,
        page_token: &str,
    ) -> ClientResult<Vec<crate::types::Building>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/buildings?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::Buildings = self
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
        Ok(resp.buildings.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/resources/buildings` endpoint.
     *
     * As opposed to `buildings_list`, this function returns all the pages of the request at once.
     *
     * Retrieves a list of buildings for an account.
     */
    pub async fn buildings_list_all(
        &self,
        customer: &str,
    ) -> ClientResult<Vec<crate::types::Building>> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/buildings",
                crate::progenitor_support::encode_path(customer),
            ),
            None,
        );
        let mut resp: crate::types::Buildings = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut buildings = resp.buildings;
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

            buildings.append(&mut resp.buildings);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(buildings)
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/resources/buildings` endpoint.
     *
     * Inserts a building.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `coordinates_source: crate::types::CoordinatesSource` -- Source from which Building.coordinates are derived.
     */
    pub async fn buildings_insert(
        &self,
        customer: &str,
        coordinates_source: crate::types::CoordinatesSource,
        body: &crate::types::Building,
    ) -> ClientResult<crate::types::Building> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !coordinates_source.to_string().is_empty() {
            query_args.push((
                "coordinatesSource".to_string(),
                coordinates_source.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/buildings?{}",
                crate::progenitor_support::encode_path(customer),
                query_
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
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/resources/buildings/{buildingId}` endpoint.
     *
     * Retrieves a building.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `building_id: &str` -- The unique ID of the building to retrieve.
     */
    pub async fn buildings_get(
        &self,
        customer: &str,
        building_id: &str,
    ) -> ClientResult<crate::types::Building> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/buildings/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(building_id),
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
     * This function performs a `PUT` to the `/admin/directory/v1/customer/{customer}/resources/buildings/{buildingId}` endpoint.
     *
     * Updates a building.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `building_id: &str` -- The id of the building to update.
     * * `coordinates_source: crate::types::CoordinatesSource` -- Source from which Building.coordinates are derived.
     */
    pub async fn buildings_update(
        &self,
        customer: &str,
        building_id: &str,
        coordinates_source: crate::types::CoordinatesSource,
        body: &crate::types::Building,
    ) -> ClientResult<crate::types::Building> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !coordinates_source.to_string().is_empty() {
            query_args.push((
                "coordinatesSource".to_string(),
                coordinates_source.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/buildings/{}?{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(building_id),
                query_
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
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/resources/buildings/{buildingId}` endpoint.
     *
     * Deletes a building.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `building_id: &str` -- The id of the building to delete.
     */
    pub async fn buildings_delete(&self, customer: &str, building_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/buildings/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(building_id),
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
     * This function performs a `PATCH` to the `/admin/directory/v1/customer/{customer}/resources/buildings/{buildingId}` endpoint.
     *
     * Patches a building.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `building_id: &str` -- The id of the building to update.
     * * `coordinates_source: crate::types::CoordinatesSource` -- Source from which Building.coordinates are derived.
     */
    pub async fn buildings_patch(
        &self,
        customer: &str,
        building_id: &str,
        coordinates_source: crate::types::CoordinatesSource,
        body: &crate::types::Building,
    ) -> ClientResult<crate::types::Building> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !coordinates_source.to_string().is_empty() {
            query_args.push((
                "coordinatesSource".to_string(),
                coordinates_source.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/buildings/{}?{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(building_id),
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
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/resources/calendars` endpoint.
     *
     * Retrieves a list of calendar resources for an account.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `max_results: i64` -- Maximum number of results to return.
     * * `order_by: &str` -- Field(s) to sort results by in either ascending or descending order. Supported fields include `resourceId`, `resourceName`, `capacity`, `buildingId`, and `floorName`. If no order is specified, defaults to ascending. Should be of the form "field [asc|desc], field [asc|desc], ...". For example `buildingId, capacity desc` would return results sorted first by `buildingId` in ascending order then by `capacity` in descending order.
     * * `page_token: &str` -- Token to specify the next page in the list.
     * * `query: &str` -- String query used to filter results. Should be of the form "field operator value" where field can be any of supported fields and operators can be any of supported operations. Operators include '=' for exact match, '!=' for mismatch and ':' for prefix match or HAS match where applicable. For prefix match, the value should always be followed by a *. Logical operators NOT and AND are supported (in this order of precedence). Supported fields include `generatedResourceName`, `name`, `buildingId`, `floor_name`, `capacity`, `featureInstances.feature.name`, `resourceEmail`, `resourceCategory`. For example `buildingId=US-NYC-9TH AND featureInstances.feature.name:Phone`.
     */
    pub async fn calendars_list(
        &self,
        customer: &str,
        max_results: i64,
        order_by: &str,
        page_token: &str,
        query: &str,
    ) -> ClientResult<Vec<crate::types::CalendarResource>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !order_by.is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/calendars?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::CalendarResources = self
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
        Ok(resp.items.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/resources/calendars` endpoint.
     *
     * As opposed to `calendars_list`, this function returns all the pages of the request at once.
     *
     * Retrieves a list of calendar resources for an account.
     */
    pub async fn calendars_list_all(
        &self,
        customer: &str,
        order_by: &str,
        query: &str,
    ) -> ClientResult<Vec<crate::types::CalendarResource>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order_by.is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/calendars?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let mut resp: crate::types::CalendarResources = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut items = resp.items;
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

            items.append(&mut resp.items);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(items)
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/resources/calendars` endpoint.
     *
     * Inserts a calendar resource.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     */
    pub async fn calendars_insert(
        &self,
        customer: &str,
        body: &crate::types::CalendarResource,
    ) -> ClientResult<crate::types::CalendarResource> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/calendars",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/resources/calendars/{calendarResourceId}` endpoint.
     *
     * Retrieves a calendar resource.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `calendar_resource_id: &str` -- The unique ID of the calendar resource to retrieve.
     */
    pub async fn calendars_get(
        &self,
        customer: &str,
        calendar_resource_id: &str,
    ) -> ClientResult<crate::types::CalendarResource> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/calendars/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(calendar_resource_id),
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
     * This function performs a `PUT` to the `/admin/directory/v1/customer/{customer}/resources/calendars/{calendarResourceId}` endpoint.
     *
     * Updates a calendar resource. This method supports patch semantics, meaning you only need to include the fields you wish to update. Fields that are not present in the request will be preserved.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `calendar_resource_id: &str` -- The unique ID of the calendar resource to update.
     */
    pub async fn calendars_update(
        &self,
        customer: &str,
        calendar_resource_id: &str,
        body: &crate::types::CalendarResource,
    ) -> ClientResult<crate::types::CalendarResource> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/calendars/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(calendar_resource_id),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/resources/calendars/{calendarResourceId}` endpoint.
     *
     * Deletes a calendar resource.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `calendar_resource_id: &str` -- The unique ID of the calendar resource to delete.
     */
    pub async fn calendars_delete(
        &self,
        customer: &str,
        calendar_resource_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/calendars/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(calendar_resource_id),
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
     * This function performs a `PATCH` to the `/admin/directory/v1/customer/{customer}/resources/calendars/{calendarResourceId}` endpoint.
     *
     * Patches a calendar resource.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `calendar_resource_id: &str` -- The unique ID of the calendar resource to update.
     */
    pub async fn calendars_patch(
        &self,
        customer: &str,
        calendar_resource_id: &str,
        body: &crate::types::CalendarResource,
    ) -> ClientResult<crate::types::CalendarResource> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/calendars/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(calendar_resource_id),
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
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/resources/features` endpoint.
     *
     * Retrieves a list of features for an account.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `max_results: i64` -- Maximum number of results to return.
     * * `page_token: &str` -- Token to specify the next page in the list.
     */
    pub async fn features_list(
        &self,
        customer: &str,
        max_results: i64,
        page_token: &str,
    ) -> ClientResult<Vec<crate::types::Feature>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/features?{}",
                crate::progenitor_support::encode_path(customer),
                query_
            ),
            None,
        );
        let resp: crate::types::Features = self
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
        Ok(resp.features.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/resources/features` endpoint.
     *
     * As opposed to `features_list`, this function returns all the pages of the request at once.
     *
     * Retrieves a list of features for an account.
     */
    pub async fn features_list_all(
        &self,
        customer: &str,
    ) -> ClientResult<Vec<crate::types::Feature>> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/features",
                crate::progenitor_support::encode_path(customer),
            ),
            None,
        );
        let mut resp: crate::types::Features = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut features = resp.features;
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

            features.append(&mut resp.features);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(features)
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/resources/features` endpoint.
     *
     * Inserts a feature.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     */
    pub async fn features_insert(
        &self,
        customer: &str,
        body: &crate::types::Feature,
    ) -> ClientResult<crate::types::Feature> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/features",
                crate::progenitor_support::encode_path(customer),
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
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/resources/features/{featureKey}` endpoint.
     *
     * Retrieves a feature.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `feature_key: &str` -- The unique ID of the feature to retrieve.
     */
    pub async fn features_get(
        &self,
        customer: &str,
        feature_key: &str,
    ) -> ClientResult<crate::types::Feature> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/features/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(feature_key),
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
     * This function performs a `PUT` to the `/admin/directory/v1/customer/{customer}/resources/features/{featureKey}` endpoint.
     *
     * Updates a feature.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `feature_key: &str` -- The unique ID of the feature to update.
     */
    pub async fn features_update(
        &self,
        customer: &str,
        feature_key: &str,
        body: &crate::types::Feature,
    ) -> ClientResult<crate::types::Feature> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/features/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(feature_key),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/resources/features/{featureKey}` endpoint.
     *
     * Deletes a feature.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `feature_key: &str` -- The unique ID of the feature to delete.
     */
    pub async fn features_delete(&self, customer: &str, feature_key: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/features/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(feature_key),
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
     * This function performs a `PATCH` to the `/admin/directory/v1/customer/{customer}/resources/features/{featureKey}` endpoint.
     *
     * Patches a feature.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `feature_key: &str` -- The unique ID of the feature to update.
     */
    pub async fn features_patch(
        &self,
        customer: &str,
        feature_key: &str,
        body: &crate::types::Feature,
    ) -> ClientResult<crate::types::Feature> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/features/{}",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(feature_key),
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
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/resources/features/{oldName}/rename` endpoint.
     *
     * Renames a feature.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
     * * `old_name: &str` -- The unique ID of the feature to rename.
     */
    pub async fn features_rename(
        &self,
        customer: &str,
        old_name: &str,
        body: &crate::types::FeatureRename,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/resources/features/{}/rename",
                crate::progenitor_support::encode_path(customer),
                crate::progenitor_support::encode_path(old_name),
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
}

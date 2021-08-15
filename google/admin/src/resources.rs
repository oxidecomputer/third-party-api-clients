use anyhow::Result;

use crate::Client;

pub struct Resources {
    client: Client,
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
    pub async fn directory_buildings_list(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        max_results: i64,
        page_token: &str,
    ) -> Result<crate::types::Buildings> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if max_results > 0 {
            query_args.push(format!("max_results={}", max_results));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/buildings?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
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
    pub async fn directory_buildings_insert(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        coordinates_source: crate::types::CoordinatesSource,
        body: &crate::types::Building,
    ) -> Result<crate::types::Building> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        query_args.push(format!("coordinates_source={}", coordinates_source));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/buildings?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_buildings_get(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        building_id: &str,
    ) -> Result<crate::types::Building> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/buildings/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&building_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
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
    pub async fn directory_buildings_update(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        building_id: &str,
        coordinates_source: crate::types::CoordinatesSource,
        body: &crate::types::Building,
    ) -> Result<crate::types::Building> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        query_args.push(format!("coordinates_source={}", coordinates_source));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/buildings/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&building_id.to_string()),
            query_
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_buildings_delete(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        building_id: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/buildings/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&building_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
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
    pub async fn directory_buildings_patch(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        building_id: &str,
        coordinates_source: crate::types::CoordinatesSource,
        body: &crate::types::Building,
    ) -> Result<crate::types::Building> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        query_args.push(format!("coordinates_source={}", coordinates_source));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/buildings/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&building_id.to_string()),
            query_
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_calendars_list(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        max_results: i64,
        order_by: &str,
        page_token: &str,
        query: &str,
    ) -> Result<crate::types::CalendarResources> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if max_results > 0 {
            query_args.push(format!("max_results={}", max_results));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if !order_by.is_empty() {
            query_args.push(format!("order_by={}", order_by));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !query.is_empty() {
            query_args.push(format!("query={}", query));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/calendars?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
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
    pub async fn directory_calendars_insert(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        body: &crate::types::CalendarResource,
    ) -> Result<crate::types::CalendarResource> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/calendars?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_calendars_get(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        calendar_resource_id: &str,
    ) -> Result<crate::types::CalendarResource> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/calendars/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&calendar_resource_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
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
    pub async fn directory_calendars_update(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        calendar_resource_id: &str,
        body: &crate::types::CalendarResource,
    ) -> Result<crate::types::CalendarResource> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/calendars/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&calendar_resource_id.to_string()),
            query_
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_calendars_delete(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        calendar_resource_id: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/calendars/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&calendar_resource_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
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
    pub async fn directory_calendars_patch(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        calendar_resource_id: &str,
        body: &crate::types::CalendarResource,
    ) -> Result<crate::types::CalendarResource> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/calendars/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&calendar_resource_id.to_string()),
            query_
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_features_list(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        max_results: i64,
        page_token: &str,
    ) -> Result<crate::types::Features> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if max_results > 0 {
            query_args.push(format!("max_results={}", max_results));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/features?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
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
    pub async fn directory_features_insert(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        body: &crate::types::Feature,
    ) -> Result<crate::types::Feature> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/features?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_features_get(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        feature_key: &str,
    ) -> Result<crate::types::Feature> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/features/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&feature_key.to_string()),
            query_
        );

        self.client.get(&url, None).await
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
    pub async fn directory_features_update(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        feature_key: &str,
        body: &crate::types::Feature,
    ) -> Result<crate::types::Feature> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/features/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&feature_key.to_string()),
            query_
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_features_delete(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        feature_key: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/features/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&feature_key.to_string()),
            query_
        );

        self.client.delete(&url, None).await
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
    pub async fn directory_features_patch(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        feature_key: &str,
        body: &crate::types::Feature,
    ) -> Result<crate::types::Feature> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/features/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&feature_key.to_string()),
            query_
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_features_rename(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        old_name: &str,
        body: &crate::types::FeatureRename,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/resources/features/{}/rename?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&old_name.to_string()),
            query_
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}

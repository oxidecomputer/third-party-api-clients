use anyhow::Result;

use crate::Client;

pub struct RoomsLocation {
    pub client: Client,
}

impl RoomsLocation {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        RoomsLocation { client }
    }

    /**
     * List Zoom Room locations.
     *
     * This function performs a `GET` to the `/rooms/locations` endpoint.
     *
     * A Zoom account owner or a Zoom Room administrator can establish a [location hierarchy](https://support.zoom.us/hc/en-us/articles/115000342983-Zoom-Rooms-Location-Hierarchy) to help manage Zoom Rooms that are spread among a variety of locations. Use this API to list the different location types used for Zoom Rooms in an account.<br><br>
     * **Prerequisites:**
     * * Account owner or admin permissions.
     * * Zoom Rooms Version 4.0 or higher<br><br>
     * **Scopes:** `room:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `parent_location_id: &str` -- A unique identifier of the parent location. For instance, if a Zoom Room is located in Floor 1 of Building A, the location of Building A will be the parent location of Floor 1. Use this parameter to filter the response by a specific location hierarchy level.
     * * `type_: &str` -- Use this field to filter the response by the type of location. The value can be one of the following:
     *   `country`, `states`, `city`, `campus`, `building`, `floor`. .
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list_zr_locations(
        &self,
        parent_location_id: &str,
        type_: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::AddAzrLocationResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !parent_location_id.is_empty() {
            query_args.push((
                "parent_location_id".to_string(),
                parent_location_id.to_string(),
            ));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/rooms/locations?{}", query_);

        let resp: crate::types::ListZrLocationsResponseData = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.locations)
    }

    /**
     * List Zoom Room locations.
     *
     * This function performs a `GET` to the `/rooms/locations` endpoint.
     *
     * As opposed to `list_zr_locations`, this function returns all the pages of the request at once.
     *
     * A Zoom account owner or a Zoom Room administrator can establish a [location hierarchy](https://support.zoom.us/hc/en-us/articles/115000342983-Zoom-Rooms-Location-Hierarchy) to help manage Zoom Rooms that are spread among a variety of locations. Use this API to list the different location types used for Zoom Rooms in an account.<br><br>
     * **Prerequisites:**
     * * Account owner or admin permissions.
     * * Zoom Rooms Version 4.0 or higher<br><br>
     * **Scopes:** `room:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn list_all_zr_locations(
        &self,
        parent_location_id: &str,
        type_: &str,
    ) -> Result<Vec<crate::types::AddAzrLocationResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !parent_location_id.is_empty() {
            query_args.push((
                "parent_location_id".to_string(),
                parent_location_id.to_string(),
            ));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/rooms/locations?{}", query_);

        let mut resp: crate::types::ListZrLocationsResponseData =
            self.client.get(&url, None).await?;

        let mut locations = resp.locations;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?next_page_token={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&next_page_token={}", url, page), None)
                    .await?;
            }

            locations.append(&mut resp.locations);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(locations)
    }

    /**
     * Add a location.
     *
     * This function performs a `POST` to the `/rooms/locations` endpoint.
     *
     * Add a location to the [location hierarchial structure(s)](https://support.zoom.us/hc/en-us/articles/115000342983-Zoom-Rooms-Location-Hierarchy) of Zoom Rooms in an account.
     *
     * **Prerequisites:**
     * * Account owner or admin permissions.
     * * Zoom Rooms Version 4.0 or higher<br><br>
     * **Scopes:** `room:write:admin`<br>
     *
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn add_azr_location(
        &self,
        body: &crate::types::AddAzrLocationRequest,
    ) -> Result<crate::types::AddAzrLocationResponse> {
        let url = "/rooms/locations".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get Zoom Room location profile.
     *
     * This function performs a `GET` to the `/rooms/locations/{locationId}` endpoint.
     *
     * Each location type of the [Zoom Rooms location hierarchy](https://support.zoom.us/hc/en-us/articles/115000342983-Zoom-Rooms-Location-Hierarchy) has a profile page that includes information such as name of the location, address, support email, etc. Use this API to retrieve information about a specific Zoom Rooms location type such as information about the city where the Zoom Rooms is located.
     *
     * **Prerequisite:**<br>
     * * Account owner or admin permission
     * * Zoom Rooms version 4.0 or higher<br>
     * **Scopes:** `room:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- Unique identifier of the location type. This can be retrieved using the [List Zoom Room Location API](https://marketplace.zoom.us/docs/api-reference/zoom-api/rooms-location/listzrlocations) (Id property in the response).
     */
    pub async fn get_zr_location_profile(
        &self,
        location_id: &str,
    ) -> Result<crate::types::GetZrLocationProfileResponse> {
        let url = format!(
            "/rooms/locations/{}",
            crate::progenitor_support::encode_path(&location_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update Zoom Room location profile.
     *
     * This function performs a `PATCH` to the `/rooms/locations/{locationId}` endpoint.
     *
     * Each location type of the [Zoom Rooms location hierarchy](https://support.zoom.us/hc/en-us/articles/115000342983-Zoom-Rooms-Location-Hierarchy) has a profile page that includes information such as name of the location, address, support email, etc. Use this API to update information about a specific Zoom Rooms location type such as information about the city where the Zoom Rooms is located.
     *
     * **Prerequisite:**<br>
     * * Account owner or admin permission
     * * Zoom Rooms version 4.0 or higher<br>
     * **Scopes:** `room:write:admin`<br>
     *
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- Unique Identifier of the location. This can be retrieved from the [List Zoom Room Locations](https://marketplace.zoom.us/docs/api-reference/zoom-api/rooms-location/listzrlocations) API.
     */
    pub async fn update_zr_location_profile(
        &self,
        location_id: &str,
        body: &crate::types::GetZrLocationProfileResponse,
    ) -> Result<()> {
        let url = format!(
            "/rooms/locations/{}",
            crate::progenitor_support::encode_path(&location_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get location settings.
     *
     * This function performs a `GET` to the `/rooms/locations/{locationId}/settings` endpoint.
     *
     * Get information on meeting or alert settings applied to Zoom Rooms located in a specific location. By default, only **Meeting Settings** are returned. To view only **Alert Settings**, specify `alert` as the value of the `setting_type` query parameter.<br><br>
     * **Prerequisites:**<br>
     * * Zoom Room licenses
     * * Owner or Admin privileges on the Zoom Account.<br>
     * **Scopes:** `room:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `setting_type: &str` -- The type of setting that you would like to retrieve.<br> `alert`: Alert Settings applied on the Zoom Rooms Account.<br>
     *   `meeting`: Meeting settings of the Zoom Rooms Account.<br>
     *   `signage`: Digital signage settings of the Zoom Rooms Account.
     * * `location_id: &str` -- Unique identifier of the location type. This can be retrieved using the [List Zoom Room Location API](https://marketplace.zoom.us/docs/api-reference/zoom-api/rooms-location/listzrlocations) (Id property in the response).
     */
    pub async fn get_zr_location_setting(
        &self,
        location_id: &str,
        setting_type: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !setting_type.is_empty() {
            query_args.push(("setting_type".to_string(), setting_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/rooms/locations/{}/settings?{}",
            crate::progenitor_support::encode_path(&location_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update location settings.
     *
     * This function performs a `PATCH` to the `/rooms/locations/{locationId}/settings` endpoint.
     *
     * Update information on either meeting or alert settings applied to Zoom Rooms located in a specific location. To update **Alert Settings**, specify `alert` as the value of the `setting_type` query parameter. Similarly, to update **Meeting Settings**, specify `meeting` as the value of the `setting_type` query parameter.<br><br>
     * **Prerequisites:**<br>
     * * Zoom Room licenses
     * * Owner or Admin privileges on the Zoom Account.<br>
     * **Scopes:** `room:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `setting_type: &str` -- The type of setting that you would like to update.<br> `alert`: Alert Settings applied on the Zoom Rooms Account.<br>
     *   `meeting`: Meeting settings of the Zoom Rooms Account.<br>
     *   `signage`: Digital signage settings.
     * * `location_id: &str` -- Unique identifier of the location type. This can be retrieved using the [List Zoom Room Location API](https://marketplace.zoom.us/docs/api-reference/zoom-api/rooms-location/listzrlocations) (Id property in the response).
     */
    pub async fn update_zr_location_settings(
        &self,
        location_id: &str,
        setting_type: &str,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !setting_type.is_empty() {
            query_args.push(("setting_type".to_string(), setting_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/rooms/locations/{}/settings?{}",
            crate::progenitor_support::encode_path(&location_id.to_string()),
            query_
        );

        self.client.patch(&url, None).await
    }

    /**
     * Get Zoom Room location structure.
     *
     * This function performs a `GET` to the `/rooms/locations/structure` endpoint.
     *
     * Get the [location hierarchial structure(s)](https://support.zoom.us/hc/en-us/articles/115000342983-Zoom-Rooms-Location-Hierarchy) applied on the Zoom Rooms in an account.<br><br>
     * **Prerequisites:**<br>
     * * Zoom Rooms version 4.0 or higher
     * * Account owner or admin permissions<br>
     * **Scopes:** `room:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn get_zr_location_structure(
        &self,
    ) -> Result<crate::types::GetZrLocationStructureResponse> {
        let url = "/rooms/locations/structure".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Update Zoom Rooms location structure.
     *
     * This function performs a `PATCH` to the `/rooms/locations/structure` endpoint.
     *
     * Update the [location hierarchial structure(s)](https://support.zoom.us/hc/en-us/articles/115000342983-Zoom-Rooms-Location-Hierarchy) applied on the Zoom Rooms in an account.<br><br>
     * **Prerequisites:**<br>
     * * Zoom Rooms version 4.0 or higher
     * * Account owner or admin permissions<br>
     * **Scopes:** `room:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn update_zoom_structure(
        &self,
        body: &crate::types::GetZrLocationStructureResponse,
    ) -> Result<()> {
        let url = "/rooms/locations/structure".to_string();
        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Change the assigned parent location.
     *
     * This function performs a `PUT` to the `/rooms/locations/{locationId}/location` endpoint.
     *
     * An account owner of a Zoom account can establish a [Zoom Rooms Location Hierarchy](https://support.zoom.us/hc/en-us/articles/115000342983-Zoom-Rooms-Location-Hierarchy) to better organize Zoom Rooms spread accross various location. The location can be structured in a hierarchy with Country being the top-level location, followed by city, campus, building, and floor. The location in the lower level in the hierarchy is considered as a child of the location that is a level above in the hierarchy. Use this API to change the parent location of a child location. <br><br> For instance, if the location hierarchy is structured in a way where there are two campuses (Campus 1, and Campus 2) in a City and Campus 1 consists of a building named Building 1 with a floor where Zoom Rooms are located, and you would like to rearrange the structure so that Building 1 along with its child locations (floor and Zoom Rooms) are relocated directly under Campus 2 instead of Campus 1, you must provide the location ID of Building 1 in the path parameter of this request and the location ID of Campus 2 as the value of `parent_location_id` in the  request body.<br><br>
     * **Prerequisite:**<br>
     * * Account owner or admin permission
     * * Zoom Rooms version 4.0 or higher<br>
     * **Scopes:** `room:write:admin`<br><br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- User's first name.
     */
    pub async fn change_parent_location(
        &self,
        location_id: &str,
        body: &crate::types::ChangeParentLocationRequest,
    ) -> Result<()> {
        let url = format!(
            "/rooms/locations/{}/location",
            crate::progenitor_support::encode_path(&location_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

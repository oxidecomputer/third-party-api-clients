use crate::Client;
use crate::ClientResult;

pub struct Rooms {
    pub client: Client,
}

impl Rooms {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Rooms { client }
    }

    /**
     * List Zoom Rooms.
     *
     * This function performs a `GET` to the `/rooms` endpoint.
     *
     * Zoom Rooms is a software-based room system that provides an integrated experience for audio conferencing, wireless screen sharing and video conferencing. Use this API to list all the existing [Zoom Rooms](https://support.zoom.us/hc/en-us/articles/207483343-Getting-Started-with-Zoom-Rooms) in a Zoom account.<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Zoom Room](https://zoom.us/zoomrooms) license.<br>
     * **Scopes**: `room:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `status: crate::types::ListZoomRoomsStatus` -- The status of the Zoom Room.
     * * `type_: crate::types::ListZoomRoomsType` -- Type of the Zoom Rooms.
     * * `unassigned_rooms: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `location_id: &str` -- Parent location ID of the Zoom Room.
     */
    pub async fn list_zoom(
        &self,
        status: crate::types::ListZoomRoomsStatus,
        type_: crate::types::ListZoomRoomsType,
        unassigned_rooms: bool,
        page_size: i64,
        next_page_token: &str,
        location_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ListZoomRoomsResponse>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !location_id.is_empty() {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if unassigned_rooms {
            query_args.push(("unassigned_rooms".to_string(), unassigned_rooms.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/rooms?{}", query_), None);
        let resp: crate::Response<crate::types::ListZoomRoomsResponseData> = self
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
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.rooms.to_vec(),
        ))
    }
    /**
     * List Zoom Rooms.
     *
     * This function performs a `GET` to the `/rooms` endpoint.
     *
     * As opposed to `list_zoom`, this function returns all the pages of the request at once.
     *
     * Zoom Rooms is a software-based room system that provides an integrated experience for audio conferencing, wireless screen sharing and video conferencing. Use this API to list all the existing [Zoom Rooms](https://support.zoom.us/hc/en-us/articles/207483343-Getting-Started-with-Zoom-Rooms) in a Zoom account.<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Zoom Room](https://zoom.us/zoomrooms) license.<br>
     * **Scopes**: `room:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn list_all_zoom(
        &self,
        status: crate::types::ListZoomRoomsStatus,
        type_: crate::types::ListZoomRoomsType,
        unassigned_rooms: bool,
        location_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ListZoomRoomsResponse>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !location_id.is_empty() {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if unassigned_rooms {
            query_args.push(("unassigned_rooms".to_string(), unassigned_rooms.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/rooms?{}", query_), None);
        let crate::Response::<crate::types::ListZoomRoomsResponseData> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut rooms = body.rooms;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::ListZoomRoomsResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::ListZoomRoomsResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            rooms.append(&mut body.rooms);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, rooms))
    }
    /**
     * Add a Zoom Room.
     *
     * This function performs a `POST` to the `/rooms` endpoint.
     *
     * Use this API to [add a Zoom Room](https://support.zoom.us/hc/en-us/articles/202822279-Add-Zoom-Rooms-on-Web-Portal) to a Zoom account.<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Zoom Room](https://zoom.us/zoomrooms) license.<br>
     * **Scopes**: `room:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn add(
        &self,
        body: &crate::types::AddRoomRequest,
    ) -> ClientResult<crate::Response<crate::types::AddRoomResponse>> {
        let url = self.client.url("/rooms", None);
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
     * Get Zoom Room profile.
     *
     * This function performs a `GET` to the `/rooms/{roomId}` endpoint.
     *
     *
     * Zoom Rooms is a software-based room system that provides an integrated experience for audio conferencing, wireless screen sharing and video conferencing. Use this API to get detailed information on a specific Zoom Room in a Zoom account.
     *
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Zoom Room](https://zoom.us/zoomrooms) license.<br>
     * **Scopes**: `room:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `room_id: &str` -- Unique Identifier of the Zoom Room. This can be retrieved from the response of [List Zoom Rooms](https://marketplace.zoom.us/docs/api-reference/zoom-api/rooms/listzoomrooms) API.
     */
    pub async fn get_zr_profile(
        &self,
        room_id: &str,
    ) -> ClientResult<crate::Response<crate::types::GetZrProfileResponse>> {
        let url = self.client.url(
            &format!("/rooms/{}", crate::progenitor_support::encode_path(room_id),),
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
     * Delete a Zoom Room.
     *
     * This function performs a `DELETE` to the `/rooms/{roomId}` endpoint.
     *
     * [Remove](https://support.zoom.us/hc/en-us/articles/360033432032-Zoom-Room-Device-Profiles#h_e55b2092-c418-4b02-819f-44de51448900) a specific Zoom Room profile from a Zoom account.<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Zoom Room](https://zoom.us/zoomrooms) license.<br>
     * **Scopes**: `room:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `room_id: &str` -- Unique Identifier of a Zoom Room.
     */
    pub async fn delete_zoom(
        &self,
        room_id: &str,
    ) -> ClientResult<crate::Response<crate::types::Domains>> {
        let url = self.client.url(
            &format!("/rooms/{}", crate::progenitor_support::encode_path(room_id),),
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
     * Update a Zoom Room profile.
     *
     * This function performs a `PATCH` to the `/rooms/{roomId}` endpoint.
     *
     * Update basic information on a specific Zoom Room in a Zoom account.<br>
     *
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Zoom Room](https://zoom.us/zoomrooms) license.<br>
     * **Scopes**: `room:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `room_id: &str` -- Unique Identifier of a Zoom Room.
     */
    pub async fn update_profile(
        &self,
        room_id: &str,
        body: &crate::types::UpdateRoomProfileRequest,
    ) -> ClientResult<crate::Response<crate::types::Domains>> {
        let url = self.client.url(
            &format!("/rooms/{}", crate::progenitor_support::encode_path(room_id),),
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
     * Get Zoom Room settings.
     *
     * This function performs a `GET` to the `/rooms/{roomId}/settings` endpoint.
     *
     * Get information on meeting or alert settings applied to a specific Zoom Room. By default, only **Meeting Settings** are returned. To view only **Alert Settings**, specify `alert` as the value of the `setting_type` query parameter.<br>
     * **Prerequisites:**<br>
     * * Zoom Room licenses
     * * Owner or Admin privileges on the Zoom Account.<br>
     * **Scopes:** `room:read:admin`
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     * **Parameters:**
     *
     * * `setting_type: &str` -- The type of setting that you would like to retrieve.<br> `alert`: Alert Settings applied on the Zoom Rooms Account.<br>
     *   `meeting`: Meeting settings of the Zoom Rooms Account.
     * * `room_id: &str` -- Unique identifier of the Zoom Room.
     */
    pub async fn get_zr_setting(
        &self,
        room_id: &str,
        setting_type: &str,
    ) -> ClientResult<crate::Response<crate::types::Domains>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !setting_type.is_empty() {
            query_args.push(("setting_type".to_string(), setting_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/rooms/{}/settings?{}",
                crate::progenitor_support::encode_path(room_id),
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
    /**
     * Update Zoom Room settings.
     *
     * This function performs a `PATCH` to the `/rooms/{roomId}/settings` endpoint.
     *
     * Update either meeting or alert settings applied to a specific Zoom Room. To update **Alert Settings**, specify `alert` as the value of the `setting_type` query parameter. To update **Meeting Settings**, specify `meeting` as the value of the `setting_type` query parameter.<br>
     * **Prerequisites:**<br>
     * * Zoom Room licenses
     * * Owner or Admin privileges on the Zoom Account.<br>
     * **Scopes:** `room:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `room_id: &str` -- Unique Identifier of the Zoom Room.
     * * `setting_type: &str` -- The type of setting that you would like to update.<br> `alert`: Alert Settings applied on the Zoom Room.<br>
     *   `meeting`: Meeting settings of the Zoom Room.<br>
     *   `signage`: Digital signage settings applied on the Zoom Room.
     */
    pub async fn update_zr_settings(
        &self,
        room_id: &str,
        setting_type: &str,
    ) -> ClientResult<crate::Response<()>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !setting_type.is_empty() {
            query_args.push(("setting_type".to_string(), setting_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/rooms/{}/settings?{}",
                crate::progenitor_support::encode_path(room_id),
                query_
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * List Zoom Room devices.
     *
     * This function performs a `GET` to the `/rooms/{roomId}/devices` endpoint.
     *
     * List information about the devices that are being used for a specific [Zoom Room](https://support.zoom.us/hc/en-us/articles/207483343-Getting-Started-with-Zoom-Rooms) in an account.
     *
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Zoom Room](https://zoom.us/zoomrooms) license.<br>
     * * Account owner or admin permissions.
     * **Scopes**: `room:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `room_id: &str` -- Unique Identifier of the Zoom Room. This can be retrieved from the response of [List Zoom Rooms](https://marketplace.zoom.us/docs/api-reference/zoom-api/rooms/listzoomrooms) API.
     */
    pub async fn list_zr_devices(
        &self,
        room_id: &str,
    ) -> ClientResult<crate::Response<crate::types::ListZrDevicesResponse>> {
        let url = self.client.url(
            &format!(
                "/rooms/{}/devices",
                crate::progenitor_support::encode_path(room_id),
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
     * Change a Zoom Room's location.
     *
     * This function performs a `PUT` to the `/rooms/{roomId}/location` endpoint.
     *
     * An account owner of a Zoom account can establish a [Zoom Rooms Location Hierarchy](https://support.zoom.us/hc/en-us/articles/115000342983-Zoom-Rooms-Location-Hierarchy) to better organize Zoom Rooms spread accress various location. The location can be structured in a hierarchy with Country being the top-level location, followed by city, campus, building, and floor. Use this API to assign a new location for a Zoom Room. Note that the Zoom Room can be assigned only to the lowest level location available in the hierarchy.
     * **Prerequisite:**<br>
     * * Account owner or admin permission
     * * Zoom Rooms version 4.0 or higher<br>
     * **Scopes:** `room:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `room_id: &str` -- Unique Identifier of the Zoom Room.
     */
    pub async fn change_zr_location(
        &self,
        room_id: &str,
        body: &crate::types::ChangeZrLocationRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/rooms/{}/location",
                crate::progenitor_support::encode_path(room_id),
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
     * Check-in or check-out of a Zoom Room.
     *
     * This function performs a `PATCH` to the `/rooms/{id}/events` endpoint.
     *
     * The Zoom Rooms check-in feature helps maximize your room utilization. Use this API to either **check in** and confirm that you are utilizing the room reservation or to **check out** of the room so that the room gets released from the scheduled meeting and will be made available for others to use. Learn more from the [Using the Zoom Rooms check-in feature](https://support.zoom.us/hc/en-us/articles/360001147163-Using-the-Zoom-Rooms-check-in-feature) guide.
     *
     * **Prerequisites:**
     * * [Zoom Rooms](https://support.zoom.us/hc/en-us/articles/207483343-Getting-started-with-Zoom-Rooms#:~:text=Zoom%20Rooms%20is%20a%20software,or%20from%20their%20mobile%20device) must have been set up for use for the account and must be online.
     * * You must have access to the Calendar Integration APIs (either Microsoft Exchange or Google Calendar APIs) to get calendar information associated with the Room.
     *
     * **Scope:** `room:write:admin`
     *
     * **Parameters:**
     *
     * * `id: &str` -- User's first name.
     */
    pub async fn check(
        &self,
        id: &str,
        body: &crate::types::CheckInRoomsRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/rooms/{}/events",
                crate::progenitor_support::encode_path(id),
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
     * List digital signage contents.
     *
     * This function performs a `GET` to the `/rooms/digital_signage` endpoint.
     *
     * List information about existing [Zoom Rooms digital signage](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-digital-signage) content in a Zoom account.<br> You can also access this information by logging into your Zoom account in the Zoom web portal and visiting the [Digital Signage Content](https://zoom.us/digitalsignage#/) page listed under **Room Management**.
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Rooms.
     * * Existing content files or folder in [Digital Signage Content](https://zoom.us/digitalsignage#/) page.
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `type_: &str` -- Specify the type of digital signane resource. The value can be one of the following:
     *   * `content`: Returns information about content files.
     *   * `folder`: Returns information about the folder where the content files are located.
     * * `folder_id: &str` -- Unique identifier of the folder where the content is located. Provide this field if you would like to filter the response by contents that are only available in a specific folder.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list_digital_signage_content(
        &self,
        type_: &str,
        folder_id: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Site>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !folder_id.is_empty() {
            query_args.push(("folder_id".to_string(), folder_id.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/rooms/digital_signage?{}", query_), None);
        let resp: crate::Response<crate::types::ListDigitalSignageContentResponse> = self
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
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.contents.to_vec(),
        ))
    }
    /**
     * List digital signage contents.
     *
     * This function performs a `GET` to the `/rooms/digital_signage` endpoint.
     *
     * As opposed to `list_digital_signage_content`, this function returns all the pages of the request at once.
     *
     * List information about existing [Zoom Rooms digital signage](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-digital-signage) content in a Zoom account.<br> You can also access this information by logging into your Zoom account in the Zoom web portal and visiting the [Digital Signage Content](https://zoom.us/digitalsignage#/) page listed under **Room Management**.
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Rooms.
     * * Existing content files or folder in [Digital Signage Content](https://zoom.us/digitalsignage#/) page.
     *
     *
     *
     */
    pub async fn list_all_digital_signage_content(
        &self,
        type_: &str,
        folder_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Site>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !folder_id.is_empty() {
            query_args.push(("folder_id".to_string(), folder_id.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/rooms/digital_signage?{}", query_), None);
        let crate::Response::<crate::types::ListDigitalSignageContentResponse> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut contents = body.contents;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::ListDigitalSignageContentResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::ListDigitalSignageContentResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            contents.append(&mut body.contents);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, contents))
    }
    /**
     * Update E911 digital signage.
     *
     * This function performs a `PATCH` to the `/rooms/events` endpoint.
     *
     * Display or hide E911 emergency alert text content from [Zoom Rooms digital signage](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-digital-signage).
     *
     * **Scope:** `room:write:admin`
     *
     * **Prerequisites:**<br>
     * * [Zoom Rooms](https://zoom.us/zoomrooms/software) 5.3.0 or higher
     * * Zoom Rooms digital signage must be [enabled](https://support.zoom.us/hc/en-us/articles/360000030683-Zoom-Rooms-Digital-Signage#h_767fbb33-82a8-45a8-8392-a1bfa9687edd)
     *
     */
    pub async fn manage_e_91_1signage(
        &self,
    ) -> ClientResult<crate::Response<crate::types::ManageE911SignageResponse>> {
        let url = self.client.url("/rooms/events", None);
        self.client
            .patch(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}

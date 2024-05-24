use crate::Client;
use crate::ClientResult;

pub struct Teamdrives {
    pub client: Client,
}

impl Teamdrives {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Teamdrives { client }
    }

    /**
     * This function performs a `GET` to the `/teamdrives` endpoint.
     *
     * Deprecated use drives.list instead.
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- A map of maximum import sizes by MIME type, in bytes.
     * * `page_token: &str` -- A link to this theme's background image.
     * * `q: &str` -- A link to this theme's background image.
     * * `use_domain_admin_access: bool` -- Issue the request as a domain administrator; if set to true, then all Team Drives of the domain in which the requester is an administrator are returned.
     */
    pub async fn list(
        &self,
        page_size: i64,
        page_token: &str,
        q: &str,
        use_domain_admin_access: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDrive>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/teamdrives?{}", query_), None);
        let resp: crate::Response<crate::types::TeamDriveList> = self
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
            resp.body.team_drives.to_vec(),
        ))
    }
    /**
     * This function performs a `GET` to the `/teamdrives` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Deprecated use drives.list instead.
     */
    pub async fn list_all(
        &self,
        q: &str,
        use_domain_admin_access: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamDrive>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/teamdrives?{}", query_), None);
        let crate::Response::<crate::types::TeamDriveList> {
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

        let mut team_drives = body.team_drives;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                crate::Response::<crate::types::TeamDriveList> {
                    status,
                    headers,
                    body,
                } = self
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
                crate::Response::<crate::types::TeamDriveList> {
                    status,
                    headers,
                    body,
                } = self
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

            team_drives.append(&mut body.team_drives);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, team_drives))
    }
    /**
     * This function performs a `POST` to the `/teamdrives` endpoint.
     *
     * Deprecated use drives.create instead.
     *
     * **Parameters:**
     *
     * * `request_id: &str` -- An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned.
     */
    pub async fn create(
        &self,
        request_id: &str,
        body: &crate::types::TeamDrive,
    ) -> ClientResult<crate::Response<crate::types::TeamDrive>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !request_id.is_empty() {
            query_args.push(("requestId".to_string(), request_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/teamdrives?{}", query_), None);
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
     * This function performs a `GET` to the `/teamdrives/{teamDriveId}` endpoint.
     *
     * Deprecated use drives.get instead.
     *
     * **Parameters:**
     *
     * * `team_drive_id: &str` -- A link to this theme's background image.
     * * `use_domain_admin_access: bool` -- Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs.
     */
    pub async fn get(
        &self,
        team_drive_id: &str,
        use_domain_admin_access: bool,
    ) -> ClientResult<crate::Response<crate::types::TeamDrive>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teamdrives/{}?{}",
                crate::progenitor_support::encode_path(team_drive_id),
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
     * This function performs a `DELETE` to the `/teamdrives/{teamDriveId}` endpoint.
     *
     * Deprecated use drives.delete instead.
     *
     * **Parameters:**
     *
     * * `team_drive_id: &str` -- A link to this theme's background image.
     */
    pub async fn delete(&self, team_drive_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/teamdrives/{}",
                crate::progenitor_support::encode_path(team_drive_id),
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
     * This function performs a `PATCH` to the `/teamdrives/{teamDriveId}` endpoint.
     *
     * Deprecated use drives.update instead
     *
     * **Parameters:**
     *
     * * `team_drive_id: &str` -- A link to this theme's background image.
     * * `use_domain_admin_access: bool` -- Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs.
     */
    pub async fn update(
        &self,
        team_drive_id: &str,
        use_domain_admin_access: bool,
        body: &crate::types::TeamDrive,
    ) -> ClientResult<crate::Response<crate::types::TeamDrive>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if use_domain_admin_access {
            query_args.push((
                "useDomainAdminAccess".to_string(),
                use_domain_admin_access.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/teamdrives/{}?{}",
                crate::progenitor_support::encode_path(team_drive_id),
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
}

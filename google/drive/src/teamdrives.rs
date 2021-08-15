use anyhow::Result;

use crate::Client;

pub struct Teamdrives {
    client: Client,
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
     * * `page_size: i64` -- Maximum number of Team Drives to return.
     * * `page_token: &str` -- A link to this theme's background image.
     * * `q: &str` -- A link to this theme's background image.
     * * `use_domain_admin_access: bool` -- Issue the request as a domain administrator; if set to true, then all Team Drives of the domain in which the requester is an administrator are returned.
     */
    pub async fn drive_list(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        page_size: i64,
        page_token: &str,
        q: &str,
        use_domain_admin_access: bool,
    ) -> Result<crate::types::TeamDriveList> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !q.is_empty() {
            query_args.push(format!("q={}", q));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if use_domain_admin_access {
            query_args.push(format!(
                "use_domain_admin_access={}",
                use_domain_admin_access
            ));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/teamdrives?{}", query);

        self.client.get(&url, None).await
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
    pub async fn drive_create(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        request_id: &str,
        body: &crate::types::TeamDrive,
    ) -> Result<crate::types::TeamDrive> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
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
        if !request_id.is_empty() {
            query_args.push(format!("request_id={}", request_id));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/teamdrives?{}", query);

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn drive_get(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        team_drive_id: &str,
        use_domain_admin_access: bool,
    ) -> Result<crate::types::TeamDrive> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
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
        if use_domain_admin_access {
            query_args.push(format!(
                "use_domain_admin_access={}",
                use_domain_admin_access
            ));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/teamdrives/{}?{}",
            crate::progenitor_support::encode_path(&team_drive_id.to_string()),
            query
        );

        self.client.get(&url, None).await
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
    pub async fn drive_delete(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        team_drive_id: &str,
    ) -> Result<()> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
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
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/teamdrives/{}?{}",
            crate::progenitor_support::encode_path(&team_drive_id.to_string()),
            query
        );

        self.client.delete(&url, None).await
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
    pub async fn drive_update(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        team_drive_id: &str,
        use_domain_admin_access: bool,
        body: &crate::types::TeamDrive,
    ) -> Result<crate::types::TeamDrive> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
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
        if use_domain_admin_access {
            query_args.push(format!(
                "use_domain_admin_access={}",
                use_domain_admin_access
            ));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/teamdrives/{}?{}",
            crate::progenitor_support::encode_path(&team_drive_id.to_string()),
            query
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}

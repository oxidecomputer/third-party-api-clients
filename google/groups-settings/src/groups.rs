use anyhow::Result;

use crate::Client;

pub struct Groups {
    client: Client,
}

impl Groups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Groups { client }
    }

    /**
     * This function performs a `GET` to the `/{groupUniqueId}` endpoint.
     *
     * Gets one resource by id.
     *
     * **Parameters:**
     *
     * * `group_unique_id: &str` -- Identifies whether members external to your organization can join the group. Possible values are:  
     *  - true: G Suite users external to your organization can become members of this group.
     *  - false: Users not belonging to the organization are not allowed to become members of this group.
     */
    pub async fn settings_get(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        group_unique_id: &str,
    ) -> Result<crate::types::Groups> {
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
            "/{}?{}",
            crate::progenitor_support::encode_path(&group_unique_id.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/{groupUniqueId}` endpoint.
     *
     * Updates an existing resource.
     *
     * **Parameters:**
     *
     * * `group_unique_id: &str` -- Identifies whether members external to your organization can join the group. Possible values are:  
     *  - true: G Suite users external to your organization can become members of this group.
     *  - false: Users not belonging to the organization are not allowed to become members of this group.
     */
    pub async fn settings_update(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        group_unique_id: &str,
        body: &crate::types::Groups,
    ) -> Result<crate::types::Groups> {
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
            "/{}?{}",
            crate::progenitor_support::encode_path(&group_unique_id.to_string()),
            query
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `PATCH` to the `/{groupUniqueId}` endpoint.
     *
     * Updates an existing resource. This method supports patch semantics.
     *
     * **Parameters:**
     *
     * * `group_unique_id: &str` -- Identifies whether members external to your organization can join the group. Possible values are:  
     *  - true: G Suite users external to your organization can become members of this group.
     *  - false: Users not belonging to the organization are not allowed to become members of this group.
     */
    pub async fn settings_patch(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        group_unique_id: &str,
        body: &crate::types::Groups,
    ) -> Result<crate::types::Groups> {
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
            "/{}?{}",
            crate::progenitor_support::encode_path(&group_unique_id.to_string()),
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

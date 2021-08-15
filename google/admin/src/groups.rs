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
     * This function performs a `GET` to the `/admin/directory/v1/groups` endpoint.
     *
     * Retrieves all groups of a domain or of a user given a userKey (paginated).
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. In case of a multi-domain account, to fetch all groups for a customer, fill this field instead of domain. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users](/admin-sdk/directory/v1/reference/users).
     * * `domain: &str` -- The domain name. Use this field to get fields from only one domain. To return all domains for a customer account, use the `customer` query parameter instead.
     * * `max_results: i64` -- Maximum number of results to return. Max allowed value is 200.
     * * `order_by: crate::types::DirectoryGroupsListOrderBy` -- Column to use for sorting results.
     * * `page_token: &str` -- Token to specify next page in the list.
     * * `query: &str` -- Query string search. Should be of the form "". Complete documentation is at https: //developers.google.com/admin-sdk/directory/v1/guides/search-groups.
     * * `sort_order: crate::types::SortOrder` -- Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
     * * `user_key: &str` -- Email or immutable ID of the user if only those groups are to be listed, the given user is a member of. If it's an ID, it should match with the ID of the user object.
     */
    pub async fn directory_list(
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
        domain: &str,
        max_results: i64,
        order_by: crate::types::DirectoryGroupsListOrderBy,
        page_token: &str,
        query: &str,
        sort_order: crate::types::SortOrder,
        user_key: &str,
    ) -> Result<crate::types::Groups> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !customer.is_empty() {
            query_args.push(format!("customer={}", customer));
        }
        if !domain.is_empty() {
            query_args.push(format!("domain={}", domain));
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
        query_args.push(format!("order_by={}", order_by));
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
        query_args.push(format!("sort_order={}", sort_order));
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        if !user_key.is_empty() {
            query_args.push(format!("user_key={}", user_key));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/admin/directory/v1/groups?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/groups` endpoint.
     *
     * Creates a group.
     */
    pub async fn directory_insert(
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
        body: &crate::types::Group,
    ) -> Result<crate::types::Group> {
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
        let url = format!("/admin/directory/v1/groups?{}", query_);

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/groups/{groupKey}` endpoint.
     *
     * Retrieves a group's properties.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn directory_get(
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
        group_key: &str,
    ) -> Result<crate::types::Group> {
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
            "/admin/directory/v1/groups/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/admin/directory/v1/groups/{groupKey}` endpoint.
     *
     * Updates a group's properties.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn directory_update(
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
        group_key: &str,
        body: &crate::types::Group,
    ) -> Result<crate::types::Group> {
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
            "/admin/directory/v1/groups/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/groups/{groupKey}` endpoint.
     *
     * Deletes a group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn directory_delete(
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
        group_key: &str,
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
            "/admin/directory/v1/groups/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/admin/directory/v1/groups/{groupKey}` endpoint.
     *
     * Updates a group's properties. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn directory_patch(
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
        group_key: &str,
        body: &crate::types::Group,
    ) -> Result<crate::types::Group> {
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
            "/admin/directory/v1/groups/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
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
     * This function performs a `GET` to the `/admin/directory/v1/groups/{groupKey}/aliases` endpoint.
     *
     * Lists all aliases for a group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn directory_aliases_list(
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
        group_key: &str,
    ) -> Result<crate::types::Aliases> {
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
            "/admin/directory/v1/groups/{}/aliases?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/groups/{groupKey}/aliases` endpoint.
     *
     * Adds an alias for the group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn directory_aliases_insert(
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
        group_key: &str,
        body: &crate::types::Alias,
    ) -> Result<crate::types::Alias> {
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
            "/admin/directory/v1/groups/{}/aliases?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/groups/{groupKey}/aliases/{alias}` endpoint.
     *
     * Removes an alias.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     * * `alias: &str` -- The alias to be removed.
     */
    pub async fn directory_aliases_delete(
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
        group_key: &str,
        alias: &str,
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
            "/admin/directory/v1/groups/{}/aliases/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&alias.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }
}

use anyhow::Result;

use crate::Client;

pub struct Users {
    client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users` endpoint.
     *
     * Retrieves a paginated list of either deleted users or all users in a domain.
     *
     * **Parameters:**
     *
     * * `custom_field_mask: &str` -- A comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when `projection=custom`.
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. In case of a multi-domain account, to fetch all groups for a customer, fill this field instead of domain. You can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users). Either the `customer` or the `domain` parameter must be provided.
     * * `domain: &str` -- The domain name. Use this field to get fields from only one domain. To return all domains for a customer account, use the `customer` query parameter instead. Either the `customer` or the `domain` parameter must be provided.
     * * `event: crate::types::Event` -- Event on which subscription is intended (if subscribing).
     * * `max_results: i64` -- Maximum number of results to return.
     * * `order_by: crate::types::DirectoryUsersListOrderBy` -- Property to use for sorting results.
     * * `page_token: &str` -- Token to specify next page in the list.
     * * `projection: crate::types::DirectoryUsersListProjection` -- What subset of fields to fetch for this user.
     * * `query: &str` -- Query string for searching user fields. For more information on constructing user queries, see [Search for Users](/admin-sdk/directory/v1/guides/search-users).
     * * `show_deleted: &str` -- If set to `true`, retrieves the list of deleted users. (Default: `false`).
     * * `sort_order: crate::types::SortOrder` -- Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
     * * `view_type: crate::types::ViewType` -- Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin).
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
        custom_field_mask: &str,
        customer: &str,
        domain: &str,
        event: crate::types::Event,
        max_results: i64,
        order_by: crate::types::DirectoryUsersListOrderBy,
        page_token: &str,
        projection: crate::types::DirectoryUsersListProjection,
        query: &str,
        show_deleted: &str,
        sort_order: crate::types::SortOrder,
        view_type: crate::types::ViewType,
    ) -> Result<crate::types::Users> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !custom_field_mask.is_empty() {
            query_args.push(format!("custom_field_mask={}", custom_field_mask));
        }
        if !customer.is_empty() {
            query_args.push(format!("customer={}", customer));
        }
        if !domain.is_empty() {
            query_args.push(format!("domain={}", domain));
        }
        query_args.push(format!("event={}", event));
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
        query_args.push(format!("projection={}", projection));
        if !query.is_empty() {
            query_args.push(format!("query={}", query));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !show_deleted.is_empty() {
            query_args.push(format!("show_deleted={}", show_deleted));
        }
        query_args.push(format!("sort_order={}", sort_order));
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("view_type={}", view_type));
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/admin/directory/v1/users?{}", query);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users` endpoint.
     *
     * Creates a user.
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
        body: &crate::types::User,
    ) -> Result<crate::types::User> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/admin/directory/v1/users?{}", query);

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/watch` endpoint.
     *
     * Watches for changes in users list.
     *
     * **Parameters:**
     *
     * * `custom_field_mask: &str` -- Comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when projection=custom.
     * * `customer: &str` -- Immutable ID of the Google Workspace account. In case of multi-domain, to fetch all users for a customer, fill this field instead of domain.
     * * `domain: &str` -- Name of the domain. Fill this field to get users from only this domain. To return all users in a multi-domain fill customer field instead.".
     * * `event: crate::types::Event` -- Event on which subscription is intended (if subscribing).
     * * `max_results: i64` -- Maximum number of results to return.
     * * `order_by: crate::types::DirectoryUsersListOrderBy` -- Property to use for sorting results.
     * * `page_token: &str` -- Token to specify next page in the list.
     * * `projection: crate::types::DirectoryUsersListProjection` -- What subset of fields to fetch for this user.
     * * `query: &str` -- Query string search. Should be of the form "". Complete documentation is at https: //developers.google.com/admin-sdk/directory/v1/guides/search-users.
     * * `show_deleted: &str` -- If set to true, retrieves the list of deleted users. (Default: false).
     * * `sort_order: crate::types::SortOrder` -- Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
     * * `view_type: crate::types::ViewType` -- Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin).
     */
    pub async fn directory_watch(
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
        custom_field_mask: &str,
        customer: &str,
        domain: &str,
        event: crate::types::Event,
        max_results: i64,
        order_by: crate::types::DirectoryUsersListOrderBy,
        page_token: &str,
        projection: crate::types::DirectoryUsersListProjection,
        query: &str,
        show_deleted: &str,
        sort_order: crate::types::SortOrder,
        view_type: crate::types::ViewType,
        body: &crate::types::Channel,
    ) -> Result<crate::types::Channel> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !custom_field_mask.is_empty() {
            query_args.push(format!("custom_field_mask={}", custom_field_mask));
        }
        if !customer.is_empty() {
            query_args.push(format!("customer={}", customer));
        }
        if !domain.is_empty() {
            query_args.push(format!("domain={}", domain));
        }
        query_args.push(format!("event={}", event));
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
        query_args.push(format!("projection={}", projection));
        if !query.is_empty() {
            query_args.push(format!("query={}", query));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !show_deleted.is_empty() {
            query_args.push(format!("show_deleted={}", show_deleted));
        }
        query_args.push(format!("sort_order={}", sort_order));
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("view_type={}", view_type));
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/admin/directory/v1/users/watch?{}", query);

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}` endpoint.
     *
     * Retrieves a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `custom_field_mask: &str` -- A comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when `projection=custom`.
     * * `projection: crate::types::DirectoryUsersListProjection` -- What subset of fields to fetch for this user.
     * * `view_type: crate::types::ViewType` -- Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin).
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
        user_key: &str,
        custom_field_mask: &str,
        projection: crate::types::DirectoryUsersListProjection,
        view_type: crate::types::ViewType,
    ) -> Result<crate::types::User> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !custom_field_mask.is_empty() {
            query_args.push(format!("custom_field_mask={}", custom_field_mask));
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
        query_args.push(format!("projection={}", projection));
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("view_type={}", view_type));
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/admin/directory/v1/users/{userKey}` endpoint.
     *
     * Updates a user. This method supports patch semantics, meaning you only need to include the fields you wish to update. Fields that are not present in the request will be preserved, and fields set to `null` will be cleared.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
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
        user_key: &str,
        body: &crate::types::User,
    ) -> Result<crate::types::User> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/users/{userKey}` endpoint.
     *
     * Deletes a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
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
        user_key: &str,
    ) -> Result<()> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/admin/directory/v1/users/{userKey}` endpoint.
     *
     * Updates a user using patch semantics. The update method should be used instead, since it also supports patch semantics and has better performance. This method is unable to clear fields that contain repeated objects (`addresses`, `phones`, etc). Use the update method instead.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
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
        user_key: &str,
        body: &crate::types::User,
    ) -> Result<crate::types::User> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/aliases` endpoint.
     *
     * Lists all aliases for a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `event: crate::types::DirectoryUsersAliasesListEvent` -- Events to watch for.
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
        user_key: &str,
        event: crate::types::DirectoryUsersAliasesListEvent,
    ) -> Result<crate::types::Aliases> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        query_args.push(format!("event={}", event));
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/aliases?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/aliases` endpoint.
     *
     * Adds an alias.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
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
        user_key: &str,
        body: &crate::types::Alias,
    ) -> Result<crate::types::Alias> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/aliases?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/aliases/watch` endpoint.
     *
     * Watches for changes in users list.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Email or immutable ID of the user.
     * * `event: crate::types::DirectoryUsersAliasesListEvent` -- Events to watch for.
     */
    pub async fn directory_aliases_watch(
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
        user_key: &str,
        event: crate::types::DirectoryUsersAliasesListEvent,
        body: &crate::types::Channel,
    ) -> Result<crate::types::Channel> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        query_args.push(format!("event={}", event));
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/aliases/watch?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `DELETE` to the `/admin/directory/v1/users/{userKey}/aliases/{alias}` endpoint.
     *
     * Removes an alias.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
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
        user_key: &str,
        alias: &str,
    ) -> Result<()> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/aliases/{}?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            crate::progenitor_support::encode_path(&alias.to_string()),
            query
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/makeAdmin` endpoint.
     *
     * Makes a user a super administrator.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn directory_make_admin(
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
        user_key: &str,
        body: &crate::types::UserMakeAdmin,
    ) -> Result<()> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/makeAdmin?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/photos/thumbnail` endpoint.
     *
     * Retrieves the user's photo.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn directory_photos_get(
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
        user_key: &str,
    ) -> Result<crate::types::UserPhoto> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/photos/thumbnail?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/admin/directory/v1/users/{userKey}/photos/thumbnail` endpoint.
     *
     * Adds a photo for the user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn directory_photos_update(
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
        user_key: &str,
        body: &crate::types::UserPhoto,
    ) -> Result<crate::types::UserPhoto> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/photos/thumbnail?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/users/{userKey}/photos/thumbnail` endpoint.
     *
     * Removes the user's photo.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn directory_photos_delete(
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
        user_key: &str,
    ) -> Result<()> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/photos/thumbnail?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/admin/directory/v1/users/{userKey}/photos/thumbnail` endpoint.
     *
     * Adds a photo for the user. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn directory_photos_patch(
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
        user_key: &str,
        body: &crate::types::UserPhoto,
    ) -> Result<crate::types::UserPhoto> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/photos/thumbnail?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/signOut` endpoint.
     *
     * Signs a user out of all web and device sessions and reset their sign-in cookies. User will have to sign in by authenticating again.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the target user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn directory_sign_out(
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
        user_key: &str,
    ) -> Result<()> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/signOut?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/undelete` endpoint.
     *
     * Undeletes a deleted user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- The immutable id of the user.
     */
    pub async fn directory_undelete(
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
        user_key: &str,
        body: &crate::types::UserUndelete,
    ) -> Result<()> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/undelete?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}

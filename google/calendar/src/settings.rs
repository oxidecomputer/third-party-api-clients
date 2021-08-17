use anyhow::Result;

use crate::Client;

pub struct Settings {
    pub client: Client,
}

impl Settings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Settings { client }
    }

    /**
     * This function performs a `GET` to the `/users/me/settings` endpoint.
     *
     * Returns all user settings for the authenticated user.
     *
     * **Parameters:**
     *
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn calendar_list(
        &self,
        max_results: i64,
        page_token: &str,
    ) -> Result<Vec<crate::types::Setting>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if max_results > 0 {
            query_args.push(format!("max_results={}", max_results));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/users/me/settings?{}", query_);

        let resp: crate::types::Settings = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.items)
    }

    /**
     * This function performs a `GET` to the `/users/me/settings` endpoint.
     *
     * As opposed to `calendar_list`, this function returns all the pages of the request at once.
     *
     * Returns all user settings for the authenticated user.
     */
    pub async fn calendar_list_settings(&self) -> Result<Vec<crate::types::Setting>> {
        let url = "/users/me/settings".to_string();
        let mut resp: crate::types::Settings = self.client.get(&url, None).await.unwrap();

        let mut items = resp.items;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?pageToken={}", url, page), None)
                    .await
                    .unwrap();
            } else {
                resp = self
                    .client
                    .get(&format!("{}&pageToken={}", url, page), None)
                    .await
                    .unwrap();
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
     * This function performs a `POST` to the `/users/me/settings/watch` endpoint.
     *
     * Watch for changes to Settings resources.
     *
     * **Parameters:**
     *
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn calendar_watch(
        &self,
        max_results: i64,
        page_token: &str,
        body: &crate::types::Channel,
    ) -> Result<crate::types::Channel> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if max_results > 0 {
            query_args.push(format!("max_results={}", max_results));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/users/me/settings/watch?{}", query_);

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/users/me/settings/{setting}` endpoint.
     *
     * Returns a single user setting.
     *
     * **Parameters:**
     *
     * * `setting: &str` -- The id of the user setting.
     */
    pub async fn calendar_get(&self, setting: &str) -> Result<crate::types::Setting> {
        let url = format!(
            "/users/me/settings/{}",
            crate::progenitor_support::encode_path(&setting.to_string()),
        );

        self.client.get(&url, None).await
    }
}

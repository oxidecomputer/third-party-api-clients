use anyhow::Result;

use crate::Client;

pub struct Contacts {
    pub client: Client,
}

impl Contacts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Contacts { client }
    }

    /**
     * Search company contacts.
     *
     * This function performs a `GET` to the `/contacts` endpoint.
     *
     * A user under an organization's Zoom account has internal users listed under Company Contacts in the Zoom Client. Use this API to search users that are in the company contacts of a Zoom account. Using the `search_key` query parameter, provide either first name, last name or the email address of the user that you would like to search for. Optionally, set `query_presence_status` to `true` in order to include the presence status of a contact. <br><br>
     *
     * **Scopes:** `contact:read:admin`, `contact:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `search_key: &str` -- Provide the keyword - either first name, last name or email of the contact whom you have to search for.
     * * `query_presence_status: &str` -- Set `query_presence_status` to `true` in order to include the presence status of a contact in the response.
     * * `page_size: i64` -- The number of records to be returned with a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn search_company(
        &self,
        search_key: &str,
        query_presence_status: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::Contacts>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !query_presence_status.is_empty() {
            query_args.push((
                "query_presence_status".to_string(),
                query_presence_status.to_string(),
            ));
        }
        if !search_key.is_empty() {
            query_args.push(("search_key".to_string(), search_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/contacts?{}", query_);

        let resp: crate::types::SearchCompanyContactsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.contacts)
    }

    /**
     * Search company contacts.
     *
     * This function performs a `GET` to the `/contacts` endpoint.
     *
     * As opposed to `search_company`, this function returns all the pages of the request at once.
     *
     * A user under an organization's Zoom account has internal users listed under Company Contacts in the Zoom Client. Use this API to search users that are in the company contacts of a Zoom account. Using the `search_key` query parameter, provide either first name, last name or the email address of the user that you would like to search for. Optionally, set `query_presence_status` to `true` in order to include the presence status of a contact. <br><br>
     *
     * **Scopes:** `contact:read:admin`, `contact:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn get_all_search_company(
        &self,
        search_key: &str,
        query_presence_status: &str,
    ) -> Result<Vec<crate::types::Contacts>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !query_presence_status.is_empty() {
            query_args.push((
                "query_presence_status".to_string(),
                query_presence_status.to_string(),
            ));
        }
        if !search_key.is_empty() {
            query_args.push(("search_key".to_string(), search_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/contacts?{}", query_);

        let mut resp: crate::types::SearchCompanyContactsResponse =
            self.client.get(&url, None).await?;

        let mut contacts = resp.contacts;
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

            contacts.append(&mut resp.contacts);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(contacts)
    }

    /**
     * List user's contacts.
     *
     * This function performs a `GET` to the `/chat/users/me/contacts` endpoint.
     *
     * A user under an organization’s Zoom account has internal users listed under Company Contacts in the Zoom Client. A Zoom user can also add another Zoom user as a [contact](https://support.zoom.us/hc/en-us/articles/115004055706-Managing-Contacts). Call this API to list all the contacts of a Zoom user. Zoom contacts are categorized into "company contacts" and "external contacts". You must specify the contact type in the `type` query parameter. If you do not specify, by default, the type will be set as company contact.
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note: </b> This API only supports <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>.</p><br>
     *
     * **Scope**: `chat_contact:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `type_: &str` -- The type of contact. The value can be one of the following:
     *   `company`: Contacts from the user's organization.
     *   `external`: External contacts. .
     * * `page_size: i64` -- The number of records returned with a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn get_user(
        &self,
        type_: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::GetUserContactsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
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
        let url = format!("/chat/users/me/contacts?{}", query_);

        let resp: crate::types::GetUserContactsResponseData = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.contacts)
    }

    /**
     * List user's contacts.
     *
     * This function performs a `GET` to the `/chat/users/me/contacts` endpoint.
     *
     * As opposed to `get_user`, this function returns all the pages of the request at once.
     *
     * A user under an organization’s Zoom account has internal users listed under Company Contacts in the Zoom Client. A Zoom user can also add another Zoom user as a [contact](https://support.zoom.us/hc/en-us/articles/115004055706-Managing-Contacts). Call this API to list all the contacts of a Zoom user. Zoom contacts are categorized into "company contacts" and "external contacts". You must specify the contact type in the `type` query parameter. If you do not specify, by default, the type will be set as company contact.
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note: </b> This API only supports <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>.</p><br>
     *
     * **Scope**: `chat_contact:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn get_all_user(
        &self,
        type_: &str,
    ) -> Result<Vec<crate::types::GetUserContactsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/chat/users/me/contacts?{}", query_);

        let mut resp: crate::types::GetUserContactsResponseData =
            self.client.get(&url, None).await?;

        let mut contacts = resp.contacts;
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

            contacts.append(&mut resp.contacts);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(contacts)
    }

    /**
     * Get user's contact details.
     *
     * This function performs a `GET` to the `/chat/users/me/contacts/{contactId}` endpoint.
     *
     * A user under an organization’s Zoom account has internal users listed under Company Contacts in the Zoom Client. A Zoom user can also add another Zoom user as a [contact](https://support.zoom.us/hc/en-us/articles/115004055706-Managing-Contacts). Call this API to get information on a specific contact of the Zoom user.
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note: </b>This API only supports <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>.</p><br>
     *
     * **Scope**: `chat_contact:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `contact_id: &str` -- The user's contact Id or email address. The contact can be either a company contact or an external contact.
     * * `query_presence_status: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     */
    pub async fn get_user_contacts(
        &self,
        contact_id: &str,
        query_presence_status: bool,
    ) -> Result<crate::types::GetUserContactResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if query_presence_status {
            query_args.push((
                "query_presence_status".to_string(),
                query_presence_status.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/chat/users/me/contacts/{}?{}",
            crate::progenitor_support::encode_path(&contact_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}

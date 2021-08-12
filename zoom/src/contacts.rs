use anyhow::Result;

use crate::Client;

pub struct Contacts {
    client: Client,
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
    ) -> Result<crate::types::SearchCompanyContactsResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(format!("next_page_token={}", next_page_token));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        if !query_presence_status.is_empty() {
            query_args.push(format!("query_presence_status={}", query_presence_status));
        }
        if !search_key.is_empty() {
            query_args.push(format!("search_key={}", search_key));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/contacts?{}", query);

        self.client.get(&url, None).await
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
     *  `company`: Contacts from the user's organization.
     *  `external`: External contacts.
     * * `page_size: i64` -- The number of records returned with a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn get_user(
        &self,
        type_: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::GetUserContactsResponseData> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(format!("next_page_token={}", next_page_token));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        if !type_.is_empty() {
            query_args.push(format!("type={}", type_));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/chat/users/me/contacts?{}", query);

        self.client.get(&url, None).await
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
     * * `query_presence_status: bool` -- The presence status of the contact.
     *  Include this query parameter with a value of `true` to get the presence status of the contact in the response.
     */
    pub async fn get_user_contact(
        &self,
        contact_id: &str,
        query_presence_status: bool,
    ) -> Result<crate::types::GetUserContactResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if query_presence_status {
            query_args.push(format!("query_presence_status={}", query_presence_status));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/chat/users/me/contacts/{}?{}",
            crate::progenitor_support::encode_path(&contact_id.to_string()),
            query
        );

        self.client.get(&url, None).await
    }
}

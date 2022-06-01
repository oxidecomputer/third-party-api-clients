use anyhow::Result;

use crate::Client;

pub struct Lists {
    pub client: Client,
}

impl Lists {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Lists { client }
    }

    /**
    * Get All Lists.
    *
    * This function performs a `GET` to the `/marketing/lists` endpoint.
    *
    * **This endpoint returns an array of all of your contact lists.**
    *
    * **Parameters:**
    *
    * * `page_size: f64` -- Maximum number of elements to return. Defaults to 100, returns 1000 max.
    * * `page_token: &str` -- The license key provided with your New Relic account.
    */
    pub async fn get_mc(
        &self,
        page_size: f64,
        page_token: &str,
    ) -> Result<crate::types::GetMcListsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_size.to_string().is_empty() {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/marketing/lists?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Create List.
    *
    * This function performs a `POST` to the `/marketing/lists` endpoint.
    *
    * **This endpoint creates a new contacts list.**
    *
    * Once you create a list, you can use the UI to [trigger an automation](https://sendgrid.com/docs/ui/sending-email/getting-started-with-automation/#create-an-automation) every time you add a new contact to the list.
    *
    * A link to the newly created object is in `_metadata`.
    */
    pub async fn post_mc(&self, body: &crate::types::IpPool) -> Result<crate::types::List> {
        let url = "/marketing/lists".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get List Contact Count.
    *
    * This function performs a `GET` to the `/marketing/lists/{id}/contacts/count` endpoint.
    *
    * **This endpoint returns the number of contacts on a specific list.**
    */
    pub async fn get_mc_contacts_count(
        &self,
        id: &str,
    ) -> Result<crate::types::GetMcListsContactsCountResponse> {
        let url = format!(
            "/marketing/lists/{}/contacts/count",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Get a List by ID.
    *
    * This function performs a `GET` to the `/marketing/lists/{id}` endpoint.
    *
    * **This endpoint returns data about a specific list.**
    *
    * Setting the optional parameter `contact_sample=true` returns the `contact_sample` in the response body. Up to fifty of the most recent contacts uploaded or attached to a list will be returned, sorted alphabetically, by email address.
    *
    * The full contact count is also returned.
    *
    * **Parameters:**
    *
    * * `contact_sample: bool` -- Indicates if your subuser statistics will be sent to your New Relic Dashboard.
    */
    pub async fn get_mc_lists(
        &self,
        id: &str,
        contact_sample: bool,
    ) -> Result<crate::types::GetMcListsResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if contact_sample {
            query_args.push(("contact_sample".to_string(), contact_sample.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/marketing/lists/{}?{}",
            crate::progenitor_support::encode_path(&id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete a list.
    *
    * This function performs a `DELETE` to the `/marketing/lists/{id}` endpoint.
    *
    * **This endpoint allows you to deletes a specific list.**
    *
    * Optionally, you can also delete contacts associated to the list. The query parameter, `delete_contacts=true`, will delete the list and start an asynchronous job to delete associated contacts.
    *
    * **Parameters:**
    *
    * * `delete_contacts: bool` -- Indicates if your subuser statistics will be sent to your New Relic Dashboard.
    */
    pub async fn delete(
        &self,
        id: &str,
        delete_contacts: bool,
    ) -> Result<crate::types::DeleteListsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if delete_contacts {
            query_args.push(("delete_contacts".to_string(), delete_contacts.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/marketing/lists/{}?{}",
            crate::progenitor_support::encode_path(&id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update List.
    *
    * This function performs a `PATCH` to the `/marketing/lists/{id}` endpoint.
    *
    * **This endpoint updates the name of a list.**
    */
    pub async fn patch_mc(
        &self,
        id: &str,
        body: &crate::types::PatchMcListsRequest,
    ) -> Result<crate::types::List> {
        let url = format!(
            "/marketing/lists/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Remove Contacts from a List.
    *
    * This function performs a `DELETE` to the `/marketing/lists/{id}/contacts` endpoint.
    *
    * **This endpoint allows you to remove contacts from a given list.**
    *
    * The contacts will not be deleted. Only their list membership will be changed.
    *
    * **Parameters:**
    *
    * * `contact_ids: &str` -- The license key provided with your New Relic account.
    */
    pub async fn delete_mc_contacts(
        &self,
        id: &str,
        contact_ids: &str,
    ) -> Result<crate::types::DeleteMcListsContactsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !contact_ids.is_empty() {
            query_args.push(("contact_ids".to_string(), contact_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/marketing/lists/{}/contacts?{}",
            crate::progenitor_support::encode_path(&id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }
}

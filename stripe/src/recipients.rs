use anyhow::Result;

use crate::Client;

pub struct Recipients {
    pub client: Client,
}

impl Recipients {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Recipients { client }
    }

    /**
    * This function performs a `GET` to the `/v1/recipients` endpoint.
    *
    * <p>Returns a list of your recipients. The recipients are returned sorted by creation date, with the most recently created recipients appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `type_: crate::types::GetRecipientsType`
    * * `verified: bool` -- Only return recipients that are verified or unverified.
    */
    pub async fn get_page(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        type_: crate::types::GetRecipientsType,
        verified: bool,
    ) -> Result<Vec<crate::types::Recipient>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if verified {
            query_args.push(("verified".to_string(), verified.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/recipients?{}", query_);

        let resp: crate::types::GetRecipientsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data)
    }

    /**
    * This function performs a `GET` to the `/v1/recipients` endpoint.
    *
    * As opposed to `get`, this function returns all the pages of the request at once.
    *
    * <p>Returns a list of your recipients. The recipients are returned sorted by creation date, with the most recently created recipients appearing first.</p>
    */
    pub async fn get_all(
        &self,
        created: &str,
        expand: &[String],
        type_: crate::types::GetRecipientsType,
        verified: bool,
    ) -> Result<Vec<crate::types::Recipient>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if verified {
            query_args.push(("verified".to_string(), verified.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/recipients?{}", query_);

        let mut resp: crate::types::GetRecipientsResponse = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                page = data.last().unwrap().id.to_string();
            }
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?startng_after={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&starting_after={}", url, page), None)
                    .await?;
            }

            data.append(&mut resp.data);

            has_more = resp.has_more;
        }

        // Return our response data.
        Ok(data)
    }

    /**
    * This function performs a `POST` to the `/v1/recipients` endpoint.
    *
    * <p>Creates a new <code>Recipient</code> object and verifies the recipient’s identity.
    * Also verifies the recipient’s bank account information or debit card, if either is provided.</p>
    */
    pub async fn post(&self) -> Result<crate::types::Recipient> {
        let url = "/v1/recipients".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/recipients/{id}` endpoint.
    *
    * <p>Retrieves the details of an existing recipient. You need only supply the unique recipient identifier that was returned upon recipient creation.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::GetRecipientsResponseAnyOf> {
        let url = format!(
            "/v1/recipients/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/recipients/{id}` endpoint.
    *
    * <p>Updates the specified recipient by setting the values of the parameters passed.
    * Any parameters not provided will be left unchanged.</p>
    *
    * <p>If you update the name or tax ID, the identity verification will automatically be rerun.
    * If you update the bank account, the bank account validation will automatically be rerun.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_recipients(&self, id: &str) -> Result<crate::types::Recipient> {
        let url = format!(
            "/v1/recipients/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/recipients/{id}` endpoint.
    *
    * <p>Permanently deletes a recipient. It cannot be undone.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn delete(&self, id: &str) -> Result<crate::types::DeletedRecipient> {
        let url = format!(
            "/v1/recipients/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

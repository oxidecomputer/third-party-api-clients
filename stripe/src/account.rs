use anyhow::Result;

use crate::Client;

pub struct Account {
    pub client: Client,
}

impl Account {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Account { client }
    }

    /**
    * This function performs a `GET` to the `/v1/account` endpoint.
    *
    * <p>Retrieves the details of an account.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get(&self) -> Result<crate::types::Account> {
        let url = "/v1/account".to_string();
        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account` endpoint.
    *
    * <p>Updates a <a href="/docs/connect/accounts">connected account</a> by setting the values of the parameters passed. Any parameters not provided are left unchanged. Most parameters can be changed only for Custom accounts. (These are marked <strong>Custom Only</strong> below.) Parameters marked <strong>Custom and Express</strong> are not supported for Standard accounts.</p>
    *
    * <p>To update your own account, use the <a href="https://dashboard.stripe.com/account">Dashboard</a>. Refer to our <a href="/docs/connect/updating-accounts">Connect</a> documentation to learn more about updating accounts.</p>
    */
    pub async fn post(&self) -> Result<crate::types::Account> {
        let url = "/v1/account".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/account` endpoint.
    *
    * <p>With <a href="/docs/connect">Connect</a>, you can delete accounts you manage.</p>
    *
    * <p>Accounts created using test-mode keys can be deleted at any time. Standard accounts created using live-mode keys cannot be deleted. Custom or Express accounts created using live-mode keys can only be deleted once all balances are zero.</p>
    *
    * <p>If you want to delete your own account, use the <a href="https://dashboard.stripe.com/account">account information tab in your account settings</a> instead.</p>
    */
    pub async fn delete(&self) -> Result<crate::types::DeletedAccount> {
        let url = "/v1/account".to_string();
        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/bank_accounts` endpoint.
    *
    * <p>Create an external account for a given account.</p>
    */
    pub async fn post_bank(&self) -> Result<crate::types::DataAnyOf> {
        let url = "/v1/account/bank_accounts".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/account/bank_accounts/{id}` endpoint.
    *
    * <p>Retrieve a specified external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_bank(&self, id: &str) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/account/bank_accounts/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/bank_accounts/{id}` endpoint.
    *
    * <p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>
    *
    * <p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_bank_account(&self, id: &str) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/account/bank_accounts/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/account/bank_accounts/{id}` endpoint.
    *
    * <p>Delete a specified external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_bank(&self, id: &str) -> Result<crate::types::DeletedExternalAccountAnyOf> {
        let url = format!(
            "/v1/account/bank_accounts/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/account/capabilities` endpoint.
    *
    * <p>Returns a list of capabilities associated with the account. The capabilities are returned sorted by creation date, with the most recent capability appearing first.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_capabilities(&self) -> Result<Vec<crate::types::Capability>> {
        let url = "/v1/account/capabilities".to_string();
        let resp: crate::types::ListAccountCapability = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }

    /**
    * This function performs a `GET` to the `/v1/account/capabilities` endpoint.
    *
    * As opposed to `get_capabilities`, this function returns all the pages of the request at once.
    *
    * <p>Returns a list of capabilities associated with the account. The capabilities are returned sorted by creation date, with the most recent capability appearing first.</p>
    */
    pub async fn get_all_capabilities(&self) -> Result<Vec<crate::types::Capability>> {
        let url = "/v1/account/capabilities".to_string();
        let mut resp: crate::types::ListAccountCapability = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(serde_json::Value::String(s)) = o.get("id") {
                        page = s.to_string();
                    }
                }
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
        Ok(data.to_vec())
    }

    /**
    * This function performs a `GET` to the `/v1/account/capabilities/{capability}` endpoint.
    *
    * <p>Retrieves information about the specified Account Capability.</p>
    *
    * **Parameters:**
    *
    * * `capability: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_capabilities_capability(
        &self,
        capability: &str,
    ) -> Result<crate::types::Capability> {
        let url = format!(
            "/v1/account/capabilities/{}",
            crate::progenitor_support::encode_path(capability),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/capabilities/{capability}` endpoint.
    *
    * <p>Updates an existing Account Capability.</p>
    *
    * **Parameters:**
    *
    * * `capability: &str` -- The account's country.
    */
    pub async fn post_capabilities_capability(
        &self,
        capability: &str,
    ) -> Result<crate::types::Capability> {
        let url = format!(
            "/v1/account/capabilities/{}",
            crate::progenitor_support::encode_path(capability),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/account/external_accounts` endpoint.
    *
    * <p>List external accounts for an account.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_external(
        &self,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> Result<Vec<crate::types::DataAnyOf>> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/account/external_accounts?{}", query_);

        let resp: crate::types::ExternalAccounts = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }

    /**
    * This function performs a `GET` to the `/v1/account/external_accounts` endpoint.
    *
    * As opposed to `get_external`, this function returns all the pages of the request at once.
    *
    * <p>List external accounts for an account.</p>
    */
    pub async fn get_all_external(&self) -> Result<Vec<crate::types::DataAnyOf>> {
        let url = "/v1/account/external_accounts".to_string();
        let mut resp: crate::types::ExternalAccounts = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(serde_json::Value::String(s)) = o.get("id") {
                        page = s.to_string();
                    }
                }
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
        Ok(data.to_vec())
    }

    /**
    * This function performs a `POST` to the `/v1/account/external_accounts` endpoint.
    *
    * <p>Create an external account for a given account.</p>
    */
    pub async fn post_external(&self) -> Result<crate::types::DataAnyOf> {
        let url = "/v1/account/external_accounts".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/account/external_accounts/{id}` endpoint.
    *
    * <p>Retrieve a specified external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_external_account(&self, id: &str) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/account/external_accounts/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/external_accounts/{id}` endpoint.
    *
    * <p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>
    *
    * <p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_external_account(&self, id: &str) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/account/external_accounts/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/account/external_accounts/{id}` endpoint.
    *
    * <p>Delete a specified external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_external(
        &self,
        id: &str,
    ) -> Result<crate::types::DeletedExternalAccountAnyOf> {
        let url = format!(
            "/v1/account/external_accounts/{}",
            crate::progenitor_support::encode_path(id),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/login_links` endpoint.
    *
    * <p>Creates a single-use login link for an Express account to access their Stripe dashboard.</p>
    *
    * <p><strong>You may only create login links for <a href="/docs/connect/express-accounts">Express accounts</a> connected to your platform</strong>.</p>
    */
    pub async fn post_login_link(&self) -> Result<crate::types::LoginLink> {
        let url = "/v1/account/login_links".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/account/people` endpoint.
    *
    * <p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `relationship: &str` -- Filters on the list of people returned based on the person's relationship to the account's company.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_people(
        &self,
        ending_before: &str,
        limit: i64,
        _relationship: &str,
        starting_after: &str,
    ) -> Result<Vec<crate::types::Person>> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/account/people?{}", query_);

        let resp: crate::types::GetAccountPeopleResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }

    /**
    * This function performs a `GET` to the `/v1/account/people` endpoint.
    *
    * As opposed to `get_people`, this function returns all the pages of the request at once.
    *
    * <p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    */
    pub async fn get_all_people(&self, _relationship: &str) -> Result<Vec<crate::types::Person>> {
        let url = "/v1/account/people".to_string();
        let mut resp: crate::types::GetAccountPeopleResponse = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(serde_json::Value::String(s)) = o.get("id") {
                        page = s.to_string();
                    }
                }
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
        Ok(data.to_vec())
    }

    /**
    * This function performs a `POST` to the `/v1/account/people` endpoint.
    *
    * <p>Creates a new person.</p>
    */
    pub async fn post_people(&self) -> Result<crate::types::Person> {
        let url = "/v1/account/people".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/account/people/{person}` endpoint.
    *
    * <p>Retrieves an existing person.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `person: &str` -- The account's country.
    */
    pub async fn get_people_person(&self, person: &str) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/account/people/{}",
            crate::progenitor_support::encode_path(person),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/people/{person}` endpoint.
    *
    * <p>Updates an existing person.</p>
    *
    * **Parameters:**
    *
    * * `person: &str` -- The account's country.
    */
    pub async fn post_people_person(&self, person: &str) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/account/people/{}",
            crate::progenitor_support::encode_path(person),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/account/people/{person}` endpoint.
    *
    * <p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    *
    * **Parameters:**
    *
    * * `person: &str` -- The account's country.
    */
    pub async fn delete_people_person(&self, person: &str) -> Result<crate::types::DeletedPerson> {
        let url = format!(
            "/v1/account/people/{}",
            crate::progenitor_support::encode_path(person),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/account/persons` endpoint.
    *
    * <p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `relationship: &str` -- Filters on the list of people returned based on the person's relationship to the account's company.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_persons(
        &self,
        ending_before: &str,
        limit: i64,
        _relationship: &str,
        starting_after: &str,
    ) -> Result<Vec<crate::types::Person>> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/account/persons?{}", query_);

        let resp: crate::types::GetAccountPeopleResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }

    /**
    * This function performs a `GET` to the `/v1/account/persons` endpoint.
    *
    * As opposed to `get_persons`, this function returns all the pages of the request at once.
    *
    * <p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    */
    pub async fn get_all_persons(&self, _relationship: &str) -> Result<Vec<crate::types::Person>> {
        let url = "/v1/account/persons".to_string();
        let mut resp: crate::types::GetAccountPeopleResponse = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(serde_json::Value::String(s)) = o.get("id") {
                        page = s.to_string();
                    }
                }
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
        Ok(data.to_vec())
    }

    /**
    * This function performs a `POST` to the `/v1/account/persons` endpoint.
    *
    * <p>Creates a new person.</p>
    */
    pub async fn post_person(&self) -> Result<crate::types::Person> {
        let url = "/v1/account/persons".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/account/persons/{person}` endpoint.
    *
    * <p>Retrieves an existing person.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `person: &str` -- The account's country.
    */
    pub async fn get_persons_person(&self, person: &str) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/account/persons/{}",
            crate::progenitor_support::encode_path(person),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/persons/{person}` endpoint.
    *
    * <p>Updates an existing person.</p>
    *
    * **Parameters:**
    *
    * * `person: &str` -- The account's country.
    */
    pub async fn post_persons_person(&self, person: &str) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/account/persons/{}",
            crate::progenitor_support::encode_path(person),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/account/persons/{person}` endpoint.
    *
    * <p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    *
    * **Parameters:**
    *
    * * `person: &str` -- The account's country.
    */
    pub async fn delete_persons_person(&self, person: &str) -> Result<crate::types::DeletedPerson> {
        let url = format!(
            "/v1/account/persons/{}",
            crate::progenitor_support::encode_path(person),
        );

        self.client.delete(&url, None).await
    }
}

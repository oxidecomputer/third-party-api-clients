use anyhow::Result;

use crate::Client;

pub struct V1 {
    pub client: Client,
}

impl V1 {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        V1 { client }
    }

    /**
    * This function performs a `POST` to the `/v1/3d_secure` endpoint.
    *
    * <p>Initiate 3D Secure authentication.</p>
    */
    pub async fn post_3d_secure(&self) -> Result<crate::types::ThreeDSecure> {
        let url = "/v1/3d_secure".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/3d_secure/{three_d_secure}` endpoint.
    *
    * <p>Retrieves a 3D Secure object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `three_d_secure: &str` -- The account's country.
    */
    pub async fn get_3d_secure_three_d(
        &self,
        expand: &[String],
        three_d_secure: &str,
    ) -> Result<crate::types::ThreeDSecure> {
        let url = format!(
            "/v1/3d_secure/{}",
            crate::progenitor_support::encode_path(&three_d_secure.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn get_account(&self, expand: &[String]) -> Result<crate::types::Account> {
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
    pub async fn post_account(&self) -> Result<crate::types::Account> {
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
    pub async fn delete_account(&self) -> Result<crate::types::DeletedAccount> {
        let url = "/v1/account".to_string();
        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/bank_accounts` endpoint.
    *
    * <p>Create an external account for a given account.</p>
    */
    pub async fn post_account_bank(&self) -> Result<crate::types::DataAnyOf> {
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
    pub async fn get_account_bank_account(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/account/bank_accounts/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
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
    pub async fn post_account_bank_v_1(&self, id: &str) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/account/bank_accounts/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
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
    pub async fn delete_account_bank_accounts(
        &self,
        id: &str,
    ) -> Result<crate::types::DeletedExternalAccountAnyOf> {
        let url = format!(
            "/v1/account/bank_accounts/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
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
    pub async fn get_account_capabilitie(
        &self,
        expand: &[String],
    ) -> Result<crate::types::ListAccountCapability> {
        let url = "/v1/account/capabilities".to_string();
        self.client.get(&url, None).await
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
    pub async fn get_account_capabilities_capability(
        &self,
        capability: &str,
        expand: &[String],
    ) -> Result<crate::types::Capability> {
        let url = format!(
            "/v1/account/capabilities/{}",
            crate::progenitor_support::encode_path(&capability.to_string()),
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
    pub async fn post_account_capabilities_capability(
        &self,
        capability: &str,
    ) -> Result<crate::types::Capability> {
        let url = format!(
            "/v1/account/capabilities/{}",
            crate::progenitor_support::encode_path(&capability.to_string()),
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
    pub async fn get_account_external_account(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::ExternalAccounts> {
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

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/external_accounts` endpoint.
    *
    * <p>Create an external account for a given account.</p>
    */
    pub async fn post_account_external(&self) -> Result<crate::types::DataAnyOf> {
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
    pub async fn get_account_external_account_v_1(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/account/external_accounts/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
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
    pub async fn post_account_external_v_1(&self, id: &str) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/account/external_accounts/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
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
    pub async fn delete_account_external_accounts(
        &self,
        id: &str,
    ) -> Result<crate::types::DeletedExternalAccountAnyOf> {
        let url = format!(
            "/v1/account/external_accounts/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
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
    pub async fn post_account_login_link(&self) -> Result<crate::types::LoginLink> {
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
    pub async fn get_account_people(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        relationship: &str,
        starting_after: &str,
    ) -> Result<crate::types::GetAccountPeopleResponse> {
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

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/people` endpoint.
    *
    * <p>Creates a new person.</p>
    */
    pub async fn post_account_people(&self) -> Result<crate::types::Person> {
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
    pub async fn get_account_people_person(
        &self,
        expand: &[String],
        person: &str,
    ) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/account/people/{}",
            crate::progenitor_support::encode_path(&person.to_string()),
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
    pub async fn post_account_people_person(&self, person: &str) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/account/people/{}",
            crate::progenitor_support::encode_path(&person.to_string()),
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
    pub async fn delete_account_people_person(
        &self,
        person: &str,
    ) -> Result<crate::types::DeletedPerson> {
        let url = format!(
            "/v1/account/people/{}",
            crate::progenitor_support::encode_path(&person.to_string()),
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
    pub async fn get_account_persons(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        relationship: &str,
        starting_after: &str,
    ) -> Result<crate::types::GetAccountPeopleResponse> {
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

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account/persons` endpoint.
    *
    * <p>Creates a new person.</p>
    */
    pub async fn post_account_person(&self) -> Result<crate::types::Person> {
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
    pub async fn get_account_persons_person(
        &self,
        expand: &[String],
        person: &str,
    ) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/account/persons/{}",
            crate::progenitor_support::encode_path(&person.to_string()),
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
    pub async fn post_account_persons_person(&self, person: &str) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/account/persons/{}",
            crate::progenitor_support::encode_path(&person.to_string()),
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
    pub async fn delete_account_persons_person(
        &self,
        person: &str,
    ) -> Result<crate::types::DeletedPerson> {
        let url = format!(
            "/v1/account/persons/{}",
            crate::progenitor_support::encode_path(&person.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/account_links` endpoint.
    *
    * <p>Creates an AccountLink object that includes a single-use Stripe URL that the platform can redirect their user to in order to take them through the Connect Onboarding flow.</p>
    */
    pub async fn post_account_link(&self) -> Result<crate::types::AccountLink> {
        let url = "/v1/account_links".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts` endpoint.
    *
    * <p>Returns a list of accounts connected to your platform via <a href="/docs/connect">Connect</a>. If you’re not a platform, the list is empty.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_accounts(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetAccountsResponse> {
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
        let url = format!("/v1/accounts?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts` endpoint.
    *
    * <p>With <a href="/docs/connect">Connect</a>, you can create Stripe accounts for your users.
    * To do this, you’ll first need to <a href="https://dashboard.stripe.com/account/applications/settings">register your platform</a>.</p>
    */
    pub async fn post_account_v_1(&self) -> Result<crate::types::Account> {
        let url = "/v1/accounts".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}` endpoint.
    *
    * <p>Retrieves the details of an account.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_accounts_account(
        &self,
        account: &str,
        expand: &[String],
    ) -> Result<crate::types::Account> {
        let url = format!(
            "/v1/accounts/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}` endpoint.
    *
    * <p>Updates a <a href="/docs/connect/accounts">connected account</a> by setting the values of the parameters passed. Any parameters not provided are left unchanged. Most parameters can be changed only for Custom accounts. (These are marked <strong>Custom Only</strong> below.) Parameters marked <strong>Custom and Express</strong> are not supported for Standard accounts.</p>
    *
    * <p>To update your own account, use the <a href="https://dashboard.stripe.com/account">Dashboard</a>. Refer to our <a href="/docs/connect/updating-accounts">Connect</a> documentation to learn more about updating accounts.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    */
    pub async fn post_accounts_account(&self, account: &str) -> Result<crate::types::Account> {
        let url = format!(
            "/v1/accounts/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/accounts/{account}` endpoint.
    *
    * <p>With <a href="/docs/connect">Connect</a>, you can delete accounts you manage.</p>
    *
    * <p>Accounts created using test-mode keys can be deleted at any time. Standard accounts created using live-mode keys cannot be deleted. Custom or Express accounts created using live-mode keys can only be deleted once all balances are zero.</p>
    *
    * <p>If you want to delete your own account, use the <a href="https://dashboard.stripe.com/account">account information tab in your account settings</a> instead.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    */
    pub async fn delete_accounts_account(
        &self,
        account: &str,
    ) -> Result<crate::types::DeletedAccount> {
        let url = format!(
            "/v1/accounts/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/bank_accounts` endpoint.
    *
    * <p>Create an external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    */
    pub async fn post_accounts_account_bank(
        &self,
        account: &str,
    ) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/accounts/{}/bank_accounts",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}/bank_accounts/{id}` endpoint.
    *
    * <p>Retrieve a specified external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_accounts_account_bank(
        &self,
        account: &str,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/accounts/{}/bank_accounts/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/bank_accounts/{id}` endpoint.
    *
    * <p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>
    *
    * <p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn post_accounts_account_bank_v_1(
        &self,
        account: &str,
        id: &str,
    ) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/accounts/{}/bank_accounts/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/accounts/{account}/bank_accounts/{id}` endpoint.
    *
    * <p>Delete a specified external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_accounts_account_bank(
        &self,
        account: &str,
        id: &str,
    ) -> Result<crate::types::DeletedExternalAccountAnyOf> {
        let url = format!(
            "/v1/accounts/{}/bank_accounts/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}/capabilities` endpoint.
    *
    * <p>Returns a list of capabilities associated with the account. The capabilities are returned sorted by creation date, with the most recent capability appearing first.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_accounts_account_capabilitie(
        &self,
        account: &str,
        expand: &[String],
    ) -> Result<crate::types::ListAccountCapability> {
        let url = format!(
            "/v1/accounts/{}/capabilities",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}/capabilities/{capability}` endpoint.
    *
    * <p>Retrieves information about the specified Account Capability.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `capability: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_accounts_account_capabilities_capability(
        &self,
        account: &str,
        capability: &str,
        expand: &[String],
    ) -> Result<crate::types::Capability> {
        let url = format!(
            "/v1/accounts/{}/capabilities/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&capability.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/capabilities/{capability}` endpoint.
    *
    * <p>Updates an existing Account Capability.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `capability: &str` -- The account's country.
    */
    pub async fn post_accounts_account_capabilities_capability(
        &self,
        account: &str,
        capability: &str,
    ) -> Result<crate::types::Capability> {
        let url = format!(
            "/v1/accounts/{}/capabilities/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&capability.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}/external_accounts` endpoint.
    *
    * <p>List external accounts for an account.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_accounts_account_external(
        &self,
        account: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::ExternalAccounts> {
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
        let url = format!(
            "/v1/accounts/{}/external_accounts?{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/external_accounts` endpoint.
    *
    * <p>Create an external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    */
    pub async fn post_accounts_account_external(
        &self,
        account: &str,
    ) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/accounts/{}/external_accounts",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}/external_accounts/{id}` endpoint.
    *
    * <p>Retrieve a specified external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_accounts_account_external_v_1(
        &self,
        account: &str,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/accounts/{}/external_accounts/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/external_accounts/{id}` endpoint.
    *
    * <p>Updates the metadata, account holder name, account holder type of a bank account belonging to a <a href="/docs/connect/custom-accounts">Custom account</a>, and optionally sets it as the default for its currency. Other bank account details are not editable by design.</p>
    *
    * <p>You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn post_accounts_account_external_v_1(
        &self,
        account: &str,
        id: &str,
    ) -> Result<crate::types::DataAnyOf> {
        let url = format!(
            "/v1/accounts/{}/external_accounts/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/accounts/{account}/external_accounts/{id}` endpoint.
    *
    * <p>Delete a specified external account for a given account.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_accounts_account_external(
        &self,
        account: &str,
        id: &str,
    ) -> Result<crate::types::DeletedExternalAccountAnyOf> {
        let url = format!(
            "/v1/accounts/{}/external_accounts/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/login_links` endpoint.
    *
    * <p>Creates a single-use login link for an Express account to access their Stripe dashboard.</p>
    *
    * <p><strong>You may only create login links for <a href="/docs/connect/express-accounts">Express accounts</a> connected to your platform</strong>.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    */
    pub async fn post_accounts_account_login_link(
        &self,
        account: &str,
    ) -> Result<crate::types::LoginLink> {
        let url = format!(
            "/v1/accounts/{}/login_links",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}/people` endpoint.
    *
    * <p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `relationship: &str` -- Filters on the list of people returned based on the person's relationship to the account's company.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_accounts_account_people(
        &self,
        account: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        relationship: &str,
        starting_after: &str,
    ) -> Result<crate::types::GetAccountPeopleResponse> {
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
        let url = format!(
            "/v1/accounts/{}/people?{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/people` endpoint.
    *
    * <p>Creates a new person.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    */
    pub async fn post_accounts_account_people(
        &self,
        account: &str,
    ) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/accounts/{}/people",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}/people/{person}` endpoint.
    *
    * <p>Retrieves an existing person.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `person: &str` -- The account's country.
    */
    pub async fn get_accounts_account_people_person(
        &self,
        account: &str,
        expand: &[String],
        person: &str,
    ) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/accounts/{}/people/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&person.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/people/{person}` endpoint.
    *
    * <p>Updates an existing person.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `person: &str` -- The account's country.
    */
    pub async fn post_accounts_account_people_person(
        &self,
        account: &str,
        person: &str,
    ) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/accounts/{}/people/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&person.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/accounts/{account}/people/{person}` endpoint.
    *
    * <p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `person: &str` -- The account's country.
    */
    pub async fn delete_accounts_account_people_person(
        &self,
        account: &str,
        person: &str,
    ) -> Result<crate::types::DeletedPerson> {
        let url = format!(
            "/v1/accounts/{}/people/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&person.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}/persons` endpoint.
    *
    * <p>Returns a list of people associated with the account’s legal entity. The people are returned sorted by creation date, with the most recent people appearing first.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `relationship: &str` -- Filters on the list of people returned based on the person's relationship to the account's company.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_accounts_account_persons(
        &self,
        account: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        relationship: &str,
        starting_after: &str,
    ) -> Result<crate::types::GetAccountPeopleResponse> {
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
        let url = format!(
            "/v1/accounts/{}/persons?{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/persons` endpoint.
    *
    * <p>Creates a new person.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    */
    pub async fn post_accounts_account_person(
        &self,
        account: &str,
    ) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/accounts/{}/persons",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/accounts/{account}/persons/{person}` endpoint.
    *
    * <p>Retrieves an existing person.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `person: &str` -- The account's country.
    */
    pub async fn get_accounts_account_persons_person(
        &self,
        account: &str,
        expand: &[String],
        person: &str,
    ) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/accounts/{}/persons/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&person.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/persons/{person}` endpoint.
    *
    * <p>Updates an existing person.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `person: &str` -- The account's country.
    */
    pub async fn post_accounts_account_persons_person(
        &self,
        account: &str,
        person: &str,
    ) -> Result<crate::types::Person> {
        let url = format!(
            "/v1/accounts/{}/persons/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&person.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/accounts/{account}/persons/{person}` endpoint.
    *
    * <p>Deletes an existing person’s relationship to the account’s legal entity. Any person with a relationship for an account can be deleted through the API, except if the person is the <code>account_opener</code>. If your integration is using the <code>executive</code> parameter, you cannot delete the only verified <code>executive</code> on file.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    * * `person: &str` -- The account's country.
    */
    pub async fn delete_accounts_account_persons_person(
        &self,
        account: &str,
        person: &str,
    ) -> Result<crate::types::DeletedPerson> {
        let url = format!(
            "/v1/accounts/{}/persons/{}",
            crate::progenitor_support::encode_path(&account.to_string()),
            crate::progenitor_support::encode_path(&person.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/accounts/{account}/reject` endpoint.
    *
    * <p>With <a href="/docs/connect">Connect</a>, you may flag accounts as suspicious.</p>
    *
    * <p>Test-mode Custom and Express accounts can be rejected at any time. Accounts created using live-mode keys may only be rejected once all balances are zero.</p>
    *
    * **Parameters:**
    *
    * * `account: &str` -- The account's country.
    */
    pub async fn post_accounts_account_reject(
        &self,
        account: &str,
    ) -> Result<crate::types::Account> {
        let url = format!(
            "/v1/accounts/{}/reject",
            crate::progenitor_support::encode_path(&account.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/apple_pay/domains` endpoint.
    *
    * <p>List apple pay domains.</p>
    *
    * **Parameters:**
    *
    * * `domain_name: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_apple_pay_domain(
        &self,
        domain_name: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::ApplePayDomainList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !domain_name.is_empty() {
            query_args.push(("domain_name".to_string(), domain_name.to_string()));
        }
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
        let url = format!("/v1/apple_pay/domains?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/apple_pay/domains` endpoint.
    *
    * <p>Create an apple pay domain.</p>
    */
    pub async fn post_apple_pay_domain(&self) -> Result<crate::types::ApplePayDomain> {
        let url = "/v1/apple_pay/domains".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/apple_pay/domains/{domain}` endpoint.
    *
    * <p>Retrieve an apple pay domain.</p>
    *
    * **Parameters:**
    *
    * * `domain: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_apple_pay_domains_domain(
        &self,
        domain: &str,
        expand: &[String],
    ) -> Result<crate::types::ApplePayDomain> {
        let url = format!(
            "/v1/apple_pay/domains/{}",
            crate::progenitor_support::encode_path(&domain.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/apple_pay/domains/{domain}` endpoint.
    *
    * <p>Delete an apple pay domain.</p>
    *
    * **Parameters:**
    *
    * * `domain: &str` -- The account's country.
    */
    pub async fn delete_apple_pay_domains_domain(
        &self,
        domain: &str,
    ) -> Result<crate::types::DeletedApplePayDomain> {
        let url = format!(
            "/v1/apple_pay/domains/{}",
            crate::progenitor_support::encode_path(&domain.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/application_fees` endpoint.
    *
    * <p>Returns a list of application fees you’ve previously collected. The application fees are returned in sorted order, with the most recent fees appearing first.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- Only return application fees for the charge specified by this charge ID.
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_application_fees(
        &self,
        charge: &str,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetApplicationFeesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
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
        let url = format!("/v1/application_fees?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/application_fees/{fee}/refunds/{id}` endpoint.
    *
    * <p>By default, you can see the 10 most recent refunds stored directly on the application fee object, but you can also retrieve details about a specific refund stored on the application fee.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `fee: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_application_fees_fee_refund(
        &self,
        expand: &[String],
        fee: &str,
        id: &str,
    ) -> Result<crate::types::FeeRefund> {
        let url = format!(
            "/v1/application_fees/{}/refunds/{}",
            crate::progenitor_support::encode_path(&fee.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/application_fees/{fee}/refunds/{id}` endpoint.
    *
    * <p>Updates the specified application fee refund by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * <p>This request only accepts metadata as an argument.</p>
    *
    * **Parameters:**
    *
    * * `fee: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn post_application_fees_fee_refund(
        &self,
        fee: &str,
        id: &str,
    ) -> Result<crate::types::FeeRefund> {
        let url = format!(
            "/v1/application_fees/{}/refunds/{}",
            crate::progenitor_support::encode_path(&fee.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/application_fees/{id}` endpoint.
    *
    * <p>Retrieves the details of an application fee that your account has collected. The same information is returned when refunding the application fee.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_application_fee(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::PlatformFee> {
        let url = format!(
            "/v1/application_fees/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/application_fees/{id}/refund` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_application_fees_refund(
        &self,
        id: &str,
    ) -> Result<crate::types::PlatformFee> {
        let url = format!(
            "/v1/application_fees/{}/refund",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/application_fees/{id}/refunds` endpoint.
    *
    * <p>You can see a list of the refunds belonging to a specific application fee. Note that the 10 most recent refunds are always available by default on the application fee object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_application_fees_refund(
        &self,
        ending_before: &str,
        expand: &[String],
        id: &str,
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::Refunds> {
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
        let url = format!(
            "/v1/application_fees/{}/refunds?{}",
            crate::progenitor_support::encode_path(&id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/application_fees/{id}/refunds` endpoint.
    *
    * <p>Refunds an application fee that has previously been collected but not yet refunded.
    * Funds will be refunded to the Stripe account from which the fee was originally collected.</p>
    *
    * <p>You can optionally refund only part of an application fee.
    * You can do so multiple times, until the entire fee has been refunded.</p>
    *
    * <p>Once entirely refunded, an application fee can’t be refunded again.
    * This method will raise an error when called on an already-refunded application fee,
    * or when trying to refund more money than is left on an application fee.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_application_fees_refund_v_1(
        &self,
        id: &str,
    ) -> Result<crate::types::FeeRefund> {
        let url = format!(
            "/v1/application_fees/{}/refunds",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/balance` endpoint.
    *
    * <p>Retrieves the current account balance, based on the authentication that was used to make the request.
    *  For a sample request, see <a href="/docs/connect/account-balances#accounting-for-negative-balances">Accounting for negative balances</a>.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_balance(&self, expand: &[String]) -> Result<crate::types::Balance> {
        let url = "/v1/balance".to_string();
        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/balance/history` endpoint.
    *
    * <p>Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth). The transactions are returned in sorted order, with the most recent transactions appearing first.</p>
    *
    * <p>Note that this endpoint was previously called “Balance history” and used the path <code>/v1/balance/history</code>.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `currency: &str` -- Only return transactions in a certain currency. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `payout: &str` -- For automatic Stripe payouts only, only returns transactions that were paid out on the specified payout ID.
    * * `source: &str` -- Only returns the original transaction.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `type_: &str` -- Only returns transactions of the given type. One of: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    */
    pub async fn get_balance_history(
        &self,
        created: &str,
        currency: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        payout: &str,
        source: &str,
        starting_after: &str,
        type_: &str,
    ) -> Result<crate::types::BalanceTransactionsList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payout.is_empty() {
            query_args.push(("payout".to_string(), payout.to_string()));
        }
        if !source.is_empty() {
            query_args.push(("source".to_string(), source.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/balance/history?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/balance/history/{id}` endpoint.
    *
    * <p>Retrieves the balance transaction with the given ID.</p>
    *
    * <p>Note that this endpoint previously used the path <code>/v1/balance/history/:id</code>.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_balance_history_v_1(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::BalanceTransaction> {
        let url = format!(
            "/v1/balance/history/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/balance_transactions` endpoint.
    *
    * <p>Returns a list of transactions that have contributed to the Stripe account balance (e.g., charges, transfers, and so forth). The transactions are returned in sorted order, with the most recent transactions appearing first.</p>
    *
    * <p>Note that this endpoint was previously called “Balance history” and used the path <code>/v1/balance/history</code>.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `currency: &str` -- Only return transactions in a certain currency. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `payout: &str` -- For automatic Stripe payouts only, only returns transactions that were paid out on the specified payout ID.
    * * `source: &str` -- Only returns the original transaction.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `type_: &str` -- Only returns transactions of the given type. One of: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `payment`, `payment_failure_refund`, `payment_refund`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`.
    */
    pub async fn get_balance_transaction(
        &self,
        created: &str,
        currency: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        payout: &str,
        source: &str,
        starting_after: &str,
        type_: &str,
    ) -> Result<crate::types::BalanceTransactionsList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payout.is_empty() {
            query_args.push(("payout".to_string(), payout.to_string()));
        }
        if !source.is_empty() {
            query_args.push(("source".to_string(), source.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/balance_transactions?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/balance_transactions/{id}` endpoint.
    *
    * <p>Retrieves the balance transaction with the given ID.</p>
    *
    * <p>Note that this endpoint previously used the path <code>/v1/balance/history/:id</code>.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_balance_transaction_v_1(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::BalanceTransaction> {
        let url = format!(
            "/v1/balance_transactions/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/billing_portal/configurations` endpoint.
    *
    * <p>Returns a list of configurations that describe the functionality of the customer portal.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Only return configurations that are active or inactive (e.g., pass `true` to only list active configurations).
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `is_default: bool` -- Only return the default or non-default configurations (e.g., pass `true` to only list the default configuration).
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_billing_portal_configurations(
        &self,
        active: bool,
        ending_before: &str,
        expand: &[String],
        is_default: bool,
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetBillingPortalConfigurationsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if is_default {
            query_args.push(("is_default".to_string(), is_default.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/billing_portal/configurations?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/billing_portal/configurations` endpoint.
    *
    * <p>Creates a configuration that describes the functionality and behavior of a PortalSession</p>
    */
    pub async fn post_billing_portal_configuration(
        &self,
    ) -> Result<crate::types::PortalConfiguration> {
        let url = "/v1/billing_portal/configurations".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/billing_portal/configurations/{configuration}` endpoint.
    *
    * <p>Retrieves a configuration that describes the functionality of the customer portal.</p>
    *
    * **Parameters:**
    *
    * * `configuration: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_billing_portal_configurations_configuration(
        &self,
        configuration: &str,
        expand: &[String],
    ) -> Result<crate::types::PortalConfiguration> {
        let url = format!(
            "/v1/billing_portal/configurations/{}",
            crate::progenitor_support::encode_path(&configuration.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/billing_portal/configurations/{configuration}` endpoint.
    *
    * <p>Updates a configuration that describes the functionality of the customer portal.</p>
    *
    * **Parameters:**
    *
    * * `configuration: &str` -- The account's country.
    */
    pub async fn post_billing_portal_configurations_configuration(
        &self,
        configuration: &str,
    ) -> Result<crate::types::PortalConfiguration> {
        let url = format!(
            "/v1/billing_portal/configurations/{}",
            crate::progenitor_support::encode_path(&configuration.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/billing_portal/sessions` endpoint.
    *
    * <p>Creates a session of the customer portal.</p>
    */
    pub async fn post_billing_portal_session(&self) -> Result<crate::types::PortalSession> {
        let url = "/v1/billing_portal/sessions".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/bitcoin/receivers` endpoint.
    *
    * <p>Returns a list of your receivers. Receivers are returned sorted by creation date, with the most recently created receivers appearing first.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Whether the account can create live charges.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `filled: bool` -- Whether the account can create live charges.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `uncaptured_funds: bool` -- Whether the account can create live charges.
    */
    pub async fn get_bitcoin_receivers(
        &self,
        active: bool,
        ending_before: &str,
        expand: &[String],
        filled: bool,
        limit: i64,
        starting_after: &str,
        uncaptured_funds: bool,
    ) -> Result<crate::types::GetBitcoinReceiversResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if filled {
            query_args.push(("filled".to_string(), filled.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if uncaptured_funds {
            query_args.push(("uncaptured_funds".to_string(), uncaptured_funds.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/bitcoin/receivers?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/bitcoin/receivers/{id}` endpoint.
    *
    * <p>Retrieves the Bitcoin receiver with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_bitcoin_receiver(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::BitcoinReceiver> {
        let url = format!(
            "/v1/bitcoin/receivers/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/bitcoin/receivers/{receiver}/transactions` endpoint.
    *
    * <p>List bitcoin transacitons for a given receiver.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- Only return transactions for the customer specified by this customer ID.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `receiver: &str` -- The account's country.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_bitcoin_receivers_receiver_transaction(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        receiver: &str,
        starting_after: &str,
    ) -> Result<crate::types::Transactions> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
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
        let url = format!(
            "/v1/bitcoin/receivers/{}/transactions?{}",
            crate::progenitor_support::encode_path(&receiver.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/bitcoin/transactions` endpoint.
    *
    * <p>List bitcoin transacitons for a given receiver.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- Only return transactions for the customer specified by this customer ID.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `receiver: &str` -- The account's country.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_bitcoin_transaction(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        receiver: &str,
        starting_after: &str,
    ) -> Result<crate::types::Transactions> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !receiver.is_empty() {
            query_args.push(("receiver".to_string(), receiver.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/bitcoin/transactions?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/charges` endpoint.
    *
    * <p>Returns a list of charges you’ve previously created. The charges are returned in sorted order, with the most recent charges appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `customer: &str` -- Only return charges for the customer specified by this customer ID.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `payment_intent: &str` -- Only return charges that were created by the PaymentIntent specified by this PaymentIntent ID.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `transfer_group: &str` -- Only return charges for this transfer group.
    */
    pub async fn get_charge(
        &self,
        created: &str,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        payment_intent: &str,
        starting_after: &str,
        transfer_group: &str,
    ) -> Result<crate::types::Charges> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !transfer_group.is_empty() {
            query_args.push(("transfer_group".to_string(), transfer_group.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/charges?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/charges` endpoint.
    *
    * <p>To charge a credit card or other payment source, you create a <code>Charge</code> object. If your API key is in test mode, the supplied payment source (e.g., card) won’t actually be charged, although everything else will occur as if in live mode. (Stripe assumes that the charge would have completed successfully).</p>
    */
    pub async fn post_charge(&self) -> Result<crate::types::Charge> {
        let url = "/v1/charges".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/charges/search` endpoint.
    *
    * <p>Search for charges you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
    * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
    * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
    * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
    * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for charges](https://stripe.com/docs/search#query-fields-for-charges).
    */
    pub async fn get_charges_search(
        &self,
        expand: &[String],
        limit: i64,
        page: &str,
        query: &str,
    ) -> Result<crate::types::SearchResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/charges/search?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/charges/{charge}` endpoint.
    *
    * <p>Retrieves the details of a charge that has previously been created. Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information. The same information is returned when creating or refunding the charge.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_charges_charge(
        &self,
        charge: &str,
        expand: &[String],
    ) -> Result<crate::types::Charge> {
        let url = format!(
            "/v1/charges/{}",
            crate::progenitor_support::encode_path(&charge.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/charges/{charge}` endpoint.
    *
    * <p>Updates the specified charge by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    */
    pub async fn post_charges_charge(&self, charge: &str) -> Result<crate::types::Charge> {
        let url = format!(
            "/v1/charges/{}",
            crate::progenitor_support::encode_path(&charge.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/charges/{charge}/capture` endpoint.
    *
    * <p>Capture the payment of an existing, uncaptured, charge. This is the second half of the two-step payment flow, where first you <a href="#create_charge">created a charge</a> with the capture option set to false.</p>
    *
    * <p>Uncaptured payments expire a set number of days after they are created (<a href="/docs/charges/placing-a-hold">7 by default</a>). If they are not captured by that point in time, they will be marked as refunded and will no longer be capturable.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    */
    pub async fn post_charges_charge_capture(&self, charge: &str) -> Result<crate::types::Charge> {
        let url = format!(
            "/v1/charges/{}/capture",
            crate::progenitor_support::encode_path(&charge.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/charges/{charge}/dispute` endpoint.
    *
    * <p>Retrieve a dispute for a specified charge.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_charges_charge_dispute(
        &self,
        charge: &str,
        expand: &[String],
    ) -> Result<crate::types::Dispute> {
        let url = format!(
            "/v1/charges/{}/dispute",
            crate::progenitor_support::encode_path(&charge.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/charges/{charge}/dispute` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    */
    pub async fn post_charges_charge_dispute(&self, charge: &str) -> Result<crate::types::Dispute> {
        let url = format!(
            "/v1/charges/{}/dispute",
            crate::progenitor_support::encode_path(&charge.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/charges/{charge}/dispute/close` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    */
    pub async fn post_charges_charge_dispute_close(
        &self,
        charge: &str,
    ) -> Result<crate::types::Dispute> {
        let url = format!(
            "/v1/charges/{}/dispute/close",
            crate::progenitor_support::encode_path(&charge.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/charges/{charge}/refund` endpoint.
    *
    * <p>When you create a new refund, you must specify a Charge or a PaymentIntent object on which to create it.</p>
    *
    * <p>Creating a new refund will refund a charge that has previously been created but not yet refunded.
    * Funds will be refunded to the credit or debit card that was originally charged.</p>
    *
    * <p>You can optionally refund only part of a charge.
    * You can do so multiple times, until the entire charge has been refunded.</p>
    *
    * <p>Once entirely refunded, a charge can’t be refunded again.
    * This method will raise an error when called on an already-refunded charge,
    * or when trying to refund more money than is left on a charge.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    */
    pub async fn post_charges_charge_refund(&self, charge: &str) -> Result<crate::types::Charge> {
        let url = format!(
            "/v1/charges/{}/refund",
            crate::progenitor_support::encode_path(&charge.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/charges/{charge}/refunds` endpoint.
    *
    * <p>You can see a list of the refunds belonging to a specific charge. Note that the 10 most recent refunds are always available by default on the charge object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_charges_charge_refund(
        &self,
        charge: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::RefundList> {
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
        let url = format!(
            "/v1/charges/{}/refunds?{}",
            crate::progenitor_support::encode_path(&charge.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/charges/{charge}/refunds` endpoint.
    *
    * <p>Create a refund.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    */
    pub async fn post_charges_charge_refund_v_1(
        &self,
        charge: &str,
    ) -> Result<crate::types::Refund> {
        let url = format!(
            "/v1/charges/{}/refunds",
            crate::progenitor_support::encode_path(&charge.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/charges/{charge}/refunds/{refund}` endpoint.
    *
    * <p>Retrieves the details of an existing refund.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `refund: &str` -- The account's country.
    */
    pub async fn get_charges_charge_refunds_refund(
        &self,
        charge: &str,
        expand: &[String],
        refund: &str,
    ) -> Result<crate::types::Refund> {
        let url = format!(
            "/v1/charges/{}/refunds/{}",
            crate::progenitor_support::encode_path(&charge.to_string()),
            crate::progenitor_support::encode_path(&refund.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/charges/{charge}/refunds/{refund}` endpoint.
    *
    * <p>Update a specified refund.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- The account's country.
    * * `refund: &str` -- The account's country.
    */
    pub async fn post_charges_charge_refunds_refund(
        &self,
        charge: &str,
        refund: &str,
    ) -> Result<crate::types::Refund> {
        let url = format!(
            "/v1/charges/{}/refunds/{}",
            crate::progenitor_support::encode_path(&charge.to_string()),
            crate::progenitor_support::encode_path(&refund.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/checkout/sessions` endpoint.
    *
    * <p>Returns a list of Checkout Sessions.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `payment_intent: &str` -- Only return the Checkout Session for the PaymentIntent specified.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `subscription: &str` -- Only return the Checkout Session for the subscription specified.
    */
    pub async fn get_checkout_session(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        payment_intent: &str,
        starting_after: &str,
        subscription: &str,
    ) -> Result<crate::types::PaymentPagesCheckoutSessionList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/checkout/sessions?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/checkout/sessions` endpoint.
    *
    * <p>Creates a Session object.</p>
    */
    pub async fn post_checkout_session(&self) -> Result<crate::types::Session> {
        let url = "/v1/checkout/sessions".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/checkout/sessions/{session}` endpoint.
    *
    * <p>Retrieves a Session object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `session: &str` -- The account's country.
    */
    pub async fn get_checkout_sessions_session(
        &self,
        expand: &[String],
        session: &str,
    ) -> Result<crate::types::Session> {
        let url = format!(
            "/v1/checkout/sessions/{}",
            crate::progenitor_support::encode_path(&session.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/checkout/sessions/{session}/expire` endpoint.
    *
    * <p>A Session can be expired when it is in one of these statuses: <code>open</code> </p>
    *
    * <p>After it expires, a customer can’t complete a Session and customers loading the Session see a message saying the Session is expired.</p>
    *
    * **Parameters:**
    *
    * * `session: &str` -- The account's country.
    */
    pub async fn post_checkout_sessions_session_expire(
        &self,
        session: &str,
    ) -> Result<crate::types::Session> {
        let url = format!(
            "/v1/checkout/sessions/{}/expire",
            crate::progenitor_support::encode_path(&session.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/checkout/sessions/{session}/line_items` endpoint.
    *
    * <p>When retrieving a Checkout Session, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `session: &str` -- The account's country.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_checkout_sessions_session_line_item(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        session: &str,
        starting_after: &str,
    ) -> Result<crate::types::LineItems> {
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
        let url = format!(
            "/v1/checkout/sessions/{}/line_items?{}",
            crate::progenitor_support::encode_path(&session.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/country_specs` endpoint.
    *
    * <p>Lists all Country Spec objects available in the API.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_country_specs(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetCountrySpecsResponse> {
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
        let url = format!("/v1/country_specs?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/country_specs/{country}` endpoint.
    *
    * <p>Returns a Country Spec for a given Country code.</p>
    *
    * **Parameters:**
    *
    * * `country: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_country_spec(
        &self,
        country: &str,
        expand: &[String],
    ) -> Result<crate::types::CountrySpec> {
        let url = format!(
            "/v1/country_specs/{}",
            crate::progenitor_support::encode_path(&country.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/coupons` endpoint.
    *
    * <p>Returns a list of your coupons.</p>
    *
    * **Parameters:**
    *
    * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_coupons(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetCouponsResponse> {
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
        let url = format!("/v1/coupons?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/coupons` endpoint.
    *
    * <p>You can create coupons easily via the <a href="https://dashboard.stripe.com/coupons">coupon management</a> page of the Stripe dashboard. Coupon creation is also accessible via the API if you need to create coupons on the fly.</p>
    *
    * <p>A coupon has either a <code>percent_off</code> or an <code>amount_off</code> and <code>currency</code>. If you set an <code>amount_off</code>, that amount will be subtracted from any invoice’s subtotal. For example, an invoice with a subtotal of <currency>100</currency> will have a final total of <currency>0</currency> if a coupon with an <code>amount_off</code> of <amount>200</amount> is applied to it and an invoice with a subtotal of <currency>300</currency> will have a final total of <currency>100</currency> if a coupon with an <code>amount_off</code> of <amount>200</amount> is applied to it.</p>
    */
    pub async fn post_coupon(&self) -> Result<crate::types::Coupon> {
        let url = "/v1/coupons".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/coupons/{coupon}` endpoint.
    *
    * <p>Retrieves the coupon with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `coupon: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_coupons_coupon(
        &self,
        coupon: &str,
        expand: &[String],
    ) -> Result<crate::types::Coupon> {
        let url = format!(
            "/v1/coupons/{}",
            crate::progenitor_support::encode_path(&coupon.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/coupons/{coupon}` endpoint.
    *
    * <p>Updates the metadata of a coupon. Other coupon details (currency, duration, amount_off) are, by design, not editable.</p>
    *
    * **Parameters:**
    *
    * * `coupon: &str` -- The account's country.
    */
    pub async fn post_coupons_coupon(&self, coupon: &str) -> Result<crate::types::Coupon> {
        let url = format!(
            "/v1/coupons/{}",
            crate::progenitor_support::encode_path(&coupon.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/coupons/{coupon}` endpoint.
    *
    * <p>You can delete coupons via the <a href="https://dashboard.stripe.com/coupons">coupon management</a> page of the Stripe dashboard. However, deleting a coupon does not affect any customers who have already applied the coupon; it means that new customers can’t redeem the coupon. You can also delete coupons via the API.</p>
    *
    * **Parameters:**
    *
    * * `coupon: &str` -- The account's country.
    */
    pub async fn delete_coupons_coupon(&self, coupon: &str) -> Result<crate::types::DeletedCoupon> {
        let url = format!(
            "/v1/coupons/{}",
            crate::progenitor_support::encode_path(&coupon.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/credit_notes` endpoint.
    *
    * <p>Returns a list of credit notes.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- Only return credit notes for the customer specified by this customer ID.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `invoice: &str` -- Only return credit notes for the invoice specified by this invoice ID.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_credit_note(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        invoice: &str,
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::CreditNotesList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/credit_notes?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/credit_notes` endpoint.
    *
    * <p>Issue a credit note to adjust the amount of a finalized invoice. For a <code>status=open</code> invoice, a credit note reduces
    * its <code>amount_due</code>. For a <code>status=paid</code> invoice, a credit note does not affect its <code>amount_due</code>. Instead, it can result
    * in any combination of the following:</p>
    *
    * <ul>
    * <li>Refund: create a new refund (using <code>refund_amount</code>) or link an existing refund (using <code>refund</code>).</li>
    * <li>Customer balance credit: credit the customer’s balance (using <code>credit_amount</code>) which will be automatically applied to their next invoice when it’s finalized.</li>
    * <li>Outside of Stripe credit: record the amount that is or will be credited outside of Stripe (using <code>out_of_band_amount</code>).</li>
    * </ul>
    *
    * <p>For post-payment credit notes the sum of the refund, credit and outside of Stripe amounts must equal the credit note total.</p>
    *
    * <p>You may issue multiple credit notes for an invoice. Each credit note will increment the invoice’s <code>pre_payment_credit_notes_amount</code>
    * or <code>post_payment_credit_notes_amount</code> depending on its <code>status</code> at the time of credit note creation.</p>
    */
    pub async fn post_credit_note(&self) -> Result<crate::types::CreditNote> {
        let url = "/v1/credit_notes".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/credit_notes/preview` endpoint.
    *
    * <p>Get a preview of a credit note without creating it.</p>
    *
    * **Parameters:**
    *
    * * `amount: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
    * * `credit_amount: i64` -- The integer amount in %s representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `invoice: &str` -- The account's country.
    * * `lines: &[String]` -- Line items that make up the credit note.
    * * `memo: &str` -- The credit note's memo appears on the credit note PDF.
    * * `metadata: &str` -- Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format. Individual keys can be unset by posting an empty value to them. All keys can be unset by posting an empty value to `metadata`.
    * * `out_of_band_amount: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
    * * `reason: crate::types::Reason` -- Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    * * `refund: &str` -- ID of an existing refund to link this credit note to.
    * * `refund_amount: i64` -- The integer amount in %s representing the amount to refund. If set, a refund will be created for the charge associated with the invoice.
    */
    pub async fn get_credit_notes_preview(
        &self,
        amount: i64,
        credit_amount: i64,
        expand: &[String],
        invoice: &str,
        lines: &[String],
        memo: &str,
        metadata: &str,
        out_of_band_amount: i64,
        reason: crate::types::Reason,
        refund: &str,
        refund_amount: i64,
    ) -> Result<crate::types::CreditNote> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if amount > 0 {
            query_args.push(("amount".to_string(), amount.to_string()));
        }
        if credit_amount > 0 {
            query_args.push(("credit_amount".to_string(), credit_amount.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        if !memo.is_empty() {
            query_args.push(("memo".to_string(), memo.to_string()));
        }
        if out_of_band_amount > 0 {
            query_args.push((
                "out_of_band_amount".to_string(),
                out_of_band_amount.to_string(),
            ));
        }
        if !reason.to_string().is_empty() {
            query_args.push(("reason".to_string(), reason.to_string()));
        }
        if !refund.is_empty() {
            query_args.push(("refund".to_string(), refund.to_string()));
        }
        if refund_amount > 0 {
            query_args.push(("refund_amount".to_string(), refund_amount.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/credit_notes/preview?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/credit_notes/preview/lines` endpoint.
    *
    * <p>When retrieving a credit note preview, you’ll get a <strong>lines</strong> property containing the first handful of those items. This URL you can retrieve the full (paginated) list of line items.</p>
    *
    * **Parameters:**
    *
    * * `amount: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
    * * `credit_amount: i64` -- The integer amount in %s representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `invoice: &str` -- The account's country.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `lines: &[String]` -- Line items that make up the credit note.
    * * `memo: &str` -- The credit note's memo appears on the credit note PDF.
    * * `metadata: &str` -- Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format. Individual keys can be unset by posting an empty value to them. All keys can be unset by posting an empty value to `metadata`.
    * * `out_of_band_amount: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
    * * `reason: crate::types::Reason` -- Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    * * `refund: &str` -- ID of an existing refund to link this credit note to.
    * * `refund_amount: i64` -- The integer amount in %s representing the amount to refund. If set, a refund will be created for the charge associated with the invoice.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_credit_notes_preview_line(
        &self,
        amount: i64,
        credit_amount: i64,
        ending_before: &str,
        expand: &[String],
        invoice: &str,
        limit: i64,
        lines: &[String],
        memo: &str,
        metadata: &str,
        out_of_band_amount: i64,
        reason: crate::types::Reason,
        refund: &str,
        refund_amount: i64,
        starting_after: &str,
    ) -> Result<crate::types::Lines> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if amount > 0 {
            query_args.push(("amount".to_string(), amount.to_string()));
        }
        if credit_amount > 0 {
            query_args.push(("credit_amount".to_string(), credit_amount.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !memo.is_empty() {
            query_args.push(("memo".to_string(), memo.to_string()));
        }
        if out_of_band_amount > 0 {
            query_args.push((
                "out_of_band_amount".to_string(),
                out_of_band_amount.to_string(),
            ));
        }
        if !reason.to_string().is_empty() {
            query_args.push(("reason".to_string(), reason.to_string()));
        }
        if !refund.is_empty() {
            query_args.push(("refund".to_string(), refund.to_string()));
        }
        if refund_amount > 0 {
            query_args.push(("refund_amount".to_string(), refund_amount.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/credit_notes/preview/lines?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/credit_notes/{credit_note}/lines` endpoint.
    *
    * <p>When retrieving a credit note, you’ll get a <strong>lines</strong> property containing the the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    *
    * **Parameters:**
    *
    * * `credit_note: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_credit_notes_note_line(
        &self,
        credit_note: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::Lines> {
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
        let url = format!(
            "/v1/credit_notes/{}/lines?{}",
            crate::progenitor_support::encode_path(&credit_note.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/credit_notes/{id}` endpoint.
    *
    * <p>Retrieves the credit note object with the given identifier.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_credit_note_v_1(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::CreditNote> {
        let url = format!(
            "/v1/credit_notes/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/credit_notes/{id}` endpoint.
    *
    * <p>Updates an existing credit note.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_credit_note_v_1(&self, id: &str) -> Result<crate::types::CreditNote> {
        let url = format!(
            "/v1/credit_notes/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/credit_notes/{id}/void` endpoint.
    *
    * <p>Marks a credit note as void. Learn more about <a href="/docs/billing/invoices/credit-notes#voiding">voiding credit notes</a>.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_credit_notes_void(&self, id: &str) -> Result<crate::types::CreditNote> {
        let url = format!(
            "/v1/credit_notes/{}/void",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers` endpoint.
    *
    * <p>Returns a list of your customers. The customers are returned sorted by creation date, with the most recent customers appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `email: &str` -- A case-sensitive filter on the list based on the customer's `email` field. The value must be a string.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `test_clock: &str` -- Provides a list of customers that are associated with the specified test clock. The response will not include customers with test clocks if this parameter is not set.
    */
    pub async fn get_customers(
        &self,
        created: &str,
        email: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        test_clock: &str,
    ) -> Result<crate::types::GetCustomersResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !test_clock.is_empty() {
            query_args.push(("test_clock".to_string(), test_clock.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/customers?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers` endpoint.
    *
    * <p>Creates a new customer object.</p>
    */
    pub async fn post_customer(&self) -> Result<crate::types::Customer> {
        let url = "/v1/customers".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/search` endpoint.
    *
    * <p>Search for customers you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
    * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
    * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
    * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
    * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for customers](https://stripe.com/docs/search#query-fields-for-customers).
    */
    pub async fn get_customers_search(
        &self,
        expand: &[String],
        limit: i64,
        page: &str,
        query: &str,
    ) -> Result<crate::types::SearchResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/customers/search?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}` endpoint.
    *
    * <p>Retrieves a Customer object.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_customers_customer(
        &self,
        customer: &str,
        expand: &[String],
    ) -> Result<crate::types::GetCustomersCustomerResponseAnyOf> {
        let url = format!(
            "/v1/customers/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}` endpoint.
    *
    * <p>Updates the specified customer by setting the values of the parameters passed. Any parameters not provided will be left unchanged. For example, if you pass the <strong>source</strong> parameter, that becomes the customer’s active source (e.g., a card) to be used for all charges in the future. When you update a customer to a new valid card source by passing the <strong>source</strong> parameter: for each of the customer’s current subscriptions, if the subscription bills automatically and is in the <code>past_due</code> state, then the latest open invoice for the subscription with automatic collection enabled will be retried. This retry will not count as an automatic retry, and will not affect the next regularly scheduled payment for the invoice. Changing the <strong>default_source</strong> for a customer will not trigger this behavior.</p>
    *
    * <p>This request accepts mostly the same arguments as the customer creation call.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    */
    pub async fn post_customers_customer(&self, customer: &str) -> Result<crate::types::Customer> {
        let url = format!(
            "/v1/customers/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/customers/{customer}` endpoint.
    *
    * <p>Permanently deletes a customer. It cannot be undone. Also immediately cancels any active subscriptions on the customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    */
    pub async fn delete_customers_customer(
        &self,
        customer: &str,
    ) -> Result<crate::types::DeletedCustomer> {
        let url = format!(
            "/v1/customers/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/balance_transactions` endpoint.
    *
    * <p>Returns a list of transactions that updated the customer’s <a href="/docs/billing/customer/balance">balances</a>.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_customers_customer_balance_transaction(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::CustomerBalanceTransactionList> {
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
        let url = format!(
            "/v1/customers/{}/balance_transactions?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/balance_transactions` endpoint.
    *
    * <p>Creates an immutable transaction that updates the customer’s credit <a href="/docs/billing/customer/balance">balance</a>.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    */
    pub async fn post_customers_customer_balance_transaction(
        &self,
        customer: &str,
    ) -> Result<crate::types::CustomerBalanceTransaction> {
        let url = format!(
            "/v1/customers/{}/balance_transactions",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/balance_transactions/{transaction}` endpoint.
    *
    * <p>Retrieves a specific customer balance transaction that updated the customer’s <a href="/docs/billing/customer/balance">balances</a>.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `transaction: &str` -- The account's country.
    */
    pub async fn get_customers_customer_balance_transactions_transaction(
        &self,
        customer: &str,
        expand: &[String],
        transaction: &str,
    ) -> Result<crate::types::CustomerBalanceTransaction> {
        let url = format!(
            "/v1/customers/{}/balance_transactions/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&transaction.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/balance_transactions/{transaction}` endpoint.
    *
    * <p>Most credit balance transaction fields are immutable, but you may update its <code>description</code> and <code>metadata</code>.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `transaction: &str` -- The account's country.
    */
    pub async fn post_customers_customer_balance_transactions_transaction(
        &self,
        customer: &str,
        transaction: &str,
    ) -> Result<crate::types::CustomerBalanceTransaction> {
        let url = format!(
            "/v1/customers/{}/balance_transactions/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&transaction.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/bank_accounts` endpoint.
    *
    * <p>You can see a list of the bank accounts belonging to a Customer. Note that the 10 most recent sources are always available by default on the Customer. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional bank accounts.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_customers_customer_bank_account(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::BankAccountList> {
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
        let url = format!(
            "/v1/customers/{}/bank_accounts?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/bank_accounts` endpoint.
    *
    * <p>When you create a new credit card, you must specify a customer or recipient on which to create it.</p>
    *
    * <p>If the card’s owner has no default card, then the new card will become the default.
    * However, if the owner already has a default, then it will not change.
    * To change the default, you should <a href="/docs/api#update_customer">update the customer</a> to have a new <code>default_source</code>.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    */
    pub async fn post_customers_customer_bank_account(
        &self,
        customer: &str,
    ) -> Result<crate::types::PaymentSourceAnyOf> {
        let url = format!(
            "/v1/customers/{}/bank_accounts",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/bank_accounts/{id}` endpoint.
    *
    * <p>By default, you can see the 10 most recent sources stored on a Customer directly on the object, but you can also retrieve details about a specific bank account stored on the Stripe account.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_customers_customer_bank_account_v_1(
        &self,
        customer: &str,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::BankAccount> {
        let url = format!(
            "/v1/customers/{}/bank_accounts/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/bank_accounts/{id}` endpoint.
    *
    * <p>Update a specified source for a given customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn post_customers_customer_bank_account_v_1(
        &self,
        customer: &str,
        id: &str,
    ) -> Result<crate::types::SourceAnyOf> {
        let url = format!(
            "/v1/customers/{}/bank_accounts/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/customers/{customer}/bank_accounts/{id}` endpoint.
    *
    * <p>Delete a specified source for a given customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_customers_customer_bank_accounts(
        &self,
        customer: &str,
        id: &str,
    ) -> Result<crate::types::DeleteCustomersCustomerCardsResponseAnyOf> {
        let url = format!(
            "/v1/customers/{}/bank_accounts/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/bank_accounts/{id}/verify` endpoint.
    *
    * <p>Verify a specified bank account for a given customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn post_customers_customer_bank_accounts_verify(
        &self,
        customer: &str,
        id: &str,
    ) -> Result<crate::types::BankAccount> {
        let url = format!(
            "/v1/customers/{}/bank_accounts/{}/verify",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/cards` endpoint.
    *
    * <p>You can see a list of the cards belonging to a customer.
    * Note that the 10 most recent sources are always available on the <code>Customer</code> object.
    * If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional cards.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_customers_customer_card(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::Cards> {
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
        let url = format!(
            "/v1/customers/{}/cards?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/cards` endpoint.
    *
    * <p>When you create a new credit card, you must specify a customer or recipient on which to create it.</p>
    *
    * <p>If the card’s owner has no default card, then the new card will become the default.
    * However, if the owner already has a default, then it will not change.
    * To change the default, you should <a href="/docs/api#update_customer">update the customer</a> to have a new <code>default_source</code>.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    */
    pub async fn post_customers_customer_card(
        &self,
        customer: &str,
    ) -> Result<crate::types::PaymentSourceAnyOf> {
        let url = format!(
            "/v1/customers/{}/cards",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/cards/{id}` endpoint.
    *
    * <p>You can always see the 10 most recent cards directly on a customer; this method lets you retrieve details about a specific card stored on the customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_customers_customer_card_v_1(
        &self,
        customer: &str,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::Card> {
        let url = format!(
            "/v1/customers/{}/cards/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/cards/{id}` endpoint.
    *
    * <p>Update a specified source for a given customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn post_customers_customer_card_v_1(
        &self,
        customer: &str,
        id: &str,
    ) -> Result<crate::types::SourceAnyOf> {
        let url = format!(
            "/v1/customers/{}/cards/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/customers/{customer}/cards/{id}` endpoint.
    *
    * <p>Delete a specified source for a given customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_customers_customer_cards(
        &self,
        customer: &str,
        id: &str,
    ) -> Result<crate::types::DeleteCustomersCustomerCardsResponseAnyOf> {
        let url = format!(
            "/v1/customers/{}/cards/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/discount` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_customers_customer_discount(
        &self,
        customer: &str,
        expand: &[String],
    ) -> Result<crate::types::DiscountData> {
        let url = format!(
            "/v1/customers/{}/discount",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/customers/{customer}/discount` endpoint.
    *
    * <p>Removes the currently applied discount on a customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    */
    pub async fn delete_customers_customer_discount(
        &self,
        customer: &str,
    ) -> Result<crate::types::DeletedDiscount> {
        let url = format!(
            "/v1/customers/{}/discount",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/payment_methods` endpoint.
    *
    * <p>Returns a list of PaymentMethods for a given Customer</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `type_: crate::types::GetCustomersCustomerPaymentMethodsType` -- A required filter on the list, based on the object `type` field.
    */
    pub async fn get_customers_customer_payment_method(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        type_: crate::types::GetCustomersCustomerPaymentMethodsType,
    ) -> Result<crate::types::PaymentFlowsMethodList> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/v1/customers/{}/payment_methods?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/sources` endpoint.
    *
    * <p>List sources for a specified customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `object: &str` -- Filter sources according to a particular object type.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_customers_customer_source(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        object: &str,
        starting_after: &str,
    ) -> Result<crate::types::Sources> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !object.is_empty() {
            query_args.push(("object".to_string(), object.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/v1/customers/{}/sources?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/sources` endpoint.
    *
    * <p>When you create a new credit card, you must specify a customer or recipient on which to create it.</p>
    *
    * <p>If the card’s owner has no default card, then the new card will become the default.
    * However, if the owner already has a default, then it will not change.
    * To change the default, you should <a href="/docs/api#update_customer">update the customer</a> to have a new <code>default_source</code>.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    */
    pub async fn post_customers_customer_source(
        &self,
        customer: &str,
    ) -> Result<crate::types::PaymentSourceAnyOf> {
        let url = format!(
            "/v1/customers/{}/sources",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/sources/{id}` endpoint.
    *
    * <p>Retrieve a specified source for a given customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_customers_customer_source_v_1(
        &self,
        customer: &str,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::PaymentSourceAnyOf> {
        let url = format!(
            "/v1/customers/{}/sources/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/sources/{id}` endpoint.
    *
    * <p>Update a specified source for a given customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn post_customers_customer_source_v_1(
        &self,
        customer: &str,
        id: &str,
    ) -> Result<crate::types::SourceAnyOf> {
        let url = format!(
            "/v1/customers/{}/sources/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/customers/{customer}/sources/{id}` endpoint.
    *
    * <p>Delete a specified source for a given customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_customers_customer_sources(
        &self,
        customer: &str,
        id: &str,
    ) -> Result<crate::types::DeleteCustomersCustomerCardsResponseAnyOf> {
        let url = format!(
            "/v1/customers/{}/sources/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/sources/{id}/verify` endpoint.
    *
    * <p>Verify a specified bank account for a given customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn post_customers_customer_sources_verify(
        &self,
        customer: &str,
        id: &str,
    ) -> Result<crate::types::BankAccount> {
        let url = format!(
            "/v1/customers/{}/sources/{}/verify",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/subscriptions` endpoint.
    *
    * <p>You can see a list of the customer’s active subscriptions. Note that the 10 most recent active subscriptions are always available by default on the customer object. If you need more than those 10, you can use the limit and starting_after parameters to page through additional subscriptions.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_customers_customer_subscription(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::Subscriptions> {
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
        let url = format!(
            "/v1/customers/{}/subscriptions?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/subscriptions` endpoint.
    *
    * <p>Creates a new subscription on an existing customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    */
    pub async fn post_customers_customer_subscription(
        &self,
        customer: &str,
    ) -> Result<crate::types::Subscription> {
        let url = format!(
            "/v1/customers/{}/subscriptions",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}` endpoint.
    *
    * <p>Retrieves the subscription with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `subscription_exposed_id: &str` -- The account's country.
    */
    pub async fn get_customers_customer_subscriptions_subscription_exposed(
        &self,
        customer: &str,
        expand: &[String],
        subscription_exposed_id: &str,
    ) -> Result<crate::types::Subscription> {
        let url = format!(
            "/v1/customers/{}/subscriptions/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&subscription_exposed_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}` endpoint.
    *
    * <p>Updates an existing subscription on a customer to match the specified parameters. When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes. To preview how the proration will be calculated, use the <a href="#upcoming_invoice">upcoming invoice</a> endpoint.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `subscription_exposed_id: &str` -- The account's country.
    */
    pub async fn post_customers_customer_subscriptions_subscription_exposed(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> Result<crate::types::Subscription> {
        let url = format!(
            "/v1/customers/{}/subscriptions/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&subscription_exposed_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}` endpoint.
    *
    * <p>Cancels a customer’s subscription. If you set the <code>at_period_end</code> parameter to <code>true</code>, the subscription will remain active until the end of the period, at which point it will be canceled and not renewed. Otherwise, with the default <code>false</code> value, the subscription is terminated immediately. In either case, the customer will not be charged again for the subscription.</p>
    *
    * <p>Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually <a href="#delete_invoiceitem">deleted</a>. If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period. But if the subscription is set to cancel immediately, pending prorations will be removed.</p>
    *
    * <p>By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer. This is intended to prevent unexpected payment attempts after the customer has canceled a subscription. However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed. Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `subscription_exposed_id: &str` -- The account's country.
    */
    pub async fn delete_customers_customer_subscriptions_subscription_exposed(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> Result<crate::types::Subscription> {
        let url = format!(
            "/v1/customers/{}/subscriptions/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&subscription_exposed_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}/discount` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `subscription_exposed_id: &str` -- The account's country.
    */
    pub async fn get_customers_customer_subscriptions_subscription_exposed_discount(
        &self,
        customer: &str,
        expand: &[String],
        subscription_exposed_id: &str,
    ) -> Result<crate::types::DiscountData> {
        let url = format!(
            "/v1/customers/{}/subscriptions/{}/discount",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&subscription_exposed_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}/discount` endpoint.
    *
    * <p>Removes the currently applied discount on a customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `subscription_exposed_id: &str` -- The account's country.
    */
    pub async fn delete_customers_customer_subscriptions_subscription_exposed_discount(
        &self,
        customer: &str,
        subscription_exposed_id: &str,
    ) -> Result<crate::types::DeletedDiscount> {
        let url = format!(
            "/v1/customers/{}/subscriptions/{}/discount",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&subscription_exposed_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/tax_ids` endpoint.
    *
    * <p>Returns a list of tax IDs for a customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_customers_customer_tax_id(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::TaxIds> {
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
        let url = format!(
            "/v1/customers/{}/tax_ids?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/customers/{customer}/tax_ids` endpoint.
    *
    * <p>Creates a new <code>TaxID</code> object for a customer.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    */
    pub async fn post_customers_customer_tax_id(
        &self,
        customer: &str,
    ) -> Result<crate::types::TaxId> {
        let url = format!(
            "/v1/customers/{}/tax_ids",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/customers/{customer}/tax_ids/{id}` endpoint.
    *
    * <p>Retrieves the <code>TaxID</code> object with the given identifier.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_customers_customer_tax_id_v_1(
        &self,
        customer: &str,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::TaxId> {
        let url = format!(
            "/v1/customers/{}/tax_ids/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/customers/{customer}/tax_ids/{id}` endpoint.
    *
    * <p>Deletes an existing <code>TaxID</code> object.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The account's country.
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_customers_customer_tax_ids(
        &self,
        customer: &str,
        id: &str,
    ) -> Result<crate::types::DeletedTaxId> {
        let url = format!(
            "/v1/customers/{}/tax_ids/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/disputes` endpoint.
    *
    * <p>Returns a list of your disputes.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- Only return disputes associated to the charge specified by this charge ID.
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `payment_intent: &str` -- Only return disputes associated to the PaymentIntent specified by this PaymentIntent ID.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_disputes(
        &self,
        charge: &str,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        payment_intent: &str,
        starting_after: &str,
    ) -> Result<crate::types::GetDisputesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/disputes?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/disputes/{dispute}` endpoint.
    *
    * <p>Retrieves the dispute with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `dispute: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_disputes_dispute(
        &self,
        dispute: &str,
        expand: &[String],
    ) -> Result<crate::types::Dispute> {
        let url = format!(
            "/v1/disputes/{}",
            crate::progenitor_support::encode_path(&dispute.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/disputes/{dispute}` endpoint.
    *
    * <p>When you get a dispute, contacting your customer is always the best first step. If that doesn’t work, you can submit evidence to help us resolve the dispute in your favor. You can do this in your <a href="https://dashboard.stripe.com/disputes">dashboard</a>, but if you prefer, you can use the API to submit evidence programmatically.</p>
    *
    * <p>Depending on your dispute type, different evidence fields will give you a better chance of winning your dispute. To figure out which evidence fields to provide, see our <a href="/docs/disputes/categories">guide to dispute types</a>.</p>
    *
    * **Parameters:**
    *
    * * `dispute: &str` -- The account's country.
    */
    pub async fn post_disputes_dispute(&self, dispute: &str) -> Result<crate::types::Dispute> {
        let url = format!(
            "/v1/disputes/{}",
            crate::progenitor_support::encode_path(&dispute.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/disputes/{dispute}/close` endpoint.
    *
    * <p>Closing the dispute for a charge indicates that you do not have any evidence to submit and are essentially dismissing the dispute, acknowledging it as lost.</p>
    *
    * <p>The status of the dispute will change from <code>needs_response</code> to <code>lost</code>. <em>Closing a dispute is irreversible</em>.</p>
    *
    * **Parameters:**
    *
    * * `dispute: &str` -- The account's country.
    */
    pub async fn post_disputes_dispute_close(
        &self,
        dispute: &str,
    ) -> Result<crate::types::Dispute> {
        let url = format!(
            "/v1/disputes/{}/close",
            crate::progenitor_support::encode_path(&dispute.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/ephemeral_keys` endpoint.
    *
    * <p>Creates a short-lived API key for a given resource.</p>
    */
    pub async fn post_ephemeral_key(&self) -> Result<crate::types::EphemeralKey> {
        let url = "/v1/ephemeral_keys".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/ephemeral_keys/{key}` endpoint.
    *
    * <p>Invalidates a short-lived API key for a given resource.</p>
    *
    * **Parameters:**
    *
    * * `key: &str` -- The account's country.
    */
    pub async fn delete_ephemeral_keys_key(&self, key: &str) -> Result<crate::types::EphemeralKey> {
        let url = format!(
            "/v1/ephemeral_keys/{}",
            crate::progenitor_support::encode_path(&key.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/events` endpoint.
    *
    * <p>List events, going back up to 30 days. Each event data is rendered according to Stripe API version at its creation time, specified in <a href="/docs/api/events/object">event object</a> <code>api_version</code> attribute (not according to your current Stripe API version or <code>Stripe-Version</code> header).</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `delivery_success: bool` -- Filter events by whether all webhooks were successfully delivered. If false, events which are still pending or have failed all delivery attempts to a webhook endpoint will be returned.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `type_: &str` -- A string containing a specific event name, or group of events using * as a wildcard. The list will be filtered to include only events with a matching event property.
    * * `types: &[String]` -- An array of up to 20 strings containing specific event names. The list will be filtered to include only events with a matching event property. You may pass either `type` or `types`, but not both.
    */
    pub async fn get_event(
        &self,
        created: &str,
        delivery_success: bool,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        type_: &str,
        types: &[String],
    ) -> Result<crate::types::NotificationEventList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if delivery_success {
            query_args.push(("delivery_success".to_string(), delivery_success.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/events?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/events/{id}` endpoint.
    *
    * <p>Retrieves the details of an event. Supply the unique identifier of the event, which you might have received in a webhook.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_event_v_1(&self, expand: &[String], id: &str) -> Result<crate::types::Event> {
        let url = format!(
            "/v1/events/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/exchange_rates` endpoint.
    *
    * <p>Returns a list of objects that contain the rates at which foreign currencies are converted to one another. Only shows the currencies for which Stripe supports.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is the currency that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with the exchange rate for currency X your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and total number of supported payout currencies, and the default is the max.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is the currency that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with the exchange rate for currency X, your subsequent call can include `starting_after=X` in order to fetch the next page of the list.
    */
    pub async fn get_exchange_rates(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetExchangeRatesResponse> {
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
        let url = format!("/v1/exchange_rates?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/exchange_rates/{rate_id}` endpoint.
    *
    * <p>Retrieves the exchange rates from the given currency to every supported currency.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `rate_id: &str` -- The account's country.
    */
    pub async fn get_exchange_rates_rate(
        &self,
        expand: &[String],
        rate_id: &str,
    ) -> Result<crate::types::ExchangeRate> {
        let url = format!(
            "/v1/exchange_rates/{}",
            crate::progenitor_support::encode_path(&rate_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/file_links` endpoint.
    *
    * <p>Returns a list of file links.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `expired: bool` -- Filter links by their expiration status. By default, all links are returned.
    * * `file: &str` -- Only return links for the given file.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_file_link(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        expired: bool,
        file: &str,
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::Links> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if expired {
            query_args.push(("expired".to_string(), expired.to_string()));
        }
        if !file.is_empty() {
            query_args.push(("file".to_string(), file.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/file_links?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/file_links` endpoint.
    *
    * <p>Creates a new file link object.</p>
    */
    pub async fn post_file_link(&self) -> Result<crate::types::FileLink> {
        let url = "/v1/file_links".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/file_links/{link}` endpoint.
    *
    * <p>Retrieves the file link with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `link: &str` -- The account's country.
    */
    pub async fn get_file_links_link(
        &self,
        expand: &[String],
        link: &str,
    ) -> Result<crate::types::FileLink> {
        let url = format!(
            "/v1/file_links/{}",
            crate::progenitor_support::encode_path(&link.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/file_links/{link}` endpoint.
    *
    * <p>Updates an existing file link object. Expired links can no longer be updated.</p>
    *
    * **Parameters:**
    *
    * * `link: &str` -- The account's country.
    */
    pub async fn post_file_links_link(&self, link: &str) -> Result<crate::types::FileLink> {
        let url = format!(
            "/v1/file_links/{}",
            crate::progenitor_support::encode_path(&link.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/files` endpoint.
    *
    * <p>Returns a list of the files that your account has access to. The files are returned sorted by creation date, with the most recently created files appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `purpose: crate::types::Purpose` -- The file purpose to filter queries by. If none is provided, files will not be filtered by purpose.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_files(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        purpose: crate::types::Purpose,
        starting_after: &str,
    ) -> Result<crate::types::GetFilesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !purpose.to_string().is_empty() {
            query_args.push(("purpose".to_string(), purpose.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/files?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/files` endpoint.
    *
    * <p>To upload a file to Stripe, you’ll need to send a request of type <code>multipart/form-data</code>. The request should contain the file you would like to upload, as well as the parameters for creating a file.</p>
    *
    * <p>All of Stripe’s officially supported Client libraries should have support for sending <code>multipart/form-data</code>.</p>
    */
    pub async fn post_file(&self) -> Result<crate::types::File> {
        let url = "/v1/files".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/files/{file}` endpoint.
    *
    * <p>Retrieves the details of an existing file object. Supply the unique file ID from a file, and Stripe will return the corresponding file object. To access file contents, see the <a href="/docs/file-upload#download-file-contents">File Upload Guide</a>.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `file: &str` -- The account's country.
    */
    pub async fn get_files_file(
        &self,
        expand: &[String],
        file: &str,
    ) -> Result<crate::types::File> {
        let url = format!(
            "/v1/files/{}",
            crate::progenitor_support::encode_path(&file.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/identity/verification_reports` endpoint.
    *
    * <p>List all verification reports.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `type_: crate::types::GelatoVerificationReportType` -- Only return VerificationReports of this type.
    * * `verification_session: &str` -- Only return VerificationReports created by this VerificationSession ID. It is allowed to provide a VerificationIntent ID.
    */
    pub async fn get_identity_verification_reports(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        type_: crate::types::GelatoVerificationReportType,
        verification_session: &str,
    ) -> Result<crate::types::GetIdentityVerificationReportsResponse> {
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
        if !verification_session.is_empty() {
            query_args.push((
                "verification_session".to_string(),
                verification_session.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/identity/verification_reports?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/identity/verification_reports/{report}` endpoint.
    *
    * <p>Retrieves an existing VerificationReport</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `report: &str` -- The account's country.
    */
    pub async fn get_identity_verification_reports_report(
        &self,
        expand: &[String],
        report: &str,
    ) -> Result<crate::types::GelatoVerificationReport> {
        let url = format!(
            "/v1/identity/verification_reports/{}",
            crate::progenitor_support::encode_path(&report.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/identity/verification_sessions` endpoint.
    *
    * <p>Returns a list of VerificationSessions</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::GelatoVerificationSessionStatus` -- Only return VerificationSessions with this status. [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    */
    pub async fn get_identity_verification_sessions(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        status: crate::types::GelatoVerificationSessionStatus,
    ) -> Result<crate::types::GetIdentityVerificationSessionsResponse> {
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
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/identity/verification_sessions?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/identity/verification_sessions` endpoint.
    *
    * <p>Creates a VerificationSession object.</p>
    *
    * <p>After the VerificationSession is created, display a verification modal using the session <code>client_secret</code> or send your users to the session’s <code>url</code>.</p>
    *
    * <p>If your API key is in test mode, verification checks won’t actually process, though everything else will occur as if in live mode.</p>
    *
    * <p>Related guide: <a href="/docs/identity/verify-identity-documents">Verify your users’ identity documents</a>.</p>
    */
    pub async fn post_identity_verification_session(
        &self,
    ) -> Result<crate::types::GelatoVerificationSession> {
        let url = "/v1/identity/verification_sessions".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/identity/verification_sessions/{session}` endpoint.
    *
    * <p>Retrieves the details of a VerificationSession that was previously created.</p>
    *
    * <p>When the session status is <code>requires_input</code>, you can use this method to retrieve a valid
    * <code>client_secret</code> or <code>url</code> to allow re-submission.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `session: &str` -- The account's country.
    */
    pub async fn get_identity_verification_sessions_session(
        &self,
        expand: &[String],
        session: &str,
    ) -> Result<crate::types::GelatoVerificationSession> {
        let url = format!(
            "/v1/identity/verification_sessions/{}",
            crate::progenitor_support::encode_path(&session.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/identity/verification_sessions/{session}` endpoint.
    *
    * <p>Updates a VerificationSession object.</p>
    *
    * <p>When the session status is <code>requires_input</code>, you can use this method to update the
    * verification check and options.</p>
    *
    * **Parameters:**
    *
    * * `session: &str` -- The account's country.
    */
    pub async fn post_identity_verification_sessions_session(
        &self,
        session: &str,
    ) -> Result<crate::types::GelatoVerificationSession> {
        let url = format!(
            "/v1/identity/verification_sessions/{}",
            crate::progenitor_support::encode_path(&session.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/identity/verification_sessions/{session}/cancel` endpoint.
    *
    * <p>A VerificationSession object can be canceled when it is in <code>requires_input</code> <a href="/docs/identity/how-sessions-work">status</a>.</p>
    *
    * <p>Once canceled, future submission attempts are disabled. This cannot be undone. <a href="/docs/identity/verification-sessions#cancel">Learn more</a>.</p>
    *
    * **Parameters:**
    *
    * * `session: &str` -- The account's country.
    */
    pub async fn post_identity_verification_sessions_session_cancel(
        &self,
        session: &str,
    ) -> Result<crate::types::GelatoVerificationSession> {
        let url = format!(
            "/v1/identity/verification_sessions/{}/cancel",
            crate::progenitor_support::encode_path(&session.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/identity/verification_sessions/{session}/redact` endpoint.
    *
    * <p>Redact a VerificationSession to remove all collected information from Stripe. This will redact
    * the VerificationSession and all objects related to it, including VerificationReports, Events,
    * request logs, etc.</p>
    *
    * <p>A VerificationSession object can be redacted when it is in <code>requires_input</code> or <code>verified</code>
    * <a href="/docs/identity/how-sessions-work">status</a>. Redacting a VerificationSession in <code>requires_action</code>
    * state will automatically cancel it.</p>
    *
    * <p>The redaction process may take up to four days. When the redaction process is in progress, the
    * VerificationSession’s <code>redaction.status</code> field will be set to <code>processing</code>; when the process is
    * finished, it will change to <code>redacted</code> and an <code>identity.verification_session.redacted</code> event
    * will be emitted.</p>
    *
    * <p>Redaction is irreversible. Redacted objects are still accessible in the Stripe API, but all the
    * fields that contain personal data will be replaced by the string <code>[redacted]</code> or a similar
    * placeholder. The <code>metadata</code> field will also be erased. Redacted objects cannot be updated or
    * used for any purpose.</p>
    *
    * <p><a href="/docs/identity/verification-sessions#redact">Learn more</a>.</p>
    *
    * **Parameters:**
    *
    * * `session: &str` -- The account's country.
    */
    pub async fn post_identity_verification_sessions_session_redact(
        &self,
        session: &str,
    ) -> Result<crate::types::GelatoVerificationSession> {
        let url = format!(
            "/v1/identity/verification_sessions/{}/redact",
            crate::progenitor_support::encode_path(&session.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/invoiceitems` endpoint.
    *
    * <p>Returns a list of your invoice items. Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `customer: &str` -- The identifier of the customer whose invoice items to return. If none is provided, all invoice items will be returned.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `invoice: &str` -- Only return invoice items belonging to this invoice. If none is provided, all invoice items will be returned. If specifying an invoice, no customer identifier is needed.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `pending: bool` -- Set to `true` to only show pending invoice items, which are not yet attached to any invoices. Set to `false` to only show invoice items already attached to invoices. If unspecified, no filter is applied.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_invoiceitems(
        &self,
        created: &str,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        invoice: &str,
        limit: i64,
        pending: bool,
        starting_after: &str,
    ) -> Result<crate::types::GetInvoiceitemsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if !invoice.is_empty() {
            query_args.push(("invoice".to_string(), invoice.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if pending {
            query_args.push(("pending".to_string(), pending.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/invoiceitems?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/invoiceitems` endpoint.
    *
    * <p>Creates an item to be added to a draft invoice (up to 250 items per invoice). If no invoice is specified, the item will be on the next invoice created for the customer specified.</p>
    */
    pub async fn post_invoiceitem(&self) -> Result<crate::types::InvoiceItem> {
        let url = "/v1/invoiceitems".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/invoiceitems/{invoiceitem}` endpoint.
    *
    * <p>Retrieves the invoice item with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `invoiceitem: &str` -- The account's country.
    */
    pub async fn get_invoiceitems_invoiceitem(
        &self,
        expand: &[String],
        invoiceitem: &str,
    ) -> Result<crate::types::InvoiceItem> {
        let url = format!(
            "/v1/invoiceitems/{}",
            crate::progenitor_support::encode_path(&invoiceitem.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/invoiceitems/{invoiceitem}` endpoint.
    *
    * <p>Updates the amount or description of an invoice item on an upcoming invoice. Updating an invoice item is only possible before the invoice it’s attached to is closed.</p>
    *
    * **Parameters:**
    *
    * * `invoiceitem: &str` -- The account's country.
    */
    pub async fn post_invoiceitems_invoiceitem(
        &self,
        invoiceitem: &str,
    ) -> Result<crate::types::InvoiceItem> {
        let url = format!(
            "/v1/invoiceitems/{}",
            crate::progenitor_support::encode_path(&invoiceitem.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/invoiceitems/{invoiceitem}` endpoint.
    *
    * <p>Deletes an invoice item, removing it from an invoice. Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.</p>
    *
    * **Parameters:**
    *
    * * `invoiceitem: &str` -- The account's country.
    */
    pub async fn delete_invoiceitems_invoiceitem(
        &self,
        invoiceitem: &str,
    ) -> Result<crate::types::DeletedInvoiceItem> {
        let url = format!(
            "/v1/invoiceitems/{}",
            crate::progenitor_support::encode_path(&invoiceitem.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/invoices` endpoint.
    *
    * <p>You can list all invoices, or list the invoices for a specific customer. The invoices are returned sorted by creation date, with the most recently created invoices appearing first.</p>
    *
    * **Parameters:**
    *
    * * `collection_method: crate::types::CollectionMethod` -- Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer. When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    * * `created: &str`
    * * `customer: &str` -- Only return invoices for the customer specified by this customer ID.
    * * `due_date: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::GetInvoicesStatus` -- The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`. [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    * * `subscription: &str` -- Only return invoices for the subscription specified by this subscription ID.
    */
    pub async fn get_invoice(
        &self,
        collection_method: crate::types::CollectionMethod,
        created: &str,
        customer: &str,
        due_date: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        status: crate::types::GetInvoicesStatus,
        subscription: &str,
    ) -> Result<crate::types::InvoicesList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_method.to_string().is_empty() {
            query_args.push((
                "collection_method".to_string(),
                collection_method.to_string(),
            ));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/invoices?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/invoices` endpoint.
    *
    * <p>This endpoint creates a draft invoice for a given customer. The draft invoice created pulls in all pending invoice items on that customer, including prorations. The invoice remains a draft until you <a href="#finalize_invoice">finalize</a> the invoice, which allows you to <a href="#pay_invoice">pay</a> or <a href="#send_invoice">send</a> the invoice to your customers.</p>
    */
    pub async fn post_invoice(&self) -> Result<crate::types::Invoice> {
        let url = "/v1/invoices".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/invoices/search` endpoint.
    *
    * <p>Search for invoices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
    * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
    * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
    * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
    * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for invoices](https://stripe.com/docs/search#query-fields-for-invoices).
    */
    pub async fn get_invoices_search(
        &self,
        expand: &[String],
        limit: i64,
        page: &str,
        query: &str,
    ) -> Result<crate::types::SearchResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/invoices/search?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/invoices/upcoming` endpoint.
    *
    * <p>At any time, you can preview the upcoming invoice for a customer. This will show you all the charges that are pending, including subscription renewal charges, invoice item charges, etc. It will also show you any discounts that are applicable to the invoice.</p>
    *
    * <p>Note that when you are viewing an upcoming invoice, you are simply viewing a preview – the invoice has not yet been created. As such, the upcoming invoice will not show up in invoice listing calls, and you cannot use the API to pay or edit the invoice. If you want to change the amount that your customer will be billed, you can add, remove, or update pending invoice items, or update the customer’s discount.</p>
    *
    * <p>You can preview the effects of updating a subscription, including a preview of what proration will take place. To ensure that the actual proration is calculated exactly the same as the previewed proration, you should pass a <code>proration_date</code> parameter when doing the actual subscription update. The value passed in should be the same as the <code>subscription_proration_date</code> returned on the upcoming invoice resource. The recommended way to get only the prorations being previewed is to consider only proration line items where <code>period[start]</code> is equal to the <code>subscription_proration_date</code> on the upcoming invoice resource.</p>
    *
    * **Parameters:**
    *
    * * `automatic_tax: &str` -- Settings for automatic tax lookup for this invoice preview.
    * * `coupon: &str` -- The code of the coupon to apply. If `subscription` or `subscription_items` is provided, the invoice returned will preview updating or creating a subscription with that coupon. Otherwise, it will preview applying that coupon to the customer for the next upcoming invoice from among the customer's subscriptions. The invoice can be previewed without a coupon by passing this value as an empty string.
    * * `customer: &str` -- The identifier of the customer whose upcoming invoice you'd like to retrieve.
    * * `customer_details: &str` -- Details about the customer you want to invoice or overrides for an existing customer.
    * * `discounts: &str` -- The coupons to redeem into discounts for the invoice preview. If not specified, inherits the discount from the customer or subscription. This only works for coupons directly applied to the invoice. To apply a coupon to a subscription, you must use the `coupon` parameter instead. Pass an empty string to avoid inheriting any discounts. To preview the upcoming invoice for a subscription that hasn't been created, use `coupon` instead.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `invoice_items: &[String]` -- List of invoice items to add or update in the upcoming invoice preview.
    * * `schedule: &str` -- The identifier of the unstarted schedule whose upcoming invoice you'd like to retrieve. Cannot be used with subscription or subscription fields.
    * * `subscription: &str` -- The identifier of the subscription for which you'd like to retrieve the upcoming invoice. If not provided, but a `subscription_items` is provided, you will preview creating a subscription with those items. If neither `subscription` nor `subscription_items` is provided, you will retrieve the next upcoming invoice from among the customer's subscriptions.
    * * `subscription_billing_cycle_anchor: &str` -- For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle). This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. For existing subscriptions, the value can only be set to `now` or `unchanged`.
    * * `subscription_cancel_at: &str` -- Timestamp indicating when the subscription should be scheduled to cancel. Will prorate if within the current period and prorations have been enabled using `proration_behavior`.
    * * `subscription_cancel_at_period_end: bool` -- Boolean indicating whether this subscription should cancel at the end of the current period.
    * * `subscription_cancel_now: bool` -- This simulates the subscription being canceled or expired immediately.
    * * `subscription_default_tax_rates: &str` -- If provided, the invoice returned will preview updating or creating a subscription with these default tax rates. The default tax rates will apply to any line item that does not have `tax_rates` set.
    * * `subscription_items: &[String]` -- A list of up to 20 subscription items, each with an attached price.
    * * `subscription_proration_behavior: crate::types::ProrationBehavior` -- Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes. Valid values are `create_prorations`, `none`, or `always_invoice`.
    *
    *   Passing `create_prorations` will cause proration invoice items to be created when applicable. These proration items will only be invoiced immediately under [certain conditions](https://stripe.com/docs/subscriptions/upgrading-downgrading#immediate-payment). In order to always invoice immediately for prorations, pass `always_invoice`.
    *
    *   Prorations can be disabled by passing `none`.
    * * `subscription_proration_date: i64` -- If previewing an update to a subscription, and doing proration, `subscription_proration_date` forces the proration to be calculated as though the update was done at the specified time. The time given must be within the current subscription period, and cannot be before the subscription was on its current plan. If set, `subscription`, and one of `subscription_items`, or `subscription_trial_end` are required. Also, `subscription_proration_behavior` cannot be set to 'none'.
    * * `subscription_start_date: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
    * * `subscription_trial_end: &str` -- If provided, the invoice returned will preview updating or creating a subscription with that trial end. If set, one of `subscription_items` or `subscription` is required.
    * * `subscription_trial_from_plan: bool` -- Indicates if a plan's `trial_period_days` should be applied to the subscription. Setting `subscription_trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `subscription_trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    */
    pub async fn get_invoices_upcoming(
        &self,
        automatic_tax: &str,
        coupon: &str,
        customer: &str,
        customer_details: &str,
        discounts: &str,
        expand: &[String],
        invoice_items: &[String],
        schedule: &str,
        subscription: &str,
        subscription_billing_cycle_anchor: &str,
        subscription_cancel_at: &str,
        subscription_cancel_at_period_end: bool,
        subscription_cancel_now: bool,
        subscription_default_tax_rates: &str,
        subscription_items: &[String],
        subscription_proration_behavior: crate::types::ProrationBehavior,
        subscription_proration_date: i64,
        subscription_start_date: i64,
        subscription_trial_end: &str,
        subscription_trial_from_plan: bool,
    ) -> Result<crate::types::Invoice> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !coupon.is_empty() {
            query_args.push(("coupon".to_string(), coupon.to_string()));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !schedule.is_empty() {
            query_args.push(("schedule".to_string(), schedule.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        if subscription_cancel_at_period_end {
            query_args.push((
                "subscription_cancel_at_period_end".to_string(),
                subscription_cancel_at_period_end.to_string(),
            ));
        }
        if subscription_cancel_now {
            query_args.push((
                "subscription_cancel_now".to_string(),
                subscription_cancel_now.to_string(),
            ));
        }
        if !subscription_proration_behavior.to_string().is_empty() {
            query_args.push((
                "subscription_proration_behavior".to_string(),
                subscription_proration_behavior.to_string(),
            ));
        }
        if subscription_proration_date > 0 {
            query_args.push((
                "subscription_proration_date".to_string(),
                subscription_proration_date.to_string(),
            ));
        }
        if subscription_start_date > 0 {
            query_args.push((
                "subscription_start_date".to_string(),
                subscription_start_date.to_string(),
            ));
        }
        if subscription_trial_from_plan {
            query_args.push((
                "subscription_trial_from_plan".to_string(),
                subscription_trial_from_plan.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/invoices/upcoming?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/invoices/upcoming/lines` endpoint.
    *
    * <p>When retrieving an upcoming invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    *
    * **Parameters:**
    *
    * * `automatic_tax: &str` -- Settings for automatic tax lookup for this invoice preview.
    * * `coupon: &str` -- The code of the coupon to apply. If `subscription` or `subscription_items` is provided, the invoice returned will preview updating or creating a subscription with that coupon. Otherwise, it will preview applying that coupon to the customer for the next upcoming invoice from among the customer's subscriptions. The invoice can be previewed without a coupon by passing this value as an empty string.
    * * `customer: &str` -- The identifier of the customer whose upcoming invoice you'd like to retrieve.
    * * `customer_details: &str` -- Details about the customer you want to invoice or overrides for an existing customer.
    * * `discounts: &str` -- The coupons to redeem into discounts for the invoice preview. If not specified, inherits the discount from the customer or subscription. This only works for coupons directly applied to the invoice. To apply a coupon to a subscription, you must use the `coupon` parameter instead. Pass an empty string to avoid inheriting any discounts. To preview the upcoming invoice for a subscription that hasn't been created, use `coupon` instead.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `invoice_items: &[String]` -- List of invoice items to add or update in the upcoming invoice preview.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `schedule: &str` -- The identifier of the unstarted schedule whose upcoming invoice you'd like to retrieve. Cannot be used with subscription or subscription fields.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `subscription: &str` -- The identifier of the subscription for which you'd like to retrieve the upcoming invoice. If not provided, but a `subscription_items` is provided, you will preview creating a subscription with those items. If neither `subscription` nor `subscription_items` is provided, you will retrieve the next upcoming invoice from among the customer's subscriptions.
    * * `subscription_billing_cycle_anchor: &str` -- For new subscriptions, a future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle). This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. For existing subscriptions, the value can only be set to `now` or `unchanged`.
    * * `subscription_cancel_at: &str` -- Timestamp indicating when the subscription should be scheduled to cancel. Will prorate if within the current period and prorations have been enabled using `proration_behavior`.
    * * `subscription_cancel_at_period_end: bool` -- Boolean indicating whether this subscription should cancel at the end of the current period.
    * * `subscription_cancel_now: bool` -- This simulates the subscription being canceled or expired immediately.
    * * `subscription_default_tax_rates: &str` -- If provided, the invoice returned will preview updating or creating a subscription with these default tax rates. The default tax rates will apply to any line item that does not have `tax_rates` set.
    * * `subscription_items: &[String]` -- A list of up to 20 subscription items, each with an attached price.
    * * `subscription_proration_behavior: crate::types::ProrationBehavior` -- Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes. Valid values are `create_prorations`, `none`, or `always_invoice`.
    *
    *   Passing `create_prorations` will cause proration invoice items to be created when applicable. These proration items will only be invoiced immediately under [certain conditions](https://stripe.com/docs/subscriptions/upgrading-downgrading#immediate-payment). In order to always invoice immediately for prorations, pass `always_invoice`.
    *
    *   Prorations can be disabled by passing `none`.
    * * `subscription_proration_date: i64` -- If previewing an update to a subscription, and doing proration, `subscription_proration_date` forces the proration to be calculated as though the update was done at the specified time. The time given must be within the current subscription period, and cannot be before the subscription was on its current plan. If set, `subscription`, and one of `subscription_items`, or `subscription_trial_end` are required. Also, `subscription_proration_behavior` cannot be set to 'none'.
    * * `subscription_start_date: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
    * * `subscription_trial_end: &str` -- If provided, the invoice returned will preview updating or creating a subscription with that trial end. If set, one of `subscription_items` or `subscription` is required.
    * * `subscription_trial_from_plan: bool` -- Indicates if a plan's `trial_period_days` should be applied to the subscription. Setting `subscription_trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `subscription_trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    */
    pub async fn get_invoices_upcoming_line(
        &self,
        automatic_tax: &str,
        coupon: &str,
        customer: &str,
        customer_details: &str,
        discounts: &str,
        ending_before: &str,
        expand: &[String],
        invoice_items: &[String],
        limit: i64,
        schedule: &str,
        starting_after: &str,
        subscription: &str,
        subscription_billing_cycle_anchor: &str,
        subscription_cancel_at: &str,
        subscription_cancel_at_period_end: bool,
        subscription_cancel_now: bool,
        subscription_default_tax_rates: &str,
        subscription_items: &[String],
        subscription_proration_behavior: crate::types::ProrationBehavior,
        subscription_proration_date: i64,
        subscription_start_date: i64,
        subscription_trial_end: &str,
        subscription_trial_from_plan: bool,
    ) -> Result<crate::types::InvoiceLinesList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !coupon.is_empty() {
            query_args.push(("coupon".to_string(), coupon.to_string()));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !schedule.is_empty() {
            query_args.push(("schedule".to_string(), schedule.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        if subscription_cancel_at_period_end {
            query_args.push((
                "subscription_cancel_at_period_end".to_string(),
                subscription_cancel_at_period_end.to_string(),
            ));
        }
        if subscription_cancel_now {
            query_args.push((
                "subscription_cancel_now".to_string(),
                subscription_cancel_now.to_string(),
            ));
        }
        if !subscription_proration_behavior.to_string().is_empty() {
            query_args.push((
                "subscription_proration_behavior".to_string(),
                subscription_proration_behavior.to_string(),
            ));
        }
        if subscription_proration_date > 0 {
            query_args.push((
                "subscription_proration_date".to_string(),
                subscription_proration_date.to_string(),
            ));
        }
        if subscription_start_date > 0 {
            query_args.push((
                "subscription_start_date".to_string(),
                subscription_start_date.to_string(),
            ));
        }
        if subscription_trial_from_plan {
            query_args.push((
                "subscription_trial_from_plan".to_string(),
                subscription_trial_from_plan.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/invoices/upcoming/lines?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/invoices/{invoice}` endpoint.
    *
    * <p>Retrieves the invoice with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `invoice: &str` -- The account's country.
    */
    pub async fn get_invoices_invoice(
        &self,
        expand: &[String],
        invoice: &str,
    ) -> Result<crate::types::Invoice> {
        let url = format!(
            "/v1/invoices/{}",
            crate::progenitor_support::encode_path(&invoice.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/invoices/{invoice}` endpoint.
    *
    * <p>Draft invoices are fully editable. Once an invoice is <a href="/docs/billing/invoices/workflow#finalized">finalized</a>,
    * monetary values, as well as <code>collection_method</code>, become uneditable.</p>
    *
    * <p>If you would like to stop the Stripe Billing engine from automatically finalizing, reattempting payments on,
    * sending reminders for, or <a href="/docs/billing/invoices/reconciliation">automatically reconciling</a> invoices, pass
    * <code>auto_advance=false</code>.</p>
    *
    * **Parameters:**
    *
    * * `invoice: &str` -- The account's country.
    */
    pub async fn post_invoices_invoice(&self, invoice: &str) -> Result<crate::types::Invoice> {
        let url = format!(
            "/v1/invoices/{}",
            crate::progenitor_support::encode_path(&invoice.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/invoices/{invoice}` endpoint.
    *
    * <p>Permanently deletes a one-off invoice draft. This cannot be undone. Attempts to delete invoices that are no longer in a draft state will fail; once an invoice has been finalized or if an invoice is for a subscription, it must be <a href="#void_invoice">voided</a>.</p>
    *
    * **Parameters:**
    *
    * * `invoice: &str` -- The account's country.
    */
    pub async fn delete_invoices_invoice(
        &self,
        invoice: &str,
    ) -> Result<crate::types::DeletedInvoice> {
        let url = format!(
            "/v1/invoices/{}",
            crate::progenitor_support::encode_path(&invoice.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/invoices/{invoice}/finalize` endpoint.
    *
    * <p>Stripe automatically finalizes drafts before sending and attempting payment on invoices. However, if you’d like to finalize a draft invoice manually, you can do so using this method.</p>
    *
    * **Parameters:**
    *
    * * `invoice: &str` -- The account's country.
    */
    pub async fn post_invoices_invoice_finalize(
        &self,
        invoice: &str,
    ) -> Result<crate::types::Invoice> {
        let url = format!(
            "/v1/invoices/{}/finalize",
            crate::progenitor_support::encode_path(&invoice.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/invoices/{invoice}/lines` endpoint.
    *
    * <p>When retrieving an invoice, you’ll get a <strong>lines</strong> property containing the total count of line items and the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `invoice: &str` -- The account's country.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_invoices_invoice_line(
        &self,
        ending_before: &str,
        expand: &[String],
        invoice: &str,
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::InvoiceLinesList> {
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
        let url = format!(
            "/v1/invoices/{}/lines?{}",
            crate::progenitor_support::encode_path(&invoice.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/invoices/{invoice}/mark_uncollectible` endpoint.
    *
    * <p>Marking an invoice as uncollectible is useful for keeping track of bad debts that can be written off for accounting purposes.</p>
    *
    * **Parameters:**
    *
    * * `invoice: &str` -- The account's country.
    */
    pub async fn post_invoices_invoice_mark_uncollectible(
        &self,
        invoice: &str,
    ) -> Result<crate::types::Invoice> {
        let url = format!(
            "/v1/invoices/{}/mark_uncollectible",
            crate::progenitor_support::encode_path(&invoice.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/invoices/{invoice}/pay` endpoint.
    *
    * <p>Stripe automatically creates and then attempts to collect payment on invoices for customers on subscriptions according to your <a href="https://dashboard.stripe.com/account/billing/automatic">subscriptions settings</a>. However, if you’d like to attempt payment on an invoice out of the normal collection schedule or for some other reason, you can do so.</p>
    *
    * **Parameters:**
    *
    * * `invoice: &str` -- The account's country.
    */
    pub async fn post_invoices_invoice_pay(&self, invoice: &str) -> Result<crate::types::Invoice> {
        let url = format!(
            "/v1/invoices/{}/pay",
            crate::progenitor_support::encode_path(&invoice.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/invoices/{invoice}/send` endpoint.
    *
    * <p>Stripe will automatically send invoices to customers according to your <a href="https://dashboard.stripe.com/account/billing/automatic">subscriptions settings</a>. However, if you’d like to manually send an invoice to your customer out of the normal schedule, you can do so. When sending invoices that have already been paid, there will be no reference to the payment in the email.</p>
    *
    * <p>Requests made in test-mode result in no emails being sent, despite sending an <code>invoice.sent</code> event.</p>
    *
    * **Parameters:**
    *
    * * `invoice: &str` -- The account's country.
    */
    pub async fn post_invoices_invoice_send(&self, invoice: &str) -> Result<crate::types::Invoice> {
        let url = format!(
            "/v1/invoices/{}/send",
            crate::progenitor_support::encode_path(&invoice.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/invoices/{invoice}/void` endpoint.
    *
    * <p>Mark a finalized invoice as void. This cannot be undone. Voiding an invoice is similar to <a href="#delete_invoice">deletion</a>, however it only applies to finalized invoices and maintains a papertrail where the invoice can still be found.</p>
    *
    * **Parameters:**
    *
    * * `invoice: &str` -- The account's country.
    */
    pub async fn post_invoices_invoice_void(&self, invoice: &str) -> Result<crate::types::Invoice> {
        let url = format!(
            "/v1/invoices/{}/void",
            crate::progenitor_support::encode_path(&invoice.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuer_fraud_records` endpoint.
    *
    * <p>Returns a list of issuer fraud records.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- Only return issuer fraud records for the charge specified by this charge ID.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_issuer_fraud_record(
        &self,
        charge: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::RadarIssuerFraudRecordList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
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
        let url = format!("/v1/issuer_fraud_records?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuer_fraud_records/{issuer_fraud_record}` endpoint.
    *
    * <p>Retrieves the details of an issuer fraud record that has previously been created. </p>
    *
    * <p>Please refer to the <a href="#issuer_fraud_record_object">issuer fraud record</a> object reference for more details.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `issuer_fraud_record: &str` -- The account's country.
    */
    pub async fn get_issuer_fraud_records_record(
        &self,
        expand: &[String],
        issuer_fraud_record: &str,
    ) -> Result<crate::types::IssuerFraudRecord> {
        let url = format!(
            "/v1/issuer_fraud_records/{}",
            crate::progenitor_support::encode_path(&issuer_fraud_record.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/authorizations` endpoint.
    *
    * <p>Returns a list of Issuing <code>Authorization</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    *
    * **Parameters:**
    *
    * * `card: &str` -- Only return authorizations that belong to the given card.
    * * `cardholder: &str` -- Only return authorizations that belong to the given cardholder.
    * * `created: &str` -- Only return authorizations that were created during the given date interval.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::IssuingAuthorizationStatus` -- Only return authorizations with the given status. One of `pending`, `closed`, or `reversed`.
    */
    pub async fn get_issuing_authorizations(
        &self,
        card: &str,
        cardholder: &str,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        status: crate::types::IssuingAuthorizationStatus,
    ) -> Result<crate::types::GetIssuingAuthorizationsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !card.is_empty() {
            query_args.push(("card".to_string(), card.to_string()));
        }
        if !cardholder.is_empty() {
            query_args.push(("cardholder".to_string(), cardholder.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/issuing/authorizations?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/authorizations/{authorization}` endpoint.
    *
    * <p>Retrieves an Issuing <code>Authorization</code> object.</p>
    *
    * **Parameters:**
    *
    * * `authorization: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_issuing_authorizations_authorization(
        &self,
        expand: &[String],
    ) -> Result<crate::types::IssuingAuthorization> {
        let url = format!(
            "/v1/issuing/authorizations/{}",
            crate::progenitor_support::encode_path(&authorization.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/authorizations/{authorization}` endpoint.
    *
    * <p>Updates the specified Issuing <code>Authorization</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `authorization: &str` -- The account's country.
    */
    pub async fn post_issuing_authorizations_authorization(
        &self,
    ) -> Result<crate::types::IssuingAuthorization> {
        let url = format!(
            "/v1/issuing/authorizations/{}",
            crate::progenitor_support::encode_path(&authorization.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/authorizations/{authorization}/approve` endpoint.
    *
    * <p>Approves a pending Issuing <code>Authorization</code> object. This request should be made within the timeout window of the <a href="/docs/issuing/controls/real-time-authorizations">real-time authorization</a> flow.</p>
    *
    * **Parameters:**
    *
    * * `authorization: &str` -- The account's country.
    */
    pub async fn post_issuing_authorizations_authorization_approve(
        &self,
    ) -> Result<crate::types::IssuingAuthorization> {
        let url = format!(
            "/v1/issuing/authorizations/{}/approve",
            crate::progenitor_support::encode_path(&authorization.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/authorizations/{authorization}/decline` endpoint.
    *
    * <p>Declines a pending Issuing <code>Authorization</code> object. This request should be made within the timeout window of the <a href="/docs/issuing/controls/real-time-authorizations">real time authorization</a> flow.</p>
    *
    * **Parameters:**
    *
    * * `authorization: &str` -- The account's country.
    */
    pub async fn post_issuing_authorizations_authorization_decline(
        &self,
    ) -> Result<crate::types::IssuingAuthorization> {
        let url = format!(
            "/v1/issuing/authorizations/{}/decline",
            crate::progenitor_support::encode_path(&authorization.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/cardholders` endpoint.
    *
    * <p>Returns a list of Issuing <code>Cardholder</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str` -- Only return cardholders that were created during the given date interval.
    * * `email: &str` -- Only return cardholders that have the given email address.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `phone_number: &str` -- Only return cardholders that have the given phone number.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::IssuingCardholderStatus` -- Only return cardholders that have the given status. One of `active`, `inactive`, or `blocked`.
    * * `type_: crate::types::AccountHolderType` -- Type of entity that holds the account. This can be either `individual` or `company`.
    */
    pub async fn get_issuing_cardholders(
        &self,
        created: &str,
        email: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        phone_number: &str,
        starting_after: &str,
        status: crate::types::IssuingCardholderStatus,
        type_: crate::types::AccountHolderType,
    ) -> Result<crate::types::GetIssuingCardholdersResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !phone_number.is_empty() {
            query_args.push(("phone_number".to_string(), phone_number.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/issuing/cardholders?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/cardholders` endpoint.
    *
    * <p>Creates a new Issuing <code>Cardholder</code> object that can be issued cards.</p>
    */
    pub async fn post_issuing_cardholder(&self) -> Result<crate::types::IssuingCardholder> {
        let url = "/v1/issuing/cardholders".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/cardholders/{cardholder}` endpoint.
    *
    * <p>Retrieves an Issuing <code>Cardholder</code> object.</p>
    *
    * **Parameters:**
    *
    * * `cardholder: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_issuing_cardholders_cardholder(
        &self,
        cardholder: &str,
        expand: &[String],
    ) -> Result<crate::types::IssuingCardholder> {
        let url = format!(
            "/v1/issuing/cardholders/{}",
            crate::progenitor_support::encode_path(&cardholder.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/cardholders/{cardholder}` endpoint.
    *
    * <p>Updates the specified Issuing <code>Cardholder</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `cardholder: &str` -- The account's country.
    */
    pub async fn post_issuing_cardholders_cardholder(
        &self,
        cardholder: &str,
    ) -> Result<crate::types::IssuingCardholder> {
        let url = format!(
            "/v1/issuing/cardholders/{}",
            crate::progenitor_support::encode_path(&cardholder.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/cards` endpoint.
    *
    * <p>Returns a list of Issuing <code>Card</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    *
    * **Parameters:**
    *
    * * `cardholder: &str` -- Only return cards belonging to the Cardholder with the provided ID.
    * * `created: &str` -- Only return cards that were issued during the given date interval.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `exp_month: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
    * * `exp_year: i64` -- Time at which the account was connected. Measured in seconds since the Unix epoch.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `last_4: &str` -- Only return cards that have the given last four digits.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::IssuingCardStatus` -- Only return cards that have the given status. One of `active`, `inactive`, or `canceled`.
    * * `type_: crate::types::IssuingCardType` -- Only return cards that have the given type. One of `virtual` or `physical`.
    */
    pub async fn get_issuing_cards(
        &self,
        cardholder: &str,
        created: &str,
        ending_before: &str,
        exp_month: i64,
        exp_year: i64,
        expand: &[String],
        last_4: &str,
        limit: i64,
        starting_after: &str,
        status: crate::types::IssuingCardStatus,
        type_: crate::types::IssuingCardType,
    ) -> Result<crate::types::GetIssuingCardsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cardholder.is_empty() {
            query_args.push(("cardholder".to_string(), cardholder.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if exp_month > 0 {
            query_args.push(("exp_month".to_string(), exp_month.to_string()));
        }
        if exp_year > 0 {
            query_args.push(("exp_year".to_string(), exp_year.to_string()));
        }
        if !last_4.is_empty() {
            query_args.push(("last4".to_string(), last_4.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/issuing/cards?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/cards` endpoint.
    *
    * <p>Creates an Issuing <code>Card</code> object.</p>
    */
    pub async fn post_issuing_card(&self) -> Result<crate::types::IssuingCard> {
        let url = "/v1/issuing/cards".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/cards/{card}` endpoint.
    *
    * <p>Retrieves an Issuing <code>Card</code> object.</p>
    *
    * **Parameters:**
    *
    * * `card: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_issuing_cards_card(
        &self,
        card: &str,
        expand: &[String],
    ) -> Result<crate::types::IssuingCard> {
        let url = format!(
            "/v1/issuing/cards/{}",
            crate::progenitor_support::encode_path(&card.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/cards/{card}` endpoint.
    *
    * <p>Updates the specified Issuing <code>Card</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `card: &str` -- The account's country.
    */
    pub async fn post_issuing_cards_card(&self, card: &str) -> Result<crate::types::IssuingCard> {
        let url = format!(
            "/v1/issuing/cards/{}",
            crate::progenitor_support::encode_path(&card.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/disputes` endpoint.
    *
    * <p>Returns a list of Issuing <code>Dispute</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str` -- Select Issuing disputes that were created during the given date interval.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::IssuingDisputeStatus` -- Select Issuing disputes with the given status.
    * * `transaction: &str` -- Select the Issuing dispute for the given transaction.
    */
    pub async fn get_issuing_dispute(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        status: crate::types::IssuingDisputeStatus,
        transaction: &str,
    ) -> Result<crate::types::IssuingDisputeList> {
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
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !transaction.is_empty() {
            query_args.push(("transaction".to_string(), transaction.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/issuing/disputes?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/disputes` endpoint.
    *
    * <p>Creates an Issuing <code>Dispute</code> object. Individual pieces of evidence within the <code>evidence</code> object are optional at this point. Stripe only validates that required evidence is present during submission. Refer to <a href="/docs/issuing/purchases/disputes#dispute-reasons-and-evidence">Dispute reasons and evidence</a> for more details about evidence requirements.</p>
    */
    pub async fn post_issuing_dispute(&self) -> Result<crate::types::IssuingDispute> {
        let url = "/v1/issuing/disputes".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/disputes/{dispute}` endpoint.
    *
    * <p>Retrieves an Issuing <code>Dispute</code> object.</p>
    *
    * **Parameters:**
    *
    * * `dispute: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_issuing_disputes_dispute(
        &self,
        dispute: &str,
        expand: &[String],
    ) -> Result<crate::types::IssuingDispute> {
        let url = format!(
            "/v1/issuing/disputes/{}",
            crate::progenitor_support::encode_path(&dispute.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/disputes/{dispute}` endpoint.
    *
    * <p>Updates the specified Issuing <code>Dispute</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Properties on the <code>evidence</code> object can be unset by passing in an empty string.</p>
    *
    * **Parameters:**
    *
    * * `dispute: &str` -- The account's country.
    */
    pub async fn post_issuing_disputes_dispute(
        &self,
        dispute: &str,
    ) -> Result<crate::types::IssuingDispute> {
        let url = format!(
            "/v1/issuing/disputes/{}",
            crate::progenitor_support::encode_path(&dispute.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/disputes/{dispute}/submit` endpoint.
    *
    * <p>Submits an Issuing <code>Dispute</code> to the card network. Stripe validates that all evidence fields required for the dispute’s reason are present. For more details, see <a href="/docs/issuing/purchases/disputes#dispute-reasons-and-evidence">Dispute reasons and evidence</a>.</p>
    *
    * **Parameters:**
    *
    * * `dispute: &str` -- The account's country.
    */
    pub async fn post_issuing_disputes_dispute_submit(
        &self,
        dispute: &str,
    ) -> Result<crate::types::IssuingDispute> {
        let url = format!(
            "/v1/issuing/disputes/{}/submit",
            crate::progenitor_support::encode_path(&dispute.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/settlements` endpoint.
    *
    * <p>Returns a list of Issuing <code>Settlement</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str` -- Only return issuing settlements that were created during the given date interval.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_issuing_settlements(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetIssuingSettlementsResponse> {
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
        let url = format!("/v1/issuing/settlements?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/settlements/{settlement}` endpoint.
    *
    * <p>Retrieves an Issuing <code>Settlement</code> object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `settlement: &str` -- The account's country.
    */
    pub async fn get_issuing_settlements_settlement(
        &self,
        expand: &[String],
        settlement: &str,
    ) -> Result<crate::types::IssuingSettlement> {
        let url = format!(
            "/v1/issuing/settlements/{}",
            crate::progenitor_support::encode_path(&settlement.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/settlements/{settlement}` endpoint.
    *
    * <p>Updates the specified Issuing <code>Settlement</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `settlement: &str` -- The account's country.
    */
    pub async fn post_issuing_settlements_settlement(
        &self,
        settlement: &str,
    ) -> Result<crate::types::IssuingSettlement> {
        let url = format!(
            "/v1/issuing/settlements/{}",
            crate::progenitor_support::encode_path(&settlement.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/transactions` endpoint.
    *
    * <p>Returns a list of Issuing <code>Transaction</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    *
    * **Parameters:**
    *
    * * `card: &str` -- Only return transactions that belong to the given card.
    * * `cardholder: &str` -- Only return transactions that belong to the given cardholder.
    * * `created: &str` -- Only return transactions that were created during the given date interval.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `type_: crate::types::IssuingTransactionType` -- Only return transactions that have the given type. One of `capture` or `refund`.
    */
    pub async fn get_issuing_transactions(
        &self,
        card: &str,
        cardholder: &str,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        type_: crate::types::IssuingTransactionType,
    ) -> Result<crate::types::GetIssuingTransactionsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !card.is_empty() {
            query_args.push(("card".to_string(), card.to_string()));
        }
        if !cardholder.is_empty() {
            query_args.push(("cardholder".to_string(), cardholder.to_string()));
        }
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/issuing/transactions?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/issuing/transactions/{transaction}` endpoint.
    *
    * <p>Retrieves an Issuing <code>Transaction</code> object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `transaction: &str` -- The account's country.
    */
    pub async fn get_issuing_transactions_transaction(
        &self,
        expand: &[String],
        transaction: &str,
    ) -> Result<crate::types::IssuingTransaction> {
        let url = format!(
            "/v1/issuing/transactions/{}",
            crate::progenitor_support::encode_path(&transaction.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/issuing/transactions/{transaction}` endpoint.
    *
    * <p>Updates the specified Issuing <code>Transaction</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `transaction: &str` -- The account's country.
    */
    pub async fn post_issuing_transactions_transaction(
        &self,
        transaction: &str,
    ) -> Result<crate::types::IssuingTransaction> {
        let url = format!(
            "/v1/issuing/transactions/{}",
            crate::progenitor_support::encode_path(&transaction.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/mandates/{mandate}` endpoint.
    *
    * <p>Retrieves a Mandate object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `mandate: &str` -- The account's country.
    */
    pub async fn get_mandates_mandate(
        &self,
        expand: &[String],
        mandate: &str,
    ) -> Result<crate::types::Mandate> {
        let url = format!(
            "/v1/mandates/{}",
            crate::progenitor_support::encode_path(&mandate.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/order_returns` endpoint.
    *
    * <p>Returns a list of your order returns. The returns are returned sorted by creation date, with the most recently created return appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str` -- Date this return was created.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `order: &str` -- The order to retrieve returns for.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_order_return(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        order: &str,
        starting_after: &str,
    ) -> Result<crate::types::Returns> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/order_returns?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/order_returns/{id}` endpoint.
    *
    * <p>Retrieves the details of an existing order return. Supply the unique order ID from either an order return creation request or the order return list, and Stripe will return the corresponding order information.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_order_return_v_1(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::OrderReturn> {
        let url = format!(
            "/v1/order_returns/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/orders` endpoint.
    *
    * <p>Returns a list of your orders. The orders are returned sorted by creation date, with the most recently created orders appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str` -- Date this order was created.
    * * `customer: &str` -- Only return orders for the given customer.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `ids: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: &str` -- Only return orders that have the given status. One of `created`, `paid`, `fulfilled`, or `refunded`.
    * * `status_transitions: &str` -- Filter orders based on when they were paid, fulfilled, canceled, or returned.
    * * `upstream_ids: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_order(
        &self,
        created: &str,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        ids: &[String],
        limit: i64,
        starting_after: &str,
        status: &str,
        status_transitions: &str,
        upstream_ids: &[String],
    ) -> Result<crate::types::OrdersLegacyResourceOrderList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/orders?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/orders` endpoint.
    *
    * <p>Creates a new order object.</p>
    */
    pub async fn post_order(&self) -> Result<crate::types::Order> {
        let url = "/v1/orders".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/orders/{id}` endpoint.
    *
    * <p>Retrieves the details of an existing order. Supply the unique order ID from either an order creation request or the order list, and Stripe will return the corresponding order information.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_order_v_1(&self, expand: &[String], id: &str) -> Result<crate::types::Order> {
        let url = format!(
            "/v1/orders/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/orders/{id}` endpoint.
    *
    * <p>Updates the specific order by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_order_v_1(&self, id: &str) -> Result<crate::types::Order> {
        let url = format!(
            "/v1/orders/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/orders/{id}/pay` endpoint.
    *
    * <p>Pay an order by providing a <code>source</code> to create a payment.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_orders_pay(&self, id: &str) -> Result<crate::types::Order> {
        let url = format!(
            "/v1/orders/{}/pay",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/orders/{id}/returns` endpoint.
    *
    * <p>Return all or part of an order. The order must have a status of <code>paid</code> or <code>fulfilled</code> before it can be returned. Once all items have been returned, the order will become <code>canceled</code> or <code>returned</code> depending on which status the order started in.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_orders_return(&self, id: &str) -> Result<crate::types::OrderReturn> {
        let url = format!(
            "/v1/orders/{}/returns",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payment_intents` endpoint.
    *
    * <p>Returns a list of PaymentIntents.</p>
    *
    * **Parameters:**
    *
    * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    * * `customer: &str` -- Only return PaymentIntents for the customer specified by this customer ID.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_payment_intent(
        &self,
        created: &str,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::PaymentFlowsIntentList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
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
        let url = format!("/v1/payment_intents?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_intents` endpoint.
    *
    * <p>Creates a PaymentIntent object.</p>
    *
    * <p>After the PaymentIntent is created, attach a payment method and <a href="/docs/api/payment_intents/confirm">confirm</a>
    * to continue the payment. You can read more about the different payment flows
    * available via the Payment Intents API <a href="/docs/payments/payment-intents">here</a>.</p>
    *
    * <p>When <code>confirm=true</code> is used during creation, it is equivalent to creating
    * and confirming the PaymentIntent in the same call. You may use any parameters
    * available in the <a href="/docs/api/payment_intents/confirm">confirm API</a> when <code>confirm=true</code>
    * is supplied.</p>
    */
    pub async fn post_payment_intent(&self) -> Result<crate::types::PaymentIntent> {
        let url = "/v1/payment_intents".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payment_intents/search` endpoint.
    *
    * <p>Search for PaymentIntents you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
    * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
    * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
    * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
    * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for payment intents](https://stripe.com/docs/search#query-fields-for-payment-intents).
    */
    pub async fn get_payment_intents_search(
        &self,
        expand: &[String],
        limit: i64,
        page: &str,
        query: &str,
    ) -> Result<crate::types::SearchResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/payment_intents/search?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payment_intents/{intent}` endpoint.
    *
    * <p>Retrieves the details of a PaymentIntent that has previously been created. </p>
    *
    * <p>Client-side retrieval using a publishable key is allowed when the <code>client_secret</code> is provided in the query string. </p>
    *
    * <p>When retrieved with a publishable key, only a subset of properties will be returned. Please refer to the <a href="#payment_intent_object">payment intent</a> object reference for more details.</p>
    *
    * **Parameters:**
    *
    * * `client_secret: &str` -- The client secret of the PaymentIntent. Required if a publishable key is used to retrieve the source.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `intent: &str` -- The account's country.
    */
    pub async fn get_payment_intents_intent(
        &self,
        client_secret: &str,
        expand: &[String],
        intent: &str,
    ) -> Result<crate::types::PaymentIntent> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_secret.is_empty() {
            query_args.push(("client_secret".to_string(), client_secret.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/v1/payment_intents/{}?{}",
            crate::progenitor_support::encode_path(&intent.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_intents/{intent}` endpoint.
    *
    * <p>Updates properties on a PaymentIntent object without confirming.</p>
    *
    * <p>Depending on which properties you update, you may need to confirm the
    * PaymentIntent again. For example, updating the <code>payment_method</code> will
    * always require you to confirm the PaymentIntent again. If you prefer to
    * update and confirm at the same time, we recommend updating properties via
    * the <a href="/docs/api/payment_intents/confirm">confirm API</a> instead.</p>
    *
    * **Parameters:**
    *
    * * `intent: &str` -- The account's country.
    */
    pub async fn post_payment_intents_intent(
        &self,
        intent: &str,
    ) -> Result<crate::types::PaymentIntent> {
        let url = format!(
            "/v1/payment_intents/{}",
            crate::progenitor_support::encode_path(&intent.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_intents/{intent}/cancel` endpoint.
    *
    * <p>A PaymentIntent object can be canceled when it is in one of these statuses: <code>requires_payment_method</code>, <code>requires_capture</code>, <code>requires_confirmation</code>, <code>requires_action</code>, or <code>processing</code>. </p>
    *
    * <p>Once canceled, no additional charges will be made by the PaymentIntent and any operations on the PaymentIntent will fail with an error. For PaymentIntents with <code>status=’requires_capture’</code>, the remaining <code>amount_capturable</code> will automatically be refunded. </p>
    *
    * <p>You cannot cancel the PaymentIntent for a Checkout Session. <a href="/docs/api/checkout/sessions/expire">Expire the Checkout Session</a> instead</p>
    *
    * **Parameters:**
    *
    * * `intent: &str` -- The account's country.
    */
    pub async fn post_payment_intents_intent_cancel(
        &self,
        intent: &str,
    ) -> Result<crate::types::PaymentIntent> {
        let url = format!(
            "/v1/payment_intents/{}/cancel",
            crate::progenitor_support::encode_path(&intent.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_intents/{intent}/capture` endpoint.
    *
    * <p>Capture the funds of an existing uncaptured PaymentIntent when its status is <code>requires_capture</code>.</p>
    *
    * <p>Uncaptured PaymentIntents will be canceled a set number of days after they are created (7 by default).</p>
    *
    * <p>Learn more about <a href="/docs/payments/capture-later">separate authorization and capture</a>.</p>
    *
    * **Parameters:**
    *
    * * `intent: &str` -- The account's country.
    */
    pub async fn post_payment_intents_intent_capture(
        &self,
        intent: &str,
    ) -> Result<crate::types::PaymentIntent> {
        let url = format!(
            "/v1/payment_intents/{}/capture",
            crate::progenitor_support::encode_path(&intent.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_intents/{intent}/confirm` endpoint.
    *
    * <p>Confirm that your customer intends to pay with current or provided
    * payment method. Upon confirmation, the PaymentIntent will attempt to initiate
    * a payment.</p>
    *
    * <p>If the selected payment method requires additional authentication steps, the
    * PaymentIntent will transition to the <code>requires_action</code> status and
    * suggest additional actions via <code>next_action</code>. If payment fails,
    * the PaymentIntent will transition to the <code>requires_payment_method</code> status. If
    * payment succeeds, the PaymentIntent will transition to the <code>succeeded</code>
    * status (or <code>requires_capture</code>, if <code>capture_method</code> is set to <code>manual</code>).</p>
    *
    * <p>If the <code>confirmation_method</code> is <code>automatic</code>, payment may be attempted
    * using our <a href="/docs/stripe-js/reference#stripe-handle-card-payment">client SDKs</a>
    * and the PaymentIntent’s <a href="#payment_intent_object-client_secret">client_secret</a>.
    * After <code>next_action</code>s are handled by the client, no additional
    * confirmation is required to complete the payment.</p>
    *
    * <p>If the <code>confirmation_method</code> is <code>manual</code>, all payment attempts must be
    * initiated using a secret key.
    * If any actions are required for the payment, the PaymentIntent will
    * return to the <code>requires_confirmation</code> state
    * after those actions are completed. Your server needs to then
    * explicitly re-confirm the PaymentIntent to initiate the next payment
    * attempt. Read the <a href="/docs/payments/payment-intents/web-manual">expanded documentation</a>
    * to learn more about manual confirmation.</p>
    *
    * **Parameters:**
    *
    * * `intent: &str` -- The account's country.
    */
    pub async fn post_payment_intents_intent_confirm(
        &self,
        intent: &str,
    ) -> Result<crate::types::PaymentIntent> {
        let url = format!(
            "/v1/payment_intents/{}/confirm",
            crate::progenitor_support::encode_path(&intent.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_intents/{intent}/verify_microdeposits` endpoint.
    *
    * <p>Verifies microdeposits on a PaymentIntent object.</p>
    *
    * **Parameters:**
    *
    * * `intent: &str` -- The account's country.
    */
    pub async fn post_payment_intents_intent_verify_microdeposit(
        &self,
        intent: &str,
    ) -> Result<crate::types::PaymentIntent> {
        let url = format!(
            "/v1/payment_intents/{}/verify_microdeposits",
            crate::progenitor_support::encode_path(&intent.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payment_links` endpoint.
    *
    * <p>Returns a list of your payment links.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Only return payment links that are active or inactive (e.g., pass `false` to list all inactive payment links).
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_payment_links(
        &self,
        active: bool,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetPaymentLinksResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
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
        let url = format!("/v1/payment_links?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_links` endpoint.
    *
    * <p>Creates a payment link.</p>
    */
    pub async fn post_payment_link(&self) -> Result<crate::types::PaymentLink> {
        let url = "/v1/payment_links".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payment_links/{payment_link}` endpoint.
    *
    * <p>Retrieve a payment link.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `payment_link: &str` -- The account's country.
    */
    pub async fn get_payment_links_link(
        &self,
        expand: &[String],
        payment_link: &str,
    ) -> Result<crate::types::PaymentLink> {
        let url = format!(
            "/v1/payment_links/{}",
            crate::progenitor_support::encode_path(&payment_link.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_links/{payment_link}` endpoint.
    *
    * <p>Updates a payment link.</p>
    *
    * **Parameters:**
    *
    * * `payment_link: &str` -- The account's country.
    */
    pub async fn post_payment_links_link(
        &self,
        payment_link: &str,
    ) -> Result<crate::types::PaymentLink> {
        let url = format!(
            "/v1/payment_links/{}",
            crate::progenitor_support::encode_path(&payment_link.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payment_links/{payment_link}/line_items` endpoint.
    *
    * <p>When retrieving a payment link, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `payment_link: &str` -- The account's country.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_payment_links_link_line_item(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        payment_link: &str,
        starting_after: &str,
    ) -> Result<crate::types::LineItems> {
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
        let url = format!(
            "/v1/payment_links/{}/line_items?{}",
            crate::progenitor_support::encode_path(&payment_link.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payment_methods` endpoint.
    *
    * <p>Returns a list of PaymentMethods. For listing a customer’s payment methods, you should use <a href="/docs/api/payment_methods/customer_list">List a Customer’s PaymentMethods</a></p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The ID of the customer whose PaymentMethods will be retrieved. If not provided, the response list will be empty.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `type_: crate::types::GetCustomersCustomerPaymentMethodsType` -- A required filter on the list, based on the object `type` field.
    */
    pub async fn get_payment_method(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        type_: crate::types::GetCustomersCustomerPaymentMethodsType,
    ) -> Result<crate::types::PaymentFlowsMethodList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/payment_methods?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_methods` endpoint.
    *
    * <p>Creates a PaymentMethod object. Read the <a href="/docs/stripe-js/reference#stripe-create-payment-method">Stripe.js reference</a> to learn how to create PaymentMethods via Stripe.js.</p>
    *
    * <p>Instead of creating a PaymentMethod directly, we recommend using the <a href="/docs/payments/accept-a-payment">PaymentIntents</a> API to accept a payment immediately or the <a href="/docs/payments/save-and-reuse">SetupIntent</a> API to collect payment method details ahead of a future payment.</p>
    */
    pub async fn post_payment_method(&self) -> Result<crate::types::PaymentMethod> {
        let url = "/v1/payment_methods".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payment_methods/{payment_method}` endpoint.
    *
    * <p>Retrieves a PaymentMethod object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `payment_method: &str` -- The account's country.
    */
    pub async fn get_payment_methods_method(
        &self,
        expand: &[String],
        payment_method: &str,
    ) -> Result<crate::types::PaymentMethod> {
        let url = format!(
            "/v1/payment_methods/{}",
            crate::progenitor_support::encode_path(&payment_method.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_methods/{payment_method}` endpoint.
    *
    * <p>Updates a PaymentMethod object. A PaymentMethod must be attached a customer to be updated.</p>
    *
    * **Parameters:**
    *
    * * `payment_method: &str` -- The account's country.
    */
    pub async fn post_payment_methods_method(
        &self,
        payment_method: &str,
    ) -> Result<crate::types::PaymentMethod> {
        let url = format!(
            "/v1/payment_methods/{}",
            crate::progenitor_support::encode_path(&payment_method.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_methods/{payment_method}/attach` endpoint.
    *
    * <p>Attaches a PaymentMethod object to a Customer.</p>
    *
    * <p>To attach a new PaymentMethod to a customer for future payments, we recommend you use a <a href="/docs/api/setup_intents">SetupIntent</a>
    * or a PaymentIntent with <a href="/docs/api/payment_intents/create#create_payment_intent-setup_future_usage">setup_future_usage</a>.
    * These approaches will perform any necessary steps to ensure that the PaymentMethod can be used in a future payment. Using the
    * <code>/v1/payment_methods/:id/attach</code> endpoint does not ensure that future payments can be made with the attached PaymentMethod.
    * See <a href="/docs/payments/payment-intents#future-usage">Optimizing cards for future payments</a> for more information about setting up future payments.</p>
    *
    * <p>To use this PaymentMethod as the default for invoice or subscription payments,
    * set <a href="/docs/api/customers/update#update_customer-invoice_settings-default_payment_method"><code>invoice_settings.default_payment_method</code></a>,
    * on the Customer to the PaymentMethod’s ID.</p>
    *
    * **Parameters:**
    *
    * * `payment_method: &str` -- The account's country.
    */
    pub async fn post_payment_methods_method_attach(
        &self,
        payment_method: &str,
    ) -> Result<crate::types::PaymentMethod> {
        let url = format!(
            "/v1/payment_methods/{}/attach",
            crate::progenitor_support::encode_path(&payment_method.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payment_methods/{payment_method}/detach` endpoint.
    *
    * <p>Detaches a PaymentMethod object from a Customer. After a PaymentMethod is detached, it can no longer be used for a payment or re-attached to a Customer.</p>
    *
    * **Parameters:**
    *
    * * `payment_method: &str` -- The account's country.
    */
    pub async fn post_payment_methods_method_detach(
        &self,
        payment_method: &str,
    ) -> Result<crate::types::PaymentMethod> {
        let url = format!(
            "/v1/payment_methods/{}/detach",
            crate::progenitor_support::encode_path(&payment_method.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payouts` endpoint.
    *
    * <p>Returns a list of existing payouts sent to third-party bank accounts or that Stripe has sent you. The payouts are returned in sorted order, with the most recently created payouts appearing first.</p>
    *
    * **Parameters:**
    *
    * * `arrival_date: &str`
    * * `created: &str`
    * * `destination: &str` -- The ID of an external account - only return payouts sent to this external account.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: &str` -- Only return payouts that have the given status: `pending`, `paid`, `failed`, or `canceled`.
    */
    pub async fn get_payout(
        &self,
        arrival_date: &str,
        created: &str,
        destination: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        status: &str,
    ) -> Result<crate::types::PayoutList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !destination.is_empty() {
            query_args.push(("destination".to_string(), destination.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/payouts?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payouts` endpoint.
    *
    * <p>To send funds to your own bank account, you create a new payout object. Your <a href="#balance">Stripe balance</a> must be able to cover the payout amount, or you’ll receive an “Insufficient Funds” error.</p>
    *
    * <p>If your API key is in test mode, money won’t actually be sent, though everything else will occur as if in live mode.</p>
    *
    * <p>If you are creating a manual payout on a Stripe account that uses multiple payment source types, you’ll need to specify the source type balance that the payout should draw from. The <a href="#balance_object">balance object</a> details available and pending amounts by source type.</p>
    */
    pub async fn post_payout(&self) -> Result<crate::types::Payout> {
        let url = "/v1/payouts".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/payouts/{payout}` endpoint.
    *
    * <p>Retrieves the details of an existing payout. Supply the unique payout ID from either a payout creation request or the payout list, and Stripe will return the corresponding payout information.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `payout: &str` -- The account's country.
    */
    pub async fn get_payouts_payout(
        &self,
        expand: &[String],
        payout: &str,
    ) -> Result<crate::types::Payout> {
        let url = format!(
            "/v1/payouts/{}",
            crate::progenitor_support::encode_path(&payout.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payouts/{payout}` endpoint.
    *
    * <p>Updates the specified payout by setting the values of the parameters passed. Any parameters not provided will be left unchanged. This request accepts only the metadata as arguments.</p>
    *
    * **Parameters:**
    *
    * * `payout: &str` -- The account's country.
    */
    pub async fn post_payouts_payout(&self, payout: &str) -> Result<crate::types::Payout> {
        let url = format!(
            "/v1/payouts/{}",
            crate::progenitor_support::encode_path(&payout.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payouts/{payout}/cancel` endpoint.
    *
    * <p>A previously created payout can be canceled if it has not yet been paid out. Funds will be refunded to your available balance. You may not cancel automatic Stripe payouts.</p>
    *
    * **Parameters:**
    *
    * * `payout: &str` -- The account's country.
    */
    pub async fn post_payouts_payout_cancel(&self, payout: &str) -> Result<crate::types::Payout> {
        let url = format!(
            "/v1/payouts/{}/cancel",
            crate::progenitor_support::encode_path(&payout.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/payouts/{payout}/reverse` endpoint.
    *
    * <p>Reverses a payout by debiting the destination bank account. Only payouts for connected accounts to US bank accounts may be reversed at this time. If the payout is in the <code>pending</code> status, <code>/v1/payouts/:id/cancel</code> should be used instead.</p>
    *
    * <p>By requesting a reversal via <code>/v1/payouts/:id/reverse</code>, you confirm that the authorized signatory of the selected bank account has authorized the debit on the bank account and that no other authorization is required.</p>
    *
    * **Parameters:**
    *
    * * `payout: &str` -- The account's country.
    */
    pub async fn post_payouts_payout_reverse(&self, payout: &str) -> Result<crate::types::Payout> {
        let url = format!(
            "/v1/payouts/{}/reverse",
            crate::progenitor_support::encode_path(&payout.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/plans` endpoint.
    *
    * <p>Returns a list of your plans.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Only return plans that are active or inactive (e.g., pass `false` to list all inactive plans).
    * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `product: &str` -- Only return plans for the given product.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_plan(
        &self,
        active: bool,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        product: &str,
        starting_after: &str,
    ) -> Result<crate::types::PlanList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/plans?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/plans` endpoint.
    *
    * <p>You can now model subscriptions more flexibly using the <a href="#prices">Prices API</a>. It replaces the Plans API and is backwards compatible to simplify your migration.</p>
    */
    pub async fn post_plan(&self) -> Result<crate::types::PlanData> {
        let url = "/v1/plans".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/plans/{plan}` endpoint.
    *
    * <p>Retrieves the plan with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `plan: &str` -- The account's country.
    */
    pub async fn get_plans_plan(
        &self,
        expand: &[String],
        plan: &str,
    ) -> Result<crate::types::PlanData> {
        let url = format!(
            "/v1/plans/{}",
            crate::progenitor_support::encode_path(&plan.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/plans/{plan}` endpoint.
    *
    * <p>Updates the specified plan by setting the values of the parameters passed. Any parameters not provided are left unchanged. By design, you cannot change a plan’s ID, amount, currency, or billing cycle.</p>
    *
    * **Parameters:**
    *
    * * `plan: &str` -- The account's country.
    */
    pub async fn post_plans_plan(&self, plan: &str) -> Result<crate::types::PlanData> {
        let url = format!(
            "/v1/plans/{}",
            crate::progenitor_support::encode_path(&plan.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/plans/{plan}` endpoint.
    *
    * <p>Deleting plans means new subscribers can’t be added. Existing subscribers aren’t affected.</p>
    *
    * **Parameters:**
    *
    * * `plan: &str` -- The account's country.
    */
    pub async fn delete_plans_plan(&self, plan: &str) -> Result<crate::types::DeletedPlan> {
        let url = format!(
            "/v1/plans/{}",
            crate::progenitor_support::encode_path(&plan.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/prices` endpoint.
    *
    * <p>Returns a list of your prices.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Only return prices that are active or inactive (e.g., pass `false` to list all inactive prices).
    * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    * * `currency: &str` -- Only return prices for the given currency.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `lookup_keys: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `product: &str` -- Only return prices for the given product.
    * * `recurring: &str` -- Only return prices with these recurring fields.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `type_: crate::types::PriceType` -- One of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase.
    */
    pub async fn get_price(
        &self,
        active: bool,
        created: &str,
        currency: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        lookup_keys: &[String],
        product: &str,
        recurring: &str,
        starting_after: &str,
        type_: crate::types::PriceType,
    ) -> Result<crate::types::PriceList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/prices?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/prices` endpoint.
    *
    * <p>Creates a new price for an existing product. The price can be recurring or one-time.</p>
    */
    pub async fn post_price(&self) -> Result<crate::types::PriceData> {
        let url = "/v1/prices".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/prices/search` endpoint.
    *
    * <p>Search for prices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
    * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
    * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
    * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
    * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for prices](https://stripe.com/docs/search#query-fields-for-prices).
    */
    pub async fn get_prices_search(
        &self,
        expand: &[String],
        limit: i64,
        page: &str,
        query: &str,
    ) -> Result<crate::types::SearchResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/prices/search?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/prices/{price}` endpoint.
    *
    * <p>Retrieves the price with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `price: &str` -- The account's country.
    */
    pub async fn get_prices_price(
        &self,
        expand: &[String],
        price: &str,
    ) -> Result<crate::types::PriceData> {
        let url = format!(
            "/v1/prices/{}",
            crate::progenitor_support::encode_path(&price.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/prices/{price}` endpoint.
    *
    * <p>Updates the specified price by setting the values of the parameters passed. Any parameters not provided are left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `price: &str` -- The account's country.
    */
    pub async fn post_prices_price(&self, price: &str) -> Result<crate::types::PriceData> {
        let url = format!(
            "/v1/prices/{}",
            crate::progenitor_support::encode_path(&price.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/products` endpoint.
    *
    * <p>Returns a list of your products. The products are returned sorted by creation date, with the most recently created products appearing first.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Only return products that are active or inactive (e.g., pass `false` to list all inactive products).
    * * `created: &str` -- Only return products that were created during the given date interval.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `ids: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `shippable: bool` -- Only return products that can be shipped (i.e., physical, not digital products).
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `url: &str` -- Only return products with the given url.
    */
    pub async fn get_product(
        &self,
        active: bool,
        created: &str,
        ending_before: &str,
        expand: &[String],
        ids: &[String],
        limit: i64,
        shippable: bool,
        starting_after: &str,
        url: &str,
    ) -> Result<crate::types::ProductList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if shippable {
            query_args.push(("shippable".to_string(), shippable.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !url.is_empty() {
            query_args.push(("url".to_string(), url.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/products?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/products` endpoint.
    *
    * <p>Creates a new product object.</p>
    */
    pub async fn post_product(&self) -> Result<crate::types::Product> {
        let url = "/v1/products".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/products/search` endpoint.
    *
    * <p>Search for products you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
    * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
    * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
    * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
    * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for products](https://stripe.com/docs/search#query-fields-for-products).
    */
    pub async fn get_products_search(
        &self,
        expand: &[String],
        limit: i64,
        page: &str,
        query: &str,
    ) -> Result<crate::types::SearchResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/products/search?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/products/{id}` endpoint.
    *
    * <p>Retrieves the details of an existing product. Supply the unique product ID from either a product creation request or the product list, and Stripe will return the corresponding product information.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_product_v_1(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::Product> {
        let url = format!(
            "/v1/products/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/products/{id}` endpoint.
    *
    * <p>Updates the specific product by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_product_v_1(&self, id: &str) -> Result<crate::types::Product> {
        let url = format!(
            "/v1/products/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/products/{id}` endpoint.
    *
    * <p>Delete a product. Deleting a product is only possible if it has no prices associated with it. Additionally, deleting a product with <code>type=good</code> is only possible if it has no SKUs associated with it.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_products(&self, id: &str) -> Result<crate::types::DeletedProduct> {
        let url = format!(
            "/v1/products/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/promotion_codes` endpoint.
    *
    * <p>Returns a list of your promotion codes.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Filter promotion codes by whether they are active.
    * * `code: &str` -- Only return promotion codes that have this case-insensitive code.
    * * `coupon: &str` -- Only return promotion codes for this coupon.
    * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    * * `customer: &str` -- Only return promotion codes that are restricted to this customer.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_promotion_codes(
        &self,
        active: bool,
        code: &str,
        coupon: &str,
        created: &str,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetPromotionCodesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !code.is_empty() {
            query_args.push(("code".to_string(), code.to_string()));
        }
        if !coupon.is_empty() {
            query_args.push(("coupon".to_string(), coupon.to_string()));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
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
        let url = format!("/v1/promotion_codes?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/promotion_codes` endpoint.
    *
    * <p>A promotion code points to a coupon. You can optionally restrict the code to a specific customer, redemption limit, and expiration date.</p>
    */
    pub async fn post_promotion_code(&self) -> Result<crate::types::PromotionCode> {
        let url = "/v1/promotion_codes".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/promotion_codes/{promotion_code}` endpoint.
    *
    * <p>Retrieves the promotion code with the given ID. In order to retrieve a promotion code by the customer-facing <code>code</code> use <a href="/docs/api/promotion_codes/list">list</a> with the desired <code>code</code>.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `promotion_code: &str` -- The account's country.
    */
    pub async fn get_promotion_codes_code(
        &self,
        expand: &[String],
        promotion_code: &str,
    ) -> Result<crate::types::PromotionCode> {
        let url = format!(
            "/v1/promotion_codes/{}",
            crate::progenitor_support::encode_path(&promotion_code.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/promotion_codes/{promotion_code}` endpoint.
    *
    * <p>Updates the specified promotion code by setting the values of the parameters passed. Most fields are, by design, not editable.</p>
    *
    * **Parameters:**
    *
    * * `promotion_code: &str` -- The account's country.
    */
    pub async fn post_promotion_codes_code(
        &self,
        promotion_code: &str,
    ) -> Result<crate::types::PromotionCode> {
        let url = format!(
            "/v1/promotion_codes/{}",
            crate::progenitor_support::encode_path(&promotion_code.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/quotes` endpoint.
    *
    * <p>Returns a list of your quotes.</p>
    *
    * **Parameters:**
    *
    * * `customer: &str` -- The ID of the customer whose quotes will be retrieved.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::QuoteStatus` -- The status of the quote.
    * * `test_clock: &str` -- Provides a list of quotes that are associated with the specified test clock. The response will not include quotes with test clocks if this and the customer parameter is not set.
    */
    pub async fn get_quotes(
        &self,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        status: crate::types::QuoteStatus,
        test_clock: &str,
    ) -> Result<crate::types::GetQuotesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !test_clock.is_empty() {
            query_args.push(("test_clock".to_string(), test_clock.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/quotes?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/quotes` endpoint.
    *
    * <p>A quote models prices and services for a customer. Default options for <code>header</code>, <code>description</code>, <code>footer</code>, and <code>expires_at</code> can be set in the dashboard via the <a href="https://dashboard.stripe.com/settings/billing/quote">quote template</a>.</p>
    */
    pub async fn post_quote(&self) -> Result<crate::types::Quote> {
        let url = "/v1/quotes".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/quotes/{quote}` endpoint.
    *
    * <p>Retrieves the quote with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `quote: &str` -- The account's country.
    */
    pub async fn get_quotes_quote(
        &self,
        expand: &[String],
        quote: &str,
    ) -> Result<crate::types::Quote> {
        let url = format!(
            "/v1/quotes/{}",
            crate::progenitor_support::encode_path(&quote.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/quotes/{quote}` endpoint.
    *
    * <p>A quote models prices and services for a customer.</p>
    *
    * **Parameters:**
    *
    * * `quote: &str` -- The account's country.
    */
    pub async fn post_quotes_quote(&self, quote: &str) -> Result<crate::types::Quote> {
        let url = format!(
            "/v1/quotes/{}",
            crate::progenitor_support::encode_path(&quote.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/quotes/{quote}/accept` endpoint.
    *
    * <p>Accepts the specified quote.</p>
    *
    * **Parameters:**
    *
    * * `quote: &str` -- The account's country.
    */
    pub async fn post_quotes_quote_accept(&self, quote: &str) -> Result<crate::types::Quote> {
        let url = format!(
            "/v1/quotes/{}/accept",
            crate::progenitor_support::encode_path(&quote.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/quotes/{quote}/cancel` endpoint.
    *
    * <p>Cancels the quote.</p>
    *
    * **Parameters:**
    *
    * * `quote: &str` -- The account's country.
    */
    pub async fn post_quotes_quote_cancel(&self, quote: &str) -> Result<crate::types::Quote> {
        let url = format!(
            "/v1/quotes/{}/cancel",
            crate::progenitor_support::encode_path(&quote.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/quotes/{quote}/computed_upfront_line_items` endpoint.
    *
    * <p>When retrieving a quote, there is an includable <a href="https://stripe.com/docs/api/quotes/object#quote_object-computed-upfront-line_items"><strong>computed.upfront.line_items</strong></a> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of upfront line items.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `quote: &str` -- The account's country.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_quotes_quote_computed_upfront_line_item(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        quote: &str,
        starting_after: &str,
    ) -> Result<crate::types::LineItems> {
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
        let url = format!(
            "/v1/quotes/{}/computed_upfront_line_items?{}",
            crate::progenitor_support::encode_path(&quote.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/quotes/{quote}/finalize` endpoint.
    *
    * <p>Finalizes the quote.</p>
    *
    * **Parameters:**
    *
    * * `quote: &str` -- The account's country.
    */
    pub async fn post_quotes_quote_finalize(&self, quote: &str) -> Result<crate::types::Quote> {
        let url = format!(
            "/v1/quotes/{}/finalize",
            crate::progenitor_support::encode_path(&quote.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/quotes/{quote}/line_items` endpoint.
    *
    * <p>When retrieving a quote, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `quote: &str` -- The account's country.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_quotes_quote_line_item(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        quote: &str,
        starting_after: &str,
    ) -> Result<crate::types::LineItems> {
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
        let url = format!(
            "/v1/quotes/{}/line_items?{}",
            crate::progenitor_support::encode_path(&quote.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/quotes/{quote}/pdf` endpoint.
    *
    * <p>Download the PDF for a finalized quote</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `quote: &str` -- The account's country.
    */
    pub async fn get_quotes_quote_pdf(&self, expand: &[String], quote: &str) -> Result<()> {
        let url = format!(
            "/v1/quotes/{}/pdf",
            crate::progenitor_support::encode_path(&quote.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/radar/early_fraud_warnings` endpoint.
    *
    * <p>Returns a list of early fraud warnings.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- Only return early fraud warnings for the charge specified by this charge ID.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `payment_intent: &str` -- Only return early fraud warnings for charges that were created by the PaymentIntent specified by this PaymentIntent ID.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_radar_early_fraud_warning(
        &self,
        charge: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        payment_intent: &str,
        starting_after: &str,
    ) -> Result<crate::types::RadarEarlyFraudWarningList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/radar/early_fraud_warnings?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/radar/early_fraud_warnings/{early_fraud_warning}` endpoint.
    *
    * <p>Retrieves the details of an early fraud warning that has previously been created. </p>
    *
    * <p>Please refer to the <a href="#early_fraud_warning_object">early fraud warning</a> object reference for more details.</p>
    *
    * **Parameters:**
    *
    * * `early_fraud_warning: &str` -- The account's country.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_radar_early_fraud_warnings_warning(
        &self,
        early_fraud_warning: &str,
        expand: &[String],
    ) -> Result<crate::types::RadarEarlyFraudWarning> {
        let url = format!(
            "/v1/radar/early_fraud_warnings/{}",
            crate::progenitor_support::encode_path(&early_fraud_warning.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/radar/value_list_items` endpoint.
    *
    * <p>Returns a list of <code>ValueListItem</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `value: &str` -- Return items belonging to the parent list whose value matches the specified value (using an "is like" match).
    * * `value_list: &str` -- Identifier for the parent value list this item belongs to.
    */
    pub async fn get_radar_value_list_item(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        value: &str,
        value_list: &str,
    ) -> Result<crate::types::ListItems> {
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
        if !value.is_empty() {
            query_args.push(("value".to_string(), value.to_string()));
        }
        if !value_list.is_empty() {
            query_args.push(("value_list".to_string(), value_list.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/radar/value_list_items?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/radar/value_list_items` endpoint.
    *
    * <p>Creates a new <code>ValueListItem</code> object, which is added to the specified parent value list.</p>
    */
    pub async fn post_radar_value_list_item(&self) -> Result<crate::types::RadarListItem> {
        let url = "/v1/radar/value_list_items".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/radar/value_list_items/{item}` endpoint.
    *
    * <p>Retrieves a <code>ValueListItem</code> object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `item: &str` -- The account's country.
    */
    pub async fn get_radar_value_list_items_item(
        &self,
        expand: &[String],
        item: &str,
    ) -> Result<crate::types::RadarListItem> {
        let url = format!(
            "/v1/radar/value_list_items/{}",
            crate::progenitor_support::encode_path(&item.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/radar/value_list_items/{item}` endpoint.
    *
    * <p>Deletes a <code>ValueListItem</code> object, removing it from its parent value list.</p>
    *
    * **Parameters:**
    *
    * * `item: &str` -- The account's country.
    */
    pub async fn delete_radar_value_list_items_item(
        &self,
        item: &str,
    ) -> Result<crate::types::RadarListDeletedItem> {
        let url = format!(
            "/v1/radar/value_list_items/{}",
            crate::progenitor_support::encode_path(&item.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/radar/value_lists` endpoint.
    *
    * <p>Returns a list of <code>ValueList</code> objects. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    *
    * **Parameters:**
    *
    * * `alias: &str` -- The alias used to reference the value list when writing rules.
    * * `contains: &str` -- A value contained within a value list - returns all value lists containing this value.
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_radar_value_lists(
        &self,
        alias: &str,
        contains: &str,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetRadarValueListsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !alias.is_empty() {
            query_args.push(("alias".to_string(), alias.to_string()));
        }
        if !contains.is_empty() {
            query_args.push(("contains".to_string(), contains.to_string()));
        }
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
        let url = format!("/v1/radar/value_lists?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/radar/value_lists` endpoint.
    *
    * <p>Creates a new <code>ValueList</code> object, which can then be referenced in rules.</p>
    */
    pub async fn post_radar_value_list(&self) -> Result<crate::types::RadarList> {
        let url = "/v1/radar/value_lists".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/radar/value_lists/{value_list}` endpoint.
    *
    * <p>Retrieves a <code>ValueList</code> object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `value_list: &str` -- The account's country.
    */
    pub async fn get_radar_value_lists_list(
        &self,
        expand: &[String],
        value_list: &str,
    ) -> Result<crate::types::RadarList> {
        let url = format!(
            "/v1/radar/value_lists/{}",
            crate::progenitor_support::encode_path(&value_list.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/radar/value_lists/{value_list}` endpoint.
    *
    * <p>Updates a <code>ValueList</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Note that <code>item_type</code> is immutable.</p>
    *
    * **Parameters:**
    *
    * * `value_list: &str` -- The account's country.
    */
    pub async fn post_radar_value_lists_list(
        &self,
        value_list: &str,
    ) -> Result<crate::types::RadarList> {
        let url = format!(
            "/v1/radar/value_lists/{}",
            crate::progenitor_support::encode_path(&value_list.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/radar/value_lists/{value_list}` endpoint.
    *
    * <p>Deletes a <code>ValueList</code> object, also deleting any items contained within the value list. To be deleted, a value list must not be referenced in any rules.</p>
    *
    * **Parameters:**
    *
    * * `value_list: &str` -- The account's country.
    */
    pub async fn delete_radar_value_lists_list(
        &self,
        value_list: &str,
    ) -> Result<crate::types::RadarListDeleted> {
        let url = format!(
            "/v1/radar/value_lists/{}",
            crate::progenitor_support::encode_path(&value_list.to_string()),
        );

        self.client.delete(&url, None).await
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
    pub async fn get_recipients(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        type_: crate::types::GetRecipientsType,
        verified: bool,
    ) -> Result<crate::types::GetRecipientsResponse> {
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

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/recipients` endpoint.
    *
    * <p>Creates a new <code>Recipient</code> object and verifies the recipient’s identity.
    * Also verifies the recipient’s bank account information or debit card, if either is provided.</p>
    */
    pub async fn post_recipient(&self) -> Result<crate::types::Recipient> {
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
    pub async fn get_recipient(
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
    pub async fn post_recipient_v_1(&self, id: &str) -> Result<crate::types::Recipient> {
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
    pub async fn delete_recipients(&self, id: &str) -> Result<crate::types::DeletedRecipient> {
        let url = format!(
            "/v1/recipients/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/refunds` endpoint.
    *
    * <p>Returns a list of all refunds you’ve previously created. The refunds are returned in sorted order, with the most recent refunds appearing first. For convenience, the 10 most recent refunds are always available by default on the charge object.</p>
    *
    * **Parameters:**
    *
    * * `charge: &str` -- Only return refunds for the charge specified by this charge ID.
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `payment_intent: &str` -- Only return refunds for the PaymentIntent specified by this ID.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_refund(
        &self,
        charge: &str,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        payment_intent: &str,
        starting_after: &str,
    ) -> Result<crate::types::RefundList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/refunds?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/refunds` endpoint.
    *
    * <p>Create a refund.</p>
    */
    pub async fn post_refund(&self) -> Result<crate::types::Refund> {
        let url = "/v1/refunds".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/refunds/{refund}` endpoint.
    *
    * <p>Retrieves the details of an existing refund.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `refund: &str` -- The account's country.
    */
    pub async fn get_refunds_refund(
        &self,
        expand: &[String],
        refund: &str,
    ) -> Result<crate::types::Refund> {
        let url = format!(
            "/v1/refunds/{}",
            crate::progenitor_support::encode_path(&refund.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/refunds/{refund}` endpoint.
    *
    * <p>Updates the specified refund by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * <p>This request only accepts <code>metadata</code> as an argument.</p>
    *
    * **Parameters:**
    *
    * * `refund: &str` -- The account's country.
    */
    pub async fn post_refunds_refund(&self, refund: &str) -> Result<crate::types::Refund> {
        let url = format!(
            "/v1/refunds/{}",
            crate::progenitor_support::encode_path(&refund.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/refunds/{refund}/cancel` endpoint.
    *
    * <p>Cancels a refund with a status of <code>requires_action</code>.</p>
    *
    * <p>Refunds in other states cannot be canceled, and only refunds for payment methods that require customer action will enter the <code>requires_action</code> state.</p>
    *
    * **Parameters:**
    *
    * * `refund: &str` -- The account's country.
    */
    pub async fn post_refunds_refund_cancel(&self, refund: &str) -> Result<crate::types::Refund> {
        let url = format!(
            "/v1/refunds/{}/cancel",
            crate::progenitor_support::encode_path(&refund.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/reporting/report_runs` endpoint.
    *
    * <p>Returns a list of Report Runs, with the most recent appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_reporting_report_runs(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetReportingReportRunsResponse> {
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
        let url = format!("/v1/reporting/report_runs?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/reporting/report_runs` endpoint.
    *
    * <p>Creates a new object and begin running the report. (Certain report types require a <a href="https://stripe.com/docs/keys#test-live-modes">live-mode API key</a>.)</p>
    */
    pub async fn post_reporting_report_run(&self) -> Result<crate::types::ReportingReportRun> {
        let url = "/v1/reporting/report_runs".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/reporting/report_runs/{report_run}` endpoint.
    *
    * <p>Retrieves the details of an existing Report Run.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `report_run: &str` -- The account's country.
    */
    pub async fn get_reporting_report_runs_run(
        &self,
        expand: &[String],
        report_run: &str,
    ) -> Result<crate::types::ReportingReportRun> {
        let url = format!(
            "/v1/reporting/report_runs/{}",
            crate::progenitor_support::encode_path(&report_run.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/reporting/report_types` endpoint.
    *
    * <p>Returns a full list of Report Types.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    */
    pub async fn get_reporting_report_type(
        &self,
        expand: &[String],
    ) -> Result<crate::types::FinancialReportingFinanceReportTypeList> {
        let url = "/v1/reporting/report_types".to_string();
        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/reporting/report_types/{report_type}` endpoint.
    *
    * <p>Retrieves the details of a Report Type. (Certain report types require a <a href="https://stripe.com/docs/keys#test-live-modes">live-mode API key</a>.)</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `report_type: &str` -- The account's country.
    */
    pub async fn get_reporting_report_types_type(
        &self,
        expand: &[String],
        report_type: &str,
    ) -> Result<crate::types::ReportingReportType> {
        let url = format!(
            "/v1/reporting/report_types/{}",
            crate::progenitor_support::encode_path(&report_type.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/reviews` endpoint.
    *
    * <p>Returns a list of <code>Review</code> objects that have <code>open</code> set to <code>true</code>. The objects are sorted in descending order by creation date, with the most recently created object appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_reviews(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetReviewsResponse> {
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
        let url = format!("/v1/reviews?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/reviews/{review}` endpoint.
    *
    * <p>Retrieves a <code>Review</code> object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `review: &str` -- The account's country.
    */
    pub async fn get_reviews_review(
        &self,
        expand: &[String],
        review: &str,
    ) -> Result<crate::types::Review> {
        let url = format!(
            "/v1/reviews/{}",
            crate::progenitor_support::encode_path(&review.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/reviews/{review}/approve` endpoint.
    *
    * <p>Approves a <code>Review</code> object, closing it and removing it from the list of reviews.</p>
    *
    * **Parameters:**
    *
    * * `review: &str` -- The account's country.
    */
    pub async fn post_reviews_review_approve(&self, review: &str) -> Result<crate::types::Review> {
        let url = format!(
            "/v1/reviews/{}/approve",
            crate::progenitor_support::encode_path(&review.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/setup_attempts` endpoint.
    *
    * <p>Returns a list of SetupAttempts associated with a provided SetupIntent.</p>
    *
    * **Parameters:**
    *
    * * `created: &str` -- A filter on the list, based on the object `created` field. The value
    *   can be a string with an integer Unix timestamp, or it can be a
    *   dictionary with a number of different query options.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `setup_intent: &str` -- Only return SetupAttempts created by the SetupIntent specified by
    *   this ID.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_setup_attempt(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        setup_intent: &str,
        starting_after: &str,
    ) -> Result<crate::types::PaymentFlowsSetupIntentAttemptList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !setup_intent.is_empty() {
            query_args.push(("setup_intent".to_string(), setup_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/setup_attempts?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/setup_intents` endpoint.
    *
    * <p>Returns a list of SetupIntents.</p>
    *
    * **Parameters:**
    *
    * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    * * `customer: &str` -- Only return SetupIntents for the customer specified by this customer ID.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `payment_method: &str` -- Only return SetupIntents associated with the specified payment method.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_setup_intent(
        &self,
        created: &str,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        payment_method: &str,
        starting_after: &str,
    ) -> Result<crate::types::PaymentFlowsSetupIntentList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payment_method.is_empty() {
            query_args.push(("payment_method".to_string(), payment_method.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/setup_intents?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/setup_intents` endpoint.
    *
    * <p>Creates a SetupIntent object.</p>
    *
    * <p>After the SetupIntent is created, attach a payment method and <a href="/docs/api/setup_intents/confirm">confirm</a>
    * to collect any required permissions to charge the payment method later.</p>
    */
    pub async fn post_setup_intent(&self) -> Result<crate::types::SetupIntent> {
        let url = "/v1/setup_intents".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/setup_intents/{intent}` endpoint.
    *
    * <p>Retrieves the details of a SetupIntent that has previously been created. </p>
    *
    * <p>Client-side retrieval using a publishable key is allowed when the <code>client_secret</code> is provided in the query string. </p>
    *
    * <p>When retrieved with a publishable key, only a subset of properties will be returned. Please refer to the <a href="#setup_intent_object">SetupIntent</a> object reference for more details.</p>
    *
    * **Parameters:**
    *
    * * `client_secret: &str` -- The client secret of the SetupIntent. Required if a publishable key is used to retrieve the SetupIntent.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `intent: &str` -- The account's country.
    */
    pub async fn get_setup_intents_intent(
        &self,
        client_secret: &str,
        expand: &[String],
        intent: &str,
    ) -> Result<crate::types::SetupIntent> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_secret.is_empty() {
            query_args.push(("client_secret".to_string(), client_secret.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/v1/setup_intents/{}?{}",
            crate::progenitor_support::encode_path(&intent.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/setup_intents/{intent}` endpoint.
    *
    * <p>Updates a SetupIntent object.</p>
    *
    * **Parameters:**
    *
    * * `intent: &str` -- The account's country.
    */
    pub async fn post_setup_intents_intent(
        &self,
        intent: &str,
    ) -> Result<crate::types::SetupIntent> {
        let url = format!(
            "/v1/setup_intents/{}",
            crate::progenitor_support::encode_path(&intent.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/setup_intents/{intent}/cancel` endpoint.
    *
    * <p>A SetupIntent object can be canceled when it is in one of these statuses: <code>requires_payment_method</code>, <code>requires_confirmation</code>, or <code>requires_action</code>. </p>
    *
    * <p>Once canceled, setup is abandoned and any operations on the SetupIntent will fail with an error.</p>
    *
    * **Parameters:**
    *
    * * `intent: &str` -- The account's country.
    */
    pub async fn post_setup_intents_intent_cancel(
        &self,
        intent: &str,
    ) -> Result<crate::types::SetupIntent> {
        let url = format!(
            "/v1/setup_intents/{}/cancel",
            crate::progenitor_support::encode_path(&intent.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/setup_intents/{intent}/confirm` endpoint.
    *
    * <p>Confirm that your customer intends to set up the current or
    * provided payment method. For example, you would confirm a SetupIntent
    * when a customer hits the “Save” button on a payment method management
    * page on your website.</p>
    *
    * <p>If the selected payment method does not require any additional
    * steps from the customer, the SetupIntent will transition to the
    * <code>succeeded</code> status.</p>
    *
    * <p>Otherwise, it will transition to the <code>requires_action</code> status and
    * suggest additional actions via <code>next_action</code>. If setup fails,
    * the SetupIntent will transition to the
    * <code>requires_payment_method</code> status.</p>
    *
    * **Parameters:**
    *
    * * `intent: &str` -- The account's country.
    */
    pub async fn post_setup_intents_intent_confirm(
        &self,
        intent: &str,
    ) -> Result<crate::types::SetupIntent> {
        let url = format!(
            "/v1/setup_intents/{}/confirm",
            crate::progenitor_support::encode_path(&intent.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/setup_intents/{intent}/verify_microdeposits` endpoint.
    *
    * <p>Verifies microdeposits on a SetupIntent object.</p>
    *
    * **Parameters:**
    *
    * * `intent: &str` -- The account's country.
    */
    pub async fn post_setup_intents_intent_verify_microdeposit(
        &self,
        intent: &str,
    ) -> Result<crate::types::SetupIntent> {
        let url = format!(
            "/v1/setup_intents/{}/verify_microdeposits",
            crate::progenitor_support::encode_path(&intent.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/shipping_rates` endpoint.
    *
    * <p>Returns a list of your shipping rates.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Only return shipping rates that are active or inactive.
    * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    * * `currency: &str` -- Only return shipping rates for the given currency.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_shipping_rate(
        &self,
        active: bool,
        created: &str,
        currency: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::ShippingResourcesRateList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
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
        let url = format!("/v1/shipping_rates?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/shipping_rates` endpoint.
    *
    * <p>Creates a new shipping rate object.</p>
    */
    pub async fn post_shipping_rate(&self) -> Result<crate::types::ShippingRate> {
        let url = "/v1/shipping_rates".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/shipping_rates/{shipping_rate_token}` endpoint.
    *
    * <p>Returns the shipping rate object with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `shipping_rate_token: &str` -- The account's country.
    */
    pub async fn get_shipping_rates_rate_token(
        &self,
        expand: &[String],
        shipping_rate_token: &str,
    ) -> Result<crate::types::ShippingRate> {
        let url = format!(
            "/v1/shipping_rates/{}",
            crate::progenitor_support::encode_path(&shipping_rate_token.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/shipping_rates/{shipping_rate_token}` endpoint.
    *
    * <p>Updates an existing shipping rate object.</p>
    *
    * **Parameters:**
    *
    * * `shipping_rate_token: &str` -- The account's country.
    */
    pub async fn post_shipping_rates_rate_token(
        &self,
        shipping_rate_token: &str,
    ) -> Result<crate::types::ShippingRate> {
        let url = format!(
            "/v1/shipping_rates/{}",
            crate::progenitor_support::encode_path(&shipping_rate_token.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/sigma/scheduled_query_runs` endpoint.
    *
    * <p>Returns a list of scheduled query runs.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_sigma_scheduled_query_runs(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetSigmaScheduledQueryRunsResponse> {
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
        let url = format!("/v1/sigma/scheduled_query_runs?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/sigma/scheduled_query_runs/{scheduled_query_run}` endpoint.
    *
    * <p>Retrieves the details of an scheduled query run.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `scheduled_query_run: &str` -- The account's country.
    */
    pub async fn get_sigma_scheduled_query_runs_run(
        &self,
        expand: &[String],
        scheduled_query_run: &str,
    ) -> Result<crate::types::ScheduledQueryRun> {
        let url = format!(
            "/v1/sigma/scheduled_query_runs/{}",
            crate::progenitor_support::encode_path(&scheduled_query_run.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/skus` endpoint.
    *
    * <p>Returns a list of your SKUs. The SKUs are returned sorted by creation date, with the most recently created SKUs appearing first.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Only return SKUs that are active or inactive (e.g., pass `false` to list all inactive products).
    * * `attributes: &str` -- Only return SKUs that have the specified key-value pairs in this partially constructed dictionary. Can be specified only if `product` is also supplied. For instance, if the associated product has attributes `["color", "size"]`, passing in `attributes[color]=red` returns all the SKUs for this product that have `color` set to `red`.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `ids: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `in_stock: bool` -- Only return SKUs that are either in stock or out of stock (e.g., pass `false` to list all SKUs that are out of stock). If no value is provided, all SKUs are returned.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `product: &str` -- The ID of the product whose SKUs will be retrieved. Must be a product with type `good`.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_skus(
        &self,
        active: bool,
        attributes: &str,
        ending_before: &str,
        expand: &[String],
        ids: &[String],
        in_stock: bool,
        limit: i64,
        product: &str,
        starting_after: &str,
    ) -> Result<crate::types::GetSkusResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if in_stock {
            query_args.push(("in_stock".to_string(), in_stock.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/skus?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/skus` endpoint.
    *
    * <p>Creates a new SKU associated with a product.</p>
    */
    pub async fn post_sku(&self) -> Result<crate::types::Sku> {
        let url = "/v1/skus".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/skus/{id}` endpoint.
    *
    * <p>Retrieves the details of an existing SKU. Supply the unique SKU identifier from either a SKU creation request or from the product, and Stripe will return the corresponding SKU information.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_sku(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::GetSkusResponseAnyOf> {
        let url = format!(
            "/v1/skus/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/skus/{id}` endpoint.
    *
    * <p>Updates the specific SKU by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * <p>Note that a SKU’s <code>attributes</code> are not editable. Instead, you would need to deactivate the existing SKU and create a new one with the new attribute values.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_sku_v_1(&self, id: &str) -> Result<crate::types::Sku> {
        let url = format!(
            "/v1/skus/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/skus/{id}` endpoint.
    *
    * <p>Delete a SKU. Deleting a SKU is only possible until it has been used in an order.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn delete_skus(&self, id: &str) -> Result<crate::types::DeletedSku> {
        let url = format!(
            "/v1/skus/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/sources` endpoint.
    *
    * <p>Creates a new source object.</p>
    */
    pub async fn post_source(&self) -> Result<crate::types::SourceData> {
        let url = "/v1/sources".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/sources/{source}` endpoint.
    *
    * <p>Retrieves an existing source object. Supply the unique source ID from a source creation request and Stripe will return the corresponding up-to-date source object information.</p>
    *
    * **Parameters:**
    *
    * * `client_secret: &str` -- The client secret of the source. Required if a publishable key is used to retrieve the source.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `source: &str` -- The account's country.
    */
    pub async fn get_sources_source(
        &self,
        client_secret: &str,
        expand: &[String],
        source: &str,
    ) -> Result<crate::types::SourceData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_secret.is_empty() {
            query_args.push(("client_secret".to_string(), client_secret.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/v1/sources/{}?{}",
            crate::progenitor_support::encode_path(&source.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/sources/{source}` endpoint.
    *
    * <p>Updates the specified source by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * <p>This request accepts the <code>metadata</code> and <code>owner</code> as arguments. It is also possible to update type specific information for selected payment methods. Please refer to our <a href="/docs/sources">payment method guides</a> for more detail.</p>
    *
    * **Parameters:**
    *
    * * `source: &str` -- The account's country.
    */
    pub async fn post_sources_source(&self, source: &str) -> Result<crate::types::SourceData> {
        let url = format!(
            "/v1/sources/{}",
            crate::progenitor_support::encode_path(&source.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/sources/{source}/mandate_notifications/{mandate_notification}` endpoint.
    *
    * <p>Retrieves a new Source MandateNotification.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `mandate_notification: &str` -- The account's country.
    * * `source: &str` -- The account's country.
    */
    pub async fn get_sources_source_mandate_notifications_notification(
        &self,
        expand: &[String],
        mandate_notification: &str,
        source: &str,
    ) -> Result<crate::types::SourceMandateNotification> {
        let url = format!(
            "/v1/sources/{}/mandate_notifications/{}",
            crate::progenitor_support::encode_path(&source.to_string()),
            crate::progenitor_support::encode_path(&mandate_notification.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/sources/{source}/source_transactions` endpoint.
    *
    * <p>List source transactions for a given source.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `source: &str` -- The account's country.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_sources_source_transaction(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        source: &str,
        starting_after: &str,
    ) -> Result<crate::types::ApmsSourcesSourceTransactionList> {
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
        let url = format!(
            "/v1/sources/{}/source_transactions?{}",
            crate::progenitor_support::encode_path(&source.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/sources/{source}/source_transactions/{source_transaction}` endpoint.
    *
    * <p>Retrieve an existing source transaction object. Supply the unique source ID from a source creation request and the source transaction ID and Stripe will return the corresponding up-to-date source object information.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `source: &str` -- The account's country.
    * * `source_transaction: &str` -- The account's country.
    */
    pub async fn get_sources_source_transactions_transaction(
        &self,
        expand: &[String],
        source: &str,
        source_transaction: &str,
    ) -> Result<crate::types::SourceTransaction> {
        let url = format!(
            "/v1/sources/{}/source_transactions/{}",
            crate::progenitor_support::encode_path(&source.to_string()),
            crate::progenitor_support::encode_path(&source_transaction.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/sources/{source}/verify` endpoint.
    *
    * <p>Verify a given source.</p>
    *
    * **Parameters:**
    *
    * * `source: &str` -- The account's country.
    */
    pub async fn post_sources_source_verify(
        &self,
        source: &str,
    ) -> Result<crate::types::SourceData> {
        let url = format!(
            "/v1/sources/{}/verify",
            crate::progenitor_support::encode_path(&source.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/subscription_items` endpoint.
    *
    * <p>Returns a list of your subscription items for a given subscription.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `subscription: &str` -- The ID of the subscription whose items will be retrieved.
    */
    pub async fn get_subscription_item(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        subscription: &str,
    ) -> Result<crate::types::Items> {
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
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/subscription_items?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/subscription_items` endpoint.
    *
    * <p>Adds a new item to an existing subscription. No existing items will be changed or replaced.</p>
    */
    pub async fn post_subscription_item(&self) -> Result<crate::types::SubscriptionItem> {
        let url = "/v1/subscription_items".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/subscription_items/{item}` endpoint.
    *
    * <p>Retrieves the subscription item with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `item: &str` -- The account's country.
    */
    pub async fn get_subscription_items_item(
        &self,
        expand: &[String],
        item: &str,
    ) -> Result<crate::types::SubscriptionItem> {
        let url = format!(
            "/v1/subscription_items/{}",
            crate::progenitor_support::encode_path(&item.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/subscription_items/{item}` endpoint.
    *
    * <p>Updates the plan or quantity of an item on a current subscription.</p>
    *
    * **Parameters:**
    *
    * * `item: &str` -- The account's country.
    */
    pub async fn post_subscription_items_item(
        &self,
        item: &str,
    ) -> Result<crate::types::SubscriptionItem> {
        let url = format!(
            "/v1/subscription_items/{}",
            crate::progenitor_support::encode_path(&item.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/subscription_items/{item}` endpoint.
    *
    * <p>Deletes an item from the subscription. Removing a subscription item from a subscription will not cancel the subscription.</p>
    *
    * **Parameters:**
    *
    * * `item: &str` -- The account's country.
    */
    pub async fn delete_subscription_items_item(
        &self,
        item: &str,
    ) -> Result<crate::types::DeletedSubscriptionItem> {
        let url = format!(
            "/v1/subscription_items/{}",
            crate::progenitor_support::encode_path(&item.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/subscription_items/{subscription_item}/usage_record_summaries` endpoint.
    *
    * <p>For the specified subscription item, returns a list of summary objects. Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).</p>
    *
    * <p>The list is sorted in reverse-chronological order (newest first). The first list item represents the most current usage period that hasn’t ended yet. Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `subscription_item: &str` -- The account's country.
    */
    pub async fn get_subscription_items_item_usage_record_summaries(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        subscription_item: &str,
    ) -> Result<crate::types::GetSubscriptionItemsItemUsageRecordSummariesResponse> {
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
        let url = format!(
            "/v1/subscription_items/{}/usage_record_summaries?{}",
            crate::progenitor_support::encode_path(&subscription_item.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/subscription_items/{subscription_item}/usage_records` endpoint.
    *
    * <p>Creates a usage record for a specified subscription item and date, and fills it with a quantity.</p>
    *
    * <p>Usage records provide <code>quantity</code> information that Stripe uses to track how much a customer is using your service. With usage information and the pricing model set up by the <a href="https://stripe.com/docs/billing/subscriptions/metered-billing">metered billing</a> plan, Stripe helps you send accurate invoices to your customers.</p>
    *
    * <p>The default calculation for usage is to add up all the <code>quantity</code> values of the usage records within a billing period. You can change this default behavior with the billing plan’s <code>aggregate_usage</code> <a href="/docs/api/plans/create#create_plan-aggregate_usage">parameter</a>. When there is more than one usage record with the same timestamp, Stripe adds the <code>quantity</code> values together. In most cases, this is the desired resolution, however, you can change this behavior with the <code>action</code> parameter.</p>
    *
    * <p>The default pricing model for metered billing is <a href="/docs/api/plans/object#plan_object-billing_scheme">per-unit pricing</a>. For finer granularity, you can configure metered billing to have a <a href="https://stripe.com/docs/billing/subscriptions/tiers">tiered pricing</a> model.</p>
    *
    * **Parameters:**
    *
    * * `subscription_item: &str` -- The account's country.
    */
    pub async fn post_subscription_items_item_usage_record(
        &self,
        subscription_item: &str,
    ) -> Result<crate::types::UsageRecord> {
        let url = format!(
            "/v1/subscription_items/{}/usage_records",
            crate::progenitor_support::encode_path(&subscription_item.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/subscription_schedules` endpoint.
    *
    * <p>Retrieves the list of your subscription schedules.</p>
    *
    * **Parameters:**
    *
    * * `canceled_at: &str` -- Only return subscription schedules that were created canceled the given date interval.
    * * `completed_at: &str` -- Only return subscription schedules that completed during the given date interval.
    * * `created: &str` -- Only return subscription schedules that were created during the given date interval.
    * * `customer: &str` -- Only return subscription schedules for the given customer.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `released_at: &str` -- Only return subscription schedules that were released during the given date interval.
    * * `scheduled: bool` -- Only return subscription schedules that have not started yet.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_subscription_schedules(
        &self,
        canceled_at: &str,
        completed_at: &str,
        created: &str,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        released_at: &str,
        scheduled: bool,
        starting_after: &str,
    ) -> Result<crate::types::GetSubscriptionSchedulesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if scheduled {
            query_args.push(("scheduled".to_string(), scheduled.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/subscription_schedules?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/subscription_schedules` endpoint.
    *
    * <p>Creates a new subscription schedule object. Each customer can have up to 500 active or scheduled subscriptions.</p>
    */
    pub async fn post_subscription_schedule(&self) -> Result<crate::types::SubscriptionSchedule> {
        let url = "/v1/subscription_schedules".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/subscription_schedules/{schedule}` endpoint.
    *
    * <p>Retrieves the details of an existing subscription schedule. You only need to supply the unique subscription schedule identifier that was returned upon subscription schedule creation.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `schedule: &str` -- The account's country.
    */
    pub async fn get_subscription_schedules_schedule(
        &self,
        expand: &[String],
        schedule: &str,
    ) -> Result<crate::types::SubscriptionSchedule> {
        let url = format!(
            "/v1/subscription_schedules/{}",
            crate::progenitor_support::encode_path(&schedule.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/subscription_schedules/{schedule}` endpoint.
    *
    * <p>Updates an existing subscription schedule.</p>
    *
    * **Parameters:**
    *
    * * `schedule: &str` -- The account's country.
    */
    pub async fn post_subscription_schedules_schedule(
        &self,
        schedule: &str,
    ) -> Result<crate::types::SubscriptionSchedule> {
        let url = format!(
            "/v1/subscription_schedules/{}",
            crate::progenitor_support::encode_path(&schedule.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/subscription_schedules/{schedule}/cancel` endpoint.
    *
    * <p>Cancels a subscription schedule and its associated subscription immediately (if the subscription schedule has an active subscription). A subscription schedule can only be canceled if its status is <code>not_started</code> or <code>active</code>.</p>
    *
    * **Parameters:**
    *
    * * `schedule: &str` -- The account's country.
    */
    pub async fn post_subscription_schedules_schedule_cancel(
        &self,
        schedule: &str,
    ) -> Result<crate::types::SubscriptionSchedule> {
        let url = format!(
            "/v1/subscription_schedules/{}/cancel",
            crate::progenitor_support::encode_path(&schedule.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/subscription_schedules/{schedule}/release` endpoint.
    *
    * <p>Releases the subscription schedule immediately, which will stop scheduling of its phases, but leave any existing subscription in place. A schedule can only be released if its status is <code>not_started</code> or <code>active</code>. If the subscription schedule is currently associated with a subscription, releasing it will remove its <code>subscription</code> property and set the subscription’s ID to the <code>released_subscription</code> property.</p>
    *
    * **Parameters:**
    *
    * * `schedule: &str` -- The account's country.
    */
    pub async fn post_subscription_schedules_schedule_release(
        &self,
        schedule: &str,
    ) -> Result<crate::types::SubscriptionSchedule> {
        let url = format!(
            "/v1/subscription_schedules/{}/release",
            crate::progenitor_support::encode_path(&schedule.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/subscriptions` endpoint.
    *
    * <p>By default, returns a list of subscriptions that have not been canceled. In order to list canceled subscriptions, specify <code>status=canceled</code>.</p>
    *
    * **Parameters:**
    *
    * * `collection_method: crate::types::CollectionMethod` -- Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer. When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    * * `created: &str`
    * * `current_period_end: &str`
    * * `current_period_start: &str`
    * * `customer: &str` -- The ID of the customer whose subscriptions will be retrieved.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `price: &str` -- Filter for subscriptions that contain this recurring price ID.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::GetSubscriptionsStatus` -- The status of the subscriptions to retrieve. Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers. Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://stripe.com/docs/billing/subscriptions/overview#subscription-statuses). Passing in a value of `all` will return subscriptions of all statuses. If no value is supplied, all subscriptions that have not been canceled are returned.
    * * `test_clock: &str` -- Filter for subscriptions that are associated with the specified test clock. The response will not include subscriptions with test clocks if this and the customer parameter is not set.
    */
    pub async fn get_subscription(
        &self,
        collection_method: crate::types::CollectionMethod,
        created: &str,
        current_period_end: &str,
        current_period_start: &str,
        customer: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        price: &str,
        starting_after: &str,
        status: crate::types::GetSubscriptionsStatus,
        test_clock: &str,
    ) -> Result<crate::types::Subscriptions> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_method.to_string().is_empty() {
            query_args.push((
                "collection_method".to_string(),
                collection_method.to_string(),
            ));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !price.is_empty() {
            query_args.push(("price".to_string(), price.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !test_clock.is_empty() {
            query_args.push(("test_clock".to_string(), test_clock.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/subscriptions?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/subscriptions` endpoint.
    *
    * <p>Creates a new subscription on an existing customer. Each customer can have up to 500 active or scheduled subscriptions.</p>
    *
    * <p>When you create a subscription with <code>collection_method=charge_automatically</code>, the first invoice is finalized as part of the request.
    * The <code>payment_behavior</code> parameter determines the exact behavior of the initial payment.</p>
    *
    * <p>To start subscriptions where the first invoice always begins in a <code>draft</code> status, use <a href="/docs/billing/subscriptions/subscription-schedules#managing">subscription schedules</a> instead.
    * Schedules provide the flexibility to model more complex billing configurations that change over time.</p>
    */
    pub async fn post_subscription(&self) -> Result<crate::types::Subscription> {
        let url = "/v1/subscriptions".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/subscriptions/search` endpoint.
    *
    * <p>Search for subscriptions you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
    * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
    * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
    * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
    * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for subscriptions](https://stripe.com/docs/search#query-fields-for-subscriptions).
    */
    pub async fn get_subscriptions_search(
        &self,
        expand: &[String],
        limit: i64,
        page: &str,
        query: &str,
    ) -> Result<crate::types::SearchResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/subscriptions/search?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/subscriptions/{subscription_exposed_id}` endpoint.
    *
    * <p>Retrieves the subscription with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `subscription_exposed_id: &str` -- The account's country.
    */
    pub async fn get_subscriptions_subscription_exposed(
        &self,
        expand: &[String],
        subscription_exposed_id: &str,
    ) -> Result<crate::types::Subscription> {
        let url = format!(
            "/v1/subscriptions/{}",
            crate::progenitor_support::encode_path(&subscription_exposed_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/subscriptions/{subscription_exposed_id}` endpoint.
    *
    * <p>Updates an existing subscription on a customer to match the specified parameters. When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes. To preview how the proration will be calculated, use the <a href="#upcoming_invoice">upcoming invoice</a> endpoint.</p>
    *
    * **Parameters:**
    *
    * * `subscription_exposed_id: &str` -- The account's country.
    */
    pub async fn post_subscriptions_subscription_exposed(
        &self,
        subscription_exposed_id: &str,
    ) -> Result<crate::types::Subscription> {
        let url = format!(
            "/v1/subscriptions/{}",
            crate::progenitor_support::encode_path(&subscription_exposed_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/subscriptions/{subscription_exposed_id}` endpoint.
    *
    * <p>Cancels a customer’s subscription immediately. The customer will not be charged again for the subscription.</p>
    *
    * <p>Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually <a href="#delete_invoiceitem">deleted</a>. If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period. But if the subscription is set to cancel immediately, pending prorations will be removed.</p>
    *
    * <p>By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer. This is intended to prevent unexpected payment attempts after the customer has canceled a subscription. However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed. Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.</p>
    *
    * **Parameters:**
    *
    * * `subscription_exposed_id: &str` -- The account's country.
    */
    pub async fn delete_subscriptions_subscription_exposed(
        &self,
        subscription_exposed_id: &str,
    ) -> Result<crate::types::Subscription> {
        let url = format!(
            "/v1/subscriptions/{}",
            crate::progenitor_support::encode_path(&subscription_exposed_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/subscriptions/{subscription_exposed_id}/discount` endpoint.
    *
    * <p>Removes the currently applied discount on a subscription.</p>
    *
    * **Parameters:**
    *
    * * `subscription_exposed_id: &str` -- The account's country.
    */
    pub async fn delete_subscriptions_subscription_exposed_discount(
        &self,
        subscription_exposed_id: &str,
    ) -> Result<crate::types::DeletedDiscount> {
        let url = format!(
            "/v1/subscriptions/{}/discount",
            crate::progenitor_support::encode_path(&subscription_exposed_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/tax_codes` endpoint.
    *
    * <p>A list of <a href="https://stripe.com/docs/tax/tax-codes">all tax codes available</a> to add to Products in order to allow specific tax calculations.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_tax_code(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::TaxProductResourceCodeList> {
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
        let url = format!("/v1/tax_codes?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/tax_codes/{id}` endpoint.
    *
    * <p>Retrieves the details of an existing tax code. Supply the unique tax code ID and Stripe will return the corresponding tax code information.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    */
    pub async fn get_tax_code_v_1(
        &self,
        expand: &[String],
        id: &str,
    ) -> Result<crate::types::TaxCode> {
        let url = format!(
            "/v1/tax_codes/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/tax_rates` endpoint.
    *
    * <p>Returns a list of your tax rates. Tax rates are returned sorted by creation date, with the most recently created tax rates appearing first.</p>
    *
    * **Parameters:**
    *
    * * `active: bool` -- Optional flag to filter by tax rates that are either active or inactive (archived).
    * * `created: &str` -- Optional range for filtering created date.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `inclusive: bool` -- Optional flag to filter by tax rates that are inclusive (or those that are not inclusive).
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_tax_rates(
        &self,
        active: bool,
        created: &str,
        ending_before: &str,
        expand: &[String],
        inclusive: bool,
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetTaxRatesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if inclusive {
            query_args.push(("inclusive".to_string(), inclusive.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/tax_rates?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/tax_rates` endpoint.
    *
    * <p>Creates a new tax rate.</p>
    */
    pub async fn post_tax_rate(&self) -> Result<crate::types::TaxRate> {
        let url = "/v1/tax_rates".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/tax_rates/{tax_rate}` endpoint.
    *
    * <p>Retrieves a tax rate with the given ID</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `tax_rate: &str` -- The account's country.
    */
    pub async fn get_tax_rates_rate(
        &self,
        expand: &[String],
        tax_rate: &str,
    ) -> Result<crate::types::TaxRate> {
        let url = format!(
            "/v1/tax_rates/{}",
            crate::progenitor_support::encode_path(&tax_rate.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/tax_rates/{tax_rate}` endpoint.
    *
    * <p>Updates an existing tax rate.</p>
    *
    * **Parameters:**
    *
    * * `tax_rate: &str` -- The account's country.
    */
    pub async fn post_tax_rates_rate(&self, tax_rate: &str) -> Result<crate::types::TaxRate> {
        let url = format!(
            "/v1/tax_rates/{}",
            crate::progenitor_support::encode_path(&tax_rate.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/terminal/connection_tokens` endpoint.
    *
    * <p>To connect to a reader the Stripe Terminal SDK needs to retrieve a short-lived connection token from Stripe, proxied through your server. On your backend, add an endpoint that creates and returns a connection token.</p>
    */
    pub async fn post_terminal_connection_token(
        &self,
    ) -> Result<crate::types::TerminalConnectionToken> {
        let url = "/v1/terminal/connection_tokens".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/terminal/locations` endpoint.
    *
    * <p>Returns a list of <code>Location</code> objects.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_terminal_location(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::TerminalLocationList> {
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
        let url = format!("/v1/terminal/locations?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/terminal/locations` endpoint.
    *
    * <p>Creates a new <code>Location</code> object.
    * For further details, including which address fields are required in each country, see the <a href="/docs/terminal/fleet/locations">Manage locations</a> guide.</p>
    */
    pub async fn post_terminal_location(&self) -> Result<crate::types::TerminalLocation> {
        let url = "/v1/terminal/locations".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/terminal/locations/{location}` endpoint.
    *
    * <p>Retrieves a <code>Location</code> object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `location: &str` -- The account's country.
    */
    pub async fn get_terminal_locations_location(
        &self,
        expand: &[String],
        location: &str,
    ) -> Result<crate::types::GetTerminalLocationResponseAnyOf> {
        let url = format!(
            "/v1/terminal/locations/{}",
            crate::progenitor_support::encode_path(&location.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/terminal/locations/{location}` endpoint.
    *
    * <p>Updates a <code>Location</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `location: &str` -- The account's country.
    */
    pub async fn post_terminal_locations_location(
        &self,
        location: &str,
    ) -> Result<crate::types::GetTerminalLocationResponseAnyOf> {
        let url = format!(
            "/v1/terminal/locations/{}",
            crate::progenitor_support::encode_path(&location.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/terminal/locations/{location}` endpoint.
    *
    * <p>Deletes a <code>Location</code> object.</p>
    *
    * **Parameters:**
    *
    * * `location: &str` -- The account's country.
    */
    pub async fn delete_terminal_locations_location(
        &self,
        location: &str,
    ) -> Result<crate::types::DeletedTerminalLocation> {
        let url = format!(
            "/v1/terminal/locations/{}",
            crate::progenitor_support::encode_path(&location.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/terminal/readers` endpoint.
    *
    * <p>Returns a list of <code>Reader</code> objects.</p>
    *
    * **Parameters:**
    *
    * * `device_type: crate::types::DeviceType` -- Type of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, or `verifone_P400`.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `location: &str` -- A location ID to filter the response list to only readers at the specific location.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::CustomerAcceptanceType` -- The type of customer acceptance information included with the Mandate. One of `online` or `offline`.
    */
    pub async fn get_terminal_reader(
        &self,
        device_type: crate::types::DeviceType,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        location: &str,
        starting_after: &str,
        status: crate::types::CustomerAcceptanceType,
    ) -> Result<crate::types::TerminalReaderRetrieve> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !device_type.to_string().is_empty() {
            query_args.push(("device_type".to_string(), device_type.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !location.is_empty() {
            query_args.push(("location".to_string(), location.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/terminal/readers?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/terminal/readers` endpoint.
    *
    * <p>Creates a new <code>Reader</code> object.</p>
    */
    pub async fn post_terminal_reader(&self) -> Result<crate::types::TerminalReader> {
        let url = "/v1/terminal/readers".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/terminal/readers/{reader}` endpoint.
    *
    * <p>Retrieves a <code>Reader</code> object.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `reader: &str` -- The account's country.
    */
    pub async fn get_terminal_readers_reader(
        &self,
        expand: &[String],
        reader: &str,
    ) -> Result<crate::types::GetTerminalReadersReaderResponseAnyOf> {
        let url = format!(
            "/v1/terminal/readers/{}",
            crate::progenitor_support::encode_path(&reader.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/terminal/readers/{reader}` endpoint.
    *
    * <p>Updates a <code>Reader</code> object by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * **Parameters:**
    *
    * * `reader: &str` -- The account's country.
    */
    pub async fn post_terminal_readers_reader(
        &self,
        reader: &str,
    ) -> Result<crate::types::GetTerminalReadersReaderResponseAnyOf> {
        let url = format!(
            "/v1/terminal/readers/{}",
            crate::progenitor_support::encode_path(&reader.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/terminal/readers/{reader}` endpoint.
    *
    * <p>Deletes a <code>Reader</code> object.</p>
    *
    * **Parameters:**
    *
    * * `reader: &str` -- The account's country.
    */
    pub async fn delete_terminal_readers_reader(
        &self,
        reader: &str,
    ) -> Result<crate::types::DeletedTerminalReader> {
        let url = format!(
            "/v1/terminal/readers/{}",
            crate::progenitor_support::encode_path(&reader.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/terminal/readers/{reader}/cancel_action` endpoint.
    *
    * <p>Cancels the current reader action.</p>
    *
    * **Parameters:**
    *
    * * `reader: &str` -- The account's country.
    */
    pub async fn post_terminal_readers_reader_cancel_action(
        &self,
        reader: &str,
    ) -> Result<crate::types::TerminalReader> {
        let url = format!(
            "/v1/terminal/readers/{}/cancel_action",
            crate::progenitor_support::encode_path(&reader.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/terminal/readers/{reader}/process_payment_intent` endpoint.
    *
    * <p>Initiates a payment flow on a Reader.</p>
    *
    * **Parameters:**
    *
    * * `reader: &str` -- The account's country.
    */
    pub async fn post_terminal_readers_reader_process_payment_intent(
        &self,
        reader: &str,
    ) -> Result<crate::types::TerminalReader> {
        let url = format!(
            "/v1/terminal/readers/{}/process_payment_intent",
            crate::progenitor_support::encode_path(&reader.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/terminal/readers/{reader}/process_setup_intent` endpoint.
    *
    * <p>Initiates a setup intent flow on a Reader.</p>
    *
    * **Parameters:**
    *
    * * `reader: &str` -- The account's country.
    */
    pub async fn post_terminal_readers_reader_process_setup_intent(
        &self,
        reader: &str,
    ) -> Result<crate::types::TerminalReader> {
        let url = format!(
            "/v1/terminal/readers/{}/process_setup_intent",
            crate::progenitor_support::encode_path(&reader.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/terminal/readers/{reader}/set_reader_display` endpoint.
    *
    * <p>Sets reader display to show cart details.</p>
    *
    * **Parameters:**
    *
    * * `reader: &str` -- The account's country.
    */
    pub async fn post_terminal_readers_reader_set_display(
        &self,
        reader: &str,
    ) -> Result<crate::types::TerminalReader> {
        let url = format!(
            "/v1/terminal/readers/{}/set_reader_display",
            crate::progenitor_support::encode_path(&reader.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/test_helpers/terminal/readers/{reader}/present_payment_method` endpoint.
    *
    * <p>Presents a payment method on a simulated reader. Can be used to simulate accepting a payment, saving a card or refunding a transaction.</p>
    *
    * **Parameters:**
    *
    * * `reader: &str` -- The account's country.
    */
    pub async fn post_test_helpers_terminal_readers_reader_present_payment_method(
        &self,
        reader: &str,
    ) -> Result<crate::types::TerminalReader> {
        let url = format!(
            "/v1/test_helpers/terminal/readers/{}/present_payment_method",
            crate::progenitor_support::encode_path(&reader.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/test_helpers/test_clocks` endpoint.
    *
    * <p>Returns a list of your test clocks.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_test_helpers_clocks(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetTestHelpersClocksResponse> {
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
        let url = format!("/v1/test_helpers/test_clocks?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/test_helpers/test_clocks` endpoint.
    *
    * <p>Creates a new test clock that can be attached to new customers and quotes.</p>
    */
    pub async fn post_test_helpers_clock(&self) -> Result<crate::types::TestClock> {
        let url = "/v1/test_helpers/test_clocks".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/test_helpers/test_clocks/{test_clock}` endpoint.
    *
    * <p>Retrieves a test clock.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `test_clock: &str` -- The account's country.
    */
    pub async fn get_test_helpers_clocks_clock(
        &self,
        expand: &[String],
        test_clock: &str,
    ) -> Result<crate::types::TestClock> {
        let url = format!(
            "/v1/test_helpers/test_clocks/{}",
            crate::progenitor_support::encode_path(&test_clock.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/test_helpers/test_clocks/{test_clock}` endpoint.
    *
    * <p>Deletes a test clock.</p>
    *
    * **Parameters:**
    *
    * * `test_clock: &str` -- The account's country.
    */
    pub async fn delete_test_helpers_clocks_clock(
        &self,
        test_clock: &str,
    ) -> Result<crate::types::DeletedTestClock> {
        let url = format!(
            "/v1/test_helpers/test_clocks/{}",
            crate::progenitor_support::encode_path(&test_clock.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/test_helpers/test_clocks/{test_clock}/advance` endpoint.
    *
    * <p>Starts advancing a test clock to a specified time in the future. Advancement is done when status changes to <code>Ready</code>.</p>
    *
    * **Parameters:**
    *
    * * `test_clock: &str` -- The account's country.
    */
    pub async fn post_test_helpers_clocks_clock_advance(
        &self,
        test_clock: &str,
    ) -> Result<crate::types::TestClock> {
        let url = format!(
            "/v1/test_helpers/test_clocks/{}/advance",
            crate::progenitor_support::encode_path(&test_clock.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/tokens` endpoint.
    *
    * <p>Creates a single-use token that represents a bank account’s details.
    * This token can be used with any API method in place of a bank account dictionary. This token can be used only once, by attaching it to a <a href="#accounts">Custom account</a>.</p>
    */
    pub async fn post_token(&self) -> Result<crate::types::Token> {
        let url = "/v1/tokens".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/tokens/{token}` endpoint.
    *
    * <p>Retrieves the token with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `token: &str` -- The account's country.
    */
    pub async fn get_tokens_token(
        &self,
        expand: &[String],
        token: &str,
    ) -> Result<crate::types::Token> {
        let url = format!(
            "/v1/tokens/{}",
            crate::progenitor_support::encode_path(&token.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/topups` endpoint.
    *
    * <p>Returns a list of top-ups.</p>
    *
    * **Parameters:**
    *
    * * `amount: &str` -- A positive integer representing how much to transfer.
    * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `status: crate::types::GetTopupsStatus` -- Only return top-ups that have the given status. One of `canceled`, `failed`, `pending` or `succeeded`.
    */
    pub async fn get_topup(
        &self,
        amount: &str,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        status: crate::types::GetTopupsStatus,
    ) -> Result<crate::types::TopupList> {
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
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/topups?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/topups` endpoint.
    *
    * <p>Top up the balance of an account</p>
    */
    pub async fn post_topup(&self) -> Result<crate::types::Topup> {
        let url = "/v1/topups".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/topups/{topup}` endpoint.
    *
    * <p>Retrieves the details of a top-up that has previously been created. Supply the unique top-up ID that was returned from your previous request, and Stripe will return the corresponding top-up information.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `topup: &str` -- The account's country.
    */
    pub async fn get_topups_topup(
        &self,
        expand: &[String],
        topup: &str,
    ) -> Result<crate::types::Topup> {
        let url = format!(
            "/v1/topups/{}",
            crate::progenitor_support::encode_path(&topup.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/topups/{topup}` endpoint.
    *
    * <p>Updates the metadata of a top-up. Other top-up details are not editable by design.</p>
    *
    * **Parameters:**
    *
    * * `topup: &str` -- The account's country.
    */
    pub async fn post_topups_topup(&self, topup: &str) -> Result<crate::types::Topup> {
        let url = format!(
            "/v1/topups/{}",
            crate::progenitor_support::encode_path(&topup.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/topups/{topup}/cancel` endpoint.
    *
    * <p>Cancels a top-up. Only pending top-ups can be canceled.</p>
    *
    * **Parameters:**
    *
    * * `topup: &str` -- The account's country.
    */
    pub async fn post_topups_topup_cancel(&self, topup: &str) -> Result<crate::types::Topup> {
        let url = format!(
            "/v1/topups/{}/cancel",
            crate::progenitor_support::encode_path(&topup.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/transfers` endpoint.
    *
    * <p>Returns a list of existing transfers sent to connected accounts. The transfers are returned in sorted order, with the most recently created transfers appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `destination: &str` -- Only return transfers for the destination specified by this account ID.
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    * * `transfer_group: &str` -- Only return transfers with the specified transfer group.
    */
    pub async fn get_transfer(
        &self,
        created: &str,
        destination: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
        transfer_group: &str,
    ) -> Result<crate::types::TransferList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !destination.is_empty() {
            query_args.push(("destination".to_string(), destination.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !transfer_group.is_empty() {
            query_args.push(("transfer_group".to_string(), transfer_group.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/transfers?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/transfers` endpoint.
    *
    * <p>To send funds from your Stripe account to a connected account, you create a new transfer object. Your <a href="#balance">Stripe balance</a> must be able to cover the transfer amount, or you’ll receive an “Insufficient Funds” error.</p>
    */
    pub async fn post_transfer(&self) -> Result<crate::types::Transfer> {
        let url = "/v1/transfers".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/transfers/{id}/reversals` endpoint.
    *
    * <p>You can see a list of the reversals belonging to a specific transfer. Note that the 10 most recent reversals are always available by default on the transfer object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional reversals.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_transfers_reversal(
        &self,
        ending_before: &str,
        expand: &[String],
        id: &str,
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::Reversals> {
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
        let url = format!(
            "/v1/transfers/{}/reversals?{}",
            crate::progenitor_support::encode_path(&id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/transfers/{id}/reversals` endpoint.
    *
    * <p>When you create a new reversal, you must specify a transfer to create it on.</p>
    *
    * <p>When reversing transfers, you can optionally reverse part of the transfer. You can do so as many times as you wish until the entire transfer has been reversed.</p>
    *
    * <p>Once entirely reversed, a transfer can’t be reversed again. This method will return an error when called on an already-reversed transfer, or when trying to reverse more money than is left on a transfer.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    */
    pub async fn post_transfers_reversal(
        &self,
        id: &str,
    ) -> Result<crate::types::TransferReversal> {
        let url = format!(
            "/v1/transfers/{}/reversals",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/transfers/{transfer}` endpoint.
    *
    * <p>Retrieves the details of an existing transfer. Supply the unique transfer ID from either a transfer creation request or the transfer list, and Stripe will return the corresponding transfer information.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `transfer: &str` -- The account's country.
    */
    pub async fn get_transfers_transfer(
        &self,
        expand: &[String],
        transfer: &str,
    ) -> Result<crate::types::Transfer> {
        let url = format!(
            "/v1/transfers/{}",
            crate::progenitor_support::encode_path(&transfer.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/transfers/{transfer}` endpoint.
    *
    * <p>Updates the specified transfer by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * <p>This request accepts only metadata as an argument.</p>
    *
    * **Parameters:**
    *
    * * `transfer: &str` -- The account's country.
    */
    pub async fn post_transfers_transfer(&self, transfer: &str) -> Result<crate::types::Transfer> {
        let url = format!(
            "/v1/transfers/{}",
            crate::progenitor_support::encode_path(&transfer.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/transfers/{transfer}/reversals/{id}` endpoint.
    *
    * <p>By default, you can see the 10 most recent reversals stored directly on the transfer object, but you can also retrieve details about a specific reversal stored on the transfer.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `id: &str` -- The account's country.
    * * `transfer: &str` -- The account's country.
    */
    pub async fn get_transfers_transfer_reversal(
        &self,
        expand: &[String],
        id: &str,
        transfer: &str,
    ) -> Result<crate::types::TransferReversal> {
        let url = format!(
            "/v1/transfers/{}/reversals/{}",
            crate::progenitor_support::encode_path(&transfer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/transfers/{transfer}/reversals/{id}` endpoint.
    *
    * <p>Updates the specified reversal by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
    *
    * <p>This request only accepts metadata and description as arguments.</p>
    *
    * **Parameters:**
    *
    * * `id: &str` -- The account's country.
    * * `transfer: &str` -- The account's country.
    */
    pub async fn post_transfers_transfer_reversal(
        &self,
        id: &str,
        transfer: &str,
    ) -> Result<crate::types::TransferReversal> {
        let url = format!(
            "/v1/transfers/{}/reversals/{}",
            crate::progenitor_support::encode_path(&transfer.to_string()),
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/webhook_endpoints` endpoint.
    *
    * <p>Returns a list of your webhook endpoints.</p>
    *
    * **Parameters:**
    *
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_webhook_endpoints(
        &self,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<crate::types::GetWebhookEndpointsResponse> {
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
        let url = format!("/v1/webhook_endpoints?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/webhook_endpoints` endpoint.
    *
    * <p>A webhook endpoint must have a <code>url</code> and a list of <code>enabled_events</code>. You may optionally specify the Boolean <code>connect</code> parameter. If set to true, then a Connect webhook endpoint that notifies the specified <code>url</code> about events from all connected accounts is created; otherwise an account webhook endpoint that notifies the specified <code>url</code> only about events from your account is created. You can also create webhook endpoints in the <a href="https://dashboard.stripe.com/account/webhooks">webhooks settings</a> section of the Dashboard.</p>
    */
    pub async fn post_webhook_endpoint(&self) -> Result<crate::types::WebhookEndpoint> {
        let url = "/v1/webhook_endpoints".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/webhook_endpoints/{webhook_endpoint}` endpoint.
    *
    * <p>Retrieves the webhook endpoint with the given ID.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `webhook_endpoint: &str` -- The account's country.
    */
    pub async fn get_webhook_endpoints_endpoint(
        &self,
        expand: &[String],
        webhook_endpoint: &str,
    ) -> Result<crate::types::WebhookEndpoint> {
        let url = format!(
            "/v1/webhook_endpoints/{}",
            crate::progenitor_support::encode_path(&webhook_endpoint.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/v1/webhook_endpoints/{webhook_endpoint}` endpoint.
    *
    * <p>Updates the webhook endpoint. You may edit the <code>url</code>, the list of <code>enabled_events</code>, and the status of your endpoint.</p>
    *
    * **Parameters:**
    *
    * * `webhook_endpoint: &str` -- The account's country.
    */
    pub async fn post_webhook_endpoints_endpoint(
        &self,
        webhook_endpoint: &str,
    ) -> Result<crate::types::WebhookEndpoint> {
        let url = format!(
            "/v1/webhook_endpoints/{}",
            crate::progenitor_support::encode_path(&webhook_endpoint.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/v1/webhook_endpoints/{webhook_endpoint}` endpoint.
    *
    * <p>You can also delete webhook endpoints via the <a href="https://dashboard.stripe.com/account/webhooks">webhook endpoint management</a> page of the Stripe dashboard.</p>
    *
    * **Parameters:**
    *
    * * `webhook_endpoint: &str` -- The account's country.
    */
    pub async fn delete_webhook_endpoints_endpoint(
        &self,
        webhook_endpoint: &str,
    ) -> Result<crate::types::DeletedWebhookEndpoint> {
        let url = format!(
            "/v1/webhook_endpoints/{}",
            crate::progenitor_support::encode_path(&webhook_endpoint.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

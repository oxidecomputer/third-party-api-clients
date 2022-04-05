use anyhow::Result;

use crate::Client;

pub struct ApplePay {
    pub client: Client,
}

impl ApplePay {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ApplePay { client }
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
    pub async fn get_domains(
        &self,
        domain_name: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        starting_after: &str,
    ) -> Result<Vec<crate::types::ApplePayDomain>> {
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

        let resp: crate::types::ApplePayDomainList = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data)
    }

    /**
    * This function performs a `GET` to the `/v1/apple_pay/domains` endpoint.
    *
    * As opposed to `get_domains`, this function returns all the pages of the request at once.
    *
    * <p>List apple pay domains.</p>
    */
    pub async fn get_all_domains(
        &self,
        domain_name: &str,
        expand: &[String],
    ) -> Result<Vec<crate::types::ApplePayDomain>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !domain_name.is_empty() {
            query_args.push(("domain_name".to_string(), domain_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/apple_pay/domains?{}", query_);

        let mut resp: crate::types::ApplePayDomainList = self.client.get(&url, None).await?;

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
    * This function performs a `POST` to the `/v1/apple_pay/domains` endpoint.
    *
    * <p>Create an apple pay domain.</p>
    */
    pub async fn post_domain(&self) -> Result<crate::types::ApplePayDomain> {
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
    pub async fn get_domains_domain(
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
    pub async fn delete_domains_domain(
        &self,
        domain: &str,
    ) -> Result<crate::types::DeletedApplePayDomain> {
        let url = format!(
            "/v1/apple_pay/domains/{}",
            crate::progenitor_support::encode_path(&domain.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

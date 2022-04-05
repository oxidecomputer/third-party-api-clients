use anyhow::Result;

use crate::Client;

pub struct DomainAliases {
    pub client: Client,
}

impl DomainAliases {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        DomainAliases { client }
    }

    /**
    * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/domainaliases` endpoint.
    *
    * Lists the domain aliases of the customer.
    *
    * **Parameters:**
    *
    * * `customer: &str` -- Immutable ID of the Google Workspace account.
    * * `parent_domain_name: &str` -- Name of the parent domain for which domain aliases are to be fetched.
    */
    pub async fn list(
        &self,
        customer: &str,
        parent_domain_name: &str,
    ) -> Result<crate::types::DomainAliases> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !parent_domain_name.is_empty() {
            query_args.push((
                "parentDomainName".to_string(),
                parent_domain_name.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/admin/directory/v1/customer/{}/domainaliases?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/domainaliases` endpoint.
    *
    * Inserts a domain alias of the customer.
    *
    * **Parameters:**
    *
    * * `customer: &str` -- Immutable ID of the Google Workspace account.
    */
    pub async fn insert(
        &self,
        customer: &str,
        body: &crate::types::DomainAlias,
    ) -> Result<crate::types::DomainAlias> {
        let url = format!(
            "/admin/directory/v1/customer/{}/domainaliases",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/domainaliases/{domainAliasName}` endpoint.
    *
    * Retrieves a domain alias of the customer.
    *
    * **Parameters:**
    *
    * * `customer: &str` -- Immutable ID of the Google Workspace account.
    * * `domain_alias_name: &str` -- Name of domain alias to be retrieved.
    */
    pub async fn get(
        &self,
        customer: &str,
        domain_alias_name: &str,
    ) -> Result<crate::types::DomainAlias> {
        let url = format!(
            "/admin/directory/v1/customer/{}/domainaliases/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&domain_alias_name.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/domainaliases/{domainAliasName}` endpoint.
    *
    * Deletes a domain Alias of the customer.
    *
    * **Parameters:**
    *
    * * `customer: &str` -- Immutable ID of the Google Workspace account.
    * * `domain_alias_name: &str` -- Name of domain alias to be retrieved.
    */
    pub async fn delete(&self, customer: &str, domain_alias_name: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/customer/{}/domainaliases/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&domain_alias_name.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

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
    pub async fn directory_list(
        &self,
        customer: &str,
        parent_domain_name: &str,
    ) -> Result<crate::types::DomainAliases> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !parent_domain_name.is_empty() {
            query_args.push(format!("parent_domain_name={}", parent_domain_name));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
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
    pub async fn directory_insert(
        &self,
        customer: &str,
        body: &crate::types::DomainAlias,
    ) -> Result<crate::types::DomainAlias> {
        let url = format!(
            "/admin/directory/v1/customer/{}/domainaliases",
            crate::progenitor_support::encode_path(&customer.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
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
    pub async fn directory_get(
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
    pub async fn directory_delete(&self, customer: &str, domain_alias_name: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/customer/{}/domainaliases/{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&domain_alias_name.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

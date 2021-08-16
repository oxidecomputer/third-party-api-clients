use anyhow::Result;

use crate::Client;

pub struct Domains {
    client: Client,
}

impl Domains {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Domains { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/domains` endpoint.
     *
     * Lists the domains of the customer.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn directory_list(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
    ) -> Result<crate::types::Domains2> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/domains?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/domains` endpoint.
     *
     * Inserts a domain of the customer.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn directory_insert(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        body: &crate::types::Domains,
    ) -> Result<crate::types::Domains> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/domains?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            query_
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/domains/{domainName}` endpoint.
     *
     * Retrieves a domain of the customer.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `domain_name: &str` -- Name of domain to be retrieved.
     */
    pub async fn directory_get(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        domain_name: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/domains/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&domain_name.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/domains/{domainName}` endpoint.
     *
     * Deletes a domain of the customer.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `domain_name: &str` -- Name of domain to be deleted.
     */
    pub async fn directory_delete(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer: &str,
        domain_name: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/domains/{}?{}",
            crate::progenitor_support::encode_path(&customer.to_string()),
            crate::progenitor_support::encode_path(&domain_name.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }
}

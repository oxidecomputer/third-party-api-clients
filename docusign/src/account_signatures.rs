use crate::Client;
use crate::ClientResult;

pub struct AccountSignatures {
    pub client: Client,
}

impl AccountSignatures {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountSignatures { client }
    }

    /**
     * Returns the managed signature definitions for the account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/signatures` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `stamp_format: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `stamp_name: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `stamp_type: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(
        &self,
        account_id: &str,
        stamp_format: &str,
        stamp_name: &str,
        stamp_type: &str,
    ) -> ClientResult<crate::types::AccountSignaturesInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !stamp_format.is_empty() {
            query_args.push(("stamp_format".to_string(), stamp_format.to_string()));
        }
        if !stamp_name.is_empty() {
            query_args.push(("stamp_name".to_string(), stamp_name.to_string()));
        }
        if !stamp_type.is_empty() {
            query_args.push(("stamp_type".to_string(), stamp_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatures?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Updates an account signature.
    .
    *
    * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/signatures` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn put_signature(
        &self,
        account_id: &str,
        body: &crate::types::AccountSignaturesInformation,
    ) -> ClientResult<crate::types::AccountSignaturesInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatures",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Adds or updates one or more account signatures.
    This request may include images in multi-part format.
    *
    * This function performs a `POST` to the `/v2.1/accounts/{accountId}/signatures` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `decode_only: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn post(
        &self,
        account_id: &str,
        decode_only: &str,
        body: &crate::types::AccountSignaturesInformation,
    ) -> ClientResult<crate::types::AccountSignaturesInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !decode_only.is_empty() {
            query_args.push(("decode_only".to_string(), decode_only.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatures?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Returns information about the specified signature.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/signatures/{signatureId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get_signature(
        &self,
        account_id: &str,
        signature_id: &str,
    ) -> ClientResult<crate::types::AccountSignature> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatures/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(signature_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates an account signature.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/signatures/{signatureId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `close_existing_signature: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put_signature_account_signatures(
        &self,
        account_id: &str,
        signature_id: &str,
        close_existing_signature: &str,
        body: &crate::types::AccountSignatureDefinition,
    ) -> ClientResult<crate::types::AccountSignature> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !close_existing_signature.is_empty() {
            query_args.push((
                "close_existing_signature".to_string(),
                close_existing_signature.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatures/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(signature_id),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Close the specified signature by ID.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/signatures/{signatureId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete_signature(&self, account_id: &str, signature_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatures/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(signature_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Returns a signature image, initials, or stamp.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/signatures/{signatureId}/{imageType}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `image_type: &str` -- Specificies the type of image. Valid values are:
     *   
     *   - `signature_image`
     *   - `initials_image`.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_chrome: &str` -- When **true**, the chrome (or frame containing the added line and identifier) is included with the signature image.
     */
    pub async fn get_signature_image(
        &self,
        account_id: &str,
        image_type: &str,
        signature_id: &str,
        include_chrome: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_chrome.is_empty() {
            query_args.push(("include_chrome".to_string(), include_chrome.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatures/{}/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(signature_id),
                crate::progenitor_support::encode_path(image_type),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Sets a signature image, initials, or stamp.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/signatures/{signatureId}/{imageType}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `image_type: &str` -- Specificies the type of image. Valid values are:
     *   
     *   - `signature_image`
     *   - `initials_image`.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `transparent_png: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put_signature_image(
        &self,
        account_id: &str,
        image_type: &str,
        signature_id: &str,
        transparent_png: &str,
    ) -> ClientResult<crate::types::AccountSignature> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !transparent_png.is_empty() {
            query_args.push(("transparent_png".to_string(), transparent_png.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatures/{}/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(signature_id),
                crate::progenitor_support::encode_path(image_type),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes a signature image, initials, or stamp.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/signatures/{signatureId}/{imageType}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `image_type: &str` -- Specificies the type of image. Valid values are:
     *   
     *   - `signature_image`
     *   - `initials_image`.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete_signature_image(
        &self,
        account_id: &str,
        image_type: &str,
        signature_id: &str,
    ) -> ClientResult<crate::types::AccountSignature> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatures/{}/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(signature_id),
                crate::progenitor_support::encode_path(image_type),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}

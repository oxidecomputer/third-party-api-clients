use crate::Client;
use crate::ClientResult;

pub struct AccountBrands {
    pub client: Client,
}

impl AccountBrands {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountBrands { client }
    }

    /**
     * Gets a list of brands.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/brands` endpoint.
     *
     * This method returns details about all of the brands associated with an account, including the default brand profiles.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `exclude_distributor_brand: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_logos: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brands_get(
        &self,
        account_id: &str,
        exclude_distributor_brand: &str,
        include_logos: &str,
    ) -> ClientResult<crate::types::AccountBrands> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_distributor_brand.is_empty() {
            query_args.push((
                "exclude_distributor_brand".to_string(),
                exclude_distributor_brand.to_string(),
            ));
        }
        if !include_logos.is_empty() {
            query_args.push(("include_logos".to_string(), include_logos.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands?{}",
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
     * Creates one or more brand profiles for an account.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/brands` endpoint.
     *
     * This method creates one or more brand profile files for an account.
     *
     * If the `brandId` property for a brand profile is already set for the account, an error is returned. To upload a new version of an existing brand profile, you must delete the profile and then upload the newer version.
     *
     * When you upload brand profile files, you must combine them into a single zip file and set the `Content-Type` to `application/zip`.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brands_post(
        &self,
        account_id: &str,
        body: &crate::types::Brand,
    ) -> ClientResult<crate::types::AccountBrands> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands",
                crate::progenitor_support::encode_path(account_id),
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
     * Deletes one or more brand profiles.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/brands` endpoint.
     *
     * This method deletes one or more brand profiles from an account, based on the brand ids that you include in the `brandsRequest`.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brands_delete(
        &self,
        account_id: &str,
        body: &crate::types::BrandsRequest,
    ) -> ClientResult<crate::types::AccountBrands> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Gets information about a brand.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/brands/{brandId}` endpoint.
     *
     * This method returns details about an account brand.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_external_references: &str` -- When **true**, the landing pages and links associated with the brand are included in the response.
     * * `include_logos: &str` -- When **true**, the URIs for the logos associated with the brand are included in the response.
     */
    pub async fn brand_get(
        &self,
        account_id: &str,
        brand_id: &str,
        include_external_references: &str,
        include_logos: &str,
    ) -> ClientResult<crate::types::Brand> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_external_references.is_empty() {
            query_args.push((
                "include_external_references".to_string(),
                include_external_references.to_string(),
            ));
        }
        if !include_logos.is_empty() {
            query_args.push(("include_logos".to_string(), include_logos.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
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
     * Updates an existing brand.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/brands/{brandId}` endpoint.
     *
     * This method updates an account brand.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brand_put(
        &self,
        account_id: &str,
        brand_id: &str,
        body: &crate::types::Brand,
    ) -> ClientResult<crate::types::Brand> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
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
     * Deletes a brand.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/brands/{brandId}` endpoint.
     *
     * This method deletes a brand from an account.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brand_delete(&self, account_id: &str, brand_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
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
     * Exports a brand.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/brands/{brandId}/file` endpoint.
     *
     * This method exports information about a brand to an XML file.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brand_export_get_file(
        &self,
        account_id: &str,
        brand_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}/file",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
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
     * Gets a brand logo.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/brands/{brandId}/logos/{logoType}` endpoint.
     *
     * This method returns a specific logo that is used in a brand.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `logo_type: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brand_logo_get(
        &self,
        account_id: &str,
        brand_id: &str,
        logo_type: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}/logos/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
                crate::progenitor_support::encode_path(logo_type),
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
     * Updates a brand logo.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/brands/{brandId}/logos/{logoType}` endpoint.
     *
     * This method updates a single brand logo.
     *
     * You pass in the new version of the resource in the `Content-Disposition` header. Example:
     *
     * `Content-Disposition: form-data; name="file"; filename="logo.jpg"`
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `logo_type: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brand_logo_put<T: Into<reqwest::Body>>(
        &self,
        account_id: &str,
        brand_id: &str,
        logo_type: &str,
        body: bytes::Bytes,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}/logos/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
                crate::progenitor_support::encode_path(logo_type),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(body.into()),
                    content_type: Some("image/png".to_string()),
                },
            )
            .await
    }
    /**
     * Deletes a brand logo.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/brands/{brandId}/logos/{logoType}` endpoint.
     *
     * This method deletes a single logo from an account brand.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `logo_type: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brand_logo_delete(
        &self,
        account_id: &str,
        brand_id: &str,
        logo_type: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}/logos/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
                crate::progenitor_support::encode_path(logo_type),
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
     * Returns metadata about the branding resources for an account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/brands/{brandId}/resources` endpoint.
     *
     * This method returns metadata about the branding resources that are associated with an account.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn brand_resources_get_list(
        &self,
        account_id: &str,
        brand_id: &str,
    ) -> ClientResult<crate::types::BrandResourcesList> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}/resources",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
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
     * Returns a branding resource file.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/brands/{brandId}/resources/{resourceContentType}` endpoint.
     *
     * This method returns a specific branding resource file.
     *
     * A brand uses a set of brand resource files to control the sending, signing, email message, and captive (embedded) signing experiences.  You can modify the default email messages and formats in these files and upload them to your brand to customize the user experience.
     *
     * **Important**: When you upload a modified resource file, only the elements that differ from the master resource file are saved as your resource file. Similarly, when you download your resource files, only the modified elements are included in the file.
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `resource_content_type: &str` -- The type of brand resource file to return. Valid values are:
     *   
     *   - `sending`
     *   - `signing`
     *   - `email`
     *   - `signing_captive`.
     * * `langcode: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `return_master: &str` -- Specifies which resource file data to return. When **true**, only the master resource file is returned. When **false**, only the elements that you modified are returned.
     */
    pub async fn brand_resources_get(
        &self,
        account_id: &str,
        brand_id: &str,
        resource_content_type: &str,
        langcode: &str,
        return_master: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !langcode.is_empty() {
            query_args.push(("langcode".to_string(), langcode.to_string()));
        }
        if !return_master.is_empty() {
            query_args.push(("return_master".to_string(), return_master.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}/resources/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
                crate::progenitor_support::encode_path(resource_content_type),
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
     * Updates a branding resource file.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/brands/{brandId}/resources/{resourceContentType}` endpoint.
     *
     * This method updates a branding resource file.
     *
     * You pass in the new version of the resource file in the `Content-Disposition` header. Example:
     *
     * `Content-Disposition: form-data; name="file"; filename="DocuSign_SigningResource_4328673.xml"`
     *
     * **Note**: Branding for either signing or sending must be enabled for the account (`canSelfBrandSend` , `canSelfBrandSign`, or both of these account settings must be **true**).
     *
     * **Important**: Customizing resource files is an advanced branding configuration option which can significantly impact your account, and should be done only by someone with expertise in XML and HTML. The master resource files are subject to change without notice. If you customize your resource files, after each release, DocuSign recommends you review any changes and update your custom files as needed.
     *
     * When you upload a modified resource file, only the elements that differ from the master resource file are saved as your resource file. Similarly, when you download your resource files, only the modified elements are included in the file.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `brand_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `resource_content_type: &str` -- The type of brand resource file that you are updating. Valid values are:
     *   
     *   - `sending`
     *   - `signing`
     *   - `email`
     *   - `signing_captive`.
     */
    pub async fn brand_resources_put(
        &self,
        account_id: &str,
        brand_id: &str,
        resource_content_type: &str,
    ) -> ClientResult<crate::types::BrandResources> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/brands/{}/resources/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(brand_id),
                crate::progenitor_support::encode_path(resource_content_type),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("multipart/form-data".to_string()),
                },
            )
            .await
    }
}

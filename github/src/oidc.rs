use crate::Client;
use crate::ClientResult;

pub struct Oidc {
    pub client: Client,
}

impl Oidc {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Oidc { client }
    }

    /**
     * List OIDC custom property inclusions for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/oidc/customization/properties/repo` endpoint.
     *
     * Lists the repository custom properties that are included in the OIDC token for repository actions in an enterprise.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#list-oidc-custom-property-inclusions-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     */
    pub async fn list_oidc_custom_property_inclusions_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OidcCustomPropertyInclusion>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/oidc/customization/properties/repo",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * List OIDC custom property inclusions for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/oidc/customization/properties/repo` endpoint.
     *
     * As opposed to `list_oidc_custom_property_inclusions_for_enterprise`, this function returns all the pages of the request at once.
     *
     * Lists the repository custom properties that are included in the OIDC token for repository actions in an enterprise.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#list-oidc-custom-property-inclusions-for-an-enterprise>
     */
    pub async fn list_all_oidc_custom_property_inclusions_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OidcCustomPropertyInclusion>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/oidc/customization/properties/repo",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create an OIDC custom property inclusion for an enterprise.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/actions/oidc/customization/properties/repo` endpoint.
     *
     * Adds a repository custom property to be included in the OIDC token for repository actions in an enterprise.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#create-an-oidc-custom-property-inclusion-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     */
    pub async fn create_oidc_custom_property_inclusion_for_enterprise(
        &self,
        enterprise: &str,
        body: &crate::types::OidcCustomPropertyInclusionInput,
    ) -> ClientResult<crate::Response<crate::types::OidcCustomPropertyInclusion>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/oidc/customization/properties/repo",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Delete an OIDC custom property inclusion for an enterprise.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/actions/oidc/customization/properties/repo/{custom_property_name}` endpoint.
     *
     * Removes a repository custom property from being included in the OIDC token for repository actions in an enterprise.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#delete-an-oidc-custom-property-inclusion-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `custom_property_name: &str` -- The name of the custom property to remove from OIDC token inclusion.
     */
    pub async fn delete_oidc_custom_property_inclusion_for_enterprise(
        &self,
        enterprise: &str,
        custom_property_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/oidc/customization/properties/repo/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&custom_property_name.to_string()),
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
     * List OIDC custom property inclusions for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/oidc/customization/properties/repo` endpoint.
     *
     * Lists the repository custom properties that are included in the OIDC token for repository actions in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#list-oidc-custom-property-inclusions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_oidc_custom_property_inclusions_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OidcCustomPropertyInclusion>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/oidc/customization/properties/repo",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List OIDC custom property inclusions for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/oidc/customization/properties/repo` endpoint.
     *
     * As opposed to `list_oidc_custom_property_inclusions_for_org`, this function returns all the pages of the request at once.
     *
     * Lists the repository custom properties that are included in the OIDC token for repository actions in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#list-oidc-custom-property-inclusions-for-an-organization>
     */
    pub async fn list_all_oidc_custom_property_inclusions_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OidcCustomPropertyInclusion>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/oidc/customization/properties/repo",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create an OIDC custom property inclusion for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/oidc/customization/properties/repo` endpoint.
     *
     * Adds a repository custom property to be included in the OIDC token for repository actions in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#create-an-oidc-custom-property-inclusion-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_oidc_custom_property_inclusion_for_org(
        &self,
        org: &str,
        body: &crate::types::OidcCustomPropertyInclusionInput,
    ) -> ClientResult<crate::Response<crate::types::OidcCustomPropertyInclusion>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/oidc/customization/properties/repo",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Delete an OIDC custom property inclusion for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/oidc/customization/properties/repo/{custom_property_name}` endpoint.
     *
     * Removes a repository custom property from being included in the OIDC token for repository actions in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#delete-an-oidc-custom-property-inclusion-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `custom_property_name: &str` -- The name of the custom property to remove from OIDC token inclusion.
     */
    pub async fn delete_oidc_custom_property_inclusion_for_org(
        &self,
        org: &str,
        custom_property_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/oidc/customization/properties/repo/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&custom_property_name.to_string()),
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
     * Get the customization template for an OIDC subject claim for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/oidc/customization/sub` endpoint.
     *
     * Gets the customization template for an OpenID Connect (OIDC) subject claim.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#get-the-customization-template-for-an-oidc-subject-claim-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_oidc_custom_sub_template_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::OidcCustomSub>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/oidc/customization/sub",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Set the customization template for an OIDC subject claim for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/oidc/customization/sub` endpoint.
     *
     * Creates or updates the customization template for an OpenID Connect (OIDC) subject claim.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#set-the-customization-template-for-an-oidc-subject-claim-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn update_oidc_custom_sub_template_for_org(
        &self,
        org: &str,
        body: &crate::types::OidcCustomSub,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/oidc/customization/sub",
                crate::progenitor_support::encode_path(&org.to_string()),
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
}

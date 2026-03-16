use crate::Client;
use crate::ClientResult;

pub struct Orgs {
    pub client: Client,
}

impl Orgs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Orgs { client }
    }

    /**
     * List organizations.
     *
     * This function performs a `GET` to the `/organizations` endpoint.
     *
     * Lists all organizations, in the order that they were created.
     *
     * > [!NOTE]
     * > Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of organizations.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#list-organizations>
     *
     * **Parameters:**
     *
     * * `since: i64` -- An organization ID. Only return organizations with an ID greater than this ID.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list(
        &self,
        since: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/organizations?{}", query_), None);
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
     * List organizations.
     *
     * This function performs a `GET` to the `/organizations` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists all organizations, in the order that they were created.
     *
     * > [!NOTE]
     * > Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of organizations.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#list-organizations>
     */
    pub async fn list_all(
        &self,
        since: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/organizations?{}", query_), None);
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
     * Get an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}` endpoint.
     *
     * Gets information about an organization.
     *
     * When the value of `two_factor_requirement_enabled` is `true`, the organization requires all members, billing managers, outside collaborators, guest collaborators, repository collaborators, or everyone with access to any repository within the organization to enable [two-factor authentication](https://docs.github.com/articles/securing-your-account-with-two-factor-authentication-2fa/).
     *
     * To see the full details about an organization, the authenticated user must be an organization owner.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to see the full details about an organization.
     *
     * To see information about an organization's GitHub plan, GitHub Apps need the `Organization plan` permission.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#get-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::OrganizationFull>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}",
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
     * Delete an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}` endpoint.
     *
     * Deletes an organization and all its repositories.
     *
     * The organization login will be unavailable for 90 days after deletion.
     *
     * Please review the Terms of Service regarding account deletion before using this endpoint:
     *
     * https://docs.github.com/site-policy/github-terms/github-terms-of-service
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#delete-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn delete(&self, org: &str) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Update an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}` endpoint.
     *
     * > [!WARNING]
     * > **Closing down notice:** GitHub will replace and discontinue `members_allowed_repository_creation_type` in favor of more granular permissions. The new input parameters are `members_can_create_public_repositories`, `members_can_create_private_repositories` for all organizations and `members_can_create_internal_repositories` for organizations associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see the [blog post](https://developer.github.com/changes/2019-12-03-internal-visibility-changes).
     *
     * > [!WARNING]
     * > **Closing down notice:** Code security product enablement for new repositories through the organization API is closing down. Please use [code security configurations](https://docs.github.com/rest/code-security/configurations#set-a-code-security-configuration-as-a-default-for-an-organization) to set defaults instead. For more information on setting a default security configuration, see the [changelog](https://github.blog/changelog/2024-07-09-sunsetting-security-settings-defaults-parameters-in-the-organizations-rest-api/).
     *
     * Updates the organization's profile and member privileges.
     *
     * The authenticated user must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` or `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#update-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn update(
        &self,
        org: &str,
        body: &crate::types::OrgsUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::OrganizationFull>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Create an artifact deployment record.
     *
     * This function performs a `POST` to the `/orgs/{org}/artifacts/metadata/deployment-record` endpoint.
     *
     * Create or update deployment records for an artifact associated
     * with an organization.
     * This endpoint allows you to record information about a specific
     * artifact, such as its name, digest, environments, cluster, and
     * deployment.
     * The deployment name has to be uniqe within a cluster (i.e a
     * combination of logical, physical environment and cluster) as it
     * identifies unique deployment.
     * Multiple requests for the same combination of logical, physical
     * environment, cluster and deployment name will only create one
     * record, successive request will update the existing record.
     * This allows for a stable tracking of a deployment where the actual
     * deployed artifact can change over time.
     *
     * FROM: <https://docs.github.com/rest/orgs/artifact-metadata#create-an-artifact-deployment-record>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_artifact_deployment_record(
        &self,
        org: &str,
        body: &crate::types::OrgsCreateArtifactDeploymentRecordRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgsSetClusterDeploymentRecordsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/artifacts/metadata/deployment-record",
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
     * Set cluster deployment records.
     *
     * This function performs a `POST` to the `/orgs/{org}/artifacts/metadata/deployment-record/cluster/{cluster}` endpoint.
     *
     * Set deployment records for a given cluster.
     * If proposed records in the 'deployments' field have identical 'cluster', 'logical_environment',
     * 'physical_environment', and 'deployment_name' values as existing records, the existing records will be updated.
     * If no existing records match, new records will be created.
     *
     * FROM: <https://docs.github.com/rest/orgs/artifact-metadata#set-cluster-deployment-records>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `cluster: &str` -- The cluster name.
     */
    pub async fn set_cluster_deployment_records(
        &self,
        org: &str,
        cluster: &str,
        body: &crate::types::OrgsSetClusterDeploymentRecordsRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgsSetClusterDeploymentRecordsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/artifacts/metadata/deployment-record/cluster/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&cluster.to_string()),
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
     * Create artifact metadata storage record.
     *
     * This function performs a `POST` to the `/orgs/{org}/artifacts/metadata/storage-record` endpoint.
     *
     * Create metadata storage records for artifacts associated with an organization.
     * This endpoint will create a new artifact storage record on behalf of any artifact matching the provided digest and
     * associated with a repository owned by the organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/artifact-metadata#create-artifact-metadata-storage-record>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_artifact_storage_record(
        &self,
        org: &str,
        body: &crate::types::OrgsCreateArtifactStorageRecordRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgsListArtifactStorageRecordsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/artifacts/metadata/storage-record",
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
     * List artifact deployment records.
     *
     * This function performs a `GET` to the `/orgs/{org}/artifacts/{subject_digest}/metadata/deployment-records` endpoint.
     *
     * List deployment records for an artifact metadata associated with an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/artifact-metadata#list-artifact-deployment-records>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `subject_digest: &str` -- The SHA256 digest of the artifact, in the form `sha256:HEX_DIGEST`.
     */
    pub async fn list_artifact_deployment_records(
        &self,
        org: &str,
        subject_digest: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgsSetClusterDeploymentRecordsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/artifacts/{}/metadata/deployment-records",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&subject_digest.to_string()),
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
     * List artifact storage records.
     *
     * This function performs a `GET` to the `/orgs/{org}/artifacts/{subject_digest}/metadata/storage-records` endpoint.
     *
     * List a collection of artifact storage records with a given subject digest that are associated with repositories owned by an organization.
     *
     * The collection of storage records returned by this endpoint is filtered according to the authenticated user's permissions; if the authenticated user cannot read a repository, the attestations associated with that repository will not be included in the response. In addition, when using a fine-grained access token the `content:read` permission is required.
     *
     * FROM: <https://docs.github.com/rest/orgs/artifact-metadata#list-artifact-storage-records>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `subject_digest: &str` -- The parameter should be set to the attestation's subject's SHA256 digest, in the form `sha256:HEX_DIGEST`.
     */
    pub async fn list_artifact_storage_records(
        &self,
        org: &str,
        subject_digest: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgsListArtifactStorageRecordsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/artifacts/{}/metadata/storage-records",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&subject_digest.to_string()),
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
     * List attestations by bulk subject digests.
     *
     * This function performs a `POST` to the `/orgs/{org}/attestations/bulk-list` endpoint.
     *
     * List a collection of artifact attestations associated with any entry in a list of subject digests owned by an organization.
     *
     * The collection of attestations returned by this endpoint is filtered according to the authenticated user's permissions; if the authenticated user cannot read a repository, the attestations associated with that repository will not be included in the response. In addition, when using a fine-grained access token the `attestations:read` permission is required.
     *
     * **Please note:** in order to offer meaningful security benefits, an attestation's signature and timestamps **must** be cryptographically verified, and the identity of the attestation signer **must** be validated. Attestations can be verified using the [GitHub CLI `attestation verify` command](https://cli.github.com/manual/gh_attestation_verify). For more information, see [our guide on how to use artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).
     *
     * FROM: <https://docs.github.com/rest/orgs/attestations#list-attestations-by-bulk-subject-digests>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_attestations_bulk(
        &self,
        per_page: i64,
        before: &str,
        after: &str,
        org: &str,
        body: &crate::types::OrgsListAttestationsBulkRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgsListAttestationsBulkResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/attestations/bulk-list?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                query_
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
     * Delete attestations in bulk.
     *
     * This function performs a `POST` to the `/orgs/{org}/attestations/delete-request` endpoint.
     *
     * Delete artifact attestations in bulk by either subject digests or unique ID.
     *
     * FROM: <https://docs.github.com/rest/orgs/attestations#delete-attestations-in-bulk>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn delete_attestations_bulk(&self, org: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/attestations/delete-request",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Delete attestations by subject digest.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/attestations/digest/{subject_digest}` endpoint.
     *
     * Delete an artifact attestation by subject digest.
     *
     * FROM: <https://docs.github.com/rest/orgs/attestations#delete-attestations-by-subject-digest>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `subject_digest: &str` -- Subject Digest.
     */
    pub async fn delete_attestations_by_subject_digest(
        &self,
        org: &str,
        subject_digest: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/attestations/digest/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&subject_digest.to_string()),
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
     * List attestation repositories.
     *
     * This function performs a `GET` to the `/orgs/{org}/attestations/repositories` endpoint.
     *
     * List repositories owned by the provided organization that have created at least one attested artifact
     * Results will be sorted in ascending order by repository ID
     *
     * FROM: <https://docs.github.com/rest/orgs/attestations#list-attestation-repositories>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `predicate_type: &str` -- Optional filter for fetching attestations with a given predicate type.
     *   This option accepts `provenance`, `sbom`, `release`, or freeform text
     *   for custom predicate types.
     */
    pub async fn list_attestation_repositories(
        &self,
        per_page: i64,
        before: &str,
        after: &str,
        org: &str,
        predicate_type: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgsListAttestationRepositoriesResponse>>>
    {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !predicate_type.is_empty() {
            query_args.push(("predicate_type".to_string(), predicate_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/attestations/repositories?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List attestation repositories.
     *
     * This function performs a `GET` to the `/orgs/{org}/attestations/repositories` endpoint.
     *
     * As opposed to `list_attestation_repositories`, this function returns all the pages of the request at once.
     *
     * List repositories owned by the provided organization that have created at least one attested artifact
     * Results will be sorted in ascending order by repository ID
     *
     * FROM: <https://docs.github.com/rest/orgs/attestations#list-attestation-repositories>
     */
    pub async fn list_all_attestation_repositories(
        &self,
        before: &str,
        after: &str,
        org: &str,
        predicate_type: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgsListAttestationRepositoriesResponse>>>
    {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !predicate_type.is_empty() {
            query_args.push(("predicate_type".to_string(), predicate_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/attestations/repositories?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                query_
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
     * Delete attestations by ID.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/attestations/{attestation_id}` endpoint.
     *
     * Delete an artifact attestation by unique ID that is associated with a repository owned by an org.
     *
     * FROM: <https://docs.github.com/rest/orgs/attestations#delete-attestations-by-id>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `attestation_id: i64` -- Attestation ID.
     */
    pub async fn delete_attestations_by_id(
        &self,
        org: &str,
        attestation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/attestations/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&attestation_id.to_string()),
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
     * List attestations.
     *
     * This function performs a `GET` to the `/orgs/{org}/attestations/{subject_digest}` endpoint.
     *
     * List a collection of artifact attestations with a given subject digest that are associated with repositories owned by an organization.
     *
     * The collection of attestations returned by this endpoint is filtered according to the authenticated user's permissions; if the authenticated user cannot read a repository, the attestations associated with that repository will not be included in the response. In addition, when using a fine-grained access token the `attestations:read` permission is required.
     *
     * **Please note:** in order to offer meaningful security benefits, an attestation's signature and timestamps **must** be cryptographically verified, and the identity of the attestation signer **must** be validated. Attestations can be verified using the [GitHub CLI `attestation verify` command](https://cli.github.com/manual/gh_attestation_verify). For more information, see [our guide on how to use artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).
     *
     * FROM: <https://docs.github.com/rest/orgs/attestations#list-attestations>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `subject_digest: &str` -- The parameter should be set to the attestation's subject's SHA256 digest, in the form `sha256:HEX_DIGEST`.
     * * `predicate_type: &str` -- Optional filter for fetching attestations with a given predicate type.
     *   This option accepts `provenance`, `sbom`, `release`, or freeform text
     *   for custom predicate types.
     */
    pub async fn list_attestations(
        &self,
        per_page: i64,
        before: &str,
        after: &str,
        org: &str,
        subject_digest: &str,
        predicate_type: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgsListAttestationsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !predicate_type.is_empty() {
            query_args.push(("predicate_type".to_string(), predicate_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/attestations/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&subject_digest.to_string()),
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
     * List users blocked by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/blocks` endpoint.
     *
     * List the users blocked by an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/blocking#list-users-blocked-by-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_blocked_users(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List users blocked by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/blocks` endpoint.
     *
     * As opposed to `list_blocked_users`, this function returns all the pages of the request at once.
     *
     * List the users blocked by an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/blocking#list-users-blocked-by-an-organization>
     */
    pub async fn list_all_blocked_users(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks",
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
     * Check if a user is blocked by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/blocks/{username}` endpoint.
     *
     * Returns a 204 if the given user is blocked by the given organization. Returns a 404 if the organization is not blocking the user, or if the user account has been identified as spam by GitHub.
     *
     * FROM: <https://docs.github.com/rest/orgs/blocking#check-if-a-user-is-blocked-by-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn check_blocked_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Block a user from an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/blocks/{username}` endpoint.
     *
     * Blocks the given user on behalf of the specified organization and returns a 204. If the organization cannot block the given user a 422 is returned.
     *
     * FROM: <https://docs.github.com/rest/orgs/blocking#block-a-user-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn block_user(&self, org: &str, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Unblock a user from an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/blocks/{username}` endpoint.
     *
     * Unblocks the given user on behalf of the specified organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/blocking#unblock-a-user-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn unblock_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/blocks/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List failed organization invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/failed_invitations` endpoint.
     *
     * The return hash contains `failed_at` and `failed_reason` fields which represent the time at which the invitation failed and the reason for the failure.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-failed-organization-invitations>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_failed_invitations(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/failed_invitations?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List failed organization invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/failed_invitations` endpoint.
     *
     * As opposed to `list_failed_invitations`, this function returns all the pages of the request at once.
     *
     * The return hash contains `failed_at` and `failed_reason` fields which represent the time at which the invitation failed and the reason for the failure.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-failed-organization-invitations>
     */
    pub async fn list_all_failed_invitations(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/failed_invitations",
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
     * List organization webhooks.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks` endpoint.
     *
     * List webhooks for an organization.
     *
     * The authenticated user must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#list-organization-webhooks>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_webhooks(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgHook>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List organization webhooks.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks` endpoint.
     *
     * As opposed to `list_webhooks`, this function returns all the pages of the request at once.
     *
     * List webhooks for an organization.
     *
     * The authenticated user must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#list-organization-webhooks>
     */
    pub async fn list_all_webhooks(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgHook>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks",
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
     * Create an organization webhook.
     *
     * This function performs a `POST` to the `/orgs/{org}/hooks` endpoint.
     *
     * Create a hook that posts payloads in JSON format.
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or
     * edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#create-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_webhook(
        &self,
        org: &str,
        body: &crate::types::OrgsCreateWebhookRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgHook>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks",
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
     * Get an organization webhook.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}` endpoint.
     *
     * Returns a webhook configured in an organization. To get only the webhook
     * `config` properties, see "[Get a webhook configuration for an organization](/rest/orgs/webhooks#get-a-webhook-configuration-for-an-organization).
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#get-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn get_webhook(
        &self,
        org: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgHook>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Delete an organization webhook.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/hooks/{hook_id}` endpoint.
     *
     * Delete a webhook for an organization.
     *
     * The authenticated user must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#delete-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn delete_webhook(
        &self,
        org: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Update an organization webhook.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/hooks/{hook_id}` endpoint.
     *
     * Updates a webhook configured in an organization. When you update a webhook,
     * the `secret` will be overwritten. If you previously had a `secret` set, you must
     * provide the same `secret` or set a new `secret` or the secret will be removed. If
     * you are only updating individual webhook `config` properties, use "[Update a webhook
     * configuration for an organization](/rest/orgs/webhooks#update-a-webhook-configuration-for-an-organization)".
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#update-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn update_webhook(
        &self,
        org: &str,
        hook_id: i64,
        body: &crate::types::OrgsUpdateWebhookRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgHook>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get a webhook configuration for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}/config` endpoint.
     *
     * Returns the webhook configuration for an organization. To get more information about the webhook, including the `active` state and `events`, use "[Get an organization webhook ](/rest/orgs/webhooks#get-an-organization-webhook)."
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#get-a-webhook-configuration-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn get_webhook_config_for_org(
        &self,
        org: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/config",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Update a webhook configuration for an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/hooks/{hook_id}/config` endpoint.
     *
     * Updates the webhook configuration for an organization. To update more information about the webhook, including the `active` state and `events`, use "[Update an organization webhook ](/rest/orgs/webhooks#update-an-organization-webhook)."
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#update-a-webhook-configuration-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn update_webhook_config_for_org(
        &self,
        org: &str,
        hook_id: i64,
        body: &crate::types::AppsUpdateWebhookConfigAppRequest,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/config",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * List deliveries for an organization webhook.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}/deliveries` endpoint.
     *
     * Returns a list of webhook deliveries for a webhook configured in an organization.
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#list-deliveries-for-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `cursor: &str` -- Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors.
     */
    pub async fn list_webhook_deliveries(
        &self,
        org: &str,
        hook_id: i64,
        per_page: i64,
        cursor: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::HookDeliveryItem>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/deliveries?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * List deliveries for an organization webhook.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}/deliveries` endpoint.
     *
     * As opposed to `list_webhook_deliveries`, this function returns all the pages of the request at once.
     *
     * Returns a list of webhook deliveries for a webhook configured in an organization.
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#list-deliveries-for-an-organization-webhook>
     */
    pub async fn list_all_webhook_deliveries(
        &self,
        org: &str,
        hook_id: i64,
        cursor: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::HookDeliveryItem>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/deliveries?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
                query_
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
     * Get a webhook delivery for an organization webhook.
     *
     * This function performs a `GET` to the `/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}` endpoint.
     *
     * Returns a delivery for a webhook configured in an organization.
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#get-a-webhook-delivery-for-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     * * `delivery_id: i64`
     */
    pub async fn get_webhook_delivery(
        &self,
        org: &str,
        hook_id: i64,
        delivery_id: i64,
    ) -> ClientResult<crate::Response<crate::types::HookDelivery>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/deliveries/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
                crate::progenitor_support::encode_path(&delivery_id.to_string()),
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
     * Redeliver a delivery for an organization webhook.
     *
     * This function performs a `POST` to the `/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}/attempts` endpoint.
     *
     * Redeliver a delivery for a webhook configured in an organization.
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#redeliver-a-delivery-for-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     * * `delivery_id: i64`
     */
    pub async fn redeliver_webhook_delivery(
        &self,
        org: &str,
        hook_id: i64,
        delivery_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/deliveries/{}/attempts",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
                crate::progenitor_support::encode_path(&delivery_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Ping an organization webhook.
     *
     * This function performs a `POST` to the `/orgs/{org}/hooks/{hook_id}/pings` endpoint.
     *
     * This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event)
     * to be sent to the hook.
     *
     * You must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit
     * webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/webhooks#ping-an-organization-webhook>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn ping_webhook(&self, org: &str, hook_id: i64) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/hooks/{}/pings",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get route stats by actor.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/route-stats/{actor_type}/{actor_id}` endpoint.
     *
     * Get API request count statistics for an actor broken down by route within a specified time frame.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-route-stats-by-actor>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `actor_type: crate::types::ApiInsightsActorType` -- The type of the actor.
     * * `actor_id: i64` -- The ID of the actor.
     * * `min_timestamp: &str` -- The minimum timestamp to query for stats. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `max_timestamp: &str` -- The maximum timestamp to query for stats. Defaults to the time 30 days ago. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `sort: &[String]` -- The property to sort the results by.
     * * `api_route_substring: &str` -- Providing a substring will filter results where the API route contains the substring. This is a case-insensitive search.
     */
    pub async fn api_insights_get_route_stats_by_actor(
        &self,
        org: &str,
        actor_type: crate::types::ApiInsightsActorType,
        actor_id: i64,
        min_timestamp: &str,
        max_timestamp: &str,
        page: i64,
        per_page: i64,
        direction: crate::types::Order,
        sort: &[String],
        api_route_substring: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsRouteStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !api_route_substring.is_empty() {
            query_args.push((
                "api_route_substring".to_string(),
                api_route_substring.to_string(),
            ));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.is_empty() {
            query_args.push(("sort".to_string(), sort.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/route-stats/{}/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&actor_type.to_string()),
                crate::progenitor_support::encode_path(&actor_id.to_string()),
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
     * Get route stats by actor.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/route-stats/{actor_type}/{actor_id}` endpoint.
     *
     * As opposed to `api_insights_get_route_stats_by_actor`, this function returns all the pages of the request at once.
     *
     * Get API request count statistics for an actor broken down by route within a specified time frame.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-route-stats-by-actor>
     */
    pub async fn api_insights_get_all_route_stats_by_actor(
        &self,
        org: &str,
        actor_type: crate::types::ApiInsightsActorType,
        actor_id: i64,
        min_timestamp: &str,
        max_timestamp: &str,
        direction: crate::types::Order,
        sort: &[String],
        api_route_substring: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsRouteStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !api_route_substring.is_empty() {
            query_args.push((
                "api_route_substring".to_string(),
                api_route_substring.to_string(),
            ));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if !sort.is_empty() {
            query_args.push(("sort".to_string(), sort.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/route-stats/{}/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&actor_type.to_string()),
                crate::progenitor_support::encode_path(&actor_id.to_string()),
                query_
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
     * Get subject stats.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/subject-stats` endpoint.
     *
     * Get API request statistics for all subjects within an organization within a specified time frame. Subjects can be users or GitHub Apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-subject-stats>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `min_timestamp: &str` -- The minimum timestamp to query for stats. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `max_timestamp: &str` -- The maximum timestamp to query for stats. Defaults to the time 30 days ago. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `sort: &[String]` -- The property to sort the results by.
     * * `subject_name_substring: &str` -- Providing a substring will filter results where the subject name contains the substring. This is a case-insensitive search.
     */
    pub async fn api_insights_get_subject_stats(
        &self,
        org: &str,
        min_timestamp: &str,
        max_timestamp: &str,
        page: i64,
        per_page: i64,
        direction: crate::types::Order,
        sort: &[String],
        subject_name_substring: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsSubjectStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.is_empty() {
            query_args.push(("sort".to_string(), sort.join(" ")));
        }
        if !subject_name_substring.is_empty() {
            query_args.push((
                "subject_name_substring".to_string(),
                subject_name_substring.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/subject-stats?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get subject stats.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/subject-stats` endpoint.
     *
     * As opposed to `api_insights_get_subject_stats`, this function returns all the pages of the request at once.
     *
     * Get API request statistics for all subjects within an organization within a specified time frame. Subjects can be users or GitHub Apps.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-subject-stats>
     */
    pub async fn api_insights_get_all_subject_stats(
        &self,
        org: &str,
        min_timestamp: &str,
        max_timestamp: &str,
        direction: crate::types::Order,
        sort: &[String],
        subject_name_substring: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsSubjectStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if !sort.is_empty() {
            query_args.push(("sort".to_string(), sort.join(" ")));
        }
        if !subject_name_substring.is_empty() {
            query_args.push((
                "subject_name_substring".to_string(),
                subject_name_substring.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/subject-stats?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                query_
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
     * Get summary stats.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/summary-stats` endpoint.
     *
     * Get overall statistics of API requests made within an organization by all users and apps within a specified time frame.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-summary-stats>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `min_timestamp: &str` -- The minimum timestamp to query for stats. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `max_timestamp: &str` -- The maximum timestamp to query for stats. Defaults to the time 30 days ago. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     */
    pub async fn api_insights_get_summary_stats(
        &self,
        org: &str,
        min_timestamp: &str,
        max_timestamp: &str,
    ) -> ClientResult<crate::Response<crate::types::SummaryStats>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/summary-stats?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get summary stats by user.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/summary-stats/users/{user_id}` endpoint.
     *
     * Get overall statistics of API requests within the organization for a user.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-summary-stats-by-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `user_id: &str` -- The ID of the user to query for stats.
     * * `min_timestamp: &str` -- The minimum timestamp to query for stats. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `max_timestamp: &str` -- The maximum timestamp to query for stats. Defaults to the time 30 days ago. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     */
    pub async fn api_insights_get_summary_stats_by_user(
        &self,
        org: &str,
        user_id: &str,
        min_timestamp: &str,
        max_timestamp: &str,
    ) -> ClientResult<crate::Response<crate::types::SummaryStats>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/summary-stats/users/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
     * Get summary stats by actor.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/summary-stats/{actor_type}/{actor_id}` endpoint.
     *
     * Get overall statistics of API requests within the organization made by a specific actor. Actors can be GitHub App installations, OAuth apps or other tokens on behalf of a user.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-summary-stats-by-actor>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `min_timestamp: &str` -- The minimum timestamp to query for stats. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `max_timestamp: &str` -- The maximum timestamp to query for stats. Defaults to the time 30 days ago. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `actor_type: crate::types::ApiInsightsActorType` -- The type of the actor.
     * * `actor_id: i64` -- The ID of the actor.
     */
    pub async fn api_insights_get_summary_stats_by_actor(
        &self,
        org: &str,
        min_timestamp: &str,
        max_timestamp: &str,
        actor_type: crate::types::ApiInsightsActorType,
        actor_id: i64,
    ) -> ClientResult<crate::Response<crate::types::SummaryStats>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/summary-stats/{}/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&actor_type.to_string()),
                crate::progenitor_support::encode_path(&actor_id.to_string()),
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
     * Get time stats.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/time-stats` endpoint.
     *
     * Get the number of API requests and rate-limited requests made within an organization over a specified time period.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-time-stats>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `min_timestamp: &str` -- The minimum timestamp to query for stats. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `max_timestamp: &str` -- The maximum timestamp to query for stats. Defaults to the time 30 days ago. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `timestamp_increment: &str` -- The increment of time used to breakdown the query results (5m, 10m, 1h, etc.).
     */
    pub async fn api_insights_get_time_stats(
        &self,
        org: &str,
        min_timestamp: &str,
        max_timestamp: &str,
        timestamp_increment: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsTimeStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if !timestamp_increment.is_empty() {
            query_args.push((
                "timestamp_increment".to_string(),
                timestamp_increment.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/time-stats?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get time stats.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/time-stats` endpoint.
     *
     * As opposed to `api_insights_get_time_stats`, this function returns all the pages of the request at once.
     *
     * Get the number of API requests and rate-limited requests made within an organization over a specified time period.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-time-stats>
     */
    pub async fn api_insights_get_all_time_stats(
        &self,
        org: &str,
        min_timestamp: &str,
        max_timestamp: &str,
        timestamp_increment: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsTimeStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if !timestamp_increment.is_empty() {
            query_args.push((
                "timestamp_increment".to_string(),
                timestamp_increment.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/time-stats?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                query_
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
     * Get time stats by user.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/time-stats/users/{user_id}` endpoint.
     *
     * Get the number of API requests and rate-limited requests made within an organization by a specific user over a specified time period.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-time-stats-by-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `user_id: &str` -- The ID of the user to query for stats.
     * * `min_timestamp: &str` -- The minimum timestamp to query for stats. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `max_timestamp: &str` -- The maximum timestamp to query for stats. Defaults to the time 30 days ago. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `timestamp_increment: &str` -- The increment of time used to breakdown the query results (5m, 10m, 1h, etc.).
     */
    pub async fn api_insights_get_time_stats_by_user(
        &self,
        org: &str,
        user_id: &str,
        min_timestamp: &str,
        max_timestamp: &str,
        timestamp_increment: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsTimeStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if !timestamp_increment.is_empty() {
            query_args.push((
                "timestamp_increment".to_string(),
                timestamp_increment.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/time-stats/users/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
     * Get time stats by user.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/time-stats/users/{user_id}` endpoint.
     *
     * As opposed to `api_insights_get_time_stats_by_user`, this function returns all the pages of the request at once.
     *
     * Get the number of API requests and rate-limited requests made within an organization by a specific user over a specified time period.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-time-stats-by-user>
     */
    pub async fn api_insights_get_all_time_stats_by_user(
        &self,
        org: &str,
        user_id: &str,
        min_timestamp: &str,
        max_timestamp: &str,
        timestamp_increment: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsTimeStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if !timestamp_increment.is_empty() {
            query_args.push((
                "timestamp_increment".to_string(),
                timestamp_increment.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/time-stats/users/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&user_id.to_string()),
                query_
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
     * Get time stats by actor.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/time-stats/{actor_type}/{actor_id}` endpoint.
     *
     * Get the number of API requests and rate-limited requests made within an organization by a specific actor within a specified time period.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-time-stats-by-actor>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `actor_type: crate::types::ApiInsightsActorType` -- The type of the actor.
     * * `actor_id: i64` -- The ID of the actor.
     * * `min_timestamp: &str` -- The minimum timestamp to query for stats. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `max_timestamp: &str` -- The maximum timestamp to query for stats. Defaults to the time 30 days ago. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `timestamp_increment: &str` -- The increment of time used to breakdown the query results (5m, 10m, 1h, etc.).
     */
    pub async fn api_insights_get_time_stats_by_actor(
        &self,
        org: &str,
        actor_type: crate::types::ApiInsightsActorType,
        actor_id: i64,
        min_timestamp: &str,
        max_timestamp: &str,
        timestamp_increment: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsTimeStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if !timestamp_increment.is_empty() {
            query_args.push((
                "timestamp_increment".to_string(),
                timestamp_increment.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/time-stats/{}/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&actor_type.to_string()),
                crate::progenitor_support::encode_path(&actor_id.to_string()),
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
     * Get time stats by actor.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/time-stats/{actor_type}/{actor_id}` endpoint.
     *
     * As opposed to `api_insights_get_time_stats_by_actor`, this function returns all the pages of the request at once.
     *
     * Get the number of API requests and rate-limited requests made within an organization by a specific actor within a specified time period.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-time-stats-by-actor>
     */
    pub async fn api_insights_get_all_time_stats_by_actor(
        &self,
        org: &str,
        actor_type: crate::types::ApiInsightsActorType,
        actor_id: i64,
        min_timestamp: &str,
        max_timestamp: &str,
        timestamp_increment: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsTimeStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if !timestamp_increment.is_empty() {
            query_args.push((
                "timestamp_increment".to_string(),
                timestamp_increment.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/time-stats/{}/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&actor_type.to_string()),
                crate::progenitor_support::encode_path(&actor_id.to_string()),
                query_
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
     * Get user stats.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/user-stats/{user_id}` endpoint.
     *
     * Get API usage statistics within an organization for a user broken down by the type of access.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-user-stats>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `user_id: &str` -- The ID of the user to query for stats.
     * * `min_timestamp: &str` -- The minimum timestamp to query for stats. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `max_timestamp: &str` -- The maximum timestamp to query for stats. Defaults to the time 30 days ago. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `sort: &[String]` -- The property to sort the results by.
     * * `actor_name_substring: &str` -- Providing a substring will filter results where the actor name contains the substring. This is a case-insensitive search.
     */
    pub async fn api_insights_get_user_stats(
        &self,
        org: &str,
        user_id: &str,
        min_timestamp: &str,
        max_timestamp: &str,
        page: i64,
        per_page: i64,
        direction: crate::types::Order,
        sort: &[String],
        actor_name_substring: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsUserStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor_name_substring.is_empty() {
            query_args.push((
                "actor_name_substring".to_string(),
                actor_name_substring.to_string(),
            ));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.is_empty() {
            query_args.push(("sort".to_string(), sort.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/user-stats/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
     * Get user stats.
     *
     * This function performs a `GET` to the `/orgs/{org}/insights/api/user-stats/{user_id}` endpoint.
     *
     * As opposed to `api_insights_get_user_stats`, this function returns all the pages of the request at once.
     *
     * Get API usage statistics within an organization for a user broken down by the type of access.
     *
     * FROM: <https://docs.github.com/rest/orgs/api-insights#get-user-stats>
     */
    pub async fn api_insights_get_all_user_stats(
        &self,
        org: &str,
        user_id: &str,
        min_timestamp: &str,
        max_timestamp: &str,
        direction: crate::types::Order,
        sort: &[String],
        actor_name_substring: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApiInsightsUserStats>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor_name_substring.is_empty() {
            query_args.push((
                "actor_name_substring".to_string(),
                actor_name_substring.to_string(),
            ));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !max_timestamp.is_empty() {
            query_args.push(("max_timestamp".to_string(), max_timestamp.to_string()));
        }
        if !min_timestamp.is_empty() {
            query_args.push(("min_timestamp".to_string(), min_timestamp.to_string()));
        }
        if !sort.is_empty() {
            query_args.push(("sort".to_string(), sort.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/insights/api/user-stats/{}?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&user_id.to_string()),
                query_
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
     * List app installations for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/installations` endpoint.
     *
     * Lists all GitHub Apps in an organization. The installation count includes
     * all GitHub Apps installed on repositories in the organization.
     *
     * The authenticated user must be an organization owner to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:read` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#list-app-installations-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_app_installations(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::AppsListInstallationsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/installations?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List pending organization invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/invitations` endpoint.
     *
     * The return hash contains a `role` field which refers to the Organization
     * Invitation role and will be one of the following values: `direct_member`, `admin`,
     * `billing_manager`, or `hiring_manager`. If the invitee is not a GitHub
     * member, the `login` field in the return hash will be `null`.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-pending-organization-invitations>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `role: crate::types::OrgsListPendingInvitationsRole` -- Filter invitations by their member role.
     * * `invitation_source: crate::types::InvitationSource` -- Filter invitations by their invitation source.
     */
    pub async fn list_pending_invitations(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        role: crate::types::OrgsListPendingInvitationsRole,
        invitation_source: crate::types::InvitationSource,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !invitation_source.to_string().is_empty() {
            query_args.push((
                "invitation_source".to_string(),
                invitation_source.to_string(),
            ));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List pending organization invitations.
     *
     * This function performs a `GET` to the `/orgs/{org}/invitations` endpoint.
     *
     * As opposed to `list_pending_invitations`, this function returns all the pages of the request at once.
     *
     * The return hash contains a `role` field which refers to the Organization
     * Invitation role and will be one of the following values: `direct_member`, `admin`,
     * `billing_manager`, or `hiring_manager`. If the invitee is not a GitHub
     * member, the `login` field in the return hash will be `null`.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-pending-organization-invitations>
     */
    pub async fn list_all_pending_invitations(
        &self,
        org: &str,
        role: crate::types::OrgsListPendingInvitationsRole,
        invitation_source: crate::types::InvitationSource,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationInvitation>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !invitation_source.to_string().is_empty() {
            query_args.push((
                "invitation_source".to_string(),
                invitation_source.to_string(),
            ));
        }
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                query_
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
     * Create an organization invitation.
     *
     * This function performs a `POST` to the `/orgs/{org}/invitations` endpoint.
     *
     * Invite people to an organization by using their GitHub user ID or their email address. In order to create invitations in an organization, the authenticated user must be an organization owner.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * FROM: <https://docs.github.com/rest/orgs/members#create-an-organization-invitation>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_invitation(
        &self,
        org: &str,
        body: &crate::types::OrgsCreateInvitationRequest,
    ) -> ClientResult<crate::Response<crate::types::OrganizationInvitation>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations",
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
     * Cancel an organization invitation.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/invitations/{invitation_id}` endpoint.
     *
     * Cancel an organization invitation. In order to cancel an organization invitation, the authenticated user must be an organization owner.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications).
     *
     * FROM: <https://docs.github.com/rest/orgs/members#cancel-an-organization-invitation>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `invitation_id: i64` -- The unique identifier of the invitation.
     */
    pub async fn cancel_invitation(
        &self,
        org: &str,
        invitation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
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
     * List organization invitation teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/invitations/{invitation_id}/teams` endpoint.
     *
     * List all teams associated with an invitation. In order to see invitations in an organization, the authenticated user must be an organization owner.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-organization-invitation-teams>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `invitation_id: i64` -- The unique identifier of the invitation.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_invitation_teams(
        &self,
        org: &str,
        invitation_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations/{}/teams?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
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
     * List organization invitation teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/invitations/{invitation_id}/teams` endpoint.
     *
     * As opposed to `list_invitation_teams`, this function returns all the pages of the request at once.
     *
     * List all teams associated with an invitation. In order to see invitations in an organization, the authenticated user must be an organization owner.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-organization-invitation-teams>
     */
    pub async fn list_all_invitation_teams(
        &self,
        org: &str,
        invitation_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/invitations/{}/teams",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
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
     * List issue fields for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/issue-fields` endpoint.
     *
     * Lists all issue fields for an organization. OAuth app tokens and personal access tokens (classic) need the read:org scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-fields#list-issue-fields-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_issue_fields(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueField>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-fields",
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
     * List issue fields for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/issue-fields` endpoint.
     *
     * As opposed to `list_issue_fields`, this function returns all the pages of the request at once.
     *
     * Lists all issue fields for an organization. OAuth app tokens and personal access tokens (classic) need the read:org scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-fields#list-issue-fields-for-an-organization>
     */
    pub async fn list_all_issue_fields(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueField>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-fields",
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
     * Create issue field for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/issue-fields` endpoint.
     *
     * Creates a new issue field for an organization.
     *
     * You can find out more about issue fields in [Managing issue fields in an organization](https://docs.github.com/issues/tracking-your-work-with-issues/using-issues/managing-issue-fields-in-an-organization).
     *
     * To use this endpoint, the authenticated user must be an administrator for the organization. OAuth app tokens and
     * personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-fields#create-issue-field-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_issue_field(
        &self,
        org: &str,
        body: &crate::types::OrganizationCreateIssueField,
    ) -> ClientResult<crate::Response<crate::types::IssueField>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-fields",
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
     * Delete issue field for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/issue-fields/{issue_field_id}` endpoint.
     *
     * Deletes an issue field for an organization.
     *
     * You can find out more about issue fields in [Managing issue fields in an organization](https://docs.github.com/issues/tracking-your-work-with-issues/using-issues/managing-issue-fields-in-an-organization).
     *
     * To use this endpoint, the authenticated user must be an administrator for the organization. OAuth app tokens and
     * personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-fields#delete-issue-field-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `issue_field_id: i64` -- The unique identifier of the issue field.
     */
    pub async fn delete_issue_field(
        &self,
        org: &str,
        issue_field_id: i64,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-fields/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&issue_field_id.to_string()),
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
     * Update issue field for an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/issue-fields/{issue_field_id}` endpoint.
     *
     * Updates an issue field for an organization.
     *
     * You can find out more about issue fields in [Managing issue fields in an organization](https://docs.github.com/issues/tracking-your-work-with-issues/using-issues/managing-issue-fields-in-an-organization).
     *
     * To use this endpoint, the authenticated user must be an administrator for the organization. OAuth app tokens and
     * personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-fields#update-issue-field-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `issue_field_id: i64` -- The unique identifier of the issue field.
     */
    pub async fn update_issue_field(
        &self,
        org: &str,
        issue_field_id: i64,
        body: &crate::types::OrganizationUpdateIssueField,
    ) -> ClientResult<crate::Response<crate::types::IssueField>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-fields/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&issue_field_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * List issue types for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/issue-types` endpoint.
     *
     * Lists all issue types for an organization. OAuth app tokens and personal access tokens (classic) need the read:org scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-types#list-issue-types-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_issue_types(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueType>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-types",
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
     * List issue types for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/issue-types` endpoint.
     *
     * As opposed to `list_issue_types`, this function returns all the pages of the request at once.
     *
     * Lists all issue types for an organization. OAuth app tokens and personal access tokens (classic) need the read:org scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-types#list-issue-types-for-an-organization>
     */
    pub async fn list_all_issue_types(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::IssueType>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-types",
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
     * Create issue type for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/issue-types` endpoint.
     *
     * Create a new issue type for an organization.
     *
     * You can find out more about issue types in [Managing issue types in an organization](https://docs.github.com/issues/tracking-your-work-with-issues/configuring-issues/managing-issue-types-in-an-organization).
     *
     * To use this endpoint, the authenticated user must be an administrator for the organization. OAuth app tokens and
     * personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-types#create-issue-type-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_issue_type(
        &self,
        org: &str,
        body: &crate::types::OrganizationCreateIssueType,
    ) -> ClientResult<crate::Response<crate::types::IssueType>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-types",
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
     * Update issue type for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/issue-types/{issue_type_id}` endpoint.
     *
     * Updates an issue type for an organization.
     *
     * You can find out more about issue types in [Managing issue types in an organization](https://docs.github.com/issues/tracking-your-work-with-issues/configuring-issues/managing-issue-types-in-an-organization).
     *
     * To use this endpoint, the authenticated user must be an administrator for the organization. OAuth app tokens and
     * personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-types#update-issue-type-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `issue_type_id: i64` -- The unique identifier of the issue type.
     */
    pub async fn update_issue_type(
        &self,
        org: &str,
        issue_type_id: i64,
        body: &crate::types::OrganizationCreateIssueType,
    ) -> ClientResult<crate::Response<crate::types::IssueType>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-types/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&issue_type_id.to_string()),
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
     * Delete issue type for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/issue-types/{issue_type_id}` endpoint.
     *
     * Deletes an issue type for an organization.
     *
     * You can find out more about issue types in [Managing issue types in an organization](https://docs.github.com/issues/tracking-your-work-with-issues/configuring-issues/managing-issue-types-in-an-organization).
     *
     * To use this endpoint, the authenticated user must be an administrator for the organization. OAuth app tokens and
     * personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/issue-types#delete-issue-type-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `issue_type_id: i64` -- The unique identifier of the issue type.
     */
    pub async fn delete_issue_type(
        &self,
        org: &str,
        issue_type_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/issue-types/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&issue_type_id.to_string()),
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
     * List organization members.
     *
     * This function performs a `GET` to the `/orgs/{org}/members` endpoint.
     *
     * List all users who are members of an organization. If the authenticated user is also a member of this organization then both concealed and public members will be returned.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-organization-members>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `filter: crate::types::OrgsListMembersFilter` -- Filter members returned in the list. `2fa_disabled` means that only members without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled will be returned. `2fa_insecure` means that only members with [insecure 2FA methods](https://docs.github.com/organizations/keeping-your-organization-secure/managing-two-factor-authentication-for-your-organization/requiring-two-factor-authentication-in-your-organization#requiring-secure-methods-of-two-factor-authentication-in-your-organization) will be returned. These options are only available for organization owners.
     * * `role: crate::types::OrgsListMembersRole` -- Filter members returned by their role.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_members(
        &self,
        org: &str,
        filter: crate::types::OrgsListMembersFilter,
        role: crate::types::OrgsListMembersRole,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/members?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List organization members.
     *
     * This function performs a `GET` to the `/orgs/{org}/members` endpoint.
     *
     * As opposed to `list_members`, this function returns all the pages of the request at once.
     *
     * List all users who are members of an organization. If the authenticated user is also a member of this organization then both concealed and public members will be returned.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-organization-members>
     */
    pub async fn list_all_members(
        &self,
        org: &str,
        filter: crate::types::OrgsListMembersFilter,
        role: crate::types::OrgsListMembersRole,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !role.to_string().is_empty() {
            query_args.push(("role".to_string(), role.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/members?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                query_
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
     * Check organization membership for a user.
     *
     * This function performs a `GET` to the `/orgs/{org}/members/{username}` endpoint.
     *
     * Check if a user is, publicly or privately, a member of the organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#check-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn check_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/members/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Remove an organization member.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/members/{username}` endpoint.
     *
     * Removing a user from this list will remove them from all teams and they will no longer have any access to the organization's repositories.
     *
     * > [!NOTE]
     * > If a user has both direct membership in the organization as well as indirect membership via an enterprise team, only their direct membership will be removed. Their indirect membership via an enterprise team remains until the user is removed from the enterprise team.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#remove-an-organization-member>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn remove_member(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/members/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Get organization membership for a user.
     *
     * This function performs a `GET` to the `/orgs/{org}/memberships/{username}` endpoint.
     *
     * In order to get a user's membership with an organization, the authenticated user must be an organization member. The `state` parameter in the response can be used to identify the user's membership status.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#get-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgMembership>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/memberships/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Set organization membership for a user.
     *
     * This function performs a `PUT` to the `/orgs/{org}/memberships/{username}` endpoint.
     *
     * Only authenticated organization owners can add a member to the organization or update the member's role.
     *
     * *   If the authenticated user is _adding_ a member to the organization, the invited user will receive an email inviting them to the organization. The user's [membership status](https://docs.github.com/rest/orgs/members#get-organization-membership-for-a-user) will be `pending` until they accept the invitation.
     *     
     * *   Authenticated users can _update_ a user's membership by passing the `role` parameter. If the authenticated user changes a member's role to `admin`, the affected user will receive an email notifying them that they've been made an organization owner. If the authenticated user changes an owner's role to `member`, no email will be sent.
     *
     * **Rate limits**
     *
     * To prevent abuse, organization owners are limited to creating 50 organization invitations for an organization within a 24 hour period. If the organization is more than one month old or on a paid plan, the limit is 500 invitations per 24 hour period.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#set-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn set_membership_for_user(
        &self,
        org: &str,
        username: &str,
        body: &crate::types::OrgsSetMembershipUserRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgMembership>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/memberships/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Remove organization membership for a user.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/memberships/{username}` endpoint.
     *
     * In order to remove a user's membership with an organization, the authenticated user must be an organization owner.
     *
     * If the specified user is an active member of the organization, this will remove them from the organization. If the specified user has been invited to the organization, this will cancel their invitation. The specified user will receive an email notification in both cases.
     *
     * > [!NOTE]
     * > If a user has both direct membership in the organization as well as indirect membership via an enterprise team, only their direct membership will be removed. Their indirect membership via an enterprise team remains until the user is removed from the enterprise team.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#remove-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn remove_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/memberships/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Get all organization roles for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/organization-roles` endpoint.
     *
     * Lists the organization roles available in this organization. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * To use this endpoint, the authenticated user must be one of:
     *
     * - An administrator for the organization.
     * - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#get-all-organization-roles-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_org_roles(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgsListOrgRolesResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles",
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
     * Remove all organization roles for a team.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/organization-roles/teams/{team_slug}` endpoint.
     *
     * Removes all assigned organization roles from a team. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * The authenticated user must be an administrator for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     */
    pub async fn revoke_all_org_roles_team(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/teams/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * Assign an organization role to a team.
     *
     * This function performs a `PUT` to the `/orgs/{org}/organization-roles/teams/{team_slug}/{role_id}` endpoint.
     *
     * Assigns an organization role to a team in an organization. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * The authenticated user must be an administrator for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `role_id: i64` -- The unique identifier of the role.
     */
    pub async fn assign_team_to_org_role(
        &self,
        org: &str,
        team_slug: &str,
        role_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/teams/{}/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
                crate::progenitor_support::encode_path(&role_id.to_string()),
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
     * Remove an organization role from a team.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/organization-roles/teams/{team_slug}/{role_id}` endpoint.
     *
     * Removes an organization role from a team. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * The authenticated user must be an administrator for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     * * `role_id: i64` -- The unique identifier of the role.
     */
    pub async fn revoke_org_role_team(
        &self,
        org: &str,
        team_slug: &str,
        role_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/teams/{}/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
                crate::progenitor_support::encode_path(&role_id.to_string()),
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
     * Remove all organization roles for a user.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/organization-roles/users/{username}` endpoint.
     *
     * Revokes all assigned organization roles from a user. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * The authenticated user must be an administrator for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#remove-all-organization-roles-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn revoke_all_org_roles_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/users/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Assign an organization role to a user.
     *
     * This function performs a `PUT` to the `/orgs/{org}/organization-roles/users/{username}/{role_id}` endpoint.
     *
     * Assigns an organization role to a member of an organization. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * The authenticated user must be an administrator for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#assign-an-organization-role-to-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `role_id: i64` -- The unique identifier of the role.
     */
    pub async fn assign_user_to_org_role(
        &self,
        org: &str,
        username: &str,
        role_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/users/{}/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&role_id.to_string()),
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
     * Remove an organization role from a user.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/organization-roles/users/{username}/{role_id}` endpoint.
     *
     * Remove an organization role from a user. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * The authenticated user must be an administrator for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#remove-an-organization-role-from-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `role_id: i64` -- The unique identifier of the role.
     */
    pub async fn revoke_org_role_user(
        &self,
        org: &str,
        username: &str,
        role_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/users/{}/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&role_id.to_string()),
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
     * Get an organization role.
     *
     * This function performs a `GET` to the `/orgs/{org}/organization-roles/{role_id}` endpoint.
     *
     * Gets an organization role that is available to this organization. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * To use this endpoint, the authenticated user must be one of:
     *
     * - An administrator for the organization.
     * - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#get-an-organization-role>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `role_id: i64` -- The unique identifier of the role.
     */
    pub async fn get_org_role(
        &self,
        org: &str,
        role_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrganizationRole>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&role_id.to_string()),
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
     * List teams that are assigned to an organization role.
     *
     * This function performs a `GET` to the `/orgs/{org}/organization-roles/{role_id}/teams` endpoint.
     *
     * Lists the teams that are assigned to an organization role. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * To use this endpoint, you must be an administrator for the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#list-teams-that-are-assigned-to-an-organization-role>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `role_id: i64` -- The unique identifier of the role.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_org_role_teams(
        &self,
        org: &str,
        role_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamRoleAssignment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/{}/teams?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&role_id.to_string()),
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
     * List teams that are assigned to an organization role.
     *
     * This function performs a `GET` to the `/orgs/{org}/organization-roles/{role_id}/teams` endpoint.
     *
     * As opposed to `list_org_role_teams`, this function returns all the pages of the request at once.
     *
     * Lists the teams that are assigned to an organization role. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * To use this endpoint, you must be an administrator for the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#list-teams-that-are-assigned-to-an-organization-role>
     */
    pub async fn list_all_org_role_teams(
        &self,
        org: &str,
        role_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamRoleAssignment>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/{}/teams",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&role_id.to_string()),
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
     * List users that are assigned to an organization role.
     *
     * This function performs a `GET` to the `/orgs/{org}/organization-roles/{role_id}/users` endpoint.
     *
     * Lists organization members that are assigned to an organization role. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * To use this endpoint, you must be an administrator for the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#list-users-that-are-assigned-to-an-organization-role>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `role_id: i64` -- The unique identifier of the role.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_org_role_users(
        &self,
        org: &str,
        role_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::UserRoleAssignment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/{}/users?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&role_id.to_string()),
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
     * List users that are assigned to an organization role.
     *
     * This function performs a `GET` to the `/orgs/{org}/organization-roles/{role_id}/users` endpoint.
     *
     * As opposed to `list_org_role_users`, this function returns all the pages of the request at once.
     *
     * Lists organization members that are assigned to an organization role. For more information on organization roles, see "[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles)."
     *
     * To use this endpoint, you must be an administrator for the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/organization-roles#list-users-that-are-assigned-to-an-organization-role>
     */
    pub async fn list_all_org_role_users(
        &self,
        org: &str,
        role_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::UserRoleAssignment>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/organization-roles/{}/users",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&role_id.to_string()),
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
     * List outside collaborators for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/outside_collaborators` endpoint.
     *
     * List all users who are outside collaborators of an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/outside-collaborators#list-outside-collaborators-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `filter: crate::types::OrgsListMembersFilter` -- Filter members returned in the list. `2fa_disabled` means that only members without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled will be returned. `2fa_insecure` means that only members with [insecure 2FA methods](https://docs.github.com/organizations/keeping-your-organization-secure/managing-two-factor-authentication-for-your-organization/requiring-two-factor-authentication-in-your-organization#requiring-secure-methods-of-two-factor-authentication-in-your-organization) will be returned. These options are only available for organization owners.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_outside_collaborators(
        &self,
        org: &str,
        filter: crate::types::OrgsListMembersFilter,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/outside_collaborators?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List outside collaborators for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/outside_collaborators` endpoint.
     *
     * As opposed to `list_outside_collaborators`, this function returns all the pages of the request at once.
     *
     * List all users who are outside collaborators of an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/outside-collaborators#list-outside-collaborators-for-an-organization>
     */
    pub async fn list_all_outside_collaborators(
        &self,
        org: &str,
        filter: crate::types::OrgsListMembersFilter,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/outside_collaborators?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                query_
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
     * Convert an organization member to outside collaborator.
     *
     * This function performs a `PUT` to the `/orgs/{org}/outside_collaborators/{username}` endpoint.
     *
     * When an organization member is converted to an outside collaborator, they'll only have access to the repositories that their current team membership allows. The user will no longer be a member of the organization. For more information, see "[Converting an organization member to an outside collaborator](https://docs.github.com/articles/converting-an-organization-member-to-an-outside-collaborator/)". Converting an organization member to an outside collaborator may be restricted by enterprise administrators. For more information, see "[Enforcing repository management policies in your enterprise](https://docs.github.com/admin/policies/enforcing-policies-for-your-enterprise/enforcing-repository-management-policies-in-your-enterprise#enforcing-a-policy-for-inviting-outside-collaborators-to-repositories)."
     *
     * FROM: <https://docs.github.com/rest/orgs/outside-collaborators#convert-an-organization-member-to-outside-collaborator>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn convert_member_to_outside_collaborator(
        &self,
        org: &str,
        username: &str,
        body: &crate::types::OrgsConvertMemberOutsideCollaboratorRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/outside_collaborators/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Remove outside collaborator from an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/outside_collaborators/{username}` endpoint.
     *
     * Removing a user from this list will remove them from all the organization's repositories.
     *
     * FROM: <https://docs.github.com/rest/orgs/outside-collaborators#remove-outside-collaborator-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn remove_outside_collaborator(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/outside_collaborators/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List requests to access organization resources with fine-grained personal access tokens.
     *
     * This function performs a `GET` to the `/orgs/{org}/personal-access-token-requests` endpoint.
     *
     * Lists requests from organization members to access organization resources with a fine-grained personal access token.
     *
     * Only GitHub Apps can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/personal-access-tokens#list-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `sort: crate::types::PersonalAccessTokenSort` -- The property by which to sort the results.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `owner: &[String]` -- The functions in the package that are affected by the vulnerability.
     * * `repository: &str` -- The name of the repository to use to filter the results.
     * * `permission: &str` -- The permission to use to filter the results.
     * * `last_used_before: chrono::DateTime<chrono::Utc>` -- Only show fine-grained personal access tokens used before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `last_used_after: chrono::DateTime<chrono::Utc>` -- Only show fine-grained personal access tokens used after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `token_id: &[String]` -- The functions in the package that are affected by the vulnerability.
     */
    pub async fn list_pat_grant_requests(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        sort: crate::types::PersonalAccessTokenSort,
        direction: crate::types::Order,
        owner: &[String],
        repository: &str,
        permission: &str,
        last_used_before: Option<chrono::DateTime<chrono::Utc>>,
        last_used_after: Option<chrono::DateTime<chrono::Utc>>,
        token_id: &[String],
    ) -> ClientResult<crate::Response<crate::types::BasicError>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if let Some(date) = last_used_after {
            query_args.push(("last_used_after".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = last_used_before {
            query_args.push(("last_used_before".to_string(), date.to_rfc3339()));
        }
        if !owner.is_empty() {
            query_args.push(("owner".to_string(), owner.join(" ")));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !permission.is_empty() {
            query_args.push(("permission".to_string(), permission.to_string()));
        }
        if !repository.is_empty() {
            query_args.push(("repository".to_string(), repository.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !token_id.is_empty() {
            query_args.push(("token_id".to_string(), token_id.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/personal-access-token-requests?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Review requests to access organization resources with fine-grained personal access tokens.
     *
     * This function performs a `POST` to the `/orgs/{org}/personal-access-token-requests` endpoint.
     *
     * Approves or denies multiple pending requests to access organization resources via a fine-grained personal access token.
     *
     * Only GitHub Apps can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/personal-access-tokens#review-requests-to-access-organization-resources-with-fine-grained-personal-access-tokens>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn review_pat_grant_requests_in_bulk(
        &self,
        org: &str,
        body: &crate::types::OrgsReviewPatGrantRequestsInBulkRequest,
    ) -> ClientResult<crate::Response<crate::types::BasicError>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/personal-access-token-requests",
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
     * Review a request to access organization resources with a fine-grained personal access token.
     *
     * This function performs a `POST` to the `/orgs/{org}/personal-access-token-requests/{pat_request_id}` endpoint.
     *
     * Approves or denies a pending request to access organization resources via a fine-grained personal access token.
     *
     * Only GitHub Apps can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/personal-access-tokens#review-a-request-to-access-organization-resources-with-a-fine-grained-personal-access-token>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `pat_request_id: i64` -- Unique identifier of the request for access via fine-grained personal access token.
     */
    pub async fn review_pat_grant_request(
        &self,
        org: &str,
        pat_request_id: i64,
        body: &crate::types::OrgsReviewPatGrantRequest,
    ) -> ClientResult<crate::Response<crate::types::BasicError>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/personal-access-token-requests/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&pat_request_id.to_string()),
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
     * List repositories requested to be accessed by a fine-grained personal access token.
     *
     * This function performs a `GET` to the `/orgs/{org}/personal-access-token-requests/{pat_request_id}/repositories` endpoint.
     *
     * Lists the repositories a fine-grained personal access token request is requesting access to.
     *
     * Only GitHub Apps can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-requested-to-be-accessed-by-a-fine-grained-personal-access-token>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `pat_request_id: i64` -- Unique identifier of the request for access via fine-grained personal access token.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_pat_grant_request_repositories(
        &self,
        org: &str,
        pat_request_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::BasicError>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/personal-access-token-requests/{}/repositories?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&pat_request_id.to_string()),
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
     * List fine-grained personal access tokens with access to organization resources.
     *
     * This function performs a `GET` to the `/orgs/{org}/personal-access-tokens` endpoint.
     *
     * Lists approved fine-grained personal access tokens owned by organization members that can access organization resources.
     *
     * Only GitHub Apps can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/personal-access-tokens#list-fine-grained-personal-access-tokens-with-access-to-organization-resources>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `sort: crate::types::PersonalAccessTokenSort` -- The property by which to sort the results.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `owner: &[String]` -- The functions in the package that are affected by the vulnerability.
     * * `repository: &str` -- The name of the repository to use to filter the results.
     * * `permission: &str` -- The permission to use to filter the results.
     * * `last_used_before: chrono::DateTime<chrono::Utc>` -- Only show fine-grained personal access tokens used before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `last_used_after: chrono::DateTime<chrono::Utc>` -- Only show fine-grained personal access tokens used after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `token_id: &[String]` -- The functions in the package that are affected by the vulnerability.
     */
    pub async fn list_pat_grants(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        sort: crate::types::PersonalAccessTokenSort,
        direction: crate::types::Order,
        owner: &[String],
        repository: &str,
        permission: &str,
        last_used_before: Option<chrono::DateTime<chrono::Utc>>,
        last_used_after: Option<chrono::DateTime<chrono::Utc>>,
        token_id: &[String],
    ) -> ClientResult<crate::Response<crate::types::BasicError>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if let Some(date) = last_used_after {
            query_args.push(("last_used_after".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = last_used_before {
            query_args.push(("last_used_before".to_string(), date.to_rfc3339()));
        }
        if !owner.is_empty() {
            query_args.push(("owner".to_string(), owner.join(" ")));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !permission.is_empty() {
            query_args.push(("permission".to_string(), permission.to_string()));
        }
        if !repository.is_empty() {
            query_args.push(("repository".to_string(), repository.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !token_id.is_empty() {
            query_args.push(("token_id".to_string(), token_id.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/personal-access-tokens?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Update the access to organization resources via fine-grained personal access tokens.
     *
     * This function performs a `POST` to the `/orgs/{org}/personal-access-tokens` endpoint.
     *
     * Updates the access organization members have to organization resources via fine-grained personal access tokens. Limited to revoking a token's existing access.
     *
     * Only GitHub Apps can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-to-organization-resources-via-fine-grained-personal-access-tokens>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn update_pat_accesses(
        &self,
        org: &str,
        body: &crate::types::OrgsUpdatePatAccessesRequest,
    ) -> ClientResult<crate::Response<crate::types::BasicError>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/personal-access-tokens",
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
     * Update the access a fine-grained personal access token has to organization resources.
     *
     * This function performs a `POST` to the `/orgs/{org}/personal-access-tokens/{pat_id}` endpoint.
     *
     * Updates the access an organization member has to organization resources via a fine-grained personal access token. Limited to revoking the token's existing access. Limited to revoking a token's existing access.
     *
     * Only GitHub Apps can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/personal-access-tokens#update-the-access-a-fine-grained-personal-access-token-has-to-organization-resources>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `pat_id: i64` -- The unique identifier of the fine-grained personal access token.
     */
    pub async fn update_pat_access(
        &self,
        org: &str,
        pat_id: i64,
        body: &crate::types::OrgsUpdatePatAccessRequest,
    ) -> ClientResult<crate::Response<crate::types::BasicError>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/personal-access-tokens/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&pat_id.to_string()),
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
     * List repositories a fine-grained personal access token has access to.
     *
     * This function performs a `GET` to the `/orgs/{org}/personal-access-tokens/{pat_id}/repositories` endpoint.
     *
     * Lists the repositories a fine-grained personal access token has access to.
     *
     * Only GitHub Apps can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/personal-access-tokens#list-repositories-a-fine-grained-personal-access-token-has-access-to>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `pat_id: i64` -- Unique identifier of the fine-grained personal access token.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_pat_grant_repositories(
        &self,
        org: &str,
        pat_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::BasicError>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/personal-access-tokens/{}/repositories?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&pat_id.to_string()),
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
     * Get all custom properties for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/properties/schema` endpoint.
     *
     * Gets all custom properties defined for an organization.
     * Organization members can read these properties.
     *
     * FROM: <https://docs.github.com/rest/orgs/custom-properties#get-all-custom-properties-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn custom_properties_for_repos_get_organization_definitions(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CustomProperty>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/properties/schema",
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
     * Get all custom properties for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/properties/schema` endpoint.
     *
     * As opposed to `custom_properties_for_repos_get_organization_definitions`, this function returns all the pages of the request at once.
     *
     * Gets all custom properties defined for an organization.
     * Organization members can read these properties.
     *
     * FROM: <https://docs.github.com/rest/orgs/custom-properties#get-all-custom-properties-for-an-organization>
     */
    pub async fn custom_properties_for_repos_get_all_organization_definitions(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CustomProperty>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/properties/schema",
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
     * Create or update custom properties for an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/properties/schema` endpoint.
     *
     * Creates new or updates existing custom properties defined for an organization in a batch.
     *
     * If the property already exists, the existing property will be replaced with the new values.
     * Missing optional values will fall back to default values, previous values will be overwritten.
     * E.g. if a property exists with `values_editable_by: org_and_repo_actors` and it's updated without specifying `values_editable_by`, it will be updated to default value `org_actors`.
     *
     * To use this endpoint, the authenticated user must be one of:
     *   - An administrator for the organization.
     *   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-properties-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn custom_properties_for_repos_create_or_update_organization_definitions(
        &self,
        org: &str,
        body: &crate::types::OrgsCustomPropertiesReposCreateUpdateOrganizationDefinitionsRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::CustomProperty>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/properties/schema",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get a custom property for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/properties/schema/{custom_property_name}` endpoint.
     *
     * Gets a custom property that is defined for an organization.
     * Organization members can read these properties.
     *
     * FROM: <https://docs.github.com/rest/orgs/custom-properties#get-a-custom-property-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `custom_property_name: &str` -- The custom property name.
     */
    pub async fn custom_properties_for_repos_get_organization_definition(
        &self,
        org: &str,
        custom_property_name: &str,
    ) -> ClientResult<crate::Response<crate::types::CustomProperty>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/properties/schema/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&custom_property_name.to_string()),
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
     * Create or update a custom property for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/properties/schema/{custom_property_name}` endpoint.
     *
     * Creates a new or updates an existing custom property that is defined for an organization.
     *
     * To use this endpoint, the authenticated user must be one of:
     * - An administrator for the organization.
     * - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/custom-properties#create-or-update-a-custom-property-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `custom_property_name: &str` -- The custom property name.
     */
    pub async fn custom_properties_for_repos_create_or_update_organization_definition(
        &self,
        org: &str,
        custom_property_name: &str,
        body: &crate::types::CustomPropertySetPayload,
    ) -> ClientResult<crate::Response<crate::types::CustomProperty>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/properties/schema/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&custom_property_name.to_string()),
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
     * Remove a custom property for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/properties/schema/{custom_property_name}` endpoint.
     *
     * Removes a custom property that is defined for an organization.
     *
     * To use this endpoint, the authenticated user must be one of:
     *   - An administrator for the organization.
     *   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/custom-properties#remove-a-custom-property-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `custom_property_name: &str` -- The custom property name.
     */
    pub async fn custom_properties_for_repos_delete_organization_definition(
        &self,
        org: &str,
        custom_property_name: &str,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/properties/schema/{}",
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
     * List custom property values for organization repositories.
     *
     * This function performs a `GET` to the `/orgs/{org}/properties/values` endpoint.
     *
     * Lists organization repositories with all of their custom property values.
     * Organization members can read these properties.
     *
     * FROM: <https://docs.github.com/rest/orgs/custom-properties#list-custom-property-values-for-organization-repositories>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `repository_query: &str` -- Finds repositories in the organization with a query containing one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as the web interface for GitHub. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query). See "[Searching for repositories](https://docs.github.com/articles/searching-for-repositories/)" for a detailed list of qualifiers.
     */
    pub async fn custom_properties_for_repos_get_organization_values(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        repository_query: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgRepoCustomPropertyValues>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !repository_query.is_empty() {
            query_args.push(("repository_query".to_string(), repository_query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/properties/values?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List custom property values for organization repositories.
     *
     * This function performs a `GET` to the `/orgs/{org}/properties/values` endpoint.
     *
     * As opposed to `custom_properties_for_repos_get_organization_values`, this function returns all the pages of the request at once.
     *
     * Lists organization repositories with all of their custom property values.
     * Organization members can read these properties.
     *
     * FROM: <https://docs.github.com/rest/orgs/custom-properties#list-custom-property-values-for-organization-repositories>
     */
    pub async fn custom_properties_for_repos_get_all_organization_values(
        &self,
        org: &str,
        repository_query: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgRepoCustomPropertyValues>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !repository_query.is_empty() {
            query_args.push(("repository_query".to_string(), repository_query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/properties/values?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                query_
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
     * Create or update custom property values for organization repositories.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/properties/values` endpoint.
     *
     * Create new or update existing custom property values for repositories in a batch that belong to an organization.
     * Each target repository will have its custom property values updated to match the values provided in the request.
     *
     * A maximum of 30 repositories can be updated in a single request.
     *
     * Using a value of `null` for a custom property will remove or 'unset' the property value from the repository.
     *
     * To use this endpoint, the authenticated user must be one of:
     *   - An administrator for the organization.
     *   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_values_editor` in the organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/custom-properties#create-or-update-custom-property-values-for-organization-repositories>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn custom_properties_for_repos_create_or_update_organization_values(
        &self,
        org: &str,
        body: &crate::types::OrgsCustomPropertiesReposCreateUpdateOrganizationValuesRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/properties/values",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * List public organization members.
     *
     * This function performs a `GET` to the `/orgs/{org}/public_members` endpoint.
     *
     * Members of an organization can choose to have their membership publicized or not.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-public-organization-members>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_public_members(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List public organization members.
     *
     * This function performs a `GET` to the `/orgs/{org}/public_members` endpoint.
     *
     * As opposed to `list_public_members`, this function returns all the pages of the request at once.
     *
     * Members of an organization can choose to have their membership publicized or not.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-public-organization-members>
     */
    pub async fn list_all_public_members(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members",
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
     * Check public organization membership for a user.
     *
     * This function performs a `GET` to the `/orgs/{org}/public_members/{username}` endpoint.
     *
     * Check if the provided user is a public member of the organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#check-public-organization-membership-for-a-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn check_public_membership_for_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Set public organization membership for the authenticated user.
     *
     * This function performs a `PUT` to the `/orgs/{org}/public_members/{username}` endpoint.
     *
     * The user can publicize their own membership. (A user cannot publicize the membership for another user.)
     *
     * Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
     *
     * FROM: <https://docs.github.com/rest/orgs/members#set-public-organization-membership-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn set_public_membership_for_authenticated_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Remove public organization membership for the authenticated user.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/public_members/{username}` endpoint.
     *
     * Removes the public membership for the authenticated user from the specified organization, unless public visibility is enforced by default.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#remove-public-organization-membership-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn remove_public_membership_for_authenticated_user(
        &self,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/public_members/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Get organization ruleset history.
     *
     * This function performs a `GET` to the `/orgs/{org}/rulesets/{ruleset_id}/history` endpoint.
     *
     * Get the history of an organization ruleset.
     *
     * FROM: <https://docs.github.com/rest/orgs/rules#get-organization-ruleset-history>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `ruleset_id: i64` -- The ID of the ruleset.
     */
    pub async fn get_org_ruleset_history(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        ruleset_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::RulesetVersion>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets/{}/history?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Get organization ruleset history.
     *
     * This function performs a `GET` to the `/orgs/{org}/rulesets/{ruleset_id}/history` endpoint.
     *
     * As opposed to `get_org_ruleset_history`, this function returns all the pages of the request at once.
     *
     * Get the history of an organization ruleset.
     *
     * FROM: <https://docs.github.com/rest/orgs/rules#get-organization-ruleset-history>
     */
    pub async fn get_all_org_ruleset_history(
        &self,
        org: &str,
        ruleset_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::RulesetVersion>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets/{}/history",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Get organization ruleset version.
     *
     * This function performs a `GET` to the `/orgs/{org}/rulesets/{ruleset_id}/history/{version_id}` endpoint.
     *
     * Get a version of an organization ruleset.
     *
     * FROM: <https://docs.github.com/rest/orgs/rules#get-organization-ruleset-version>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `ruleset_id: i64` -- The ID of the ruleset.
     * * `version_id: i64` -- The ID of the version.
     */
    pub async fn get_org_ruleset_version(
        &self,
        org: &str,
        ruleset_id: i64,
        version_id: i64,
    ) -> ClientResult<crate::Response<crate::types::RulesetVersionWithStateAllOf>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets/{}/history/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
                crate::progenitor_support::encode_path(&version_id.to_string()),
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
     * List security manager teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/security-managers` endpoint.
     *
     * > [!WARNING]
     * > **Closing down notice:** This operation is closing down and will be removed starting January 1, 2026. Please use the "[Organization Roles](https://docs.github.com/rest/orgs/organization-roles)" endpoints instead.
     *
     * FROM: <https://docs.github.com/rest/orgs/security-managers#list-security-manager-teams>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_security_manager_teams(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamSimpleData>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/security-managers",
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
     * List security manager teams.
     *
     * This function performs a `GET` to the `/orgs/{org}/security-managers` endpoint.
     *
     * As opposed to `list_security_manager_teams`, this function returns all the pages of the request at once.
     *
     * > [!WARNING]
     * > **Closing down notice:** This operation is closing down and will be removed starting January 1, 2026. Please use the "[Organization Roles](https://docs.github.com/rest/orgs/organization-roles)" endpoints instead.
     *
     * FROM: <https://docs.github.com/rest/orgs/security-managers#list-security-manager-teams>
     */
    pub async fn list_all_security_manager_teams(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::TeamSimpleData>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/security-managers",
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
     * Add a security manager team.
     *
     * This function performs a `PUT` to the `/orgs/{org}/security-managers/teams/{team_slug}` endpoint.
     *
     * > [!WARNING]
     * > **Closing down notice:** This operation is closing down and will be removed starting January 1, 2026. Please use the "[Organization Roles](https://docs.github.com/rest/orgs/organization-roles)" endpoints instead.
     *
     * FROM: <https://docs.github.com/rest/orgs/security-managers#add-a-security-manager-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     */
    pub async fn add_security_manager_team(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/security-managers/teams/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * Remove a security manager team.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/security-managers/teams/{team_slug}` endpoint.
     *
     * > [!WARNING]
     * > **Closing down notice:** This operation is closing down and will be removed starting January 1, 2026. Please use the "[Organization Roles](https://docs.github.com/rest/orgs/organization-roles)" endpoints instead.
     *
     * FROM: <https://docs.github.com/rest/orgs/security-managers#remove-a-security-manager-team>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `team_slug: &str` -- The slug of the team name.
     */
    pub async fn remove_security_manager_team(
        &self,
        org: &str,
        team_slug: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/security-managers/teams/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&team_slug.to_string()),
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
     * Get immutable releases settings for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/settings/immutable-releases` endpoint.
     *
     * Gets the immutable releases policy for repositories in an organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#get-immutable-releases-settings-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_immutable_releases_settings(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ImmutableReleasesOrganizationSettings>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/immutable-releases",
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
     * Set immutable releases settings for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/settings/immutable-releases` endpoint.
     *
     * Sets the immutable releases policy for repositories in an organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#set-immutable-releases-settings-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_immutable_releases_settings(
        &self,
        org: &str,
        body: &crate::types::OrgsSetImmutableReleasesSettingsRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/immutable-releases",
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
    /**
     * List selected repositories for immutable releases enforcement.
     *
     * This function performs a `GET` to the `/orgs/{org}/settings/immutable-releases/repositories` endpoint.
     *
     * List all of the repositories that have been selected for immutable releases enforcement in an organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#list-selected-repositories-for-immutable-releases-enforcement>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_immutable_releases_settings_repositories(
        &self,
        org: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<crate::types::CodespacesListRepositoriesSecretResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/immutable-releases/repositories?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Set selected repositories for immutable releases enforcement.
     *
     * This function performs a `PUT` to the `/orgs/{org}/settings/immutable-releases/repositories` endpoint.
     *
     * Replaces all repositories that have been selected for immutable releases enforcement in an organization. To use this endpoint, the organization immutable releases policy for `enforced_repositories` must be configured to `selected`.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#set-selected-repositories-for-immutable-releases-enforcement>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_immutable_releases_settings_repositories(
        &self,
        org: &str,
        body: &crate::types::CodespacesSetRepositoriesSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/immutable-releases/repositories",
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
    /**
     * Enable a selected repository for immutable releases in an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/settings/immutable-releases/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to the list of selected repositories that are enforced for immutable releases in an organization. To use this endpoint, the organization immutable releases policy for `enforced_repositories` must be configured to `selected`.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#enable-a-selected-repository-for-immutable-releases-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `repository_id: i64` -- The unique identifier of the repository.
     */
    pub async fn enable_selected_repository_immutable_releases_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/immutable-releases/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&repository_id.to_string()),
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
     * Disable a selected repository for immutable releases in an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/settings/immutable-releases/repositories/{repository_id}` endpoint.
     *
     * Removes a repository from the list of selected repositories that are enforced for immutable releases in an organization. To use this endpoint, the organization immutable releases policy for `enforced_repositories` must be configured to `selected`.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#disable-a-selected-repository-for-immutable-releases-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `repository_id: i64` -- The unique identifier of the repository.
     */
    pub async fn disable_selected_repository_immutable_releases_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/settings/immutable-releases/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&repository_id.to_string()),
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
     * Enable or disable a security feature for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/{security_product}/{enablement}` endpoint.
     *
     * > [!WARNING]
     * > **Closing down notice:** The ability to enable or disable a security feature for all eligible repositories in an organization is closing down. Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead. For more information, see the [changelog](https://github.blog/changelog/2024-07-22-deprecation-of-api-endpoint-to-enable-or-disable-a-security-feature-for-an-organization/).
     *
     * Enables or disables the specified security feature for all eligible repositories in an organization. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
     *
     * The authenticated user must be an organization owner or be member of a team with the security manager role to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org`, `write:org`, or `repo` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#enable-or-disable-a-security-feature-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `security_product: crate::types::SecurityProduct` -- The security feature to enable or disable.
     * * `enablement: crate::types::Enablement` -- The action to take.
     *  
     *  `enable_all` means to enable the specified security feature for all repositories in the organization.
     *  `disable_all` means to disable the specified security feature for all repositories in the organization.
     */
    pub async fn enable_or_disable_security_product_on_all_org_repos(
        &self,
        org: &str,
        security_product: crate::types::SecurityProduct,
        enablement: crate::types::Enablement,
        body: &crate::types::OrgsEnableDisableSecurityProductOnAllOrgReposRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/{}/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&security_product.to_string()),
                crate::progenitor_support::encode_path(&enablement.to_string()),
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
     * List organization memberships for the authenticated user.
     *
     * This function performs a `GET` to the `/user/memberships/orgs` endpoint.
     *
     * Lists all of the authenticated user's organization memberships.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-organization-memberships-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `state: crate::types::OrgMembershipState` -- The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_memberships_for_authenticated_user(
        &self,
        state: crate::types::OrgMembershipState,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgMembership>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/memberships/orgs?{}", query_), None);
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
     * List organization memberships for the authenticated user.
     *
     * This function performs a `GET` to the `/user/memberships/orgs` endpoint.
     *
     * As opposed to `list_memberships_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists all of the authenticated user's organization memberships.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#list-organization-memberships-for-the-authenticated-user>
     */
    pub async fn list_all_memberships_for_authenticated_user(
        &self,
        state: crate::types::OrgMembershipState,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgMembership>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/memberships/orgs?{}", query_), None);
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
     * Get an organization membership for the authenticated user.
     *
     * This function performs a `GET` to the `/user/memberships/orgs/{org}` endpoint.
     *
     * If the authenticated user is an active or pending member of the organization, this endpoint will return the user's membership. If the authenticated user is not affiliated with the organization, a `404` is returned. This endpoint will return a `403` if the request is made by a GitHub App that is blocked by the organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#get-an-organization-membership-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_membership_for_authenticated_user(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgMembership>> {
        let url = self.client.url(
            &format!(
                "/user/memberships/orgs/{}",
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
     * Update an organization membership for the authenticated user.
     *
     * This function performs a `PATCH` to the `/user/memberships/orgs/{org}` endpoint.
     *
     * Converts the authenticated user to an active member of the organization, if that user has a pending invitation from the organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/members#update-an-organization-membership-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn update_membership_for_authenticated_user(
        &self,
        org: &str,
        body: &crate::types::OrgsUpdateMembershipRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgMembership>> {
        let url = self.client.url(
            &format!(
                "/user/memberships/orgs/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * List organizations for the authenticated user.
     *
     * This function performs a `GET` to the `/user/orgs` endpoint.
     *
     * List organizations for the authenticated user.
     *
     * For OAuth app tokens and personal access tokens (classic), this endpoint only lists organizations that your authorization allows you to operate on in some way (e.g., you can list teams with `read:org` scope, you can publicize your organization membership with `user` scope, etc.). Therefore, this API requires at least `user` or `read:org` scope for OAuth app tokens and personal access tokens (classic). Requests with insufficient scope will receive a `403 Forbidden` response.
     *
     * > [!NOTE]
     * > Requests using a fine-grained access token will receive a `200 Success` response with an empty list.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/orgs?{}", query_), None);
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
     * List organizations for the authenticated user.
     *
     * This function performs a `GET` to the `/user/orgs` endpoint.
     *
     * As opposed to `list_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * List organizations for the authenticated user.
     *
     * For OAuth app tokens and personal access tokens (classic), this endpoint only lists organizations that your authorization allows you to operate on in some way (e.g., you can list teams with `read:org` scope, you can publicize your organization membership with `user` scope, etc.). Therefore, this API requires at least `user` or `read:org` scope for OAuth app tokens and personal access tokens (classic). Requests with insufficient scope will receive a `403 Forbidden` response.
     *
     * > [!NOTE]
     * > Requests using a fine-grained access token will receive a `200 Success` response with an empty list.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user>
     */
    pub async fn list_all_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let url = self.client.url(&"/user/orgs".to_string(), None);
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
     * List organizations for a user.
     *
     * This function performs a `GET` to the `/users/{username}/orgs` endpoint.
     *
     * List [public organization memberships](https://docs.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.
     *
     * This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user) API instead.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#list-organizations-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/orgs?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List organizations for a user.
     *
     * This function performs a `GET` to the `/users/{username}/orgs` endpoint.
     *
     * As opposed to `list_for_user`, this function returns all the pages of the request at once.
     *
     * List [public organization memberships](https://docs.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.
     *
     * This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user) API instead.
     *
     * FROM: <https://docs.github.com/rest/orgs/orgs#list-organizations-for-a-user>
     */
    pub async fn list_all_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrganizationSimple>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/orgs",
                crate::progenitor_support::encode_path(&username.to_string()),
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
}

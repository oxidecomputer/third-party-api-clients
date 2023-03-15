use crate::Client;
use crate::ClientResult;

pub struct NotaryJurisdiction {
    pub client: Client,
}

impl NotaryJurisdiction {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        NotaryJurisdiction { client }
    }

    /**
     * Returns a list of jurisdictions that the notary is registered in.
     *
     * This function performs a `GET` to the `/v2.1/current_user/notary/jurisdictions` endpoint.
     *
     * Returns a list of jurisdictions that the notary is registered in.
     * The current user must be a notary.
     */
    pub async fn s_get(&self) -> ClientResult<crate::types::NotaryJurisdictionList> {
        let url = self
            .client
            .url("/v2.1/current_user/notary/jurisdictions", None);
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
     * Creates a jurisdiction object.
     *
     * This function performs a `POST` to the `/v2.1/current_user/notary/jurisdictions` endpoint.
     *
     * Creates a jurisdiction object.
     */
    pub async fn s_post(
        &self,
        body: &crate::types::NotaryJurisdictionData,
    ) -> ClientResult<crate::types::NotaryJurisdictionData> {
        let url = self
            .client
            .url("/v2.1/current_user/notary/jurisdictions", None);
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
     * Gets a jurisdiction object for the current user. The user must be a notary.
     *
     * This function performs a `GET` to the `/v2.1/current_user/notary/jurisdictions/{jurisdictionId}` endpoint.
     *
     * Gets a jurisdiction object for the current user.  The following restrictions apply:
     *
     * - The current user must be a notary.
     * - The `jurisdictionId` must be a jurisdiction that the notary is registered for.
     *
     *
     * **Parameters:**
     *
     * * `jurisdiction_id: &str` -- The ID of the jurisdiction.
     *   The following jurisdictions
     *   are supported:
     *   
     *   -  `5 - California`
     *   -  `6 - Colorado`
     *   -  `9 - Florida`
     *   -  `10 - Georgia`
     *   -  `12 - Idaho`
     *   -  `13 - Illinois`
     *   -  `14 - Indiana`
     *   -  `15 - Iowa`
     *   -  `17 - Kentucky`
     *   -  `23 - Minnesota`
     *   -  `25 - Missouri`
     *   -  `30 - New Jersey`
     *   -  `32 - New York`
     *   -  `33 - North Carolina`
     *   -  `35 - Ohio`
     *   -  `37 - Oregon`
     *   -  `38 - Pennsylvania`
     *   -  `40 - South Carolina`
     *   -  `43 - Texas`
     *   -  `44 - Utah`
     *   -  `47 - Washington`
     *   -  `48 - West Virginia`
     *   -  `49 - Wisconsin`
     *   -  `62 - Florida Commissioner of Deeds`
     *   .
     */
    pub async fn s_get_jurisdiction(
        &self,
        jurisdiction_id: &str,
    ) -> ClientResult<crate::types::NotaryJurisdictionData> {
        let url = self.client.url(
            &format!(
                "/v2.1/current_user/notary/jurisdictions/{}",
                crate::progenitor_support::encode_path(jurisdiction_id),
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
     * Updates the jurisdiction information about a notary.
     *
     * This function performs a `PUT` to the `/v2.1/current_user/notary/jurisdictions/{jurisdictionId}` endpoint.
     *
     * Updates the jurisdiction information about a notary.
     *
     * The following restrictions apply:
     *
     * - The current user must be a notary.
     * - The `jurisdictionId` path parameter must be a jurisdiction that the notary is registered for.
     * - The `jurisdictionId` path parameter must match the request body's `jurisdiction.jurisdictionId`.
     *
     * The request body must have a full `jurisdiction` object for the jurisdiction property.
     * The best way to do this is to use `getNotaryJurisdiction` to obtain the current values and update the properties you want to change.
     *
     * For example, assume `getNotaryJurisdiction` returns this:
     *
     * ```
     * {
     *     "jurisdiction": {
     *         "jurisdictionId": "15",
     *         "name": "Iowa",
     *         "county": "",
     *         "enabled": "true",
     *         "countyInSeal": "false",
     *         "commissionIdInSeal": "true",
     *         "stateNameInSeal": "true",
     *         "notaryPublicInSeal": "true",
     *         "allowSystemCreatedSeal": "true",
     *         "allowUserUploadedSeal": "false"
     *     },
     *     "commissionId": "123456",
     *     "commissionExpiration": "2020-08-31T07:00:00.0000000Z",
     *     "registeredName": "Bob Notary",
     *     "county": "Adams",
     *     "sealType": "system_created"
     * }
     * ```
     *
     * If you want to change the name of the notary from "Bob Notary" to "Robert Notary", your request body would be:
     *
     * ```
     * {
     *     "jurisdiction": {
     *         "jurisdictionId": "15",
     *         "name": "Iowa",
     *         "county": "",
     *         "enabled": "true",
     *         "countyInSeal": "false",
     *         "commissionIdInSeal": "true",
     *         "stateNameInSeal": "true",
     *         "notaryPublicInSeal": "true",
     *         "allowSystemCreatedSeal": "true",
     *         "allowUserUploadedSeal": "false"
     *     },
     *     "commissionId": "123456",
     *     "commissionExpiration": "2020-08-31T07:00:00.0000000Z",
     *     "registeredName": "Robert Notary",
     *     "county": "Adams",
     *     "sealType": "system_created"
     * }
     * ```
     *
     *
     * **Parameters:**
     *
     * * `jurisdiction_id: &str` -- The ID of the jurisdiction.
     *   The following jurisdictions
     *   are supported:
     *   
     *   -  `5 - California`
     *   -  `6 - Colorado`
     *   -  `9 - Florida`
     *   -  `10 - Georgia`
     *   -  `12 - Idaho`
     *   -  `13 - Illinois`
     *   -  `14 - Indiana`
     *   -  `15 - Iowa`
     *   -  `17 - Kentucky`
     *   -  `23 - Minnesota`
     *   -  `25 - Missouri`
     *   -  `30 - New Jersey`
     *   -  `32 - New York`
     *   -  `33 - North Carolina`
     *   -  `35 - Ohio`
     *   -  `37 - Oregon`
     *   -  `38 - Pennsylvania`
     *   -  `40 - South Carolina`
     *   -  `43 - Texas`
     *   -  `44 - Utah`
     *   -  `47 - Washington`
     *   -  `48 - West Virginia`
     *   -  `49 - Wisconsin`
     *   -  `62 - Florida Commissioner of Deeds`
     *   .
     */
    pub async fn s_put_jurisdiction(
        &self,
        jurisdiction_id: &str,
        body: &crate::types::NotaryJurisdictionData,
    ) -> ClientResult<crate::types::NotaryJurisdictionData> {
        let url = self.client.url(
            &format!(
                "/v2.1/current_user/notary/jurisdictions/{}",
                crate::progenitor_support::encode_path(jurisdiction_id),
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
     * Deletes the specified jurisdiction.
     *
     * This function performs a `DELETE` to the `/v2.1/current_user/notary/jurisdictions/{jurisdictionId}` endpoint.
     *
     * Deletes the specified jurisdiction.
     *
     * **Parameters:**
     *
     * * `jurisdiction_id: &str` -- The ID of the jurisdiction.
     *   The following jurisdictions
     *   are supported:
     *   
     *   -  `5 - California`
     *   -  `6 - Colorado`
     *   -  `9 - Florida`
     *   -  `10 - Georgia`
     *   -  `12 - Idaho`
     *   -  `13 - Illinois`
     *   -  `14 - Indiana`
     *   -  `15 - Iowa`
     *   -  `17 - Kentucky`
     *   -  `23 - Minnesota`
     *   -  `25 - Missouri`
     *   -  `30 - New Jersey`
     *   -  `32 - New York`
     *   -  `33 - North Carolina`
     *   -  `35 - Ohio`
     *   -  `37 - Oregon`
     *   -  `38 - Pennsylvania`
     *   -  `40 - South Carolina`
     *   -  `43 - Texas`
     *   -  `44 - Utah`
     *   -  `47 - Washington`
     *   -  `48 - West Virginia`
     *   -  `49 - Wisconsin`
     *   -  `62 - Florida Commissioner of Deeds`
     *   .
     */
    pub async fn s_delete_jurisdiction(&self, jurisdiction_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/current_user/notary/jurisdictions/{}",
                crate::progenitor_support::encode_path(jurisdiction_id),
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

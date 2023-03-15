use crate::Client;
use crate::ClientResult;

pub struct AccountConsumerDisclosures {
    pub client: Client,
}

impl AccountConsumerDisclosures {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountConsumerDisclosures { client }
    }

    /**
     * Gets the default Electronic Record and Signature Disclosure for an account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/consumer_disclosure` endpoint.
     *
     * Retrieves the default, HTML-formatted Electronic Record and Signature Disclosure (ERSD) associated with the account.
     *
     * This is the default ERSD disclosure that DocuSign provides for the convenience of U.S.-based customers only. This default disclosure is only valid for transactions between U.S.-based parties.
     *
     * To set the language of the disclosure that you want to retrieve, use the optional `langCode` query parameter.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `lang_code: &str` -- (Optional) The code for the signer language version of the disclosure that you want to retrieve. The following languages are supported:
     *   
     *   - Arabic (`ar`)
     *   - Bulgarian (`bg`)
     *   - Czech (`cs`)
     *   - Chinese Simplified (`zh_CN`)
     *   - Chinese Traditional (`zh_TW`)
     *   - Croatian (`hr`)
     *   - Danish (`da`)
     *   - Dutch (`nl`)
     *   - English US (`en`)
     *   - English UK (`en_GB`)
     *   - Estonian (`et`)
     *   - Farsi (`fa`)
     *   - Finnish (`fi`)
     *   - French (`fr`)
     *   - French Canadian (`fr_CA`)
     *   - German (`de`)
     *   - Greek (`el`)
     *   - Hebrew (`he`)
     *   - Hindi (`hi`)
     *   - Hungarian (`hu`)
     *   - Bahasa Indonesian (`id`)
     *   - Italian (`it`)
     *   - Japanese (`ja`)
     *   - Korean (`ko`)
     *   - Latvian (`lv`)
     *   - Lithuanian (`lt`)
     *   - Bahasa Melayu (`ms`)
     *   - Norwegian (`no`)
     *   - Polish (`pl`)
     *   - Portuguese (`pt`)
     *   - Portuguese Brazil (`pt_BR`)
     *   - Romanian (`ro`)
     *   - Russian (`ru`)
     *   - Serbian (`sr`)
     *   - Slovak (`sk`)
     *   - Slovenian (`sl`)
     *   - Spanish (`es`)
     *   - Spanish Latin America (`es_MX`)
     *   - Swedish (`sv`)
     *   - Thai (`th`)
     *   - Turkish (`tr`)
     *   - Ukrainian (`uk`)
     *   - Vietnamese (`vi`)
     *   
     *   Additionally, you can automatically detect the browser language being used by the viewer and display the disclosure in that language by setting the value to `browser`.
     */
    pub async fn consumer_disclosure_get(
        &self,
        account_id: &str,
        lang_code: &str,
    ) -> ClientResult<crate::types::AccountConsumerDisclosures> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !lang_code.is_empty() {
            query_args.push(("langCode".to_string(), lang_code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/consumer_disclosure?{}",
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
     * Gets the Electronic Record and Signature Disclosure for an account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/consumer_disclosure/{langCode}` endpoint.
     *
     * Retrieves the HTML-formatted Electronic Record and Signature Disclosure (ERSD) associated with the account.
     *
     * To set the language of the disclosure that you want to retrieve, use the optional `langCode` query parameter.
     *
     * **Note**: The text of the default disclosure is always in English, but if you are using a custom disclosure and have created versions of it in different signer languages, you can use the `langCode` parameter to specify the signer language version that you want to retrieve.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `lang_code: &str` -- (Optional) The code for the signer language version of the disclosure that you want to retrieve. The following languages are supported:
     *   
     *   - Arabic (`ar`)
     *   - Bulgarian (`bg`)
     *   - Czech (`cs`)
     *   - Chinese Simplified (`zh_CN`)
     *   - Chinese Traditional (`zh_TW`)
     *   - Croatian (`hr`)
     *   - Danish (`da`)
     *   - Dutch (`nl`)
     *   - English US (`en`)
     *   - English UK (`en_GB`)
     *   - Estonian (`et`)
     *   - Farsi (`fa`)
     *   - Finnish (`fi`)
     *   - French (`fr`)
     *   - French Canadian (`fr_CA`)
     *   - German (`de`)
     *   - Greek (`el`)
     *   - Hebrew (`he`)
     *   - Hindi (`hi`)
     *   - Hungarian (`hu`)
     *   - Bahasa Indonesian (`id`)
     *   - Italian (`it`)
     *   - Japanese (`ja`)
     *   - Korean (`ko`)
     *   - Latvian (`lv`)
     *   - Lithuanian (`lt`)
     *   - Bahasa Melayu (`ms`)
     *   - Norwegian (`no`)
     *   - Polish (`pl`)
     *   - Portuguese (`pt`)
     *   - Portuguese Brazil (`pt_BR`)
     *   - Romanian (`ro`)
     *   - Russian (`ru`)
     *   - Serbian (`sr`)
     *   - Slovak (`sk`)
     *   - Slovenian (`sl`)
     *   - Spanish (`es`)
     *   - Spanish Latin America (`es_MX`)
     *   - Swedish (`sv`)
     *   - Thai (`th`)
     *   - Turkish (`tr`)
     *   - Ukrainian (`uk`)
     *   - Vietnamese (`vi`)
     *   
     *   Additionally, you can automatically detect the browser language being used by the viewer and display the disclosure in that language by setting the value to `browser`.
     */
    pub async fn consumer_disclosure_get_lang_code(
        &self,
        account_id: &str,
        lang_code: &str,
    ) -> ClientResult<crate::types::AccountConsumerDisclosures> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/consumer_disclosure/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(lang_code),
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
     * Updates the Electronic Record and Signature Disclosure for an account.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/consumer_disclosure/{langCode}` endpoint.
     *
     * Account administrators can use this method to perform the following tasks:
     *
     * - Customize values in the default disclosure.
     * - Switch to a custom disclosure that uses your own text and HTML formatting.
     * - Change values in your existing consumer disclosure.
     *
     * To specify the signer language version of the disclosure that you are updating, use the optional `langCode` query parameter.
     *
     * **Note**: Only account administrators can use this method. Each time you change the disclosure content, all unsigned recipients of outstanding documents will be required to accept a new version.
     *
     * ## Updating the default disclosure
     *
     * When you update the default disclosure, you can edit all properties except for the following ones:
     *
     * - `accountEsignId`: This property is read only.
     * - `custom`: The default value is **false**. Editing this property causes the default disclosure to switch to a custom disclosure.
     * - `esignAgreement`: This property is read only.
     * - `esignText`: You cannot edit this property when `custom` is set to **false**. The API returns a 200 OK HTTP response, but does not update the `esignText`.
     * - Metadata properties: These properties are read only.
     *
     * **Note**: The text of the default disclosure is always in English.
     *
     * ## Switching to a custom disclosure
     *
     * To switch to a custom disclosure, set the `custom` property to **true** and customize the value for the `eSignText` property.
     *
     * You can also edit all of the other properties except for the following ones:
     *
     * - `accountEsignId`: This property is read only.
     * - `esignAgreement`: This property is read only.
     * - Metadata properties: These properties are read only.
     *
     * **Note**: When you use a custom disclosure, you can create versions of it in different signer languages and se the `langCode` parameter to specify the signer language version that you are updating.
     *
     * **Important**:  When you switch from a default to a custom disclosure, note the following information:
     *
     * - You will not be able to return to using the default disclosure.
     * - Only the disclosure for the currently selected signer language is saved. DocuSign will not automatically translate your custom disclosure. You must create a disclosure for each language that your signers use.
     *
     * ## Updating a custom disclosure
     *
     * When you update a custom disclosure, you can update all of the properties except for the following ones:
     *
     * - `accountEsignId`: This property is read only.
     * - `esignAgreement`: This property is read only.
     * - Metadata properties: These properties are read only.
     *
     * **Important**: Only the disclosure for the currently selected signer language is saved. DocuSign will not automatically translate your custom disclosure. You must create a disclosure for each language that your signers use.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `lang_code: &str` -- (Optional) The code for the signer language version of the disclosure that you want to update. The following languages are supported:
     *   
     *   - Arabic (`ar`)
     *   - Bulgarian (`bg`)
     *   - Czech (`cs`)
     *   - Chinese Simplified (`zh_CN`)
     *   - Chinese Traditional (`zh_TW`)
     *   - Croatian (`hr`)
     *   - Danish (`da`)
     *   - Dutch (`nl`)
     *   - English US (`en`)
     *   - English UK (`en_GB`)
     *   - Estonian (`et`)
     *   - Farsi (`fa`)
     *   - Finnish (`fi`)
     *   - French (`fr`)
     *   - French Canadian (`fr_CA`)
     *   - German (`de`)
     *   - Greek (`el`)
     *   - Hebrew (`he`)
     *   - Hindi (`hi`)
     *   - Hungarian (`hu`)
     *   - Bahasa Indonesian (`id`)
     *   - Italian (`it`)
     *   - Japanese (`ja`)
     *   - Korean (`ko`)
     *   - Latvian (`lv`)
     *   - Lithuanian (`lt`)
     *   - Bahasa Melayu (`ms`)
     *   - Norwegian (`no`)
     *   - Polish (`pl`)
     *   - Portuguese (`pt`)
     *   - Portuguese Brazil (`pt_BR`)
     *   - Romanian (`ro`)
     *   - Russian (`ru`)
     *   - Serbian (`sr`)
     *   - Slovak (`sk`)
     *   - Slovenian (`sl`)
     *   - Spanish (`es`)
     *   - Spanish Latin America (`es_MX`)
     *   - Swedish (`sv`)
     *   - Thai (`th`)
     *   - Turkish (`tr`)
     *   - Ukrainian (`uk`)
     *   - Vietnamese (`vi`)
     *   
     *   Additionally, you can automatically detect the browser language being used by the viewer and display the disclosure in that language by setting the value to `browser`.
     * * `include_metadata: &str` -- (Optional) When set to true, the response includes metadata indicating which properties are editable.
     */
    pub async fn consumer_disclosure_put(
        &self,
        account_id: &str,
        lang_code: &str,
        include_metadata: &str,
        body: &crate::types::ConsumerDisclosure,
    ) -> ClientResult<crate::types::ConsumerDisclosure> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_metadata.is_empty() {
            query_args.push(("include_metadata".to_string(), include_metadata.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/consumer_disclosure/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(lang_code),
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
}

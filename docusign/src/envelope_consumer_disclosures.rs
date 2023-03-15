use crate::Client;
use crate::ClientResult;

pub struct EnvelopeConsumerDisclosures {
    pub client: Client,
}

impl EnvelopeConsumerDisclosures {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeConsumerDisclosures { client }
    }

    /**
     * Gets the default Electronic Record and Signature Disclosure for an envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/consumer_disclosure` endpoint.
     *
     * Retrieves the default, HTML-formatted Electronic Record and Signature Disclosure (ERSD) for the envelope that you specify.
     *
     * This is the default ERSD disclosure that DocuSign provides for the convenience of U.S.-based customers only. This default disclosure is only valid for transactions between U.S.-based parties.
     *
     * To set the language of the disclosure that you want to retrieve, use the optional `langCode` query parameter.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
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
    pub async fn consumer_disclosure_get_envelope_recipient(
        &self,
        account_id: &str,
        envelope_id: &str,
        recipient_id: &str,
        lang_code: &str,
    ) -> ClientResult<crate::types::ConsumerDisclosure> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !lang_code.is_empty() {
            query_args.push(("langCode".to_string(), lang_code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}/consumer_disclosure?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
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
     * Gets the Electronic Record and Signature Disclosure for a specific envelope recipient.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/consumer_disclosure/{langCode}` endpoint.
     *
     * Retrieves the HTML-formatted Electronic Record and Signature Disclosure (ERSD) for the envelope recipient that you specify. This disclosure might differ from the account-level disclosure, based on the signing brand applied to the envelope and the recipient's language settings.
     *
     * To set the language of the disclosure that you want to retrieve, specify the `langCode` as either a path or query parameter.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `lang_code: &str` -- (Optional) The code for the signer language version of the disclosure that you want to retrieve, as a path parameter. The following languages are supported:
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
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `lang_code: &str` -- (Optional) The code for the signer language version of the disclosure that you want to retrieve, as a query parameter. The following languages are supported:
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
    pub async fn consumer_disclosure_get_envelope_recipient_lang_code(
        &self,
        account_id: &str,
        envelope_id: &str,
        lang_code: &str,
        recipient_id: &str,
    ) -> ClientResult<crate::types::ConsumerDisclosure> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}/consumer_disclosure/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
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
}

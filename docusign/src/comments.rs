use anyhow::Result;

use crate::Client;

pub struct Comments {
    pub client: Client,
}

impl Comments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Comments { client }
    }

    /**
     * Gets a PDF transcript of all of the comments in an envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/comments/transcript` endpoint.
     *
     * Retrieves a PDF file containing all of the comments that senders and recipients have added to the documents in an envelope.
     *
     * **Note**: Comments are disabled by default. To use the comments feature, an account administrator must enable comments on the account (in the `accountSettingsInformation` object, set the `enableSigningExtensionComments` property to **true**).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `encoding: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get_transcript(
        &self,
        account_id: &str,
        envelope_id: &str,
        encoding: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !encoding.is_empty() {
            query_args.push(format!("encoding={}", encoding));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/comments/transcript?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&envelope_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}

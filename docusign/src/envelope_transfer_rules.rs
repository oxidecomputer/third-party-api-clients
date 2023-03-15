use crate::Client;
use crate::ClientResult;

pub struct EnvelopeTransferRules {
    pub client: Client,
}

impl EnvelopeTransferRules {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeTransferRules { client }
    }

    /**
     * Gets envelope transfer rules.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/transfer_rules` endpoint.
     *
     * This method retrieves a list of envelope transfer rules associated with an account.
     *
     * **Note**: Only Administrators can create and use envelope transfer rules. In addition, to use envelope transfer rules, the **Transfer Custody** feature must be enabled for your account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- (Optional) The position within the total result set from which to start returning values. The value **thumbnail** may be used to return the page image.
     */
    pub async fn get(
        &self,
        account_id: &str,
        count: &str,
        start_position: &str,
    ) -> ClientResult<crate::types::EnvelopeTransferRuleInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/transfer_rules?{}",
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
     * Changes the status of multiple envelope transfer rules.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/transfer_rules` endpoint.
     *
     * This method changes the status for one or more envelope transfer rules based on the `envelopeTransferRuleId`s in the request body. You use this method to change whether or not the rules are enabled.
     *
     * **Note**: You cannot change any other information about the envelope transfer rule. Only Administrators can update envelope transfer rules. In addition, to use envelope transfer rules, the **Transfer Custody** feature must be enabled for your account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        body: &crate::types::EnvelopeTransferRuleInformation,
    ) -> ClientResult<crate::types::EnvelopeTransferRuleInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/transfer_rules",
                crate::progenitor_support::encode_path(account_id),
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
     * Creates an envelope transfer rule.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/transfer_rules` endpoint.
     *
     * This method creates an envelope transfer rule.
     *
     * When you create an envelope transfer rule, you specify the following properties:
     *
     * - `eventType`
     * - `fromGroups`
     * - `toUser`
     * - `toFolder`
     * - `carbonCopyOriginalOwner`
     * - `enabled`
     *
     * **Note**: Only Administrators can create envelope transfer rules. In addition, to use envelope transfer rules, the **Transfer Custody** feature must be enabled for your account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn post(
        &self,
        account_id: &str,
        body: &crate::types::EnvelopeTransferRuleRequest,
    ) -> ClientResult<crate::types::EnvelopeTransferRuleInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/transfer_rules",
                crate::progenitor_support::encode_path(account_id),
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
     * Changes the status of an envelope transfer rule.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/transfer_rules/{envelopeTransferRuleId}` endpoint.
     *
     * This method changes the status of an envelope transfer rule. You use this method to change whether or not the rule is enabled.
     *
     * You must include the `envelopeTransferRuleId` both as a query parameter, and in the request body.
     *
     * **Note**: You cannot change any other information about the envelope transfer rule. Only Administrators can update an envelope transfer rule. In addition, to use envelope transfer rules, the **Transfer Custody** feature must be enabled for your account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_transfer_rule_id: &str` -- The id of the envelope transfer rule. The system generates this id when the rule is first created.
     */
    pub async fn put_rule(
        &self,
        account_id: &str,
        envelope_transfer_rule_id: &str,
        body: &crate::types::EnvelopeTransferRule,
    ) -> ClientResult<crate::types::EnvelopeTransferRule> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/transfer_rules/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_transfer_rule_id),
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
     * Deletes an envelope transfer rule.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/transfer_rules/{envelopeTransferRuleId}` endpoint.
     *
     * This method deletes an envelope transfer rule.
     *
     * **Note**: Only Administrators can delete envelope transfer rules. In addition, to use envelope transfer rules, the **Transfer Custody** feature must be enabled for your account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_transfer_rule_id: &str` -- The id of the envelope transfer rule. The system generates this id when the rule is first created.
     */
    pub async fn delete(
        &self,
        account_id: &str,
        envelope_transfer_rule_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/transfer_rules/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_transfer_rule_id),
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

use anyhow::Result;

use crate::Client;

pub struct Policies {
    pub client: Client,
}

impl Policies {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Policies { client }
    }

    /**
    * This function performs a `GET` to the `/api/v1/policies` endpoint.
    *
    * Gets all policies with the specified type.
    *
    * **Parameters:**
    *
    * * `type_: &str`
    * * `status: &str`
    * * `expand: &str`
    */
    pub async fn list(
        &self,
        type_: &str,
        status: &str,
        expand: &str,
    ) -> Result<Vec<crate::types::Policy>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/api/v1/policies?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/api/v1/policies` endpoint.
    *
    * As opposed to `list`, this function returns all the pages of the request at once.
    *
    * Gets all policies with the specified type.
    */
    pub async fn list_all(
        &self,
        type_: &str,
        status: &str,
        expand: &str,
    ) -> Result<Vec<crate::types::Policy>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/api/v1/policies?{}", query_);

        self.client.get_all_pages(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/api/v1/policies` endpoint.
    *
    * Creates a policy.
    *
    * **Parameters:**
    *
    * * `activate: bool`
    */
    pub async fn create_policy(
        &self,
        activate: bool,
        body: &crate::types::Policy,
    ) -> Result<crate::types::Policy> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if activate {
            query_args.push(("activate".to_string(), activate.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/api/v1/policies?{}", query_);

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * This function performs a `GET` to the `/api/v1/policies/{policyId}` endpoint.
    *
    * Gets a policy.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    * * `expand: &str`
    */
    pub async fn get_policy(&self, policy_id: &str, expand: &str) -> Result<crate::types::Policy> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/api/v1/policies/{}?{}",
            crate::progenitor_support::encode_path(policy_id),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `PUT` to the `/api/v1/policies/{policyId}` endpoint.
    *
    * Updates a policy.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    */
    pub async fn update_policy(
        &self,
        policy_id: &str,
        body: &crate::types::Policy,
    ) -> Result<crate::types::Policy> {
        let url = format!(
            "/api/v1/policies/{}",
            crate::progenitor_support::encode_path(policy_id),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * This function performs a `DELETE` to the `/api/v1/policies/{policyId}` endpoint.
    *
    * Removes a policy.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    */
    pub async fn delete_policy(&self, policy_id: &str) -> Result<()> {
        let url = format!(
            "/api/v1/policies/{}",
            crate::progenitor_support::encode_path(policy_id),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/api/v1/policies/{policyId}/lifecycle/activate` endpoint.
    *
    * Activates a policy.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    */
    pub async fn activate_policy(&self, policy_id: &str) -> Result<()> {
        let url = format!(
            "/api/v1/policies/{}/lifecycle/activate",
            crate::progenitor_support::encode_path(policy_id),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/api/v1/policies/{policyId}/lifecycle/deactivate` endpoint.
    *
    * Deactivates a policy.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    */
    pub async fn deactivate_policy(&self, policy_id: &str) -> Result<()> {
        let url = format!(
            "/api/v1/policies/{}/lifecycle/deactivate",
            crate::progenitor_support::encode_path(policy_id),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/api/v1/policies/{policyId}/rules` endpoint.
    *
    * Enumerates all policy rules.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    */
    pub async fn list_policy_rules(
        &self,
        policy_id: &str,
    ) -> Result<Vec<crate::types::PolicyRule>> {
        let url = format!(
            "/api/v1/policies/{}/rules",
            crate::progenitor_support::encode_path(policy_id),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/api/v1/policies/{policyId}/rules` endpoint.
    *
    * As opposed to `list_policy_rules`, this function returns all the pages of the request at once.
    *
    * Enumerates all policy rules.
    */
    pub async fn list_all_policy_rules(
        &self,
        policy_id: &str,
    ) -> Result<Vec<crate::types::PolicyRule>> {
        let url = format!(
            "/api/v1/policies/{}/rules",
            crate::progenitor_support::encode_path(policy_id),
        );

        self.client.get_all_pages(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/api/v1/policies/{policyId}/rules` endpoint.
    *
    * Creates a policy rule.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    */
    pub async fn create_policy_rule(
        &self,
        policy_id: &str,
        body: &crate::types::PolicyRule,
    ) -> Result<crate::types::PolicyRule> {
        let url = format!(
            "/api/v1/policies/{}/rules",
            crate::progenitor_support::encode_path(policy_id),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * This function performs a `GET` to the `/api/v1/policies/{policyId}/rules/{ruleId}` endpoint.
    *
    * Gets a policy rule.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    * * `rule_id: &str`
    */
    pub async fn get_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> Result<crate::types::PolicyRule> {
        let url = format!(
            "/api/v1/policies/{}/rules/{}",
            crate::progenitor_support::encode_path(policy_id),
            crate::progenitor_support::encode_path(rule_id),
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `PUT` to the `/api/v1/policies/{policyId}/rules/{ruleId}` endpoint.
    *
    * Updates a policy rule.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    * * `rule_id: &str`
    */
    pub async fn update_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
        body: &crate::types::PolicyRule,
    ) -> Result<crate::types::PolicyRule> {
        let url = format!(
            "/api/v1/policies/{}/rules/{}",
            crate::progenitor_support::encode_path(policy_id),
            crate::progenitor_support::encode_path(rule_id),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * This function performs a `DELETE` to the `/api/v1/policies/{policyId}/rules/{ruleId}` endpoint.
    *
    * Removes a policy rule.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    * * `rule_id: &str`
    */
    pub async fn delete_policy_rule(&self, policy_id: &str, rule_id: &str) -> Result<()> {
        let url = format!(
            "/api/v1/policies/{}/rules/{}",
            crate::progenitor_support::encode_path(policy_id),
            crate::progenitor_support::encode_path(rule_id),
        );

        self.client.delete(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/api/v1/policies/{policyId}/rules/{ruleId}/lifecycle/activate` endpoint.
    *
    * Activates a policy rule.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    * * `rule_id: &str`
    */
    pub async fn activate_policy_rule(&self, policy_id: &str, rule_id: &str) -> Result<()> {
        let url = format!(
            "/api/v1/policies/{}/rules/{}/lifecycle/activate",
            crate::progenitor_support::encode_path(policy_id),
            crate::progenitor_support::encode_path(rule_id),
        );

        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/api/v1/policies/{policyId}/rules/{ruleId}/lifecycle/deactivate` endpoint.
    *
    * Deactivates a policy rule.
    *
    * **Parameters:**
    *
    * * `policy_id: &str`
    * * `rule_id: &str`
    */
    pub async fn deactivate_policy_rule(&self, policy_id: &str, rule_id: &str) -> Result<()> {
        let url = format!(
            "/api/v1/policies/{}/rules/{}/lifecycle/deactivate",
            crate::progenitor_support::encode_path(policy_id),
            crate::progenitor_support::encode_path(rule_id),
        );

        self.client.post(&url, None).await
    }
}

use crate::Client;
use crate::ClientResult;

pub struct IpAccessManagement {
    pub client: Client,
}

impl IpAccessManagement {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        IpAccessManagement { client }
    }

    /**
     * Retrieve a list of currently allowed IPs.
     *
     * This function performs a `GET` to the `/access_settings/whitelist` endpoint.
     *
     * **This endpoint allows you to retrieve a list of IP addresses that are currently allowed to access your account.**
     *
     * Each IP address returned to you will have `created_at` and `updated_at` dates. Each IP will also be associated with an `id` that can be used to remove the address from your allow list.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_access_settings_whitelist(
        &self,
    ) -> ClientResult<crate::types::IpAccessResponse> {
        let url = self.client.url("/access_settings/whitelist", None);
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
     * Add one or more IPs to the allow list.
     *
     * This function performs a `POST` to the `/access_settings/whitelist` endpoint.
     *
     * **This endpoint allows you to add one or more allowed IP addresses.**
     *
     * To allow one or more IP addresses, pass them to this endpoint in an array. Once an IP address is added to your allow list, it will be assigned an `id` that can be used to remove the address. You can retrieve the ID associated with an IP using the "Retrieve a list of currently allowed IPs" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_access_settings_whitelist(
        &self,
        body: &crate::types::PostAccessSettingsWhitelistRequest,
    ) -> ClientResult<crate::types::IpAccessResponse> {
        let url = self.client.url("/access_settings/whitelist", None);
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
     * Remove one or more IPs from the allow list.
     *
     * This function performs a `DELETE` to the `/access_settings/whitelist` endpoint.
     *
     * **This endpoint allows you to remove one or more IP addresses from your list of allowed addresses.**
     *
     * To remove one or more IP addresses, pass this endpoint an array containing the ID(s) associated with the IP(s) you intend to remove. You can retrieve the IDs associated with your allowed IP addresses using the "Retrieve a list of currently allowed IPs" endpoint.
     *
     * It is possible to remove your own IP address, which will block access to your account. You will need to submit a [support ticket](https://sendgrid.com/docs/ui/account-and-settings/support/) if this happens. For this reason, it is important to double check that you are removing only the IPs you intend to remove when using this endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_access_settings_whitelist(
        &self,
        body: &crate::types::DeleteAccessSettingsWhitelistRequest,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url("/access_settings/whitelist", None);
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieve all recent access attempts.
     *
     * This function performs a `GET` to the `/access_settings/activity` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all of the IP addresses that recently attempted to access your account either through the User Interface or the API.**
     *
     * **Parameters:**
     *
     * * `limit: i64` -- Limits the number of IPs to return.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_access_settings_activity(
        &self,
        limit: i64,
    ) -> ClientResult<crate::types::GetAccessSettingsActivityResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/access_settings/activity?{}", query_), None);
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
     * Retrieve a specific allowed IP.
     *
     * This function performs a `GET` to the `/access_settings/whitelist/{rule_id}` endpoint.
     *
     * **This endpoint allows you to retreive a specific IP address that has been allowed to access your account.**
     *
     * You must include the ID for the specific IP address you want to retrieve in your call. You can retrieve the IDs associated with your allowed IP addresses using the "Retrieve a  list of currently allowed IPs" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_access_settings_whitelist_rule(
        &self,
        rule_id: &str,
    ) -> ClientResult<crate::types::IpAccessResponse> {
        let url = self.client.url(
            &format!(
                "/access_settings/whitelist/{}",
                crate::progenitor_support::encode_path(rule_id),
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
     * Remove a specific IP from the allowed list.
     *
     * This function performs a `DELETE` to the `/access_settings/whitelist/{rule_id}` endpoint.
     *
     * **This endpoint allows you to remove a specific IP address from your list of allowed addresses.**
     *
     * When removing a specific IP address from your list, you must include the ID in your call.  You can retrieve the IDs associated with your allowed IP addresses using the "Retrieve a list of currently allowed IPs" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_access_settings_whitelist_rule(
        &self,
        rule_id: &str,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/access_settings/whitelist/{}",
                crate::progenitor_support::encode_path(rule_id),
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

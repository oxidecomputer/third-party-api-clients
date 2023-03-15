use crate::Client;
use crate::ClientResult;

pub struct IpAddresses {
    pub client: Client,
}

impl IpAddresses {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        IpAddresses { client }
    }

    /**
     * Retrieve all IP addresses.
     *
     * This function performs a `GET` to the `/ips` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all assigned and unassigned IPs.**
     *
     * Response includes warm up status, pools, assigned subusers, and reverse DNS info. The start_date field corresponds to when warmup started for that IP.
     *
     * A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.
     *
     * **Parameters:**
     *
     * * `ip: &str` -- The license key provided with your New Relic account.
     * * `exclude_whitelabels: bool` -- Indicates if your subuser statistics will be sent to your New Relic Dashboard.
     * * `limit: i64` -- The number of IPs you want returned at the same time.
     * * `offset: i64` -- The offset for the number of IPs that you are requesting.
     * * `subuser: &str` -- The license key provided with your New Relic account.
     * * `sort_by_direction: crate::types::SortByDirection` -- The direction to sort the results.
     */
    pub async fn get_ips(
        &self,
        ip: &str,
        exclude_whitelabels: bool,
        limit: i64,
        offset: i64,
        subuser: &str,
        sort_by_direction: crate::types::SortByDirection,
    ) -> ClientResult<Vec<crate::types::GetIpsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if exclude_whitelabels {
            query_args.push((
                "exclude_whitelabels".to_string(),
                exclude_whitelabels.to_string(),
            ));
        }
        if !ip.is_empty() {
            query_args.push(("ip".to_string(), ip.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !sort_by_direction.to_string().is_empty() {
            query_args.push((
                "sort_by_direction".to_string(),
                sort_by_direction.to_string(),
            ));
        }
        if !subuser.is_empty() {
            query_args.push(("subuser".to_string(), subuser.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/ips?{}", query_), None);
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
     * Retrieve all IP addresses.
     *
     * This function performs a `GET` to the `/ips` endpoint.
     *
     * As opposed to `get_ips`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a list of all assigned and unassigned IPs.**
     *
     * Response includes warm up status, pools, assigned subusers, and reverse DNS info. The start_date field corresponds to when warmup started for that IP.
     *
     * A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.
     */
    pub async fn get_all_ips(
        &self,
        ip: &str,
        exclude_whitelabels: bool,
        offset: i64,
        subuser: &str,
        sort_by_direction: crate::types::SortByDirection,
    ) -> ClientResult<Vec<crate::types::GetIpsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if exclude_whitelabels {
            query_args.push((
                "exclude_whitelabels".to_string(),
                exclude_whitelabels.to_string(),
            ));
        }
        if !ip.is_empty() {
            query_args.push(("ip".to_string(), ip.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !sort_by_direction.to_string().is_empty() {
            query_args.push((
                "sort_by_direction".to_string(),
                sort_by_direction.to_string(),
            ));
        }
        if !subuser.is_empty() {
            query_args.push(("subuser".to_string(), subuser.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/ips?{}", query_), None);
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
     * Add IPs.
     *
     * This function performs a `POST` to the `/ips` endpoint.
     *
     * **This endpoint is for adding a(n) IP Address(es) to your account.**
     */
    pub async fn post_ip(
        &self,
        body: &crate::types::PostIpsRequest,
    ) -> ClientResult<crate::types::PostIpsResponseData> {
        let url = self.client.url("/ips", None);
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
     * Get remaining IPs count.
     *
     * This function performs a `GET` to the `/ips/remaining` endpoint.
     *
     * **This endpoint gets amount of IP Addresses that can still be created during a given period and the price of those IPs.**
     */
    pub async fn get_ips_remaining(&self) -> ClientResult<crate::types::GetIpsRemainingResponse> {
        let url = self.client.url("/ips/remaining", None);
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
     * Retrieve all assigned IPs.
     *
     * This function performs a `GET` to the `/ips/assigned` endpoint.
     *
     * **This endpoint allows you to retrieve only assigned IP addresses.**
     *
     * A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.
     */
    pub async fn get_ips_assigned(
        &self,
    ) -> ClientResult<Vec<crate::types::GetIpsAssignedResponse>> {
        let url = self.client.url("/ips/assigned", None);
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
     * Retrieve all assigned IPs.
     *
     * This function performs a `GET` to the `/ips/assigned` endpoint.
     *
     * As opposed to `get_ips_assigned`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve only assigned IP addresses.**
     *
     * A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.
     */
    pub async fn get_all_ips_assigned(
        &self,
    ) -> ClientResult<Vec<crate::types::GetIpsAssignedResponse>> {
        let url = self.client.url("/ips/assigned", None);
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
     * Retrieve all IP pools an IP address belongs to.
     *
     * This function performs a `GET` to the `/ips/{ip_address}` endpoint.
     *
     * **This endpoint allows you to see which IP pools a particular IP address has been added to.**
     *
     * The same IP address can be added to multiple IP pools.
     *
     * A single IP address or a range of IP addresses may be dedicated to an account in order to send email for multiple domains. The reputation of this IP is based on the aggregate performance of all the senders who use it.
     */
    pub async fn get_ips_ip_address(
        &self,
        ip_address: &str,
    ) -> ClientResult<crate::types::GetIpsIpAddressResponse> {
        let url = self.client.url(
            &format!(
                "/ips/{}",
                crate::progenitor_support::encode_path(ip_address),
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

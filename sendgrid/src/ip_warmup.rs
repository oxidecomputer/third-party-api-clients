use crate::Client;
use crate::ClientResult;

pub struct IpWarmup {
    pub client: Client,
}

impl IpWarmup {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        IpWarmup { client }
    }

    /**
     * Retrieve all IPs currently in warmup.
     *
     * This function performs a `GET` to the `/ips/warmup` endpoint.
     *
     * **This endpoint allows you to retrieve all of your IP addresses that are currently warming up.**
     */
    pub async fn get_ips_warmup(&self) -> ClientResult<Vec<crate::types::IpWarmupResponse>> {
        let url = self.client.url("/ips/warmup", None);
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
     * Retrieve all IPs currently in warmup.
     *
     * This function performs a `GET` to the `/ips/warmup` endpoint.
     *
     * As opposed to `get_ips_warmup`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all of your IP addresses that are currently warming up.**
     */
    pub async fn get_all_ips_warmup(&self) -> ClientResult<Vec<crate::types::IpWarmupResponse>> {
        let url = self.client.url("/ips/warmup", None);
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
     * Start warming up an IP address.
     *
     * This function performs a `POST` to the `/ips/warmup` endpoint.
     *
     * **This endpoint allows you to put an IP address into warmup mode.**
     */
    pub async fn post_ips_warmup(
        &self,
        body: &crate::types::PostIpsWarmupRequest,
    ) -> ClientResult<Vec<crate::types::IpWarmupResponse>> {
        let url = self.client.url("/ips/warmup", None);
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
     * Retrieve the warmup status for a specific IP address.
     *
     * This function performs a `GET` to the `/ips/warmup/{ip_address}` endpoint.
     *
     * **This endpoint allows you to retrieve the warmup status for a specific IP address.**
     *
     * You can retrieve all of your warming IPs using the "Retrieve all IPs currently in warmup" endpoint.
     */
    pub async fn get_ips_warmup_ip_address(
        &self,
        ip_address: &str,
    ) -> ClientResult<Vec<crate::types::IpWarmupResponse>> {
        let url = self.client.url(
            &format!(
                "/ips/warmup/{}",
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
    /**
     * Retrieve the warmup status for a specific IP address.
     *
     * This function performs a `GET` to the `/ips/warmup/{ip_address}` endpoint.
     *
     * As opposed to `get_ips_warmup_ip_address`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve the warmup status for a specific IP address.**
     *
     * You can retrieve all of your warming IPs using the "Retrieve all IPs currently in warmup" endpoint.
     */
    pub async fn get_all_ips_warmup_ip_address(
        &self,
        ip_address: &str,
    ) -> ClientResult<Vec<crate::types::IpWarmupResponse>> {
        let url = self.client.url(
            &format!(
                "/ips/warmup/{}",
                crate::progenitor_support::encode_path(ip_address),
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
     * Stop warming up an IP address.
     *
     * This function performs a `DELETE` to the `/ips/warmup/{ip_address}` endpoint.
     *
     * **This endpoint allows you to remove an IP address from warmup mode.**
     *
     * Your request will return a 204 status code if the specified IP was successfully removed from warmup mode. To retrieve details of the IP’s warmup status *before* removing it from warmup mode, call the  "Retrieve the warmpup status for a specific IP address" endpoint.
     */
    pub async fn delete_ips_warmup_ip_address(
        &self,
        ip_address: &str,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url(
            &format!(
                "/ips/warmup/{}",
                crate::progenitor_support::encode_path(ip_address),
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

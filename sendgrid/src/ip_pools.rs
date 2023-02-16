use anyhow::Result;

use crate::Client;

pub struct IpPools {
    pub client: Client,
}

impl IpPools {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        IpPools { client }
    }

    /**
     * Retrieve all IP pools.
     *
     * This function performs a `GET` to the `/ips/pools` endpoint.
     *
     * **This endpoint allows you to get all of your IP pools.**
     */
    pub async fn get_ips_pools(&self) -> Result<Vec<crate::types::IpPoolsPoolResp>> {
        let url = "/ips/pools".to_string();
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * Retrieve all IP pools.
     *
     * This function performs a `GET` to the `/ips/pools` endpoint.
     *
     * As opposed to `get_ips_pools`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to get all of your IP pools.**
     */
    pub async fn get_all_ips_pools(&self) -> Result<Vec<crate::types::IpPoolsPoolResp>> {
        let url = "/ips/pools".to_string();
        self.client.get_all_pages(&url, None).await
    }
    /**
     * Create an IP pool.
     *
     * This function performs a `POST` to the `/ips/pools` endpoint.
     *
     * **This endpoint allows you to create an IP pool.**
     *
     * Before you can create an IP pool, you need to activate the IP in your SendGrid account:
     *
     * 1. Log into your SendGrid account.  
     * 1. Navigate to **Settings** and then select **IP Addresses**.  
     * 1. Find the IP address you want to activate and then click **Edit**.  
     * 1. Check **Allow my account to send mail using this IP address**.
     * 1. Click **Save**.
     */
    pub async fn post_ips_pool(
        &self,
        body: &crate::types::IpPool,
    ) -> Result<crate::types::IpPoolsPoolResp> {
        let url = "/ips/pools".to_string();
        let url = self.client.url(&url, None);
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
    /**
     * Add an IP address to a pool.
     *
     * This function performs a `POST` to the `/ips/pools/{pool_name}/ips` endpoint.
     *
     * **This endpoint allows you to add an IP address to an IP pool.**
     *
     * You can add the same IP address to multiple pools. It may take up to 60 seconds for your IP address to be added to a pool after your request is made.
     *
     * Before you can add an IP to a pool, you need to activate it in your SendGrid account:
     *
     * 1. Log into your SendGrid account.  
     * 1. Navigate to **Settings** and then select **IP Addresses**.  
     * 1. Find the IP address you want to activate and then click **Edit**.  
     * 1. Check **Allow my account to send mail using this IP address**.
     * 1. Click **Save**.
     *
     * You can retrieve all of your available IP addresses from the "Retrieve all IP addresses" endpoint.
     */
    pub async fn post_ips_pools_pool_name_ip(
        &self,
        pool_name: &str,
        body: &crate::types::PostIpsWarmupRequest,
    ) -> Result<crate::types::GetIpsAssignedResponse> {
        let url = format!(
            "/ips/pools/{}/ips",
            crate::progenitor_support::encode_path(&pool_name.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
    /**
     * Retrieve all the IPs in a specified pool.
     *
     * This function performs a `GET` to the `/ips/pools/{pool_name}` endpoint.
     *
     * **This endpoint allows you to get all of the IP addresses that are in a specific IP pool.**
     */
    pub async fn get_ips_pools_pool_name(
        &self,
        pool_name: &str,
    ) -> Result<crate::types::GetIpsPoolsPoolNameResponse> {
        let url = format!(
            "/ips/pools/{}",
            crate::progenitor_support::encode_path(&pool_name.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * Rename an IP pool.
     *
     * This function performs a `PUT` to the `/ips/pools/{pool_name}` endpoint.
     *
     * **This endpoint allows you to update the name of an IP pool.**
     */
    pub async fn put_ips_pools_pool_name(
        &self,
        pool_name: &str,
        body: &crate::types::PutIpsPoolsPoolNameRequest,
    ) -> Result<crate::types::IpPoolsPoolResp> {
        let url = format!(
            "/ips/pools/{}",
            crate::progenitor_support::encode_path(&pool_name.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
    /**
     * Delete an IP pool.
     *
     * This function performs a `DELETE` to the `/ips/pools/{pool_name}` endpoint.
     *
     * **This endpoint allows you to delete an IP pool.**
     */
    pub async fn delete_ips_pools_pool_name(&self, pool_name: &str) -> Result<crate::types::Help> {
        let url = format!(
            "/ips/pools/{}",
            crate::progenitor_support::encode_path(&pool_name.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client.delete(&url, None).await
    }
    /**
     * Remove an IP address from a pool.
     *
     * This function performs a `DELETE` to the `/ips/pools/{pool_name}/ips/{ip}` endpoint.
     *
     * **This endpoint allows you to remove an IP address from an IP pool.**
     */
    pub async fn delete_ips_pools_pool_name_ip(
        &self,
        pool_name: &str,
        ip: &str,
    ) -> Result<crate::types::Help> {
        let url = format!(
            "/ips/pools/{}/ips/{}",
            crate::progenitor_support::encode_path(&pool_name.to_string()),
            crate::progenitor_support::encode_path(&ip.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client.delete(&url, None).await
    }
}

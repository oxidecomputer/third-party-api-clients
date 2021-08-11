use anyhow::Result;

use crate::Client;

pub struct CustomId {
    client: Client,
}

impl CustomId {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        CustomId {
            client,
        }
    }

    /**
* GET the Custom ID provider linked to the current OAuth token.
*
* This function performs a `GET` to the `/custom-id-provider` endpoint.
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
*/
pub async fn get_provider(
&self,
authorization: &str,
) -> Result<crate::types::GetProviderResponse> {
let url =
"/custom-id-provider".to_string();
self.client.get(&url, None).await
}

/**
* Create a Custom ID provider.
*
* This function performs a `POST` to the `/custom-id-provider` endpoint.
*
* 
*
* **Parameters:**
*
* * `authorization_bearer_111111111111: &str` -- OAuth Access token.
*/
pub async fn postprovider(
&self,
authorization_bearer_111111111111: &str,
) -> Result<crate::types::PostproviderResponse> {
let url =
"/custom-id-provider".to_string();
self.client.post(&url, None).await
}

/**
* .
*
* This function performs a `POST` to the `/custom-id-provider/application-link` endpoint.
*
* Register an access token with a custom ID provider
*/
pub async fn post_provider_application_link(
&self,
body: &crate::types::GetProviderResponse
) -> Result<()> {
let url =
"/custom-id-provider/application-link".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Convert custom id to ramp id.
*
* This function performs a `GET` to the `/custom-id-provider/<entity-type>/<custom-id>/ramp-id` endpoint.
*
* 
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
*/
pub async fn get_entity_type_ramp(
&self,
authorization: &str,
) -> Result<crate::types::GetEntityTypeRampResponse> {
let url =
"/custom-id-provider/<entity-type>/<custom-id>/ramp-id".to_string();
self.client.get(&url, None).await
}

/**
* Convert ramp id to custom id.
*
* This function performs a `GET` to the `/custom-id-provider/<entity-type>/<ramp-id>/custom-id` endpoint.
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
*/
pub async fn get_entity_type_ramp_custom(
&self,
authorization: &str,
) -> Result<crate::types::GetEntityTypeRampCustomResponse> {
let url =
"/custom-id-provider/<entity-type>/<ramp-id>/custom-id".to_string();
self.client.get(&url, None).await
}

/**
* Create custom id link.
*
* This function performs a `POST` to the `/custom-id-provider/<entity-type>/custom-id-link` endpoint.
*
* Create a mapping between custom\_id and ramp\_id under the namespace specified by entity\_type.
*/
pub async fn post_provider_entity_type_link(
&self,
body: &crate::types::PostProviderEntityTypeLinkRequest
) -> Result<()> {
let url =
"/custom-id-provider/<entity-type>/custom-id-link".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}
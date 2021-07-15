use anyhow::Result;

use crate::Client;

pub struct CurrentUser {
    client: Client,
}

impl CurrentUser {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        CurrentUser {
            client,
        }
    }

    /**
* Get the current user.
*
* This function performs a `GET` to the `/v1/me` endpoint.
*
* Returns information pertaining to the user associated with the provided access token.
*/
pub async fn get_v_1_me(
&self,
) -> Result<crate::types::CurrentUser> {
let url =
"/v1/me".to_string();
self.client.get(&url).await
}


}
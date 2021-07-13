use anyhow::Result;

use crate::Client;

pub struct Interactions {
    client: Client,
}

impl Interactions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Interactions { client }
    }

    /**
     * Remove interaction restrictions from your public repositories.
     *
     * This function performs a `DELETE` to the `/user/interaction-limits` endpoint.
     *
     * Removes any interaction restrictions from your public repositories.
     *
     * FROM: <https://docs.github.com/rest/reference/interactions#remove-interaction-restrictions-from-your-public-repositories>
     */
    pub async fn remove_restrictions_for_authenticated_user(&self) -> Result<()> {
        let url = "/user/interaction-limits".to_string();
        self.client.delete(&url, None).await
    }
}

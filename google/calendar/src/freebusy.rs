use anyhow::Result;

use crate::Client;

pub struct Freebusy {
    pub client: Client,
}

impl Freebusy {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Freebusy { client }
    }

    /**
    * This function performs a `POST` to the `/freeBusy` endpoint.
    *
    * Returns free/busy information for a set of calendars.
    */
    pub async fn query(
        &self,
        body: &crate::types::FreeBusyRequest,
    ) -> Result<crate::types::FreeBusyResponse> {
        let url = "/freeBusy".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

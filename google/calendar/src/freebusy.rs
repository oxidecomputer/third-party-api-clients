use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<crate::Response<crate::types::FreeBusyResponse>> {
        let url = self.client.url("/freeBusy", None);
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
}

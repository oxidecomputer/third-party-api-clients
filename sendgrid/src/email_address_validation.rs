use crate::Client;
use crate::ClientResult;

pub struct EmailAddressValidation {
    pub client: Client,
}

impl EmailAddressValidation {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EmailAddressValidation { client }
    }

    /**
     * Validate an email.
     *
     * This function performs a `POST` to the `/validations/email` endpoint.
     *
     * **This endpoint allows you to validate an email address.**
     */
    pub async fn post_validations_email(
        &self,
        body: &crate::types::PostValidationsEmailRequest,
    ) -> ClientResult<crate::types::PostValidationsEmailResponse> {
        let url = self.client.url("/validations/email", None);
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

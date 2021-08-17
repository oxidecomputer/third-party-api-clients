use anyhow::Result;

use crate::Client;

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
    ) -> Result<crate::types::PostValidationsEmailResponse> {
        let url = "/validations/email".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}

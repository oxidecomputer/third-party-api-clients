use anyhow::Result;

use crate::Client;

pub struct CustomerJourneys {
    pub client: Client,
}

impl CustomerJourneys {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CustomerJourneys { client }
    }

    /**
    * Customer Journeys API trigger for a contact.
    *
    * This function performs a `POST` to the `/customer-journeys/journeys/{journey_id}/steps/{step_id}/actions/trigger` endpoint.
    *
    * A step trigger in a Customer Journey. To use it, create a starting point or step from the Customer Journey builder in the app using the Customer Journeys API condition. We’ll provide a url during the process that includes the {journey_id} and {step_id}. You’ll then be able to use this endpoint to trigger the condition for the posted contact.
    *
    * **Parameters:**
    *
    * * `journey_id: i64` -- The display order for interests.
    * * `step_id: i64` -- The display order for interests.
    */
    pub async fn post_steps_actions_trigger(
        &self,
        journey_id: i64,
        step_id: i64,
        body: &crate::types::SubscriberInAutomationQueue,
    ) -> Result<()> {
        let url = format!(
            "/customer-journeys/journeys/{}/steps/{}/actions/trigger",
            crate::progenitor_support::encode_path(&journey_id.to_string()),
            crate::progenitor_support::encode_path(&step_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

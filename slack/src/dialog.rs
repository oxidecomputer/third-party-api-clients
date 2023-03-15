use crate::Client;
use crate::ClientResult;

pub struct Dialog {
    pub client: Client,
}

impl Dialog {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Dialog { client }
    }

    /**
     * This function performs a `GET` to the `/dialog.open` endpoint.
     *
     * Open a dialog with a user
     *
     * FROM: <https://api.slack.com/methods/dialog.open>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `dialog: &str` -- The dialog definition. This must be a JSON-encoded string.
     * * `trigger_id: &str` -- Exchange a trigger to post to the user.
     */
    pub async fn open(
        &self,
        dialog: &str,
        trigger_id: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !dialog.is_empty() {
            query_args.push(("dialog".to_string(), dialog.to_string()));
        }
        if !trigger_id.is_empty() {
            query_args.push(("trigger_id".to_string(), trigger_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/dialog.open?{}", query_), None);
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
}

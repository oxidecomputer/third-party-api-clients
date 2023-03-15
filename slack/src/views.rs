use crate::Client;
use crate::ClientResult;

pub struct Views {
    pub client: Client,
}

impl Views {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Views { client }
    }

    /**
     * This function performs a `GET` to the `/views.open` endpoint.
     *
     * Open a view for a user.
     *
     * FROM: <https://api.slack.com/methods/views.open>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `trigger_id: &str` -- Exchange a trigger to post to the user.
     * * `view: &str` -- A [view payload](/reference/surfaces/views). This must be a JSON-encoded string.
     */
    pub async fn open(
        &self,
        trigger_id: &str,
        view: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !trigger_id.is_empty() {
            query_args.push(("trigger_id".to_string(), trigger_id.to_string()));
        }
        if !view.is_empty() {
            query_args.push(("view".to_string(), view.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/views.open?{}", query_), None);
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
    /**
     * This function performs a `GET` to the `/views.publish` endpoint.
     *
     * Publish a static view for a User.
     *
     * FROM: <https://api.slack.com/methods/views.publish>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `user_id: &str` -- `id` of the user you want publish a view to.
     * * `view: &str` -- A [view payload](/reference/surfaces/views). This must be a JSON-encoded string.
     * * `hash: &str` -- A string that represents view state to protect against possible race conditions.
     */
    pub async fn publish(
        &self,
        user_id: &str,
        view: &str,
        hash: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !hash.is_empty() {
            query_args.push(("hash".to_string(), hash.to_string()));
        }
        if !user_id.is_empty() {
            query_args.push(("user_id".to_string(), user_id.to_string()));
        }
        if !view.is_empty() {
            query_args.push(("view".to_string(), view.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/views.publish?{}", query_), None);
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
    /**
     * This function performs a `GET` to the `/views.push` endpoint.
     *
     * Push a view onto the stack of a root view.
     *
     * FROM: <https://api.slack.com/methods/views.push>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `trigger_id: &str` -- Exchange a trigger to post to the user.
     * * `view: &str` -- A [view payload](/reference/surfaces/views). This must be a JSON-encoded string.
     */
    pub async fn push(
        &self,
        trigger_id: &str,
        view: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !trigger_id.is_empty() {
            query_args.push(("trigger_id".to_string(), trigger_id.to_string()));
        }
        if !view.is_empty() {
            query_args.push(("view".to_string(), view.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/views.push?{}", query_), None);
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
    /**
     * This function performs a `GET` to the `/views.update` endpoint.
     *
     * Update an existing view.
     *
     * FROM: <https://api.slack.com/methods/views.update>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `view_id: &str` -- A unique identifier of the view to be updated. Either `view_id` or `external_id` is required.
     * * `external_id: &str` -- A unique identifier of the view set by the developer. Must be unique for all views on a team. Max length of 255 characters. Either `view_id` or `external_id` is required.
     * * `view: &str` -- A [view object](/reference/surfaces/views). This must be a JSON-encoded string.
     * * `hash: &str` -- A string that represents view state to protect against possible race conditions.
     */
    pub async fn update(
        &self,
        view_id: &str,
        external_id: &str,
        view: &str,
        hash: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !external_id.is_empty() {
            query_args.push(("external_id".to_string(), external_id.to_string()));
        }
        if !hash.is_empty() {
            query_args.push(("hash".to_string(), hash.to_string()));
        }
        if !view.is_empty() {
            query_args.push(("view".to_string(), view.to_string()));
        }
        if !view_id.is_empty() {
            query_args.push(("view_id".to_string(), view_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/views.update?{}", query_), None);
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

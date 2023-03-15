use crate::Client;
use crate::ClientResult;

pub struct Query {
    pub client: Client,
}

impl Query {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Query { client }
    }

    /**
     * Filter all messages.
     *
     * This function performs a `GET` to the `/messages` endpoint.
     *
     * This is **BETA** functionality. You may not have access, and we reserve the right to change functionality without notice.
     *
     * Filter all messages to search your Email Activity. All queries need to be [URL encoded](https://meyerweb.com/eric/tools/dencoder/), and have this format:
     *
     * `query={query_type}="{query_content}"`
     *
     * encoded, this would look like this:
     *
     * `query=type%3D%22query_content%22`
     *
     * for example:
     *
     * Filter by a specific email - `query=to_email%3D%22example%40example.com%22`
     *
     * Filter by subject line - `query=subject%3d%22A%20Great%20Subject%22`
     *
     * **Full list of basic query types and examples:**
     *
     *
     * | **Filter query**    | **Unencoded Example** (put this one into the try it out query - it'll automatically encode it for you) | **Encoded Example** (use this one in your code)                        |
     * |-----------------|----------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------|
     * | msg_id          | msg_id=“filter0307p1las1-16816-5A023E36-1.0”                                                                               | msg_id%3D%22filter0307p1las1-16816-5A023E36-1.0%22           |
     * | from_email      | from_email=“testing@sendgrid.net”                                                                                          | from_email%3D%22testing%40sendgrid.net%22                    |
     * | subject         | subject="This is a subject test"                                                                                      | subject%22This%20is%20a%20subject%20test%22                  |
     * | to_email        | to_email="example@example.com"                                                                                       | to_email%3D%22example%40example.com%22                       |
     * | status          |                                                                                                                            | status%22processed%22                                        |
     * | template_id     |                                                                                                                            |                                                                    |
     * | asm_group_id    |                                                                                                                            |                                                                    |
     * | api_key_id      |                                                                                                                            |                                                                    |
     * | events          | status="processed"                                                                                                   | status%3D%22processed%22                                     |
     * | originating_ip  |                                                                                                                            |                                                                    |
     * | categories      |                                                                                                                            |                                                                    |
     * | unique_args     |                                                                                                                            |                                                                    |
     * | outbound_ip     |                                                                                                                            |                                                                    |
     * | last_event_time | last_event_time=“2017-11-07T23:13:58Z”                                                                               | last_event_time%3D%E2%80%9C2017-11-07T23%3A13%3A58Z%E2%80%9D |
     * | clicks          | clicks="0"                                                                                                           | clicks%3D%220%22                                             |
     *
     * For information about building compound queries, and for the full query language functionality, see the [query language reference](https://docs.google.com/a/sendgrid.com/document/d/1fWoKTFNfg5UUsB6t9KuIcSo9CetKF_T0bGfWJ_gdPCs/edit?usp=sharing).
     *
     * Coming soon, example compound queries: limit + to email + date
     *
     * **Parameters:**
     *
     * * `query: &str` -- The license key provided with your New Relic account.
     * * `limit: f64` -- The number of messages returned. This parameter must be greater than 0 and less than or equal to 1000.
     * * `x_query_id: &str` -- The license key provided with your New Relic account.
     * * `x_cursor: &str` -- The license key provided with your New Relic account.
     * * `authorization: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_messages(
        &self,
        query: &str,
        limit: f64,
    ) -> ClientResult<crate::types::GetMessagesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/messages?{}", query_), None);
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
     * Filter messages by message ID.
     *
     * This function performs a `GET` to the `/messages/{msg_id}` endpoint.
     *
     * This is BETA functionality. You may not have access, and we reserve the right to change functionality without notice.
     *
     * Get all of the details about the specified message.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_messages_msg(&self, msg_id: &str) -> ClientResult<crate::types::Message> {
        let url = self.client.url(
            &format!(
                "/messages/{}",
                crate::progenitor_support::encode_path(msg_id),
            ),
            None,
        );
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

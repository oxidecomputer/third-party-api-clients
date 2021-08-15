use anyhow::Result;

use crate::Client;

pub struct About {
    client: Client,
}

impl About {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        About { client }
    }

    /**
     * This function performs a `GET` to the `/about` endpoint.
     *
     * Gets information about the user, the user's Drive, and system capabilities.
     */
    pub async fn drive_get(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
    ) -> Result<crate::types::About> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/about?{}", query);

        self.client.get(&url, None).await
    }
}

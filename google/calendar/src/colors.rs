use anyhow::Result;

use crate::Client;

pub struct Colors {
    client: Client,
}

impl Colors {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Colors { client }
    }

    /**
     * This function performs a `GET` to the `/colors` endpoint.
     *
     * Returns the color definitions for calendars and events.
     */
    pub async fn calendar_get(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        quota_user: &str,
        user_ip: &str,
    ) -> Result<crate::types::Colors> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/colors?{}", query_);

        self.client.get(&url, None).await
    }
}

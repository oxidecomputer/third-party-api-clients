use anyhow::Result;

use crate::Client;

pub struct Groups {
    pub client: Client,
}

impl Groups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Groups { client }
    }

    /**
    * This function performs a `GET` to the `/{groupUniqueId}` endpoint.
    *
    * Gets one resource by id.
    *
    * **Parameters:**
    *
    * * `group_unique_id: &str` -- Identifies whether members external to your organization can join the group. Possible values are:  
    *  - true: G Suite users external to your organization can become members of this group.
    *  - false: Users not belonging to the organization are not allowed to become members of this group.
    */
    pub async fn get(
        &self,
        alt: crate::types::Alt,
        group_unique_id: &str,
    ) -> Result<crate::types::Groups> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !alt.to_string().is_empty() {
            query_args.push(("alt".to_string(), alt.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/{}?{}",
            crate::progenitor_support::encode_path(&group_unique_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `PUT` to the `/{groupUniqueId}` endpoint.
    *
    * Updates an existing resource.
    *
    * **Parameters:**
    *
    * * `group_unique_id: &str` -- Identifies whether members external to your organization can join the group. Possible values are:  
    *  - true: G Suite users external to your organization can become members of this group.
    *  - false: Users not belonging to the organization are not allowed to become members of this group.
    */
    pub async fn update(
        &self,
        alt: crate::types::Alt,
        group_unique_id: &str,
        body: &crate::types::Groups,
    ) -> Result<crate::types::Groups> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !alt.to_string().is_empty() {
            query_args.push(("alt".to_string(), alt.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/{}?{}",
            crate::progenitor_support::encode_path(&group_unique_id.to_string()),
            query_
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * This function performs a `PATCH` to the `/{groupUniqueId}` endpoint.
    *
    * Updates an existing resource. This method supports patch semantics.
    *
    * **Parameters:**
    *
    * * `group_unique_id: &str` -- Identifies whether members external to your organization can join the group. Possible values are:  
    *  - true: G Suite users external to your organization can become members of this group.
    *  - false: Users not belonging to the organization are not allowed to become members of this group.
    */
    pub async fn patch(
        &self,
        alt: crate::types::Alt,
        group_unique_id: &str,
        body: &crate::types::Groups,
    ) -> Result<crate::types::Groups> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !alt.to_string().is_empty() {
            query_args.push(("alt".to_string(), alt.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/{}?{}",
            crate::progenitor_support::encode_path(&group_unique_id.to_string()),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

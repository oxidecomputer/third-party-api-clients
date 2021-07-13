use anyhow::Result;

use crate::Client;

pub struct Scim {
    client: Client,
}

impl Scim {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Scim { client }
    }

    /**
     * Update an attribute for a SCIM user.
     *
     * This function performs a `PATCH` to the `/scim/v2/organizations/{org}/Users/{scim_user_id}` endpoint.
     *
     * Allows you to change a provisioned user's individual attributes. To change a user's values, you must provide a specific `Operations` JSON format that contains at least one of the `add`, `remove`, or `replace` operations. For examples and more information on the SCIM operations format, see the [SCIM specification](https://tools.ietf.org/html/rfc7644#section-3.5.2).
     *
     * **Note:** Complicated SCIM `path` selectors that include filters are not supported. For example, a `path` selector defined as `"path": "emails[type eq \"work\"]"` will not work.
     *
     * **Warning:** If you set `active:false` using the `replace` operation (as shown in the JSON example below), it removes the user from the organization, deletes the external identity, and deletes the associated `:scim_user_id`.
     *
     * ```
     * {
     *   "Operations":[{
     *     "op":"replace",
     *     "value":{
     *       "active":false
     *     }
     *   }]
     * }
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/scim#update-an-attribute-for-a-scim-user>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `scim_user_id: &str` -- scim_user_id parameter.
     */
    pub async fn update_attribute_for_user(
        &self,
        org: &str,
        scim_user_id: &str,
        body: &crate::types::ScimUpdateAttributeUserRequest,
    ) -> Result<crate::types::ScimUser> {
        let url = format!(
            "/scim/v2/organizations/{}/Users/{}",
            crate::progenitor_support::encode_path(&org.to_string()),
            crate::progenitor_support::encode_path(&scim_user_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}

use crate::Client;
use crate::ClientResult;

pub struct UserSignatures {
    pub client: Client,
}

impl UserSignatures {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UserSignatures { client }
    }

    /**
     * Retrieves a list of signature definitions for a user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/signatures` endpoint.
     *
     * This method retrieves the signature definitions for the user that you specify.
     *
     * The `userId` parameter specified in the endpoint must match the authenticated user's user ID, and the user must be a member of the account.
     *
     * The `signatureId` parameter accepts a signature ID or a signature name. DocuSign recommends you use signature ID (`signatureId`), since some names contain characters that do not properly encode into a URL. If you use the user name, it is likely that the name includes spaces. In that case, URL encode the name before using it in the endpoint.
     *
     * For example, encode "Bob Smith" as "Bob%20Smith".
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `stamp_type: &str` -- The type of stamps to return. Valid values are:
     *   
     *   - `signature`: Returns information about signature images only. This is the default value.
     *   - `stamp`: Returns information about eHanko and custom stamps only.
     *   - null.
     */
    pub async fn get(
        &self,
        account_id: &str,
        user_id: &str,
        stamp_type: &str,
    ) -> ClientResult<crate::types::UserSignaturesInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !stamp_type.is_empty() {
            query_args.push(("stamp_type".to_string(), stamp_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/signatures?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                query_
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
    /**
     * Adds/updates a user signature.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/users/{userId}/signatures` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn put_signature(
        &self,
        account_id: &str,
        user_id: &str,
        body: &crate::types::UserSignaturesInformation,
    ) -> ClientResult<crate::types::UserSignaturesInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/signatures",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Adds user Signature and initials images to a Signature.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/users/{userId}/signatures` endpoint.
     *
     * Adds a user signature image and/or user initials image to the specified user.
     *
     * The userId property specified in the endpoint must match the authenticated user's `userId` and the user must be a member of the account.
     *
     * The rules and processes associated with this are:
     *
     * * If `Content-Type` is set to `application/json`, then the default behavior for creating a default signature image, based on the name and a DocuSign font, is used.
     * * If `Content-Type` is set to `multipart/form-data`, then the request must contain a first part with the user signature information, followed by parts that contain the images.
     *
     * For each Image part, the Content-Disposition header has a "filename" value that is used to map to the `signatureName` and/or `signatureInitials` properties in the JSON to the image.
     *
     * For example:
     * `Content-Disposition: file; filename="Ron Test20121127083900"`
     *
     * If no matching image (by filename value) is found, then the image is not set. One, both, or neither of the signature and initials images can be set with this call.
     *
     * The Content-Transfer-Encoding: base64 header, set in the header for the part containing the image, can be set to indicate that the images are formatted as base64 instead of as binary.
     *
     * If successful, 200-OK is returned, and a JSON structure containing the signature information is provided, note that the signatureId can change with each API POST, PUT, or DELETE since the changes to the signature structure cause the current signature to be closed, and a new signature record to be created.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn post(
        &self,
        account_id: &str,
        user_id: &str,
        body: &crate::types::UserSignaturesInformation,
    ) -> ClientResult<crate::types::UserSignaturesInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/signatures",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Gets the user signature information for the specified user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/signatures/{signatureId}` endpoint.
     *
     * Retrieves the structure of a single signature with a known signature name.
     *
     * The userId specified in the endpoint must match the authenticated user's user ID and the user must be a member of the account.
     *
     * The `signatureId` parameter accepts a signature ID or a signature name. DocuSign recommends you use signature ID (`signatureId`), since some names contain characters that do not properly encode into a URL. If you use the user name, it is likely that the name includes spaces. In that case, URL encode the name before using it in the endpoint.
     *
     * For example encode "Bob Smith" as "Bob%20Smith".
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn get_signature(
        &self,
        account_id: &str,
        signature_id: &str,
        user_id: &str,
    ) -> ClientResult<crate::types::UserSignature> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/signatures/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(signature_id),
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
    /**
     * Updates the user signature for a specified user.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/users/{userId}/signatures/{signatureId}` endpoint.
     *
     * Creates, or updates, the signature font and initials for the specified user. When creating a signature, you use this resource to create the signature name and then add the signature and initials images into the signature.
     *
     * **Note**: This will also create a default signature for the user when one does not exist.
     *
     * The userId property specified in the endpoint must match the authenticated user's user ID and the user must be a member of the account.
     *
     * The `signatureId` parameter accepts a signature ID. DocuSign recommends you use signature ID (`signatureId`), since some names contain characters that do not properly encode into a URL. If you use the user name, it is likely that the name includes spaces. In that case, URL encode the name before using it in the endpoint.
     *
     * For example encode "Bob Smith" as "Bob%20Smith".
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `close_existing_signature: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put_signature_user_signatures(
        &self,
        account_id: &str,
        signature_id: &str,
        user_id: &str,
        close_existing_signature: &str,
        body: &crate::types::UserSignatureDefinition,
    ) -> ClientResult<crate::types::UserSignature> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !close_existing_signature.is_empty() {
            query_args.push((
                "close_existing_signature".to_string(),
                close_existing_signature.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/signatures/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(signature_id),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Removes removes signature information for the specified user.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/users/{userId}/signatures/{signatureId}` endpoint.
     *
     * Removes the signature information for the user.
     *
     * The userId parameter specified in the endpoint must match the authenticated user's user ID and the user must be a member of the account.
     *
     * The `signatureId` accepts a signature ID or a signature name. DocuSign recommends you use signature ID (`signatureId`), since some names contain characters that do not properly encode into a URL. If you use the user name, it is likely that the name includes spaces. In that case, URL encode the name before using it in the endpoint.
     *
     * For example encode "Bob Smith" as "Bob%20Smith".
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn delete_signature(
        &self,
        account_id: &str,
        signature_id: &str,
        user_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/signatures/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(signature_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the user initials image or the  user signature image for the specified user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/signatures/{signatureId}/{imageType}` endpoint.
     *
     * Retrieves the specified initials image or signature image for the specified user. The image is returned in the same format in which it was uploaded. In the request you can specify if the chrome (the added line and identifier around the initial image) is returned with the image.
     *
     * The userId property specified in the endpoint must match the authenticated user's user ID and the user must be a member of the account.
     *
     * The `signatureId` parameter accepts a signature ID or a signature name. DocuSign recommends you use signature ID (`signatureId`), since some names contain characters that do not properly encode into a URL. If you use the user name, it is likely that the name includes spaces. In that case, URL encode the name before using it in the endpoint.
     *
     * For example encode "Bob Smith" as "Bob%20Smith".
     *
     * **Note**: Older envelopes might only have chromed images. If getting the non-chromed image fails, try getting the chromed image.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `image_type: &str` -- Specificies the type of image. Valid values are:
     *   
     *   - `signature_image`
     *   - `initials_image`.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `include_chrome: &str` -- When **true**, the chrome (or frame containing the added line and identifier) is included with the signature image.
     */
    pub async fn get_signature_image(
        &self,
        account_id: &str,
        image_type: &str,
        signature_id: &str,
        user_id: &str,
        include_chrome: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_chrome.is_empty() {
            query_args.push(("include_chrome".to_string(), include_chrome.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/signatures/{}/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(signature_id),
                crate::progenitor_support::encode_path(image_type),
                query_
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
    /**
     * Updates the user signature image or user initials image for the specified user.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/users/{userId}/signatures/{signatureId}/{imageType}` endpoint.
     *
     * Updates the user signature image or user initials image for the specified user. The supported image formats for this file are: gif, png, jpeg, and bmp. The file must be less than 200K.
     *
     * The userId property specified in the endpoint must match the authenticated user's user ID and the user must be a member of the account.
     *
     * The `signatureId` parameter accepts a signature ID or a signature name. DocuSign recommends you use signature ID (`signatureId`), since some names contain characters that do not properly encode into a URL. If you use the user name, it is likely that the name includes spaces. In that case, URL encode the name before using it in the endpoint.
     *
     * For example encode "Bob Smith" as "Bob%20Smith".
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `image_type: &str` -- Specificies the type of image. Valid values are:
     *   
     *   - `signature_image`
     *   - `initials_image`.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `transparent_png: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put_signature_image(
        &self,
        account_id: &str,
        image_type: &str,
        signature_id: &str,
        user_id: &str,
        transparent_png: &str,
    ) -> ClientResult<crate::types::UserSignature> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !transparent_png.is_empty() {
            query_args.push(("transparent_png".to_string(), transparent_png.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/signatures/{}/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(signature_id),
                crate::progenitor_support::encode_path(image_type),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes the user initials image or the  user signature image for the specified user.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/users/{userId}/signatures/{signatureId}/{imageType}` endpoint.
     *
     * Deletes the specified initials image or signature image for the specified user.
     *
     * The function deletes one or the other of the image types, to delete both the initials image and signature image you must call the endpoint twice.
     *
     * The userId parameter specified in the endpoint must match the authenticated user's user ID and the user must be a member of the account.
     *
     * The `signatureId` parameter accepts a signature ID or a signature name. DocuSign recommends you use signature ID (`signatureId`), since some names contain characters that do not properly encode into a URL. If you use the user name, it is likely that the name includes spaces. In that case, URL encode the name before using it in the endpoint.
     *
     * For example encode "Bob Smith" as "Bob%20Smith".
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `image_type: &str` -- Specificies the type of image. Valid values are:
     *   
     *   - `signature_image`
     *   - `initials_image`.
     * * `signature_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn delete_signature_image(
        &self,
        account_id: &str,
        image_type: &str,
        signature_id: &str,
        user_id: &str,
    ) -> ClientResult<crate::types::UserSignature> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/signatures/{}/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(signature_id),
                crate::progenitor_support::encode_path(image_type),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}

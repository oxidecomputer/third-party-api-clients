use anyhow::Result;

use crate::Client;

pub struct Templates {
    client: Client,
}

impl Templates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Templates {
            client,
        }
    }

    /**
* Gets template definitions.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates` endpoint.
*
* Retrieves the list of templates for the specified account. The request can be limited to a specific folder.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `created_from_date: &str` -- The billing period end date in UTC timedate format.
* * `created_to_date: &str` -- The billing period end date in UTC timedate format.
* * `folder_ids: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `folder_types: &str` -- The type of folder to return templates for. Possible values are:
*   
*   - `templates`: Templates in the **My Templates** folder.
*     Templates in the **Shared Templates**  and **All Template** folders (if the request id from and Admin) are excluded.
*   - `templates_root`: Templates in the root level of the **My Templates** folder, but not in an actual folder. Note that the **My Templates** folder is not a real folder.
*   - `recylebin`: Templates that have been deleted.
*   .
* * `from_date: &str` -- Start of the search date range. Only returns templates created on or after this date/time. If no value is specified, there is no limit on the earliest date created.
* * `include: &str` -- A comma-separated list
*   of additional template attributes
*   to include in the response.
*   Valid values are:
*   
*   - `powerforms`: Includes details about the PowerForms associated with the templates.
*   - `documents`: Includes information about template documents.
*   - `folders`: Includes information about the folder that holds the template.
*   - `favorite_template_status`: Includes the template `favoritedByMe` property. **Note**: You can mark a template as a favorite only in eSignature v2.1.
*   - `advanced_templates`: Includes information about advanced templates.
*   - `recipients`: Includes information about template recipients.
*   - `custom_fields`: Includes information about template custom fields.
*   - `notifications`: Includes information about the notification settings for templates.
* * `is_deleted_template_only: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `is_download: &str` -- When set to **true**, downloads the templates listed in `template_ids` as a collection of JSON definitions in a single zip file.
*   
*   The `Content-Disposition` header is set in the response. The value of the header provides the filename of the file.
*   
*   The default is **false**.
*   
*   **Note**: This parameter only works when you specify a list of templates in the `template_ids` parameter.
* * `modified_from_date: &str` -- The billing period end date in UTC timedate format.
* * `modified_to_date: &str` -- The billing period end date in UTC timedate format.
* * `order: &str` -- Specifies the sort order of the search results.
*   Valid values are:
*   
*   - `asc`: Ascending (A to Z)
*   - `desc`: Descending (Z to A).
* * `order_by: &str` -- Specifies how the search results are listed.
*   Valid values are:
*   
*   - `name`: template name
*   - `modified`: date/time template was last modified
*   - `used`: date/time the template was last used.
* * `search_fields: &str` -- A comma-separated list of additional template properties to search.
*   
*   
*   - `sender`: Include sender name and email in the search.
*   - `recipients`: Include recipient names and emails in the search.
*   - `envelope`: Not used in template searches.
*   .
* * `search_text: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `shared_by_me: &str` -- If **true**, the response only includes templates shared by the user. If false, the response only returns template not shared by the user. If not specified, the response is not affected.
* * `start_position: &str` -- The starting zero-based index position for the first template to show in the response. This value must be greater than or equal to `0` (zero).
* * `template_ids: &str` -- A comma-separated list of template ids to download. This value is valid only when `is_download` is **true**.
* * `to_date: &str` -- The end of a search date range in UTC DateTime format. When you use this parameter, only templates created up to this date and time are returned.
*   
*   **Note**: If this property is null, the value defaults to the current date.
* * `used_from_date: &str` -- Start of the search date range. Only returns templates used or edited on or after this date/time. If no value is specified, there is no limit on the earliest date used.
* * `used_to_date: &str` -- End of the search date range. Only returns templates used or edited up to this date/time. If no value is provided, this defaults to the current date.
* * `user_filter: &str` -- Filters the templates in the response. Valid values are:
*   
*   - `owned_by_me`: Results include only templates owned by the user.
*   - `shared_with_me`: Results include only templates owned by the user.
*   - `all`:  Results include all templates owned or shared with the user.
* * `user_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn get(
&self,
account_id: &str, count: &str, created_from_date: &str, created_to_date: &str, folder_ids: &str, folder_types: &str, from_date: &str, include: &str, is_deleted_template_only: &str, is_download: &str, modified_from_date: &str, modified_to_date: &str, order: &str, order_by: &str, search_fields: &str, search_text: &str, shared_by_me: &str, start_position: &str, template_ids: &str, to_date: &str, used_from_date: &str, used_to_date: &str, user_filter: &str, user_id: &str,
) -> Result<crate::types::EnvelopeTemplateResults> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !count.is_empty() { query_args.push(format!("count={}", count)); }
if !created_from_date.is_empty() { query_args.push(format!("created_from_date={}", created_from_date)); }
if !created_to_date.is_empty() { query_args.push(format!("created_to_date={}", created_to_date)); }
if !folder_ids.is_empty() { query_args.push(format!("folder_ids={}", folder_ids)); }
if !folder_types.is_empty() { query_args.push(format!("folder_types={}", folder_types)); }
if !from_date.is_empty() { query_args.push(format!("from_date={}", from_date)); }
if !include.is_empty() { query_args.push(format!("include={}", include)); }
if !is_deleted_template_only.is_empty() { query_args.push(format!("is_deleted_template_only={}", is_deleted_template_only)); }
if !is_download.is_empty() { query_args.push(format!("is_download={}", is_download)); }
if !modified_from_date.is_empty() { query_args.push(format!("modified_from_date={}", modified_from_date)); }
if !modified_to_date.is_empty() { query_args.push(format!("modified_to_date={}", modified_to_date)); }
if !order.is_empty() { query_args.push(format!("order={}", order)); }
if !order_by.is_empty() { query_args.push(format!("order_by={}", order_by)); }
if !search_fields.is_empty() { query_args.push(format!("search_fields={}", search_fields)); }
if !search_text.is_empty() { query_args.push(format!("search_text={}", search_text)); }
if !shared_by_me.is_empty() { query_args.push(format!("shared_by_me={}", shared_by_me)); }
if !start_position.is_empty() { query_args.push(format!("start_position={}", start_position)); }
if !template_ids.is_empty() { query_args.push(format!("template_ids={}", template_ids)); }
if !to_date.is_empty() { query_args.push(format!("to_date={}", to_date)); }
if !used_from_date.is_empty() { query_args.push(format!("used_from_date={}", used_from_date)); }
if !used_to_date.is_empty() { query_args.push(format!("used_to_date={}", used_to_date)); }
if !user_filter.is_empty() { query_args.push(format!("user_filter={}", user_filter)); }
if !user_id.is_empty() { query_args.push(format!("user_id={}", user_id)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/v2.1/accounts/{}/templates?{}",
crate::progenitor_support::encode_path(&account_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* Creates one or more templates.
*
* This function performs a `POST` to the `/v2.1/accounts/{accountId}/templates` endpoint.
*
* Creates one or more template definitions, using a multipart request for each template.
* 
* Templates help streamline the sending process when you frequently send the same or similar documents, or send different documents to the same group of people.
* 
* When you create a template, you define placeholder roles. Rather than specifying a person, you specify a role that regularly participates in a transaction that uses the template. Then, when you create or send an envelope based on the template, you assign actual recipients to the template roles. The recipients automatically inherit all of the workflow that is defined for that role in the template, such as the tabs and routing information.
* 
* For code examples and more information, see [REST API Templates](https://developers.docusign.com/esign-rest-api/guides/features/templates#sending-from-a-template).
* 
* ## Template Email Subject Merge Fields
* 
* Placeholder roles have associated merge fields that personalize the email notification that DocuSign sends. For example, the template automatically personalizes the email message by adding placeholders for the recipient's name and email address within the email subject line, based on the recipient's role. When the sender adds the name and email information for the recipient and sends the envelope, the recipient information is automatically merged into the appropriate fields in the email subject line.
* 
* Both the sender and the recipients will see the information in the email subject line for any emails associated with the template. This provides an easy way for senders to organize their envelope emails without having to open an envelope to find out who the recipient is.
* 
* **Warning: If merging the recipient information into the subject line causes the subject line to exceed 100 characters, then any characters over the 100 character limit are not included in the subject line. For cases where you expect the recipient name or email to be long, you should consider placing the merge field at the start of the email subject.**
* 
* To insert a recipient's name into the subject line, add the following text in the `emailSubject` property when you create the template:
* 
* `[[<roleName>_UserName]]`
* 
* Example:
* 
* `"emailSubject":"[[Signer 1_UserName]], Please sign this NDA",`
* 
* To add a recipient's email address in the subject line, add the following text in the `emailSubject` property when you create the template or send an envelope from the template:
* 
* `[[<roleName>_Email]]`
* 
* Example:
* 
* `"emailSubject":"[[Signer 1_Email]], Please sign this NDA",`
* 
* In these examples, the role name specified in the template  is “Signer 1”.  When the envelope is sent, the placeholder will be dynamically substituted with the recipient's name or email.
* 
* ## Creating multiple templates
* 
* To create multiple templates, you provide a zip file of JSON files. You can also use the Templates::ListTemplates method with the `is_download` query parameter to download a zip file containing your existing templates and use that as a guide. The API supports both .zip and .gzip file formats as input.
* 
* You also need to set the following headers:
* 
* - `Content-Length`
* - `Content-Type`
* - `Content-Disposition`
* 
* Example:
* 
* `Content-Length: 71068`
* 
* `Content-Type: application/zip`
* 
* `Content-Disposition: file; filename="DocuSignTemplates_Nov_25_2019_20_40_21.zip"; fileExtension=.zip`
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn post(
&self,
account_id: &str,
body: &crate::types::EnvelopeTemplate
) -> Result<crate::types::TemplateSummary> {
let url =
format!("/v2.1/accounts/{}/templates",
crate::progenitor_support::encode_path(&account_id.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Gets a specific template associated with a specified account.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}` endpoint.
*
* Retrieves the definition of the specified template.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `include: &str` -- A comma-separated list
*   of additional template attributes
*   to include in the response.
*   Valid values are:
*   
*   - `powerforms`: Includes information about PowerForms.
*   - `tabs`: Includes information about tabs.
*   - `documents`: Includes information about documents.
*   - `favorite_template_status`: : Includes the template `favoritedByMe` property in the response. **Note**: You can mark a template as a favorite only in eSignature v2.1.
*/
pub async fn get_template(
&self,
account_id: &str, template_id: &str, include: &str,
) -> Result<crate::types::EnvelopeTemplate> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !include.is_empty() { query_args.push(format!("include={}", include)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/v2.1/accounts/{}/templates/{}?{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* Updates an existing template.
*
* This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}` endpoint.
*
* Updates an existing template.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn put_template(
&self,
account_id: &str, template_id: &str,
body: &crate::types::EnvelopeTemplate
) -> Result<crate::types::EnvelopeUpdateSummary> {
let url =
format!("/v2.1/accounts/{}/templates/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Returns document page image(s) based on input.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/pages` endpoint.
*
* Returns images of the pages in a template document for display based on the parameters that you specify.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `document_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `dpi: &str` -- The number of dots per inch (DPI) for the resulting images. Valid values are 1-310 DPI. The default value is 94.
* * `max_height: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `max_width: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `nocache: &str` -- If **true**, using cache is disabled and image information is retrieved from a database. **True** is the default value. .
* * `show_changes: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `start_position: &str` -- The position within the total result set from which to start returning values. The value **thumbnail** may be used to return the page image.
*/
pub async fn pages_get_template_page_image(
&self,
account_id: &str, document_id: &str, template_id: &str, count: &str, dpi: &str, max_height: &str, max_width: &str, nocache: &str, show_changes: &str, start_position: &str,
) -> Result<crate::types::PageImages> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !count.is_empty() { query_args.push(format!("count={}", count)); }
if !dpi.is_empty() { query_args.push(format!("dpi={}", dpi)); }
if !max_height.is_empty() { query_args.push(format!("max_height={}", max_height)); }
if !max_width.is_empty() { query_args.push(format!("max_width={}", max_width)); }
if !nocache.is_empty() { query_args.push(format!("nocache={}", nocache)); }
if !show_changes.is_empty() { query_args.push(format!("show_changes={}", show_changes)); }
if !start_position.is_empty() { query_args.push(format!("start_position={}", start_position)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/v2.1/accounts/{}/templates/{}/documents/{}/pages?{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),crate::progenitor_support::encode_path(&document_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* Deletes a page from a document in an template.
*
* This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/pages/{pageNumber}` endpoint.
*
* Deletes a page from a document in a template based on the page number.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
* * `page_number: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn pages_delete_template_page(
&self,
account_id: &str, document_id: &str, page_number: &str, template_id: &str,
body: &crate::types::PageRequest
) -> Result<()> {
let url =
format!("/v2.1/accounts/{}/templates/{}/documents/{}/pages/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),crate::progenitor_support::encode_path(&document_id.to_string()),crate::progenitor_support::encode_path(&page_number.to_string()),);

self.client.delete(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Gets a page image from a template for display.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/pages/{pageNumber}/page_image` endpoint.
*
* Retrieves a page image for display from the specified template.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
* * `page_number: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `dpi: &str` -- The number of dots per inch (DPI) for the resulting images. Valid values are 1-310 DPI. The default value is 94.
* * `max_height: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `max_width: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `show_changes: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn pages_get_template_page_image_templates(
&self,
account_id: &str, document_id: &str, page_number: &str, template_id: &str, dpi: &str, max_height: &str, max_width: &str, show_changes: &str,
) -> Result<()> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !dpi.is_empty() { query_args.push(format!("dpi={}", dpi)); }
if !max_height.is_empty() { query_args.push(format!("max_height={}", max_height)); }
if !max_width.is_empty() { query_args.push(format!("max_width={}", max_width)); }
if !show_changes.is_empty() { query_args.push(format!("show_changes={}", show_changes)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/v2.1/accounts/{}/templates/{}/documents/{}/pages/{}/page_image?{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),crate::progenitor_support::encode_path(&document_id.to_string()),crate::progenitor_support::encode_path(&page_number.to_string()),query);

self.client.get(&url, None).await
}

/**
* Rotates page image from a template for display.
*
* This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/pages/{pageNumber}/page_image` endpoint.
*
* Rotates page image from a template for display. The page image can be rotated to the left or right.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
* * `page_number: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn pages_put_template_page_image(
&self,
account_id: &str, document_id: &str, page_number: &str, template_id: &str,
body: &crate::types::PageRequest
) -> Result<()> {
let url =
format!("/v2.1/accounts/{}/templates/{}/documents/{}/pages/{}/page_image",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),crate::progenitor_support::encode_path(&document_id.to_string()),crate::progenitor_support::encode_path(&page_number.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Gets template notification information.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/notification` endpoint.
*
* Retrieves the envelope notification, reminders and expirations, information for an existing template.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn notification_get_template(
&self,
account_id: &str, template_id: &str,
) -> Result<crate::types::Notification> {
let url =
format!("/v2.1/accounts/{}/templates/{}/notification",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),);

self.client.get(&url, None).await
}

/**
* Updates the notification  structure for an existing template.
*
* This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/notification` endpoint.
*
* Updates the notification structure for an existing template. Use this endpoint to set reminder and expiration notifications.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn notification_put_template(
&self,
account_id: &str, template_id: &str,
body: &crate::types::TemplateNotificationRequest
) -> Result<crate::types::Notification> {
let url =
format!("/v2.1/accounts/{}/templates/{}/notification",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Shares a template with a group.
*
* This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/{templatePart}` endpoint.
*
* Shares a template with the specified members group.
* 
* **Note**: For a newer version of this functionality, see [Accounts::Update Shared Access](https://developers.docusign.com/docs/esign-rest-api/reference/Accounts/Accounts/updateSharedAccess).
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_part: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn put_template_part(
&self,
account_id: &str, template_id: &str, template_part: &str,
body: &crate::types::GroupInformation
) -> Result<crate::types::GroupInformation> {
let url =
format!("/v2.1/accounts/{}/templates/{}/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),crate::progenitor_support::encode_path(&template_part.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Removes a member group's sharing permissions for a template.
*
* This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/{templatePart}` endpoint.
*
* Removes a member group's sharing permissions for a specified template.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_part: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn delete_template_part(
&self,
account_id: &str, template_id: &str, template_part: &str,
body: &crate::types::GroupInformation
) -> Result<crate::types::GroupInformation> {
let url =
format!("/v2.1/accounts/{}/templates/{}/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),crate::progenitor_support::encode_path(&template_part.to_string()),);

self.client.delete(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}
//! A fully generated, opinionated API client library for DocuSign.
//!
//! [![docs.rs](https://docs.rs/docusign/badge.svg)](https://docs.rs/docusign)
//!
//! ## API Details
//!
//! The DocuSign REST API provides you with a powerful, convenient, and simple Web services API for interacting with DocuSign.
//!
//! [API Terms of Service](https://www.docusign.com/company/terms-and-conditions/web)
//!
//! ### Contact
//!
//!
//! | name | url | email |
//! |----|----|----|
//! | DocuSign Developer Center | <https://developers.docusign.com/> | devcenter@docusign.com |
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [DocuSign OpenAPI
//! specs](https://github.com/docusign/OpenAPI-Specifications) based on API spec version `v2.1`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! docusign = "0.2.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use docusign::Client;
//!
//! let docusign = Client::new(
//!     String::from("client-id"),
//!     String::from("client-secret"),
//!     String::from("redirect-uri"),
//!     String::from("token"),
//!     String::from("refresh-token"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `DOCUSIGN_CLIENT_ID`
//! - `DOCUSIGN_CLIENT_SECRET`
//! - `DOCUSIGN_REDIRECT_URI`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use docusign::Client;
//!
//! let docusign = Client::new_from_env(String::from("token"), String::from("refresh-token"));
//! ```
//!
//! It is okay to pass empty values for `token` and `refresh_token`. In
//! the initial state of the client, you will not know these values.
//!
//! To start off a fresh client and get a `token` and `refresh_token`, use the following.
//!
//! ```
//! use docusign::Client;
//!
//! async fn do_call() {
//!     let mut docusign = Client::new_from_env("", "");
//!
//!     // Get the URL to request consent from the user.
//!     // You can optionally pass in scopes. If none are provided, then the
//!     // resulting URL will not have any scopes.
//!     let user_consent_url = docusign.user_consent_url(&["some-scope".to_string()]);
//!
//!     // In your redirect URL capture the code sent and our state.
//!     // Send it along to the request for the token.
//!     let code = "thing-from-redirect-url";
//!     let state = "state-from-redirect-url";
//!     let mut access_token = docusign.get_access_token(code, state).await.unwrap();
//!
//!     // You can additionally refresh the access token with the following.
//!     // You must have a refresh token to be able to call this function.
//!     access_token = docusign.refresh_access_token().await.unwrap();
//! }
//! ```
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// The AcccountBrands resource provides methods that enable you to create and manage brands for an account.
///
///Branding enables you to add the look and feel of your organization's brand to the sending, signing, and email processes, making it easier for recipients to identify envelopes coming from your organization.
///
///The DocuSign Account Custom Branding feature enables you to set the colors, logo, and text that recipients see at the account level. The settings associated with a brand are applied to all of the envelopes that use the brand. You can create multiple brand profiles for different corporate brands or internal departments.
///
///**Note**: To use this resource, branding for either signing or sending must be enabled for the account (either `canSelfBrandSend`, `canSelfBrandSign`, or both of these account settings must be set to **true**).
pub mod account_brands;
/// The `AccountConsumerDisclosures` resource provides methods that enable you to enable, retrieve, and manage the Electronic Record and Signature Consent Disclosure (ERSD) options for your account. This is the disclosure that displays to each new recipient who is going to sign or add other information, or who is required to view the documents you send to them. The recipient must read and agree to the terms of the disclosure before they can access and take action on the documents you send. The ERSD does not apply to copy-only recipients, but does apply to recipients who must sign or view your documents.
///
///You can use either the default ERSD that DocuSign provides for U.S.-based transactions, or a custom ERSD.
///
///## Languages
///
///**Important**: The system does not translate the ERSD for you. The default ERSD is always in English. For a custom ERSD, an account administrator must create a version of the disclosure for each language that your signers use. When you create a version of your custom ERSD for a specific signer language, you must:
///
///1. Specify the language code (`langCode`) for the signer language.
///2. Provide the `esignAgreementText` and `esignText` in the language associated with the `langCode`.
///
///For more information, see [Legal Disclosure](https://support.docusign.com/en/guides/ndse-admin-guide-legal-disclosure).
pub mod account_consumer_disclosures;
/// Custom fields enable you to record custom information about envelopes that you can then use for sorting, organizing, searching, and other downstream processes.
///
///For example, you can use custom fields to copy envelopes or data to multiple areas in Salesforce. eOriginal customers can eVault all of their documents from the web app by setting an account custom field with a name like `eVault with eOriginal` to **true**.
///
///You can also use account custom fields to set the following information:
///
///- Tracking ID
///- Department
///- Use case
///- Other envelope metadata
///
///## Envelope Custom Field Visibility
///
///When you create an envelope custom field for your account, you have the following options:
///
///- Make it a required field for senders at the time of sending
///- Display it as an optional field at the time of sending
///- Set a specific value for the field behind the scenes (NOT SURE IF THIS IS RIGHT; MIGHT JUST BE AN UNUSED DRAFT FIELD)
///
///Envelope recipients do not see the envelope custom fields.
///
///## Types of Envelope Custom Fields
///
///There are two types of envelope custom fields:
///
///- `text`: Enables the sender to enter the value for the field.
///- `list`: Enables the sender to select the value of the field from a predetermined list.
pub mod account_custom_fields;
/// The AccountPasswordRules resource provides methods that allow you to obtain and update account password rules, as well as membership and account rules.
pub mod account_password_rules;
/// An account permission profile is assigned to a group of users, enabling you to set permissions for all of the users in the group at the same time. You are not required to set Permission Profiles for a group, but it makes it easier to manage user permissions for a large number of users.
///
///DocuSign offers the following account permission profiles:
///
///- DocuSign Viewer
///- DocuSign Sender
///- Account Administrator
///
///You can also create your own custom account permission profiles.
///.
pub mod account_permission_profiles;
/// .
pub mod account_seal_providers;
/// Standards-Based Signatures (SBS) is the label used to describe DocuSign's suite of signatures that comply with regional and industry regulations, such as the electronic IDentification, Authentication and trust Services (eIDAS) regulation in Europe.
///
///## Feature Differences When Using Standards-Based Signatures
///
///Some DocuSign features are not compatible with Standards-Based Signatures, while others work somewhat differently. It's important to understand these key differences.
///
///### DocuSign Features Not Compatible with SBS
///
///- Attachment by fax
///- Concatenation of signer attachments
///- Legacy digital signatures
///- Markup
///- Notary
///
///### DocuSign Features That Change with SBS
///
///The following features work slightly differently with SBS:
///
///- **Advanced Correct**: After the first signature, adding or deleting a field is not allowed. This behavior occurs because SBS does not allow adding or removing form fields after a digital signature has already been applied to a PDF.
///- **Downloading Combined Envelopes**: A combined PDF is not digitally signed. This behavior occurs because concatenating digitally signed PDFs breaks the digital signatures on the PDFs.
///- **Freeform Signing**: After someone has signed, allows only signature and initials on free-form. This behavior occurs because if another signer has already signed the document, adding fields other than signature fields will break the existing digital signatures on the document.
///- **Watermarks**: All watermarks are added as PDF annotations. This behavior occurs because burning the watermark into the PDF will break the digital signatures on the document.
///- **Wet Signing**: Wet-signed documents are added as new documents to envelopes. This behavior results in the uploaded or faxed, physically signed document being added as a new document to the envelope. This new document gets only a platform seal.
///
///For more information, see [Standards Based Signatures](https://developers.docusign.com/esign-rest-api/guides/standards-based-signatures).
pub mod account_signature_providers;
/// .
pub mod account_signatures;
/// The AccountTabSettings resource provides methods that allow you to manage
///tab settings for an account.
///.
pub mod account_tab_settings;
/// The AccountWatermarks resource provides methods that allow you to obtain, preview and update watermark information.
pub mod account_watermarks;
/// The Accounts resource provides methods that allow you to create, delete, and manage your accounts.
pub mod accounts;
/// .
pub mod appliance_info;
/// You can configure automatic archiving of emails sent from all of your DocuSign accounts.
///
///For more information, see [Email Archive Configuration](https://support.docusign.com/en/guides/ndse-admin-guide-email-archive-configuration).
///
///**Note**: This feature is only available for certain account plans and must be enabled by DocuSign.
pub mod bcc_email_archive;
/// The Billing resource provides methods that allow you to manage the billing plans,associated with an account.
pub mod billing_plans;
/// You can use bulk send lists for anything that you need to send to a large number of recipients on a
///recurring basis, such as:
///
///- Compliance letters (privacy, security, and ethics consent forms)
///- New hire onboarding documents (benefits, transit, and parking information and payroll forms)
///- Other Human Resources documents
///- Event-related forms
///
///The API creates a separate copy of the envelope for each recipient that you specify. Each instance of the envelope is called a
///`BulkCopy`. You can use a bulk send list to send up to 1,000 copies per call.
///
///After you create a bulk send list, it persists and can be reused and edited any number of times.
///
///## Customizing Bulk Send Lists
///
///You can customize individual copies of the envelope. For example, you can customize the email notification and
///language and add personalized notes.
///
///For example, if one recipient prefers to access their DocuSign envelopes behind an access code, and another prefers her
///email in French, you can implement those customizations.
///
///## Using Bulk Send
///
///The bulk send feature uses the following flow:
///
///1. Create a draft envelope by calling the [Envelopes::createEnvelope][createEnvelope] method. Add placeholders for bulk send information to the envelope, including:
///    - Email address placeholders.
///    - Tab placeholders. Assign `tabLabels` to these placeholders that will make sense for matching the tabs to values in the bulk send list. For example, if you're sending a field trip permission slip to parents, you might create a placeholder text tab called `StudentName` that will you will then populate with the names of individual students in the bulk send list. You can use the following types of text tabs, radio group tabs, and list tabs to match bulk send recipients to an envelope.
///    - Any envelope custom fields. These fields must match the envelope custom fields in envelope copies in the bulk send list.
///2. Create a bulk send list by using the [BulkSend::createBulkSendList][create_list] method.
///3. (Optional) Test compatibility. Use the [BulkSend::createBulkSendTestRequest][create_test] method to test your bulk
///   send list for compatibility with the envelope or template that you want to send. For example, a template that has
///   three roles is not compatible with a bulk send list that has only two recipients. For this reason, you might want to
///   test compatibility first. A successful test result returns the Boolean value `true`. An unsuccessful test returns a
///   JSON response that contains information about the errors that occurred.
///4. Send your envelope to the list by using the [BulkSend::createBulkSendRequest][create_request] method. The response
///   returns a `batchId` that you can use for tracking and other purposes.
///
///The API creates and queues your envelopes asynchronously behind the scenes. You can get the status of the batch by
///using the [BulkEnvelopes_GetBulkEnvelopesBatchId][getbulkenv] method, passing in the `batchId`.
///
///To get the envelopes generated for the `batchId`, use the [Envelopes:listStatusChanges][GetEnvelopes] method, passing in
///a `custom_field` named `BulkBatchId` where the value is the `batchId` that was returned in step 4.
///
///Example:
///
///`custom_field=BulkBatchId={{batchId}}`
///
///**Bulk Send Requirements and Limitations**
///
///* Bulk send must be enabled for your account (in the `accountSettingsInformation` object, `enableBulkRecipient` must be set to **true**) and for the user sending the envelopes (the `allowBulkRecipients` property in `userSettings` must be set to **true**).
///* You can include up to 1,000 bulk recipients in each request.
///* When you send an envelope with bulk recipients, envelopes are added to a bulk recipient queue and sent in a metered fashion. An account can have a total of 2,000 total envelopes in the queue at a time.  If this limit is reached, an error message displays to the sender. If you receive this error, wait and resend the envelope at a later time.
///
///If you frequently run into queue limits, contact your account manager to discuss modifying the queue limits for your account.
///
///[create_list]:    https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/createBulkSendList/
///[create_test]:    https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/createBulkSendTestRequest/
///[create_request]: https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/createBulkSendRequest/
///[getbulkenv]:     https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/getBulkSendLists/
///[GetEnvelopes]:   https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/listStatusChanges/
///
///[createEnvelope]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeCustomFields/create/
///
///
///For more information about using bulk send, see [Bulk Sending Envelopes](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/envelopes/bulk-send/).
///.
pub mod bulk_send;
/// A chunked upload is a temporary file that you upload in parts and stage at DocuSign, then refer to as the content for other API calls. For example, you might use it for document content when assembling an envelope or template.
///
///A chunked upload is linked to the DocuSign account member who initiated the API call. This user is the only user who is able to reference the chunked upload.
///
///A ChunkedUpload is intended to be an area for briefly staging data for use with other DocuSign API calls. The ChunkedUpload API endpoints do not provide an action to download the ChunkedUpload's content.
///
///The typical flow for using a chunked upload involves the following steps:
///
///1) Initiate the chunked upload with content representing part 0.
///
///2) Add more parts to the chunked upload until you have transmitted the entirety of the content.
///
///3) Commit the chunked upload, preparing it for use with other API calls.
///
///4) Assemble a DocuSign envelope with a document that includes a reference to the chunked upload as the content.
///
///5) Continue with envelope-related processes.
///
///**Note**: You must fully upload and use a chunked upload within 20 minutes of initializing it.
///
///After the chunked upload has been correctly referenced within another API call, it becomes unavailable for any further use and is promptly removed from the system.
///
///Chunked uploads have the following limits, which are configured per DocuSign environment, account, or integrator:
///
///- The maximum number of all of a member's unexpired, unconsumed ChunkedUploads. The default value is 10.
///- The maximum total size of all of a member's unexpired, unconsumed  ChunkedUploads. The default value is 1 GB.
///- The amount of time that a chunked upload is active after you initialize it. The default value is 20 minutes.
///.
pub mod chunked_uploads;
/// The CloudStorage resource provides methods that allow you to list files stored on your cloud storage provider.
pub mod cloud_storage;
/// The following providers are supported:
///
///* Google Drive
///* Dropbox
///* Box
///* Evernote
///* OneDrive
///
///To use cloud storage files, you must first give DocuSign access to your cloud storage provider. You can disconnect authorized a cloud storage provider at any time.
pub mod cloud_storage_providers;
/// The comments resource provides a method that enables you to download a PDF file containing all of the comments that recipients and the sender have added to the documents in an envelope.
///
///Comments are disabled by default. To use the comments feature, an account administrator must enable comments on the account (in the `accountSettingsInformation` object, set the `enableSigningExtensionComments` property to **true**).
///
///For more information, see [Comments Settings](https://support.docusign.com/en/guides/ndse-admin-guide-comments-settings).
pub mod comments;
/// The `ConnectConfigurations` resource enables you to configure the DocuSign Connect service for your account.
///
///You can use this resource to configure account-level webhooks that send notifications about every envelope sent from your account. You can set account-level webhooks to listen for events for envelopes sent by a specific user on your account, by multiple specific users, or from any of the users on your account. These events will be tracked, and can be delivered to a listening application.
///
///**Note**: To use DocuSign Connect, it must be enabled in your DocuSign account. It is not enabled by default.
///
///## Aggregated Messages
///
///To avoid duplicate simultaneous events, you can configure Connect to aggregate similar events into a single delivery. By default, aggregation is enabled on all Connect configurations. Similar or simultaneous events will be aggregated so your listener doesn't receive extraneous messages.
///
///For example, when the final recipient signs an envelope,  the system delivers a single, aggregated Connect event, rather than separate Recipient: complete and Envelope: complete messages. This aggregation process ensures that you only receive the minimal viable number of messages about an envelope's life cycle.
///
///## Send Individual Messages
///
///When you enable Send Individual Messages (SIM) mode on a Connect configuration, DocuSign will deliver notifications for all envelope events individually. In contrast with aggregated messages, when a final recipient completes an envelope, your listener will receive a single Recipient: complete event followed by a single Envelope: complete event for the final participating party on the agreement.  If you need more granular control over event notifications, you can enable SIM mode in the Admin area of the RADmin console. For more information about SIM mode, see [Using Connect's New Send Individual Messages Feature](https://www.docusign.com/blog/dsdev-connect-send-individual-messages/).
///
///**Note**: To create an envelope-level webhook instead of using account-level webhooks, use the Envelopes::Create method and add an `eventNotification` object to an envelope object.
pub mod connect_configurations;
/// The ConnectEvents resource provides methods that allow you to read, delete, and republish the connect logs associated with an envelope.
pub mod connect_events;
/// .
pub mod connect_secret;
/// DocuSign eSignature includes a contacts list (also referred to as an address book) to help make sending envelopes even easier. When you send an envelope, the recipients' names and email addresses are automatically added to your contacts list. You can use the contacts list to quickly add recipients to the envelopes you send. The `Contacts` resource provides methods that enable you to manage your contacts.
///
///.
pub mod contacts;
/// The CustomTabs resource provides methods that allow you create and manage custom tabs based on the existing DocuSign tabs.
///
///You can create a tab with pre-defined properties, such as a text tab with a certain font type and validation pattern. Users can access the custom tabs when sending documents through the DocuSign web application.
///
///Custom tabs can be created based on the  approve, checkbox, company, date, date signed, decline, email, email address, envelope ID, first name, formula, full name, initial here, last name, list, note, number, radio, sign here, signer attachment, SSN, text, title, and zip tabs.
pub mod custom_tabs;
/// **Note**: Responsive Signing is disabled by default. To use this functionality, an account administrator must switch the account setting `enableResponsiveSigning` to **true**.
///Also note that Smart Sections (creating a signable HTML document that uses collapsible sections and rotating tables) are premium features. To request them, contact your DocuSign account manager.
pub mod document_responsive_html_preview;
/// The ENoteConfigurations resource provides methods that allow you to manage
///information for the eNote eOriginal integration.
///.
pub mod e_note_configurations;
/// The EnvelopeAttachments resource provides methods that allow you to manage attachments.
///.
pub mod envelope_attachments;
/// The `EnvelopeConsumerDisclosures` resource provides methods that enable you to retrieve the Electronic Record and Signature Disclosure (ERSD) for an envelope recipient. This is the disclosure that displays to each new recipient who is going to sign or add other information, or who is required to view the documents you send to them. The recipient must read and agree to the terms of the disclosure before they can access and take action on the documents you send. The ERSD does not apply to copy-only recipients, but does apply to recipients who must sign or view your documents.
///
///You can retrieve either the default ERSD that DocuSign provides for U.S.-based transactions, or a custom ERSD.
///
///## Languages
///
///You specify the language of the disclosure version that you want to retrieve by using the `langCode` parameter.
///
///**Important**: The system does not translate the ERSD for you. An account administrator must create a version of the account-level disclosure for each language that your signers use.
///
///For more information, see [Legal Disclosure](https://support.docusign.com/en/guides/ndse-admin-guide-legal-disclosure).
pub mod envelope_consumer_disclosures;
/// The EnvelopeCustomFields resource provides methods that allow you manage custom fields in an envelope.
///
///Custom fields can be used in the envelopes for your account to record information about the envelope, help search for envelopes and track information. The envelope custom fields are shown in the Envelope Settings section when a user is creating an envelope in the DocuSign member console. The envelope custom fields are not seen by the envelope recipients.
///
///There are two types of envelope custom fields:
///
///- `text`: Enables the sender to enter the value for the field.
///- `list`: Enables the sender to select the value of the field from a predetermined list.
///
///You may assign up to three envelope custom fields to an envelope. This limit does not include account (document) custom fields.
pub mod envelope_custom_fields;
/// The EnvelopeDocumentFields resource provides methods that allow you to manage custom fields on a document.
///
///You can create custom versions of standard fields that combine of field properties, such as font type or size, or a validation setting.
///
///Note: Some advanced features and options are supported only in certain DocuSign plans. Your account plan might not support some options discussed in this help topic. For more information about which options are available for your account, check your account plan or contact your Account Manager.
pub mod envelope_document_fields;
/// .
pub mod envelope_document_html_definitions;
/// The EnvelopeDocumentTabs resource provides methods that enable you to manage tabs in envelopes. For a complete list of options, see the following Properties section.
pub mod envelope_document_tabs;
/// The Envelope Document Visibility resource provides methods for managing document visibility by recipient.
///.
pub mod envelope_document_visibility;
/// <!-- resources aren't rendered the same way
///     as other pages. This is a little hack to
///     make the headings work better -->
///<style>
///h1, h2, h3 {
///  margin-top: 1em;
///}
///</style>
///
///The EnvelopeDocuments resource provides methods
///that manage documents in an envelope.
///You can:
///* add one or more documents to the envelope
///* retrieve one or more documents from the envelope
///* delete documents from the envelope
///
///All of the methods in this resource
///operate on on an existing envelope.
///Before you can add documents
///to an envelope,
///you must first create it
///with the [Envelopes: create][envelopescreate] method.
///
///
///[envelopescreate]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/create/
///
///.
pub mod envelope_documents;
/// The EnvelopeEmailSettings provide methods that allow you to manage the email override settings for an envelope.
///
///Email override settings change the reply to email address, name, or the BCC for email archive information, for the envelope. Note that changing email settings will only affect email communications that occur after the addition was made.
///
///The BCC Email address feature is designed to provide a copy of all email communications for external archiving purposes. DocuSign recommends that envelopes sent using the BCC for Email Archive feature, including the BCC Email Override option, include additional signer authentication options. To send a copy of the envelope to a recipient who does not need to sign, use a Carbon Copies or Certified Deliveries Recipient Type.
pub mod envelope_email_settings;
/// The EnvelopeFormData resource provides a method for downloading the data that users have entered into a form associated with an envelope.
pub mod envelope_form_data;
/// .
pub mod envelope_html_definitions;
/// The EnvelopeLocks resource provides methods that allow you to
///manage locks on an envelope.
///
///To prevent users from changing an envelope while another user is
///modifying it, you can lock the envelope and set the time until
///the lock expires.
///
///For example, you would use the following flow:
///
///1. Lock the envelope.
///2. Make changes to envelope.
///3. Delete the envelope lock and save the changes. If the envelope is based on a template that has a password, you must supply the template password to save the changes.
///
///**Note**: To use envelope locks, the user must have envelope locking capability enabled.
pub mod envelope_locks;
/// The EnvelopeRecipientTabs resource provides methods that enable you
///to add,
///update,
///and delete tabs
///from an envelope.
///Tabs are associated with a specific recipient
///in an envelope
///and are only used by the recipient types
///In Person Signers and Signers.
///
///<!-- START doctoc generated TOC please keep comment here to allow auto update -->
///<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
///**On this page**
///
///- [Tab Types](#tab-types)
///- [View Tab](#view-tab)
///- [Requesting Payments](#requesting-payments)
///- [Using Custom Tabs in Envelopes and Templates](#using-custom-tabs-in-envelopes-and-templates)
///- [Anchoring Tabs](#anchoring-tabs)
///- [Automatically Populating Tabs](#automatically-populating-tabs)
///
///<!-- END doctoc generated TOC please keep comment here to allow auto update -->
///
///
///## Tab Types
///
///Some tabs enable values to be entered by the signer.
///Those tabs' values can be preset either through the web browser
///or via the API. Other tab types use information that is already
///recognized by the DocuSign platform.
///These tabs cannot have their value updated on a per-tab basis
///by the API or via the browser. In some cases, the info might be
///settable using a different technique.
///For example, the Full name tab uses the signer's name,
///which is set elsewhere in the request.
///
///Here is the list of tabs and whether you can or cannot set their values in the tab definition:
///
///<br>
///
///| Tab Type                               | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
///| :------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
///| Approve (`approve`)                    | Allows the recipient to approve documents without placing a signature or initials on the document. If the recipient clicks the tab during the signing process, the recipient is considered to have signed the document. No information is shown on the document of the approval, but it is recorded as a signature in the envelope history. This value **can't** be set.                                                                                                                                                                                                                                              |
///| Checkbox (`checkbox`)                  | Allows the recipient to select a yes/no (on/off) option. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
///| Company (`company`)                    | Displays the recipient's company name. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
///| Date Signed (`dateSigned`)             | Displays the date that the recipient signed the document. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
///| Date (`date`)                          | Allows the recipient to enter a date. Date tabs are one-line fields that allow date information to be entered in any format. The tooltip for this tab recommends entering the date as MM/DD/YYYY, but this is not enforced. The format entered by the signer is retained. If you need a particular date format enforced, DocuSign recommends using a Text tab with a validation pattern and a validation message to enforce the format. This value can be set.                                                                                                                                                        |
///| Decline (`decline`)                    | Allows the recipient the option of declining an envelope. If the recipient clicks the tab during the signing process, the envelope is voided. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                            |
///| Email Address (`emailAddress`)         | Displays the recipient's email as entered in the recipient information. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
///| Email (`email`)                        | Allows the recipient to enter an email address. This is a one-line field that checks that a valid email address is entered. It uses the same parameters as a Text tab, with the validation message and pattern set for email information.<br><br>When getting information that includes this tab type, the original value of the tab when the associated envelope was sent is included in the response. This value can be set.                                                                                                                                                                                        |
///| Envelope ID (`envelopeId`)             | Displays the envelope ID. Recipients cannot enter or change the information in this tab.  This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
///| First Name (`firstName`)               | Displays the recipient's first name. This tab takes the recipient's name as entered in the recipient information, splits it into sections based on spaces and uses the first section as the first name. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                  |
///| Formula Tab (`formulaTab`)             | The value of a formula tab is calculated from the values of other number or date tabs in the document. When the recipient completes the underlying fields, the formula tab calculates and displays the result. This value can be set.The `formula` property of the tab contains the references to the underlying tabs. See [Calculated Fields] in the DocuSign Support Center to learn more about formulas. If a formula tab contains a `paymentDetails` property, the tab is considered a payment item. See [Requesting Payments Along with Signatures] in the DocuSign Support Center to learn more about payments. |
///| Full Name (`fullName`)                 | Displays the recipient's full name. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
///| Initial Here (`initialHere`)           | Allows the recipient to initial the document. May be optional. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
///| Last Name (`lastName`)                 | Displays the recipient's last name. This tab takes the recipient's name as entered in the recipient information, splits it into sections based on spaces and uses the last section as the last name. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                     |
///| List (`list`)                          | This tab offers a list of options to choose from. The `listItems` property is used to specify the selectable options. This value can be set,                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
///| Notarize (`notarize`)                  | Place this tab on a page to alert Notary recipients that they must take action. Only one notarize tab can appear on a page. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
///| Note (`note`)                          | Displays additional information, in the form of a note, for the recipient. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
///| Number (`number`)                      | Allows the recipient to enter numbers and decimal (.) points. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
///| Radio Group (`radioGroup`)             | This group tab is used to place radio buttons on a document. The `radios` property is used to add and place the radio buttons associated with the group. Only one radio button can be selected in a group. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                     |
///| Sign Here (`signHere`)                 | Allows the recipient to sign a document. May be optional. This value **can't** be set. <br/>**Note**: `signHere` tabs can also be used to add a visual representation for  an electronic seal in a document.                                                                                                                                                                                                                                                                                                                                                                                                          |
///| Signer Attachment (`signerAttachment`) | Allows the recipient to attach supporting documents to an envelope. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
///| SSN (`ssn`)                            | A one-line field that allows the recipient to enter a Social Security Number. The SSN can be typed with or without dashes. It uses the same parameters as a Text tab, with the validation message and pattern set for SSN information.  This value can be set.                                                                                                                                                                                                                                                                                                                                                        |
///| Text (`text`)                          | Allows the recipient to enter any type of text. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
///| Title (`title`)                        | Displays the recipient's title. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
///| View (`view`)                          | The View tab is used with the Approve tab to handle supplemental documents. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
///| Zip (`zip`)                            | Allows the recipient to enter a ZIP code. The ZIP code can be five digits or nine digits in the ZIP+4 format. The zip code can be typed with or without dashes. It uses the same parameters as a Text tab, with the validation message and pattern set for ZIP code information. This value can be set.                                                                                                                                                                                                                                                                                                               |
///
///
///[approve]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[checkbox]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[company]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[dateSigned]:		    https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[date]:		          https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[decline]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[emailAddress]:     https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[email]:		        https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[envelopeId]:	    	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[firstName]:	    	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[formulaTab]:	    	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[fullName]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[initialHere]:	  	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[lastName]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[list]:	           	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[notarize]:         https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[note]:		          https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[number]:		        https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[radioGroup]:		    https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[signerAttachment]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[signHere]:	      	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[ssn]:	          	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[text]:	          	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[title]:	        	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[view]:		          https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///[zip]:	          	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
///
///[calculatedfields]: https://support.docusign.com/en/guides/ndse-user-guide-calculated-fields
///[paymentguide]:     https://support.docusign.com/en/guides/requesting-payments-along-with-signatures
///
///
///
///
///
///
///## View Tab
///
///The View tab is used on supplemental documents.
///To learn more about using the View tab with
///supplemental documents, see
///[Using Supplemental Documents][usingsupdocs]
///in the [Sending Documents][sendenvelopes] section of
///the [Envelope: create][envelopecreate] method.
///
///[sendenvelopes]:  https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/create/
///[usingsupdocs]:   https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/documents/supplemental/
///[envelopecreate]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/create/
///
///<br>
///
///| Name              | Required  | Type    | Description |
///| :----             | :----     | :----   | :----       |
///| documentId        | Yes       | String  | The document ID number that the tab is placed on. This must refer to an existing Document's ID attribute. |
///| pageNumber        | Yes       | String  | Must be set to 1. |
///| recipientId       | Yes       | String  | The recipient associated with the tab. Must refer to an existing recipient's ID attribute. |
///| required          | No        | Boolean | If **true**, the recipient is required to select the supplemental document View button during signing. |
///| tabLabel          | No        | String  | The label used for the tab.  If an empty string is provided for this, an empty sting is used. If no value is provided, the tab type is used as the value. Maximum of 500 characters. |
///| templateLocked    | No        | Boolean | Optional. Used only when working with template tabs. If **true**, the attributes of the tab cannot be changed by the sender. |
///| templateRequired  | No        | Boolean | Optional. Used only when working with template tabs. If **true**, the tab cannot be removed by the sender. |
///| xPosition         | Yes       | String  | Required, but can be 0.  |
///| yPosition         | Yes       | String  | Required, but can be 0. |
///
///
///## Requesting Payments
///
///The Payments feature of the DocuSign eSignature REST API
///lets you collect payments
///along with signatures and other information.
///
///To send a request for payment
///and collect payments,
///you need a payment gateway account,
///which is the destination for the payments.
///Create a payment account
///with a supported payment gateway,
///and then connect the payment gateway account
///to your DocuSign account.
///To learn how to connect a payment gateway account
///to your DocuSign account
///see [Managing Payment Gateways][paymentgateways]
///in the DocuSign Support Center.
///You must connect and manage payment gateway accounts manually
///through the DocuSign Admin console
///and through your chosen payment gateway.
///There is no public API
///for connecting payment gateway accounts
///to DocuSign accounts
///or for managing payment gateway accounts.
///
///Currently
///[Stripe][stripe],
///[Braintree](https://www.braintreepayments.com/), and
///[Authorize.net](https://www.authorize.net/)
///are the supported payment gateways.
///
///### How Payments Work
///
///To make a request for payment,
///use a [`formulaTab`][formulatab]
///that has a
///[`paymentDetails`][paymentdetails] object.
///This object includes
///a list of [`paymentLineItem`][paymentlineitem] objects.
///Each line item refers to a [`number`][numbertab] tab
///that contains the value of the each item.
///The purpose of the line items
///is to transmit them to the payment gateway
///as metadata, so that you can use the information
///in the payment processor.
///
///**Note**: If the fileExtension parameter is not added in an API call, only base64 converted pdf files will be accepted.
///Any attempt to send a non pdf file without using fileExtension results in an error.
///
///This is an example request for two books.
///Each book is specified in the `number` tabs
///labeled "Hamlet" and "Tempest".
///The books are referenced as line items
///by their tab labels
///within the `paymentDetails` object
///of a `formula` tab.
///The formula of the `formula` tab
///add the values of the same two `number` tabs.
///
///```json
/// {
///  "documents": [
///    {
///      "documentBase64": "<base64-encoded PDF document>",
///      "documentId": "1",
///      "fileExtension": "pdf",
///      "name": "payment.pdf"
///    }
///  ],
///  "emailSubject": "Order Some Books",
///  "recipients": {
///    "signers": [
///      {
///        "email": "vreader@example.com",
///        "name": "Voracious Reader",
///        "recipientId": "1",
///        "routingOrder": "1",
///        "tabs": {
///          . . .
///          "numberTabs": [
///            {
///              "value": "10.00",
///              "width": 78,
///              "required": "true",
///              "locked": "true",
///              "tabLabel": "Hamlet",
///              "documentId": "1",
///              "pageNumber": "1",
///              "xPosition": "323",
///              "yPosition": "134"
///            },
///            {
///              "value": "10.00",
///              "width": 78,
///              "required": "true",
///              "locked": "true",
///              "tabLabel": "Tempest",
///              "documentId": "1",
///              "pageNumber": "1",
///              "xPosition": "323",
///              "yPosition": "154"
///            }
///          ],
///          "formulaTabs": [
///            {
///              "required": "true",
///              "formula": "([Hamlet] + [Tempest]) * 100",
///              "roundDecimalPlaces": "2",
///              "paymentDetails": {
///                "currencyCode": "USD",
///                "lineItems": [
///                  {
///                    "name": "Hamlet",
///                    "description": "The Danish Play",
///                    "itemCode": "SHAK1",
///                    "amountReference": "Hamlet"
///                  },
///                  {
///                    "name": "Othello",
///                    "description": "The one with Caliban in it",
///                    "itemCode": "SHAK2",
///                    "amountReference": "Tempest"
///                  }
///                ],
///                "gatewayAccountId": "e76668b4-xxxx-xxxx-xxxx-a208d659e490"
///              },
///              "tabLabel": "Payment1",
///              "documentId": "1",
///              "pageNumber": "1",
///              "xPosition": 300,
///              "yPosition": 200
///            }
///          ]
///        }
///      }
///    ]
///  },
///  "status": "sent"
/// }
/// ```
///
///Use the
///[EnvelopeRecipients: list][enveloperecipientslist] method
///to check the status of a payment.
///When the payment is successful,
///the `status` property of
///the [`paymentDetails`][paymentdetails] object
///is `payment_complete`.
///
///```json
/// {
///  "signers": [
///    {
///      "tabs": {
///        . . .
///        "numberTabs": [
///          {
///            "value": "10.00",
///            "tabLabel": "Hamlet",
///            "documentId": "1",
///            "recipientId": "1",
///            "pageNumber": "1",
///            "xPosition": "323",
///            "yPosition": "134",
///          },
///          {
///            "value": "10.00",
///            "tabLabel": "Tempest",
///            "documentId": "1",
///            "recipientId": "1",
///            "pageNumber": "1",
///          }
///        ],
///        "formulaTabs": [
///          {
///            "formula": "([Hamlet] + [Tempest]) * 100",
///            "roundDecimalPlaces": "2",
///            "paymentDetails": {
///              "status": "payment_complete",
///              "currencyCode": "USD",
///              "lineItems": [
///                {
///                  "name": "Hamlet",
///                  "description": "The Danish Play",
///                  "itemCode": "SHAK1",
///                  "amountReference": "Hamlet"
///                },
///                {
///                  "name": "Tempest",
///                  "description": "The one with Caliban in it",
///                  "itemCode": "SHAK2",
///                  "amountReference": "Tempest"
///                }
///              ],
///              "gatewayAccountId": "e76668b4-xxxx-xxxx-xxxx-a208d659e490"
///            },
///            "value": "20",
///            "required": "true",
///            "locked": "false",
///            "tabLabel": "Payment1",
///            "documentId": "1",
///            "recipientId": "1",
///            "pageNumber": "1",
///          }
///        ]
///      },
///      "creationReason": "sender",
///      "email": "vreader@example.com",
///      "name": "Voracious Reader",
///      "recipientId": "1",
///      "requireIdLookup": "false",
///      "status": "completed",
///    }
///  ],
///  . . .
/// }
/// ```
///
///#### How to make a request for future payments
///
///Use the following steps to make a request to collect a signer's payment method for future use:
///
///1. Add a text tab with a descriptive `tabLabel` such as `FuturePayment`.
///2. In the formula tab's `paymentDetails` object, add a `lineItem` object that references the `tabLabel` from step 1.
///
///**Note**: Do not include this new `lineItem` in formula calculations.
///
///The following example builds on the previous code block to also collect a payment method for future use:
///
///```
/// {
///  "documents": [
///    {
///      "documentBase64": "<base64-encoded PDF document>",
///      "documentId": "1",
///      "fileExtension": "pdf",
///      "name": "payment.pdf"
///    }
///  ],
///  "emailSubject": "Order Some Books",
///  "recipients": {
///    "signers": [
///      {
///        "email": "vreader@example.com",
///        "name": "Voracious Reader",
///        "recipientId": "1",
///        "routingOrder": "1",
///        "tabs": {
///          "numberTabs": [
///            {
///              "value": "10.00",
///              "width": 78,
///              "required": "true",
///              "locked": "true",
///              "tabLabel": "Hamlet",
///              "documentId": "1",
///              "pageNumber": "1",
///              "xPosition": "323",
///              "yPosition": "134"
///            },
///            {
///              "value": "10.00",
///              "width": 78,
///              "required": "true",
///              "locked": "true",
///              "tabLabel": "Tempest",
///              "documentId": "1",
///              "pageNumber": "1",
///              "xPosition": "323",
///              "yPosition": "154"
///            }
///          ],
///          "textTabs": [
///            {
///              "value": "",
///              "width": 78,
///              "required": "true",
///              "locked": "true",
///              "tabLabel": "FuturePayment",
///              "documentId": "1",
///              "pageNumber": "1",
///              "xPosition": "323",
///              "yPosition": "174"
///            }
///          ],
///          "formulaTabs": [
///            {
///              "required": "true",
///              "formula": "([Hamlet] + [Tempest]) * 100",
///              "roundDecimalPlaces": "2",
///              "paymentDetails": {
///                "currencyCode": "USD",
///                "paymentOption": "save_and_authorize",
///                "lineItems": [
///                  {
///                    "name": "Hamlet",
///                    "description": "The Danish Play",
///                    "itemCode": "SHAK1",
///                    "amountReference": "Hamlet"
///                  },
///                  {
///                    "name": "Othello",
///                    "description": "The one with Caliban in it",
///                    "itemCode": "SHAK2",
///                    "amountReference": "Tempest"
///                  },
///                  {
///                    "name": "Request books",
///                    "description": "collect Payment method",
///                    "itemCode": "",
///                    "amountReference": "FuturePayment"
///                  }
///                ],
///                "gatewayAccountId": "e76668b4-xxxx-xxxx-xxxx-a208d659e490"
///              },
///              "tabLabel": "Payment1",
///              "documentId": "1",
///              "pageNumber": "1",
///              "xPosition": 300,
///              "yPosition": 200
///            }
///          ]
///        }
///      }
///    ]
///  },
///  "status": "sent"
/// }
/// ```
///
///### Some Things to Keep in Mind About Payments
///
///* An envelope is not completed until all payments are completed.
///
///* If a DocuSign account Administrator
///  deletes a payment gateway account connection
///  DocuSign cancels all in-process envelopes
///  that reference the deleted payment gateway account.
///
///* If the sender voids an envelope,
///  all payment authorizations are canceled.
///
///* If a required recipient refuses to sign,
///  all authorized payments are canceled.
///
///* If a required recipient's payment fails authorization,
///  DocuSign attempts to recover
///  by sending the recipient
///  notification about the failed payment authorization.
///  The recipient has the opportunity
///  to correct the payment method information.
///
///* Each recipient's payment is authorized separately.
///  Accounts are charged and payment made
///  after all required tabs are completed,
///  and all payments in an envelope for all recipients are authorized.
///
///* Refunds are not supported.
///  Conduct refunds or payment changes
///  with the payment gateway separately from DocuSign.
///
///* At this time, DocuSign does not charge a per-transaction fee.
///
///
///[enveloperecipientslist]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipients/list/
///[formulatab]:             https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/create/
///[ISO4217]:                https://en.wikipedia.org/wiki/ISO_4217
///[numbertab]:              https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/create/
///[paymentgateways]:        https://support.docusign.com/en/guides/managing-payment-gateways
///[paymentguide]:           https://support.docusign.com/en/guides/requesting-payments-along-with-signatures
///[paymentlineitem]:        https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/create/
///[paymentdetails]:         https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/create/
///[stripe]:                 https://stripe.com/
///
///
///## Using Custom Tabs in Envelopes and Templates
///
///Custom Tabs can be added to envelopes and templates
///by setting the `customTabId` property
///when creating an envelope or template recipient
///or when adding a new tab for an existing recipient.
///The custom tab must be added as the correct tab type.
///For example if the custom tab type is text, it cannot be used as a number tab.
///
///When the `customTabId` property is set,
///the new tab inherits all the custom tab properties.
///Required information that is not included in the custom tab,
///such as document ID and page ID, must be included when adding the tab.
///If the custom tab does not have anchor settings, the X and Y positions must be included.
///
///After the tab is created,
///it is treated as any other tab for updating or deleting.
///
///## Anchoring Tabs
///
///The tab anchoring option
///allows you to send documents for signature
///that do not have a fixed layout or format.
///In these documents you might not know
///the absolute location of the tabs
///when you design your API client application because the tabs must move with text.
///As an alternative to sending X and Y coordinates for tabs,
///the DocuSign Service can derive an anchor location for the tab
///by correlating anchor information to data within the document.
///
///When the DocuSign Service receives a request that contains tabs
///with anchor information,
///it searches the document for instances of the `anchorString` property.
///When found,
///it places a tab of the specified type for the designated recipient.
///Tab positions are established by setting offsets for the tab.
///
///When you apply tabs to the document,
///DocuSign does not remove or replace the text in the `anchorString` property. You can hide codified anchors by using the same font color as the background of the document. So the anchor can be used by DocuSign processes and it will not be visible on the document.
///
///To use an anchoring option:
///
///1. Identify the location in the document by text string. You can use a pre-existing text string or add a new one.
///For best performance DocuSign recommends using single word anchor strings when possible, especially when there are a large number of pages in the envelope.
///For example, you might want to add a Sign Here tab to the "Borrower's Signature" lines in a document, but that phrase might occur in places in the document where you don't want to tab to appear. In this case, you could add the text "BorrowerSignHere" in white font color (so that isn't visible in the document) to all the places you want Sign Here tabs to appear and use "BorrowerSignHere" as the anchor string.
///1. Reference the anchor through the `anchorString` property of the tab.
///1. Determine the offset from the anchor string location to where the tab should be placed.
///
///Setting a positive value in the `anchorXOffset` property moves the tab right on the page and positive values in the  `anchorYoffset` prove moves the tab down the page. The `anchorUnits` property specifies the units used for the offsets.
///For Sign Here and Initial Here tabs the bottom-left of the anchor string is equivalent to position (0,0), and the bottom-left of the tab graphic is placed relative to that.
///For all other tabs the bottom-left of the anchor string is equivalent to position (0,0), and the top-left of the tab graphic is placed relative to that.
///DocuSign does not currently provide tools to derive the offset values. Determination of the proper offset will likely require some trial-and-error.
///
///### Rules for working with anchor tags
///
///When anchor tabs are used, all documents in the envelope are searched for the `anchorString` property.
///
///* You set the text of the anchor string in the `anchorString` property. DocuSign tabs are created for each instance of the `anchorString` property within the document, so special care must be taken to establish unique anchor strings that do not result in unintentional tabs.
///* You cannot use the same anchored tab for different recipients for the same document.
///* The DocuSign system cannot search for text that is embedded in an image when checking for anchor strings.
///* X or Y offsets supplied for a tab apply to all instances of the tab in the document. To use different offsets at different locations in the document for the same recipient, create multiple, unique anchor tabs.
///* If the Y offset value of an anchor string would force a tab outside of the page boundaries, the tag is placed at the page boundary. If the X offset value places a tab outside of the page boundaries, the error message `Invalid_User_Offset` is sent. The error message includes the X offset that resulted in the error.
///* The system does not support an anchor string embedded in the form of a PDF X-object in the document.
///* The system does not re-flow the text that surrounds the anchor tabs. It is the responsibility of the document author to provide sufficient white space to contain the potential width of the ultimate tab value.
///
///### Tips and Tricks
///
///The following are tips for effective use of anchor tags:
///* In order to avoid unintentional conflicts between text contained in an `anchorString` property and the text that naturally exists in documents, establish a codified syntax for the anchor string text that is unlikely to appear elsewhere in a document.
///* Develop an extensible and consistent syntax that can be used across multiple document types.
///* Especially for documents that have variable numbers of tabs or signers, author the source document to include hidden anchor tabs for all potential signers/permutations. Then, control the tabs that are actually placed by including/excluding the anchor tabs in the request. This approach allows a single document to be used for all use cases instead of maintaining separate documents for each scenario.
///
///## Automatically Populating Tabs
///
///If you want similar tab types
///to automatically populate with the same data,
///you must follow these guidelines:
///
///* Each `tabLabel` entry must have the characters
///  `\\*` in front of the label.
///  If you omit the `\\*` prefix,
///  only the first occurrence of the tab is populated.
///
///  When automatically populating tabs,
///  the `tabLabel` must not contain any spaces.
///  In the JSON example below,
///  the Text tabs  properties `StudentLastName` and `StudentFirstName`
///  will be auto-populated as specified ("Doe" and "John")
///  each place they appear throughout the envelope.
///
///  ```
///  "tabs": {
///    "textTabs": [
///    {
///      "tabLabel": "\\*StudentLastName",
///      "value": "Doe"
///    },
///    {
///      "tabLabel": "\\*StudentFirstName",
///      "value": "John"
///    }]
///  }
///  ```
///
///* Note that `\\*` matches _anything_.  If you were to add
///  another tab with the `tabLabel` set to `\\*Name` to the
///  example above, it would end up matching the other two
///  labels as well.
///
///* Each occurrence of the tab must have identical properties.
///
///  For example, suppose there are two Text tabs in a document,
///  each with `tabLabel` set to `Name`.
///  If one tab has the `bold` property set to **true**,
///  and the other has the `bold` property set to **false**,
///  only the first one will be populated.
///  In order to automatically populate both occurrences
///  of the `Name` Text tabs,
///  the `bold` property must be set to the same value for both tabs.
///.
pub mod envelope_recipient_tabs;
/// The EnvelopeRecipients resource enables you manage the recipients of an envelope. All recipient types share a set of [core parameters](#core-recipient-parameters), but some recipient types have additional parameters. You specify the recipient type using the `recipientType` parameter. The recipient types are as follows:
///
///<br>
///
///| Recipient type | Description |
///| :--- | :--- |
///| [Agent](#agent-recipient) | Agent recipients can add name and email information for recipients that appear after the agent in routing order. |
///| [Carbon Copy](#carbon-copy-recipient) | Carbon copy recipients get a copy of the envelope but don't need to sign, initial, date, or add information to any of the documents. This type of recipient can be used in any routing order. Carbon copy recipients receive their copy of the envelope when the envelope reaches the recipient's order in the process flow and when the envelope is completed. |
///| [Certified Delivery](#certified-delivery-recipient) | Certified delivery recipients must receive the completed documents for the envelope to be completed. However, they don't need to sign, initial, date, or add information to any of the documents. |
///| [Editor](#editor-recipient) | Editors have the same management and access rights for the envelope as the sender. They can make changes to the envelope as if they were using the Advanced Correct feature. This recipient can add name and email information, add or change the routing order, set authentication options, and can edit signature/initial tabs and data fields for the remaining recipients. The recipient must have a DocuSign account to be an editor. |
///| [In-Person Signer](#in-person-signer-recipient) | In-person signer recipients are DocuSign users who act as signing hosts in the same physical location as the signer. |
///| [Intermediary](#intermediary-recipient) | Intermediaries are recipients who can, but are not required to, add name and email information for recipients at the same or subsequent level in the routing order, unless subsequent agents, editors, or intermediaries are added. |
///| [Seal](#seal-recipient) | Electronic seal recipients represent legal entities rather than individuals. Organizations and governments can use electronic seals to show evidence of origin and integrity of documents. |
///| [Signer](#signer-recipient) | Signers are recipients who must sign, initial, date, or add data to form fields on the documents in the envelope. |
///| [Witness](#witness-recipient) | Witnesses are recipients whose signatures affirm that the identified signers have signed the documents in the envelope. |
///
///<br>
///
///Not all recipients are are available to all account types. Review your account plan to determine which recipient types are available to you. All recipient types are available in the Demo environment.
///
///
///## Core Recipient Parameters
///
///All recipients, regardless of type, have the same common core parameters. The following table contains the descriptions for the core properties for all recipient types:
///
///<br>
///
///| Name | Required | Schema Type | Description |
///| :--- | :--- | :--- | :--- |
///| email | Yes | Email | Email of the recipient. Notification will be sent to this email id.<br/>Maximum Length: 100 characters. |
///| name | Yes | String | Full legal name of the recipient.<br/>Maximum Length: 100 characters.<br/><br/>**Note:** If you are creating an envelope with DocuSign EU advanced signature enabled, ensure that recipient names do not contain any of the following characters: **^ : \ @ , + <** |
///| accessCode | No | String | This optional element specifies the access code a recipient has to enter to validate the identity.<br/>Maximum Length: 50 characters. |
///| addAccessCodeToEmail | No | Boolean | This optional attribute indicates that the access code is added to the email sent to the recipient; this nullifies the Security measure of Access Code on the recipient. |
///| agentCanEditEmail | No | Boolean | When set to **true**, the agent recipient associated with this recipient can change the recipient's pre-populated email address. This element is only active if enabled for the account. |
///| agentCanEditName | No | Boolean | When set to **true**, the agent recipient associated with this recipient can change the recipient's pre-populated name (`UserName`). This element is only active if enabled for the account. |
///| clientUserId | No | String | This specifies whether the recipient is embedded or remote.<br/><br/>If the `clientUserId` property is not null then the recipient is embedded. Note that if the `ClientUserId` property is set and either `SignerMustHaveAccount` or `SignerMustLoginToSign` property of the account settings is set to  **true**, an error is generated on sending. Maximum length: 100 characters. |
///| embeddedRecipientStartURL | No | String | This is a sender provided valid URL string for redirecting an embedded recipient. When using this option, the embedded recipient still receives an email from DocuSign, just as a remote recipient would, but when the document link in the email is clicked the recipient is redirected, through DocuSign, to this URL to complete their actions. When routing to the URL, it is up to the sender's system (the server responding to the URL) to then request a recipient token to launch a signing session.<br/><br/>If the value `SIGN_AT_DOCUSIGN` is used for this node, the recipient is directed to an embedded signing or viewing process directly at DocuSign. The signing or viewing action is initiated by the DocuSign system and the transaction activity and Certificate of Completion records will reflect this. In all other ways the process is identical to an embedded signing or viewing operation that would be launched by any partner.<br/><br/>It is important to remember that in a typical embedded workflow the authentication of an embedded recipient is the responsibility of the sending application and DocuSign expects that senders will follow their own process for establishing the recipient's identity. In this workflow the recipient goes through the sending application before the embedded signing or viewing process in initiated. However, when the sending application sets  the `EmbeddedRecipientStartURL` property to `SIGN_AT_DOCUSIGN`, the recipient goes directly to the embedded signing or viewing process bypassing the sending application and any authentication steps the sending application would use. In this case, DocuSign recommends that one of the normal DocuSign authentication features (Access Code, Phone Authentication, SMS Authentication, etc.) be used to verify the identity of the recipient.<br/><br>If the `clientUserId` property is NOT set and the `embeddedRecipientStartURL` property is set, DocuSign ignores the redirect URL and launch the standard signing process for the email recipient. Information can be appended to the `embeddedRecipientStartURL` property using merge fields. The available merge fields items are: envelopeId, recipientId, recipientName, recipientEmail, and customFields. The customFields must be part of the recipient or envelope. The merge fields are enclosed in double brackets.<br/><br/>_Example_:<br/>`http://senderHost/[[mergeField1]]/ beginSigningSession? [[mergeField2]]&[[mergeField3]]` |
///| customFields | No | customField |An optional array of strings that allows the sender to provide custom data about the recipient. This information is returned in the envelope status but otherwise not used by DocuSign. String `customField` properties have a maximum length of 100 characters. |
///| emailNotification | No | emailNotification | An optional complex type that has information for setting the language for the recipient's email information. It is composed of three elements:<br/><br/>*emailBody*: a string with the email message sent to the recipient.<br/>Maximum Length: 10000 characters.<br/><br/>*emailSubject*: a string with the subject of the email sent to the recipient.<br/>Maximum Length: 100 characters.<br/><br/>*supportedLanguage*: The simple type enumeration (two-letter code) for the language to use for the standard email format and the signing view for the recipient. To retrieve the possible values, use the [Accounts::listSupportedLanguages method][ListLang].<br/><br/>**Note**: You can set the `emailNotification` property separately for each recipient. If you set the value only for certain recipients, the other recipients will inherit the this value from the top-level `emailSubject` and `emailBlurb`.  |
///| excludedDocuments | No | Array of Strings | Specifies the documents that are not visible to this recipient. Document Visibility must be enabled for the account and the enforceSignerVisibility property must be set to true for the envelope to use this.<br/><br/>When the enforceSignerVisibility property is set to **true**, documents with tabs can only be viewed by signers that have a tab on that document. Recipients that have an administrative role (Agent, Editor, or Intermediaries) or informational role (Certified Deliveries or Carbon Copies) can always see all the documents in an envelope, unless they are specifically excluded using this setting when an envelope is sent. Documents that do not have tabs are always visible to all recipients, unless they are specifically excluded using this setting when an envelope is sent. |
///| idCheckConfigurationName | No | String |Specifies authentication check by name. The names used here must be the same as the authentication type names used by the account (these name can also be found in the web console sending interface in the Identify list for a recipient). This overrides any default authentication setting.<br/><br/>_Example_:<br/> Your account has ID Check and SMS Authentication available and in the web console Identify list these appear as 'ID Check $' and 'SMS Auth $'. To use ID check in an envelope, the `idCheckConfigurationName` property must be  set to `ID Check $`. To use SMS, it must be set to `SMS Auth $` and you must add phone number information to the `smsAuthentication` node.|
///| iDCheckInformationInput | No | IdCheckInformationInput | This complex element contains input information related to a recipient ID check. It can include the following information.<br/><br/>*addressInformationInput*: Used to set recipient address information and consists of:<br/><br/>*addressInformation*: consists of six elements, with street2 and zipPlus4 being optional. The elements are: street1, street2, city, state, zip, zipPlus4\. The maximum number of characters in each element are: street1/street2 = 150 characters, city = 50 characters, state = 2 characters, and zip/zipPlus4 = 20 characters.<br/><br/>displayLevelCode: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*dobInformationInput*: Used to set recipient date of birth information and consists of:<br/><br/>*dateOfBirth*: Specifies the recipient's date, month and year of birth.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*ssn4InformationInput*: Used to set the last four digits of the recipient's SSN information and consists of:<br/><br/>*ssn4*: Specifies the last four digits of the recipient's SSN.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*ssn9InformationInput*: Used to set the recipient's SSN information. Note that the ssn9 information can never be returned in the response. The ssn9 input consists of:<br/><br/><br/><br/>*ssn9*: Specifies the recipient's SSN.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay. |
///| inheritEmailNotificationConfiguration | No | Boolean | Optional element. If true and the envelope recipient creates a DocuSign account after signing, the Manage Account Email Notification settings are used as the default settings for the recipient's account. |
///| note | No | String | A note that is unique to this recipient. This note is sent to the recipient via the signing email. The note displays in the signing UI near the upper left corner of the document on the signing screen.<br/>Maximum Length: 1000 characters. |
///| phoneAuthentication | No | RecipientPhoneAuthentication | Optional element. Contains the elements: <br/><br/> *recipMayProvideNumber*:Boolean. When set to **true** thenrecipient can use whatever phone number they choose to.<br/><br/> *senderProvidedNumbers*: ArrayOfString. A list of phone numbers the recipient can use. <br/>
///| recipientId | No | String | Unique for the recipient. It is used by the tab element to indicate which recipient is to sign the Document. |
///| requireIdLookup | No | Boolean | When set to **true**, the recipient is required to use the specified ID check method (including Phone and SMS authentication) to validate their identity. |
///| roleName | No* | String | Optional element. Specifies the role name associated with the recipient.<br/><br/>This is required when working with template recipients. |
///| routingOrder | Yes | String | This element specifies the routing order of the recipient in the envelope. |
///| smsAuthentication | No | senderProvidedNumbers | Optional element. Contains the element:<br/><br/>*senderProvidedNumbers*: Array that contains a list of phone numbers the recipient can use for SMS text authentication. |
///| templateAccessCodeRequired | No | Boolean | Optional element. Used only when working with template recipients. When set to **true** and the `TemplateLocked` parameter is set to **true**, the sender must enter an access code. |
///| templateLocked | No | Boolean | Optional element. Used only when working with template recipients. When set to **true**, the sender cannot change any attributes of the recipient. |
///| templateRequired | No | Boolean | Optional element. Used only when working with template recipients. When set to **true**,  the sender may not remove the recipient. |
///| identityVerification <a name="toto"></a> | No | identityVerification | Optional element. Specifies ID Verification applied on an envelope by workflow ID. <br/>See the [list](https://developers.docusign.com/docs/esign-rest-api/reference/accounts/identityverifications/list/) method in the [IdentityVerifications](https://developers.docusign.com/docs/esign-rest-api/reference/accounts/identityverifications/) resource for more information on how to retrieve workflow IDs available for an account. <br/>This can be used in addition to other [recipient authentication](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/recipients/auth/) methods. <br/>Note that ID Verification and ID Check are two distinct methods. ID Verification checks recipients' identity by verifying their ID while ID Check relies on data available on public records (such as current and former address). |
///
///<br>
///
///
///
///## Agent Recipient
///
///An agent recipient can add name and email information for recipients that appear after the agent in routing order.
///
///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
///
///<br>
///
///| Name | Required | Schema Type | Description |
///| :--- | :--- | :--- | :--- |
///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
///
///<br>
///
///
///
///## Carbon Copy Recipient
///
///Carbon copy recipients receive a copy of the envelope but don't need to sign, initial, date or add information to any of the documents. You can place this type of recipient in any routing order. Carbon copy recipients receive their copy of the envelope when the envelope reaches the recipient's order in the process flow and when the envelope is completed.
///
///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
///
///<br>
///
///| Name | Required | Schema Type | Description |
///| :--- | :--- | :--- | :--- |
///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
///
///<br>
///
///
///
///## Certified Delivery Recipient
///
///Certified delivery recipients must receive the completed documents for the envelope to be completed. However, they don't need to sign, initial, date or add information to any of the documents.
///
///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
///
///<br>
///
///| Name | Required | Schema Type | Description |
///| :--- | :--- | :--- | :--- |
///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
///
///<br>
///
///
///
///## Editor Recipient
///
///Editors have the same management and access rights for the envelope as the sender. They can make changes to the envelope as if they were using the Advanced Correct feature. This recipient can add name and email information, add or change the routing order and set authentication options for the remaining recipients. Additionally, this recipient can edit signature/initial tabs and data fields for the remaining recipients. The recipient must have a DocuSign account to be an editor.
///
///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
///
///<br>
///
///
///| Name | Required | Schema Type | Description |
///| :--- | :--- | :--- | :--- |
///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
///
///
///<br>
///
///
///## In-Person Signer Recipient
///
///An in-person recipient is a DocuSign user, acting as a Signing Host, who is in the same physical location as the signer.
///
///The following restrictions apply to using electronic notary when sending documents:
///
///* Authentication methods are allowed for the signer but not the notary.
///* The Sign On Paper, Document Markup, Field Markup and Change Signer options cannot be used for the documents.
///* Tabs may be assigned to the signer, but cannot be assigned to the notary.
///
///See [eNotary Resources][enotary-resources] in the DocuSign Support Center for more information about how the eNotary feature works.
///
///In addition to the [core parameters](#core-recipient-parameters), this type adds the following parameters:
///
///<br>
///
///
///| Name 	| Required 	| Schema Type 	| Description 	|
///|-----------------------	|-----------------------------------------------------	|-------------	|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------	|
///| inPersonSigningType 	| No 	| String 	| Specifies whether the envelope uses the eNotary feature. The accepted values are:<ul markdown=1><li>`inPersonSigner` The envelope uses the normal in-person signing flow.</li><li>`notary`: The envelope uses the eNotary signing flow.</li></ul> 	|
///| notaryHost 	| Yes, when `inPersonSigningType` is `notary` 	| NotaryHost 	| Sets the information for the notary host for the notary in person signing flow. The following information is required: <ul markdown=1> <li>`recipientId`: A unique ID number for the notary signing host. </li> <li>`name`: Specifies the notary's full legal name.</li> <li>`email`: Specifies the notary's email address. </li> </ul> 	|
///| autoNavigation 	| No 	| Boolean 	| Specifies whether auto navigation is set for the recipient. 	|
///| defaultRecipient 	| No 	| Boolean 	| When set to **true**, this is the default recipient for the envelope. This option is used when creating an envelope from a template. 	|
///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
///| hostName 	| Yes, when `inPersonSigningType` is `inPersonSigner` 	| String 	| The name of the signing host. This is the DocuSign user that is hosting the in-person signing session. 	|
///| hostEmail 	| Yes, when `inPersonSigningType` is `inPersonSigner` 	| String 	| The email address of the signing host. This is the DocuSign user that is hosting the in-person signing session. 	|
///| signerName 	| Yes, when `inPersonSigningType` is `inPersonSigner` 	| String 	| The in-person signer's full legal name. 	|
///| Name 	| Yes, when `inPersonSigningType` is `notary` 	| String 	| The full legal name of the signer in an eNotary flow. 	|
///| email 	| Yes, when `inPersonSigningType` is `notary` 	| String 	| The signer's email address in an eNotary flow. 	|
///| recipientSuppliesTabs 	| No 	| Boolean 	| Indicates whether the recipient supplies tabs in the document. 	|
///| signatureInfo 	| No 	| String 	| Optional element only used with the recipient types In Person Signers, Signers, and Witnesses.<br/><br/>Allows the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. 	|
///| signInEachLocation 	| No 	| Boolean 	| When set to **true** and the feature is enabled in the sender's account, the signing recipient is required to draw signatures and initials at each signature/initial tab (instead of adopting a signature/initial style or only drawing a signature/initial once). 	|
///| tabs 	| No 	| Tab 	| Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Specifies the Tabs associated with the recipient. See the [EnvelopeRecipientTabs resource](https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/) for more information about tabs. 	|
///|  	|  	|  	|  	|
///
///
///<br>
///
///
///
///## Intermediary Recipient
///
///An intermediary is a recipient who can, but is not required to, add name and email information for recipients at the same or subsequent level in the routing order, unless subsequent agents, editors or intermediaries are added.
///
///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
///
///<br>
///
///
///| Name | Required | Schema Type | Description |
///| :--- | :--- | :--- | :--- |
///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
///
///<br>
///
///
///
///## Seal Recipient
///
///An electronic seal recipient is a legal entity rather than an actual person. Electronic Seals can be used by organizations and governments to show evidence of origin and integrity of documents. Even though electronic seals can be represented by a tab in a document, they do not require user interaction and apply automatically in the order specified by the sender. The sender is therefore the person authorizing usage of the electronic seal in the flow.
///
///Electronic seal recipients rely on a subset of core properties, described as follows, plus the `recipientSignatureProviders` parameter:
///
///
///<br>
///
///
///| Name&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Required&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Schema Type&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Description&nbsp;&nbsp;&nbsp;&nbsp; |
///| :--- | :--- | :--- | :--- |
///| recipientId| Yes | String | Indicates the unique ID of the applied electronic seal.<br/>|
///| routingOrder| No (default: 1) | String | Specifies the routing order of the electronic seal in the envelope. <br />The routing order assigned to your electronic seal cannot be shared with another recipient. It is recommended that you set a routing order for your electronic seals.. |
///| recipientSignatureProviders| Yes | String | Indicates which electronic seal to apply on documents when creating an envelope. |
///
///
///<br>
///
///
///
///By default, Electronic Seals apply on all documents in an envelope. However, the sealDocumentsWithTabsOnly property (see recipientSignatureProvider) allows you to seal only documents that have signHere tabs set for the Electronic Seal recipients.
///
///<br>
///
///
///
///## Signer Recipient
///
///A signer is a recipient who must sign, initial, date, or add data to form fields on the documents in the envelope.
///
///In addition to the [core parameters](#core-recipient-parameters), this recipient type adds the following parameters:
///
///<br>
///
///
///| Name | Required | Schema Type | Description |
///| :--- | :--- | :--- | :--- |
///| autoNavigation | No | Boolean | Specifies whether auto navigation is set for the recipient.|
///| defaultRecipient | No | Boolean | When set to **true**, this is the default recipient for the envelope. This option is used with the CreateEnvelopeFromTemplatesAndForms method. |
///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
///| isBulkRecipient | No | Boolean | Indicates whether the recipient is a bulk send recipient or not. |
///| recipientSuppliesTabs | No | Boolean | Indicates whether the recipient supplies tabs in the document. |
///| signInEachLocation | No | Boolean | When set to **true** and the feature is enabled in the sender's account, the signing recipient is required to draw signatures and initials at each signature/initial tab (instead of adopting a signature/initial style or only drawing a signature/initial once). |
///| signatureInfo | No | String | Optional element only used with recipient types In Person Signers, Signers, and Witnesses.<br/><br/>Allows the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. |
///| signerEmail | No | String | Optional element. The email address for an In-Person Signer recipient Type.<br/>Maximum Length: 100 characters. |
///| signerName | Yes | String | Required element with recipient type In Person Signers.<br/>Maximum Length: 100 characters.<br/><br/>The full legal name of a signer for the envelope. |
///| tabs | No | Tab | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Specifies the Tabs associated with the recipient. See the the [EnvelopeRecipientTabs resource](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/enveloperecipienttabs/)  for more information about tabs. |
///
///<br>
///
///[enotary-resources]: https://support.docusign.com/en/guides/ndse-user-guide-enotary-resources
///
///
///## Witness Recipient
///
///A witness is a recipient whose signature affirms that the identified signers have signed the documents in the envelope.
///
///In addition to the [core parameters](#core-recipient-parameters), this recipient type adds the following parameters:
///
///<br>
///
///
///| Name | Required | Schema Type | Description |
///| :--- | :--- | :--- | :--- |
///| autoNavigation | No | Boolean |	Specifies whether auto navigation is set for the recipient. |
///| defaultRecipient | No | Boolean | When set to **true**, this is the default recipient for the envelope. This option is used when creating an envelope from a template. |
///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
///| isBulkRecipient | No | Boolean | Indicates whether the recipient is a bulk send recipient or not. |
///| recipientSignatureProviders | Yes | String | Indicates which electronic seal to apply on documents when creating an envelope. |
///| recipientSuppliesTabs | No | Boolean | Indicates whether the recipient supplies tabs in the document. |
///| recipientType | Yes | String | Indicates the recipient type. |
///|requireSignerCertificate | No | Boolean | Indicates whether the envelope requires a signer certificate for this recipient. |
///| requireSignOnPaper | No | Boolean | Specifies whether the signer must print, sign, and upload or fax the signed documents to DocuSign. |
///| signatureInfo | No | Boolean | Optional element only used with recipient types In Person Signers, Signers, and Witnesses. Enables the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. |
///| signInEachLocation | No | Boolean | When set to **true** and the feature is enabled in the sender's account, specifies that the signing recipient is required to sign and initial at each signature/initial tab (instead of once). |
///| signingGroupId | No | String | The id of the [signing group](https://support.docusign.com/en/guides/ndse-user-guide-signing-groups). |
///| signingGroupName | No | String | The display name for the signing group. Maximum Length: 100 characters. |
///| signingGroupUsers | No | userInfo | A complex type that contains information about the users in the signing group. |
///| witnessFor | Yes | String | Indicates the person or party for whom the recipient is a witness. |
///| witnessForGuid | Yes | String | GUID identifying the person or party for whom the recipient is a witness. |
///
///
///[ListLang]: https://developers.docusign.com/docs/esign-rest-api/reference/Accounts/Accounts/listSupportedLanguages/
///.
pub mod envelope_recipients;
/// The EnvelopeTemplates resource provides methods that allow you to add and delete templates on envelopes and documents.
pub mod envelope_templates;
/// Envelope transfer rules enable administrators to transfer envelopes and templates from user to another. For example, you might do this when an employee leaves the company. To transfer ownership of envelopes and templates, the **Transfer Custody** feature must be enabled for your account.
///
///For more information about this functionality, see [Transfer Envelopes and Templates](https://support.docusign.com/en/guides/ndse-admin-guide-transfer-envelopes-templates).
pub mod envelope_transfer_rules;
/// The `EnvelopeViews` resource provides methods that return URLs that you can embed into your application to provide access to the DocuSign UI.
///
///The following Envelope Views are available:
///
///-  Console View: The authentication view of the DocuSign UI.
///-  Correct View: The correction view of the DocuSign UI.
///-  Edit View: The editing view of the DocuSign UI.
///   **Note**: This provides the same functionality as the sender view.
///-  Recipient View: The view the recipient sees in the DocuSign UI.
///-  Shared Recipient View: The view a user sees of a shared envelope in the DocuSign UI.
///-  Sender View: The sending view of the DocuSign UI.
pub mod envelope_views;
/// .
pub mod envelope_workflow_definition;
/// The Envelope resource provides methods that allow you to manipulate and monitor envelopes.
///
///Once you have authenticated the user you can use the Envelopes: createEnvelope method to create an envelope. You can call the EnvelopeDocuments: update or EnvelopeDocuments: updateList method to add additional documents.
///
///Setting the `status` property on the envelope to `sent` allows you to send it or `created` to save it as a draft.
///
///You can receive envelope event notifications by setting the `eventNotification` properties. When the envelope or recipient status changes to one of the specified status codes, a notification is sent to a URL that you specify.
///
///If you have an envelope that you have previously saved, you can modify the subject and message, send it, void it, or place it in the purge queue using the Envelope: update method.
///
///In addition to receiving notifications you can monitor the status of the envelopes using the following methods:
///* Envelope: getEnvelope - To get the status of a envelope.
///* Envelope: listStatus - To get the envelope status for a set of envelopes.
///* Envelope: listStatusChanges - To get status changes information for one or more envelopes.
///
///If you need to delete a page from a document in an envelope, use the Envelope: deleteDocumentPage method.
///
///The resource also includes a number of methods that allow you to retrieve and set the initials and signature for certain types of recipients on the document.
pub mod envelopes;
/// .
pub mod favorite_templates;
/// The Folders resource provides methods that allow you to view contents of folders on the account and move envelopes between folders.
pub mod folders;
/// For the custom groups you define for your account, you can assign brands to specify the ones that group members can use. Group members can use the available brands when they send envelopes or create templates. For more information, see [Assign Brands to Groups](https://support.docusign.com/en/guides/ndse-admin-guide-assign-brands-to-groups).
pub mod group_brands;
/// The GroupUsers resource provides methods that allow you to manage the users in a group.
pub mod group_users;
/// The Groups resource provides methods that allow you to manage groups for the account.
///
///Groups can be used to help manage users by associating users with a group. A group can be associated with a Permission Profile, which sets the user permissions for users in that group without having to set the `userSettings` property for each user. You are not required to set Permission Profiles for a group, but this makes it easier to manage user permissions for a large number of users. Groups can also be used with template sharing to limit user access to templates.
pub mod groups;
/// The DocuSign Identity Verification process requires a signer to submit an image of their valid government ID and wait for the image to be uploaded and verified before they can access a document.
///
///Identity Verification supports government photo IDs and European eIDs, analyzing the document security features and matching the name on the agreement against the name on the ID. After a successful verification, the signer can view the agreement and sign as usual.
///
///To use Identity Verification, the [DocuSign Identity Verification](https://www.docusign.com/products/identify) product must be enabled for your account.
///
///For more information, see [Can I see (a photo of) your ID? Digital verification of real-world IDs](https://www.docusign.com/blog/can-i-see-a-photo-of-your-id-digital-verification-of-real-world-ids/).
pub mod identity_verifications;
/// The Invoices resource provides methods that allow you to manage the invoices for an account.
pub mod invoices;
/// DocuSign eNotary makes the notarization process fully digital
///for senders, signers, and notaries.
///It enables a notary public to act as an in-person witness
///to electronic signing of documents.
///Check the [DocuSign eNotary support documentation](https://support.docusign.com/en/guides/ndse-user-guide-enotary-resources)
///to see which jurisdictions are supported.
///
///.
pub mod notary;
/// .
pub mod notary_journals;
/// Creating, updating, and deleting notary jurisdiction objects.
pub mod notary_jurisdiction;
/// .
pub mod payment_gateway_accounts;
/// The Payments resource provides methods that allow you to manage payments for an account.
///
///These calls can only be used by users with account administrator privileges.
pub mod payments;
/// The PowerFormData resource provides a method for accessing PowerForm data.
///.
pub mod power_form_data;
/// PowerForms enable you to create self-service documents for signature. A PowerForm is an envelope initiated from a URL that you make available for signers to complete. You can either add a PowerForm to your website or email the URL to recipients. DocuSign saves the data that recipients enter so  you can easily integrate it with your other applications.
///
///For more information, see [Using PowerForms](https://support.docusign.com/en/guides/ndse-user-guide-using-powerforms).
///
///**Note**: PowerForms are available only for DocuSign Enterprise accounts.
///
///
///### Errors
///
///PowerForm methods return the following 404 errors:
///
///- `PowerForms_Recipient_Denied_Documents`: The recipient is denied access to the documents.
///- `PowerForms_DigitalCerts_Shared_Tabs_Not_Allowed`: Shared tags are not allowed because a digital certificate is required
///for a signer.
///- `PowerForms_DigitalCerts_Free_Form_Tabs_Not_Allowed`: Signers that are required to use a digital certificate must have at
///least one required, non-conditional signature or initials tab.
///- `PowerForms_DigitalCerts_Multiple_Recipients_Routing_Order`: Signers that are required to use a digital certificate must be the
///only recipient in a routing order. Edit the routing order or remove the digital certificate requirement.
///- `PowerForms_DigitalCerts_Markup_Not_Allowed`: Document markup is not allowed because a digital certificate is
///required for a signer.
///- `PowerForms_Incomplete_Recipient`: The recipient's username, email, or role is not set.
///- `PowerForms_PowerFormId_Required`: A `powerFormId` is required.
///- `PowerForms_PowerFormId_Mismatch`: A `powerFormId` mismatch has occurred.
///.
pub mod power_forms;
/// .
pub mod reports;
/// The RequestLogs resource provide methods that allow you to retrieve and delete the API request log files.
///
///The log files contain the API requests associated with your integration. They can aid you in troubleshooting specific issues within an integration, or if DocuSign Support requests an API trace log.
pub mod request_logs;
/// The Resources resource provides a method which retrieves the base resources that are available.
pub mod resources;
/// **Note**: Responsive Signing is disabled by default. To use this functionality, an account administrator must switch the account setting `enableResponsiveSigning` to **true**.
///Also note that Smart Sections (creating a signable HTML document that uses collapsible sections and rotating tables) are premium features. To request them, contact your DocuSign account manager.
pub mod responsive_html_preview;
/// The Services resource provides a method that allow you to retrieve the available service versions.
pub mod services;
/// The SigningGroupUsers resource provides methods that allow you to manage users in Signing Groups.
pub mod signing_group_users;
/// The SigningGroups resource provides methods that allow you manage signing groups.
///
///Signing Groups allow you to create a group of people to which an envelope is sent. Any member of that group can open an envelope and sign the documents in the envelope with their own signature, even though a signature field was not directly assigned to them. When the Signing Group option is used, group members that open and sign the envelope are tracked in the envelope history and certificate.
///
///When one group member opens the envelope, it is temporarily locked and if other members try to open the envelope they will see a message saying the envelope is currently opened. If the group member exits the envelope without finishing the lock expires, allowing other group members to open and complete the envelope.
///
///When the envelope is complete, all members of the group will receive a completed notification and can access the completed envelope.
///The envelope history and Certificate of Completion will show that the envelope was sent to a signing group and record which members viewed and signed the envelope.
///
///An account can have a maximum of 50 signing groups. Each signing group can have a maximum of 50 group members.
///
///The Signing Groups feature is only supported in certain DocuSign Enterprise and System Automated Premium plans. Your account might not support this option. To access this functionality, contact your Account Manager or DocuSign Support (support@docusign.com) for assistance.
///
///For more information, see [Signing Groups](https://support.docusign.com/en/guides/ndse-user-guide-signing-groups).
pub mod signing_groups;
/// .
pub mod tabs_blob;
/// The TemplateBulkRecipients resource provide methods that allow you manage the bulk recipient file for an template.
///
///The bulk recipient CSV (Comma Separated Value) file contains the list of recipient names and email addresses that you can add to an template to send the same document to a large number of recipients.
///
///The required and optional information that can be included the file is described in the BulkEnvelope: updateRecipients method.
pub mod template_bulk_recipients;
/// The TemplateCustomFields resource provides methods that allow you manage custom fields in an template.
///
///Custom fields can be used in the templates for your account to record information about the template, help search for templates and track information. The template custom fields are shown in the Template Settings section when a user is creating an template in the DocuSign member console. The template custom fields are not seen by the template recipients.
///
///There are two types of custom fields:
///
///- `text`: Enables the sender to enter the value for the field.
///- `list`: Enables the sender to select the value of the field from a predetermined list.
pub mod template_custom_fields;
/// The TemplateDocumentFields resource provides methods that allow you to manage custom fields on a document.
///
///You can create custom versions of standard fields that combine of field properties, such as font type or size, or a validation setting.
///
///Note: Some advanced features and options are supported only in certain DocuSign plans. Your account plan might not support some options discussed in this help topic. For more information about which options are available for your account, check your account plan or contact your Account Manager.
pub mod template_document_fields;
/// .
pub mod template_document_html_definitions;
/// **Note**: Responsive Signing is disabled by default. To use this functionality, an account administrator must switch the account setting `enableResponsiveSigning` to **true**.
///Also note that Smart Sections (creating a signable HTML document that uses collapsible sections and rotating tables) are premium features. To request them, contact your DocuSign account manager.
pub mod template_document_responsive_html_preview;
/// The EnvelopeDocumentTabs resource provides methods that enable you to manage tabs in a template.
pub mod template_document_tabs;
/// The Template Document Visibility resource provides methods for managing template document visibility by recipient.
///.
pub mod template_document_visibility;
/// <!-- resources aren't rendered the same way
///     as other pages. This is a little hack to
///     make the headings work better -->
///<style>
///h1, h2, h3 {
///  margin-top: 1em;
///}
///</style>
///
///The TemplateDocuments resource provides methods
///that manage documents in a template.
///You can:
///* Add one or more documents to the template
///* Retrieve one or more documents from the template
///* Delete documents from the template
///
///All of the methods associated with this resource
///operate on an existing template.
///Before you can add documents
///to a template,
///you must first create it
///with the [Templates:: Create][templatescreate] method.
///
///[templatescreate]: https://developers.docusign.com/docs/esign-rest-api/reference/Templates/Templates/create/
///.
pub mod template_documents;
/// .
pub mod template_html_definitions;
/// The TemplateLocks resource provides methods that allow you to
///manage locks on a template.
///
///To prevent users from changing a template while another user is
///modifying it, you can lock the template and set the time until
///the lock expires.
///
///For example, you would use the following flow:
///
///1. Lock the template.
///2. Make changes to template.
///3. Delete the template lock and save the changes. If the template has a password, you must supply the template password to save the changes.
///
///**Note**: To use template locks, the user must have envelope locking capability enabled.
pub mod template_locks;
/// <!-- resources aren't rendered the same way
///     as other pages. This is a little hack to
///     make the headings work better -->
///<style>
///h1, h2, h3 {
///  margin-top: 1em;
///}
///</style>
///
///The TemplateRecipientTabs resource provides methods that let you
///add,
///update,
///and delete tabs
///from an envelope.
///Tabs are associated with a specific recipient
///in an envelope
///and are only used by the recipient types
///In Person Signers and Signers.
///
///
///## Tab Types
///
///This table lists the available tab types.
///
///<br>
///
///
///| Tab Type              | Description |
///| :-------              | :---------- |
///| Approve Tab           | Place this tab on the document if you want the recipient to approve documents in an envelope without placing a signature or initials on the document. If the recipient clicks the Approve tab during the signing process, the recipient is considered to have signed the document. No information is shown on the document for the approval, but it is recorded as a signature in the envelope history. |
///| Checkbox Tab          | Place this tab on the document in a location where the recipient can select a yes/no (on/off) type option. |
///| Company Tab           | Place this tab on the document where you want the recipient's company name to appear. |
///| Date Signed Tab       | Place this tab on the document where you want the date the recipient signed the document to appear. |
///| Date Tab              | Place this tab on the document where you want the recipient to enter a date. Date tabs are single-line fields that allow date information to be entered in any format. The tooltip for this tab recommends entering the date as MM/DD/YYYY, but this is not enforced. The format entered by the signer is retained. If you need a particular date format enforced, DocuSign recommends using a Text tab with a Validation Pattern and Validation Message to enforce the format. |
///| Decline Tab           | Place this tab on the document where you want to give the recipient the option of declining an envelope. If the recipient clicks the Decline tab during the signing process, the envelope is voided. |
///| Email Address Tab     | Place this tab on a document where you want the recipient's email, as entered in the recipient information, to appear. |
///| Email Tab             | This is a single-line field that accepts all characters. |
///| Envelope ID Tab       | Place this tab on the document where you want the envelope ID for to appear. Recipients cannot enter or change the information in this tab. It is for informational purposes only. |
///| First Name Tab        | Place this tab on a document where you want the recipient's first name to appear. This tab splits the recipient's name (as entered in the recipient information) into sections based on spaces and uses the first section as the first name. |
///| Formula Tab           | This tab is used to add a calculated field to a document. Envelope recipients cannot directly enter information into the tab. The formula tab calculates and displays a new value when changes are made to the reference tab values. The reference tab information and calculation operations are entered in the "formula" element. See the Using the Calculated Fields Feature quick start guide or DocuSign Service User Guide for more information about formulas.  |
///| Full Name Tab         | Place this tab on the document where you want the recipient's full name to appear. |
///| Initial Here Tab      | Place this tab where the recipient must initial the document. This tab can be set to be optional.  |
///| Last Name Tab         | Place this tab on a document where you want the recipient's last name to appear. This tab splits the recipient's name (as entered in the recipient information) into sections based on spaces and uses the last section as the last name. |
///| List Tab              | This tab has a list of options that a recipient can select. The `listItems` parameter is used to set the options that can be selected.  |
///| Note Tab              | Place this tab on the document where you want to add a note for the recipient on a document. |
///| Number Tab            | This tab is a field where the recipient can enter numbers and decimal (.) points.  |
///| Radio Group Tab       | This group tab is used to place radio buttons on a document. The `radios` parameter is used to add and place the radio buttons associated with the group. Only one radio button can be selected in a group.  |
///| Sign Here Tab         | Place this tab where the recipient must sign the document. This tab can be set to be optional.  |
///| Signer Attachment Tab | The signer attachment is where the recipient initiates the process of adding supporting documents to an envelope. |
///| SSN Tab               | This tab is a single-line field where the recipient can enter numbers. A Social Security Number can be typed with or without dashes. |
///| Text Tab              | This tab is a field where the recipient can enter any type of characters.  |
///| Title Tab             | Place this tab on the document where you want the recipient's title to appear. |
///| Zip Tab               | This tab is a single-line field where the recipient can enter numbers. |
///
///
///## Using Custom Tabs in Envelopes and Templates
///
///Custom Tabs can be added to envelopes and templates
///by setting the `customTabId` property
///when creating an envelope or template recipient
///or when adding a new tab for an existing recipient.
///The custom tab must be added as the correct tab type.
///For example if the custom tab type is text, it cannot be used as a number tab.
///
///When the `customTabId` property is set,
///the new tab inherits all the custom tab properties.
///Required information that is not included in the custom tab,
///such as document ID and page ID, must be included when adding the tab.
///If the custom tab does not have anchor settings, the X and Y positions must be included.
///
///After the tab is created,
///it is treated as any other tab for updating or deleting.
///
///## Anchoring Tabs
///
///The tab anchoring option
///allows you to send documents for signature
///that do not have a fixed layout or format.
///In these documents you might not know
///the absolute location of the tabs
///when you design your API client application because the tabs must move with text.
///As an alternative to sending X and Y coordinates for tabs,
///the DocuSign Service can derive an anchor location for the tab
///by correlating anchor information to data within the document.
///
///When the DocuSign Service receives a request that contains tabs
///with anchor information,
///it searches the document for instances of the `anchorString` property.
///When found,
///it places a tab of the specified type for the designated recipient.
///Tab positions are established by setting offsets for the tab.
///
///When you apply tabs to the document,
///DocuSign does not remove or replace the text in the `anchorString` property. You can hide codified anchors by using the same font color as the background of the document. So the anchor can be used by DocuSign processes and it will not be visible on the document.
///
///To use an anchoring option:
///
///1. Identify the location in the document by text string. You can use a pre-existing text string or add a new one.
///For best performance DocuSign recommends using single word anchor strings when possible, especially when there are a large number of pages in the envelope.
///For example, you might want to add a Sign Here tab to the "Borrower's Signature" lines in a document, but that phrase might occur in places in the document where you don't want to tab to appear. In this case, you could add the text "BorrowerSignHere" in white font color (so that isn't visible in the document) to all the places you want Sign Here tabs to appear and use "BorrowerSignHere" as the anchor string.
///1. Reference the anchor through the `anchorString` property of the tab.
///1. Determine the offset from the anchor string location to where the tab should be placed.
///
///Setting a positive value in the `anchorXOffset` property moves the tab right on the page and positive values in the  `anchorYoffset` prove moves the tab down the page. The `anchorUnits` property specifies the units used for the offsets.
///For Sign Here and Initial Here tabs the bottom-left of the anchor string is equivalent to position (0,0), and the bottom-left of the tab graphic is placed relative to that.
///For all other tabs the bottom-left of the anchor string is equivalent to position (0,0), and the top-left of the tab graphic is placed relative to that.
///DocuSign does not currently provide tools to derive the offset values. Determination of the proper offset will likely require some trial-and-error.
///
///### Rules for working with anchor tags
///
///When anchor tabs are used, all documents in the envelope are searched for the `anchorString` property.
///
///* You set the text of the anchor string in the `anchorString` property. DocuSign tabs are created for each instance of the `anchorString` property within the document, so special care must be taken to establish unique anchor strings that do not result in unintentional tabs.
///* You cannot use the same anchored tab for different recipients for the same document.
///* The DocuSign system cannot search for text that is embedded in an image when checking for anchor strings.
///* X or Y offsets supplied for a tab apply to all instances of the tab in the document. To use different offsets at different locations in the document for the same recipient, create multiple, unique anchor tabs.
///* If the Y offset value of an anchor string would force a tab outside of the page boundaries, the tag is placed at the page boundary. If the X offset value places a tab outside of the page boundaries, the error message `Invalid_User_Offset` is sent. The error message includes the X offset that resulted in the error.
///* The system does not support an anchor string embedded in the form of a PDF X-object in the document.
///* The system does not re-flow the text that surrounds the anchor tabs. It is the responsibility of the document author to provide sufficient white space to contain the potential width of the ultimate tab value.
///
///### Tips and Tricks
///
///The following are tips for effective use of anchor tags:
///* In order to avoid unintentional conflicts between text contained in an `anchorString` property and the text that naturally exists in documents, establish a codified syntax for the anchor string text that is unlikely to appear elsewhere in a document.
///* Develop an extensible and consistent syntax that can be used across multiple document types.
///* Especially for documents that have variable numbers of tabs or signers, author the source document to include hidden anchor tabs for all potential signers/permutations. Then, control the tabs that are actually placed by including/excluding the anchor tabs in the request. This approach allows a single document to be used for all use cases instead of maintaining separate documents for each scenario.
///
///## Automatically Populating Tabs
///
///If you want similar tab types
///to automatically populate with the same data,
///you must follow these guidelines:
///
///* Each `tabLabel` entry must have the characters
///  `\\*` in front of the label.
///  If you omit the `\\*` prefix,
///  only the first occurrence of the tab is populated.
///
///  When automatically populating tabs,
///  the `tabLabel` must not contain any spaces.
///  In the JSON example below,
///  the Text tabs  properties `StudentLastName` and `StudentFirstName`
///  will be auto-populated as specified ("Doe" and "John")
///  each place they appear throughout the envelope.
///
///  ```
///  "tabs": {
///    "textTabs": [
///    {
///      "tabLabel": "\\*StudentLastName",
///      "value": "Doe"
///    },
///    {
///      "tabLabel": "\\*StudentFirstName",
///      "value": "John"
///    }]
///  }
///  ```
///
///* Each occurrence of the tab must have identical properties.
///
///  For example, suppose there are two Text tabs in a document,
///  each with `tabLabel` set to `Name`.
///  If one tab has the `bold` property set to **true**,
///  and the other has the `bold` property set to **false**,
///  only the first one will be populated.
///  In order to automatically populate both occurrences
///  of the `Name` Text tabs,
///  the `bold` property must be set to the same value for both tabs.
///.
pub mod template_recipient_tabs;
/// The TemplateRecipients resource allows you manage the recipients of an template.
///
///The exact parameters associated with a recipient depend on the recipient type. There are seven recipient types: Agents, Carbon Copies, Certified Deliveries, Editors, In Person Signers, Intermediaries, and Signers.
///
///Not all recipients are are available to all account types, review you account plan to determine which recipient types are available to you. If you are working in the Demo environment, all recipient types are available.
///
///Each recipient type is described below:
///
///[Agents](#agents): This recipient can add name and email information for recipients that appear after the recipient in routing order.
///
///[Carbon Copies](#carboncopies): Use this action if the recipient should get a copy of the template, but the recipient does not need to sign, initial, date or add information to any of the documents. This type of recipient can be placed in any order in the recipient list. The recipient receives a copy of the template when the template reaches the recipient's order in the process flow and when the template is completed.
///
///[Certified Deliveries](#certifiedDeliveries): Use this action if the recipient must receive the completed documents for the template to be completed, but the recipient does not need to sign, initial, date or add information to any of the documents.
///
///[Editors](#editors): This recipient has the same management and access rights for the template as the sender and can make changes to the template as if they were using the Advanced Correct feature. This recipient can add name and email information, add or change the routing order and set authentication options for the remaining recipients. Additionally, this recipient can edit signature/initial tabs and data fields for the remaining recipients. The recipient must have a DocuSign account to be an editor.
///
///[In Person Signers](#inPerson): Use this action if the signer is in the same physical location as a DocuSign user who will act as a Signing Host for the transaction. The recipient added is the Signing Host and new separate Signer Name field appears after Sign in person is selected.
///
///[Intermediaries](#intermediaries): This recipient can, but is not required to, add name and email information for recipients at the same or subsequent level in the routing order (until subsequent Agents, Editors or Intermediaries recipient types are added).
///
///[Signers](#signers): Use this action if your recipient must sign, initial, date or add data to form fields on the documents in the template.
///
///### Core JSON Layout
///
///The following shows the core JSON layout for a recipient.
///
///```
/// "email": "email.name@company.com",
///  "name": "recipient name",
///  "accessCode": "",
///  "addAccessCodeToEmail": false,
///  "clientUserIs": null,
///  "embeddedRecipientStartURL": "string",
///  "customFields": {
///    "sample string 1",
///    "sample string 2"
///  },
///  "emailNotification"{
///    "emailBody":"email text",
///    "emailSubject":"Subject text",
///    "supportedLanguage":"en",
///  },
///  "excludedDocuments": ["2", "4"],
///  "idCheckConfigurationName": null,
///  "idCheckInformationInput": {
///    "addressInformationInput": {
///      "addressInformation": {
///        "street1": "sample string 1",
///        "street2": "sample string 2",
///        "city": "sample string 3",
///        "state": "sample string 4",
///        "zip": "sample string 5",
///        "zipPlus4": "sample string 6"
///      },
///      "displayLevelCode": "sample string 1",
///      "receiveInResponse": "sample string 2"
///    },
///    "dobInformationInput": {
///      "dateOfBirth": "sample string 1",
///      "displayLevelCode": "sample string 2",
///      "receiveInResponse": "sample string 3"
///    },
///    "ssn4InformationInput": {
///      "ssn4": "sample string 1",
///      "displayLevelCode": "sample string 2",
///      "receiveInResponse": "sample string 3"
///    },
///    "ssn9InformationInput": {
///      "ssn9": "sample string 1",
///      "displayLevelCode": "sample string 2"
///    }
///  },
///  "inheritEmailNotificationConfiguration": false,
///  "note": "",
///  "phoneAuthentication": {
///    "recipMayProvideNumber": "sample string 1",
///    "validateRecipProvidedNumber": "sample string 2",
///    "recordVoicePrint": "sample string 3",
///    "senderProvidedNumbers": [
///      "sample string 1",
///      "sample string 2"
///    ]
///  },
///  "recipientAttachment": null,
///  "recipientCaptiveInfo": null,
///  "recipientId": "1",
///  "requireIdLookup": false,
///  "roleName": "",
///  "routingOrder": 1,
///  "samlAuthentication": {
///    "samlAssertionAttributes": [
///      {
///        "name": "string",
///        "value": "string"
///      },
///      {
///        "name": "string",
///        "value": "string"
///      }
///    ]
///  },
///  "smsAuthentication": {
///    "senderProvidedNumbers":[
///      "sample string 1",
///      "sample string 2"
///    ]
///  },
///  "socialAuthentications": null,
///  "templateAccessCodeRequired": false,
///  "templateLocked": false,
///  "templateRequired": false,
/// ...
/// ```
///
///### Core Recipient Parameters
///
///The following table contains the descriptions for the core properties for a recipient.
///
///| Name | Required? | Schema Type | Description |
///| --- | --- | --- | --- |
///| email | Yes | Email | Email of the recipient. Notification will be sent to this email id.<br/>Maximum Length: 100 characters. |
///| name | Yes | String | Full legal name of the recipient.<br/>Maximum Length: 100 characters. |
///| accessCode | No | String | This optional element specifies the access code a recipient has to enter to validate the identity.<br/>Maximum Length: 50 characters. |
///| addAccessCodeToEmail | No | Boolean | This optional attribute indicates that the access code is added to the email sent to the recipient; this nullifies the Security measure of Access Code on the recipient. |
///| clientUserId | No | String | This specifies whether the recipient is embedded or remote.<br/><br/>If the `clientUserId` property is not null then the recipient is embedded. Note that if the `ClientUserId` property is set and either `SignerMustHaveAccount` or `SignerMustLoginToSign` property of the account settings is set to  **true**, an error is generated on sending. |
///| embeddedRecipientStartURL | No | String | This is a sender provided valid URL string for redirecting an embedded recipient. When using this option, the embedded recipient still receives an email from DocuSign, just as a remote recipient would, but when the document link in the email is clicked the recipient is redirected, through DocuSign, to this URL to complete their actions. When routing to the URL, it is up to the sender's system (the server responding to the URL) to then request a recipient token to launch a signing session.<br/><br/>If the value `SIGN_AT_DOCUSIGN` is used for this node, the recipient is directed to an embedded signing or viewing process directly at DocuSign. The signing or viewing action is initiated by the DocuSign system and the transaction activity and Certificate of Completion records will reflect this. In all other ways the process is identical to an embedded signing or viewing operation that would be launched by any partner.<br/><br/>It is important to remember that in a typical embedded workflow the authentication of an embedded recipient is the responsibility of the sending application and DocuSign expects that senders will follow their own process for establishing the recipient's identity. In this workflow the recipient goes through the sending application before the embedded signing or viewing process in initiated. However, when the sending application sets  the `EmbeddedRecipientStartURL` property to `SIGN_AT_DOCUSIGN`, the recipient goes directly to the embedded signing or viewing process bypassing the sending application and any authentication steps the sending application would use. In this case, DocuSign recommends that one of the normal DocuSign authentication features (Access Code, Phone Authentication, SMS Authentication, etc.) be used to verify the identity of the recipient.<br/><br>If the `clientUserId` property is NOT set and the `embeddedRecipientStartURL` property is set, DocuSign ignores the redirect URL and launch the standard signing process for the email recipient. Information can be appended to the `embeddedRecipientStartURL` property using merge fields. The available merge fields items are: templateId, recipientId, recipientName, recipientEmail, and customFields. The customFields must be part of the recipient or template. The merge fields are enclosed in double brackets.<br/><br/>_Example_:<br/>`http://senderHost/[[mergeField1]]/ beginSigningSession? [[mergeField2]]&[[mergeField3]]` |
///| customFields | No | customField |An optional array of strings that allows the sender to provide custom data about the recipient. This information is returned in the template status but otherwise not used by DocuSign. String `customField` properties have a maximum length of 100 characters. |
///| emailNotification | No | emailNotification | An optional complex type that has information for setting the language for the recipient's email information. It is composed of three elements:<br/><br/>*emailBody*: a string with the email message sent to the recipient.<br/>Maximum Length: 10000 characters.<br/><br/>*emailSubject*: a string with the subject of the email sent to the recipient.<br/>Maximum Length: 100 characters.<br/><br/>*supportedLanguage*: The simple type enumeration of the language used. The supported languages, with the language value shown in parenthesis, are: Arabic (ar), Bahasa Indonesia (id), Bahasa Melayu (ms) Bulgarian (bg), Czech (cs), Chinese Simplified (zh_CN), Chinese Traditional (zh_TW), Croatian (hr), Danish (da), Dutch (nl), English US (en), English UK (en_GB), Estonian (et), Farsi (fa), Finnish (fi), French (fr), French Canada (fr_CA), German (de), Greek (el), Hebrew (he), Hindi (hi), Hungarian (hu), Italian (it), Japanese (ja), Korean (ko), Latvian (lv), Lithuanian (lt), Norwegian (no), Polish (pl), Portuguese (pt), Portuguese Brazil (pt_BR), Romanian (ro),Russian (ru), Serbian (sr), Slovak (sk), Slovenian (sl), Spanish (es),Spanish Latin America (es_MX), Swedish (sv), Thai (th), Turkish (tr), Ukrainian (uk) and Vietnamese (vi).<br/><br/>**IMPORTANT**: If this is enabled for one recipient, it overrides the Template Subject and `EmailBlurb` property settings. Also, you must set the  `emailNotification` property for all recipients. |
///| excludedDocuments | No | Array of Strings | Specifies the documents that are not visible to this recipient. Document Visibility must be enabled for the account and the enforceSignerVisibility property must be set to true for the template to use this.<br/><br/>When the enforceSignerVisibility property is set to **true**, documents with tabs can only be viewed by signers that have a tab on that document. Recipients that have an administrative role (Agent, Editor, or Intermediaries) or informational role (Certified Deliveries or Carbon Copies) can always see all the documents in an template, unless they are specifically excluded using this setting when an template is sent. Documents that do not have tabs are always visible to all recipients, unless they are specifically excluded using this setting when an template is sent. |
///| idCheckConfigurationName | No | String |Specifies authentication check by name. The names used here must be the same as the authentication type names used by the account (these name can also be found in the web console sending interface in the Identify list for a recipient). This overrides any default authentication setting.<br/><br/>_Example_:<br/> Your account has ID Check and SMS Authentication available and in the web console Identify list these appear as 'ID Check $' and 'SMS Auth $'. To use ID check in an template, the `idCheckConfigurationName` property must be  set to `ID Check $`. To use SMS, it must be set to `SMS Auth $` and you must add phone number information to the `smsAuthentication` node.|
///| iDCheckInformationInput | No | IdCheckInformationInput | This complex element contains input information related to a recipient ID check. It can include the following information.<br/><br/>*addressInformationInput*: Used to set recipient address information and consists of:<br/><br/>*addressInformation*: consists of six elements, with street2 and zipPlus4 being optional. The elements are: street1, street2, city, state, zip, zipPlus4\. The maximum number of characters in each element are: street1/street2 = 150 characters, city = 50 characters, state = 2 characters, and zip/zipPlus4 = 20 characters.<br/><br/>displayLevelCode: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*dobInformationInput*: Used to set recipient date of birth information and consists of:<br/><br/>*dateOfBirth*: Specifies the recipient's date, month and year of birth.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*ssn4InformationInput*: Used to set the last four digits of the recipient's SSN information and consists of:<br/><br/>*ssn4*: Specifies the last four digits of the recipient's SSN.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*ssn9InformationInput*: Used to set the recipient's SSN information. Note that the ssn9 information can never be returned in the response. The ssn9 input consists of:<br/><br/><br/><br/>*ssn9*: Specifies the recipient's SSN.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay. |
///| inheritEmailNotificationConfiguration | No | Boolean | Optional element. If true and the template recipient creates a DocuSign account after signing, the Manage Account Email Notification settings are used as the default settings for the recipient's account. |
///| note | No | String | A note that is unique to this recipient. This note is sent to the recipient via the signing email. The note displays in the signing UI near the upper left corner of the document on the signing screen.<br/>Maximum Length: 1000 characters. |
///| phoneAuthentication | No | RecipientPhoneAuthentication | Optional element. Contains the elements: <br/><br/> *recipMayProvideNumber*:Boolean. When set to **true** thenrecipient can use whatever phone number they choose to.<br/><br/> *senderProvidedNumbers*: ArrayOfString. A list of phone numbers the recipient can use. <br/><br/>*recordVoicePrint* - Reserved <br/><br/>*validateRecipProvidedNumber* - Reserved| |
///| recipientAttachment | No | Attachment | Reserved |
///| recipientId | No | String | Unique for the recipient. It is used by the tab element to indicate which recipient is to sign the Document. |
///| requireIdLookup | No | Boolean | When set to **true**, the recipient is required to use the specified ID check method (including Phone and SMS authentication) to validate their identity. |
///| roleName | No* | String | Optional element. Specifies the role name associated with the recipient.<br/><br/>This is required when working with template recipients. |
///| routingOrder | Yes | String | This element specifies the routing order of the recipient in the template. |
///| samlAuthentication | No | samlAssertionAttributes | Optional element, account must be set up to use SSO to use this. Contains the name/value pair information for the SAML assertion attributes:<br/><br/>*name*: The name of the SAML assertion attribute.<br/><br/>*value*: The value associated with the named SAML assertion attribute. |
///| smsAuthentication | No | senderProvidedNumbers | Optional element. Contains the element:<br/><br/>*senderProvidedNumbers*: Array that contains a list of phone numbers the recipient can use for SMS text authentication. |
///| socialAuthentications | No | Boolean | Lists the social ID type that can be used for recipient authentication. |
///| templateAccessCodeRequired | No | Boolean | Optional element. Used only when working with template recipients. When set to **true** and the `TemplateLocked` parameter is set to **true**, the sender must enter an access code. |
///| templateLocked | No | Boolean | Optional element. Used only when working with template recipients. When set to **true**, the sender cannot change any attributes of the recipient. |
///| templateRequired | No | Boolean | Optional element. Used only when working with template recipients. When set to **true**,  the sender may not remove the recipient. |
///
///## <a name="agents"></a> Agents Recipient
///
///This recipient can add name and email information for recipients that appear after the recipient in routing order.
///
///#### Example Agents layout
///
///```
/// "agents": [{
/// <core parameters>
///  "canEditRecipientEmails": false,
///  "canEditRecipientNames": false
/// }],
/// ```
///
///The additional parameters for Agents recipient are shown below:
///
///| Name | Required? | Schema Type | Description |
///| --- | --- | --- | --- |
///| canEditRecipientEmails | No | Boolean | Optional element. When set to **true**, the Agents Recipient associated with this Recipient can change the Recipient's pre-populated Email address. This element is only active if enabled for the account. |
///| canEditRecipientNames | No | Boolean | Optional element. When set to **true**, the Agents Recipient associated with this recipient can change the recipient's pre-populated name (`UserName`). This element is only active if enabled for the account. |
///
///## <a name="carbonCopies"></a>Carbon Copies Recipient
///
///Use this type if the recipient should get a copy of the template, but the recipient does not need to sign, initial, date, or add information to any of the documents. This type of recipient can be placed in any order in the recipient list. The recipient receives a copy of the template when the template reaches the recipient's order in the process flow and when the template is completed.
///
///#### Example Carbon Copies layout
///
///```
/// "carbonCopies": [{
/// <core parameters>
/// ```
///
///The Carbon Copies recipient uses only the core parameters.
///
///## <a name="certifiedDeliveries"></a>Certified Deliveries Recipient
///
///Use this action if the recipient must receive the completed documents for the template to be completed, but the recipient does not need to sign, initial, date, or add information to any of the documents.
///
///#### Example Certified Deliveries layout
///
///```
/// "certifiedDeliveries": [{
/// <core parameters>
/// }],
/// ```
///The Certified Deliveries recipient uses only the core parameters.
///
///## <a name="editors"></a>Editors Recipient
///
///This recipient has the same management and access rights for the template as the sender and can make changes to the template as if they were using the Advanced Correct feature. This recipient can add name and email information, add or change the routing order and set authentication options for the remaining recipients. Additionally, this recipient can edit signature/initial tabs and data fields for the remaining recipients. The recipient must have a DocuSign account to be an editor.
///
///#### Example Editors layout
///
///```
/// "editors": [{
/// <core parameters>
///  "canEditRecipientEmails": false,
///  "canEditRecipientNames": false
/// }],
/// ```
///
///The additional parameters for Editors recipient are shown below:
///
///| Name | Required? | Schema Type | Description |
///| --- | --- | --- | --- |
///| canEditRecipientEmails | No | Boolean | Optional element. When set to **true**, the Editors Recipient associated with this Recipient can change the Recipient's pre-populated Email address. This element is only active if enabled for the account. |
///| canEditRecipientNames | No | Boolean | Optional element. When set to **true**, the Editors Recipient associated with this recipient can change the recipient's pre-populated name (`UserName`). This element is only active if enabled for the account. |
///
///## <a name="inPerson"></a>In Person Signers Recipient
///
///Use this type if the signer is in the same physical location as a DocuSign user who will act as a Signing Host for the transaction. The recipient added is the Signing Host and new separate Signer Name field appears after Sign in person is selected.
///
///#### Example In Person Signers layout
///
///```
/// "inPersonSigners": [{
///  "hostEmail": "signing.host@company.com",
///  "hostName": "Mike Host",
/// <core parameters>
///  "autoNavigation": false,
///  "defaultRecipient": false,
///  "signInEachLocation": false,
///  "signatureInfo": null,
///  "signerEmail": "signer.name@company.com",
///  "signerName": "MC Clap Your Hands",
///  "tabs": {
///    "approveTabs": null,
///    "checkboxTabs": null,
///    "companyTabs": null,
///    "dateSignedTabs": null,
///    "dateTabs": null,
///    "declineTabs": null,
///    "emailTabs": null,
///    "templateIdTabs": null,
///    "fullNameTabs": null,
///    "initialHereTabs": null,
///    "listTabs": null,
///    "noteTabs": null,
///    "numberTabs": null,
///    "radioGroupTabs": null,
///    "signHereTabs": [{
///    "signerAttachmentTabs": null,
///    "ssnTabs": null,
///    "textTabs": null,
///    "titleTabs": null,
///    "zipTabs": null
///  }
/// }],
/// ```
///
///The additional and changed parameters for In Person Signers recipient are shown below:
///
///| Name | Required? | Schema Type | Description |
///| --- | --- | --- | --- |
///| hostEmail | Yes | Email | Required element for In Person Signers recipient Type.<br/>Maximum Length: 100 characters.<br/><br/>Specifies the email for the signing host. |
///| hostName | Yes | String | Required element for In Person Signers recipient Type.<br/>Maximum Length: 100 characters.<br/><br/>Specifies the email for the signing host.|
///| autoNavigation | No | Boolean | Specifies whether auto navigation is set for the recipient.|
///| defaultRecipient | No | Boolean | When set to **true**, this is the default recipient for the template. This option is used with the CreateTemplateFromTemplatesAndForms method. |
///| signInEachLocation | No | Boolean | When set to **true** and the feature is enabled in the sender's account, the signing recipient is required to draw signatures and initials at each signature/initial tab (instead of adopting a signature/initial style or only drawing a signature/initial once). |
///| signatureInfo | No | String | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Allows the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. |
///| signerEmail | No | String | Optional element. The email address for an InPersonSigner recipient Type.<br/>Maximum Length: 100 characters. |
///| signerName | Yes | String | Required element with recipient type In Person Signers.<br/>Maximum Length: 100 characters.<br/><br/>The full legal name of a signer for the template. |
///| tabs | No | Tab | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Specifies the Tabs associated with the recipient. See the See the the [EnvelopeRecipientTabs resource](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/enveloperecipienttabs/)  for more information about tabs. for more information about tabs. |
///
///## <a name="intermediaries"></a>Intermediaries Recipient
///
///This recipient can, but is not required to, add name and email information for recipients at the same or subsequent level in the routing order (until subsequent Agents, Editors or Intermediaries recipient types are added).
///
///#### Example Intermediaries layout
///
///```
/// "intermediaries": [{
/// <core parameters>
///  "canEditRecipientEmails": false,
///  "canEditRecipientNames": false
/// }],
/// ```
///
///The parameters for Intermediaries recipient are shown below:
///
///| Name | Required? | Schema Type | Description |
///| --- | --- | --- | --- |
///| canEditRecipientEmails | No | Boolean | Optional element. When set to **true**, the Agents Recipient associated with this Recipient can change the Recipient's pre-populated Email address. This element is only active if enabled for the account. |
///| canEditRecipientNames | No | Boolean | Optional element. When set to **true**, the Agents Recipient associated with this recipient can change the recipient's pre-populated name (`UserName`). This element is only active if enabled for the account. |
///
///## <a name="signers"></a>Signers Recipient
///
///Use this action if your recipient must sign, initial, date, or add data to form fields on the documents in the template.
///
///#### Example Signers layout
///
///```
/// "Signers": [{
/// <core paramters>
///  "autoNavigation": false,
///  "defaultRecipient": false,
///  "signInEachLocation": false,
///  "signatureInfo": null,
///  "tabs": {
///    "approveTabs": null,
///    "checkboxTabs": null,
///    "companyTabs": null,
///    "dateSignedTabs": null,
///    "dateTabs": null,
///    "declineTabs": null,
///    "emailTabs": null,
///    "templateIdTabs": null,
///    "fullNameTabs": null,
///    "initialHereTabs": null,
///    "listTabs": null,
///    "noteTabs": null,
///    "numberTabs": null,
///    "radioGroupTabs": null,
///    "signHereTabs": [{
///    "signerAttachmentTabs": null,
///    "ssnTabs": null,
///    "textTabs": null,
///    "titleTabs": null,
///    "zipTabs": null
///  }
///  "deliveryMethod":"",
///  "deliveredDateTime":"String Content",
///  "signedDateTime":"String Content",
///  "offlineAttributes":{
///    "deviceName":"String Content",
///    "deviceModel":"String Content",
///    "gpsLatitude":"String Content",
///    "gpsLongitude":"String Content",
///    "accountEsignId":"String Content"
///  }
/// }],
/// ```
///
///The additional parameters for Signers recipient are shown below:
///
///| Name | Required? | Schema Type | Description |
///| --- | --- | --- | --- |
///| autoNavigation | No | Boolean | Specifies whether auto navigation is set for the recipient.|
///| defaultRecipient | No | Boolean | When set to **true**, this is the default recipient for the template. This option is used with the CreateTemplateFromTemplatesAndForms method. |
///| signInEachLocation | No | Boolean | When set to **true** and the feature is enabled in the sender's account, the signing recipient is required to draw signatures and initials at each signature/initial tab (instead of adopting a signature/initial style or only drawing a signature/initial once). |
///| signatureInfo | No | String | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Allows the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. |
///| signerEmail | No | String | Optional element. The email address for an InPersonSigner recipient Type.<br/>Maximum Length: 100 characters. |
///| signerName | Yes | String | Required element with recipient type In Person Signers.<br/>Maximum Length: 100 characters.<br/><br/>The full legal name of a signer for the template. |
///| tabs | No | Tab | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Specifies the Tabs associated with the recipient. See the See the [EnvelopeRecipientTabs resource](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/enveloperecipienttabs/)  for more information about tabs. section for more information about tabs. |
///| deliveryMethod | No | String | Reserved: For DocuSign use only.|
///| deliveredDateTime | No | DateTime | Reserved: For DocuSign use only. |
///| signedDateTime | No | DateTime | Reserved: For DocuSign use only. |
///| offlineAttributes | No | &nbsp; | Reserved: For DocuSign use only.|
///.
pub mod template_recipients;
/// **Note**: Responsive Signing is disabled by default. To use this functionality, an account administrator must switch the account setting `enableResponsiveSigning` to **true**.
///Also note that Smart Sections (creating a signable HTML document that uses collapsible sections and rotating tables) are premium features. To request them, contact your DocuSign account manager.
pub mod template_responsive_html_preview;
/// The TemplateViews resource
///provides a method that returns a URL
///that you can embed in your application
///to generate a template view that uses the DocuSign UI.
///
///One template view is available:
///
///* Edit View: The DocuSign UI for editing a template.
///
///This resource is related to the [EnvelopeViews][envelopeviews] resource.
///Both resources enable you to embed the DocuSign UI into your application.
///
///
///[envelopeviews]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeViews/
///.
pub mod template_views;
/// The Template resource provides methods that allow you to manipulate and monitor templates.
///
///Once you have authenticated the user you can use the Templates: createTemplate method to create a template. You can call the TemplateDocuments: update or TemplateDocuments: updateList method to add more documents.
///
///If you have a template that you have previously saved, you can modify the subject and message, send it, void it, or purge it from the DocuSign system using the Template: update method.
///
///In addition to receiving notifications, you can monitor the status of the templates using the following methods:
///
///* Template: getTemplate - Gets the status of a template.
///* Template: listStatus - Gets the status for a set of templates.
///* Template: listStatusChanges - Gets status change information for one or more templates.
///
///If you need to delete a page from a document in a template, use the Template: deleteDocumentPage method.
///
///The resource also includes a number of methods that allow you to retrieve and set the initials and signature for certain types of recipients on the document.
pub mod templates;
#[cfg(test)]
mod tests;
pub mod types;
/// The UserCustomSettings resource provides methods that allow you to manage the custom settings for a user.
///
///Custom settings are a flexible way to store and retrieve custom user information that can be used in your own system.
///
///There is a limit on the size for all the custom user settings for a single user. All the custom user settings for a single user is limited to 4,000 characters, which includes the xml and json structure for the settings.
pub mod user_custom_settings;
/// The UserProfiles resource provides methods that allow you to manage a user's profile.
pub mod user_profiles;
/// The UserSignatures resource provides methods that allow you manage the  intials and signature images for a user.
pub mod user_signatures;
/// The Users resource provides methods that enable you to manage users for an account.
///
///The following User methods do not use the `title` property in the Users object:
///
///- create
///- delete
///- deleteProfileImage
///- get
///- getProfileImage
///- getSettings
///- list
///- update
///- updateList
///- updateProfileImag
///- updateSettings
///
///Instead, you can retrieve and set the user's job title by using the UserProfiles:Get and UserProfiles:Update methods. For more information, see [UserProfiles](https://developers.docusign.com/docs/esign-rest-api/reference/Users/UserProfiles).
pub mod users;
#[doc(hidden)]
pub mod utils;
/// The WorkspaceItems resource provides methods that enable you to manage
///workspace items.
///.
pub mod workspace_items;
/// A workspace is a collaborative space for sharing documents and managing workflows. A workspace has a single owner who must be a DocuSign user. The owner can invite others to the workspace as collaborators. Individuals who are not DocuSign users must create a DocuSign account to join a workspace as a collaborator.
///
///You can create an envelope directly from a workspace.
///
///Workspaces store the following information:
///
///- **Files**: Files uploaded to a workspace for storage or reuse.
///- **Documents**: A document is a component of a transaction, template, or workspace. When a file is added to a transaction, template, or workspace, it is copied as a document. Each document in a workspace has a single owner.
///- **Templates**: A set of documents that you can use to create a transaction or a workspace.
///- **Transactions**: A transaction is a series of workflow events related to one or more documents. These events route the documents to one or more individuals for the purposes of doing business. Each transaction has a single owner (the sender).
///
///**Note**: Documents in a template are not individually listed as files.
pub mod workspaces;

use anyhow::{anyhow, Error, Result};

pub const DEFAULT_HOST: &str = "https://na4.docusign.net";

mod progenitor_support {
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

    const PATH_SET: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}');

    #[allow(dead_code)]
    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}

use std::env;

const TOKEN_ENDPOINT: &str = "https://account.docusign.com/oauth/token";
const USER_CONSENT_ENDPOINT: &str = "https://account.docusign.com/oauth/auth";

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    token: String,
    // This will expire within a certain amount of time as determined by the
    // expiration date passed back in the initial request.
    refresh_token: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,

    client: reqwest::Client,
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, JsonSchema, Clone, Default, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token_type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
    #[serde(default)]
    pub expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub refresh_token: String,
    #[serde(default, alias = "x_refresh_token_expires_in")]
    pub refresh_token_expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scope: String,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<I, K, R, T, Q>(
        client_id: I,
        client_secret: K,
        redirect_uri: R,
        token: T,
        refresh_token: Q,
    ) -> Self
    where
        I: ToString,
        K: ToString,
        R: ToString,
        T: ToString,
        Q: ToString,
    {
        let client = reqwest::Client::builder().build();
        match client {
            Ok(c) => {
                // We do not refresh the access token here since we leave that up to the
                // user to do so they can re-save it to their database.
                // TODO: But in the future we should save the expires in date and refresh it
                // if it needs to be refreshed.
                //
                Client {
                    host: DEFAULT_HOST.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    redirect_uri: redirect_uri.to_string(),
                    token: token.to_string(),
                    refresh_token: refresh_token.to_string(),

                    client: c,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Override the default host for the client.
    pub fn with_host<H>(&self, host: H) -> Self
    where
        H: ToString,
    {
        let mut c = self.clone();
        c.host = host.to_string();
        c
    }

    /// Create a new Client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key and your requests will work.
    /// We pass in the token and refresh token to the client so if you are storing
    /// it in a database, you can get it first.
    pub fn new_from_env<T, R>(token: T, refresh_token: R) -> Self
    where
        T: ToString,
        R: ToString,
    {
        let client_id = env::var("DOCUSIGN_CLIENT_ID").expect("must set DOCUSIGN_CLIENT_ID");
        let client_secret =
            env::var("DOCUSIGN_CLIENT_SECRET").expect("must set DOCUSIGN_CLIENT_SECRET");
        let redirect_uri =
            env::var("DOCUSIGN_REDIRECT_URI").expect("must set DOCUSIGN_REDIRECT_URI");

        Client::new(client_id, client_secret, redirect_uri, token, refresh_token)
    }

    /// Return a user consent url with an optional set of scopes.
    /// If no scopes are provided, they will not be passed in the url.
    pub fn user_consent_url(&self, scopes: &[String]) -> String {
        let state = uuid::Uuid::new_v4();

        let url = format!(
            "{}?client_id={}&response_type=code&redirect_uri={}&state={}",
            USER_CONSENT_ENDPOINT, self.client_id, self.redirect_uri, state
        );

        if scopes.is_empty() {
            return url;
        }

        // Add the scopes.
        format!("{}&scope={}", url, scopes.join(" "))
    }

    /// Refresh an access token from a refresh token. Client must have a refresh token
    /// for this to work.
    pub async fn refresh_access_token(&mut self) -> Result<AccessToken> {
        if self.refresh_token.is_empty() {
            anyhow!("refresh token cannot be empty");
        }

        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh_token),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri),
        ];
        let client = reqwest::Client::new();
        let resp = client
            .post(TOKEN_ENDPOINT)
            .headers(headers)
            .form(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;

        // Unwrap the response.
        let t: AccessToken = resp.json().await?;

        self.token = t.access_token.to_string();
        self.refresh_token = t.refresh_token.to_string();

        Ok(t)
    }

    /// Get an access token from the code returned by the URL paramter sent to the
    /// redirect URL.
    pub async fn get_access_token(&mut self, code: &str, state: &str) -> Result<AccessToken> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri),
            ("state", state),
        ];
        let client = reqwest::Client::new();
        let resp = client
            .post(TOKEN_ENDPOINT)
            .headers(headers)
            .form(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;

        // Unwrap the response.
        let t: AccessToken = resp.json().await?;

        self.token = t.access_token.to_string();
        self.refresh_token = t.refresh_token.to_string();

        Ok(t)
    }

    async fn url_and_auth(&self, uri: &str) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();

        let auth = format!("Bearer {}", self.token);
        parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
    }

    async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::Response> {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method.clone(), url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if let Some(body) = body {
            log::debug!(
                "body: {:?}",
                String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap()
            );
            req = req.body(body);
        }
        log::debug!("request: {:?}", &req);
        Ok(req.send().await?)
    }

    async fn request<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, body).await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    async fn request_with_links<Out>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<(Option<hyperx::header::Link>, Out)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, body).await?;

        let status = response.status();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| l.parse().ok());

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );

            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map(|out| (link, out)).map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };
            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn post_form<Out>(&self, uri: &str, form: reqwest::multipart::Form) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(http::Method::POST, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        log::debug!("form: {:?}", form);
        req = req.multipart(form);

        log::debug!("request: {:?}", &req);
        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                serde_json::from_value(serde_json::json!(&String::from_utf8(
                    response_body.to_vec()
                )?))
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_accept_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        accept_mime_type: &str,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_str(accept_mime_type)?,
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        log::debug!("request: {:?}", &req);
        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                serde_json::from_value(serde_json::json!(&String::from_utf8(
                    response_body.to_vec()
                )?))
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        content: &[u8],
        mime_type: &str,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_bytes(mime_type.as_bytes()).unwrap(),
        );
        // We are likely uploading a file so add the right headers.
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-type"),
            reqwest::header::HeaderValue::from_static("application/octet-stream"),
        );
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-length"),
            reqwest::header::HeaderValue::from_bytes(format!("{}", content.len()).as_bytes())
                .unwrap(),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if content.len() > 1 {
            let b = bytes::Bytes::copy_from_slice(content);
            // We are uploading a file so add that as the body.
            req = req.body(b);
        }

        log::debug!("request: {:?}", &req);
        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let r = self.request(method, uri, body).await?;
        Ok(r)
    }

    #[allow(dead_code)]
    async fn get<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::GET, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn get_all_pages<D>(&self, uri: &str, _message: Option<reqwest::Body>) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        // TODO: implement this.
        self.unfold(uri).await
    }

    /// "unfold" paginated results of a vector of items
    #[allow(dead_code)]
    async fn unfold<D>(&self, uri: &str) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut items) = self.get_pages(uri).await?;
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = link.as_ref().and_then(|l| crate::utils::next_link(l)) {
                let url = reqwest::Url::parse(&url)?;
                let (new_link, new_items) = self.get_pages_url(&url).await?;
                link = new_link;
                items = new_items;
            }
        }

        Ok(global_items)
    }

    #[allow(dead_code)]
    async fn get_pages<D>(&self, uri: &str) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, &(self.host.to_string() + uri), None)
            .await
    }

    #[allow(dead_code)]
    async fn get_pages_url<D>(
        &self,
        url: &reqwest::Url,
    ) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, url.as_str(), None)
            .await
    }

    #[allow(dead_code)]
    async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::POST, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PATCH, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PUT, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(self.host.to_string() + uri),
            message,
        )
        .await
    }

    /// The AcccountBrands resource provides methods that enable you to create and manage brands for an account.
    ///
    ///Branding enables you to add the look and feel of your organization's brand to the sending, signing, and email processes, making it easier for recipients to identify envelopes coming from your organization.
    ///
    ///The DocuSign Account Custom Branding feature enables you to set the colors, logo, and text that recipients see at the account level. The settings associated with a brand are applied to all of the envelopes that use the brand. You can create multiple brand profiles for different corporate brands or internal departments.
    ///
    ///**Note**: To use this resource, branding for either signing or sending must be enabled for the account (either `canSelfBrandSend`, `canSelfBrandSign`, or both of these account settings must be set to **true**).
    pub fn account_brands(&self) -> account_brands::AccountBrands {
        account_brands::AccountBrands::new(self.clone())
    }

    /// The `AccountConsumerDisclosures` resource provides methods that enable you to enable, retrieve, and manage the Electronic Record and Signature Consent Disclosure (ERSD) options for your account. This is the disclosure that displays to each new recipient who is going to sign or add other information, or who is required to view the documents you send to them. The recipient must read and agree to the terms of the disclosure before they can access and take action on the documents you send. The ERSD does not apply to copy-only recipients, but does apply to recipients who must sign or view your documents.
    ///
    ///You can use either the default ERSD that DocuSign provides for U.S.-based transactions, or a custom ERSD.
    ///
    ///## Languages
    ///
    ///**Important**: The system does not translate the ERSD for you. The default ERSD is always in English. For a custom ERSD, an account administrator must create a version of the disclosure for each language that your signers use. When you create a version of your custom ERSD for a specific signer language, you must:
    ///
    ///1. Specify the language code (`langCode`) for the signer language.
    ///2. Provide the `esignAgreementText` and `esignText` in the language associated with the `langCode`.
    ///
    ///For more information, see [Legal Disclosure](https://support.docusign.com/en/guides/ndse-admin-guide-legal-disclosure).
    pub fn account_consumer_disclosures(
        &self,
    ) -> account_consumer_disclosures::AccountConsumerDisclosures {
        account_consumer_disclosures::AccountConsumerDisclosures::new(self.clone())
    }

    /// Custom fields enable you to record custom information about envelopes that you can then use for sorting, organizing, searching, and other downstream processes.
    ///
    ///For example, you can use custom fields to copy envelopes or data to multiple areas in Salesforce. eOriginal customers can eVault all of their documents from the web app by setting an account custom field with a name like `eVault with eOriginal` to **true**.
    ///
    ///You can also use account custom fields to set the following information:
    ///
    ///- Tracking ID
    ///- Department
    ///- Use case
    ///- Other envelope metadata
    ///
    ///## Envelope Custom Field Visibility
    ///
    ///When you create an envelope custom field for your account, you have the following options:
    ///
    ///- Make it a required field for senders at the time of sending
    ///- Display it as an optional field at the time of sending
    ///- Set a specific value for the field behind the scenes (NOT SURE IF THIS IS RIGHT; MIGHT JUST BE AN UNUSED DRAFT FIELD)
    ///
    ///Envelope recipients do not see the envelope custom fields.
    ///
    ///## Types of Envelope Custom Fields
    ///
    ///There are two types of envelope custom fields:
    ///
    ///- `text`: Enables the sender to enter the value for the field.
    ///- `list`: Enables the sender to select the value of the field from a predetermined list.
    pub fn account_custom_fields(&self) -> account_custom_fields::AccountCustomFields {
        account_custom_fields::AccountCustomFields::new(self.clone())
    }

    /// The EnvelopeCustomFields resource provides methods that allow you manage custom fields in an envelope.
    ///
    ///Custom fields can be used in the envelopes for your account to record information about the envelope, help search for envelopes and track information. The envelope custom fields are shown in the Envelope Settings section when a user is creating an envelope in the DocuSign member console. The envelope custom fields are not seen by the envelope recipients.
    ///
    ///There are two types of envelope custom fields:
    ///
    ///- `text`: Enables the sender to enter the value for the field.
    ///- `list`: Enables the sender to select the value of the field from a predetermined list.
    ///
    ///You may assign up to three envelope custom fields to an envelope. This limit does not include account (document) custom fields.
    pub fn envelope_custom_fields(&self) -> envelope_custom_fields::EnvelopeCustomFields {
        envelope_custom_fields::EnvelopeCustomFields::new(self.clone())
    }

    /// The EnvelopeDocumentFields resource provides methods that allow you to manage custom fields on a document.
    ///
    ///You can create custom versions of standard fields that combine of field properties, such as font type or size, or a validation setting.
    ///
    ///Note: Some advanced features and options are supported only in certain DocuSign plans. Your account plan might not support some options discussed in this help topic. For more information about which options are available for your account, check your account plan or contact your Account Manager.
    pub fn envelope_document_fields(&self) -> envelope_document_fields::EnvelopeDocumentFields {
        envelope_document_fields::EnvelopeDocumentFields::new(self.clone())
    }

    /// The EnvelopeLocks resource provides methods that allow you to
    ///manage locks on an envelope.
    ///
    ///To prevent users from changing an envelope while another user is
    ///modifying it, you can lock the envelope and set the time until
    ///the lock expires.
    ///
    ///For example, you would use the following flow:
    ///
    ///1. Lock the envelope.
    ///2. Make changes to envelope.
    ///3. Delete the envelope lock and save the changes. If the envelope is based on a template that has a password, you must supply the template password to save the changes.
    ///
    ///**Note**: To use envelope locks, the user must have envelope locking capability enabled.
    pub fn envelope_locks(&self) -> envelope_locks::EnvelopeLocks {
        envelope_locks::EnvelopeLocks::new(self.clone())
    }

    /// The EnvelopeRecipients resource enables you manage the recipients of an envelope. All recipient types share a set of [core parameters](#core-recipient-parameters), but some recipient types have additional parameters. You specify the recipient type using the `recipientType` parameter. The recipient types are as follows:
    ///
    ///<br>
    ///
    ///| Recipient type | Description |
    ///| :--- | :--- |
    ///| [Agent](#agent-recipient) | Agent recipients can add name and email information for recipients that appear after the agent in routing order. |
    ///| [Carbon Copy](#carbon-copy-recipient) | Carbon copy recipients get a copy of the envelope but don't need to sign, initial, date, or add information to any of the documents. This type of recipient can be used in any routing order. Carbon copy recipients receive their copy of the envelope when the envelope reaches the recipient's order in the process flow and when the envelope is completed. |
    ///| [Certified Delivery](#certified-delivery-recipient) | Certified delivery recipients must receive the completed documents for the envelope to be completed. However, they don't need to sign, initial, date, or add information to any of the documents. |
    ///| [Editor](#editor-recipient) | Editors have the same management and access rights for the envelope as the sender. They can make changes to the envelope as if they were using the Advanced Correct feature. This recipient can add name and email information, add or change the routing order, set authentication options, and can edit signature/initial tabs and data fields for the remaining recipients. The recipient must have a DocuSign account to be an editor. |
    ///| [In-Person Signer](#in-person-signer-recipient) | In-person signer recipients are DocuSign users who act as signing hosts in the same physical location as the signer. |
    ///| [Intermediary](#intermediary-recipient) | Intermediaries are recipients who can, but are not required to, add name and email information for recipients at the same or subsequent level in the routing order, unless subsequent agents, editors, or intermediaries are added. |
    ///| [Seal](#seal-recipient) | Electronic seal recipients represent legal entities rather than individuals. Organizations and governments can use electronic seals to show evidence of origin and integrity of documents. |
    ///| [Signer](#signer-recipient) | Signers are recipients who must sign, initial, date, or add data to form fields on the documents in the envelope. |
    ///| [Witness](#witness-recipient) | Witnesses are recipients whose signatures affirm that the identified signers have signed the documents in the envelope. |
    ///
    ///<br>
    ///
    ///Not all recipients are are available to all account types. Review your account plan to determine which recipient types are available to you. All recipient types are available in the Demo environment.
    ///
    ///
    ///## Core Recipient Parameters
    ///
    ///All recipients, regardless of type, have the same common core parameters. The following table contains the descriptions for the core properties for all recipient types:
    ///
    ///<br>
    ///
    ///| Name | Required | Schema Type | Description |
    ///| :--- | :--- | :--- | :--- |
    ///| email | Yes | Email | Email of the recipient. Notification will be sent to this email id.<br/>Maximum Length: 100 characters. |
    ///| name | Yes | String | Full legal name of the recipient.<br/>Maximum Length: 100 characters.<br/><br/>**Note:** If you are creating an envelope with DocuSign EU advanced signature enabled, ensure that recipient names do not contain any of the following characters: **^ : \ @ , + <** |
    ///| accessCode | No | String | This optional element specifies the access code a recipient has to enter to validate the identity.<br/>Maximum Length: 50 characters. |
    ///| addAccessCodeToEmail | No | Boolean | This optional attribute indicates that the access code is added to the email sent to the recipient; this nullifies the Security measure of Access Code on the recipient. |
    ///| agentCanEditEmail | No | Boolean | When set to **true**, the agent recipient associated with this recipient can change the recipient's pre-populated email address. This element is only active if enabled for the account. |
    ///| agentCanEditName | No | Boolean | When set to **true**, the agent recipient associated with this recipient can change the recipient's pre-populated name (`UserName`). This element is only active if enabled for the account. |
    ///| clientUserId | No | String | This specifies whether the recipient is embedded or remote.<br/><br/>If the `clientUserId` property is not null then the recipient is embedded. Note that if the `ClientUserId` property is set and either `SignerMustHaveAccount` or `SignerMustLoginToSign` property of the account settings is set to  **true**, an error is generated on sending. Maximum length: 100 characters. |
    ///| embeddedRecipientStartURL | No | String | This is a sender provided valid URL string for redirecting an embedded recipient. When using this option, the embedded recipient still receives an email from DocuSign, just as a remote recipient would, but when the document link in the email is clicked the recipient is redirected, through DocuSign, to this URL to complete their actions. When routing to the URL, it is up to the sender's system (the server responding to the URL) to then request a recipient token to launch a signing session.<br/><br/>If the value `SIGN_AT_DOCUSIGN` is used for this node, the recipient is directed to an embedded signing or viewing process directly at DocuSign. The signing or viewing action is initiated by the DocuSign system and the transaction activity and Certificate of Completion records will reflect this. In all other ways the process is identical to an embedded signing or viewing operation that would be launched by any partner.<br/><br/>It is important to remember that in a typical embedded workflow the authentication of an embedded recipient is the responsibility of the sending application and DocuSign expects that senders will follow their own process for establishing the recipient's identity. In this workflow the recipient goes through the sending application before the embedded signing or viewing process in initiated. However, when the sending application sets  the `EmbeddedRecipientStartURL` property to `SIGN_AT_DOCUSIGN`, the recipient goes directly to the embedded signing or viewing process bypassing the sending application and any authentication steps the sending application would use. In this case, DocuSign recommends that one of the normal DocuSign authentication features (Access Code, Phone Authentication, SMS Authentication, etc.) be used to verify the identity of the recipient.<br/><br>If the `clientUserId` property is NOT set and the `embeddedRecipientStartURL` property is set, DocuSign ignores the redirect URL and launch the standard signing process for the email recipient. Information can be appended to the `embeddedRecipientStartURL` property using merge fields. The available merge fields items are: envelopeId, recipientId, recipientName, recipientEmail, and customFields. The customFields must be part of the recipient or envelope. The merge fields are enclosed in double brackets.<br/><br/>_Example_:<br/>`http://senderHost/[[mergeField1]]/ beginSigningSession? [[mergeField2]]&[[mergeField3]]` |
    ///| customFields | No | customField |An optional array of strings that allows the sender to provide custom data about the recipient. This information is returned in the envelope status but otherwise not used by DocuSign. String `customField` properties have a maximum length of 100 characters. |
    ///| emailNotification | No | emailNotification | An optional complex type that has information for setting the language for the recipient's email information. It is composed of three elements:<br/><br/>*emailBody*: a string with the email message sent to the recipient.<br/>Maximum Length: 10000 characters.<br/><br/>*emailSubject*: a string with the subject of the email sent to the recipient.<br/>Maximum Length: 100 characters.<br/><br/>*supportedLanguage*: The simple type enumeration (two-letter code) for the language to use for the standard email format and the signing view for the recipient. To retrieve the possible values, use the [Accounts::listSupportedLanguages method][ListLang].<br/><br/>**Note**: You can set the `emailNotification` property separately for each recipient. If you set the value only for certain recipients, the other recipients will inherit the this value from the top-level `emailSubject` and `emailBlurb`.  |
    ///| excludedDocuments | No | Array of Strings | Specifies the documents that are not visible to this recipient. Document Visibility must be enabled for the account and the enforceSignerVisibility property must be set to true for the envelope to use this.<br/><br/>When the enforceSignerVisibility property is set to **true**, documents with tabs can only be viewed by signers that have a tab on that document. Recipients that have an administrative role (Agent, Editor, or Intermediaries) or informational role (Certified Deliveries or Carbon Copies) can always see all the documents in an envelope, unless they are specifically excluded using this setting when an envelope is sent. Documents that do not have tabs are always visible to all recipients, unless they are specifically excluded using this setting when an envelope is sent. |
    ///| idCheckConfigurationName | No | String |Specifies authentication check by name. The names used here must be the same as the authentication type names used by the account (these name can also be found in the web console sending interface in the Identify list for a recipient). This overrides any default authentication setting.<br/><br/>_Example_:<br/> Your account has ID Check and SMS Authentication available and in the web console Identify list these appear as 'ID Check $' and 'SMS Auth $'. To use ID check in an envelope, the `idCheckConfigurationName` property must be  set to `ID Check $`. To use SMS, it must be set to `SMS Auth $` and you must add phone number information to the `smsAuthentication` node.|
    ///| iDCheckInformationInput | No | IdCheckInformationInput | This complex element contains input information related to a recipient ID check. It can include the following information.<br/><br/>*addressInformationInput*: Used to set recipient address information and consists of:<br/><br/>*addressInformation*: consists of six elements, with street2 and zipPlus4 being optional. The elements are: street1, street2, city, state, zip, zipPlus4\. The maximum number of characters in each element are: street1/street2 = 150 characters, city = 50 characters, state = 2 characters, and zip/zipPlus4 = 20 characters.<br/><br/>displayLevelCode: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*dobInformationInput*: Used to set recipient date of birth information and consists of:<br/><br/>*dateOfBirth*: Specifies the recipient's date, month and year of birth.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*ssn4InformationInput*: Used to set the last four digits of the recipient's SSN information and consists of:<br/><br/>*ssn4*: Specifies the last four digits of the recipient's SSN.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*ssn9InformationInput*: Used to set the recipient's SSN information. Note that the ssn9 information can never be returned in the response. The ssn9 input consists of:<br/><br/><br/><br/>*ssn9*: Specifies the recipient's SSN.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay. |
    ///| inheritEmailNotificationConfiguration | No | Boolean | Optional element. If true and the envelope recipient creates a DocuSign account after signing, the Manage Account Email Notification settings are used as the default settings for the recipient's account. |
    ///| note | No | String | A note that is unique to this recipient. This note is sent to the recipient via the signing email. The note displays in the signing UI near the upper left corner of the document on the signing screen.<br/>Maximum Length: 1000 characters. |
    ///| phoneAuthentication | No | RecipientPhoneAuthentication | Optional element. Contains the elements: <br/><br/> *recipMayProvideNumber*:Boolean. When set to **true** thenrecipient can use whatever phone number they choose to.<br/><br/> *senderProvidedNumbers*: ArrayOfString. A list of phone numbers the recipient can use. <br/>
    ///| recipientId | No | String | Unique for the recipient. It is used by the tab element to indicate which recipient is to sign the Document. |
    ///| requireIdLookup | No | Boolean | When set to **true**, the recipient is required to use the specified ID check method (including Phone and SMS authentication) to validate their identity. |
    ///| roleName | No* | String | Optional element. Specifies the role name associated with the recipient.<br/><br/>This is required when working with template recipients. |
    ///| routingOrder | Yes | String | This element specifies the routing order of the recipient in the envelope. |
    ///| smsAuthentication | No | senderProvidedNumbers | Optional element. Contains the element:<br/><br/>*senderProvidedNumbers*: Array that contains a list of phone numbers the recipient can use for SMS text authentication. |
    ///| templateAccessCodeRequired | No | Boolean | Optional element. Used only when working with template recipients. When set to **true** and the `TemplateLocked` parameter is set to **true**, the sender must enter an access code. |
    ///| templateLocked | No | Boolean | Optional element. Used only when working with template recipients. When set to **true**, the sender cannot change any attributes of the recipient. |
    ///| templateRequired | No | Boolean | Optional element. Used only when working with template recipients. When set to **true**,  the sender may not remove the recipient. |
    ///| identityVerification <a name="toto"></a> | No | identityVerification | Optional element. Specifies ID Verification applied on an envelope by workflow ID. <br/>See the [list](https://developers.docusign.com/docs/esign-rest-api/reference/accounts/identityverifications/list/) method in the [IdentityVerifications](https://developers.docusign.com/docs/esign-rest-api/reference/accounts/identityverifications/) resource for more information on how to retrieve workflow IDs available for an account. <br/>This can be used in addition to other [recipient authentication](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/recipients/auth/) methods. <br/>Note that ID Verification and ID Check are two distinct methods. ID Verification checks recipients' identity by verifying their ID while ID Check relies on data available on public records (such as current and former address). |
    ///
    ///<br>
    ///
    ///
    ///
    ///## Agent Recipient
    ///
    ///An agent recipient can add name and email information for recipients that appear after the agent in routing order.
    ///
    ///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
    ///
    ///<br>
    ///
    ///| Name | Required | Schema Type | Description |
    ///| :--- | :--- | :--- | :--- |
    ///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
    ///
    ///<br>
    ///
    ///
    ///
    ///## Carbon Copy Recipient
    ///
    ///Carbon copy recipients receive a copy of the envelope but don't need to sign, initial, date or add information to any of the documents. You can place this type of recipient in any routing order. Carbon copy recipients receive their copy of the envelope when the envelope reaches the recipient's order in the process flow and when the envelope is completed.
    ///
    ///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
    ///
    ///<br>
    ///
    ///| Name | Required | Schema Type | Description |
    ///| :--- | :--- | :--- | :--- |
    ///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
    ///
    ///<br>
    ///
    ///
    ///
    ///## Certified Delivery Recipient
    ///
    ///Certified delivery recipients must receive the completed documents for the envelope to be completed. However, they don't need to sign, initial, date or add information to any of the documents.
    ///
    ///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
    ///
    ///<br>
    ///
    ///| Name | Required | Schema Type | Description |
    ///| :--- | :--- | :--- | :--- |
    ///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
    ///
    ///<br>
    ///
    ///
    ///
    ///## Editor Recipient
    ///
    ///Editors have the same management and access rights for the envelope as the sender. They can make changes to the envelope as if they were using the Advanced Correct feature. This recipient can add name and email information, add or change the routing order and set authentication options for the remaining recipients. Additionally, this recipient can edit signature/initial tabs and data fields for the remaining recipients. The recipient must have a DocuSign account to be an editor.
    ///
    ///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
    ///
    ///<br>
    ///
    ///
    ///| Name | Required | Schema Type | Description |
    ///| :--- | :--- | :--- | :--- |
    ///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
    ///
    ///
    ///<br>
    ///
    ///
    ///## In-Person Signer Recipient
    ///
    ///An in-person recipient is a DocuSign user, acting as a Signing Host, who is in the same physical location as the signer.
    ///
    ///The following restrictions apply to using electronic notary when sending documents:
    ///
    ///* Authentication methods are allowed for the signer but not the notary.
    ///* The Sign On Paper, Document Markup, Field Markup and Change Signer options cannot be used for the documents.
    ///* Tabs may be assigned to the signer, but cannot be assigned to the notary.
    ///
    ///See [eNotary Resources][enotary-resources] in the DocuSign Support Center for more information about how the eNotary feature works.
    ///
    ///In addition to the [core parameters](#core-recipient-parameters), this type adds the following parameters:
    ///
    ///<br>
    ///
    ///
    ///| Name 	| Required 	| Schema Type 	| Description 	|
    ///|-----------------------	|-----------------------------------------------------	|-------------	|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------	|
    ///| inPersonSigningType 	| No 	| String 	| Specifies whether the envelope uses the eNotary feature. The accepted values are:<ul markdown=1><li>`inPersonSigner` The envelope uses the normal in-person signing flow.</li><li>`notary`: The envelope uses the eNotary signing flow.</li></ul> 	|
    ///| notaryHost 	| Yes, when `inPersonSigningType` is `notary` 	| NotaryHost 	| Sets the information for the notary host for the notary in person signing flow. The following information is required: <ul markdown=1> <li>`recipientId`: A unique ID number for the notary signing host. </li> <li>`name`: Specifies the notary's full legal name.</li> <li>`email`: Specifies the notary's email address. </li> </ul> 	|
    ///| autoNavigation 	| No 	| Boolean 	| Specifies whether auto navigation is set for the recipient. 	|
    ///| defaultRecipient 	| No 	| Boolean 	| When set to **true**, this is the default recipient for the envelope. This option is used when creating an envelope from a template. 	|
    ///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
    ///| hostName 	| Yes, when `inPersonSigningType` is `inPersonSigner` 	| String 	| The name of the signing host. This is the DocuSign user that is hosting the in-person signing session. 	|
    ///| hostEmail 	| Yes, when `inPersonSigningType` is `inPersonSigner` 	| String 	| The email address of the signing host. This is the DocuSign user that is hosting the in-person signing session. 	|
    ///| signerName 	| Yes, when `inPersonSigningType` is `inPersonSigner` 	| String 	| The in-person signer's full legal name. 	|
    ///| Name 	| Yes, when `inPersonSigningType` is `notary` 	| String 	| The full legal name of the signer in an eNotary flow. 	|
    ///| email 	| Yes, when `inPersonSigningType` is `notary` 	| String 	| The signer's email address in an eNotary flow. 	|
    ///| recipientSuppliesTabs 	| No 	| Boolean 	| Indicates whether the recipient supplies tabs in the document. 	|
    ///| signatureInfo 	| No 	| String 	| Optional element only used with the recipient types In Person Signers, Signers, and Witnesses.<br/><br/>Allows the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. 	|
    ///| signInEachLocation 	| No 	| Boolean 	| When set to **true** and the feature is enabled in the sender's account, the signing recipient is required to draw signatures and initials at each signature/initial tab (instead of adopting a signature/initial style or only drawing a signature/initial once). 	|
    ///| tabs 	| No 	| Tab 	| Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Specifies the Tabs associated with the recipient. See the [EnvelopeRecipientTabs resource](https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/) for more information about tabs. 	|
    ///|  	|  	|  	|  	|
    ///
    ///
    ///<br>
    ///
    ///
    ///
    ///## Intermediary Recipient
    ///
    ///An intermediary is a recipient who can, but is not required to, add name and email information for recipients at the same or subsequent level in the routing order, unless subsequent agents, editors or intermediaries are added.
    ///
    ///In addition to the [core parameters](#core-recipient-parameters), this recipient type has the following parameters:
    ///
    ///<br>
    ///
    ///
    ///| Name | Required | Schema Type | Description |
    ///| :--- | :--- | :--- | :--- |
    ///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
    ///
    ///<br>
    ///
    ///
    ///
    ///## Seal Recipient
    ///
    ///An electronic seal recipient is a legal entity rather than an actual person. Electronic Seals can be used by organizations and governments to show evidence of origin and integrity of documents. Even though electronic seals can be represented by a tab in a document, they do not require user interaction and apply automatically in the order specified by the sender. The sender is therefore the person authorizing usage of the electronic seal in the flow.
    ///
    ///Electronic seal recipients rely on a subset of core properties, described as follows, plus the `recipientSignatureProviders` parameter:
    ///
    ///
    ///<br>
    ///
    ///
    ///| Name&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Required&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Schema Type&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Description&nbsp;&nbsp;&nbsp;&nbsp; |
    ///| :--- | :--- | :--- | :--- |
    ///| recipientId| Yes | String | Indicates the unique ID of the applied electronic seal.<br/>|
    ///| routingOrder| No (default: 1) | String | Specifies the routing order of the electronic seal in the envelope. <br />The routing order assigned to your electronic seal cannot be shared with another recipient. It is recommended that you set a routing order for your electronic seals.. |
    ///| recipientSignatureProviders| Yes | String | Indicates which electronic seal to apply on documents when creating an envelope. |
    ///
    ///
    ///<br>
    ///
    ///
    ///
    ///By default, Electronic Seals apply on all documents in an envelope. However, the sealDocumentsWithTabsOnly property (see recipientSignatureProvider) allows you to seal only documents that have signHere tabs set for the Electronic Seal recipients.
    ///
    ///<br>
    ///
    ///
    ///
    ///## Signer Recipient
    ///
    ///A signer is a recipient who must sign, initial, date, or add data to form fields on the documents in the envelope.
    ///
    ///In addition to the [core parameters](#core-recipient-parameters), this recipient type adds the following parameters:
    ///
    ///<br>
    ///
    ///
    ///| Name | Required | Schema Type | Description |
    ///| :--- | :--- | :--- | :--- |
    ///| autoNavigation | No | Boolean | Specifies whether auto navigation is set for the recipient.|
    ///| defaultRecipient | No | Boolean | When set to **true**, this is the default recipient for the envelope. This option is used with the CreateEnvelopeFromTemplatesAndForms method. |
    ///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
    ///| isBulkRecipient | No | Boolean | Indicates whether the recipient is a bulk send recipient or not. |
    ///| recipientSuppliesTabs | No | Boolean | Indicates whether the recipient supplies tabs in the document. |
    ///| signInEachLocation | No | Boolean | When set to **true** and the feature is enabled in the sender's account, the signing recipient is required to draw signatures and initials at each signature/initial tab (instead of adopting a signature/initial style or only drawing a signature/initial once). |
    ///| signatureInfo | No | String | Optional element only used with recipient types In Person Signers, Signers, and Witnesses.<br/><br/>Allows the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. |
    ///| signerEmail | No | String | Optional element. The email address for an In-Person Signer recipient Type.<br/>Maximum Length: 100 characters. |
    ///| signerName | Yes | String | Required element with recipient type In Person Signers.<br/>Maximum Length: 100 characters.<br/><br/>The full legal name of a signer for the envelope. |
    ///| tabs | No | Tab | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Specifies the Tabs associated with the recipient. See the the [EnvelopeRecipientTabs resource](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/enveloperecipienttabs/)  for more information about tabs. |
    ///
    ///<br>
    ///
    ///[enotary-resources]: https://support.docusign.com/en/guides/ndse-user-guide-enotary-resources
    ///
    ///
    ///## Witness Recipient
    ///
    ///A witness is a recipient whose signature affirms that the identified signers have signed the documents in the envelope.
    ///
    ///In addition to the [core parameters](#core-recipient-parameters), this recipient type adds the following parameters:
    ///
    ///<br>
    ///
    ///
    ///| Name | Required | Schema Type | Description |
    ///| :--- | :--- | :--- | :--- |
    ///| autoNavigation | No | Boolean |	Specifies whether auto navigation is set for the recipient. |
    ///| defaultRecipient | No | Boolean | When set to **true**, this is the default recipient for the envelope. This option is used when creating an envelope from a template. |
    ///| documentVisibility | No | documentVisibility | A complex type that specifies which documents are visible to this recipient. |
    ///| isBulkRecipient | No | Boolean | Indicates whether the recipient is a bulk send recipient or not. |
    ///| recipientSignatureProviders | Yes | String | Indicates which electronic seal to apply on documents when creating an envelope. |
    ///| recipientSuppliesTabs | No | Boolean | Indicates whether the recipient supplies tabs in the document. |
    ///| recipientType | Yes | String | Indicates the recipient type. |
    ///|requireSignerCertificate | No | Boolean | Indicates whether the envelope requires a signer certificate for this recipient. |
    ///| requireSignOnPaper | No | Boolean | Specifies whether the signer must print, sign, and upload or fax the signed documents to DocuSign. |
    ///| signatureInfo | No | Boolean | Optional element only used with recipient types In Person Signers, Signers, and Witnesses. Enables the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. |
    ///| signInEachLocation | No | Boolean | When set to **true** and the feature is enabled in the sender's account, specifies that the signing recipient is required to sign and initial at each signature/initial tab (instead of once). |
    ///| signingGroupId | No | String | The id of the [signing group](https://support.docusign.com/en/guides/ndse-user-guide-signing-groups). |
    ///| signingGroupName | No | String | The display name for the signing group. Maximum Length: 100 characters. |
    ///| signingGroupUsers | No | userInfo | A complex type that contains information about the users in the signing group. |
    ///| witnessFor | Yes | String | Indicates the person or party for whom the recipient is a witness. |
    ///| witnessForGuid | Yes | String | GUID identifying the person or party for whom the recipient is a witness. |
    ///
    ///
    ///[ListLang]: https://developers.docusign.com/docs/esign-rest-api/reference/Accounts/Accounts/listSupportedLanguages/
    ///.
    pub fn envelope_recipients(&self) -> envelope_recipients::EnvelopeRecipients {
        envelope_recipients::EnvelopeRecipients::new(self.clone())
    }

    /// The EnvelopeRecipientTabs resource provides methods that enable you
    ///to add,
    ///update,
    ///and delete tabs
    ///from an envelope.
    ///Tabs are associated with a specific recipient
    ///in an envelope
    ///and are only used by the recipient types
    ///In Person Signers and Signers.
    ///
    ///<!-- START doctoc generated TOC please keep comment here to allow auto update -->
    ///<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
    ///**On this page**
    ///
    ///- [Tab Types](#tab-types)
    ///- [View Tab](#view-tab)
    ///- [Requesting Payments](#requesting-payments)
    ///- [Using Custom Tabs in Envelopes and Templates](#using-custom-tabs-in-envelopes-and-templates)
    ///- [Anchoring Tabs](#anchoring-tabs)
    ///- [Automatically Populating Tabs](#automatically-populating-tabs)
    ///
    ///<!-- END doctoc generated TOC please keep comment here to allow auto update -->
    ///
    ///
    ///## Tab Types
    ///
    ///Some tabs enable values to be entered by the signer.
    ///Those tabs' values can be preset either through the web browser
    ///or via the API. Other tab types use information that is already
    ///recognized by the DocuSign platform.
    ///These tabs cannot have their value updated on a per-tab basis
    ///by the API or via the browser. In some cases, the info might be
    ///settable using a different technique.
    ///For example, the Full name tab uses the signer's name,
    ///which is set elsewhere in the request.
    ///
    ///Here is the list of tabs and whether you can or cannot set their values in the tab definition:
    ///
    ///<br>
    ///
    ///| Tab Type                               | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
    ///| :------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    ///| Approve (`approve`)                    | Allows the recipient to approve documents without placing a signature or initials on the document. If the recipient clicks the tab during the signing process, the recipient is considered to have signed the document. No information is shown on the document of the approval, but it is recorded as a signature in the envelope history. This value **can't** be set.                                                                                                                                                                                                                                              |
    ///| Checkbox (`checkbox`)                  | Allows the recipient to select a yes/no (on/off) option. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
    ///| Company (`company`)                    | Displays the recipient's company name. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
    ///| Date Signed (`dateSigned`)             | Displays the date that the recipient signed the document. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
    ///| Date (`date`)                          | Allows the recipient to enter a date. Date tabs are one-line fields that allow date information to be entered in any format. The tooltip for this tab recommends entering the date as MM/DD/YYYY, but this is not enforced. The format entered by the signer is retained. If you need a particular date format enforced, DocuSign recommends using a Text tab with a validation pattern and a validation message to enforce the format. This value can be set.                                                                                                                                                        |
    ///| Decline (`decline`)                    | Allows the recipient the option of declining an envelope. If the recipient clicks the tab during the signing process, the envelope is voided. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                            |
    ///| Email Address (`emailAddress`)         | Displays the recipient's email as entered in the recipient information. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
    ///| Email (`email`)                        | Allows the recipient to enter an email address. This is a one-line field that checks that a valid email address is entered. It uses the same parameters as a Text tab, with the validation message and pattern set for email information.<br><br>When getting information that includes this tab type, the original value of the tab when the associated envelope was sent is included in the response. This value can be set.                                                                                                                                                                                        |
    ///| Envelope ID (`envelopeId`)             | Displays the envelope ID. Recipients cannot enter or change the information in this tab.  This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
    ///| First Name (`firstName`)               | Displays the recipient's first name. This tab takes the recipient's name as entered in the recipient information, splits it into sections based on spaces and uses the first section as the first name. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                  |
    ///| Formula Tab (`formulaTab`)             | The value of a formula tab is calculated from the values of other number or date tabs in the document. When the recipient completes the underlying fields, the formula tab calculates and displays the result. This value can be set.The `formula` property of the tab contains the references to the underlying tabs. See [Calculated Fields] in the DocuSign Support Center to learn more about formulas. If a formula tab contains a `paymentDetails` property, the tab is considered a payment item. See [Requesting Payments Along with Signatures] in the DocuSign Support Center to learn more about payments. |
    ///| Full Name (`fullName`)                 | Displays the recipient's full name. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    ///| Initial Here (`initialHere`)           | Allows the recipient to initial the document. May be optional. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
    ///| Last Name (`lastName`)                 | Displays the recipient's last name. This tab takes the recipient's name as entered in the recipient information, splits it into sections based on spaces and uses the last section as the last name. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                     |
    ///| List (`list`)                          | This tab offers a list of options to choose from. The `listItems` property is used to specify the selectable options. This value can be set,                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
    ///| Notarize (`notarize`)                  | Place this tab on a page to alert Notary recipients that they must take action. Only one notarize tab can appear on a page. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
    ///| Note (`note`)                          | Displays additional information, in the form of a note, for the recipient. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
    ///| Number (`number`)                      | Allows the recipient to enter numbers and decimal (.) points. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
    ///| Radio Group (`radioGroup`)             | This group tab is used to place radio buttons on a document. The `radios` property is used to add and place the radio buttons associated with the group. Only one radio button can be selected in a group. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                     |
    ///| Sign Here (`signHere`)                 | Allows the recipient to sign a document. May be optional. This value **can't** be set. <br/>**Note**: `signHere` tabs can also be used to add a visual representation for  an electronic seal in a document.                                                                                                                                                                                                                                                                                                                                                                                                          |
    ///| Signer Attachment (`signerAttachment`) | Allows the recipient to attach supporting documents to an envelope. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    ///| SSN (`ssn`)                            | A one-line field that allows the recipient to enter a Social Security Number. The SSN can be typed with or without dashes. It uses the same parameters as a Text tab, with the validation message and pattern set for SSN information.  This value can be set.                                                                                                                                                                                                                                                                                                                                                        |
    ///| Text (`text`)                          | Allows the recipient to enter any type of text. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
    ///| Title (`title`)                        | Displays the recipient's title. This value **can't** be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
    ///| View (`view`)                          | The View tab is used with the Approve tab to handle supplemental documents. This value can be set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
    ///| Zip (`zip`)                            | Allows the recipient to enter a ZIP code. The ZIP code can be five digits or nine digits in the ZIP+4 format. The zip code can be typed with or without dashes. It uses the same parameters as a Text tab, with the validation message and pattern set for ZIP code information. This value can be set.                                                                                                                                                                                                                                                                                                               |
    ///
    ///
    ///[approve]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[checkbox]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[company]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[dateSigned]:		    https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[date]:		          https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[decline]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[emailAddress]:     https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[email]:		        https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[envelopeId]:	    	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[firstName]:	    	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[formulaTab]:	    	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[fullName]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[initialHere]:	  	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[lastName]:		      https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[list]:	           	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[notarize]:         https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[note]:		          https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[number]:		        https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[radioGroup]:		    https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[signerAttachment]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[signHere]:	      	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[ssn]:	          	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[text]:	          	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[title]:	        	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[view]:		          https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///[zip]:	          	https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/#tab-types
    ///
    ///[calculatedfields]: https://support.docusign.com/en/guides/ndse-user-guide-calculated-fields
    ///[paymentguide]:     https://support.docusign.com/en/guides/requesting-payments-along-with-signatures
    ///
    ///
    ///
    ///
    ///
    ///
    ///## View Tab
    ///
    ///The View tab is used on supplemental documents.
    ///To learn more about using the View tab with
    ///supplemental documents, see
    ///[Using Supplemental Documents][usingsupdocs]
    ///in the [Sending Documents][sendenvelopes] section of
    ///the [Envelope: create][envelopecreate] method.
    ///
    ///[sendenvelopes]:  https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/create/
    ///[usingsupdocs]:   https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/documents/supplemental/
    ///[envelopecreate]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/create/
    ///
    ///<br>
    ///
    ///| Name              | Required  | Type    | Description |
    ///| :----             | :----     | :----   | :----       |
    ///| documentId        | Yes       | String  | The document ID number that the tab is placed on. This must refer to an existing Document's ID attribute. |
    ///| pageNumber        | Yes       | String  | Must be set to 1. |
    ///| recipientId       | Yes       | String  | The recipient associated with the tab. Must refer to an existing recipient's ID attribute. |
    ///| required          | No        | Boolean | If **true**, the recipient is required to select the supplemental document View button during signing. |
    ///| tabLabel          | No        | String  | The label used for the tab.  If an empty string is provided for this, an empty sting is used. If no value is provided, the tab type is used as the value. Maximum of 500 characters. |
    ///| templateLocked    | No        | Boolean | Optional. Used only when working with template tabs. If **true**, the attributes of the tab cannot be changed by the sender. |
    ///| templateRequired  | No        | Boolean | Optional. Used only when working with template tabs. If **true**, the tab cannot be removed by the sender. |
    ///| xPosition         | Yes       | String  | Required, but can be 0.  |
    ///| yPosition         | Yes       | String  | Required, but can be 0. |
    ///
    ///
    ///## Requesting Payments
    ///
    ///The Payments feature of the DocuSign eSignature REST API
    ///lets you collect payments
    ///along with signatures and other information.
    ///
    ///To send a request for payment
    ///and collect payments,
    ///you need a payment gateway account,
    ///which is the destination for the payments.
    ///Create a payment account
    ///with a supported payment gateway,
    ///and then connect the payment gateway account
    ///to your DocuSign account.
    ///To learn how to connect a payment gateway account
    ///to your DocuSign account
    ///see [Managing Payment Gateways][paymentgateways]
    ///in the DocuSign Support Center.
    ///You must connect and manage payment gateway accounts manually
    ///through the DocuSign Admin console
    ///and through your chosen payment gateway.
    ///There is no public API
    ///for connecting payment gateway accounts
    ///to DocuSign accounts
    ///or for managing payment gateway accounts.
    ///
    ///Currently
    ///[Stripe][stripe],
    ///[Braintree](https://www.braintreepayments.com/), and
    ///[Authorize.net](https://www.authorize.net/)
    ///are the supported payment gateways.
    ///
    ///### How Payments Work
    ///
    ///To make a request for payment,
    ///use a [`formulaTab`][formulatab]
    ///that has a
    ///[`paymentDetails`][paymentdetails] object.
    ///This object includes
    ///a list of [`paymentLineItem`][paymentlineitem] objects.
    ///Each line item refers to a [`number`][numbertab] tab
    ///that contains the value of the each item.
    ///The purpose of the line items
    ///is to transmit them to the payment gateway
    ///as metadata, so that you can use the information
    ///in the payment processor.
    ///
    ///**Note**: If the fileExtension parameter is not added in an API call, only base64 converted pdf files will be accepted.
    ///Any attempt to send a non pdf file without using fileExtension results in an error.
    ///
    ///This is an example request for two books.
    ///Each book is specified in the `number` tabs
    ///labeled "Hamlet" and "Tempest".
    ///The books are referenced as line items
    ///by their tab labels
    ///within the `paymentDetails` object
    ///of a `formula` tab.
    ///The formula of the `formula` tab
    ///add the values of the same two `number` tabs.
    ///
    ///```json
    /// {
    ///  "documents": [
    ///    {
    ///      "documentBase64": "<base64-encoded PDF document>",
    ///      "documentId": "1",
    ///      "fileExtension": "pdf",
    ///      "name": "payment.pdf"
    ///    }
    ///  ],
    ///  "emailSubject": "Order Some Books",
    ///  "recipients": {
    ///    "signers": [
    ///      {
    ///        "email": "vreader@example.com",
    ///        "name": "Voracious Reader",
    ///        "recipientId": "1",
    ///        "routingOrder": "1",
    ///        "tabs": {
    ///          . . .
    ///          "numberTabs": [
    ///            {
    ///              "value": "10.00",
    ///              "width": 78,
    ///              "required": "true",
    ///              "locked": "true",
    ///              "tabLabel": "Hamlet",
    ///              "documentId": "1",
    ///              "pageNumber": "1",
    ///              "xPosition": "323",
    ///              "yPosition": "134"
    ///            },
    ///            {
    ///              "value": "10.00",
    ///              "width": 78,
    ///              "required": "true",
    ///              "locked": "true",
    ///              "tabLabel": "Tempest",
    ///              "documentId": "1",
    ///              "pageNumber": "1",
    ///              "xPosition": "323",
    ///              "yPosition": "154"
    ///            }
    ///          ],
    ///          "formulaTabs": [
    ///            {
    ///              "required": "true",
    ///              "formula": "([Hamlet] + [Tempest]) * 100",
    ///              "roundDecimalPlaces": "2",
    ///              "paymentDetails": {
    ///                "currencyCode": "USD",
    ///                "lineItems": [
    ///                  {
    ///                    "name": "Hamlet",
    ///                    "description": "The Danish Play",
    ///                    "itemCode": "SHAK1",
    ///                    "amountReference": "Hamlet"
    ///                  },
    ///                  {
    ///                    "name": "Othello",
    ///                    "description": "The one with Caliban in it",
    ///                    "itemCode": "SHAK2",
    ///                    "amountReference": "Tempest"
    ///                  }
    ///                ],
    ///                "gatewayAccountId": "e76668b4-xxxx-xxxx-xxxx-a208d659e490"
    ///              },
    ///              "tabLabel": "Payment1",
    ///              "documentId": "1",
    ///              "pageNumber": "1",
    ///              "xPosition": 300,
    ///              "yPosition": 200
    ///            }
    ///          ]
    ///        }
    ///      }
    ///    ]
    ///  },
    ///  "status": "sent"
    /// }
    /// ```
    ///
    ///Use the
    ///[EnvelopeRecipients: list][enveloperecipientslist] method
    ///to check the status of a payment.
    ///When the payment is successful,
    ///the `status` property of
    ///the [`paymentDetails`][paymentdetails] object
    ///is `payment_complete`.
    ///
    ///```json
    /// {
    ///  "signers": [
    ///    {
    ///      "tabs": {
    ///        . . .
    ///        "numberTabs": [
    ///          {
    ///            "value": "10.00",
    ///            "tabLabel": "Hamlet",
    ///            "documentId": "1",
    ///            "recipientId": "1",
    ///            "pageNumber": "1",
    ///            "xPosition": "323",
    ///            "yPosition": "134",
    ///          },
    ///          {
    ///            "value": "10.00",
    ///            "tabLabel": "Tempest",
    ///            "documentId": "1",
    ///            "recipientId": "1",
    ///            "pageNumber": "1",
    ///          }
    ///        ],
    ///        "formulaTabs": [
    ///          {
    ///            "formula": "([Hamlet] + [Tempest]) * 100",
    ///            "roundDecimalPlaces": "2",
    ///            "paymentDetails": {
    ///              "status": "payment_complete",
    ///              "currencyCode": "USD",
    ///              "lineItems": [
    ///                {
    ///                  "name": "Hamlet",
    ///                  "description": "The Danish Play",
    ///                  "itemCode": "SHAK1",
    ///                  "amountReference": "Hamlet"
    ///                },
    ///                {
    ///                  "name": "Tempest",
    ///                  "description": "The one with Caliban in it",
    ///                  "itemCode": "SHAK2",
    ///                  "amountReference": "Tempest"
    ///                }
    ///              ],
    ///              "gatewayAccountId": "e76668b4-xxxx-xxxx-xxxx-a208d659e490"
    ///            },
    ///            "value": "20",
    ///            "required": "true",
    ///            "locked": "false",
    ///            "tabLabel": "Payment1",
    ///            "documentId": "1",
    ///            "recipientId": "1",
    ///            "pageNumber": "1",
    ///          }
    ///        ]
    ///      },
    ///      "creationReason": "sender",
    ///      "email": "vreader@example.com",
    ///      "name": "Voracious Reader",
    ///      "recipientId": "1",
    ///      "requireIdLookup": "false",
    ///      "status": "completed",
    ///    }
    ///  ],
    ///  . . .
    /// }
    /// ```
    ///
    ///#### How to make a request for future payments
    ///
    ///Use the following steps to make a request to collect a signer's payment method for future use:
    ///
    ///1. Add a text tab with a descriptive `tabLabel` such as `FuturePayment`.
    ///2. In the formula tab's `paymentDetails` object, add a `lineItem` object that references the `tabLabel` from step 1.
    ///
    ///**Note**: Do not include this new `lineItem` in formula calculations.
    ///
    ///The following example builds on the previous code block to also collect a payment method for future use:
    ///
    ///```
    /// {
    ///  "documents": [
    ///    {
    ///      "documentBase64": "<base64-encoded PDF document>",
    ///      "documentId": "1",
    ///      "fileExtension": "pdf",
    ///      "name": "payment.pdf"
    ///    }
    ///  ],
    ///  "emailSubject": "Order Some Books",
    ///  "recipients": {
    ///    "signers": [
    ///      {
    ///        "email": "vreader@example.com",
    ///        "name": "Voracious Reader",
    ///        "recipientId": "1",
    ///        "routingOrder": "1",
    ///        "tabs": {
    ///          "numberTabs": [
    ///            {
    ///              "value": "10.00",
    ///              "width": 78,
    ///              "required": "true",
    ///              "locked": "true",
    ///              "tabLabel": "Hamlet",
    ///              "documentId": "1",
    ///              "pageNumber": "1",
    ///              "xPosition": "323",
    ///              "yPosition": "134"
    ///            },
    ///            {
    ///              "value": "10.00",
    ///              "width": 78,
    ///              "required": "true",
    ///              "locked": "true",
    ///              "tabLabel": "Tempest",
    ///              "documentId": "1",
    ///              "pageNumber": "1",
    ///              "xPosition": "323",
    ///              "yPosition": "154"
    ///            }
    ///          ],
    ///          "textTabs": [
    ///            {
    ///              "value": "",
    ///              "width": 78,
    ///              "required": "true",
    ///              "locked": "true",
    ///              "tabLabel": "FuturePayment",
    ///              "documentId": "1",
    ///              "pageNumber": "1",
    ///              "xPosition": "323",
    ///              "yPosition": "174"
    ///            }
    ///          ],
    ///          "formulaTabs": [
    ///            {
    ///              "required": "true",
    ///              "formula": "([Hamlet] + [Tempest]) * 100",
    ///              "roundDecimalPlaces": "2",
    ///              "paymentDetails": {
    ///                "currencyCode": "USD",
    ///                "paymentOption": "save_and_authorize",
    ///                "lineItems": [
    ///                  {
    ///                    "name": "Hamlet",
    ///                    "description": "The Danish Play",
    ///                    "itemCode": "SHAK1",
    ///                    "amountReference": "Hamlet"
    ///                  },
    ///                  {
    ///                    "name": "Othello",
    ///                    "description": "The one with Caliban in it",
    ///                    "itemCode": "SHAK2",
    ///                    "amountReference": "Tempest"
    ///                  },
    ///                  {
    ///                    "name": "Request books",
    ///                    "description": "collect Payment method",
    ///                    "itemCode": "",
    ///                    "amountReference": "FuturePayment"
    ///                  }
    ///                ],
    ///                "gatewayAccountId": "e76668b4-xxxx-xxxx-xxxx-a208d659e490"
    ///              },
    ///              "tabLabel": "Payment1",
    ///              "documentId": "1",
    ///              "pageNumber": "1",
    ///              "xPosition": 300,
    ///              "yPosition": 200
    ///            }
    ///          ]
    ///        }
    ///      }
    ///    ]
    ///  },
    ///  "status": "sent"
    /// }
    /// ```
    ///
    ///### Some Things to Keep in Mind About Payments
    ///
    ///* An envelope is not completed until all payments are completed.
    ///
    ///* If a DocuSign account Administrator
    ///  deletes a payment gateway account connection
    ///  DocuSign cancels all in-process envelopes
    ///  that reference the deleted payment gateway account.
    ///
    ///* If the sender voids an envelope,
    ///  all payment authorizations are canceled.
    ///
    ///* If a required recipient refuses to sign,
    ///  all authorized payments are canceled.
    ///
    ///* If a required recipient's payment fails authorization,
    ///  DocuSign attempts to recover
    ///  by sending the recipient
    ///  notification about the failed payment authorization.
    ///  The recipient has the opportunity
    ///  to correct the payment method information.
    ///
    ///* Each recipient's payment is authorized separately.
    ///  Accounts are charged and payment made
    ///  after all required tabs are completed,
    ///  and all payments in an envelope for all recipients are authorized.
    ///
    ///* Refunds are not supported.
    ///  Conduct refunds or payment changes
    ///  with the payment gateway separately from DocuSign.
    ///
    ///* At this time, DocuSign does not charge a per-transaction fee.
    ///
    ///
    ///[enveloperecipientslist]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipients/list/
    ///[formulatab]:             https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/create/
    ///[ISO4217]:                https://en.wikipedia.org/wiki/ISO_4217
    ///[numbertab]:              https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/create/
    ///[paymentgateways]:        https://support.docusign.com/en/guides/managing-payment-gateways
    ///[paymentguide]:           https://support.docusign.com/en/guides/requesting-payments-along-with-signatures
    ///[paymentlineitem]:        https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/create/
    ///[paymentdetails]:         https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeRecipientTabs/create/
    ///[stripe]:                 https://stripe.com/
    ///
    ///
    ///## Using Custom Tabs in Envelopes and Templates
    ///
    ///Custom Tabs can be added to envelopes and templates
    ///by setting the `customTabId` property
    ///when creating an envelope or template recipient
    ///or when adding a new tab for an existing recipient.
    ///The custom tab must be added as the correct tab type.
    ///For example if the custom tab type is text, it cannot be used as a number tab.
    ///
    ///When the `customTabId` property is set,
    ///the new tab inherits all the custom tab properties.
    ///Required information that is not included in the custom tab,
    ///such as document ID and page ID, must be included when adding the tab.
    ///If the custom tab does not have anchor settings, the X and Y positions must be included.
    ///
    ///After the tab is created,
    ///it is treated as any other tab for updating or deleting.
    ///
    ///## Anchoring Tabs
    ///
    ///The tab anchoring option
    ///allows you to send documents for signature
    ///that do not have a fixed layout or format.
    ///In these documents you might not know
    ///the absolute location of the tabs
    ///when you design your API client application because the tabs must move with text.
    ///As an alternative to sending X and Y coordinates for tabs,
    ///the DocuSign Service can derive an anchor location for the tab
    ///by correlating anchor information to data within the document.
    ///
    ///When the DocuSign Service receives a request that contains tabs
    ///with anchor information,
    ///it searches the document for instances of the `anchorString` property.
    ///When found,
    ///it places a tab of the specified type for the designated recipient.
    ///Tab positions are established by setting offsets for the tab.
    ///
    ///When you apply tabs to the document,
    ///DocuSign does not remove or replace the text in the `anchorString` property. You can hide codified anchors by using the same font color as the background of the document. So the anchor can be used by DocuSign processes and it will not be visible on the document.
    ///
    ///To use an anchoring option:
    ///
    ///1. Identify the location in the document by text string. You can use a pre-existing text string or add a new one.
    ///For best performance DocuSign recommends using single word anchor strings when possible, especially when there are a large number of pages in the envelope.
    ///For example, you might want to add a Sign Here tab to the "Borrower's Signature" lines in a document, but that phrase might occur in places in the document where you don't want to tab to appear. In this case, you could add the text "BorrowerSignHere" in white font color (so that isn't visible in the document) to all the places you want Sign Here tabs to appear and use "BorrowerSignHere" as the anchor string.
    ///1. Reference the anchor through the `anchorString` property of the tab.
    ///1. Determine the offset from the anchor string location to where the tab should be placed.
    ///
    ///Setting a positive value in the `anchorXOffset` property moves the tab right on the page and positive values in the  `anchorYoffset` prove moves the tab down the page. The `anchorUnits` property specifies the units used for the offsets.
    ///For Sign Here and Initial Here tabs the bottom-left of the anchor string is equivalent to position (0,0), and the bottom-left of the tab graphic is placed relative to that.
    ///For all other tabs the bottom-left of the anchor string is equivalent to position (0,0), and the top-left of the tab graphic is placed relative to that.
    ///DocuSign does not currently provide tools to derive the offset values. Determination of the proper offset will likely require some trial-and-error.
    ///
    ///### Rules for working with anchor tags
    ///
    ///When anchor tabs are used, all documents in the envelope are searched for the `anchorString` property.
    ///
    ///* You set the text of the anchor string in the `anchorString` property. DocuSign tabs are created for each instance of the `anchorString` property within the document, so special care must be taken to establish unique anchor strings that do not result in unintentional tabs.
    ///* You cannot use the same anchored tab for different recipients for the same document.
    ///* The DocuSign system cannot search for text that is embedded in an image when checking for anchor strings.
    ///* X or Y offsets supplied for a tab apply to all instances of the tab in the document. To use different offsets at different locations in the document for the same recipient, create multiple, unique anchor tabs.
    ///* If the Y offset value of an anchor string would force a tab outside of the page boundaries, the tag is placed at the page boundary. If the X offset value places a tab outside of the page boundaries, the error message `Invalid_User_Offset` is sent. The error message includes the X offset that resulted in the error.
    ///* The system does not support an anchor string embedded in the form of a PDF X-object in the document.
    ///* The system does not re-flow the text that surrounds the anchor tabs. It is the responsibility of the document author to provide sufficient white space to contain the potential width of the ultimate tab value.
    ///
    ///### Tips and Tricks
    ///
    ///The following are tips for effective use of anchor tags:
    ///* In order to avoid unintentional conflicts between text contained in an `anchorString` property and the text that naturally exists in documents, establish a codified syntax for the anchor string text that is unlikely to appear elsewhere in a document.
    ///* Develop an extensible and consistent syntax that can be used across multiple document types.
    ///* Especially for documents that have variable numbers of tabs or signers, author the source document to include hidden anchor tabs for all potential signers/permutations. Then, control the tabs that are actually placed by including/excluding the anchor tabs in the request. This approach allows a single document to be used for all use cases instead of maintaining separate documents for each scenario.
    ///
    ///## Automatically Populating Tabs
    ///
    ///If you want similar tab types
    ///to automatically populate with the same data,
    ///you must follow these guidelines:
    ///
    ///* Each `tabLabel` entry must have the characters
    ///  `\\*` in front of the label.
    ///  If you omit the `\\*` prefix,
    ///  only the first occurrence of the tab is populated.
    ///
    ///  When automatically populating tabs,
    ///  the `tabLabel` must not contain any spaces.
    ///  In the JSON example below,
    ///  the Text tabs  properties `StudentLastName` and `StudentFirstName`
    ///  will be auto-populated as specified ("Doe" and "John")
    ///  each place they appear throughout the envelope.
    ///
    ///  ```
    ///  "tabs": {
    ///    "textTabs": [
    ///    {
    ///      "tabLabel": "\\*StudentLastName",
    ///      "value": "Doe"
    ///    },
    ///    {
    ///      "tabLabel": "\\*StudentFirstName",
    ///      "value": "John"
    ///    }]
    ///  }
    ///  ```
    ///
    ///* Note that `\\*` matches _anything_.  If you were to add
    ///  another tab with the `tabLabel` set to `\\*Name` to the
    ///  example above, it would end up matching the other two
    ///  labels as well.
    ///
    ///* Each occurrence of the tab must have identical properties.
    ///
    ///  For example, suppose there are two Text tabs in a document,
    ///  each with `tabLabel` set to `Name`.
    ///  If one tab has the `bold` property set to **true**,
    ///  and the other has the `bold` property set to **false**,
    ///  only the first one will be populated.
    ///  In order to automatically populate both occurrences
    ///  of the `Name` Text tabs,
    ///  the `bold` property must be set to the same value for both tabs.
    ///.
    pub fn envelope_recipient_tabs(&self) -> envelope_recipient_tabs::EnvelopeRecipientTabs {
        envelope_recipient_tabs::EnvelopeRecipientTabs::new(self.clone())
    }

    /// The EnvelopeDocumentTabs resource provides methods that enable you to manage tabs in envelopes. For a complete list of options, see the following Properties section.
    pub fn envelope_document_tabs(&self) -> envelope_document_tabs::EnvelopeDocumentTabs {
        envelope_document_tabs::EnvelopeDocumentTabs::new(self.clone())
    }

    /// The EnvelopeDocumentTabs resource provides methods that enable you to manage tabs in a template.
    pub fn template_document_tabs(&self) -> template_document_tabs::TemplateDocumentTabs {
        template_document_tabs::TemplateDocumentTabs::new(self.clone())
    }

    /// The `EnvelopeViews` resource provides methods that return URLs that you can embed into your application to provide access to the DocuSign UI.
    ///
    ///The following Envelope Views are available:
    ///
    ///-  Console View: The authentication view of the DocuSign UI.
    ///-  Correct View: The correction view of the DocuSign UI.
    ///-  Edit View: The editing view of the DocuSign UI.
    ///   **Note**: This provides the same functionality as the sender view.
    ///-  Recipient View: The view the recipient sees in the DocuSign UI.
    ///-  Shared Recipient View: The view a user sees of a shared envelope in the DocuSign UI.
    ///-  Sender View: The sending view of the DocuSign UI.
    pub fn envelope_views(&self) -> envelope_views::EnvelopeViews {
        envelope_views::EnvelopeViews::new(self.clone())
    }

    /// .
    pub fn account_seal_providers(&self) -> account_seal_providers::AccountSealProviders {
        account_seal_providers::AccountSealProviders::new(self.clone())
    }

    /// The Accounts resource provides methods that allow you to create, delete, and manage your accounts.
    pub fn accounts(&self) -> accounts::Accounts {
        accounts::Accounts::new(self.clone())
    }

    /// Standards-Based Signatures (SBS) is the label used to describe DocuSign's suite of signatures that comply with regional and industry regulations, such as the electronic IDentification, Authentication and trust Services (eIDAS) regulation in Europe.
    ///
    ///## Feature Differences When Using Standards-Based Signatures
    ///
    ///Some DocuSign features are not compatible with Standards-Based Signatures, while others work somewhat differently. It's important to understand these key differences.
    ///
    ///### DocuSign Features Not Compatible with SBS
    ///
    ///- Attachment by fax
    ///- Concatenation of signer attachments
    ///- Legacy digital signatures
    ///- Markup
    ///- Notary
    ///
    ///### DocuSign Features That Change with SBS
    ///
    ///The following features work slightly differently with SBS:
    ///
    ///- **Advanced Correct**: After the first signature, adding or deleting a field is not allowed. This behavior occurs because SBS does not allow adding or removing form fields after a digital signature has already been applied to a PDF.
    ///- **Downloading Combined Envelopes**: A combined PDF is not digitally signed. This behavior occurs because concatenating digitally signed PDFs breaks the digital signatures on the PDFs.
    ///- **Freeform Signing**: After someone has signed, allows only signature and initials on free-form. This behavior occurs because if another signer has already signed the document, adding fields other than signature fields will break the existing digital signatures on the document.
    ///- **Watermarks**: All watermarks are added as PDF annotations. This behavior occurs because burning the watermark into the PDF will break the digital signatures on the document.
    ///- **Wet Signing**: Wet-signed documents are added as new documents to envelopes. This behavior results in the uploaded or faxed, physically signed document being added as a new document to the envelope. This new document gets only a platform seal.
    ///
    ///For more information, see [Standards Based Signatures](https://developers.docusign.com/esign-rest-api/guides/standards-based-signatures).
    pub fn account_signature_providers(
        &self,
    ) -> account_signature_providers::AccountSignatureProviders {
        account_signature_providers::AccountSignatureProviders::new(self.clone())
    }

    /// The Billing resource provides methods that allow you to manage the billing plans,associated with an account.
    pub fn billing_plans(&self) -> billing_plans::BillingPlans {
        billing_plans::BillingPlans::new(self.clone())
    }

    /// The Invoices resource provides methods that allow you to manage the invoices for an account.
    pub fn invoices(&self) -> invoices::Invoices {
        invoices::Invoices::new(self.clone())
    }

    /// The Payments resource provides methods that allow you to manage payments for an account.
    ///
    ///These calls can only be used by users with account administrator privileges.
    pub fn payments(&self) -> payments::Payments {
        payments::Payments::new(self.clone())
    }

    /// The CloudStorage resource provides methods that allow you to list files stored on your cloud storage provider.
    pub fn cloud_storage(&self) -> cloud_storage::CloudStorage {
        cloud_storage::CloudStorage::new(self.clone())
    }

    /// The following providers are supported:
    ///
    ///* Google Drive
    ///* Dropbox
    ///* Box
    ///* Evernote
    ///* OneDrive
    ///
    ///To use cloud storage files, you must first give DocuSign access to your cloud storage provider. You can disconnect authorized a cloud storage provider at any time.
    pub fn cloud_storage_providers(&self) -> cloud_storage_providers::CloudStorageProviders {
        cloud_storage_providers::CloudStorageProviders::new(self.clone())
    }

    /// The `ConnectConfigurations` resource enables you to configure the DocuSign Connect service for your account.
    ///
    ///You can use this resource to configure account-level webhooks that send notifications about every envelope sent from your account. You can set account-level webhooks to listen for events for envelopes sent by a specific user on your account, by multiple specific users, or from any of the users on your account. These events will be tracked, and can be delivered to a listening application.
    ///
    ///**Note**: To use DocuSign Connect, it must be enabled in your DocuSign account. It is not enabled by default.
    ///
    ///## Aggregated Messages
    ///
    ///To avoid duplicate simultaneous events, you can configure Connect to aggregate similar events into a single delivery. By default, aggregation is enabled on all Connect configurations. Similar or simultaneous events will be aggregated so your listener doesn't receive extraneous messages.
    ///
    ///For example, when the final recipient signs an envelope,  the system delivers a single, aggregated Connect event, rather than separate Recipient: complete and Envelope: complete messages. This aggregation process ensures that you only receive the minimal viable number of messages about an envelope's life cycle.
    ///
    ///## Send Individual Messages
    ///
    ///When you enable Send Individual Messages (SIM) mode on a Connect configuration, DocuSign will deliver notifications for all envelope events individually. In contrast with aggregated messages, when a final recipient completes an envelope, your listener will receive a single Recipient: complete event followed by a single Envelope: complete event for the final participating party on the agreement.  If you need more granular control over event notifications, you can enable SIM mode in the Admin area of the RADmin console. For more information about SIM mode, see [Using Connect's New Send Individual Messages Feature](https://www.docusign.com/blog/dsdev-connect-send-individual-messages/).
    ///
    ///**Note**: To create an envelope-level webhook instead of using account-level webhooks, use the Envelopes::Create method and add an `eventNotification` object to an envelope object.
    pub fn connect_configurations(&self) -> connect_configurations::ConnectConfigurations {
        connect_configurations::ConnectConfigurations::new(self.clone())
    }

    /// The ConnectEvents resource provides methods that allow you to read, delete, and republish the connect logs associated with an envelope.
    pub fn connect_events(&self) -> connect_events::ConnectEvents {
        connect_events::ConnectEvents::new(self.clone())
    }

    /// The CustomTabs resource provides methods that allow you create and manage custom tabs based on the existing DocuSign tabs.
    ///
    ///You can create a tab with pre-defined properties, such as a text tab with a certain font type and validation pattern. Users can access the custom tabs when sending documents through the DocuSign web application.
    ///
    ///Custom tabs can be created based on the  approve, checkbox, company, date, date signed, decline, email, email address, envelope ID, first name, formula, full name, initial here, last name, list, note, number, radio, sign here, signer attachment, SSN, text, title, and zip tabs.
    pub fn custom_tabs(&self) -> custom_tabs::CustomTabs {
        custom_tabs::CustomTabs::new(self.clone())
    }

    /// The RequestLogs resource provide methods that allow you to retrieve and delete the API request log files.
    ///
    ///The log files contain the API requests associated with your integration. They can aid you in troubleshooting specific issues within an integration, or if DocuSign Support requests an API trace log.
    pub fn request_logs(&self) -> request_logs::RequestLogs {
        request_logs::RequestLogs::new(self.clone())
    }

    /// The Resources resource provides a method which retrieves the base resources that are available.
    pub fn resources(&self) -> resources::Resources {
        resources::Resources::new(self.clone())
    }

    /// The Services resource provides a method that allow you to retrieve the available service versions.
    pub fn services(&self) -> services::Services {
        services::Services::new(self.clone())
    }

    /// The `EnvelopeConsumerDisclosures` resource provides methods that enable you to retrieve the Electronic Record and Signature Disclosure (ERSD) for an envelope recipient. This is the disclosure that displays to each new recipient who is going to sign or add other information, or who is required to view the documents you send to them. The recipient must read and agree to the terms of the disclosure before they can access and take action on the documents you send. The ERSD does not apply to copy-only recipients, but does apply to recipients who must sign or view your documents.
    ///
    ///You can retrieve either the default ERSD that DocuSign provides for U.S.-based transactions, or a custom ERSD.
    ///
    ///## Languages
    ///
    ///You specify the language of the disclosure version that you want to retrieve by using the `langCode` parameter.
    ///
    ///**Important**: The system does not translate the ERSD for you. An account administrator must create a version of the account-level disclosure for each language that your signers use.
    ///
    ///For more information, see [Legal Disclosure](https://support.docusign.com/en/guides/ndse-admin-guide-legal-disclosure).
    pub fn envelope_consumer_disclosures(
        &self,
    ) -> envelope_consumer_disclosures::EnvelopeConsumerDisclosures {
        envelope_consumer_disclosures::EnvelopeConsumerDisclosures::new(self.clone())
    }

    /// <!-- resources aren't rendered the same way
    ///     as other pages. This is a little hack to
    ///     make the headings work better -->
    ///<style>
    ///h1, h2, h3 {
    ///  margin-top: 1em;
    ///}
    ///</style>
    ///
    ///The EnvelopeDocuments resource provides methods
    ///that manage documents in an envelope.
    ///You can:
    ///* add one or more documents to the envelope
    ///* retrieve one or more documents from the envelope
    ///* delete documents from the envelope
    ///
    ///All of the methods in this resource
    ///operate on on an existing envelope.
    ///Before you can add documents
    ///to an envelope,
    ///you must first create it
    ///with the [Envelopes: create][envelopescreate] method.
    ///
    ///
    ///[envelopescreate]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/create/
    ///
    ///.
    pub fn envelope_documents(&self) -> envelope_documents::EnvelopeDocuments {
        envelope_documents::EnvelopeDocuments::new(self.clone())
    }

    /// The EnvelopeEmailSettings provide methods that allow you to manage the email override settings for an envelope.
    ///
    ///Email override settings change the reply to email address, name, or the BCC for email archive information, for the envelope. Note that changing email settings will only affect email communications that occur after the addition was made.
    ///
    ///The BCC Email address feature is designed to provide a copy of all email communications for external archiving purposes. DocuSign recommends that envelopes sent using the BCC for Email Archive feature, including the BCC Email Override option, include additional signer authentication options. To send a copy of the envelope to a recipient who does not need to sign, use a Carbon Copies or Certified Deliveries Recipient Type.
    pub fn envelope_email_settings(&self) -> envelope_email_settings::EnvelopeEmailSettings {
        envelope_email_settings::EnvelopeEmailSettings::new(self.clone())
    }

    /// The Envelope resource provides methods that allow you to manipulate and monitor envelopes.
    ///
    ///Once you have authenticated the user you can use the Envelopes: createEnvelope method to create an envelope. You can call the EnvelopeDocuments: update or EnvelopeDocuments: updateList method to add additional documents.
    ///
    ///Setting the `status` property on the envelope to `sent` allows you to send it or `created` to save it as a draft.
    ///
    ///You can receive envelope event notifications by setting the `eventNotification` properties. When the envelope or recipient status changes to one of the specified status codes, a notification is sent to a URL that you specify.
    ///
    ///If you have an envelope that you have previously saved, you can modify the subject and message, send it, void it, or place it in the purge queue using the Envelope: update method.
    ///
    ///In addition to receiving notifications you can monitor the status of the envelopes using the following methods:
    ///* Envelope: getEnvelope - To get the status of a envelope.
    ///* Envelope: listStatus - To get the envelope status for a set of envelopes.
    ///* Envelope: listStatusChanges - To get status changes information for one or more envelopes.
    ///
    ///If you need to delete a page from a document in an envelope, use the Envelope: deleteDocumentPage method.
    ///
    ///The resource also includes a number of methods that allow you to retrieve and set the initials and signature for certain types of recipients on the document.
    pub fn envelopes(&self) -> envelopes::Envelopes {
        envelopes::Envelopes::new(self.clone())
    }

    /// The EnvelopeTemplates resource provides methods that allow you to add and delete templates on envelopes and documents.
    pub fn envelope_templates(&self) -> envelope_templates::EnvelopeTemplates {
        envelope_templates::EnvelopeTemplates::new(self.clone())
    }

    /// The Folders resource provides methods that allow you to view contents of folders on the account and move envelopes between folders.
    pub fn folders(&self) -> folders::Folders {
        folders::Folders::new(self.clone())
    }

    /// For the custom groups you define for your account, you can assign brands to specify the ones that group members can use. Group members can use the available brands when they send envelopes or create templates. For more information, see [Assign Brands to Groups](https://support.docusign.com/en/guides/ndse-admin-guide-assign-brands-to-groups).
    pub fn group_brands(&self) -> group_brands::GroupBrands {
        group_brands::GroupBrands::new(self.clone())
    }

    /// The Groups resource provides methods that allow you to manage groups for the account.
    ///
    ///Groups can be used to help manage users by associating users with a group. A group can be associated with a Permission Profile, which sets the user permissions for users in that group without having to set the `userSettings` property for each user. You are not required to set Permission Profiles for a group, but this makes it easier to manage user permissions for a large number of users. Groups can also be used with template sharing to limit user access to templates.
    pub fn groups(&self) -> groups::Groups {
        groups::Groups::new(self.clone())
    }

    /// The GroupUsers resource provides methods that allow you to manage the users in a group.
    pub fn group_users(&self) -> group_users::GroupUsers {
        group_users::GroupUsers::new(self.clone())
    }

    /// The SigningGroups resource provides methods that allow you manage signing groups.
    ///
    ///Signing Groups allow you to create a group of people to which an envelope is sent. Any member of that group can open an envelope and sign the documents in the envelope with their own signature, even though a signature field was not directly assigned to them. When the Signing Group option is used, group members that open and sign the envelope are tracked in the envelope history and certificate.
    ///
    ///When one group member opens the envelope, it is temporarily locked and if other members try to open the envelope they will see a message saying the envelope is currently opened. If the group member exits the envelope without finishing the lock expires, allowing other group members to open and complete the envelope.
    ///
    ///When the envelope is complete, all members of the group will receive a completed notification and can access the completed envelope.
    ///The envelope history and Certificate of Completion will show that the envelope was sent to a signing group and record which members viewed and signed the envelope.
    ///
    ///An account can have a maximum of 50 signing groups. Each signing group can have a maximum of 50 group members.
    ///
    ///The Signing Groups feature is only supported in certain DocuSign Enterprise and System Automated Premium plans. Your account might not support this option. To access this functionality, contact your Account Manager or DocuSign Support (support@docusign.com) for assistance.
    ///
    ///For more information, see [Signing Groups](https://support.docusign.com/en/guides/ndse-user-guide-signing-groups).
    pub fn signing_groups(&self) -> signing_groups::SigningGroups {
        signing_groups::SigningGroups::new(self.clone())
    }

    /// The SigningGroupUsers resource provides methods that allow you to manage users in Signing Groups.
    pub fn signing_group_users(&self) -> signing_group_users::SigningGroupUsers {
        signing_group_users::SigningGroupUsers::new(self.clone())
    }

    /// The TemplateRecipients resource allows you manage the recipients of an template.
    ///
    ///The exact parameters associated with a recipient depend on the recipient type. There are seven recipient types: Agents, Carbon Copies, Certified Deliveries, Editors, In Person Signers, Intermediaries, and Signers.
    ///
    ///Not all recipients are are available to all account types, review you account plan to determine which recipient types are available to you. If you are working in the Demo environment, all recipient types are available.
    ///
    ///Each recipient type is described below:
    ///
    ///[Agents](#agents): This recipient can add name and email information for recipients that appear after the recipient in routing order.
    ///
    ///[Carbon Copies](#carboncopies): Use this action if the recipient should get a copy of the template, but the recipient does not need to sign, initial, date or add information to any of the documents. This type of recipient can be placed in any order in the recipient list. The recipient receives a copy of the template when the template reaches the recipient's order in the process flow and when the template is completed.
    ///
    ///[Certified Deliveries](#certifiedDeliveries): Use this action if the recipient must receive the completed documents for the template to be completed, but the recipient does not need to sign, initial, date or add information to any of the documents.
    ///
    ///[Editors](#editors): This recipient has the same management and access rights for the template as the sender and can make changes to the template as if they were using the Advanced Correct feature. This recipient can add name and email information, add or change the routing order and set authentication options for the remaining recipients. Additionally, this recipient can edit signature/initial tabs and data fields for the remaining recipients. The recipient must have a DocuSign account to be an editor.
    ///
    ///[In Person Signers](#inPerson): Use this action if the signer is in the same physical location as a DocuSign user who will act as a Signing Host for the transaction. The recipient added is the Signing Host and new separate Signer Name field appears after Sign in person is selected.
    ///
    ///[Intermediaries](#intermediaries): This recipient can, but is not required to, add name and email information for recipients at the same or subsequent level in the routing order (until subsequent Agents, Editors or Intermediaries recipient types are added).
    ///
    ///[Signers](#signers): Use this action if your recipient must sign, initial, date or add data to form fields on the documents in the template.
    ///
    ///### Core JSON Layout
    ///
    ///The following shows the core JSON layout for a recipient.
    ///
    ///```
    /// "email": "email.name@company.com",
    ///  "name": "recipient name",
    ///  "accessCode": "",
    ///  "addAccessCodeToEmail": false,
    ///  "clientUserIs": null,
    ///  "embeddedRecipientStartURL": "string",
    ///  "customFields": {
    ///    "sample string 1",
    ///    "sample string 2"
    ///  },
    ///  "emailNotification"{
    ///    "emailBody":"email text",
    ///    "emailSubject":"Subject text",
    ///    "supportedLanguage":"en",
    ///  },
    ///  "excludedDocuments": ["2", "4"],
    ///  "idCheckConfigurationName": null,
    ///  "idCheckInformationInput": {
    ///    "addressInformationInput": {
    ///      "addressInformation": {
    ///        "street1": "sample string 1",
    ///        "street2": "sample string 2",
    ///        "city": "sample string 3",
    ///        "state": "sample string 4",
    ///        "zip": "sample string 5",
    ///        "zipPlus4": "sample string 6"
    ///      },
    ///      "displayLevelCode": "sample string 1",
    ///      "receiveInResponse": "sample string 2"
    ///    },
    ///    "dobInformationInput": {
    ///      "dateOfBirth": "sample string 1",
    ///      "displayLevelCode": "sample string 2",
    ///      "receiveInResponse": "sample string 3"
    ///    },
    ///    "ssn4InformationInput": {
    ///      "ssn4": "sample string 1",
    ///      "displayLevelCode": "sample string 2",
    ///      "receiveInResponse": "sample string 3"
    ///    },
    ///    "ssn9InformationInput": {
    ///      "ssn9": "sample string 1",
    ///      "displayLevelCode": "sample string 2"
    ///    }
    ///  },
    ///  "inheritEmailNotificationConfiguration": false,
    ///  "note": "",
    ///  "phoneAuthentication": {
    ///    "recipMayProvideNumber": "sample string 1",
    ///    "validateRecipProvidedNumber": "sample string 2",
    ///    "recordVoicePrint": "sample string 3",
    ///    "senderProvidedNumbers": [
    ///      "sample string 1",
    ///      "sample string 2"
    ///    ]
    ///  },
    ///  "recipientAttachment": null,
    ///  "recipientCaptiveInfo": null,
    ///  "recipientId": "1",
    ///  "requireIdLookup": false,
    ///  "roleName": "",
    ///  "routingOrder": 1,
    ///  "samlAuthentication": {
    ///    "samlAssertionAttributes": [
    ///      {
    ///        "name": "string",
    ///        "value": "string"
    ///      },
    ///      {
    ///        "name": "string",
    ///        "value": "string"
    ///      }
    ///    ]
    ///  },
    ///  "smsAuthentication": {
    ///    "senderProvidedNumbers":[
    ///      "sample string 1",
    ///      "sample string 2"
    ///    ]
    ///  },
    ///  "socialAuthentications": null,
    ///  "templateAccessCodeRequired": false,
    ///  "templateLocked": false,
    ///  "templateRequired": false,
    /// ...
    /// ```
    ///
    ///### Core Recipient Parameters
    ///
    ///The following table contains the descriptions for the core properties for a recipient.
    ///
    ///| Name | Required? | Schema Type | Description |
    ///| --- | --- | --- | --- |
    ///| email | Yes | Email | Email of the recipient. Notification will be sent to this email id.<br/>Maximum Length: 100 characters. |
    ///| name | Yes | String | Full legal name of the recipient.<br/>Maximum Length: 100 characters. |
    ///| accessCode | No | String | This optional element specifies the access code a recipient has to enter to validate the identity.<br/>Maximum Length: 50 characters. |
    ///| addAccessCodeToEmail | No | Boolean | This optional attribute indicates that the access code is added to the email sent to the recipient; this nullifies the Security measure of Access Code on the recipient. |
    ///| clientUserId | No | String | This specifies whether the recipient is embedded or remote.<br/><br/>If the `clientUserId` property is not null then the recipient is embedded. Note that if the `ClientUserId` property is set and either `SignerMustHaveAccount` or `SignerMustLoginToSign` property of the account settings is set to  **true**, an error is generated on sending. |
    ///| embeddedRecipientStartURL | No | String | This is a sender provided valid URL string for redirecting an embedded recipient. When using this option, the embedded recipient still receives an email from DocuSign, just as a remote recipient would, but when the document link in the email is clicked the recipient is redirected, through DocuSign, to this URL to complete their actions. When routing to the URL, it is up to the sender's system (the server responding to the URL) to then request a recipient token to launch a signing session.<br/><br/>If the value `SIGN_AT_DOCUSIGN` is used for this node, the recipient is directed to an embedded signing or viewing process directly at DocuSign. The signing or viewing action is initiated by the DocuSign system and the transaction activity and Certificate of Completion records will reflect this. In all other ways the process is identical to an embedded signing or viewing operation that would be launched by any partner.<br/><br/>It is important to remember that in a typical embedded workflow the authentication of an embedded recipient is the responsibility of the sending application and DocuSign expects that senders will follow their own process for establishing the recipient's identity. In this workflow the recipient goes through the sending application before the embedded signing or viewing process in initiated. However, when the sending application sets  the `EmbeddedRecipientStartURL` property to `SIGN_AT_DOCUSIGN`, the recipient goes directly to the embedded signing or viewing process bypassing the sending application and any authentication steps the sending application would use. In this case, DocuSign recommends that one of the normal DocuSign authentication features (Access Code, Phone Authentication, SMS Authentication, etc.) be used to verify the identity of the recipient.<br/><br>If the `clientUserId` property is NOT set and the `embeddedRecipientStartURL` property is set, DocuSign ignores the redirect URL and launch the standard signing process for the email recipient. Information can be appended to the `embeddedRecipientStartURL` property using merge fields. The available merge fields items are: templateId, recipientId, recipientName, recipientEmail, and customFields. The customFields must be part of the recipient or template. The merge fields are enclosed in double brackets.<br/><br/>_Example_:<br/>`http://senderHost/[[mergeField1]]/ beginSigningSession? [[mergeField2]]&[[mergeField3]]` |
    ///| customFields | No | customField |An optional array of strings that allows the sender to provide custom data about the recipient. This information is returned in the template status but otherwise not used by DocuSign. String `customField` properties have a maximum length of 100 characters. |
    ///| emailNotification | No | emailNotification | An optional complex type that has information for setting the language for the recipient's email information. It is composed of three elements:<br/><br/>*emailBody*: a string with the email message sent to the recipient.<br/>Maximum Length: 10000 characters.<br/><br/>*emailSubject*: a string with the subject of the email sent to the recipient.<br/>Maximum Length: 100 characters.<br/><br/>*supportedLanguage*: The simple type enumeration of the language used. The supported languages, with the language value shown in parenthesis, are: Arabic (ar), Bahasa Indonesia (id), Bahasa Melayu (ms) Bulgarian (bg), Czech (cs), Chinese Simplified (zh_CN), Chinese Traditional (zh_TW), Croatian (hr), Danish (da), Dutch (nl), English US (en), English UK (en_GB), Estonian (et), Farsi (fa), Finnish (fi), French (fr), French Canada (fr_CA), German (de), Greek (el), Hebrew (he), Hindi (hi), Hungarian (hu), Italian (it), Japanese (ja), Korean (ko), Latvian (lv), Lithuanian (lt), Norwegian (no), Polish (pl), Portuguese (pt), Portuguese Brazil (pt_BR), Romanian (ro),Russian (ru), Serbian (sr), Slovak (sk), Slovenian (sl), Spanish (es),Spanish Latin America (es_MX), Swedish (sv), Thai (th), Turkish (tr), Ukrainian (uk) and Vietnamese (vi).<br/><br/>**IMPORTANT**: If this is enabled for one recipient, it overrides the Template Subject and `EmailBlurb` property settings. Also, you must set the  `emailNotification` property for all recipients. |
    ///| excludedDocuments | No | Array of Strings | Specifies the documents that are not visible to this recipient. Document Visibility must be enabled for the account and the enforceSignerVisibility property must be set to true for the template to use this.<br/><br/>When the enforceSignerVisibility property is set to **true**, documents with tabs can only be viewed by signers that have a tab on that document. Recipients that have an administrative role (Agent, Editor, or Intermediaries) or informational role (Certified Deliveries or Carbon Copies) can always see all the documents in an template, unless they are specifically excluded using this setting when an template is sent. Documents that do not have tabs are always visible to all recipients, unless they are specifically excluded using this setting when an template is sent. |
    ///| idCheckConfigurationName | No | String |Specifies authentication check by name. The names used here must be the same as the authentication type names used by the account (these name can also be found in the web console sending interface in the Identify list for a recipient). This overrides any default authentication setting.<br/><br/>_Example_:<br/> Your account has ID Check and SMS Authentication available and in the web console Identify list these appear as 'ID Check $' and 'SMS Auth $'. To use ID check in an template, the `idCheckConfigurationName` property must be  set to `ID Check $`. To use SMS, it must be set to `SMS Auth $` and you must add phone number information to the `smsAuthentication` node.|
    ///| iDCheckInformationInput | No | IdCheckInformationInput | This complex element contains input information related to a recipient ID check. It can include the following information.<br/><br/>*addressInformationInput*: Used to set recipient address information and consists of:<br/><br/>*addressInformation*: consists of six elements, with street2 and zipPlus4 being optional. The elements are: street1, street2, city, state, zip, zipPlus4\. The maximum number of characters in each element are: street1/street2 = 150 characters, city = 50 characters, state = 2 characters, and zip/zipPlus4 = 20 characters.<br/><br/>displayLevelCode: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*dobInformationInput*: Used to set recipient date of birth information and consists of:<br/><br/>*dateOfBirth*: Specifies the recipient's date, month and year of birth.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*ssn4InformationInput*: Used to set the last four digits of the recipient's SSN information and consists of:<br/><br/>*ssn4*: Specifies the last four digits of the recipient's SSN.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay.<br/><br/>*receiveInResponse*: A Boolean element that specifies if the information needs to be returned in the response.<br/><br/>*ssn9InformationInput*: Used to set the recipient's SSN information. Note that the ssn9 information can never be returned in the response. The ssn9 input consists of:<br/><br/><br/><br/>*ssn9*: Specifies the recipient's SSN.<br/><br/>*displayLevelCode*: Specifies the display level for the recipient. Values are: ReadOnly, Editable, or DoNotDisplay. |
    ///| inheritEmailNotificationConfiguration | No | Boolean | Optional element. If true and the template recipient creates a DocuSign account after signing, the Manage Account Email Notification settings are used as the default settings for the recipient's account. |
    ///| note | No | String | A note that is unique to this recipient. This note is sent to the recipient via the signing email. The note displays in the signing UI near the upper left corner of the document on the signing screen.<br/>Maximum Length: 1000 characters. |
    ///| phoneAuthentication | No | RecipientPhoneAuthentication | Optional element. Contains the elements: <br/><br/> *recipMayProvideNumber*:Boolean. When set to **true** thenrecipient can use whatever phone number they choose to.<br/><br/> *senderProvidedNumbers*: ArrayOfString. A list of phone numbers the recipient can use. <br/><br/>*recordVoicePrint* - Reserved <br/><br/>*validateRecipProvidedNumber* - Reserved| |
    ///| recipientAttachment | No | Attachment | Reserved |
    ///| recipientId | No | String | Unique for the recipient. It is used by the tab element to indicate which recipient is to sign the Document. |
    ///| requireIdLookup | No | Boolean | When set to **true**, the recipient is required to use the specified ID check method (including Phone and SMS authentication) to validate their identity. |
    ///| roleName | No* | String | Optional element. Specifies the role name associated with the recipient.<br/><br/>This is required when working with template recipients. |
    ///| routingOrder | Yes | String | This element specifies the routing order of the recipient in the template. |
    ///| samlAuthentication | No | samlAssertionAttributes | Optional element, account must be set up to use SSO to use this. Contains the name/value pair information for the SAML assertion attributes:<br/><br/>*name*: The name of the SAML assertion attribute.<br/><br/>*value*: The value associated with the named SAML assertion attribute. |
    ///| smsAuthentication | No | senderProvidedNumbers | Optional element. Contains the element:<br/><br/>*senderProvidedNumbers*: Array that contains a list of phone numbers the recipient can use for SMS text authentication. |
    ///| socialAuthentications | No | Boolean | Lists the social ID type that can be used for recipient authentication. |
    ///| templateAccessCodeRequired | No | Boolean | Optional element. Used only when working with template recipients. When set to **true** and the `TemplateLocked` parameter is set to **true**, the sender must enter an access code. |
    ///| templateLocked | No | Boolean | Optional element. Used only when working with template recipients. When set to **true**, the sender cannot change any attributes of the recipient. |
    ///| templateRequired | No | Boolean | Optional element. Used only when working with template recipients. When set to **true**,  the sender may not remove the recipient. |
    ///
    ///## <a name="agents"></a> Agents Recipient
    ///
    ///This recipient can add name and email information for recipients that appear after the recipient in routing order.
    ///
    ///#### Example Agents layout
    ///
    ///```
    /// "agents": [{
    /// <core parameters>
    ///  "canEditRecipientEmails": false,
    ///  "canEditRecipientNames": false
    /// }],
    /// ```
    ///
    ///The additional parameters for Agents recipient are shown below:
    ///
    ///| Name | Required? | Schema Type | Description |
    ///| --- | --- | --- | --- |
    ///| canEditRecipientEmails | No | Boolean | Optional element. When set to **true**, the Agents Recipient associated with this Recipient can change the Recipient's pre-populated Email address. This element is only active if enabled for the account. |
    ///| canEditRecipientNames | No | Boolean | Optional element. When set to **true**, the Agents Recipient associated with this recipient can change the recipient's pre-populated name (`UserName`). This element is only active if enabled for the account. |
    ///
    ///## <a name="carbonCopies"></a>Carbon Copies Recipient
    ///
    ///Use this type if the recipient should get a copy of the template, but the recipient does not need to sign, initial, date, or add information to any of the documents. This type of recipient can be placed in any order in the recipient list. The recipient receives a copy of the template when the template reaches the recipient's order in the process flow and when the template is completed.
    ///
    ///#### Example Carbon Copies layout
    ///
    ///```
    /// "carbonCopies": [{
    /// <core parameters>
    /// ```
    ///
    ///The Carbon Copies recipient uses only the core parameters.
    ///
    ///## <a name="certifiedDeliveries"></a>Certified Deliveries Recipient
    ///
    ///Use this action if the recipient must receive the completed documents for the template to be completed, but the recipient does not need to sign, initial, date, or add information to any of the documents.
    ///
    ///#### Example Certified Deliveries layout
    ///
    ///```
    /// "certifiedDeliveries": [{
    /// <core parameters>
    /// }],
    /// ```
    ///The Certified Deliveries recipient uses only the core parameters.
    ///
    ///## <a name="editors"></a>Editors Recipient
    ///
    ///This recipient has the same management and access rights for the template as the sender and can make changes to the template as if they were using the Advanced Correct feature. This recipient can add name and email information, add or change the routing order and set authentication options for the remaining recipients. Additionally, this recipient can edit signature/initial tabs and data fields for the remaining recipients. The recipient must have a DocuSign account to be an editor.
    ///
    ///#### Example Editors layout
    ///
    ///```
    /// "editors": [{
    /// <core parameters>
    ///  "canEditRecipientEmails": false,
    ///  "canEditRecipientNames": false
    /// }],
    /// ```
    ///
    ///The additional parameters for Editors recipient are shown below:
    ///
    ///| Name | Required? | Schema Type | Description |
    ///| --- | --- | --- | --- |
    ///| canEditRecipientEmails | No | Boolean | Optional element. When set to **true**, the Editors Recipient associated with this Recipient can change the Recipient's pre-populated Email address. This element is only active if enabled for the account. |
    ///| canEditRecipientNames | No | Boolean | Optional element. When set to **true**, the Editors Recipient associated with this recipient can change the recipient's pre-populated name (`UserName`). This element is only active if enabled for the account. |
    ///
    ///## <a name="inPerson"></a>In Person Signers Recipient
    ///
    ///Use this type if the signer is in the same physical location as a DocuSign user who will act as a Signing Host for the transaction. The recipient added is the Signing Host and new separate Signer Name field appears after Sign in person is selected.
    ///
    ///#### Example In Person Signers layout
    ///
    ///```
    /// "inPersonSigners": [{
    ///  "hostEmail": "signing.host@company.com",
    ///  "hostName": "Mike Host",
    /// <core parameters>
    ///  "autoNavigation": false,
    ///  "defaultRecipient": false,
    ///  "signInEachLocation": false,
    ///  "signatureInfo": null,
    ///  "signerEmail": "signer.name@company.com",
    ///  "signerName": "MC Clap Your Hands",
    ///  "tabs": {
    ///    "approveTabs": null,
    ///    "checkboxTabs": null,
    ///    "companyTabs": null,
    ///    "dateSignedTabs": null,
    ///    "dateTabs": null,
    ///    "declineTabs": null,
    ///    "emailTabs": null,
    ///    "templateIdTabs": null,
    ///    "fullNameTabs": null,
    ///    "initialHereTabs": null,
    ///    "listTabs": null,
    ///    "noteTabs": null,
    ///    "numberTabs": null,
    ///    "radioGroupTabs": null,
    ///    "signHereTabs": [{
    ///    "signerAttachmentTabs": null,
    ///    "ssnTabs": null,
    ///    "textTabs": null,
    ///    "titleTabs": null,
    ///    "zipTabs": null
    ///  }
    /// }],
    /// ```
    ///
    ///The additional and changed parameters for In Person Signers recipient are shown below:
    ///
    ///| Name | Required? | Schema Type | Description |
    ///| --- | --- | --- | --- |
    ///| hostEmail | Yes | Email | Required element for In Person Signers recipient Type.<br/>Maximum Length: 100 characters.<br/><br/>Specifies the email for the signing host. |
    ///| hostName | Yes | String | Required element for In Person Signers recipient Type.<br/>Maximum Length: 100 characters.<br/><br/>Specifies the email for the signing host.|
    ///| autoNavigation | No | Boolean | Specifies whether auto navigation is set for the recipient.|
    ///| defaultRecipient | No | Boolean | When set to **true**, this is the default recipient for the template. This option is used with the CreateTemplateFromTemplatesAndForms method. |
    ///| signInEachLocation | No | Boolean | When set to **true** and the feature is enabled in the sender's account, the signing recipient is required to draw signatures and initials at each signature/initial tab (instead of adopting a signature/initial style or only drawing a signature/initial once). |
    ///| signatureInfo | No | String | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Allows the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. |
    ///| signerEmail | No | String | Optional element. The email address for an InPersonSigner recipient Type.<br/>Maximum Length: 100 characters. |
    ///| signerName | Yes | String | Required element with recipient type In Person Signers.<br/>Maximum Length: 100 characters.<br/><br/>The full legal name of a signer for the template. |
    ///| tabs | No | Tab | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Specifies the Tabs associated with the recipient. See the See the the [EnvelopeRecipientTabs resource](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/enveloperecipienttabs/)  for more information about tabs. for more information about tabs. |
    ///
    ///## <a name="intermediaries"></a>Intermediaries Recipient
    ///
    ///This recipient can, but is not required to, add name and email information for recipients at the same or subsequent level in the routing order (until subsequent Agents, Editors or Intermediaries recipient types are added).
    ///
    ///#### Example Intermediaries layout
    ///
    ///```
    /// "intermediaries": [{
    /// <core parameters>
    ///  "canEditRecipientEmails": false,
    ///  "canEditRecipientNames": false
    /// }],
    /// ```
    ///
    ///The parameters for Intermediaries recipient are shown below:
    ///
    ///| Name | Required? | Schema Type | Description |
    ///| --- | --- | --- | --- |
    ///| canEditRecipientEmails | No | Boolean | Optional element. When set to **true**, the Agents Recipient associated with this Recipient can change the Recipient's pre-populated Email address. This element is only active if enabled for the account. |
    ///| canEditRecipientNames | No | Boolean | Optional element. When set to **true**, the Agents Recipient associated with this recipient can change the recipient's pre-populated name (`UserName`). This element is only active if enabled for the account. |
    ///
    ///## <a name="signers"></a>Signers Recipient
    ///
    ///Use this action if your recipient must sign, initial, date, or add data to form fields on the documents in the template.
    ///
    ///#### Example Signers layout
    ///
    ///```
    /// "Signers": [{
    /// <core paramters>
    ///  "autoNavigation": false,
    ///  "defaultRecipient": false,
    ///  "signInEachLocation": false,
    ///  "signatureInfo": null,
    ///  "tabs": {
    ///    "approveTabs": null,
    ///    "checkboxTabs": null,
    ///    "companyTabs": null,
    ///    "dateSignedTabs": null,
    ///    "dateTabs": null,
    ///    "declineTabs": null,
    ///    "emailTabs": null,
    ///    "templateIdTabs": null,
    ///    "fullNameTabs": null,
    ///    "initialHereTabs": null,
    ///    "listTabs": null,
    ///    "noteTabs": null,
    ///    "numberTabs": null,
    ///    "radioGroupTabs": null,
    ///    "signHereTabs": [{
    ///    "signerAttachmentTabs": null,
    ///    "ssnTabs": null,
    ///    "textTabs": null,
    ///    "titleTabs": null,
    ///    "zipTabs": null
    ///  }
    ///  "deliveryMethod":"",
    ///  "deliveredDateTime":"String Content",
    ///  "signedDateTime":"String Content",
    ///  "offlineAttributes":{
    ///    "deviceName":"String Content",
    ///    "deviceModel":"String Content",
    ///    "gpsLatitude":"String Content",
    ///    "gpsLongitude":"String Content",
    ///    "accountEsignId":"String Content"
    ///  }
    /// }],
    /// ```
    ///
    ///The additional parameters for Signers recipient are shown below:
    ///
    ///| Name | Required? | Schema Type | Description |
    ///| --- | --- | --- | --- |
    ///| autoNavigation | No | Boolean | Specifies whether auto navigation is set for the recipient.|
    ///| defaultRecipient | No | Boolean | When set to **true**, this is the default recipient for the template. This option is used with the CreateTemplateFromTemplatesAndForms method. |
    ///| signInEachLocation | No | Boolean | When set to **true** and the feature is enabled in the sender's account, the signing recipient is required to draw signatures and initials at each signature/initial tab (instead of adopting a signature/initial style or only drawing a signature/initial once). |
    ///| signatureInfo | No | String | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Allows the sender to pre-specify the signature name, signature initials, and signature font used in the signature stamp for the recipient. |
    ///| signerEmail | No | String | Optional element. The email address for an InPersonSigner recipient Type.<br/>Maximum Length: 100 characters. |
    ///| signerName | Yes | String | Required element with recipient type In Person Signers.<br/>Maximum Length: 100 characters.<br/><br/>The full legal name of a signer for the template. |
    ///| tabs | No | Tab | Optional element only used with recipient types In Person Signers and Signers.<br/><br/>Specifies the Tabs associated with the recipient. See the See the [EnvelopeRecipientTabs resource](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/enveloperecipienttabs/)  for more information about tabs. section for more information about tabs. |
    ///| deliveryMethod | No | String | Reserved: For DocuSign use only.|
    ///| deliveredDateTime | No | DateTime | Reserved: For DocuSign use only. |
    ///| signedDateTime | No | DateTime | Reserved: For DocuSign use only. |
    ///| offlineAttributes | No | &nbsp; | Reserved: For DocuSign use only.|
    ///.
    pub fn template_recipients(&self) -> template_recipients::TemplateRecipients {
        template_recipients::TemplateRecipients::new(self.clone())
    }

    /// The TemplateBulkRecipients resource provide methods that allow you manage the bulk recipient file for an template.
    ///
    ///The bulk recipient CSV (Comma Separated Value) file contains the list of recipient names and email addresses that you can add to an template to send the same document to a large number of recipients.
    ///
    ///The required and optional information that can be included the file is described in the BulkEnvelope: updateRecipients method.
    pub fn template_bulk_recipients(&self) -> template_bulk_recipients::TemplateBulkRecipients {
        template_bulk_recipients::TemplateBulkRecipients::new(self.clone())
    }

    /// The TemplateCustomFields resource provides methods that allow you manage custom fields in an template.
    ///
    ///Custom fields can be used in the templates for your account to record information about the template, help search for templates and track information. The template custom fields are shown in the Template Settings section when a user is creating an template in the DocuSign member console. The template custom fields are not seen by the template recipients.
    ///
    ///There are two types of custom fields:
    ///
    ///- `text`: Enables the sender to enter the value for the field.
    ///- `list`: Enables the sender to select the value of the field from a predetermined list.
    pub fn template_custom_fields(&self) -> template_custom_fields::TemplateCustomFields {
        template_custom_fields::TemplateCustomFields::new(self.clone())
    }

    /// The TemplateDocumentFields resource provides methods that allow you to manage custom fields on a document.
    ///
    ///You can create custom versions of standard fields that combine of field properties, such as font type or size, or a validation setting.
    ///
    ///Note: Some advanced features and options are supported only in certain DocuSign plans. Your account plan might not support some options discussed in this help topic. For more information about which options are available for your account, check your account plan or contact your Account Manager.
    pub fn template_document_fields(&self) -> template_document_fields::TemplateDocumentFields {
        template_document_fields::TemplateDocumentFields::new(self.clone())
    }

    /// <!-- resources aren't rendered the same way
    ///     as other pages. This is a little hack to
    ///     make the headings work better -->
    ///<style>
    ///h1, h2, h3 {
    ///  margin-top: 1em;
    ///}
    ///</style>
    ///
    ///The TemplateDocuments resource provides methods
    ///that manage documents in a template.
    ///You can:
    ///* Add one or more documents to the template
    ///* Retrieve one or more documents from the template
    ///* Delete documents from the template
    ///
    ///All of the methods associated with this resource
    ///operate on an existing template.
    ///Before you can add documents
    ///to a template,
    ///you must first create it
    ///with the [Templates:: Create][templatescreate] method.
    ///
    ///[templatescreate]: https://developers.docusign.com/docs/esign-rest-api/reference/Templates/Templates/create/
    ///.
    pub fn template_documents(&self) -> template_documents::TemplateDocuments {
        template_documents::TemplateDocuments::new(self.clone())
    }

    /// The TemplateLocks resource provides methods that allow you to
    ///manage locks on a template.
    ///
    ///To prevent users from changing a template while another user is
    ///modifying it, you can lock the template and set the time until
    ///the lock expires.
    ///
    ///For example, you would use the following flow:
    ///
    ///1. Lock the template.
    ///2. Make changes to template.
    ///3. Delete the template lock and save the changes. If the template has a password, you must supply the template password to save the changes.
    ///
    ///**Note**: To use template locks, the user must have envelope locking capability enabled.
    pub fn template_locks(&self) -> template_locks::TemplateLocks {
        template_locks::TemplateLocks::new(self.clone())
    }

    /// The Template resource provides methods that allow you to manipulate and monitor templates.
    ///
    ///Once you have authenticated the user you can use the Templates: createTemplate method to create a template. You can call the TemplateDocuments: update or TemplateDocuments: updateList method to add more documents.
    ///
    ///If you have a template that you have previously saved, you can modify the subject and message, send it, void it, or purge it from the DocuSign system using the Template: update method.
    ///
    ///In addition to receiving notifications, you can monitor the status of the templates using the following methods:
    ///
    ///* Template: getTemplate - Gets the status of a template.
    ///* Template: listStatus - Gets the status for a set of templates.
    ///* Template: listStatusChanges - Gets status change information for one or more templates.
    ///
    ///If you need to delete a page from a document in a template, use the Template: deleteDocumentPage method.
    ///
    ///The resource also includes a number of methods that allow you to retrieve and set the initials and signature for certain types of recipients on the document.
    pub fn templates(&self) -> templates::Templates {
        templates::Templates::new(self.clone())
    }

    /// <!-- resources aren't rendered the same way
    ///     as other pages. This is a little hack to
    ///     make the headings work better -->
    ///<style>
    ///h1, h2, h3 {
    ///  margin-top: 1em;
    ///}
    ///</style>
    ///
    ///The TemplateRecipientTabs resource provides methods that let you
    ///add,
    ///update,
    ///and delete tabs
    ///from an envelope.
    ///Tabs are associated with a specific recipient
    ///in an envelope
    ///and are only used by the recipient types
    ///In Person Signers and Signers.
    ///
    ///
    ///## Tab Types
    ///
    ///This table lists the available tab types.
    ///
    ///<br>
    ///
    ///
    ///| Tab Type              | Description |
    ///| :-------              | :---------- |
    ///| Approve Tab           | Place this tab on the document if you want the recipient to approve documents in an envelope without placing a signature or initials on the document. If the recipient clicks the Approve tab during the signing process, the recipient is considered to have signed the document. No information is shown on the document for the approval, but it is recorded as a signature in the envelope history. |
    ///| Checkbox Tab          | Place this tab on the document in a location where the recipient can select a yes/no (on/off) type option. |
    ///| Company Tab           | Place this tab on the document where you want the recipient's company name to appear. |
    ///| Date Signed Tab       | Place this tab on the document where you want the date the recipient signed the document to appear. |
    ///| Date Tab              | Place this tab on the document where you want the recipient to enter a date. Date tabs are single-line fields that allow date information to be entered in any format. The tooltip for this tab recommends entering the date as MM/DD/YYYY, but this is not enforced. The format entered by the signer is retained. If you need a particular date format enforced, DocuSign recommends using a Text tab with a Validation Pattern and Validation Message to enforce the format. |
    ///| Decline Tab           | Place this tab on the document where you want to give the recipient the option of declining an envelope. If the recipient clicks the Decline tab during the signing process, the envelope is voided. |
    ///| Email Address Tab     | Place this tab on a document where you want the recipient's email, as entered in the recipient information, to appear. |
    ///| Email Tab             | This is a single-line field that accepts all characters. |
    ///| Envelope ID Tab       | Place this tab on the document where you want the envelope ID for to appear. Recipients cannot enter or change the information in this tab. It is for informational purposes only. |
    ///| First Name Tab        | Place this tab on a document where you want the recipient's first name to appear. This tab splits the recipient's name (as entered in the recipient information) into sections based on spaces and uses the first section as the first name. |
    ///| Formula Tab           | This tab is used to add a calculated field to a document. Envelope recipients cannot directly enter information into the tab. The formula tab calculates and displays a new value when changes are made to the reference tab values. The reference tab information and calculation operations are entered in the "formula" element. See the Using the Calculated Fields Feature quick start guide or DocuSign Service User Guide for more information about formulas.  |
    ///| Full Name Tab         | Place this tab on the document where you want the recipient's full name to appear. |
    ///| Initial Here Tab      | Place this tab where the recipient must initial the document. This tab can be set to be optional.  |
    ///| Last Name Tab         | Place this tab on a document where you want the recipient's last name to appear. This tab splits the recipient's name (as entered in the recipient information) into sections based on spaces and uses the last section as the last name. |
    ///| List Tab              | This tab has a list of options that a recipient can select. The `listItems` parameter is used to set the options that can be selected.  |
    ///| Note Tab              | Place this tab on the document where you want to add a note for the recipient on a document. |
    ///| Number Tab            | This tab is a field where the recipient can enter numbers and decimal (.) points.  |
    ///| Radio Group Tab       | This group tab is used to place radio buttons on a document. The `radios` parameter is used to add and place the radio buttons associated with the group. Only one radio button can be selected in a group.  |
    ///| Sign Here Tab         | Place this tab where the recipient must sign the document. This tab can be set to be optional.  |
    ///| Signer Attachment Tab | The signer attachment is where the recipient initiates the process of adding supporting documents to an envelope. |
    ///| SSN Tab               | This tab is a single-line field where the recipient can enter numbers. A Social Security Number can be typed with or without dashes. |
    ///| Text Tab              | This tab is a field where the recipient can enter any type of characters.  |
    ///| Title Tab             | Place this tab on the document where you want the recipient's title to appear. |
    ///| Zip Tab               | This tab is a single-line field where the recipient can enter numbers. |
    ///
    ///
    ///## Using Custom Tabs in Envelopes and Templates
    ///
    ///Custom Tabs can be added to envelopes and templates
    ///by setting the `customTabId` property
    ///when creating an envelope or template recipient
    ///or when adding a new tab for an existing recipient.
    ///The custom tab must be added as the correct tab type.
    ///For example if the custom tab type is text, it cannot be used as a number tab.
    ///
    ///When the `customTabId` property is set,
    ///the new tab inherits all the custom tab properties.
    ///Required information that is not included in the custom tab,
    ///such as document ID and page ID, must be included when adding the tab.
    ///If the custom tab does not have anchor settings, the X and Y positions must be included.
    ///
    ///After the tab is created,
    ///it is treated as any other tab for updating or deleting.
    ///
    ///## Anchoring Tabs
    ///
    ///The tab anchoring option
    ///allows you to send documents for signature
    ///that do not have a fixed layout or format.
    ///In these documents you might not know
    ///the absolute location of the tabs
    ///when you design your API client application because the tabs must move with text.
    ///As an alternative to sending X and Y coordinates for tabs,
    ///the DocuSign Service can derive an anchor location for the tab
    ///by correlating anchor information to data within the document.
    ///
    ///When the DocuSign Service receives a request that contains tabs
    ///with anchor information,
    ///it searches the document for instances of the `anchorString` property.
    ///When found,
    ///it places a tab of the specified type for the designated recipient.
    ///Tab positions are established by setting offsets for the tab.
    ///
    ///When you apply tabs to the document,
    ///DocuSign does not remove or replace the text in the `anchorString` property. You can hide codified anchors by using the same font color as the background of the document. So the anchor can be used by DocuSign processes and it will not be visible on the document.
    ///
    ///To use an anchoring option:
    ///
    ///1. Identify the location in the document by text string. You can use a pre-existing text string or add a new one.
    ///For best performance DocuSign recommends using single word anchor strings when possible, especially when there are a large number of pages in the envelope.
    ///For example, you might want to add a Sign Here tab to the "Borrower's Signature" lines in a document, but that phrase might occur in places in the document where you don't want to tab to appear. In this case, you could add the text "BorrowerSignHere" in white font color (so that isn't visible in the document) to all the places you want Sign Here tabs to appear and use "BorrowerSignHere" as the anchor string.
    ///1. Reference the anchor through the `anchorString` property of the tab.
    ///1. Determine the offset from the anchor string location to where the tab should be placed.
    ///
    ///Setting a positive value in the `anchorXOffset` property moves the tab right on the page and positive values in the  `anchorYoffset` prove moves the tab down the page. The `anchorUnits` property specifies the units used for the offsets.
    ///For Sign Here and Initial Here tabs the bottom-left of the anchor string is equivalent to position (0,0), and the bottom-left of the tab graphic is placed relative to that.
    ///For all other tabs the bottom-left of the anchor string is equivalent to position (0,0), and the top-left of the tab graphic is placed relative to that.
    ///DocuSign does not currently provide tools to derive the offset values. Determination of the proper offset will likely require some trial-and-error.
    ///
    ///### Rules for working with anchor tags
    ///
    ///When anchor tabs are used, all documents in the envelope are searched for the `anchorString` property.
    ///
    ///* You set the text of the anchor string in the `anchorString` property. DocuSign tabs are created for each instance of the `anchorString` property within the document, so special care must be taken to establish unique anchor strings that do not result in unintentional tabs.
    ///* You cannot use the same anchored tab for different recipients for the same document.
    ///* The DocuSign system cannot search for text that is embedded in an image when checking for anchor strings.
    ///* X or Y offsets supplied for a tab apply to all instances of the tab in the document. To use different offsets at different locations in the document for the same recipient, create multiple, unique anchor tabs.
    ///* If the Y offset value of an anchor string would force a tab outside of the page boundaries, the tag is placed at the page boundary. If the X offset value places a tab outside of the page boundaries, the error message `Invalid_User_Offset` is sent. The error message includes the X offset that resulted in the error.
    ///* The system does not support an anchor string embedded in the form of a PDF X-object in the document.
    ///* The system does not re-flow the text that surrounds the anchor tabs. It is the responsibility of the document author to provide sufficient white space to contain the potential width of the ultimate tab value.
    ///
    ///### Tips and Tricks
    ///
    ///The following are tips for effective use of anchor tags:
    ///* In order to avoid unintentional conflicts between text contained in an `anchorString` property and the text that naturally exists in documents, establish a codified syntax for the anchor string text that is unlikely to appear elsewhere in a document.
    ///* Develop an extensible and consistent syntax that can be used across multiple document types.
    ///* Especially for documents that have variable numbers of tabs or signers, author the source document to include hidden anchor tabs for all potential signers/permutations. Then, control the tabs that are actually placed by including/excluding the anchor tabs in the request. This approach allows a single document to be used for all use cases instead of maintaining separate documents for each scenario.
    ///
    ///## Automatically Populating Tabs
    ///
    ///If you want similar tab types
    ///to automatically populate with the same data,
    ///you must follow these guidelines:
    ///
    ///* Each `tabLabel` entry must have the characters
    ///  `\\*` in front of the label.
    ///  If you omit the `\\*` prefix,
    ///  only the first occurrence of the tab is populated.
    ///
    ///  When automatically populating tabs,
    ///  the `tabLabel` must not contain any spaces.
    ///  In the JSON example below,
    ///  the Text tabs  properties `StudentLastName` and `StudentFirstName`
    ///  will be auto-populated as specified ("Doe" and "John")
    ///  each place they appear throughout the envelope.
    ///
    ///  ```
    ///  "tabs": {
    ///    "textTabs": [
    ///    {
    ///      "tabLabel": "\\*StudentLastName",
    ///      "value": "Doe"
    ///    },
    ///    {
    ///      "tabLabel": "\\*StudentFirstName",
    ///      "value": "John"
    ///    }]
    ///  }
    ///  ```
    ///
    ///* Each occurrence of the tab must have identical properties.
    ///
    ///  For example, suppose there are two Text tabs in a document,
    ///  each with `tabLabel` set to `Name`.
    ///  If one tab has the `bold` property set to **true**,
    ///  and the other has the `bold` property set to **false**,
    ///  only the first one will be populated.
    ///  In order to automatically populate both occurrences
    ///  of the `Name` Text tabs,
    ///  the `bold` property must be set to the same value for both tabs.
    ///.
    pub fn template_recipient_tabs(&self) -> template_recipient_tabs::TemplateRecipientTabs {
        template_recipient_tabs::TemplateRecipientTabs::new(self.clone())
    }

    /// The TemplateViews resource
    ///provides a method that returns a URL
    ///that you can embed in your application
    ///to generate a template view that uses the DocuSign UI.
    ///
    ///One template view is available:
    ///
    ///* Edit View: The DocuSign UI for editing a template.
    ///
    ///This resource is related to the [EnvelopeViews][envelopeviews] resource.
    ///Both resources enable you to embed the DocuSign UI into your application.
    ///
    ///
    ///[envelopeviews]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeViews/
    ///.
    pub fn template_views(&self) -> template_views::TemplateViews {
        template_views::TemplateViews::new(self.clone())
    }

    /// The UserCustomSettings resource provides methods that allow you to manage the custom settings for a user.
    ///
    ///Custom settings are a flexible way to store and retrieve custom user information that can be used in your own system.
    ///
    ///There is a limit on the size for all the custom user settings for a single user. All the custom user settings for a single user is limited to 4,000 characters, which includes the xml and json structure for the settings.
    pub fn user_custom_settings(&self) -> user_custom_settings::UserCustomSettings {
        user_custom_settings::UserCustomSettings::new(self.clone())
    }

    /// The UserProfiles resource provides methods that allow you to manage a user's profile.
    pub fn user_profiles(&self) -> user_profiles::UserProfiles {
        user_profiles::UserProfiles::new(self.clone())
    }

    /// The Users resource provides methods that enable you to manage users for an account.
    ///
    ///The following User methods do not use the `title` property in the Users object:
    ///
    ///- create
    ///- delete
    ///- deleteProfileImage
    ///- get
    ///- getProfileImage
    ///- getSettings
    ///- list
    ///- update
    ///- updateList
    ///- updateProfileImag
    ///- updateSettings
    ///
    ///Instead, you can retrieve and set the user's job title by using the UserProfiles:Get and UserProfiles:Update methods. For more information, see [UserProfiles](https://developers.docusign.com/docs/esign-rest-api/reference/Users/UserProfiles).
    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }

    /// The UserSignatures resource provides methods that allow you manage the  intials and signature images for a user.
    pub fn user_signatures(&self) -> user_signatures::UserSignatures {
        user_signatures::UserSignatures::new(self.clone())
    }

    /// DocuSign eSignature includes a contacts list (also referred to as an address book) to help make sending envelopes even easier. When you send an envelope, the recipients' names and email addresses are automatically added to your contacts list. You can use the contacts list to quickly add recipients to the envelopes you send. The `Contacts` resource provides methods that enable you to manage your contacts.
    ///
    ///.
    pub fn contacts(&self) -> contacts::Contacts {
        contacts::Contacts::new(self.clone())
    }

    /// The EnvelopeAttachments resource provides methods that allow you to manage attachments.
    ///.
    pub fn envelope_attachments(&self) -> envelope_attachments::EnvelopeAttachments {
        envelope_attachments::EnvelopeAttachments::new(self.clone())
    }

    /// An account permission profile is assigned to a group of users, enabling you to set permissions for all of the users in the group at the same time. You are not required to set Permission Profiles for a group, but it makes it easier to manage user permissions for a large number of users.
    ///
    ///DocuSign offers the following account permission profiles:
    ///
    ///- DocuSign Viewer
    ///- DocuSign Sender
    ///- Account Administrator
    ///
    ///You can also create your own custom account permission profiles.
    ///.
    pub fn account_permission_profiles(
        &self,
    ) -> account_permission_profiles::AccountPermissionProfiles {
        account_permission_profiles::AccountPermissionProfiles::new(self.clone())
    }

    /// PowerForms enable you to create self-service documents for signature. A PowerForm is an envelope initiated from a URL that you make available for signers to complete. You can either add a PowerForm to your website or email the URL to recipients. DocuSign saves the data that recipients enter so  you can easily integrate it with your other applications.
    ///
    ///For more information, see [Using PowerForms](https://support.docusign.com/en/guides/ndse-user-guide-using-powerforms).
    ///
    ///**Note**: PowerForms are available only for DocuSign Enterprise accounts.
    ///
    ///
    ///### Errors
    ///
    ///PowerForm methods return the following 404 errors:
    ///
    ///- `PowerForms_Recipient_Denied_Documents`: The recipient is denied access to the documents.
    ///- `PowerForms_DigitalCerts_Shared_Tabs_Not_Allowed`: Shared tags are not allowed because a digital certificate is required
    ///for a signer.
    ///- `PowerForms_DigitalCerts_Free_Form_Tabs_Not_Allowed`: Signers that are required to use a digital certificate must have at
    ///least one required, non-conditional signature or initials tab.
    ///- `PowerForms_DigitalCerts_Multiple_Recipients_Routing_Order`: Signers that are required to use a digital certificate must be the
    ///only recipient in a routing order. Edit the routing order or remove the digital certificate requirement.
    ///- `PowerForms_DigitalCerts_Markup_Not_Allowed`: Document markup is not allowed because a digital certificate is
    ///required for a signer.
    ///- `PowerForms_Incomplete_Recipient`: The recipient's username, email, or role is not set.
    ///- `PowerForms_PowerFormId_Required`: A `powerFormId` is required.
    ///- `PowerForms_PowerFormId_Mismatch`: A `powerFormId` mismatch has occurred.
    ///.
    pub fn power_forms(&self) -> power_forms::PowerForms {
        power_forms::PowerForms::new(self.clone())
    }

    /// The PowerFormData resource provides a method for accessing PowerForm data.
    ///.
    pub fn power_form_data(&self) -> power_form_data::PowerFormData {
        power_form_data::PowerFormData::new(self.clone())
    }

    /// The AccountTabSettings resource provides methods that allow you to manage
    ///tab settings for an account.
    ///.
    pub fn account_tab_settings(&self) -> account_tab_settings::AccountTabSettings {
        account_tab_settings::AccountTabSettings::new(self.clone())
    }

    /// The ENoteConfigurations resource provides methods that allow you to manage
    ///information for the eNote eOriginal integration.
    ///.
    pub fn e_note_configurations(&self) -> e_note_configurations::ENoteConfigurations {
        e_note_configurations::ENoteConfigurations::new(self.clone())
    }

    /// The WorkspaceItems resource provides methods that enable you to manage
    ///workspace items.
    ///.
    pub fn workspace_items(&self) -> workspace_items::WorkspaceItems {
        workspace_items::WorkspaceItems::new(self.clone())
    }

    /// A workspace is a collaborative space for sharing documents and managing workflows. A workspace has a single owner who must be a DocuSign user. The owner can invite others to the workspace as collaborators. Individuals who are not DocuSign users must create a DocuSign account to join a workspace as a collaborator.
    ///
    ///You can create an envelope directly from a workspace.
    ///
    ///Workspaces store the following information:
    ///
    ///- **Files**: Files uploaded to a workspace for storage or reuse.
    ///- **Documents**: A document is a component of a transaction, template, or workspace. When a file is added to a transaction, template, or workspace, it is copied as a document. Each document in a workspace has a single owner.
    ///- **Templates**: A set of documents that you can use to create a transaction or a workspace.
    ///- **Transactions**: A transaction is a series of workflow events related to one or more documents. These events route the documents to one or more individuals for the purposes of doing business. Each transaction has a single owner (the sender).
    ///
    ///**Note**: Documents in a template are not individually listed as files.
    pub fn workspaces(&self) -> workspaces::Workspaces {
        workspaces::Workspaces::new(self.clone())
    }

    /// A chunked upload is a temporary file that you upload in parts and stage at DocuSign, then refer to as the content for other API calls. For example, you might use it for document content when assembling an envelope or template.
    ///
    ///A chunked upload is linked to the DocuSign account member who initiated the API call. This user is the only user who is able to reference the chunked upload.
    ///
    ///A ChunkedUpload is intended to be an area for briefly staging data for use with other DocuSign API calls. The ChunkedUpload API endpoints do not provide an action to download the ChunkedUpload's content.
    ///
    ///The typical flow for using a chunked upload involves the following steps:
    ///
    ///1) Initiate the chunked upload with content representing part 0.
    ///
    ///2) Add more parts to the chunked upload until you have transmitted the entirety of the content.
    ///
    ///3) Commit the chunked upload, preparing it for use with other API calls.
    ///
    ///4) Assemble a DocuSign envelope with a document that includes a reference to the chunked upload as the content.
    ///
    ///5) Continue with envelope-related processes.
    ///
    ///**Note**: You must fully upload and use a chunked upload within 20 minutes of initializing it.
    ///
    ///After the chunked upload has been correctly referenced within another API call, it becomes unavailable for any further use and is promptly removed from the system.
    ///
    ///Chunked uploads have the following limits, which are configured per DocuSign environment, account, or integrator:
    ///
    ///- The maximum number of all of a member's unexpired, unconsumed ChunkedUploads. The default value is 10.
    ///- The maximum total size of all of a member's unexpired, unconsumed  ChunkedUploads. The default value is 1 GB.
    ///- The amount of time that a chunked upload is active after you initialize it. The default value is 20 minutes.
    ///.
    pub fn chunked_uploads(&self) -> chunked_uploads::ChunkedUploads {
        chunked_uploads::ChunkedUploads::new(self.clone())
    }

    /// The EnvelopeFormData resource provides a method for downloading the data that users have entered into a form associated with an envelope.
    pub fn envelope_form_data(&self) -> envelope_form_data::EnvelopeFormData {
        envelope_form_data::EnvelopeFormData::new(self.clone())
    }

    /// The Envelope Document Visibility resource provides methods for managing document visibility by recipient.
    ///.
    pub fn envelope_document_visibility(
        &self,
    ) -> envelope_document_visibility::EnvelopeDocumentVisibility {
        envelope_document_visibility::EnvelopeDocumentVisibility::new(self.clone())
    }

    /// The Template Document Visibility resource provides methods for managing template document visibility by recipient.
    ///.
    pub fn template_document_visibility(
        &self,
    ) -> template_document_visibility::TemplateDocumentVisibility {
        template_document_visibility::TemplateDocumentVisibility::new(self.clone())
    }

    /// The AccountPasswordRules resource provides methods that allow you to obtain and update account password rules, as well as membership and account rules.
    pub fn account_password_rules(&self) -> account_password_rules::AccountPasswordRules {
        account_password_rules::AccountPasswordRules::new(self.clone())
    }

    /// The AccountWatermarks resource provides methods that allow you to obtain, preview and update watermark information.
    pub fn account_watermarks(&self) -> account_watermarks::AccountWatermarks {
        account_watermarks::AccountWatermarks::new(self.clone())
    }

    /// .
    pub fn payment_gateway_accounts(&self) -> payment_gateway_accounts::PaymentGatewayAccounts {
        payment_gateway_accounts::PaymentGatewayAccounts::new(self.clone())
    }

    /// .
    pub fn notary_journals(&self) -> notary_journals::NotaryJournals {
        notary_journals::NotaryJournals::new(self.clone())
    }

    /// The DocuSign Identity Verification process requires a signer to submit an image of their valid government ID and wait for the image to be uploaded and verified before they can access a document.
    ///
    ///Identity Verification supports government photo IDs and European eIDs, analyzing the document security features and matching the name on the agreement against the name on the ID. After a successful verification, the signer can view the agreement and sign as usual.
    ///
    ///To use Identity Verification, the [DocuSign Identity Verification](https://www.docusign.com/products/identify) product must be enabled for your account.
    ///
    ///For more information, see [Can I see (a photo of) your ID? Digital verification of real-world IDs](https://www.docusign.com/blog/can-i-see-a-photo-of-your-id-digital-verification-of-real-world-ids/).
    pub fn identity_verifications(&self) -> identity_verifications::IdentityVerifications {
        identity_verifications::IdentityVerifications::new(self.clone())
    }

    /// .
    pub fn envelope_document_html_definitions(
        &self,
    ) -> envelope_document_html_definitions::EnvelopeDocumentHtmlDefinitions {
        envelope_document_html_definitions::EnvelopeDocumentHtmlDefinitions::new(self.clone())
    }

    /// **Note**: Responsive Signing is disabled by default. To use this functionality, an account administrator must switch the account setting `enableResponsiveSigning` to **true**.
    ///Also note that Smart Sections (creating a signable HTML document that uses collapsible sections and rotating tables) are premium features. To request them, contact your DocuSign account manager.
    pub fn document_responsive_html_preview(
        &self,
    ) -> document_responsive_html_preview::DocumentResponsiveHtmlPreview {
        document_responsive_html_preview::DocumentResponsiveHtmlPreview::new(self.clone())
    }

    /// .
    pub fn envelope_html_definitions(&self) -> envelope_html_definitions::EnvelopeHtmlDefinitions {
        envelope_html_definitions::EnvelopeHtmlDefinitions::new(self.clone())
    }

    /// **Note**: Responsive Signing is disabled by default. To use this functionality, an account administrator must switch the account setting `enableResponsiveSigning` to **true**.
    ///Also note that Smart Sections (creating a signable HTML document that uses collapsible sections and rotating tables) are premium features. To request them, contact your DocuSign account manager.
    pub fn responsive_html_preview(&self) -> responsive_html_preview::ResponsiveHtmlPreview {
        responsive_html_preview::ResponsiveHtmlPreview::new(self.clone())
    }

    /// **Note**: Responsive Signing is disabled by default. To use this functionality, an account administrator must switch the account setting `enableResponsiveSigning` to **true**.
    ///Also note that Smart Sections (creating a signable HTML document that uses collapsible sections and rotating tables) are premium features. To request them, contact your DocuSign account manager.
    pub fn template_document_responsive_html_preview(
        &self,
    ) -> template_document_responsive_html_preview::TemplateDocumentResponsiveHtmlPreview {
        template_document_responsive_html_preview::TemplateDocumentResponsiveHtmlPreview::new(
            self.clone(),
        )
    }

    /// **Note**: Responsive Signing is disabled by default. To use this functionality, an account administrator must switch the account setting `enableResponsiveSigning` to **true**.
    ///Also note that Smart Sections (creating a signable HTML document that uses collapsible sections and rotating tables) are premium features. To request them, contact your DocuSign account manager.
    pub fn template_responsive_html_preview(
        &self,
    ) -> template_responsive_html_preview::TemplateResponsiveHtmlPreview {
        template_responsive_html_preview::TemplateResponsiveHtmlPreview::new(self.clone())
    }

    /// .
    pub fn template_document_html_definitions(
        &self,
    ) -> template_document_html_definitions::TemplateDocumentHtmlDefinitions {
        template_document_html_definitions::TemplateDocumentHtmlDefinitions::new(self.clone())
    }

    /// .
    pub fn template_html_definitions(&self) -> template_html_definitions::TemplateHtmlDefinitions {
        template_html_definitions::TemplateHtmlDefinitions::new(self.clone())
    }

    /// Envelope transfer rules enable administrators to transfer envelopes and templates from user to another. For example, you might do this when an employee leaves the company. To transfer ownership of envelopes and templates, the **Transfer Custody** feature must be enabled for your account.
    ///
    ///For more information about this functionality, see [Transfer Envelopes and Templates](https://support.docusign.com/en/guides/ndse-admin-guide-transfer-envelopes-templates).
    pub fn envelope_transfer_rules(&self) -> envelope_transfer_rules::EnvelopeTransferRules {
        envelope_transfer_rules::EnvelopeTransferRules::new(self.clone())
    }

    /// You can use bulk send lists for anything that you need to send to a large number of recipients on a
    ///recurring basis, such as:
    ///
    ///- Compliance letters (privacy, security, and ethics consent forms)
    ///- New hire onboarding documents (benefits, transit, and parking information and payroll forms)
    ///- Other Human Resources documents
    ///- Event-related forms
    ///
    ///The API creates a separate copy of the envelope for each recipient that you specify. Each instance of the envelope is called a
    ///`BulkCopy`. You can use a bulk send list to send up to 1,000 copies per call.
    ///
    ///After you create a bulk send list, it persists and can be reused and edited any number of times.
    ///
    ///## Customizing Bulk Send Lists
    ///
    ///You can customize individual copies of the envelope. For example, you can customize the email notification and
    ///language and add personalized notes.
    ///
    ///For example, if one recipient prefers to access their DocuSign envelopes behind an access code, and another prefers her
    ///email in French, you can implement those customizations.
    ///
    ///## Using Bulk Send
    ///
    ///The bulk send feature uses the following flow:
    ///
    ///1. Create a draft envelope by calling the [Envelopes::createEnvelope][createEnvelope] method. Add placeholders for bulk send information to the envelope, including:
    ///    - Email address placeholders.
    ///    - Tab placeholders. Assign `tabLabels` to these placeholders that will make sense for matching the tabs to values in the bulk send list. For example, if you're sending a field trip permission slip to parents, you might create a placeholder text tab called `StudentName` that will you will then populate with the names of individual students in the bulk send list. You can use the following types of text tabs, radio group tabs, and list tabs to match bulk send recipients to an envelope.
    ///    - Any envelope custom fields. These fields must match the envelope custom fields in envelope copies in the bulk send list.
    ///2. Create a bulk send list by using the [BulkSend::createBulkSendList][create_list] method.
    ///3. (Optional) Test compatibility. Use the [BulkSend::createBulkSendTestRequest][create_test] method to test your bulk
    ///   send list for compatibility with the envelope or template that you want to send. For example, a template that has
    ///   three roles is not compatible with a bulk send list that has only two recipients. For this reason, you might want to
    ///   test compatibility first. A successful test result returns the Boolean value `true`. An unsuccessful test returns a
    ///   JSON response that contains information about the errors that occurred.
    ///4. Send your envelope to the list by using the [BulkSend::createBulkSendRequest][create_request] method. The response
    ///   returns a `batchId` that you can use for tracking and other purposes.
    ///
    ///The API creates and queues your envelopes asynchronously behind the scenes. You can get the status of the batch by
    ///using the [BulkEnvelopes_GetBulkEnvelopesBatchId][getbulkenv] method, passing in the `batchId`.
    ///
    ///To get the envelopes generated for the `batchId`, use the [Envelopes:listStatusChanges][GetEnvelopes] method, passing in
    ///a `custom_field` named `BulkBatchId` where the value is the `batchId` that was returned in step 4.
    ///
    ///Example:
    ///
    ///`custom_field=BulkBatchId={{batchId}}`
    ///
    ///**Bulk Send Requirements and Limitations**
    ///
    ///* Bulk send must be enabled for your account (in the `accountSettingsInformation` object, `enableBulkRecipient` must be set to **true**) and for the user sending the envelopes (the `allowBulkRecipients` property in `userSettings` must be set to **true**).
    ///* You can include up to 1,000 bulk recipients in each request.
    ///* When you send an envelope with bulk recipients, envelopes are added to a bulk recipient queue and sent in a metered fashion. An account can have a total of 2,000 total envelopes in the queue at a time.  If this limit is reached, an error message displays to the sender. If you receive this error, wait and resend the envelope at a later time.
    ///
    ///If you frequently run into queue limits, contact your account manager to discuss modifying the queue limits for your account.
    ///
    ///[create_list]:    https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/createBulkSendList/
    ///[create_test]:    https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/createBulkSendTestRequest/
    ///[create_request]: https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/createBulkSendRequest/
    ///[getbulkenv]:     https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/getBulkSendLists/
    ///[GetEnvelopes]:   https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/listStatusChanges/
    ///
    ///[createEnvelope]: https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/EnvelopeCustomFields/create/
    ///
    ///
    ///For more information about using bulk send, see [Bulk Sending Envelopes](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/envelopes/bulk-send/).
    ///.
    pub fn bulk_send(&self) -> bulk_send::BulkSend {
        bulk_send::BulkSend::new(self.clone())
    }

    /// You can configure automatic archiving of emails sent from all of your DocuSign accounts.
    ///
    ///For more information, see [Email Archive Configuration](https://support.docusign.com/en/guides/ndse-admin-guide-email-archive-configuration).
    ///
    ///**Note**: This feature is only available for certain account plans and must be enabled by DocuSign.
    pub fn bcc_email_archive(&self) -> bcc_email_archive::BccEmailArchive {
        bcc_email_archive::BccEmailArchive::new(self.clone())
    }

    /// .
    pub fn tabs_blob(&self) -> tabs_blob::TabsBlob {
        tabs_blob::TabsBlob::new(self.clone())
    }

    /// The comments resource provides a method that enables you to download a PDF file containing all of the comments that recipients and the sender have added to the documents in an envelope.
    ///
    ///Comments are disabled by default. To use the comments feature, an account administrator must enable comments on the account (in the `accountSettingsInformation` object, set the `enableSigningExtensionComments` property to **true**).
    ///
    ///For more information, see [Comments Settings](https://support.docusign.com/en/guides/ndse-admin-guide-comments-settings).
    pub fn comments(&self) -> comments::Comments {
        comments::Comments::new(self.clone())
    }

    /// .
    pub fn favorite_templates(&self) -> favorite_templates::FavoriteTemplates {
        favorite_templates::FavoriteTemplates::new(self.clone())
    }

    /// DocuSign eNotary makes the notarization process fully digital
    ///for senders, signers, and notaries.
    ///It enables a notary public to act as an in-person witness
    ///to electronic signing of documents.
    ///Check the [DocuSign eNotary support documentation](https://support.docusign.com/en/guides/ndse-user-guide-enotary-resources)
    ///to see which jurisdictions are supported.
    ///
    ///.
    pub fn notary(&self) -> notary::Notary {
        notary::Notary::new(self.clone())
    }

    /// Creating, updating, and deleting notary jurisdiction objects.
    pub fn notary_jurisdiction(&self) -> notary_jurisdiction::NotaryJurisdiction {
        notary_jurisdiction::NotaryJurisdiction::new(self.clone())
    }

    /// .
    pub fn envelope_workflow_definition(
        &self,
    ) -> envelope_workflow_definition::EnvelopeWorkflowDefinition {
        envelope_workflow_definition::EnvelopeWorkflowDefinition::new(self.clone())
    }

    /// .
    pub fn reports(&self) -> reports::Reports {
        reports::Reports::new(self.clone())
    }

    /// .
    pub fn account_signatures(&self) -> account_signatures::AccountSignatures {
        account_signatures::AccountSignatures::new(self.clone())
    }

    /// .
    pub fn appliance_info(&self) -> appliance_info::ApplianceInfo {
        appliance_info::ApplianceInfo::new(self.clone())
    }

    /// .
    pub fn connect_secret(&self) -> connect_secret::ConnectSecret {
        connect_secret::ConnectSecret::new(self.clone())
    }
}

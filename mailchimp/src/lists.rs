use anyhow::Result;

use crate::Client;

pub struct Lists {
    pub client: Client,
}

impl Lists {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Lists { client }
    }

    /**
    * Get lists info.
    *
    * This function performs a `GET` to the `/lists` endpoint.
    *
    * Get information about all lists in the account.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `before_date_created: &str` -- Restrict response to lists created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_date_created: &str` -- Restrict results to lists created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `before_campaign_last_sent: &str` -- Restrict results to lists created before the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_campaign_last_sent: &str` -- Restrict results to lists created after the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `email: &str` -- Restrict results to lists that include a specific subscriber's email address.
    * * `sort_field: crate::types::GetListsSortField` -- Returns files sorted by the specified field.
    * * `sort_dir: crate::types::SortDir` -- Determines the order direction for sorted results.
    * * `has_ecommerce_store: bool` -- Restrict results to lists that contain an active, connected, undeleted ecommerce store.
    * * `include_total_contacts: bool` -- Return the total_contacts field in the stats response, which contains an approximate count of all contacts in any state.
    */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        before_date_created: &str,
        since_date_created: &str,
        before_campaign_last_sent: &str,
        since_campaign_last_sent: &str,
        email: &str,
        sort_field: crate::types::GetListsSortField,
        sort_dir: crate::types::SortDir,
        has_ecommerce_store: bool,
        include_total_contacts: bool,
    ) -> Result<crate::types::SubscriberLists> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !before_campaign_last_sent.is_empty() {
            query_args.push((
                "before_campaign_last_sent".to_string(),
                before_campaign_last_sent.to_string(),
            ));
        }
        if !before_date_created.is_empty() {
            query_args.push((
                "before_date_created".to_string(),
                before_date_created.to_string(),
            ));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if has_ecommerce_store {
            query_args.push((
                "has_ecommerce_store".to_string(),
                has_ecommerce_store.to_string(),
            ));
        }
        if include_total_contacts {
            query_args.push((
                "include_total_contacts".to_string(),
                include_total_contacts.to_string(),
            ));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !since_campaign_last_sent.is_empty() {
            query_args.push((
                "since_campaign_last_sent".to_string(),
                since_campaign_last_sent.to_string(),
            ));
        }
        if !since_date_created.is_empty() {
            query_args.push((
                "since_date_created".to_string(),
                since_date_created.to_string(),
            ));
        }
        if !sort_dir.to_string().is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/lists?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Add list.
    *
    * This function performs a `POST` to the `/lists` endpoint.
    *
    * Create a new list in your Mailchimp account.
    */
    pub async fn post(&self, body: &crate::types::SubscriberList) -> Result<crate::types::Lists> {
        let url = "/lists".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get list info.
    *
    * This function performs a `GET` to the `/lists/{list_id}` endpoint.
    *
    * Get information about a specific list in your Mailchimp account. Results include list members who have signed up but haven't confirmed their subscription yet and unsubscribed or cleaned.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `list_id: &str` -- The unique ID for the list.
    * * `include_total_contacts: bool` -- Return the total_contacts field in the stats response, which contains an approximate count of all contacts in any state.
    */
    pub async fn get_lists(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        list_id: &str,
        include_total_contacts: bool,
    ) -> Result<crate::types::Lists> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if include_total_contacts {
            query_args.push((
                "include_total_contacts".to_string(),
                include_total_contacts.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Batch subscribe or unsubscribe.
    *
    * This function performs a `POST` to the `/lists/{list_id}` endpoint.
    *
    * Batch subscribe or unsubscribe list members.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `skip_merge_validation: bool` -- If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    * * `skip_duplicate_check: bool` -- If skip_duplicate_check is true, we will ignore duplicates sent in the request when using the batch sub/unsub on the lists endpoint. The status of the first appearance in the request will be saved. This defaults to false.
    */
    pub async fn post_lists(
        &self,
        list_id: &str,
        skip_merge_validation: bool,
        skip_duplicate_check: bool,
        body: &crate::types::MembersSubscribeUnsubscribeFromAListInBatch,
    ) -> Result<crate::types::BatchUpdateListMembers> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if skip_duplicate_check {
            query_args.push((
                "skip_duplicate_check".to_string(),
                skip_duplicate_check.to_string(),
            ));
        }
        if skip_merge_validation {
            query_args.push((
                "skip_merge_validation".to_string(),
                skip_merge_validation.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Delete list.
    *
    * This function performs a `DELETE` to the `/lists/{list_id}` endpoint.
    *
    * Delete a list from your Mailchimp account. If you delete a list, you'll lose the list history—including subscriber activity, unsubscribes, complaints, and bounces. You’ll also lose subscribers’ email addresses, unless you exported and backed up your list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn delete(&self, list_id: &str) -> Result<()> {
        let url = format!(
            "/lists/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update lists.
    *
    * This function performs a `PATCH` to the `/lists/{list_id}` endpoint.
    *
    * Update the settings for a specific list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn patch(
        &self,
        list_id: &str,
        body: &crate::types::SubscriberListData,
    ) -> Result<crate::types::Lists> {
        let url = format!(
            "/lists/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List abuse reports.
    *
    * This function performs a `GET` to the `/lists/{list_id}/abuse-reports` endpoint.
    *
    * Get all abuse reports for a specific list.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn get_abuse_report(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        list_id: &str,
    ) -> Result<crate::types::AbuseComplaints> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/abuse-reports?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get abuse report.
    *
    * This function performs a `GET` to the `/lists/{list_id}/abuse-reports/{report_id}` endpoint.
    *
    * Get details about a specific abuse report.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `list_id: &str` -- The unique ID for the list.
    * * `report_id: &str` -- The id for the abuse report.
    */
    pub async fn get_abuse_report_lists(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        list_id: &str,
        report_id: &str,
    ) -> Result<crate::types::AbuseReports> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/abuse-reports/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&report_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List recent activity.
    *
    * This function performs a `GET` to the `/lists/{list_id}/activity` endpoint.
    *
    * Get up to the previous 180 days of daily detailed aggregated activity stats for a list, not including Automation activity.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn get_activity(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        list_id: &str,
    ) -> Result<crate::types::ListActivity> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/activity?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List top email clients.
    *
    * This function performs a `GET` to the `/lists/{list_id}/clients` endpoint.
    *
    * Get a list of the top email clients based on user-agent strings.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn get_client(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        list_id: &str,
    ) -> Result<crate::types::EmailClients> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/clients?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List growth history data.
    *
    * This function performs a `GET` to the `/lists/{list_id}/growth-history` endpoint.
    *
    * Get a month-by-month summary of a specific list's growth activity.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `list_id: &str` -- The unique ID for the list.
    * * `sort_field: crate::types::GetListsGrowthHistorySortField` -- Returns files sorted by the specified field.
    * * `sort_dir: crate::types::SortDir` -- Determines the order direction for sorted results.
    */
    pub async fn get_growth_history(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        list_id: &str,
        sort_field: crate::types::GetListsGrowthHistorySortField,
        sort_dir: crate::types::SortDir,
    ) -> Result<crate::types::GrowthHistory> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !sort_dir.to_string().is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/growth-history?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get growth history by month.
    *
    * This function performs a `GET` to the `/lists/{list_id}/growth-history/{month}` endpoint.
    *
    * Get a summary of a specific list's growth activity for a specific month and year.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `list_id: &str` -- The unique ID for the list.
    * * `month: &str` -- A specific month of list growth history.
    */
    pub async fn get_growth_history_lists(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        list_id: &str,
        month: &str,
    ) -> Result<crate::types::History> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/growth-history/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&month.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List interest categories.
    *
    * This function performs a `GET` to the `/lists/{list_id}/interest-categories` endpoint.
    *
    * Get information about a list's interest categories.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `type_: &str` -- Restrict results a type of interest group.
    */
    pub async fn get_interest_categorie(
        &self,
        list_id: &str,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        type_: &str,
    ) -> Result<crate::types::InterestGroupings> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/interest-categories?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add interest category.
    *
    * This function performs a `POST` to the `/lists/{list_id}/interest-categories` endpoint.
    *
    * Create a new interest category.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn post_interest_categorie(
        &self,
        list_id: &str,
        body: &crate::types::InterestCategory,
    ) -> Result<crate::types::Categories> {
        let url = format!(
            "/lists/{}/interest-categories",
            crate::progenitor_support::encode_path(&list_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get interest category info.
    *
    * This function performs a `GET` to the `/lists/{list_id}/interest-categories/{interest_category_id}` endpoint.
    *
    * Get information about a specific interest category.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `interest_category_id: &str` -- The unique ID for the interest category.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get_interest_categorie_lists(
        &self,
        list_id: &str,
        interest_category_id: &str,
        fields: &[String],
        exclude_fields: &[String],
    ) -> Result<crate::types::Categories> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/interest-categories/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&interest_category_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete interest category.
    *
    * This function performs a `DELETE` to the `/lists/{list_id}/interest-categories/{interest_category_id}` endpoint.
    *
    * Delete a specific interest category.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `interest_category_id: &str` -- The unique ID for the interest category.
    */
    pub async fn delete_interest_categories(
        &self,
        list_id: &str,
        interest_category_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/lists/{}/interest-categories/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&interest_category_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update interest category.
    *
    * This function performs a `PATCH` to the `/lists/{list_id}/interest-categories/{interest_category_id}` endpoint.
    *
    * Update a specific interest category.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `interest_category_id: &str` -- The unique ID for the interest category.
    */
    pub async fn patch_interest_categories(
        &self,
        list_id: &str,
        interest_category_id: &str,
        body: &crate::types::InterestCategory,
    ) -> Result<crate::types::Categories> {
        let url = format!(
            "/lists/{}/interest-categories/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&interest_category_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List interests in category.
    *
    * This function performs a `GET` to the `/lists/{list_id}/interest-categories/{interest_category_id}/interests` endpoint.
    *
    * Get a list of this category's interests.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `interest_category_id: &str` -- The unique ID for the interest category.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    */
    pub async fn get_interest_categories_interest(
        &self,
        list_id: &str,
        interest_category_id: &str,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
    ) -> Result<crate::types::InterestsData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/interest-categories/{}/interests?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&interest_category_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add interest in category.
    *
    * This function performs a `POST` to the `/lists/{list_id}/interest-categories/{interest_category_id}/interests` endpoint.
    *
    * Create a new interest or 'group name' for a specific category.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `interest_category_id: &str` -- The unique ID for the interest category.
    */
    pub async fn post_interest_categories(
        &self,
        list_id: &str,
        interest_category_id: &str,
        body: &crate::types::Interest,
    ) -> Result<crate::types::InterestsInterest> {
        let url = format!(
            "/lists/{}/interest-categories/{}/interests",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&interest_category_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get interest in category.
    *
    * This function performs a `GET` to the `/lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}` endpoint.
    *
    * Get interests or 'group names' for a specific category.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `interest_category_id: &str` -- The unique ID for the interest category.
    * * `interest_id: &str` -- The specific interest or 'group name'.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get_interest_categories_interest_lists(
        &self,
        list_id: &str,
        interest_category_id: &str,
        interest_id: &str,
        fields: &[String],
        exclude_fields: &[String],
    ) -> Result<crate::types::InterestsInterest> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/interest-categories/{}/interests/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&interest_category_id.to_string()),
            crate::progenitor_support::encode_path(&interest_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete interest in category.
    *
    * This function performs a `DELETE` to the `/lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}` endpoint.
    *
    * Delete interests or group names in a specific category.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `interest_category_id: &str` -- The unique ID for the interest category.
    * * `interest_id: &str` -- The specific interest or 'group name'.
    */
    pub async fn delete_interest_categories_interests(
        &self,
        list_id: &str,
        interest_category_id: &str,
        interest_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/lists/{}/interest-categories/{}/interests/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&interest_category_id.to_string()),
            crate::progenitor_support::encode_path(&interest_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update interest in category.
    *
    * This function performs a `PATCH` to the `/lists/{list_id}/interest-categories/{interest_category_id}/interests/{interest_id}` endpoint.
    *
    * Update interests or 'group names' for a specific category.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `interest_category_id: &str` -- The unique ID for the interest category.
    * * `interest_id: &str` -- The specific interest or 'group name'.
    */
    pub async fn patch_interest_categories_interests(
        &self,
        list_id: &str,
        interest_category_id: &str,
        interest_id: &str,
        body: &crate::types::Interest,
    ) -> Result<crate::types::InterestsInterest> {
        let url = format!(
            "/lists/{}/interest-categories/{}/interests/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&interest_category_id.to_string()),
            crate::progenitor_support::encode_path(&interest_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List segments.
    *
    * This function performs a `GET` to the `/lists/{list_id}/segments` endpoint.
    *
    * Get information about all available segments for a specific list.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `list_id: &str` -- The unique ID for the list.
    * * `type_: &str` -- Limit results based on segment type.
    * * `since_created_at: &str` -- Restrict results to segments created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `before_created_at: &str` -- Restrict results to segments created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `include_cleaned: bool` -- Whether the webhook is triggered when a list subscriber is added.
    * * `include_transactional: bool` -- Whether the webhook is triggered when a list subscriber is added.
    * * `include_unsubscribed: bool` -- Whether the webhook is triggered when a list subscriber is added.
    * * `since_updated_at: &str` -- Restrict results to segments update after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `before_updated_at: &str` -- Restrict results to segments update before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    */
    pub async fn preview_segment(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        list_id: &str,
        type_: &str,
        since_created_at: &str,
        before_created_at: &str,
        include_cleaned: bool,
        include_transactional: bool,
        include_unsubscribed: bool,
        since_updated_at: &str,
        before_updated_at: &str,
    ) -> Result<crate::types::CollectionOfSegments> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !before_created_at.is_empty() {
            query_args.push((
                "before_created_at".to_string(),
                before_created_at.to_string(),
            ));
        }
        if !before_updated_at.is_empty() {
            query_args.push((
                "before_updated_at".to_string(),
                before_updated_at.to_string(),
            ));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if include_cleaned {
            query_args.push(("include_cleaned".to_string(), include_cleaned.to_string()));
        }
        if include_transactional {
            query_args.push((
                "include_transactional".to_string(),
                include_transactional.to_string(),
            ));
        }
        if include_unsubscribed {
            query_args.push((
                "include_unsubscribed".to_string(),
                include_unsubscribed.to_string(),
            ));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !since_created_at.is_empty() {
            query_args.push(("since_created_at".to_string(), since_created_at.to_string()));
        }
        if !since_updated_at.is_empty() {
            query_args.push(("since_updated_at".to_string(), since_updated_at.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/segments?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add segment.
    *
    * This function performs a `POST` to the `/lists/{list_id}/segments` endpoint.
    *
    * Create a new segment in a specific list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn post_segment(
        &self,
        list_id: &str,
        body: &crate::types::ListData,
    ) -> Result<crate::types::Segments> {
        let url = format!(
            "/lists/{}/segments",
            crate::progenitor_support::encode_path(&list_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get segment info.
    *
    * This function performs a `GET` to the `/lists/{list_id}/segments/{segment_id}` endpoint.
    *
    * Get information about a specific segment.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `list_id: &str` -- The unique ID for the list.
    * * `segment_id: &str` -- The unique id for the segment.
    * * `include_cleaned: bool` -- Whether the webhook is triggered when a list subscriber is added.
    * * `include_transactional: bool` -- Whether the webhook is triggered when a list subscriber is added.
    * * `include_unsubscribed: bool` -- Whether the webhook is triggered when a list subscriber is added.
    */
    pub async fn get_segment(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        list_id: &str,
        segment_id: &str,
        include_cleaned: bool,
        include_transactional: bool,
        include_unsubscribed: bool,
    ) -> Result<crate::types::Segments> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if include_cleaned {
            query_args.push(("include_cleaned".to_string(), include_cleaned.to_string()));
        }
        if include_transactional {
            query_args.push((
                "include_transactional".to_string(),
                include_transactional.to_string(),
            ));
        }
        if include_unsubscribed {
            query_args.push((
                "include_unsubscribed".to_string(),
                include_unsubscribed.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/segments/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&segment_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Batch add or remove members.
    *
    * This function performs a `POST` to the `/lists/{list_id}/segments/{segment_id}` endpoint.
    *
    * Batch add/remove list members to static segment
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `segment_id: &str` -- The unique id for the segment.
    */
    pub async fn post_segment_lists(
        &self,
        list_id: &str,
        segment_id: &str,
        body: &crate::types::MembersAddRemoveFromAStaticSegment,
    ) -> Result<crate::types::BatchAddRemoveListMembersFromStaticSegment> {
        let url = format!(
            "/lists/{}/segments/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&segment_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Delete segment.
    *
    * This function performs a `DELETE` to the `/lists/{list_id}/segments/{segment_id}` endpoint.
    *
    * Delete a specific segment in a list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `segment_id: &str` -- The unique id for the segment.
    */
    pub async fn delete_segments(&self, list_id: &str, segment_id: &str) -> Result<()> {
        let url = format!(
            "/lists/{}/segments/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&segment_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update segment.
    *
    * This function performs a `PATCH` to the `/lists/{list_id}/segments/{segment_id}` endpoint.
    *
    * Update a specific segment in a list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `segment_id: &str` -- The unique id for the segment.
    */
    pub async fn patch_segments(
        &self,
        list_id: &str,
        segment_id: &str,
        body: &crate::types::ListDataType,
    ) -> Result<crate::types::Segments> {
        let url = format!(
            "/lists/{}/segments/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&segment_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List members in segment.
    *
    * This function performs a `GET` to the `/lists/{list_id}/segments/{segment_id}/members` endpoint.
    *
    * Get information about members in a saved segment.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `list_id: &str` -- The unique ID for the list.
    * * `segment_id: &str` -- The unique id for the segment.
    * * `include_cleaned: bool` -- Whether the webhook is triggered when a list subscriber is added.
    * * `include_transactional: bool` -- Whether the webhook is triggered when a list subscriber is added.
    * * `include_unsubscribed: bool` -- Whether the webhook is triggered when a list subscriber is added.
    */
    pub async fn get_segments_member(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        list_id: &str,
        segment_id: &str,
        include_cleaned: bool,
        include_transactional: bool,
        include_unsubscribed: bool,
    ) -> Result<crate::types::SegmentMembers> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if include_cleaned {
            query_args.push(("include_cleaned".to_string(), include_cleaned.to_string()));
        }
        if include_transactional {
            query_args.push((
                "include_transactional".to_string(),
                include_transactional.to_string(),
            ));
        }
        if include_unsubscribed {
            query_args.push((
                "include_unsubscribed".to_string(),
                include_unsubscribed.to_string(),
            ));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/segments/{}/members?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&segment_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add member to segment.
    *
    * This function performs a `POST` to the `/lists/{list_id}/segments/{segment_id}/members` endpoint.
    *
    * Add a member to a static segment.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `segment_id: &str` -- The unique id for the segment.
    */
    pub async fn post_segments_member(
        &self,
        list_id: &str,
        segment_id: &str,
        body: &crate::types::SubscriberInAutomationQueue,
    ) -> Result<crate::types::ListMembers> {
        let url = format!(
            "/lists/{}/segments/{}/members",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&segment_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Remove list member from segment.
    *
    * This function performs a `DELETE` to the `/lists/{list_id}/segments/{segment_id}/members/{subscriber_hash}` endpoint.
    *
    * Remove a member from the specified static segment.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `segment_id: &str` -- The unique id for the segment.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn delete_segments_members(
        &self,
        list_id: &str,
        segment_id: &str,
        subscriber_hash: &str,
    ) -> Result<()> {
        let url = format!(
            "/lists/{}/segments/{}/members/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&segment_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Search for tags on a list by name.
    *
    * This function performs a `GET` to the `/lists/{list_id}/tag-search` endpoint.
    *
    * Search for tags on a list by name. If no name is provided, will return all tags on the list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `name: &str` -- The search query used to filter tags.  The search query will be compared to each tag as a prefix, so all tags that have a name starting with this field will be returned.
    */
    pub async fn search_tags_name(
        &self,
        list_id: &str,
        name: &str,
    ) -> Result<crate::types::TagSearchResults> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/tag-search?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List members info.
    *
    * This function performs a `GET` to the `/lists/{list_id}/members` endpoint.
    *
    * Get information about members in a specific Mailchimp list.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `list_id: &str` -- The unique ID for the list.
    * * `email_type: &str` -- The name of the folder.
    * * `status: crate::types::GetListsMembersStatus` -- The subscriber's status.
    * * `since_timestamp_opt: &str` -- Restrict results to subscribers who opted-in after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `before_timestamp_opt: &str` -- Restrict results to subscribers who opted-in before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_last_changed: &str` -- Restrict results to subscribers whose information changed after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `before_last_changed: &str` -- Restrict results to subscribers whose information changed before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `unique_email_id: &str` -- A unique identifier for the email address across all Mailchimp lists.
    * * `vip_only: bool` -- A filter to return only the list's VIP members. Passing `true` will restrict results to VIP list members, passing `false` will return all list members.
    * * `interest_category_id: &str` -- The unique id for the interest category.
    * * `interest_ids: &str` -- Used to filter list members by interests. Must be accompanied by interest_category_id and interest_match. The value must be a comma separated list of interest ids present for any supplied interest categories.
    * * `interest_match: crate::types::InterestMatch` -- Used to filter list members by interests. Must be accompanied by interest_category_id and interest_ids. "any" will match a member with any of the interest supplied, "all" will only match members with every interest supplied, and "none" will match members without any of the interest supplied.
    * * `sort_field: crate::types::GetListsMembersSortField` -- Returns files sorted by the specified field.
    * * `sort_dir: crate::types::SortDir` -- Determines the order direction for sorted results.
    * * `since_last_campaign: bool` -- Filter subscribers by those subscribed/unsubscribed/pending/cleaned since last email campaign send. Member status is required to use this filter.
    * * `unsubscribed_since: &str` -- Filter subscribers by those unsubscribed since a specific date. Using any status other than unsubscribed with this filter will result in an error.
    */
    pub async fn get_member(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        list_id: &str,
        email_type: &str,
        status: crate::types::GetListsMembersStatus,
        since_timestamp_opt: &str,
        before_timestamp_opt: &str,
        since_last_changed: &str,
        before_last_changed: &str,
        unique_email_id: &str,
        vip_only: bool,
        interest_category_id: &str,
        interest_ids: &str,
        interest_match: crate::types::InterestMatch,
        sort_field: crate::types::GetListsMembersSortField,
        sort_dir: crate::types::SortDir,
        since_last_campaign: bool,
        unsubscribed_since: &str,
    ) -> Result<crate::types::ListMembersDataType> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !before_last_changed.is_empty() {
            query_args.push((
                "before_last_changed".to_string(),
                before_last_changed.to_string(),
            ));
        }
        if !before_timestamp_opt.is_empty() {
            query_args.push((
                "before_timestamp_opt".to_string(),
                before_timestamp_opt.to_string(),
            ));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !email_type.is_empty() {
            query_args.push(("email_type".to_string(), email_type.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if !interest_category_id.is_empty() {
            query_args.push((
                "interest_category_id".to_string(),
                interest_category_id.to_string(),
            ));
        }
        if !interest_ids.is_empty() {
            query_args.push(("interest_ids".to_string(), interest_ids.to_string()));
        }
        if !interest_match.to_string().is_empty() {
            query_args.push(("interest_match".to_string(), interest_match.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if since_last_campaign {
            query_args.push((
                "since_last_campaign".to_string(),
                since_last_campaign.to_string(),
            ));
        }
        if !since_last_changed.is_empty() {
            query_args.push((
                "since_last_changed".to_string(),
                since_last_changed.to_string(),
            ));
        }
        if !since_timestamp_opt.is_empty() {
            query_args.push((
                "since_timestamp_opt".to_string(),
                since_timestamp_opt.to_string(),
            ));
        }
        if !sort_dir.to_string().is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !unique_email_id.is_empty() {
            query_args.push(("unique_email_id".to_string(), unique_email_id.to_string()));
        }
        if !unsubscribed_since.is_empty() {
            query_args.push((
                "unsubscribed_since".to_string(),
                unsubscribed_since.to_string(),
            ));
        }
        if vip_only {
            query_args.push(("vip_only".to_string(), vip_only.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add member to list.
    *
    * This function performs a `POST` to the `/lists/{list_id}/members` endpoint.
    *
    * Add a new member to the list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `skip_merge_validation: bool` -- If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    */
    pub async fn post_member(
        &self,
        list_id: &str,
        skip_merge_validation: bool,
        body: &crate::types::AddListMembers,
    ) -> Result<crate::types::ListMembersData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if skip_merge_validation {
            query_args.push((
                "skip_merge_validation".to_string(),
                skip_merge_validation.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get member info.
    *
    * This function performs a `GET` to the `/lists/{list_id}/members/{subscriber_hash}` endpoint.
    *
    * Get information about a specific list member, including a currently subscribed, unsubscribed, or bounced member.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts email addresses.
    */
    pub async fn get_member_lists(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        list_id: &str,
        subscriber_hash: &str,
    ) -> Result<crate::types::ListMembersData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add or update list member.
    *
    * This function performs a `PUT` to the `/lists/{list_id}/members/{subscriber_hash}` endpoint.
    *
    * Add or update a list member.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `skip_merge_validation: bool` -- If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    */
    pub async fn put_members(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        skip_merge_validation: bool,
        body: &crate::types::AddListMembersData,
    ) -> Result<crate::types::ListMembersData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if skip_merge_validation {
            query_args.push((
                "skip_merge_validation".to_string(),
                skip_merge_validation.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Archive list member.
    *
    * This function performs a `DELETE` to the `/lists/{list_id}/members/{subscriber_hash}` endpoint.
    *
    * Archive a list member. To permanently delete, use the delete-permanent action.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn delete_members(&self, list_id: &str, subscriber_hash: &str) -> Result<()> {
        let url = format!(
            "/lists/{}/members/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update list member.
    *
    * This function performs a `PATCH` to the `/lists/{list_id}/members/{subscriber_hash}` endpoint.
    *
    * Update information for a specific list member.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `skip_merge_validation: bool` -- If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    */
    pub async fn patch_members(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        skip_merge_validation: bool,
        body: &crate::types::AddListMembersDataType,
    ) -> Result<crate::types::ListMembersData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if skip_merge_validation {
            query_args.push((
                "skip_merge_validation".to_string(),
                skip_merge_validation.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * View recent activity 50.
    *
    * This function performs a `GET` to the `/lists/{list_id}/members/{subscriber_hash}/activity` endpoint.
    *
    * Get the last 50 events of a member's activity on a specific list, including opens, clicks, and unsubscribes.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `action: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get_members_activity(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        fields: &[String],
        exclude_fields: &[String],
        action: &[String],
    ) -> Result<crate::types::MemberActivityEvents> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !action.is_empty() {
            query_args.push(("action".to_string(), action.join(" ")));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}/activity?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * View recent activity.
    *
    * This function performs a `GET` to the `/lists/{list_id}/members/{subscriber_hash}/activity-feed` endpoint.
    *
    * Get a member's activity on a specific list, including opens, clicks, and unsubscribes.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `activity_filters: &[String]` -- A comma-separated list of activity filters that correspond to a set of activity types, e.g "?activity_filters=open,bounce,click".
    */
    pub async fn get_members_activity_feed(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        activity_filters: &[String],
    ) -> Result<crate::types::MemberActivityEventsData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !activity_filters.is_empty() {
            query_args.push(("activity_filters".to_string(), activity_filters.join(" ")));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}/activity-feed?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List member tags.
    *
    * This function performs a `GET` to the `/lists/{list_id}/members/{subscriber_hash}/tags` endpoint.
    *
    * Get the tags on a list member.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    */
    pub async fn get_member_tag(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
    ) -> Result<crate::types::CollectionOfTags> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}/tags?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add or remove member tags.
    *
    * This function performs a `POST` to the `/lists/{list_id}/members/{subscriber_hash}/tags` endpoint.
    *
    * Add or remove tags from a list member. If a tag that does not exist is passed in and set as 'active', a new tag will be created.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn post_member_tag(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        body: &crate::types::MemberTags,
    ) -> Result<()> {
        let url = format!(
            "/lists/{}/members/{}/tags",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List member events.
    *
    * This function performs a `GET` to the `/lists/{list_id}/members/{subscriber_hash}/events` endpoint.
    *
    * Get events for a contact.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts email addresses.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get_members_event(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        count: i64,
        offset: i64,
        fields: &[String],
        exclude_fields: &[String],
    ) -> Result<crate::types::CollectionOfEvents> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}/events?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add event.
    *
    * This function performs a `POST` to the `/lists/{list_id}/members/{subscriber_hash}/events` endpoint.
    *
    * Add an event for a list member.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts email addresses.
    */
    pub async fn post_member_event(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        body: &crate::types::EventsData,
    ) -> Result<()> {
        let url = format!(
            "/lists/{}/members/{}/events",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List member goal events.
    *
    * This function performs a `GET` to the `/lists/{list_id}/members/{subscriber_hash}/goals` endpoint.
    *
    * Get the last 50 Goal events for a member on a specific list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get_members_goal(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        fields: &[String],
        exclude_fields: &[String],
    ) -> Result<crate::types::CollectionOfMemberActivityEvents> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}/goals?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List recent member notes.
    *
    * This function performs a `GET` to the `/lists/{list_id}/members/{subscriber_hash}/notes` endpoint.
    *
    * Get recent notes for a specific list member.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `sort_field: crate::types::GetListsMembersNotesSortField` -- Returns notes sorted by the specified field.
    * * `sort_dir: crate::types::SortDir` -- Determines the order direction for sorted results.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    */
    pub async fn get_members_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        sort_field: crate::types::GetListsMembersNotesSortField,
        sort_dir: crate::types::SortDir,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
    ) -> Result<crate::types::CollectionOfNotes> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !sort_dir.to_string().is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}/notes?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add member note.
    *
    * This function performs a `POST` to the `/lists/{list_id}/members/{subscriber_hash}/notes` endpoint.
    *
    * Add a new note for a specific subscriber.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn post_members_note(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        body: &crate::types::MemberNotes,
    ) -> Result<crate::types::CollectionOfNotesMember> {
        let url = format!(
            "/lists/{}/members/{}/notes",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get member note.
    *
    * This function performs a `GET` to the `/lists/{list_id}/members/{subscriber_hash}/notes/{note_id}` endpoint.
    *
    * Get a specific note for a specific list member.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `note_id: &str` -- The name of the folder.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get_members_note_lists(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        note_id: &str,
        fields: &[String],
        exclude_fields: &[String],
    ) -> Result<crate::types::CollectionOfNotesMember> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/members/{}/notes/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            crate::progenitor_support::encode_path(&note_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete note.
    *
    * This function performs a `DELETE` to the `/lists/{list_id}/members/{subscriber_hash}/notes/{note_id}` endpoint.
    *
    * Delete a specific note for a specific list member.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `note_id: &str` -- The name of the folder.
    */
    pub async fn delete_members_notes(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        note_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/lists/{}/members/{}/notes/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            crate::progenitor_support::encode_path(&note_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update note.
    *
    * This function performs a `PATCH` to the `/lists/{list_id}/members/{subscriber_hash}/notes/{note_id}` endpoint.
    *
    * Update a specific note for a specific list member.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `note_id: &str` -- The name of the folder.
    */
    pub async fn patch_members_notes(
        &self,
        list_id: &str,
        subscriber_hash: &str,
        note_id: &str,
        body: &crate::types::MemberNotes,
    ) -> Result<crate::types::CollectionOfNotesMember> {
        let url = format!(
            "/lists/{}/members/{}/notes/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            crate::progenitor_support::encode_path(&note_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Delete list member.
    *
    * This function performs a `POST` to the `/lists/{list_id}/members/{subscriber_hash}/actions/delete-permanent` endpoint.
    *
    * Delete all personally identifiable information related to a list member, and remove them from a list. This will make it impossible to re-import the list member.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn post_members_hash_actions_delete_permanent(
        &self,
        list_id: &str,
        subscriber_hash: &str,
    ) -> Result<()> {
        let url = format!(
            "/lists/{}/members/{}/actions/delete-permanent",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * List merge fields.
    *
    * This function performs a `GET` to the `/lists/{list_id}/merge-fields` endpoint.
    *
    * Get a list of all merge fields ([audience fields](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `type_: &str` -- The name of the folder.
    * * `required: bool` -- Whether the webhook is triggered when a list subscriber is added.
    */
    pub async fn get_merge_field(
        &self,
        list_id: &str,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        type_: &str,
        required: bool,
    ) -> Result<crate::types::CollectionOfMergeFields> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if required {
            query_args.push(("required".to_string(), required.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/merge-fields?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add merge field.
    *
    * This function performs a `POST` to the `/lists/{list_id}/merge-fields` endpoint.
    *
    * Add a new merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for a specific list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn post_merge_field(
        &self,
        list_id: &str,
        body: &crate::types::MergeFieldData,
    ) -> Result<crate::types::MergeField> {
        let url = format!(
            "/lists/{}/merge-fields",
            crate::progenitor_support::encode_path(&list_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get merge field.
    *
    * This function performs a `GET` to the `/lists/{list_id}/merge-fields/{merge_id}` endpoint.
    *
    * Get information about a specific merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) in a list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `merge_id: &str` -- The id for the merge field.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    */
    pub async fn get_merge_field_lists(
        &self,
        list_id: &str,
        merge_id: &str,
        exclude_fields: &[String],
        fields: &[String],
    ) -> Result<crate::types::MergeField> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/merge-fields/{}?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&merge_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete merge field.
    *
    * This function performs a `DELETE` to the `/lists/{list_id}/merge-fields/{merge_id}` endpoint.
    *
    * Delete a specific merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) in a list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `merge_id: &str` -- The id for the merge field.
    */
    pub async fn delete_merge_fields(&self, list_id: &str, merge_id: &str) -> Result<()> {
        let url = format!(
            "/lists/{}/merge-fields/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&merge_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update merge field.
    *
    * This function performs a `PATCH` to the `/lists/{list_id}/merge-fields/{merge_id}` endpoint.
    *
    * Update a specific merge field ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) in a list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `merge_id: &str` -- The id for the merge field.
    */
    pub async fn patch_merge_fields(
        &self,
        list_id: &str,
        merge_id: &str,
        body: &crate::types::MergeFieldDataType,
    ) -> Result<crate::types::MergeField> {
        let url = format!(
            "/lists/{}/merge-fields/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&merge_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List webhooks.
    *
    * This function performs a `GET` to the `/lists/{list_id}/webhooks` endpoint.
    *
    * Get information about all webhooks for a specific list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn get_webhook(&self, list_id: &str) -> Result<crate::types::ListWebhooksData> {
        let url = format!(
            "/lists/{}/webhooks",
            crate::progenitor_support::encode_path(&list_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Add webhook.
    *
    * This function performs a `POST` to the `/lists/{list_id}/webhooks` endpoint.
    *
    * Create a new webhook for a specific list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn post_webhook(
        &self,
        list_id: &str,
        body: &crate::types::AddWebhook,
    ) -> Result<crate::types::ListWebhooks> {
        let url = format!(
            "/lists/{}/webhooks",
            crate::progenitor_support::encode_path(&list_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get webhook info.
    *
    * This function performs a `GET` to the `/lists/{list_id}/webhooks/{webhook_id}` endpoint.
    *
    * Get information about a specific webhook.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `webhook_id: &str` -- The name of the folder.
    */
    pub async fn get_webhook_lists(
        &self,
        list_id: &str,
        webhook_id: &str,
    ) -> Result<crate::types::ListWebhooks> {
        let url = format!(
            "/lists/{}/webhooks/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&webhook_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete webhook.
    *
    * This function performs a `DELETE` to the `/lists/{list_id}/webhooks/{webhook_id}` endpoint.
    *
    * Delete a specific webhook in a list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `webhook_id: &str` -- The name of the folder.
    */
    pub async fn delete_webhooks(&self, list_id: &str, webhook_id: &str) -> Result<()> {
        let url = format!(
            "/lists/{}/webhooks/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&webhook_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update webhook.
    *
    * This function performs a `PATCH` to the `/lists/{list_id}/webhooks/{webhook_id}` endpoint.
    *
    * Update the settings for an existing webhook.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    * * `webhook_id: &str` -- The name of the folder.
    */
    pub async fn patch_webhooks(
        &self,
        list_id: &str,
        webhook_id: &str,
        body: &crate::types::AddWebhook,
    ) -> Result<crate::types::ListWebhooks> {
        let url = format!(
            "/lists/{}/webhooks/{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            crate::progenitor_support::encode_path(&webhook_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List signup forms.
    *
    * This function performs a `GET` to the `/lists/{list_id}/signup-forms` endpoint.
    *
    * Get signup forms for a specific list.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn get_signup_form(&self, list_id: &str) -> Result<crate::types::ListSignupForms> {
        let url = format!(
            "/lists/{}/signup-forms",
            crate::progenitor_support::encode_path(&list_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Customize signup form.
    *
    * This function performs a `POST` to the `/lists/{list_id}/signup-forms` endpoint.
    *
    * Customize a list's default signup form.
    *
    * **Parameters:**
    *
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn post_signup_form(
        &self,
        list_id: &str,
        body: &crate::types::SignupFormData,
    ) -> Result<crate::types::SignupForm> {
        let url = format!(
            "/lists/{}/signup-forms",
            crate::progenitor_support::encode_path(&list_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List locations.
    *
    * This function performs a `GET` to the `/lists/{list_id}/locations` endpoint.
    *
    * Get the locations (countries) that the list's subscribers have been tagged to based on geocoding their IP address.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `list_id: &str` -- The unique ID for the list.
    */
    pub async fn get_location(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        list_id: &str,
    ) -> Result<crate::types::ListLocations> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/lists/{}/locations?{}",
            crate::progenitor_support::encode_path(&list_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}

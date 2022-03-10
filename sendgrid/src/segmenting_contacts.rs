use anyhow::Result;

use crate::Client;

pub struct SegmentingContacts {
    pub client: Client,
}

impl SegmentingContacts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SegmentingContacts { client }
    }

    /**
     * Get List of Segments.
     *
     * This function performs a `GET` to the `/marketing/segments` endpoint.
     *
     * **This endpoint allows you to retrieve a list of segments.**
     *
     * The query param `parent_list_ids` is treated as a filter.  Any match will be returned.  0 matches will return a response code of 200 with an empty `results` array.
     *
     * `parent_list_ids` | `no_parent_list_id` | `result`
     * -----------------:|:--------------------:|:-------------
     * empty | false | all segments
     * values | false | segments filtered by list_ids
     * values | true | segments filtered by list_ids and segments with no parent list_ids
     * empty | true | segments with no parent list_ids
     *
     * **Parameters:**
     *
     * * `parent_list_ids: &str` -- A comma separated list of list ids to be used when searching for segments with the specified parent_list_id, no more than 50 is allowed.
     * * `no_parent_list_id: bool` -- If set to `true` segments with an empty value of `parent_list_id` will be returned in the filter.  If the value is not present it defaults to 'false'.
     */
    pub async fn get_marketing_segments(
        &self,
        parent_list_ids: &str,
        no_parent_list_id: bool,
    ) -> Result<crate::types::GetMarketingSegmentsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if no_parent_list_id {
            query_args.push((
                "no_parent_list_id".to_string(),
                no_parent_list_id.to_string(),
            ));
        }
        if !parent_list_ids.is_empty() {
            query_args.push(("parent_list_ids".to_string(), parent_list_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/marketing/segments?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Create Segment.
     *
     * This function performs a `POST` to the `/marketing/segments` endpoint.
     *
     * **This endpoint allows you to create a segment.**
     */
    pub async fn post_marketing_segment(
        &self,
        body: &crate::types::PostMarketingSegmentsRequestAllOf,
    ) -> Result<crate::types::FullSegmentAllOf> {
        let url = "/marketing/segments".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get Segment by ID.
     *
     * This function performs a `GET` to the `/marketing/segments/{segment_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a single segment by ID.**
     *
     * **Parameters:**
     *
     * * `query_json: bool` -- Defaults to `false`.  Set to `true` to return the parsed SQL AST as a JSON object in the field `query_json`.
     */
    pub async fn get_marketing_segments_segment(
        &self,
        segment_id: &str,
        query_json: bool,
    ) -> Result<crate::types::FullSegmentAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if query_json {
            query_args.push(("query_json".to_string(), query_json.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/marketing/segments/{}?{}",
            crate::progenitor_support::encode_path(segment_id),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete Segment.
     *
     * This function performs a `DELETE` to the `/marketing/segments/{segment_id}` endpoint.
     *
     * **This endpoint allows you to delete a segment by `segment_id`.**
     *
     * Note that deleting a segment does not delete the contacts associated with the segment by default. Contacts associated with a deleted segment will remain in your list of all contacts and any other segments they belong to.
     */
    pub async fn delete_marketing_segments_segment(
        &self,
        segment_id: &str,
    ) -> Result<crate::types::Help> {
        let url = format!(
            "/marketing/segments/{}",
            crate::progenitor_support::encode_path(segment_id),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update Segment.
     *
     * This function performs a `PATCH` to the `/marketing/segments/{segment_id}` endpoint.
     *
     * **This endpoint allows you to update a segment.**
     *
     * Segment `name` needs to be unique. A user can not update a segment name to an existing one.
     */
    pub async fn patch_marketing_segments_segment(
        &self,
        segment_id: &str,
        body: &crate::types::SegmentWriteV2,
    ) -> Result<crate::types::FullSegmentAllOf> {
        let url = format!(
            "/marketing/segments/{}",
            crate::progenitor_support::encode_path(segment_id),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Bulk Delete Segments.
     *
     * This function performs a `POST` to the `/marketing/segments/delete` endpoint.
     *
     * This endpoint allows you to delete segments in bulk.
     *
     * If the segments are used by automations or the segments do not exist in the database, the segment IDs that could not be deleted along with automation IDs that are associated to those segments will be returned.
     */
    pub async fn post_marketing_segments_delete(
        &self,
        body: &crate::types::PostMarketingSegmentsDeleteRequest,
    ) -> Result<crate::types::PostMarketingSegmentsDeleteResponse> {
        let url = "/marketing/segments/delete".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

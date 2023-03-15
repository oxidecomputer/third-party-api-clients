use crate::Client;
use crate::ClientResult;

pub struct SegmentingContactsBeta {
    pub client: Client,
}

impl SegmentingContactsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SegmentingContactsBeta { client }
    }

    /**
     * Get List of Segments.
     *
     * This function performs a `GET` to the `/marketing/segments/2.0` endpoint.
     *
     * **The Segmentation V2 API is currently in private beta. If you'd like to be added to the beta, please fill out this [form](https://docs.google.com/forms/d/e/1FAIpQLSd5zwC9dRk8lAp1oTWjdGc-aSY69flW_7wnutvKBhpUluSnfQ/viewform)**
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
     * * `parent_list_ids: &str` -- A comma separated list up to 50 in size, to filter segments on.  Only segments that have any of these list ids as the parent list will be retrieved. This is different from the parameter of the same name used when creating a segment.
     * * `no_parent_list_id: bool` -- If set to `true` segments with an empty value of `parent_list_id` will be returned in the filter.  If the value is not present it defaults to 'false'.
     */
    pub async fn get_segments(
        &self,
        parent_list_ids: &str,
        no_parent_list_id: bool,
    ) -> ClientResult<crate::types::AllSegmentsResponse> {
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
        let url = self
            .client
            .url(&format!("/marketing/segments/2.0?{}", query_), None);
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
     * Create Segment.
     *
     * This function performs a `POST` to the `/marketing/segments/2.0` endpoint.
     *
     * **The Segmentation V2 API is currently in private beta. If you'd like to be added to the beta, please fill out this [form](https://docs.google.com/forms/d/e/1FAIpQLSd5zwC9dRk8lAp1oTWjdGc-aSY69flW_7wnutvKBhpUluSnfQ/viewform)**
     *
     * Segment `name` has to be unique. A user can not create a new segment with an existing segment name.
     */
    pub async fn post_segment(
        &self,
        body: &crate::types::SegmentWriteV2,
    ) -> ClientResult<crate::types::SegmentResponse> {
        let url = self.client.url("/marketing/segments/2.0", None);
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
     * Get Segment by ID.
     *
     * This function performs a `GET` to the `/marketing/segments/2.0/{segment_id}` endpoint.
     *
     * **The Segmentation V2 API is currently in private beta. If you'd like to be added to the beta, please fill out this [form](https://docs.google.com/forms/d/e/1FAIpQLSd5zwC9dRk8lAp1oTWjdGc-aSY69flW_7wnutvKBhpUluSnfQ/viewform)**
     *
     * **Parameters:**
     *
     * * `contacts_sample: bool` -- Defaults to `true`. Set to `false` to exclude the contacts_sample in the response.
     */
    pub async fn get_segments_segment(
        &self,
        segment_id: &str,
        contacts_sample: bool,
    ) -> ClientResult<crate::types::SegmentResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if contacts_sample {
            query_args.push(("contacts_sample".to_string(), contacts_sample.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/marketing/segments/2.0/{}?{}",
                crate::progenitor_support::encode_path(segment_id),
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
     * Delete segment.
     *
     * This function performs a `DELETE` to the `/marketing/segments/2.0/{segment_id}` endpoint.
     *
     * **The Segmentation V2 API is currently in private beta. If you'd like to be added to the beta, please fill out this [form](https://docs.google.com/forms/d/e/1FAIpQLSd5zwC9dRk8lAp1oTWjdGc-aSY69flW_7wnutvKBhpUluSnfQ/viewform)**
     */
    pub async fn delete_segments_segment(&self, segment_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/marketing/segments/2.0/{}",
                crate::progenitor_support::encode_path(segment_id),
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
     * Update Segment.
     *
     * This function performs a `PATCH` to the `/marketing/segments/2.0/{segment_id}` endpoint.
     *
     * **The Segmentation V2 API is currently in private beta. If you'd like to be added to the beta, please fill out this [form](https://docs.google.com/forms/d/e/1FAIpQLSd5zwC9dRk8lAp1oTWjdGc-aSY69flW_7wnutvKBhpUluSnfQ/viewform)**
     *
     * Segment `name` has to be unique. A user can not create a new segment with an existing segment name.
     */
    pub async fn patch_segments_segment(
        &self,
        segment_id: &str,
        body: &crate::types::SegmentUpdate,
    ) -> ClientResult<crate::types::SegmentResponse> {
        let url = self.client.url(
            &format!(
                "/marketing/segments/2.0/{}",
                crate::progenitor_support::encode_path(segment_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}

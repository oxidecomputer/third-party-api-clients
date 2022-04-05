use anyhow::Result;

use crate::Client;

pub struct Files {
    pub client: Client,
}

impl Files {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Files { client }
    }

    /**
    * This function performs a `GET` to the `/v1/files` endpoint.
    *
    * <p>Returns a list of the files that your account has access to. The files are returned sorted by creation date, with the most recently created files appearing first.</p>
    *
    * **Parameters:**
    *
    * * `created: &str`
    * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
    * * `purpose: crate::types::Purpose` -- The file purpose to filter queries by. If none is provided, files will not be filtered by purpose.
    * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    */
    pub async fn get_page(
        &self,
        created: &str,
        ending_before: &str,
        expand: &[String],
        limit: i64,
        purpose: crate::types::Purpose,
        starting_after: &str,
    ) -> Result<Vec<crate::types::File>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !purpose.to_string().is_empty() {
            query_args.push(("purpose".to_string(), purpose.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/files?{}", query_);

        let resp: crate::types::GetFilesResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.data)
    }

    /**
    * This function performs a `GET` to the `/v1/files` endpoint.
    *
    * As opposed to `get`, this function returns all the pages of the request at once.
    *
    * <p>Returns a list of the files that your account has access to. The files are returned sorted by creation date, with the most recently created files appearing first.</p>
    */
    pub async fn get_all(
        &self,
        created: &str,
        expand: &[String],
        purpose: crate::types::Purpose,
    ) -> Result<Vec<crate::types::File>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !purpose.to_string().is_empty() {
            query_args.push(("purpose".to_string(), purpose.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/files?{}", query_);

        let mut resp: crate::types::GetFilesResponse = self.client.get(&url, None).await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                page = data.last().unwrap().id.to_string();
            }
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?startng_after={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&starting_after={}", url, page), None)
                    .await?;
            }

            data.append(&mut resp.data);

            has_more = resp.has_more;
        }

        // Return our response data.
        Ok(data.to_vec())
    }

    /**
    * This function performs a `POST` to the `/v1/files` endpoint.
    *
    * <p>To upload a file to Stripe, you’ll need to send a request of type <code>multipart/form-data</code>. The request should contain the file you would like to upload, as well as the parameters for creating a file.</p>
    *
    * <p>All of Stripe’s officially supported Client libraries should have support for sending <code>multipart/form-data</code>.</p>
    */
    pub async fn post(&self) -> Result<crate::types::File> {
        let url = "/v1/files".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/v1/files/{file}` endpoint.
    *
    * <p>Retrieves the details of an existing file object. Supply the unique file ID from a file, and Stripe will return the corresponding file object. To access file contents, see the <a href="/docs/file-upload#download-file-contents">File Upload Guide</a>.</p>
    *
    * **Parameters:**
    *
    * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    * * `file: &str` -- The account's country.
    */
    pub async fn get(&self, expand: &[String], file: &str) -> Result<crate::types::File> {
        let url = format!(
            "/v1/files/{}",
            crate::progenitor_support::encode_path(&file.to_string()),
        );

        self.client.get(&url, None).await
    }
}

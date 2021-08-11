use anyhow::Result;

use crate::Client;

pub struct Department {
    client: Client,
}

impl Department {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Department {
            client,
        }
    }

    /**
* List departments.
*
* This function performs a `GET` to the `/departments` endpoint.
*
* Retrieve all departments.
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
* * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
* * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
*/
pub async fn get_departments(
&self,
authorization: &str, start: &str, page_size: f64,
) -> Result<crate::types::GetDepartmentsResponse> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
query_args.push(format!("page_size={}", page_size));
if !start.is_empty() { query_args.push(format!("start={}", start)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/departments?{}",
query);

self.client.get(&url, None).await
}

/**
* Create department.
*
* This function performs a `POST` to the `/departments` endpoint.
*
* Create a new department.
*/
pub async fn post_department(
&self,
body: &crate::types::PostLocationRequest
) -> Result<crate::types::GetResponse> {
let url =
"/departments".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* GET a department.
*
* This function performs a `GET` to the `/departments/<id>` endpoint.
*
* Retrieve a single department.
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
*/
pub async fn get(
&self,
authorization: &str,
) -> Result<crate::types::GetResponse> {
let url =
"/departments/<id>".to_string();
self.client.get(&url, None).await
}

/**
* Update department.
*
* This function performs a `PATCH` to the `/departments/<id>` endpoint.
*
* Modify a department. 
*/
pub async fn patch(
&self,
body: &crate::types::PatchRequest
) -> Result<crate::types::GetResponse> {
let url =
"/departments/<id>".to_string();
self.client.patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}
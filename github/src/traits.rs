use async_trait::async_trait;
use bytes::Bytes;

use crate::{actions::Actions, ClientResult, Response};

#[async_trait]
pub trait ActionsExt {
    async fn download_contents(
        &self,
        owner: &str,
        repo: &str,
        artifact_id: i64,
        archive_format: &str,
    ) -> ClientResult<Response<Bytes>>;
}

#[async_trait]
impl ActionsExt for Actions {
    async fn download_contents(
        &self,
        owner: &str,
        repo: &str,
        artifact_id: i64,
        archive_format: &str,
    ) -> ClientResult<Response<Bytes>> {
        let response = self
            .download_artifact(owner, repo, artifact_id, archive_format)
            .await?;

        let location = response
            .headers
            .get("Location")
            .map(|v| v.to_str().unwrap())
            .unwrap();

        let download = self
            .client
            .make_request(
                http::Method::GET,
                location,
                crate::Message::default(),
                crate::utils::MediaType::Json,
                crate::auth::AuthenticationConstraint::Unconstrained,
            )
            .await?
            .send()
            .await?;

        Ok(Response::new(
            download.status(),
            download.headers().clone(),
            download.bytes().await?,
        ))
    }
}

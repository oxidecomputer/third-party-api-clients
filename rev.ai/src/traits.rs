use anyhow::{anyhow, Result};

#[async_trait::async_trait]
pub trait JobOps {
    /// Send a plain text email.
    ///
    /// This is a nicer experience than using `post`.
    async fn post(&self, b: bytes::Bytes) -> Result<crate::types::Job>;
}

#[async_trait::async_trait]
impl JobOps for crate::jobs::Jobs {
    /// Create a job.
    async fn post(&self, b: bytes::Bytes) -> Result<crate::types::Job> {
        let form = requwest::multipart::Form::new()
            .part(
                "media",
                reqwest::multipart::Part::bytes(bytes.to_vec())
                    .mime_str("video/mp4")
                    .unwrap()
                    .file_name("testing.mp4"),
            )
            .text("options", "{}");

        self.client.post_form("jobs", form).await
    }
}

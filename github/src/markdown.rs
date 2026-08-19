use crate::Client;
use crate::ClientResult;

pub struct Markdown {
    pub client: Client,
}

impl Markdown {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Markdown { client }
    }

    /**
     * Render a Markdown document.
     *
     * This function performs a `POST` to the `/markdown` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/markdown#render-a-markdown-document>
     */
    pub async fn render(
        &self,
        body: &crate::types::MarkdownRenderRequest,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url("/markdown", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Render a Markdown document in raw mode.
     *
     * This function performs a `POST` to the `/markdown/raw` endpoint.
     *
     * You must send Markdown as plain text (using a `Content-Type` header of `text/plain` or `text/x-markdown`) to this endpoint, rather than using JSON format. In raw mode, [GitHub Flavored Markdown](https://github.github.com/gfm/) is not supported and Markdown will be rendered in plain format like a README.md file. Markdown content must be 400 KB or less.
     *
     * FROM: <https://docs.github.com/rest/reference/markdown#render-a-markdown-document-in-raw-mode>
     */
    pub async fn render_raw<T: Into<reqwest::Body>>(
        &self,
        body: T,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url("/markdown/raw", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(body.into()),
                    content_type: Some("text/plain".to_string()),
                },
            )
            .await
    }
}

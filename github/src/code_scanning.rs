use anyhow::Result;

use crate::Client;

pub struct CodeScanning {
    client: Client,
}

impl CodeScanning {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CodeScanning { client }
    }

    /**
     * Get information about a SARIF upload.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/sarifs/{sarif_id}` endpoint.
     *
     * Gets information about a SARIF upload, including the status and the URL of the analysis that was uploaded so that you can retrieve details of the analysis. For more information, see "[Get a code scanning analysis for a repository](/rest/reference/code-scanning#get-a-code-scanning-analysis-for-a-repository)." You must use an access token with the `security_events` scope to use this endpoint. GitHub Apps must have the `security_events` read permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#list-recent-code-scanning-analyses-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `sarif_id: &str` -- The SARIF ID obtained after uploading.
     */
    pub async fn get_sarif(
        &self,
        owner: &str,
        repo: &str,
        sarif_id: &str,
    ) -> Result<crate::types::CodeScanningSarifsStatus> {
        let url = format!(
            "/repos/{}/{}/code-scanning/sarifs/{}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&sarif_id.to_string()),
        );

        self.client.get(&url).await
    }
}

use anyhow::Result;
use github_scopes_rs::{oauth::OAuthContext, transform::GithubTokenScope};

pub struct OAuth {}

impl OAuth {
    /**
     * Scopes returns exactly what type of access you have by a given token.
     *
     * This function discover the exactly oauth scope permissions of the given token.
     *
     * **Note:** Accessing this endpoint does not count against your REST API rate limit.
     *
     * FROM: <https://docs.github.com/en/developers/apps/building-oauth-apps/scopes-for-oauth-apps>
     */
    pub fn from_token(token: &str) -> Result<GithubTokenScope> {
        let p = OAuthContext::new(token)?;
        Ok(p.get_scope_permissions())
    }
}

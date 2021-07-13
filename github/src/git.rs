use anyhow::Result;

use crate::Client;

pub struct Git {
    client: Client,
}

impl Git {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Git { client }
    }

    /**
     * Get a tree.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/git/trees/{tree_sha}` endpoint.
     *
     * Returns a single tree using the SHA1 value for that tree.
     *
     * If `truncated` is `true` in the response then the number of items in the `tree` array exceeded our maximum limit. If you need to fetch more items, use the non-recursive method of fetching trees, and fetch one sub-tree at a time.
     *
     * FROM: <https://docs.github.com/rest/reference/git#get-a-tree>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `tree_sha: &str`
     * * `recursive: &str` -- Setting this parameter to any value returns the objects or subtrees referenced by the tree specified in `:tree_sha`. For example, setting `recursive` to any of the following will enable returning objects or subtrees: `0`, `1`, `"true"`, and `"false"`. Omit this parameter to prevent recursively returning objects or subtrees.
     */
    pub async fn get_tree(
        &self,
        owner: &str,
        repo: &str,
        tree_sha: &str,
        recursive: &str,
    ) -> Result<crate::types::GitTree> {
        let url = format!(
            "/repos/{}/{}/git/trees/{}?recursive={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&tree_sha.to_string()),
            recursive.to_string(),
        );

        self.client.get(&url).await
    }
}

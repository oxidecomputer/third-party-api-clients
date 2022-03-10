use anyhow::Result;

use crate::Client;

pub struct Gitignore {
    pub client: Client,
}

impl Gitignore {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Gitignore { client }
    }

    /**
     * Get all gitignore templates.
     *
     * This function performs a `GET` to the `/gitignore/templates` endpoint.
     *
     * List all templates available to pass as an option when [creating a repository](https://docs.github.com/rest/reference/repos#create-a-repository-for-the-authenticated-user).
     *
     * FROM: <https://docs.github.com/rest/reference/gitignore#get-all-gitignore-templates>
     */
    pub async fn get_all_templates(&self) -> Result<Vec<String>> {
        let url = "/gitignore/templates".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Get all gitignore templates.
     *
     * This function performs a `GET` to the `/gitignore/templates` endpoint.
     *
     * As opposed to `get_all_templates`, this function returns all the pages of the request at once.
     *
     * List all templates available to pass as an option when [creating a repository](https://docs.github.com/rest/reference/repos#create-a-repository-for-the-authenticated-user).
     *
     * FROM: <https://docs.github.com/rest/reference/gitignore#get-all-gitignore-templates>
     */
    pub async fn get_all_all_templates(&self) -> Result<Vec<String>> {
        let url = "/gitignore/templates".to_string();
        self.client.get_all_pages(&url, None).await
    }

    /**
     * Get a gitignore template.
     *
     * This function performs a `GET` to the `/gitignore/templates/{name}` endpoint.
     *
     * The API also allows fetching the source of a single template.
     * Use the raw [media type](https://docs.github.com/rest/overview/media-types/) to get the raw contents.
     *
     * FROM: <https://docs.github.com/rest/reference/gitignore#get-a-gitignore-template>
     *
     * **Parameters:**
     *
     * * `name: &str`
     */
    pub async fn get_template(&self, name: &str) -> Result<crate::types::GitignoreTemplate> {
        let url = format!(
            "/gitignore/templates/{}",
            crate::progenitor_support::encode_path(&name.to_string()),
        );

        self.client.get(&url, None).await
    }
}

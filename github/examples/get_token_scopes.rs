use std::env;

use anyhow::{anyhow, Result};
use octorust::scopes;

fn main() -> Result<()> {
    let token = match env::var("GITHUB_TOKEN") {
        Ok(t) => t,
        Err(_e) => return Err(anyhow!("github token not provide")),
    };

    let permissions = scopes::OAuth::from_token(token.as_str())?;

    if !permissions.repo.all {
        return Err(anyhow!("`repo` permission is mandatory"));
    }
    Ok(())
}

use std::env;

#[cfg(feature = "httpcache")]
use octorust::http_cache::FileBasedCache;
use octorust::{
    auth::{Credentials, InstallationTokenGenerator, JWTCredentials},
    types::IssuesListState,
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id_str = env::var("GH_APP_ID").unwrap();
    let app_id = app_id_str.parse::<u64>().unwrap();

    let app_installation_id_str = env::var("GH_INSTALLATION_ID").unwrap();
    let app_installation_id = app_installation_id_str.parse::<u64>().unwrap();

    let encoded_private_key = env::var("GH_PRIVATE_KEY").unwrap();
    let private_key = base64::decode(encoded_private_key).unwrap();

    // Decode the key.
    let key = nom_pem::decode_block(&private_key).unwrap();

    // Get the JWT credentials.
    let jwt = JWTCredentials::new(app_id, key.data).unwrap();

    // Create the HTTP cache.
    #[cfg(feature = "httpcache")]
    let mut dir = dirs::home_dir().expect("Expected a home dir");
    #[cfg(feature = "httpcache")]
    dir.push(".cache/github");
    #[cfg(feature = "httpcache")]
    let http_cache = Box::new(FileBasedCache::new(dir));

    let token_generator = InstallationTokenGenerator::new(app_installation_id, jwt);

    #[cfg(not(feature = "httpcache"))]
    let github = Client::custom(
        "https://api.github.com",
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::InstallationToken(token_generator),
        reqwest::Client::builder().build().unwrap(),
    );

    #[cfg(feature = "httpcache")]
    let github = Client::custom(
        "https://api.github.com",
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::InstallationToken(token_generator),
        reqwest::Client::builder().build().unwrap(),
        http_cache,
    );

    // List the repos for an org.
    let repos = github
        .pulls()
        .list_all(
            "oxidecomputer",
            "rfd",
            IssuesListState::All,
            "",                 // head
            "",                 // base
            Default::default(), // sort
            Default::default(), // direction
        )
        .await
        .unwrap();
    println!("length: {}", repos.len());

    Ok(())
}

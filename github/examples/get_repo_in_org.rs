use std::env;

#[cfg(feature = "httpcache")]
use octorust::http_cache::FileBasedCache;
use octorust::{
    auth::{Credentials, InstallationTokenGenerator, JWTCredentials},
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

    let http = reqwest::Client::builder().build()?;
    let retry_policy =
        reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
    let client = reqwest_middleware::ClientBuilder::new(http)
        // Trace HTTP requests. See the tracing crate to make use of these traces.
        .with(reqwest_tracing::TracingMiddleware::default())
        // Retry failed requests.
        .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
            retry_policy,
        ))
        .build();

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
        client,
    );

    #[cfg(feature = "httpcache")]
    let github = Client::custom(
        "https://api.github.com",
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::InstallationToken(token_generator),
        client,
        http_cache,
    );

    // Get this repo.
    let repo = github
        .repos()
        .get("oxidecomputer", "third-party-api-clients")
        .await
        .unwrap();

    println!("{:#?}", repo);

    // Get a repo without a license.
    let repo = github
        .repos()
        .get("oxidecomputer", "configs")
        .await
        .unwrap();

    println!("{:#?}", repo);

    Ok(())
}

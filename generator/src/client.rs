use inflector::cases::snakecase::to_snake_case;

use crate::struct_name;

/*
 * Declare the client object:
 */
pub const GITHUB_TEMPLATE: &str = r#"/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    host_override: Option<String>,
    agent: String,
    client: reqwest_middleware::ClientWithMiddleware,
    credentials: Option<crate::auth::Credentials>,
    #[cfg(feature = "httpcache")]
    http_cache: crate::http_cache::BoxedHttpCache,
}

impl Client {
    pub fn new<A, C>(agent: A, credentials: C) -> ClientResult<Self>
    where
        A: Into<String>,
        C: Into<Option<crate::auth::Credentials>>,
    {
        let http = reqwest::Client::builder().build()?;
        let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        let client = reqwest_middleware::ClientBuilder::new(http)
            // Trace HTTP requests. See the tracing crate to make use of these traces.
            .with(reqwest_tracing::TracingMiddleware::default())
            // Retry failed requests.
            .with(
                reqwest_conditional_middleware::ConditionalMiddleware::new(
                    reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                    |req: &reqwest::Request| req.try_clone().is_some()
                )
            )
            .build();

        #[cfg(feature = "httpcache")]
        {
            Ok(Self::custom(
                agent,
                credentials,
                client,
                <dyn crate::http_cache::HttpCache>::noop(),
            ))
        }
        #[cfg(not(feature = "httpcache"))]
        {
            Ok(Self::custom(agent, credentials, client))
        }
    }

    #[cfg(feature = "httpcache")]
    pub fn custom<A, CR>(
        agent: A,
        credentials: CR,
        http: reqwest_middleware::ClientWithMiddleware,
        http_cache: crate::http_cache::BoxedHttpCache,
    ) -> Self
    where
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: RootDefaultServer::default().default_url().to_string(),
            host_override: None,
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
            http_cache,
        }
    }

    #[cfg(not(feature = "httpcache"))]
    pub fn custom<A, CR>(agent: A, credentials: CR, http: reqwest_middleware::ClientWithMiddleware) -> Self
    where
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: RootDefaultServer::default().default_url().to_string(),
            host_override: None,
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
        }
    }

    /// Override the host for all endpoins in the client.
    pub fn with_host_override<H>(&mut self, host: H) -> &mut Self
    where
        H: ToString,
    {
        self.host_override = Some(host.to_string());
        self
    }

    /// Disables the global host override for the client.
    pub fn remove_host_override(&mut self) -> &mut Self
    {
        self.host_override = None;
        self
    }

    pub fn get_host_override(&self) -> Option<&str> {
        self.host_override.as_deref()
    }

    pub(crate) fn url(&self, path: &str, host: Option<&str>) -> String {
        format!("{}{}", self.get_host_override().or(host).unwrap_or(self.host.as_str()), path)
    }

    pub fn set_credentials<CR>(&mut self, credentials: CR)
    where
        CR: Into<Option<crate::auth::Credentials>>,
    {
        self.credentials = credentials.into();
    }

    fn credentials(&self, authentication: crate::auth::AuthenticationConstraint) -> Option<&crate::auth::Credentials> {
        match (authentication, self.credentials.as_ref()) {
            (crate::auth::AuthenticationConstraint::Unconstrained, creds) => creds,
            (crate::auth::AuthenticationConstraint::JWT, creds @ Some(&crate::auth::Credentials::JWT(_))) => creds,
            (
                crate::auth::AuthenticationConstraint::JWT,
                Some(&crate::auth::Credentials::InstallationToken(ref apptoken)),
            ) => Some(apptoken.jwt()),
            (crate::auth::AuthenticationConstraint::JWT, _) => {
                log::info!(
                    "Request needs JWT authentication but only a mismatched method is available"
                );
                None
            }
        }
    }

    async fn url_and_auth(
        &self,
        uri: &str,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> ClientResult<(reqwest::Url, Option<String>)> {
        let mut parsed_url = uri.parse::<reqwest::Url>()?;

        match self.credentials(authentication) {
            Some(&crate::auth::Credentials::Client(ref id, ref secret)) => {
                parsed_url.query_pairs_mut()
                        .append_pair("client_id", id)
                        .append_pair("client_secret", secret);
                Ok((parsed_url, None))
            },
            Some(&crate::auth::Credentials::Token(ref token)) => {
                let auth = format!("token {}", token);
                Ok((parsed_url, Some(auth)))
            }
            Some(&crate::auth::Credentials::JWT(ref jwt)) => {
                let auth = format!("Bearer {}", jwt.token());
                Ok((parsed_url, Some(auth)))
            }
            Some(&crate::auth::Credentials::InstallationToken(ref apptoken)) => {
                let token = if let Some(token) = apptoken.token().await {
                    token
                } else {
                    let mut token_guard = apptoken.access_key.write().await;
                    if let Some(token) = token_guard.as_ref().and_then(|t| t.token()) {
                        token.to_owned()
                    } else {
                        log::debug!("app token is stale, refreshing");

                        let created_at = tokio::time::Instant::now();
                        let token = self
                            .apps()
                            .create_installation_access_token(
                                apptoken.installation_id as i64,
                                &types::AppsCreateInstallationAccessTokenRequest {
                                    permissions: Default::default(),
                                    repositories: Default::default(),
                                    repository_ids: Default::default(),
                                },
                            )
                            .await?;
                        *token_guard = Some(crate::auth::ExpiringInstallationToken::new(
                            token.token.clone(),
                            created_at,
                        ));
                        token.token
                    }
                };
                let auth = format!("token {}", token);
                Ok((parsed_url, Some(auth)))
            }
            None => Ok((parsed_url, None)),
        }
    }

    async fn request<Out>(
        &self,
        method: http::Method,
        uri: &str,
        message: Message,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> ClientResult<(Option<crate::utils::NextLink>, Out)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        #[cfg(feature = "httpcache")]
        let uri2 = uri.to_string();

        let (url, auth) = self.url_and_auth(uri, authentication).await?;

        let instance = <&Client>::clone(&self);

        #[cfg(not(feature = "httpcache"))]
        let mut req = instance.client.request(method, url);

        #[cfg(feature = "httpcache")]
        let mut req = {
            let mut req = instance.client.request(method.clone(), url);
            if method == http::Method::GET {
                if let Ok(etag) = instance.http_cache.lookup_etag(&uri2) {
                    req = req.header(http::header::IF_NONE_MATCH, etag);
                }
            }
            req
        };

        if let Some(content_type) = &message.content_type {
            req = req.header(http::header::CONTENT_TYPE, content_type.clone());
        }

        req = req.header(http::header::USER_AGENT, &*instance.agent);
        req = req.header(
            http::header::ACCEPT,
            &media_type.to_string()
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if let Some(body) = message.body {
            req = req.body(body);
        }
        let response = req.send().await?;

        #[cfg(feature = "httpcache")]
        let instance2 = <&Client>::clone(&self);

        #[cfg(feature = "httpcache")]
        let uri3 = uri.to_string();

        #[cfg(not(feature = "httpcache"))]
        let (remaining, reset) = crate::utils::get_header_values(response.headers());

        #[cfg(feature = "httpcache")]
        let (remaining, reset, etag) = crate::utils::get_header_values(response.headers());

        let status = response.status();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| parse_link_header::parse(l).ok());
        let next_link = link.as_ref().and_then(crate::utils::next_link);

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            #[cfg(feature = "httpcache")]
            {
                if let Some(etag) = etag {
                    if let Err(e) = instance2.http_cache.cache_response(
                        &uri3,
                        &response_body,
                        &etag,
                        &next_link.as_ref().map(|n| n.0.clone()),
                    ) {
                        // failing to cache isn't fatal, so just log & swallow the error
                        log::info!("failed to cache body & etag: {}", e);
                    }
                }
            }

            let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){
                serde_json::from_str("null")?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok((next_link, parsed_response))
        } else if status == http::StatusCode::NOT_MODIFIED {
                // only supported case is when client provides if-none-match
                // header when cargo builds with --cfg feature="httpcache"
                #[cfg(feature = "httpcache")]
                {
                    let body = instance2.http_cache.lookup_body(&uri3).unwrap();
                    let out = serde_json::from_str::<Out>(&body).unwrap();
                    let link = match next_link {
                        Some(next_link) => Ok(Some(next_link)),
                        None => instance2.http_cache.lookup_next_link(&uri3)
                            .map(|next_link| next_link.map(crate::utils::NextLink))
                    };
                    link.map(|link| (link, out))
                }
                #[cfg(not(feature = "httpcache"))]
                {
                    unreachable!("this should not be reachable without the httpcache feature enabled")
                }
        } else {
            let error = match (remaining, reset) {
                (Some(remaining), Some(reset)) if remaining == 0 => {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    ClientError::RateLimited{duration: u64::from(reset).saturating_sub(now)}
                },
                _ => {
                    if response_body.is_empty() {
                        ClientError::HttpError{status: status, error: "empty response".into()}
                    } else {
                        ClientError::HttpError{status: status, error: String::from_utf8_lossy(&response_body).into()}
                    }
                }
            };
            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        message: Message,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let (_ , r) = self.request(method, uri, message, media_type, authentication).await?;
        Ok(r)
    }

    async fn get<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.get_media(uri, crate::utils::MediaType::Json, message).await
    }

    async fn get_media<D>(&self, uri: &str, media: crate::utils::MediaType, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::GET,
            &uri,
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn get_all_pages<D>(&self, uri: &str,  _message: Message) -> ClientResult<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.unfold(uri).await
    }

    async fn get_pages<D>(&self, uri: &str) -> ClientResult<(Option<crate::utils::NextLink>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request(
            http::Method::GET,
            &uri,
            Message::default(),
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn get_pages_url<D>(&self, url: &reqwest::Url) -> ClientResult<(Option<crate::utils::NextLink>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request(
            http::Method::GET,
            url.as_str(),
            Message::default(),
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn post<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.post_media(
            uri,
            message,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn post_media<D>(
        &self,
        uri: &str,
        message: Message,
        media: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::POST,
            &uri,
            message,
            media,
            authentication,
        ).await
    }

    async fn patch_media<D>(&self, uri: &str, message: Message, media: crate::utils::MediaType) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PATCH,
            &uri,
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn patch<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.patch_media(uri, message, crate::utils::MediaType::Json).await
    }

    async fn put<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.put_media(uri, message, crate::utils::MediaType::Json).await
    }

    async fn put_media<D>(&self, uri: &str, message: Message, media: crate::utils::MediaType) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PUT,
            &uri,
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn delete<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &uri,
            message,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    /// "unfold" paginated results of a vector of items
    async fn unfold<D>(
        &self,
        uri: &str,
    ) -> ClientResult<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut items) = self.get_pages(uri).await?;
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = &link {
                let url = reqwest::Url::parse(&url.0)?;
                let (new_link, new_items) = self.get_pages_url(&url).await?;
                link = new_link;
                items = new_items;
            }
        }

        Ok(global_items)
    }"#;

pub fn generate_client_generic_token(
    proper_name: &str,
    token_endpoint: &str,
    user_consent_endpoint: &str,
    add_post_header: &str,
    servers: &GeneratedServers,
) -> String {
    let mut new_from_env = basic_new_from_env(proper_name, add_post_header, servers);
    if proper_name.starts_with("Google") {
        new_from_env = GOOGLE_NEW_FROM_ENV_TEMPLATE.to_string();
    }

    let add_post_header_struct = if add_post_header.is_empty() {
        "".to_string()
    } else {
        format!("{}: String,", to_snake_case(add_post_header))
    };

    let add_post_header_args = if add_post_header.is_empty() {
        "".to_string()
    } else {
        format!("{}: P,", to_snake_case(add_post_header))
    };

    let add_post_header_args_where = if add_post_header.is_empty() {
        "".to_string()
    } else {
        "P: ToString,".to_string()
    };

    let add_post_header_fn = if add_post_header.is_empty() {
        "".to_string()
    } else {
        format!(
            "{} : {}.to_string(),",
            to_snake_case(add_post_header),
            to_snake_case(add_post_header)
        )
    };

    let add_post_header_type = if !add_post_header.is_empty() {
        ", P".to_string()
    } else {
        "".to_string()
    };

    let consent_pattern = if proper_name.starts_with("Google") {
        "{}?client_id={}&access_type=offline&response_type=code&redirect_uri={}&state={}"
    } else {
        "{}?client_id={}&response_type=code&redirect_uri={}&state={}"
    };

    let token_auth_template = get_token_auth_template(consent_pattern);

    let server_block = servers.server_block();
    let server_arg = servers.server_arg();
    let server_to_host = servers.host_from_server();

    format!(
        r#"use std::sync::Arc;
use std::convert::TryInto;
use std::env;
use std::ops::Add;
use std::time::{{Duration, Instant}};
use tokio::sync::RwLock;

const TOKEN_ENDPOINT: &str = "https://{}";
const USER_CONSENT_ENDPOINT: &str = "https://{}";

{server_block}

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {{
    host: String,
    host_override: Option<String>,
    token: Arc<RwLock<InnerToken>>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    {}
    auto_refresh: bool,
    client: reqwest_middleware::ClientWithMiddleware,
}}

{}

/// Time in seconds before the access token expiration point that a refresh should
/// be performed. This value is subtracted from the `expires_in` value returned by
/// the provider prior to storing
const REFRESH_THRESHOLD: Duration = Duration::from_secs(60);

#[derive(Debug, Clone)]
struct InnerToken {{
    access_token: String,
    refresh_token: String,
    expires_at: Option<Instant>,
}}

impl Client {{
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<I, K, R, T, Q{}>(
        client_id: I,
        client_secret: K,
        redirect_uri: R,
        token: T,
        refresh_token: Q,
        {}
        {server_arg}
    ) -> Self
    where
        I: ToString,
        K: ToString,
        R: ToString,
        T: ToString,
        Q: ToString,
        {}
    {{
        // Retry up to 3 times with increasing intervals between attempts.
        let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        let client = reqwest::Client::builder().build();
        match client {{
            Ok(c) => {{
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(
                        reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some()
                        )
                    )
                    .build();

                {server_to_host}

                Client {{
                    host,
                    host_override: None,
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    redirect_uri: redirect_uri.to_string(),
                    token: Arc::new(RwLock::new(InnerToken {{
                        access_token: token.to_string(),
                        refresh_token: refresh_token.to_string(),
                        expires_at: None
                    }})),
                    {}
                    auto_refresh: false,
                    client,
                }}
            }}
            Err(e) => panic!("creating reqwest client failed: {{:?}}", e),
        }}
    }}

    /// Enables or disables the automatic refreshing of access tokens upon expiration
    pub fn set_auto_access_token_refresh(&mut self, enabled: bool) -> &mut Self {{
        self.auto_refresh = enabled;
        self
    }}

    /// Sets a specific `Instant` at which the access token should be considered expired.
    /// The expiration value will only be used when automatic access token refreshing is
    /// also enabled. `None` may be passed in if the expiration is unknown. In this case
    /// automatic refreshes will be attempted when encountering an UNAUTHENTICATED status
    /// code on a response.
    pub async fn set_expires_at(&self, expires_at: Option<Instant>) -> &Self {{
        self.token.write().await.expires_at = expires_at;
        self
    }}

    /// Gets the `Instant` at which the access token used by this client is set to expire
    /// if one is known
    pub async fn expires_at(&self) -> Option<Instant> {{
        self.token.read().await.expires_at
    }}

    /// Sets the number of seconds in which the current access token should be considered
    /// expired
    pub async fn set_expires_in(&self, expires_in: i64) -> &Self {{
        self.token.write().await.expires_at = Self::compute_expires_at(expires_in);
        self
    }}

    /// Gets the number of seconds from now in which the current access token will be
    /// considered expired if one is known
    pub async fn expires_in(&self) -> Option<Duration> {{
        self.token.read().await.expires_at.map(|i| i.duration_since(Instant::now()))
    }}

    /// Determines if the access token currently stored in the client is expired. If the
    /// expiration can not be determined, None is returned
    pub async fn is_expired(&self) -> Option<bool> {{
        self.token
            .read()
            .await
            .expires_at
            .map(|expiration| expiration <= Instant::now())
    }}

    fn compute_expires_at(expires_in: i64) -> Option<Instant> {{
        let seconds_valid = expires_in
            .try_into()
            .ok()
            .map(Duration::from_secs)
            .and_then(|dur| dur.checked_sub(REFRESH_THRESHOLD))
            .or_else(|| Some(Duration::from_secs(0)));

        seconds_valid.map(|seconds_valid| Instant::now().add(seconds_valid))
    }}

    /// Override the host for all endpoins in the client.
    pub fn with_host_override<H>(&mut self, host: H) -> &mut Self
    where
        H: ToString,
    {{
        self.host_override = Some(host.to_string());
        self
    }}

    /// Disables the global host override for the client.
    pub fn remove_host_override(&mut self) -> &mut Self
    {{
        self.host_override = None;
        self
    }}

    pub fn get_host_override(&self) -> Option<&str> {{
        self.host_override.as_deref()
    }}

    pub(crate) fn url(&self, path: &str, host: Option<&str>) -> String {{
        format!("{{}}{{}}", self.get_host_override().or(host).unwrap_or(self.host.as_str()), path)
    }}

    {}



    {}

    {}"#,
        token_endpoint.trim_start_matches("https://"),
        user_consent_endpoint.trim_start_matches("https://"),
        add_post_header_struct,
        ACCESS_TOKEN_STRUCT_TEMPLATE,
        add_post_header_type,
        add_post_header_args,
        add_post_header_args_where,
        add_post_header_fn,
        new_from_env,
        token_auth_template,
        get_shared_functions(proper_name, add_post_header)
    )
}

fn basic_new_from_env(
    proper_name: &str,
    add_post_header: &str,
    servers: &GeneratedServers,
) -> String {
    let add_post_header_type = if !add_post_header.is_empty() {
        ", P".to_string()
    } else {
        "".to_string()
    };
    let add_post_header_args = if !add_post_header.is_empty() {
        format!(",{}: P", to_snake_case(add_post_header))
    } else {
        "".to_string()
    };
    let add_post_header_args_where = if add_post_header.is_empty() {
        "".to_string()
    } else {
        "P: ToString,".to_string()
    };
    let add_post_header_fn = if !add_post_header.is_empty() {
        format!("{},", to_snake_case(add_post_header))
    } else {
        "".to_string()
    };

    let server_arg = servers.server_arg();
    let server_param = servers.server_param();

    format!(
        r#"
/// Create a new Client struct from environment variables. It
/// takes a type that can convert into
/// an &str (`String` or `Vec<u8>` for example). As long as the function is
/// given a valid API key and your requests will work.
/// We pass in the token and refresh token to the client so if you are storing
/// it in a database, you can get it first.
pub fn new_from_env<T, R{}>(token: T, refresh_token: R{}, {server_arg}) -> Self
where
    T: ToString,
    R: ToString,
    {}
{{
    let client_id = env::var("{}_CLIENT_ID").expect("must set {}_CLIENT_ID");
    let client_secret = env::var("{}_CLIENT_SECRET").expect("must set {}_CLIENT_SECRET");
    let redirect_uri = env::var("{}_REDIRECT_URI").expect("must set {}_REDIRECT_URI");

    Client::new(
        client_id,
        client_secret,
        redirect_uri,
        token,
        refresh_token,
        {}
        {server_param}
    )
}}"#,
        add_post_header_type,
        add_post_header_args,
        add_post_header_args_where,
        proper_name.to_uppercase().replace('.', ""),
        proper_name.to_uppercase().replace('.', ""),
        proper_name.to_uppercase().replace('.', ""),
        proper_name.to_uppercase().replace('.', ""),
        proper_name.to_uppercase().replace('.', ""),
        proper_name.to_uppercase().replace('.', ""),
        add_post_header_fn,
    )
}

const GOOGLE_NEW_FROM_ENV_TEMPLATE: &str = r#"
/// Create a new Client struct from environment variables. It
/// takes a type that can convert into
/// an &str (`String` or `Vec<u8>` for example). As long as the function is
/// given a valid API key and your requests will work.
/// We pass in the token and refresh token to the client so if you are storing
/// it in a database, you can get it first.
pub async fn new_from_env<T, R>(token: T, refresh_token: R) -> Self
where
    T: ToString,
    R: ToString,
{
    let google_key = env::var("GOOGLE_KEY_ENCODED").unwrap_or_default();
    let decoded_google_key = base64::decode(google_key).unwrap();
    let secret = yup_oauth2::parse_application_secret(decoded_google_key)
        .expect("failed to read from google credential env var");

    let client = reqwest::Client::builder().build();
    let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);

    match client {
        Ok(c) => {
            let client = reqwest_middleware::ClientBuilder::new(c)
                // Trace HTTP requests. See the tracing crate to make use of these traces.
                .with(reqwest_tracing::TracingMiddleware::default())
                // Retry failed requests.
                .with(
                    reqwest_conditional_middleware::ConditionalMiddleware::new(
                        reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                        |req: &reqwest::Request| req.try_clone().is_some()
                    )
                )
                .build();

            let host = RootDefaultServer::default().default_url().to_string();

            Client {
                host,
                host_override: None,
                client_id: secret.client_id.to_string(),
                client_secret: secret.client_secret.to_string(),
                redirect_uri: secret.redirect_uris[0].to_string(),
                token: Arc::new(RwLock::new(InnerToken {
                    access_token: token.to_string(),
                    refresh_token: refresh_token.to_string(),
                    expires_at: None
                })),
                auto_refresh: false,
                client,
            }
        },
        Err(e) => panic!("creating reqwest client failed: {:?}", e),
    }
}
"#;

pub fn generate_client_generic_api_key(
    proper_name: &str,
    add_post_header: &str,
    servers: &GeneratedServers,
) -> String {
    let server_block = if servers.count > 0 {
        servers.output.as_deref().unwrap()
    } else {
        ""
    };

    let server_arg = servers.server_arg();
    let server_to_host = servers.host_from_server();

    format!(
        r#"use std::env;

{server_block}

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {{
    host: String,
    host_override: Option<String>,
    token: String,

    client: reqwest_middleware::ClientWithMiddleware,
}}

impl Client {{
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<T>(
        token: T,
        {server_arg}
    ) -> Self
    where
        T: ToString,
    {{
        let client = reqwest::Client::builder().build();
        let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        match client {{
            Ok(c) => {{
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(
                        reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some()
                        )
                    )
                    .build();

                {server_to_host}

                Client {{
                    host,
                    host_override: None,
                    token: token.to_string(),

                    client,
                }}
            }}
            Err(e) => panic!("creating reqwest client failed: {{:?}}", e),
        }}
    }}

    /// Override the host for all endpoins in the client.
    pub fn with_host_override<H>(&mut self, host: H) -> &mut Self
    where
        H: ToString,
    {{
        self.host_override = Some(host.to_string());
        self
    }}

    /// Disables the global host override for the client.
    pub fn remove_host_override(&mut self) -> &mut Self
    {{
        self.host_override = None;
        self
    }}

    pub fn get_host_override(&self) -> Option<&str> {{
        self.host_override.as_deref()
    }}

    pub(crate) fn url(&self, path: &str, host: Option<&str>) -> String {{
        format!("{{}}{{}}", self.get_host_override().or(host).unwrap_or(self.host.as_str()), path)
    }}

    /// Create a new Client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key and your requests will work.
    /// We pass in the token and refresh token to the client so if you are storing
    /// it in a database, you can get it first.
    pub fn new_from_env() -> Self
    {{
        let token = env::var("{}_API_KEY").expect("must set {}_API_KEY");

        Client::new(
            token,
        )
    }}

    {}"#,
        proper_name.to_uppercase().replace('.', ""),
        proper_name.to_uppercase().replace('.', ""),
        get_shared_functions(proper_name, add_post_header)
    )
}

fn get_shared_functions(proper_name: &str, add_post_header: &str) -> String {
    let post_header_args = if !add_post_header.is_empty() {
        format!(
            r#"if method == reqwest::Method::POST {{
            req = req.header(
        reqwest::header::HeaderName::from_bytes(b"{}")?,
        reqwest::header::HeaderValue::from_str(&self.{})?,
    );
            }}"#,
            to_snake_case(add_post_header),
            to_snake_case(add_post_header)
        )
    } else {
        String::new()
    };

    let bearer = if proper_name == "Okta" {
        "SSWS".to_string()
    } else {
        "Bearer".to_string()
    };

    // Add auto refresh functionality to clients that support it
    let raw_request = if proper_name.starts_with("Google")
        || proper_name == "DocuSign"
        || proper_name == "Gusto"
        || proper_name == "MailChimp"
        || proper_name == "Ramp"
        || proper_name == "Shopify"
        || proper_name == "Slack"
        || proper_name == "Zoom"
    {
        get_shared_raw_functions_with_refresh("Bearer", &post_header_args)
    } else {
        get_shared_raw_functions_without_refresh(&bearer, &post_header_args)
    };

    format!(
        r#"
{}

async fn request<Out>(
    &self,
    method: reqwest::Method,
    uri: &str,
    message: Message,
) -> ClientResult<Out>
    where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let response = self.request_raw(method, uri, message).await?;

    let status = response.status();

    let response_body = response.bytes().await?;

    if status.is_success() {{
        log::debug!("Received successful response. Read payload.");
        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")?
        }} else {{
            serde_json::from_slice::<Out>(&response_body)?
        }};
        Ok(parsed_response)
    }} else {{
        let error = if response_body.is_empty() {{
            ClientError::HttpError{{status: status, error: "empty response".into()}}
        }} else {{
            ClientError::HttpError{{status: status, error: String::from_utf8_lossy(&response_body).into()}}
        }};

        Err(error)
    }}
}}

async fn request_with_links<Out>(
    &self,
    method: http::Method,
    uri: &str,
    message: Message,
) -> ClientResult<(Option<crate::utils::NextLink>, Out)>
where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let response = self.request_raw(method, uri, message).await?;

    let status = response.status();
    let link = response
        .headers()
        .get(http::header::LINK)
        .and_then(|l| l.to_str().ok())
        .and_then(|l| parse_link_header::parse(l).ok())
        .as_ref()
        .and_then(crate::utils::next_link);

    let response_body = response.bytes().await?;

    if status.is_success() {{
        log::debug!("Received successful response. Read payload.");

        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")?
        }} else {{
            serde_json::from_slice::<Out>(&response_body)?
        }};
        Ok((link, parsed_response))
    }} else {{
        let error = if response_body.is_empty() {{
            ClientError::HttpError{{status: status, error: "empty response".into()}}
        }} else {{
            ClientError::HttpError{{status: status, error: String::from_utf8_lossy(&response_body).into()}}
        }};
        Err(error)
    }}
}}

/* TODO: make this more DRY */
#[allow(dead_code)]
async fn post_form<Out>(
    &self,
    uri: &str,
    form: reqwest::multipart::Form,
) -> ClientResult<Out>
    where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let (url, auth) = self.url_and_auth(uri).await?;

    let instance = <&Client>::clone(&self);

    let mut req = instance.client.request(http::Method::POST, url);

    // Set the default headers.
    req = req.header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    if let Some(auth_str) = auth {{
        req = req.header(http::header::AUTHORIZATION, &*auth_str);
    }}

    req = req.multipart(form);

    let response = req.send().await?;

    let status = response.status();

    let response_body = response.bytes().await?;

    if status.is_success() {{
        log::debug!("Received successful response. Read payload.");
        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")?
        }} else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {{
            // Parse the output as a string.
            let s = String::from_utf8(response_body.to_vec())?;
            serde_json::from_value(serde_json::json!(&s))?
        }} else {{
            serde_json::from_slice::<Out>(&response_body)?
        }};
        Ok(parsed_response)
    }} else {{
        let error = if response_body.is_empty() {{
            ClientError::HttpError{{status: status, error: "empty response".into()}}
        }} else {{
            ClientError::HttpError{{status: status, error: String::from_utf8_lossy(&response_body).into()}}
        }};

        Err(error)
    }}
}}

/* TODO: make this more DRY */
#[allow(dead_code)]
async fn request_with_accept_mime<Out>(
    &self,
    method: reqwest::Method,
    uri: &str,
    accept_mime_type: &str,
) -> ClientResult<Out>
    where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let (url, auth) = self.url_and_auth(uri).await?;

    let instance = <&Client>::clone(&self);

    let mut req = instance.client.request(method, url);

    // Set the default headers.
    req = req.header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_str(accept_mime_type)?,
    );

    if let Some(auth_str) = auth {{
        req = req.header(http::header::AUTHORIZATION, &*auth_str);
    }}

    let response = req.send().await?;

    let status = response.status();

    let response_body = response.bytes().await?;

    if status.is_success() {{
        log::debug!("Received successful response. Read payload.");
        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")?
        }} else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {{
            // Parse the output as a string.
            let s = String::from_utf8(response_body.to_vec())?;
            serde_json::from_value(serde_json::json!(&s))?
        }} else {{
            serde_json::from_slice::<Out>(&response_body)?
        }};
        Ok(parsed_response)
    }} else {{
        let error = if response_body.is_empty() {{
            ClientError::HttpError{{status: status, error: "empty response".into()}}
        }} else {{
            ClientError::HttpError{{status: status, error: String::from_utf8_lossy(&response_body).into()}}
        }};

        Err(error)
    }}
}}

/* TODO: make this more DRY */
#[allow(dead_code)]
async fn request_with_mime<Out>(
    &self,
    method: reqwest::Method,
    uri: &str,
    content: &[u8],
    mime_type: &str,
) -> ClientResult<Out>
    where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let (url, auth) = self.url_and_auth(uri).await?;

    let instance = <&Client>::clone(&self);

    let mut req = instance.client.request(method, url);

    // Set the default headers.
    req = req.header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    req = req.header(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_bytes(mime_type.as_bytes()).unwrap(),
    );
    // We are likely uploading a file so add the right headers.
    req = req.header(
        reqwest::header::HeaderName::from_static("x-upload-content-type"),
        reqwest::header::HeaderValue::from_static("application/octet-stream"),
    );
    req = req.header(
        reqwest::header::HeaderName::from_static("x-upload-content-length"),
        reqwest::header::HeaderValue::from_bytes(format!("{{}}", content.len()).as_bytes()).unwrap(),
    );

    if let Some(auth_str) = auth {{
        req = req.header(http::header::AUTHORIZATION, &*auth_str);
    }}

    if content.len() > 1 {{
        let b = bytes::Bytes::copy_from_slice(content);
        // We are uploading a file so add that as the body.
        req = req.body(b);
    }}

    let response = req.send().await?;

    let status = response.status();

    let response_body = response.bytes().await?;

    if status.is_success() {{
        log::debug!("Received successful response. Read payload.");
        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")?
        }} else {{
            serde_json::from_slice::<Out>(&response_body)?
        }};
        Ok(parsed_response)
    }} else {{
        let error = if response_body.is_empty() {{
            ClientError::HttpError{{status: status, error: "empty response".into()}}
        }} else {{
            ClientError::HttpError{{status: status, error: String::from_utf8_lossy(&response_body).into()}}
        }};

        Err(error)
    }}
}}

async fn request_entity<D>(
    &self,
    method: http::Method,
    uri: &str,
    message: Message,
) -> ClientResult<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    let r = self
        .request(method, uri, message)
        .await?;
    Ok(r)
}}

#[allow(dead_code)]
async fn get<D>(&self, uri: &str,  message: Message) -> ClientResult<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::GET,
        uri,
        message,
    ).await
}}

#[allow(dead_code)]
async fn get_all_pages<D>(&self, uri: &str,  _message: Message) -> ClientResult<Vec<D>>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    // TODO: implement this.
    self.unfold(uri).await
}}

/// "unfold" paginated results of a vector of items
#[allow(dead_code)]
async fn unfold<D>(
    &self,
    uri: &str,
) -> ClientResult<Vec<D>>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    let mut global_items = Vec::new();
    let (new_link, mut items) = self.get_pages(uri).await?;
    let mut link = new_link;
    while !items.is_empty() {{
        global_items.append(&mut items);
        // We need to get the next link.
        if let Some(url) = link.as_ref() {{
            let url = reqwest::Url::parse(&url.0)?;
            let (new_link, new_items) = self.get_pages_url(&url).await?;
            link = new_link;
            items = new_items;
        }}
    }}

    Ok(global_items)
}}

#[allow(dead_code)]
async fn get_pages<D>(&self, uri: &str) -> ClientResult<(Option<crate::utils::NextLink>, Vec<D>)>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_with_links(
        http::Method::GET,
        uri,
        Message::default(),
    ).await
}}

#[allow(dead_code)]
async fn get_pages_url<D>(&self, url: &reqwest::Url) -> ClientResult<(Option<crate::utils::NextLink>, Vec<D>)>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_with_links(
        http::Method::GET,
        url.as_str(),
        Message::default(),
    ).await
}}

#[allow(dead_code)]
async fn post<D>(&self, uri: &str, message: Message) -> ClientResult<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::POST,
        uri,
        message,
    ).await
}}

#[allow(dead_code)]
async fn patch<D>(&self, uri: &str, message: Message) -> ClientResult<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::PATCH,
        uri,
        message,
    ).await
}}

#[allow(dead_code)]
async fn put<D>(&self, uri: &str, message: Message) -> ClientResult<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::PUT,
        uri,
        message,
    ).await
}}

#[allow(dead_code)]
async fn delete<D>(&self, uri: &str, message: Message) -> ClientResult<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::DELETE,
        uri,
        message,
    ).await
}}"#,
        raw_request
    )
}

fn get_shared_raw_functions_without_refresh(bearer: &str, post_header_args: &str) -> String {
    format!(
        r#"
async fn url_and_auth(
    &self,
    uri: &str,
) -> ClientResult<(reqwest::Url, Option<String>)> {{
    let parsed_url = uri.parse::<reqwest::Url>()?;
    let auth = format!("{} {{}}", self.token);
    Ok((parsed_url, Some(auth)))
}}

async fn request_raw(
    &self,
    method: reqwest::Method,
    uri: &str,
    message: Message,
) -> ClientResult<reqwest::Response>
{{
    let (url, auth) = self.url_and_auth(uri).await?;
    let instance = <&Client>::clone(&self);
    let mut req = instance.client.request(method.clone(), url);
    // Set the default headers.
    req = req.header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    if let Some(content_type) = &message.content_type {{
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_str(content_type).unwrap(),
        );
    }} else {{
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
    }}

    {}
    if let Some(auth_str) = auth {{
        req = req.header(http::header::AUTHORIZATION, &*auth_str);
    }}
    if let Some(body) = message.body {{
        req = req.body(body);
    }}
    Ok(req.send().await?)
}}
"#,
        bearer, post_header_args
    )
}

fn get_shared_raw_functions_with_refresh(bearer: &str, post_header_args: &str) -> String {
    format!(
        r#"
async fn url_and_auth(
    &self,
    uri: &str,
) -> ClientResult<(reqwest::Url, Option<String>)> {{
    let parsed_url = uri.parse::<reqwest::Url>()?;

    let auth = format!("{} {{}}", self.token.read().await.access_token);
    Ok((parsed_url, Some(auth)))
}}

async fn make_request(
    &self,
    method: &reqwest::Method,
    uri: &str,
    message: Message,
) -> ClientResult<reqwest::Request> {{
    let (url, auth) = self.url_and_auth(uri).await?;

    let instance = <&Client>::clone(&self);

    let mut req = instance.client.request(method.clone(), url);

    // Set the default headers.
    req = req.header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    if let Some(content_type) = &message.content_type {{
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_str(content_type).unwrap(),
        );
    }} else {{
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
    }}

    {}

    if let Some(auth_str) = auth {{
        req = req.header(http::header::AUTHORIZATION, &*auth_str);
    }}

    if let Some(body) = message.body {{
        req = req.body(body);
    }}

    Ok(req.build()?)
}}

async fn request_raw(
    &self,
    method: reqwest::Method,
    uri: &str,
    message: Message,
) -> ClientResult<reqwest::Response> {{
    if self.auto_refresh {{
        let expired = self.is_expired().await;

        match expired {{
            // We have a known expired token, we know we need to perform a refresh prior to
            // attempting to make a request
            Some(true) => {{
                self.refresh_access_token().await?;
            }}

            // We have a (theoretically) known good token available. We make an optimistic
            // attempting at the request. If the token is no longer good, then something other
            // than the expiration is triggering the failure. We defer handling of these errors
            // to the caller
            Some(false) => (),

            // We do not know what state we are in. We could have a valid or expired token.
            // Generally this means we are in one of two cases:
            //   1. We have not yet performed a token refresh, nor has the user provided
            //      expiration data, and therefore do not know the expiration of the user
            //      provided token
            //   2. The provider is returning unusable expiration times, at which point we
            //      choose to ignore them
            None => (),
        }}
    }}

    let req = self.make_request(&method, uri, message).await?;
    let resp = self.client.execute(req).await?;

    Ok(resp)
}}"#,
        bearer, post_header_args
    )
}

fn get_token_auth_template<S: AsRef<str>>(consent_pattern: S) -> String {
    format!(
        r#"
/// Return a user consent url with an optional set of scopes.
/// If no scopes are provided, they will not be passed in the url.
pub fn user_consent_url(&self, scopes: &[String]) -> String {{
    let state = uuid::Uuid::new_v4();

    let url = format!(
        "{}",
        USER_CONSENT_ENDPOINT, self.client_id, self.redirect_uri, state
    );

    if scopes.is_empty() {{
        return url;
    }}

    // Add the scopes.
    format!("{{}}&scope={{}}", url, scopes.join(" "))
}}

/// Refresh an access token from a refresh token. Client must have a refresh token
/// for this to work.
pub async fn refresh_access_token(&self) -> ClientResult<AccessToken> {{
    let response = {{
        let refresh_token = &self.token.read().await.refresh_token;

        if refresh_token.is_empty() {{
            return Err(ClientError::EmptyRefreshToken);
        }}

        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri),
        ];
        let client = reqwest::Client::new();
        client
            .post(TOKEN_ENDPOINT)
            .headers(headers)
            .form(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?
    }};

    // Unwrap the response.
    let t: AccessToken = response.json().await?;

    let refresh_token = self.token.read().await.refresh_token.clone();

    *self.token.write().await = InnerToken {{
        access_token: t.access_token.clone(),
        refresh_token,
        expires_at: Self::compute_expires_at(t.expires_in),
    }};

    Ok(t)
}}

/// Get an access token from the code returned by the URL paramter sent to the
/// redirect URL.
pub async fn get_access_token(&mut self, code: &str, state: &str) -> ClientResult<AccessToken> {{
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("client_id", &self.client_id),
        ("client_secret", &self.client_secret),
        ("redirect_uri", &self.redirect_uri),
        ("state", state),
    ];
    let client = reqwest::Client::new();
    let resp = client
        .post(TOKEN_ENDPOINT)
        .headers(headers)
        .form(&params)
        .basic_auth(&self.client_id, Some(&self.client_secret))
        .send()
        .await?;

    // Unwrap the response.
    let t: AccessToken = resp.json().await?;

    *self.token.write().await = InnerToken {{
        access_token: t.access_token.clone(),
        refresh_token: t.refresh_token.clone(),
        expires_at: Self::compute_expires_at(t.expires_in),
    }};

    Ok(t)
}}"#,
        consent_pattern.as_ref()
    )
}

const CLIENT_AUTH_TEMPLATE: &str = r#"
/// Get an access token from the code returned by the URL paramter sent to the
/// redirect URL.
pub async fn get_access_token(&mut self) -> ClientResult<AccessToken> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", &self.client_id),
        ("client_secret", &self.client_secret),
    ];
    let client = reqwest::Client::new();
    let resp = client
        .post(TOKEN_ENDPOINT)
        .headers(headers)
        .form(&params)
        .basic_auth(&self.client_id, Some(&self.client_secret))
        .send()
        .await?;

    // Unwrap the response.
    let t: AccessToken = resp.json().await?;

    self.token = t.access_token.to_string();

    Ok(t)
}"#;

pub fn generate_client_generic_client_credentials(
    proper_name: &str,
    token_endpoint: &str,
    add_post_header: &str,
    servers: &GeneratedServers,
) -> String {
    let server_block = if servers.count > 0 {
        servers.output.as_deref().unwrap()
    } else {
        ""
    };

    let server_arg = servers.server_arg();
    let server_to_host = servers.host_from_server();

    format!(
        r#"use std::env;

{server_block}

const TOKEN_ENDPOINT: &str = "https://{}";

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {{
    host: String,
    host_override: Option<String>,
    token: String,
    client_id: String,
    client_secret: String,
    client: reqwest_middleware::ClientWithMiddleware,
}}

{}

impl Client {{
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<I, K, T>(
        client_id: I,
        client_secret: K,
        token: T,
        {server_arg}
    ) -> Self
    where
        I: ToString,
        K: ToString,
        T: ToString,
    {{
        let client = reqwest::Client::builder().build();
        let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        match client {{
            Ok(c) => {{
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(
                        reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some()
                        )
                    )
                    .build();

                {server_to_host}

                Client {{
                    host,
                    host_override: None,
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    token: token.to_string(),

                    client,
                }}
            }}
            Err(e) => panic!("creating reqwest client failed: {{:?}}", e),
        }}
    }}
    
    /// Override the host for all endpoins in the client.
    pub fn with_host_override<H>(&mut self, host: H) -> &mut Self
    where
        H: ToString,
    {{
        self.host_override = Some(host.to_string());
        self
    }}

    /// Disables the global host override for the client.
    pub fn remove_host_override(&mut self) -> &mut Self
    {{
        self.host_override = None;
        self
    }}

    pub fn get_host_override(&self) -> Option<&str> {{
        self.host_override.as_deref()
    }}

    pub(crate) fn url(&self, path: &str, host: Option<&str>) -> String {{
        format!("{{}}{{}}", self.get_host_override().or(host).unwrap_or(self.host.as_str()), path)
    }}

    /// Create a new Client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key and your requests will work.
    /// We pass in the token and refresh token to the client so if you are storing
    /// it in a database, you can get it first.
    pub fn new_from_env<T>(token: T) -> Self
    where
        T: ToString,
    {{
        let client_id = env::var("{}_CLIENT_ID").expect("must set {}_CLIENT_ID");
        let client_secret = env::var("{}_CLIENT_SECRET").expect("must set {}_CLIENT_SECRET");

        Client::new(
            client_id,
            client_secret,
            token,
        )
    }}

    {}

    {}"#,
        token_endpoint.trim_start_matches("https://"),
        ACCESS_TOKEN_STRUCT_TEMPLATE,
        proper_name.to_uppercase().replace('.', ""),
        proper_name.to_uppercase().replace('.', ""),
        proper_name.to_uppercase().replace('.', ""),
        proper_name.to_uppercase().replace('.', ""),
        CLIENT_AUTH_TEMPLATE,
        get_shared_functions(proper_name, add_post_header)
    )
}

const ACCESS_TOKEN_STRUCT_TEMPLATE: &str = r#"use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Debug, JsonSchema, Clone, Default, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token_type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
    #[serde(default)]
    pub expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub refresh_token: String,
    #[serde(default, alias = "x_refresh_token_expires_in")]
    pub refresh_token_expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scope: String,
}"#;

#[derive(Clone, Debug, Default)]
pub struct GeneratedServers {
    pub count: u64,
    pub output: Option<String>,
    pub top_level_type: Option<String>,
}

impl GeneratedServers {
    pub fn server_block(&self) -> &str {
        self.output.as_deref().unwrap_or("")
    }

    pub fn server_arg(&self) -> String {
        if self.count > 1 {
            format!(
                "server: impl Into<{}>,",
                self.top_level_type.as_ref().unwrap()
            )
        } else {
            String::new()
        }
    }

    pub fn server_param(&self) -> String {
        if self.count > 1 {
            "server,".to_string()
        } else {
            String::new()
        }
    }

    pub fn host_from_server(&self) -> String {
        if self.count > 1 {
            String::from("let host = server.into().default_url().to_string();")
        } else if self.count == 1 {
            format!(
                "let host = {}::default().default_url().to_string();",
                self.top_level_type.as_ref().unwrap()
            )
        } else {
            String::from("let host = FALLBACK_HOST.to_string();")
        }
    }
}

pub fn generate_servers(servers: &[openapiv3::Server], server_prefix: &str) -> GeneratedServers {
    if servers.len() > 1 {
        let mut server_variants = String::new();
        let mut server_branches = String::new();
        let mut server_structs = String::new();
        let mut server_into = String::new();

        let server_enum = format!("{server_prefix}DefaultServers");

        for server in servers {
            if let Some(description) = &server.description {
                let server_name = struct_name(&format!("{server_prefix}{}Server", description));
                let server_struct = generate_server(&server_name, server);

                server_variants.push_str(&format!("{server_name}({server_name}),\n"));

                server_branches.push_str(&format!(
                    "Self::{server_name}(inner) => inner.default_url(),\n"
                ));

                server_structs.push_str(&format!("{server_struct}\n"));

                server_into.push_str(&format!(
                    r#"impl From<{server_name}> for {server_enum} {{
    fn from(server: {server_name}) -> Self {{
        Self::{server_name}(server)
    }}
}}
"#
                ));
            } else {
                panic!("If multiple servers are present, they must have descriptions")
            }
        }

        GeneratedServers {
            count: servers.len() as u64,
            output: Some(format!(
                r#"
pub enum {server_enum} {{
    {server_variants}
}}

impl {server_enum} {{
    pub fn default_url(&self) -> &str {{
        match self {{
            {server_branches}
        }}
    }}
}}

{server_structs}

{server_into}
"#
            )),
            top_level_type: Some(server_enum),
        }
    } else if servers.len() == 1 {
        let server = &servers[0];
        let server_struct_name = format!("{server_prefix}DefaultServer");
        let server_struct = generate_server(&server_struct_name, server);

        GeneratedServers {
            count: servers.len() as u64,
            output: Some(server_struct),
            top_level_type: Some(server_struct_name),
        }
    } else {
        GeneratedServers::default()
    }
}

// TODO: Only default server urls are supported
fn generate_server(server_name: &str, server: &openapiv3::Server) -> String {
    let mut url = server.url.clone();

    if let Some(variables) = &server.variables {
        for (variable_name, variable) in variables {
            let key = format!(r#"{{{variable_name}}}"#);
            url = url.replace(&key, &variable.default);
        }
    };

    let server_struct = format!(
        r#"
#[derive(Debug, Default, Clone)]
pub struct {server_name} {{
}}

impl {server_name} {{
    pub fn default_url(&self) -> &str {{
        "{url}"
    }}
}}
"#
    );

    server_struct
}

use inflector::cases::snakecase::to_snake_case;

/*
 * Declare the client object:
 */
pub const GITHUB_TEMPLATE: &str = r#"/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    agent: String,
    client: reqwest_middleware::ClientWithMiddleware,
    credentials: Option<crate::auth::Credentials>,
    #[cfg(feature = "httpcache")]
    http_cache: crate::http_cache::BoxedHttpCache,
}

impl Client {
    pub fn new<A, C>(agent: A, credentials: C) -> Result<Self>
    where
        A: Into<String>,
        C: Into<Option<crate::auth::Credentials>>,
    {
        Self::host(DEFAULT_HOST, agent, credentials)
    }

    pub fn host<H, A, C>(host: H, agent: A, credentials: C) -> Result<Self>
    where
        H: Into<String>,
        A: Into<String>,
        C: Into<Option<crate::auth::Credentials>>,
    {
        let http = reqwest::Client::builder().build()?;
        let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
                let client = reqwest_middleware::ClientBuilder::new(http)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware)
                    // Retry failed requests.
                    .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy))
                    .build();
        #[cfg(feature = "httpcache")]
        {
            Ok(Self::custom(
                host,
                agent,
                credentials,
                http:client,
                <dyn crate::http_cache::HttpCache>::noop(),
            ))
        }
        #[cfg(not(feature = "httpcache"))]
        {
            Ok(Self::custom(host, agent, credentials, client))
        }
    }

    #[cfg(feature = "httpcache")]
    pub fn custom<H, A, CR>(
        host: H,
        agent: A,
        credentials: CR,
        http: reqwest_middleware::ClientWithMiddleware,
        http_cache: crate::http_cache::BoxedHttpCache,
    ) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
            http_cache,
        }
    }

    #[cfg(not(feature = "httpcache"))]
    pub fn custom<H, A, CR>(host: H, agent: A, credentials: CR, http: reqwest_middleware::ClientWithMiddleware) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        CR: Into<Option<crate::auth::Credentials>>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client: http,
            credentials: credentials.into(),
        }
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
            (crate::auth::AuthenticationConstraint::JWT, creds) => {
                log::info!(
                    "Request needs JWT authentication but only {:?} available",
                    creds
                );
                None
            }
        }
    }

    async fn url_and_auth(
        &self,
        uri: &str,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();

        match self.credentials(authentication) {
            Some(&crate::auth::Credentials::Client(ref id, ref secret)) => parsed_url
                .map(|mut u| {
                    u.query_pairs_mut()
                        .append_pair("client_id", id)
                        .append_pair("client_secret", secret);
                    (u, None)
                })
                .map_err(Error::from),
            Some(&crate::auth::Credentials::Token(ref token)) => {
                let auth = format!("token {}", token);
                parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
            }
            Some(&crate::auth::Credentials::JWT(ref jwt)) => {
                let auth = format!("Bearer {}", jwt.token());
                parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
            }
            Some(&crate::auth::Credentials::InstallationToken(ref apptoken)) => {
                if let Some(token) = apptoken.token() {
                    let auth = format!("token {}", token);
                    parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
                } else {
                    log::debug!("app token is stale, refreshing");
                    let token_ref = apptoken.access_key.clone();

                    let token = self.apps().create_installation_access_token(apptoken.installation_id as i64,
                    &types::AppsCreateInstallationAccessTokenRequest{
                        permissions: Default::default(),
                        repositories: Default::default(),
                        repository_ids: Default::default(),
                    }).await?;
                    let auth = format!("token {}", &token.token);
                    *token_ref.lock().unwrap() = Some(token.token);
                    parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
                }
            }
            None => parsed_url.map(|u| (u, None)).map_err(Error::from),
        }
    }

    async fn request<Out>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<(Option<hyperx::header::Link>, Out)>
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

        req = req.header(http::header::USER_AGENT, &*instance.agent);
        req = req.header(
            http::header::ACCEPT,
            &*format!("{}", hyperx::header::qitem::<mime::Mime>(From::from(media_type))),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if let Some(body) = body {
            log::debug!("body: {:?}", String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap());
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
            .and_then(|l| l.parse().ok());

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("response payload {}", String::from_utf8_lossy(&response_body));
            #[cfg(feature = "httpcache")]
            {
                if let Some(etag) = etag {
                    let next_link = link.as_ref().and_then(|l| crate::utils::next_link(l));
                    if let Err(e) = instance2.http_cache.cache_response(
                        &uri3,
                        &response_body,
                        &etag,
                        &next_link,
                    ) {
                        // failing to cache isn't fatal, so just log & swallow the error
                        log::info!("failed to cache body & etag: {}", e);
                    }
                }
            }

            let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map(|out| (link, out)).map_err(Error::from)
        } else if status == http::StatusCode::NOT_MODIFIED {
                // only supported case is when client provides if-none-match
                // header when cargo builds with --cfg feature="httpcache"
                #[cfg(feature = "httpcache")]
                {
                    let body = instance2.http_cache.lookup_body(&uri3).unwrap();
                    let out = serde_json::from_str::<Out>(&body).unwrap();
                    let link = match link {
                        Some(link) => Ok(Some(link)),
                        None => instance2.http_cache.lookup_next_link(&uri3)
                                    .map(|next_link| next_link.map(|next| {
                                        let next = hyperx::header::LinkValue::new(next).push_rel(hyperx::header::RelationType::Next);
                                        hyperx::header::Link::new(vec![next])
                                    }))
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
                    anyhow!("rate limit exceeded, will reset in {} seconds", u64::from(reset) - now)
                },
                _ => {
                    if response_body.is_empty() {
                        anyhow!("code: {}, empty response", status)
                    } else {
                        anyhow!("code: {}, error: {:?}", status, String::from_utf8_lossy(&response_body),)
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
        body: Option<reqwest::Body>,
        media_type: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let (_ , r) = self.request(method, uri, body, media_type, authentication).await?;
        Ok(r)
    }

    async fn get<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.get_media(uri, crate::utils::MediaType::Json, message).await
    }

    async fn get_media<D>(&self, uri: &str, media: crate::utils::MediaType, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::GET,
            &(self.host.clone() + uri),
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn get_all_pages<D>(&self, uri: &str,  _message: Option<reqwest::Body>) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.unfold(uri).await
    }

    async fn get_pages<D>(&self, uri: &str) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request(
            http::Method::GET,
            &(self.host.clone() + uri),
            None,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn get_pages_url<D>(&self, url: &reqwest::Url) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request(
            http::Method::GET,
            url.as_str(),
            None,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
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
        message: Option<reqwest::Body>,
        media: crate::utils::MediaType,
        authentication: crate::auth::AuthenticationConstraint,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::POST,
            &(self.host.clone() + uri),
            message,
            media,
            authentication,
        ).await
    }

    async fn patch_media<D>(&self, uri: &str, message: Option<reqwest::Body>, media: crate::utils::MediaType) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PATCH,
            &(self.host.clone() + uri),
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.patch_media(uri, message, crate::utils::MediaType::Json).await
    }

    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.put_media(uri, message, crate::utils::MediaType::Json).await
    }

    async fn put_media<D>(&self, uri: &str, message: Option<reqwest::Body>, media: crate::utils::MediaType) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PUT,
            &(self.host.clone() + uri),
            message,
            media,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(self.host.clone() + uri),
            message,
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    /// "unfold" paginated results of a vector of items
    async fn unfold<D>(
        &self,
        uri: &str,
    ) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut items) = self.get_pages(uri).await?;
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = link.as_ref().and_then(|l| crate::utils::next_link(l)) {
                let url = reqwest::Url::parse(&url)?;
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
) -> String {
    let mut new_from_env = basic_new_from_env(proper_name, add_post_header);
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

    format!(
        r#"use std::env;

const TOKEN_ENDPOINT: &str = "https://{}";
const USER_CONSENT_ENDPOINT: &str = "https://{}";

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {{
    host: String,
    token: String,
    // This will expire within a certain amount of time as determined by the
    // expiration date passed back in the initial request.
    refresh_token: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    {}

    client: reqwest_middleware::ClientWithMiddleware,
}}

{}

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
                // We do not refresh the access token here since we leave that up to the
                // user to do so they can re-save it to their database.
                // TODO: But in the future we should save the expires in date and refresh it
                // if it needs to be refreshed.
                //
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware)
                    // Retry failed requests.
                    .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy))
                    .build();

                Client {{
                    host: DEFAULT_HOST.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    redirect_uri: redirect_uri.to_string(),
                    token: token.to_string(),
                    refresh_token: refresh_token.to_string(),
                    {}

                    client,
                }}
            }}
            Err(e) => panic!("creating reqwest client failed: {{:?}}", e),
        }}
    }}

    /// Override the default host for the client.
    pub fn with_host<H>(&self, host: H) -> Self
    where
        H: ToString,
    {{
        let mut c = self.clone();
        c.host = host.to_string();
        c
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
        TOKEN_AUTH_TEMPLATE,
        get_shared_functions(proper_name, add_post_header)
    )
}

fn basic_new_from_env(proper_name: &str, add_post_header: &str) -> String {
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

    format!(
        r#"
/// Create a new Client struct from environment variables. It
/// takes a type that can convert into
/// an &str (`String` or `Vec<u8>` for example). As long as the function is
/// given a valid API key and your requests will work.
/// We pass in the token and refresh token to the client so if you are storing
/// it in a database, you can get it first.
pub fn new_from_env<T, R{}>(token: T, refresh_token: R{}) -> Self
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
    let b = base64::decode(google_key).unwrap();
    // Save the google key to a tmp file.
    let mut file_path = env::temp_dir();
    file_path.push("google_key.json");
    // Create the file and write to it.
    let mut file = std::fs::File::create(file_path.clone()).unwrap();
    file.write_all(&b).unwrap();
    // Set the Google credential file to the temp path.
    let google_credential_file = file_path.to_str().unwrap().to_string();

    let secret = yup_oauth2::read_application_secret(google_credential_file)
        .await
        .expect("failed to read google credential file");

    let client = reqwest::Client::builder().build();
        let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        match client {
            Ok(c) => {
            // We do not refresh the access token here since we leave that up to the
            // user to do so they can re-save it to their database.
            // TODO: But in the future we should save the expires in date and refresh it
            // if it needs to be refreshed.
            //
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware)
                    // Retry failed requests.
                    .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy))
                    .build();

            Client {
                host: DEFAULT_HOST.to_string(),
                client_id: secret.client_id.to_string(),
                client_secret: secret.client_secret.to_string(),
                redirect_uri: secret.redirect_uris[0].to_string(),
                token: token.to_string(),
                refresh_token: refresh_token.to_string(),

                client,
            }
        },
        Err(e) => panic!("creating reqwest client failed: {:?}", e),
    }
}
"#;

pub fn generate_client_generic_api_key(proper_name: &str, add_post_header: &str) -> String {
    format!(
        r#"use std::env;

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {{
    host: String,
    token: String,

    client: reqwest_middleware::ClientWithMiddleware,
}}

impl Client {{
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<T>(
        token: T,
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
                    .with(reqwest_tracing::TracingMiddleware)
                    // Retry failed requests.
                    .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy))
                    .build();

                Client {{
                    host: DEFAULT_HOST.to_string(),
                    token: token.to_string(),

                    client,
                }}
            }}
            Err(e) => panic!("creating reqwest client failed: {{:?}}", e),
        }}
    }}

    /// Override the default host for the client.
    pub fn with_host<H>(&self, host: H) -> Self
    where
        H: ToString,
    {{
        let mut c = self.clone();
        c.host = host.to_string();
        c
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

    format!(
        r#"
async fn url_and_auth(
    &self,
    uri: &str,
) -> Result<(reqwest::Url, Option<String>)> {{
    let parsed_url = uri.parse::<reqwest::Url>();

    let auth = format!("{} {{}}", self.token);
    parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
}}

async fn request_raw(
    &self,
    method: reqwest::Method,
    uri: &str,
    body: Option<reqwest::Body>,
) -> Result<reqwest::Response>
{{
    let u = if uri.starts_with("https://") {{
        uri.to_string()
    }} else {{
        (self.host.clone() + uri).to_string()
    }};
    let (url, auth) = self.url_and_auth(&u).await?;

    let instance = <&Client>::clone(&self);

    let mut req = instance.client.request(method.clone(), url);

    // Set the default headers.
    req = req.header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    req = req.header(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    {}

    if let Some(auth_str) = auth {{
        req = req.header(http::header::AUTHORIZATION, &*auth_str);
    }}

    if let Some(body) = body {{
        log::debug!("body: {{:?}}", String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap());
        req = req.body(body);
    }}
    Ok(req.send().await?)
}}

async fn request<Out>(
    &self,
    method: reqwest::Method,
    uri: &str,
    body: Option<reqwest::Body>,
) -> Result<Out>
    where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let response = self.request_raw(method, uri, body).await?;

    let status = response.status();

    let response_body = response.bytes().await?;

    if status.is_success() {{
        log::debug!("response payload {{}}", String::from_utf8_lossy(&response_body));
        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")
        }} else {{
            serde_json::from_slice::<Out>(&response_body)
        }};
        parsed_response.map_err(Error::from)
    }} else {{
        let error = if response_body.is_empty() {{
            anyhow!("code: {{}}, empty response", status)
        }} else {{
            anyhow!(
                "code: {{}}, error: {{:?}}",
                status,
                String::from_utf8_lossy(&response_body),
            )
        }};

        Err(error)
    }}
}}

async fn request_with_links<Out>(
    &self,
    method: http::Method,
    uri: &str,
    body: Option<reqwest::Body>,
) -> Result<(Option<hyperx::header::Link>, Out)>
where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let response = self.request_raw(method, uri, body).await?;

    let status = response.status();
    let link = response
        .headers()
        .get(http::header::LINK)
        .and_then(|l| l.to_str().ok())
        .and_then(|l| l.parse().ok());

    let response_body = response.bytes().await?;

    if status.is_success() {{
        log::debug!("response payload {{}}", String::from_utf8_lossy(&response_body));

        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")
        }} else {{
            serde_json::from_slice::<Out>(&response_body)
        }};
        parsed_response.map(|out| (link, out)).map_err(Error::from)
    }} else {{
        let error = if response_body.is_empty() {{
            anyhow!("code: {{}}, empty response", status)
        }} else {{
            anyhow!(
                "code: {{}}, error: {{:?}}",
                status,
                String::from_utf8_lossy(&response_body),
            )
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
) -> Result<Out>
    where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let u = if uri.starts_with("https://") {{
        uri.to_string()
    }} else {{
        (self.host.clone() + uri).to_string()
    }};
    let (url, auth) = self.url_and_auth(&u).await?;

    let instance = <&Client>::clone(&self);

    let mut req = instance.client.request(http::Method::POST, url);

    // Set the default headers.
    req = req.header(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    req = req.header(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    if let Some(auth_str) = auth {{
        req = req.header(http::header::AUTHORIZATION, &*auth_str);
    }}

    log::debug!("form: {{:?}}", form);
    req = req.multipart(form);

    let response = req.send().await?;

    let status = response.status();

    let response_body = response.bytes().await?;

    if status.is_success() {{
        log::debug!("response payload {{}}", String::from_utf8_lossy(&response_body));
        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")
        }} else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {{
            // Parse the output as a string.
            serde_json::from_value(serde_json::json!(&String::from_utf8(response_body.to_vec())?))
        }} else {{
            serde_json::from_slice::<Out>(&response_body)
        }};
        parsed_response.map_err(Error::from)
    }} else {{
        let error = if response_body.is_empty() {{
            anyhow!("code: {{}}, empty response", status)
        }} else {{
            anyhow!(
                "code: {{}}, error: {{:?}}",
                status,
                String::from_utf8_lossy(&response_body),
            )
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
) -> Result<Out>
    where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let u = if uri.starts_with("https://") {{
        uri.to_string()
    }} else {{
        (self.host.clone() + uri).to_string()
    }};
    let (url, auth) = self.url_and_auth(&u).await?;

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
        log::debug!("response payload {{}}", String::from_utf8_lossy(&response_body));
        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")
        }} else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {{
            // Parse the output as a string.
            serde_json::from_value(serde_json::json!(&String::from_utf8(response_body.to_vec())?))
        }} else {{
            serde_json::from_slice::<Out>(&response_body)
        }};
        parsed_response.map_err(Error::from)
    }} else {{
        let error = if response_body.is_empty() {{
            anyhow!("code: {{}}, empty response", status)
        }} else {{
            anyhow!(
                "code: {{}}, error: {{:?}}",
                status,
                String::from_utf8_lossy(&response_body),
            )
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
) -> Result<Out>
    where
    Out: serde::de::DeserializeOwned + 'static + Send,
{{
    let u = if uri.starts_with("https://") {{
        uri.to_string()
    }} else {{
        (self.host.clone() + uri).to_string()
    }};
    let (url, auth) = self.url_and_auth(&u).await?;

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
        log::debug!("response payload {{}}", String::from_utf8_lossy(&response_body));
        let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){{
            serde_json::from_str("null")
        }} else {{
            serde_json::from_slice::<Out>(&response_body)
        }};
        parsed_response.map_err(Error::from)
    }} else {{
        let error = if response_body.is_empty() {{
            anyhow!("code: {{}}, empty response", status)
        }} else {{
            anyhow!(
                "code: {{}}, error: {{:?}}",
                status,
                String::from_utf8_lossy(&response_body),
            )
        }};

        Err(error)
    }}
}}

async fn request_entity<D>(
    &self,
    method: http::Method,
    uri: &str,
    body: Option<reqwest::Body>,
) -> Result<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    let r = self
        .request(method, uri, body)
        .await?;
    Ok(r)
}}

#[allow(dead_code)]
async fn get<D>(&self, uri: &str,  message: Option<reqwest::Body>) -> Result<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::GET,
        &(self.host.to_string() + uri),
        message,
    ).await
}}

#[allow(dead_code)]
async fn get_all_pages<D>(&self, uri: &str,  _message: Option<reqwest::Body>) -> Result<Vec<D>>
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
) -> Result<Vec<D>>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    let mut global_items = Vec::new();
    let (new_link, mut items) = self.get_pages(uri).await?;
    let mut link = new_link;
    while !items.is_empty() {{
        global_items.append(&mut items);
        // We need to get the next link.
        if let Some(url) = link.as_ref().and_then(|l| crate::utils::next_link(l)) {{
            let url = reqwest::Url::parse(&url)?;
            let (new_link, new_items) = self.get_pages_url(&url).await?;
            link = new_link;
            items = new_items;
        }}
    }}

    Ok(global_items)
}}

#[allow(dead_code)]
async fn get_pages<D>(&self, uri: &str) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_with_links(
        http::Method::GET,
        &(self.host.to_string() + uri),
        None,
    ).await
}}

#[allow(dead_code)]
async fn get_pages_url<D>(&self, url: &reqwest::Url) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_with_links(
        http::Method::GET,
        url.as_str(),
        None,
    ).await
}}

#[allow(dead_code)]
async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::POST,
        &(self.host.to_string() + uri),
        message,
    ).await
}}

#[allow(dead_code)]
async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::PATCH,
        &(self.host.to_string() + uri),
        message,
    ).await
}}

#[allow(dead_code)]
async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::PUT,
        &(self.host.to_string() + uri),
        message,
    ).await
}}

#[allow(dead_code)]
async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
where
    D: serde::de::DeserializeOwned + 'static + Send,
{{
    self.request_entity(
        http::Method::DELETE,
        &(self.host.to_string() + uri),
        message,
    ).await
}}"#,
        bearer, post_header_args
    )
}

const TOKEN_AUTH_TEMPLATE: &str = r#"
/// Return a user consent url with an optional set of scopes.
/// If no scopes are provided, they will not be passed in the url.
pub fn user_consent_url(&self, scopes: &[String]) -> String {
    let state = uuid::Uuid::new_v4();

    let url = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}&state={}",
        USER_CONSENT_ENDPOINT, self.client_id, self.redirect_uri, state
    );

    if scopes.is_empty() {
        return url;
    }

    // Add the scopes.
    format!("{}&scope={}", url, scopes.join(" "))
}

/// Refresh an access token from a refresh token. Client must have a refresh token
/// for this to work.
pub async fn refresh_access_token(&mut self) -> Result<AccessToken> {
    if self.refresh_token.is_empty() {
        anyhow!("refresh token cannot be empty");
    }

    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &self.refresh_token),
        ("client_id", &self.client_id),
        ("client_secret", &self.client_secret),
        ("redirect_uri", &self.redirect_uri),
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
    self.refresh_token = t.refresh_token.to_string();

    Ok(t)
}

/// Get an access token from the code returned by the URL paramter sent to the
/// redirect URL.
pub async fn get_access_token(&mut self, code: &str, state: &str) -> Result<AccessToken> {
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

    self.token = t.access_token.to_string();
    self.refresh_token = t.refresh_token.to_string();

    Ok(t)
}"#;

const CLIENT_AUTH_TEMPLATE: &str = r#"
/// Get an access token from the code returned by the URL paramter sent to the
/// redirect URL.
pub async fn get_access_token(&mut self) -> Result<AccessToken> {
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
) -> String {
    format!(
        r#"use std::env;


const TOKEN_ENDPOINT: &str = "https://{}";

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {{
    host: String,
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
                    .with(reqwest_tracing::TracingMiddleware)
                    // Retry failed requests.
                    .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy))
                    .build();

                Client {{
                    host: DEFAULT_HOST.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    token: token.to_string(),

                    client,
                }}
            }}
            Err(e) => panic!("creating reqwest client failed: {{:?}}", e),
        }}
    }}

    /// Override the default host for the client.
    pub fn with_host<H>(&self, host: H) -> Self
    where
        H: ToString,
    {{
        let mut c = self.clone();
        c.host = host.to_string();
        c
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

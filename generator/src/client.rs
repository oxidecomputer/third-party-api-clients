/*
 * Declare the client object:
 */
pub const TEMPLATE: &str = r#"/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    agent: String,
    client: reqwest::Client,
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
        #[cfg(feature = "httpcache")]
        {
            Ok(Self::custom(
                host,
                agent,
                credentials,
                http,
                <dyn crate::http_cache::HttpCache>::noop(),
            ))
        }
        #[cfg(not(feature = "httpcache"))]
        {
            Ok(Self::custom(host, agent, credentials, http))
        }
    }

    #[cfg(feature = "httpcache")]
    pub fn custom<H, A, CR>(
        host: H,
        agent: A,
        credentials: CR,
        http: reqwest::Client,
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
    pub fn custom<H, A, CR>(host: H, agent: A, credentials: CR, http: reqwest::Client) -> Self
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
                println!(
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
                    println!("App token is stale, refreshing");
                    let token_ref = apptoken.access_key.clone();

                    let token = self.apps().create_installation_access_token(apptoken.installation_id as i64,
                    &types::AppsCreateInstallationAccessTokenRequest{
                        permissions: Default::default(),
                        repositories: Default::default(),
                        repository_ids: Default::default(),
                    }).await.unwrap();
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
            //println!("Body: {:?}", String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap());
            req = req.body(body);
        }
        //println!("Request: {:?}", &req);
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
            //println!("response payload {}", String::from_utf8_lossy(&response_body));
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
                        println!("Failed to cache body & etag: {}", e);
                    }
                }
            }
            let parsed_response = if status == http::StatusCode::NO_CONTENT { serde_json::from_str("null") } else { serde_json::from_slice::<Out>(&response_body) };
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
            /*println!("error status: {:?}, response payload: {}",
                status,
                String::from_utf8_lossy(&response_body),
            );*/
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
                        anyhow!("code: {}, error: {:?}", status, serde_json::from_slice(&response_body)?)
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

    async fn get<D>(&self, uri: &str) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.get_media(uri, crate::utils::MediaType::Json).await
    }

    async fn get_media<D>(&self, uri: &str, media: crate::utils::MediaType) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::GET,
            &(self.host.clone() + uri),
            None,
            media,
            self::auth::AuthenticationConstraint::Unconstrained,
        ).await
    }

    async fn get_all_pages<D>(&self, uri: &str) -> Result<Vec<D>>
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
        let (new_link, mut items) = self.get_pages(uri).await.unwrap();
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = link.as_ref().and_then(|l| crate::utils::next_link(l)) {
                let url = reqwest::Url::parse(&url).unwrap();
                let (new_link, new_items) = self.get_pages_url(&url).await?;
                link = new_link;
                items = new_items;
            }
        }

        Ok(global_items)
    }"#;

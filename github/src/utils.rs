const X_GITHUB_REQUEST_ID: &str = "x-github-request-id";
const X_RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
const X_RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";
const X_RATELIMIT_RESET: &str = "x-ratelimit-reset";

pub fn next_link(l: &hyperx::header::Link) -> Option<String> {
    l.values().iter().find_map(|value| {
        value.rel().and_then(|rels| {
            if rels
                .iter()
                .any(|rel| rel == &hyperx::header::RelationType::Next)
            {
                Some(value.link().into())
            } else {
                None
            }
        })
    })
}

#[cfg(not(feature = "httpcache"))]
type HeaderValues = (Option<u32>, Option<u32>);
#[cfg(feature = "httpcache")]
type HeaderValues = (Option<u32>, Option<u32>, Option<Vec<u8>>);

pub fn get_header_values(
    headers: &http::header::HeaderMap<http::header::HeaderValue>,
) -> HeaderValues {
    if let Some(value) = headers.get(X_GITHUB_REQUEST_ID) {
        println!("x-github-request-id: {:?}", value)
    }
    if let Some(value) = headers.get(X_RATELIMIT_LIMIT) {
        println!("x-rate-limit-limit: {:?}", value)
    }
    let remaining = headers
        .get(X_RATELIMIT_REMAINING)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u32>().ok());
    let reset = headers
        .get(X_RATELIMIT_RESET)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u32>().ok());
    if let Some(value) = remaining {
        println!("x-rate-limit-remaining: {}", value)
    }
    if let Some(value) = reset {
        println!("x-rate-limit-reset: {}", value)
    }
    let etag = headers.get(http::header::ETAG);
    if let Some(value) = etag {
        println!("etag: {:?}", value)
    }

    #[cfg(feature = "httpcache")]
    {
        let etag = etag.map(|etag| etag.as_bytes().to_vec());
        (remaining, reset, etag)
    }
    #[cfg(not(feature = "httpcache"))]
    (remaining, reset)
}

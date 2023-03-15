//! Implements <https://tools.ietf.org/html/rfc7232> Conditional Requests.
use std::{
    collections::hash_map::DefaultHasher,
    ffi::OsStr,
    fmt::Debug,
    fs,
    hash::{Hash, Hasher},
    io,
    path::{Path, PathBuf},
};

use crate::ClientError;
use crate::ClientResult;
use http::Uri;

/// A type for an HTTP cache.
pub type BoxedHttpCache = Box<dyn HttpCache + Send + Sync>;

/// Implements a cached response and looking up the etag and next link.
pub trait HttpCache: HttpCacheClone + Debug {
    fn cache_response(
        &self,
        uri: &str,
        body: &[u8],
        etag: &[u8],
        next_link: &Option<String>,
    ) -> ClientResult<()>;
    fn lookup_etag(&self, uri: &str) -> ClientResult<String>;
    fn lookup_body(&self, uri: &str) -> ClientResult<String>;
    fn lookup_next_link(&self, uri: &str) -> ClientResult<Option<String>>;
}

impl dyn HttpCache {
    pub fn noop() -> BoxedHttpCache {
        Box::new(NoCache)
    }

    pub fn in_home_dir() -> BoxedHttpCache {
        Self::in_dir(&dirs::home_dir().expect("Expected a home dir"))
    }

    pub fn in_dir(path: &Path) -> BoxedHttpCache {
        if path.is_dir() {
            let mut path = path.to_owned();
            path.push(".github/cache");
            Box::new(FileBasedCache::new(path))
        } else {
            panic!("Expected a dir");
        }
    }
}

impl Clone for BoxedHttpCache {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

/// Noop type for no cache.
#[derive(Clone, Debug)]
pub struct NoCache;

impl HttpCache for NoCache {
    fn cache_response(&self, _: &str, _: &[u8], _: &[u8], _: &Option<String>) -> ClientResult<()> {
        Ok(())
    }
    fn lookup_etag(&self, _uri: &str) -> ClientResult<String> {
        no_read("No etag cached")
    }
    fn lookup_body(&self, _uri: &str) -> ClientResult<String> {
        no_read("No body cached")
    }
    fn lookup_next_link(&self, _uri: &str) -> ClientResult<Option<String>> {
        no_read("No next link cached")
    }
}

/// A file based cache.
#[derive(Clone, Debug)]
pub struct FileBasedCache {
    root: PathBuf,
}

impl FileBasedCache {
    #[doc(hidden)] // public for integration testing only
    pub fn new<P: Into<PathBuf>>(root: P) -> FileBasedCache {
        FileBasedCache { root: root.into() }
    }
}

impl HttpCache for FileBasedCache {
    fn cache_response(
        &self,
        uri: &str,
        body: &[u8],
        etag: &[u8],
        next_link: &Option<String>,
    ) -> ClientResult<()> {
        let mut path = cache_path(&self.root, uri, "json");
        //println!("caching body at path: {}", path.display());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, body)?;
        path.set_extension("etag");
        fs::write(&path, etag)?;
        if let Some(next_link) = next_link {
            path.set_extension("next_link");
            fs::write(&path, next_link)?;
        }
        Ok(())
    }

    fn lookup_etag(&self, uri: &str) -> ClientResult<String> {
        read_to_string(cache_path(&self.root, uri, "etag"))
    }

    fn lookup_body(&self, uri: &str) -> ClientResult<String> {
        read_to_string(cache_path(&self.root, uri, "json"))
    }

    fn lookup_next_link(&self, uri: &str) -> ClientResult<Option<String>> {
        let path = cache_path(&self.root, uri, "next_link");
        if path.exists() {
            Ok(Some(read_to_string(path)?))
        } else {
            Ok(None)
        }
    }
}

/// Construct the cache path for the given URI and extension, from an initial directory.
///
/// # Examples
///
/// ```
/// # use std::path::PathBuf;
/// # use octorust::http_cache::cache_path;
/// assert_eq!(
///     cache_path(
///         &PathBuf::from("/home/.github/cache"),
///         "https://api.github.com/users/dwijnand/repos",
///         "json"
///     ),
///     PathBuf::from("/home/.github/cache/v1/https/api.github.com/users/dwijnand/repos.json"),
/// );
/// assert_eq!(
///     cache_path(
///         &PathBuf::from("/home/.github/cache"),
///         "https://api.github.com/users/dwijnand/repos?page=2",
///         "json"
///     ),
///     PathBuf::from(
///         "/home/.github/cache/v1/https/api.github.com/users/dwijnand/repos/6dd58bde8abb0869.\
///          json"
///     ),
/// );
/// assert_eq!(
///     cache_path(
///         &PathBuf::from("/home/.github/cache"),
///         "https://api.github.com/users/dwijnand/repos?page=2&per_page=5",
///         "json"
///     ),
///     PathBuf::from(
///         "/home/.github/cache/v1/https/api.github.com/users/dwijnand/repos/d862dcd2d85cebca.\
///          json"
///     ),
/// );
/// ```
#[doc(hidden)] // public for doc testing only
pub fn cache_path<S: AsRef<OsStr>>(dir: &Path, uri: &str, extension: S) -> PathBuf {
    let uri_encoded = uri.replace(" ", "%20");
    let uri = uri_encoded
        .parse::<Uri>()
        .unwrap_or_else(|_| panic!("Expected a URI, got {}", uri_encoded));
    let parts = uri.clone().into_parts();
    let mut path = dir.to_path_buf();
    path.push("v1");
    path.push(parts.scheme.expect("no URI scheme").as_str()); // https
    path.push(parts.authority.expect("no URI authority").as_str()); // api.github.com
    path.push(Path::new(&uri.path()[1..])); // users/dwijnand/repos
    if let Some(query) = uri.query() {
        path.push(hash1(query, DefaultHasher::new())); // fa269019d5035d5f
    }
    path.set_extension(extension); // .json
    path
}

fn read_to_string<P: AsRef<Path>>(path: P) -> ClientResult<String> {
    //println!("reading path: {}", path.as_ref().display());
    Ok(fs::read_to_string(path)?)
}

fn no_read<T, E: Into<Box<dyn std::error::Error + Send + Sync>>>(error: E) -> ClientResult<T> {
    Err(ClientError::IoError(io::Error::new(
        io::ErrorKind::NotFound,
        error,
    )))
}

// Separate to provide a blanket implementation for `T: HttpCache + Clone`
// https://stackoverflow.com/a/30353928/463761
#[doc(hidden)]
pub trait HttpCacheClone {
    #[doc(hidden)]
    fn box_clone(&self) -> BoxedHttpCache;
}

impl<T> HttpCacheClone for T
where
    T: 'static + HttpCache + Clone + Send + Sync,
{
    fn box_clone(&self) -> BoxedHttpCache {
        Box::new(self.clone())
    }
}

fn hash1<A: Hash, H: Hasher>(x: A, mut hasher: H) -> String {
    x.hash(&mut hasher);
    u64_to_padded_hex(hasher.finish())
}

/// Construct a 0-padded hex string from a u64.
///
/// # Examples
///
/// ```
/// # use octorust::http_cache::u64_to_padded_hex;
/// assert_eq!(u64_to_padded_hex(0), "0000000000000000");
/// assert_eq!(u64_to_padded_hex(u64::max_value()), "ffffffffffffffff");
/// ```
#[doc(hidden)] // public for doc testing only
pub fn u64_to_padded_hex(x: u64) -> String {
    format!("{:016x}", x)
}

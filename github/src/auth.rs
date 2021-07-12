//! For performing functions related to authentication for the API.
use std::{
    fmt,
    sync::{Arc, Mutex},
    time,
};

use anyhow::Result;
use jsonwebtoken as jwt;
use serde::Serialize;

// We use 9 minutes for the life to give some buffer for clock drift between
// our clock and GitHub's. The absolute max is 10 minutes.
const MAX_JWT_TOKEN_LIFE: time::Duration = time::Duration::from_secs(60 * 9);
// 8 minutes so we refresh sooner than it actually expires
const JWT_TOKEN_REFRESH_PERIOD: time::Duration = time::Duration::from_secs(60 * 8);

/// Controls what sort of authentication is required for this request.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AuthenticationConstraint {
    /// No constraint
    Unconstrained,
    /// Must be JWT
    JWT,
}

/// Various forms of authentication credentials supported by GitHub.
#[derive(PartialEq, Clone)]
pub enum Credentials {
    /// Oauth token string
    /// https://developer.github.com/v3/#oauth2-token-sent-in-a-header
    Token(String),
    /// Oauth client id and secret
    /// https://developer.github.com/v3/#oauth2-keysecret
    Client(String, String),
    /// JWT token exchange, to be performed transparently in the
    /// background. app-id, DER key-file.
    /// https://developer.github.com/apps/building-github-apps/authenticating-with-github-apps/
    JWT(JWTCredentials),
    /// JWT-based App Installation Token
    /// https://developer.github.com/apps/building-github-apps/authenticating-with-github-apps/
    InstallationToken(InstallationTokenGenerator),
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Credentials::Token(value) => f.debug_tuple("Credentials::Token").field(&"*".repeat(value.len())).finish(),
            Credentials::Client(id, secret) => f.debug_tuple("Credentials::Client").field(&id).field(&"*".repeat(secret.len())).finish(),
            Credentials::JWT(jwt) => f
                .debug_struct("Credentials::JWT")
                .field("app_id", &jwt.app_id)
                .field("private_key", &"vec![***]")
                .finish(),
            Credentials::InstallationToken(generator) => f
                .debug_struct("Credentials::InstallationToken")
                .field("installation_id", &generator.installation_id)
                .field("jwt_credential", &"***")
                .finish(),
        }
    }
}

/// JSON Web Token authentication mechanism.
///
/// The GitHub client methods are all &self, but the dynamically
/// generated JWT token changes regularly. The token is also a bit
/// expensive to regenerate, so we do want to have a mutable cache.
///
/// We use a token inside a Mutex so we can have interior mutability
/// even though JWTCredentials is not mutable.
#[derive(Clone)]
pub struct JWTCredentials {
    pub app_id: u64,
    /// DER RSA key. Generate with
    /// `openssl rsa -in private_rsa_key.pem -outform DER -out private_rsa_key.der`
    pub private_key: Vec<u8>,
    cache: Arc<Mutex<ExpiringJWTCredential>>,
}

impl JWTCredentials {
    pub fn new(app_id: u64, private_key: Vec<u8>) -> Result<JWTCredentials> {
        let creds = ExpiringJWTCredential::calculate(app_id, &private_key)?;

        Ok(JWTCredentials {
            app_id,
            private_key,
            cache: Arc::new(Mutex::new(creds)),
        })
    }

    fn is_stale(&self) -> bool {
        self.cache.lock().unwrap().is_stale()
    }

    /// Fetch a valid JWT token, regenerating it if necessary
    pub fn token(&self) -> String {
        let mut expiring = self.cache.lock().unwrap();
        if expiring.is_stale() {
            *expiring =
                ExpiringJWTCredential::calculate(self.app_id, &self.private_key).expect("JWT private key worked before, it should work now...");
        }

        expiring.token.clone()
    }
}

impl PartialEq for JWTCredentials {
    fn eq(&self, other: &JWTCredentials) -> bool {
        self.app_id == other.app_id && self.private_key == other.private_key
    }
}

#[derive(Debug)]
struct ExpiringJWTCredential {
    token: String,
    created_at: time::Instant,
}

#[derive(Serialize)]
struct JWTCredentialClaim {
    iat: u64,
    exp: u64,
    iss: u64,
}

impl ExpiringJWTCredential {
    fn calculate(app_id: u64, private_key: &[u8]) -> Result<ExpiringJWTCredential> {
        // SystemTime can go backwards, Instant can't, so always use
        // Instant for ensuring regular cycling.
        let created_at = time::Instant::now();
        let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
        let expires = now + MAX_JWT_TOKEN_LIFE;

        let payload = JWTCredentialClaim {
            iat: now.as_secs(),
            exp: expires.as_secs(),
            iss: app_id,
        };
        let header = jwt::Header::new(jwt::Algorithm::RS256);
        let jwt = jwt::encode(&header, &payload, &jsonwebtoken::EncodingKey::from_rsa_der(private_key))?;

        Ok(ExpiringJWTCredential { created_at, token: jwt })
    }

    fn is_stale(&self) -> bool {
        self.created_at.elapsed() >= JWT_TOKEN_REFRESH_PERIOD
    }
}

/// A caching token "generator" which contains JWT credentials.
///
/// The authentication mechanism in the GitHub client library
/// determines if the token is stale, and if so, uses the contained
/// JWT credentials to fetch a new installation token.
///
/// The Mutex<Option> access key is for interior mutability.
#[derive(Debug, Clone)]
pub struct InstallationTokenGenerator {
    pub installation_id: u64,
    pub jwt_credential: Box<Credentials>,
    pub access_key: Arc<Mutex<Option<String>>>,
}

impl InstallationTokenGenerator {
    pub fn new(installation_id: u64, creds: JWTCredentials) -> InstallationTokenGenerator {
        InstallationTokenGenerator {
            installation_id,
            jwt_credential: Box::new(Credentials::JWT(creds)),
            access_key: Arc::new(Mutex::new(None)),
        }
    }

    pub fn token(&self) -> Option<String> {
        if let Credentials::JWT(ref creds) = *self.jwt_credential {
            if creds.is_stale() {
                return None;
            }
        }
        self.access_key.lock().unwrap().clone()
    }

    pub fn jwt(&self) -> &Credentials {
        &*self.jwt_credential
    }
}

impl PartialEq for InstallationTokenGenerator {
    fn eq(&self, other: &InstallationTokenGenerator) -> bool {
        self.installation_id == other.installation_id && self.jwt_credential == other.jwt_credential
    }
}

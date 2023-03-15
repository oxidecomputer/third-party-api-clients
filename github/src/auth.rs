//! For performing functions related to authentication for the API.
use std::{
    fmt,
    sync::{Arc, Mutex},
    time,
};

use crate::ClientResult;
use jsonwebtoken as jwt;
use serde::Serialize;
use tokio::sync::RwLock;

// We use 9 minutes for the life to give some buffer for clock drift between
// our clock and GitHub's. The absolute max is 10 minutes.
const MAX_JWT_TOKEN_LIFE: time::Duration = time::Duration::from_secs(60 * 9);
// 8 minutes so we refresh sooner than it actually expires
const JWT_TOKEN_REFRESH_PERIOD: time::Duration = time::Duration::from_secs(60 * 8);

// Installation tokens are valid for one hour.
// We'll refresh sooner to avoid problems with clock drift.
const INSTALLATION_TOKEN_REFRESH_PERIOD: time::Duration = time::Duration::from_secs(60 * 58);

/// Controls what sort of authentication is required for this request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
            Credentials::Token(value) => f
                .debug_tuple("Credentials::Token")
                .field(&"*".repeat(value.len()))
                .finish(),
            Credentials::Client(id, secret) => f
                .debug_tuple("Credentials::Client")
                .field(&id)
                .field(&"*".repeat(secret.len()))
                .finish(),
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
    pub app_id: i64,
    /// DER RSA key. Generate with
    /// `openssl rsa -in private_rsa_key.pem -outform DER -out private_rsa_key.der`
    pub private_key: Vec<u8>,
    cache: Arc<Mutex<ExpiringJWTCredential>>,
}

impl JWTCredentials {
    pub fn new(app_id: i64, private_key: Vec<u8>) -> ClientResult<JWTCredentials> {
        let creds = ExpiringJWTCredential::calculate(app_id, &private_key)?;

        Ok(JWTCredentials {
            app_id,
            private_key,
            cache: Arc::new(Mutex::new(creds)),
        })
    }

    /// Fetch a valid JWT token, regenerating it if necessary
    pub fn token(&self) -> String {
        let mut expiring = self.cache.lock().unwrap();
        if expiring.is_stale() {
            *expiring = ExpiringJWTCredential::calculate(self.app_id, &self.private_key)
                .expect("JWT private key worked before, it should work now...");
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
    created_at: tokio::time::Instant,
}

#[derive(Serialize)]
struct JWTCredentialClaim {
    iat: u64,
    exp: u64,
    iss: i64,
}

impl ExpiringJWTCredential {
    fn calculate(app_id: i64, private_key: &[u8]) -> ClientResult<ExpiringJWTCredential> {
        // SystemTime can go backwards, Instant can't, so always use
        // Instant for ensuring regular cycling.
        let created_at = tokio::time::Instant::now();
        let now = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap();
        let expires = now + MAX_JWT_TOKEN_LIFE;

        let payload = JWTCredentialClaim {
            // GitHub recommends specifying this as 60 seconds in the past to avoid problems with clock drift.
            iat: now.as_secs().saturating_sub(60),
            exp: expires.as_secs(),
            iss: app_id,
        };
        let header = jwt::Header::new(jwt::Algorithm::RS256);
        let jwt = jwt::encode(
            &header,
            &payload,
            &jsonwebtoken::EncodingKey::from_rsa_der(private_key),
        )?;

        Ok(ExpiringJWTCredential {
            created_at,
            token: jwt,
        })
    }

    fn is_stale(&self) -> bool {
        self.created_at.elapsed() >= JWT_TOKEN_REFRESH_PERIOD
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExpiringInstallationToken {
    token: String,
    created_at: tokio::time::Instant,
}

impl ExpiringInstallationToken {
    pub fn new(token: String, created_at: tokio::time::Instant) -> Self {
        Self { token, created_at }
    }

    pub fn token(&self) -> Option<&str> {
        if self.created_at.elapsed() < INSTALLATION_TOKEN_REFRESH_PERIOD {
            Some(&self.token)
        } else {
            None
        }
    }
}

/// A caching token "generator" which contains JWT credentials.
///
/// The authentication mechanism in the GitHub client library
/// determines if the token is stale, and if so, uses the contained
/// JWT credentials to fetch a new installation token.
///
/// The RwLock<Option> access key is for interior mutability.
#[derive(Debug, Clone)]
pub struct InstallationTokenGenerator {
    pub installation_id: i64,
    pub jwt_credential: Box<Credentials>,
    pub(crate) access_key: Arc<RwLock<Option<ExpiringInstallationToken>>>,
}

impl InstallationTokenGenerator {
    pub fn new(installation_id: i64, creds: JWTCredentials) -> InstallationTokenGenerator {
        InstallationTokenGenerator {
            installation_id,
            jwt_credential: Box::new(Credentials::JWT(creds)),
            access_key: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn token(&self) -> Option<String> {
        self.access_key
            .read()
            .await
            .as_ref()
            .and_then(|t| t.token())
            .map(|t| t.to_owned())
    }

    pub fn jwt(&self) -> &Credentials {
        &self.jwt_credential
    }
}

impl PartialEq for InstallationTokenGenerator {
    fn eq(&self, other: &InstallationTokenGenerator) -> bool {
        self.installation_id == other.installation_id && self.jwt_credential == other.jwt_credential
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;
    use rsa::{pkcs1::EncodeRsaPrivateKey, RsaPrivateKey};
    use std::time::Duration;

    fn app_id() -> i64 {
        let mut rng = rand::thread_rng();
        rng.next_u32() as i64
    }

    fn installation_id() -> i64 {
        let mut rng = rand::thread_rng();
        rng.next_u32() as i64
    }

    fn private_key() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048)
            .unwrap()
            .to_pkcs1_der()
            .unwrap()
            .to_bytes();
        private_key.to_vec()
    }

    #[tokio::test(start_paused = true)]
    async fn jwt_credentials_refreshes_when_necessary() {
        let credentials = JWTCredentials::new(app_id(), private_key())
            .expect("Should be able to create credentials");

        assert!(
            !credentials.cache.lock().unwrap().is_stale(),
            "Credentials should not be initially stale",
        );
        let initial_token = credentials.token();

        // Sleep 1 real second so the token content changes on refresh.
        std::thread::sleep(Duration::from_secs(1));
        // Sleep fake time to make the current token stale.
        tokio::time::advance(JWT_TOKEN_REFRESH_PERIOD).await;

        assert!(
            credentials.cache.lock().unwrap().is_stale(),
            "Credentials should be stale after refresh period",
        );

        let new_token = credentials.token();
        assert_ne!(initial_token, new_token, "New token should be generated");
    }

    #[tokio::test(start_paused = true)]
    async fn installation_token_generator_expires_after_token_interval() {
        let jwt_credentials = JWTCredentials::new(app_id(), private_key())
            .expect("Should be able to create JWT credentials");

        let generator = InstallationTokenGenerator::new(installation_id(), jwt_credentials.clone());

        assert_eq!(
            None,
            generator.token().await,
            "Generator should initially have no token",
        );
        *generator.access_key.write().await = Some(ExpiringInstallationToken::new(
            "initial token".to_owned(),
            tokio::time::Instant::now(),
        ));
        assert_eq!(
            Some("initial token"),
            generator.token().await.as_deref(),
            "Generator should have token after set",
        );

        // Sleep fake time to make the current token stale.
        tokio::time::advance(INSTALLATION_TOKEN_REFRESH_PERIOD).await;

        assert_eq!(
            None,
            generator.token().await,
            "Generator token should expire after interval",
        );
    }
}

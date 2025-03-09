/// structs and methods for oauth2 authentication flow
pub mod redirect;

/// structs and methods for token management
pub mod token;

/// methods for cache
pub mod cache;

use crate::config::oauth_config::AuthConfig;
use color_eyre::Result;
use rand::{distr::Alphanumeric, rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_urlencoded;
use std::{io::Error, iter, str::FromStr}; // process::Output
use token::{Token, TokenWrapper};
use url::Url;

const USER_AGENT: &str = "mal-tui";
const AUTHORIZE_URL: &str = "https://myanimelist.net/v1/oauth2/authorize";
const TOKEN_URL: &str = "https://myanimelist.net/v1/oauth2/token";

#[derive(Clone, Debug)]
pub enum AuthError {
    UnknownError,
    NetworkTimeout,
    InvalidResponse(String),
    AuthNotPresent,
    TokenNotPresent,
}

impl From<reqwest::Error> for AuthError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            AuthError::NetworkTimeout
        } else {
            AuthError::UnknownError
        }
    }
}

impl std::error::Error for AuthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            AuthError::UnknownError => None,
            AuthError::NetworkTimeout => None,
            AuthError::InvalidResponse(_) => None,
            AuthError::AuthNotPresent => None,
            AuthError::TokenNotPresent => None,
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            AuthError::UnknownError => write!(f, "Unknown Error"),
            AuthError::NetworkTimeout => write!(f, "Network Timeout"),
            AuthError::InvalidResponse(ref err) => err.fmt(f),
            AuthError::AuthNotPresent => write!(f, "Auth is not present"),
            AuthError::TokenNotPresent => write!(f, "Token is not present"),
        }
    }
}

const CODE_CHALLENGE_LENGTH: usize = 128;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub redirect_url: String,
    pub user_agent: String,
    pub challenge: String,
    pub state: String,
    pub auth_code: Option<String>,
    pub token: Option<TokenWrapper>,
}

impl OAuth {
    /// Start of a new oauth2 flow
    /// # Parameters
    /// * `user`
    pub fn new<A: ToString>(
        user_agent: A,
        client_id: A,
        client_secret: Option<A>,
        redirect_url: A,
    ) -> Self {
        OAuth {
            client_id: client_id.to_string(),
            client_secret: if let Some(cs) = client_secret {
                Some(cs.to_string())
            } else {
                None
            },
            redirect_url: redirect_url.to_string(),
            user_agent: user_agent.to_string(),
            challenge: Self::new_challenge(CODE_CHALLENGE_LENGTH),
            state: "AUTHSTART".to_string(),
            auth_code: None,
            token: None,
        }
    }

    /// Generates a new base64-encoded SHA-256 PKCE code
    /// # Panic
    /// `len` needs to be a value between 48 and 128
    fn new_challenge(len: usize) -> String {
        // Check whether the len in in between the valid length for a
        // PKCE code (43 chars - 128 chars)
        if len < 48 || len > 128 {
            panic!("len is not in between 48 and 128");
        }
        let mut rng = rng();
        // needs to be url safe so we use Alphanumeric
        let challenge: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric) as char)
            .take(len)
            .collect();
        challenge
    }

    /// Returns user agent
    pub fn user_agent(&self) -> &String {
        &self.user_agent
    }

    /// Creates a new authorization url
    pub fn get_auth_url(&self) -> Url {
        #[derive(Serialize, Debug)]
        struct AuthQuery {
            response_type: String,
            client_id: String,
            code_challenge: String,
            state: String,
            redirect_url: String,
            code_challenge_method: String,
        }

        let auth_query = AuthQuery {
            response_type: "code".to_string(),
            client_id: self.client_id.clone(),
            code_challenge: self.challenge.clone(),
            state: self.state.to_string(),
            redirect_url: self.redirect_url.clone(),
            // mal only supports plain
            code_challenge_method: "plain".to_string(),
        };

        url::Url::from_str(&format!(
            "{}?{}",
            AUTHORIZE_URL,
            serde_urlencoded::to_string(auth_query).unwrap()
        ))
        .unwrap()
    }

    /// Parses redirection url
    pub fn parse_redirect_query_string(&mut self, query_string: &str) -> Result<(), AuthError> {
        #[derive(Deserialize, Debug)]
        struct AuthResponse {
            code: String,
            state: String,
        }

        let auth_response = match serde_urlencoded::from_str::<AuthResponse>(query_string) {
            Ok(r) => r,
            Err(e) => {
                return Err(AuthError::InvalidResponse(e.to_string()));
            }
        };

        if auth_response.state != self.state {
            return Err(AuthError::InvalidResponse("State Mismatch".to_string()));
        }

        self.auth_code = Some(auth_response.code);
        Ok(())
    }

    /// Creates a new url to get the token
    pub fn get_token_query_string(&self) -> Result<String, AuthError> {
        #[derive(Serialize, Debug)]
        struct TokenRequest {
            client_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_secret: Option<String>,
            code: String,
            code_verifier: String,
            grant_type: String,
        }

        if self.auth_code.is_none() {
            return Err(AuthError::AuthNotPresent);
        }

        let query = TokenRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code: self.auth_code.as_ref().unwrap().clone(),
            code_verifier: self.challenge.clone(),
            grant_type: "authorization_code".to_string(),
        };

        Ok(serde_urlencoded::to_string(query).unwrap())
    }

    /// Get access token
    pub fn get_access_token(&mut self) -> Result<(), AuthError> {
        let request = reqwest::blocking::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()?
            .post(TOKEN_URL)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(self.get_token_query_string()?);

        let response = request.send()?;
        let success = response.status().is_success();
        let body = response.text()?;
        self.handle_response(success, &body)
    }

    /// Refresh the token (async)
    pub async fn get_access_token_async(&mut self) -> Result<(), AuthError> {
        let request = reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()?
            .post(TOKEN_URL)
            .header(reqwest::header::ACCEPT, "application/json")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(self.get_token_query_string()?);

        let response = request.send().await?;
        let success = response.status().is_success();
        let body = response.text().await?;
        self.handle_response(success, &body)
    }

    /// Handle a repsonse for get_access_token()
    pub fn handle_response(&mut self, success: bool, body: &str) -> Result<(), AuthError> {
        if success {
            match serde_json::from_str::<Token>(body) {
                Ok(result) => {
                    self.token = Some(TokenWrapper::new(result));
                    Ok(())
                }
                Err(e) => Err(AuthError::InvalidResponse(e.to_string())),
            }
        } else {
            println!("{}", body);
            Err(AuthError::UnknownError)
        }
    }

    /// Get a token reference
    pub fn token(&self) -> Option<&TokenWrapper> {
        self.token.as_ref()
    }

    pub fn get_token_refresh_query_string(&self) -> Result<String, AuthError> {
        #[derive(Serialize, Debug)]
        struct TokenRequest {
            client_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_secret: Option<String>,
            code: String,
            code_verifier: String,
            grant_type: String,
            refresh_token: String,
        }

        if self.auth_code.is_none() {
            return Err(AuthError::AuthNotPresent);
        }
        if self.token.is_none() {
            return Err(AuthError::TokenNotPresent);
        }

        let query = TokenRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code: self.auth_code.as_ref().unwrap().clone(),
            code_verifier: self.challenge.clone(),
            grant_type: "refresh_token".to_string(),
            refresh_token: self.token().unwrap().token.refresh_token.clone(),
        };

        Ok(serde_urlencoded::to_string(query).unwrap())
    }

    /// Refresh the token
    pub fn refresh(&mut self) -> Result<(), AuthError> {
        if self.token().unwrap().expired() {
            let request = reqwest::blocking::ClientBuilder::new()
                .user_agent(USER_AGENT)
                .build()?
                .post(TOKEN_URL)
                .header(reqwest::header::ACCEPT, "application/json")
                .header(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(self.get_token_refresh_query_string()?);

            let response = request.send()?;
            let success = response.status().is_success();
            let body = response.text()?;
            self.handle_response(success, &body)
        } else {
            Ok(())
        }
    }

    /// Refresh the token (async)
    pub async fn refresh_async(&mut self) -> Result<(), AuthError> {
        if self.token().unwrap().expired() {
            let request = reqwest::ClientBuilder::new()
                .user_agent(USER_AGENT)
                .build()?
                .post(TOKEN_URL)
                .header(reqwest::header::ACCEPT, "application/json")
                .header(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(self.get_token_refresh_query_string()?);

            let response = request.send().await?;
            let success = response.status().is_success();
            let body = response.text().await?;
            self.handle_response(success, &body)
        } else {
            Ok(())
        }
    }

    pub async fn get_auth_async(config: AuthConfig) -> Result<OAuth, AuthError> {
        if let Some(mut auth) = cache::load_cached_auth() {
            auth.refresh_async().await?;
            Ok(auth)
        } else {
            let auth = OAuth::new(
                config.get_user_agent(),
                config.client_id.clone(),
                None,
                config.get_redirect_uri(),
            );

            let url = auth.get_auth_url();
            open(url).unwrap();

            let mut auth = redirect::Server::new(config.get_user_agent(), auth)
                .go()
                .unwrap();

            auth.get_access_token_async().await.unwrap();

            cache::cache_auth(&auth);

            Ok(auth)
        }
    }
    pub fn get_auth(config: AuthConfig) -> Result<OAuth, AuthError> {
        if let Some(mut auth) = cache::load_cached_auth() {
            auth.refresh()?;
            Ok(auth)
        } else {
            let auth = OAuth::new(
                config.get_user_agent(),
                config.client_id.clone(),
                None,
                config.get_redirect_uri(),
            );

            let url = auth.get_auth_url();
            open(url).unwrap();

            let mut auth = redirect::Server::new(config.get_user_agent(), auth)
                .go()
                .unwrap();

            auth.get_access_token().unwrap();

            cache::cache_auth(&auth);

            Ok(auth)
        }
    }
}

/// use webbrowser crate to open url in browser
pub fn open(url: Url) -> Result<(), Error> {
    webbrowser::open(&url.to_string())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub fn get_auth() -> OAuth {
        let config = AuthConfig::load().unwrap();
        let auth = OAuth::get_auth(config).unwrap();
        auth
    }
    #[test]
    fn test_get_auth() {
        // Get config from file
        let config = AuthConfig::load().unwrap();

        // make auth
        let auth = OAuth::new(
            config.get_user_agent(),
            config.client_id.clone(),
            None,
            config.get_redirect_uri(),
        );

        println!("{}", auth.redirect_url);

        // create and open url
        let url = auth.get_auth_url();
        open(url).unwrap();

        // wait for redirect
        let mut auth = redirect::Server::new(config.get_user_agent(), auth)
            .go()
            .unwrap();

        // get access token
        auth.get_access_token().unwrap();
        println!("{}", serde_json::to_string(&auth).unwrap());

        // get refresh token
        auth.refresh().unwrap();
        println!("{}", serde_json::to_string(&auth).unwrap());

        cache::cache_auth(&auth);
    }

    #[test]
    fn test_challenge() {
        let challenge = OAuth::new_challenge(CODE_CHALLENGE_LENGTH);

        assert!(challenge.len() == CODE_CHALLENGE_LENGTH);
        println!("{}", challenge);
        println!(
            "len: {}, CODE_CHALLENGE_LEN: {}",
            challenge.len(),
            CODE_CHALLENGE_LENGTH
        );
    }
    #[test]
    #[should_panic(expected = "len is not in between 48 and 128")]
    fn test_challenge_len() {
        // should panic
        let _challenge = OAuth::new_challenge(5);
    }
}

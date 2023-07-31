use crate::error;
use tracing::info;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl,
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use url::Url;

#[derive(Debug, Clone)]
pub struct AuthReq {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    redirect_url: String,
}

impl AuthReq {
    pub fn new() -> AuthReqBuilder {
        AuthReqBuilder::new()
    }

    pub async fn req(&self) -> Result<(), error::ArcError> {
        let client = 
            BasicClient::new(
                ClientId::new(self.client_id.to_owned()),
                Some(ClientSecret::new(self.client_secret.to_owned())),
                AuthUrl::new(self.auth_url.to_owned())?,
                Some(TokenUrl::new(self.token_url.to_owned())?)
                            )
            .set_redirect_uri(RedirectUrl::new(self.redirect_url.to_owned())?);
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read".to_string()))
            .add_scope(Scope::new("write".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();
        info!("Browse to: {}", auth_url);
        let mut code = String::new();
        info!("Enter code:");
        std::io::stdin().read_line(&mut code).expect("Failed to read input.");
        // let token_result = client
        //     .exchange_code(AuthorizationCode::new(code))
        //     .set_pkce_verifier(pkce_verifier)
        //     .request_async(async_http_client)
        //     .await?;
        // info!("Token result: {:#?}", token_result);
        let mut body = "".to_string();
        body.push_str(&format!("client_id={}", self.client_id));
        // body.push_str(&format!("&client_secret={}", self.client_secret));
        body.push_str(&format!("&grant_type=authorization_code"));
        body.push_str(&format!("&redirect_uri={}", self.redirect_url.clone()));
        body.push_str(&format!("&code={}", code));
        body.push_str(&format!("&code_verifier={}", pkce_verifier.secret()));
        let client = reqwest::Client::new();
        let res = client
            .post(self.token_url.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(reqwest::header::HeaderName::from_static("client_id"), self.client_id.clone())
            // .header(reqwest::header::HeaderName::from_static("grant_type"), "authorization_code" )
            // .header(reqwest::header::HeaderName::from_static("redirect_uri"), self.redirect_url.clone())
            .body(body)
            .send()
            .await?;
        info!("Status: {}", res.status());
        info!("Response: {}", res.text().await?);

        Ok(())

    }
}

#[derive(Debug, Default, Clone)]
pub struct AuthReqBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    auth_url: Option<String>,
    token_url: Option<String>,
    redirect_url: Option<String>,
}

impl AuthReqBuilder {
    pub fn new() -> Self {
        AuthReqBuilder::default()
    }

    pub fn client_id(&mut self, value: &str) -> Self {
        self.client_id = Some(value.to_owned());
        self.clone()
    }

    pub fn client_secret(&mut self, value: &str) -> Self {
        self.client_secret = Some(value.to_owned());
        self.clone()
    }

    pub fn auth_url(&mut self, value: &str) -> Self {
        self.auth_url = Some(value.to_owned());
        self.clone()
    }

    pub fn token_url(&mut self, value: &str) -> Self {
        self.token_url = Some(value.to_owned());
        self.clone()
    }

    pub fn redirect_url(&mut self, value: &str) -> Self {
        self.redirect_url = Some(value.to_owned());
        self.clone()
    }

    pub fn build(self) -> Result<AuthReq, error::ArcError> {
        let mut client_id = "".to_string();
        let mut client_secret = "".to_string();
        let mut auth_url = "".to_string();
        let mut token_url = "".to_string();
        let mut redirect_url = "".to_string();

        let mut errors = Vec::new();

        match self.client_id {
            Some(value) => client_id = value,
            None => errors.push("client_id".to_string()),
        }

        match self.client_secret {
            Some(value) => client_secret = value,
            None => errors.push("client_secret".to_string()),
        }

        match self.auth_url {
            Some(value) => auth_url = value,
            None => errors.push("auth_url".to_string()),
        }

        match self.token_url {
            Some(value) => token_url = value,
            None => errors.push("token_url".to_string()),
        }

        match self.redirect_url {
            Some(value) => redirect_url = value,
            None => errors.push("redirect_url".to_string()),
        }

        if errors.is_empty() {
            Ok(AuthReq {
                client_id,
                client_secret,
                auth_url,
                token_url,
                redirect_url,
            })
        } else {
            Err(error::ArcError::UserBuildError { value: errors })
        }

    }

}




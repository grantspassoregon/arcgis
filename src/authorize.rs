use crate::error;
use serde::Deserialize;
use serde_json::json;
use tracing::info;

#[derive(Debug, Default, Clone)]
pub enum ClientType {
    Referer(String),
    Ip(String),
    #[default]
    RequestIp
}

#[derive(Debug, Default, Clone, Copy)]
pub enum FormatOptions {
    #[default]
    Html,
    Json,
    Pjson,
}

#[derive(Debug, Clone)]
pub struct AuthRequest {
    username: String,
    password: String,
    client: ClientType,
    referer: Option<String>,
    ip: Option<String>,
    expiration: Option<i32>,
    f: FormatOptions,
    host: String,
}

impl AuthRequest {
    pub fn new(username: &str, password: &str, host: &str) -> Self {
        AuthRequest {
            username: username.to_owned(),
            password: password.to_owned(),
            client: ClientType::default(),
            referer: None,
            ip: None,
            expiration: None,
            f: FormatOptions::default(),
            host: host.to_owned(),
        }
    }

    pub fn set_client(&mut self, value: &ClientType) {
        self.client = value.clone();
    }

    pub fn get_client(self) -> String {
        match self.client {
            ClientType::Ip(value) => value,
            ClientType::Referer(value) => value,
            ClientType::RequestIp => "requestip".to_string(),
        }
    }

    pub fn referer(&mut self, value: &str) {
        self.referer = Some(value.to_owned());
    }

    pub fn ip(&mut self, value: &str) {
        self.ip = Some(value.to_owned());
    }

    pub fn expiration(&mut self, value: i32) {
        self.expiration = Some(value);
    }

    pub fn set_f(&mut self, value: &FormatOptions) {
        self.f = value.clone();
    }

    pub fn get_f(self) -> String {
        match self.f {
            FormatOptions::Html => "html".to_owned(),
            FormatOptions::Json => "json".to_owned(),
            FormatOptions::Pjson => "pjson".to_owned(),
        }
    }

    pub fn body(&self) -> String {
        let mut res = format!("username={}@{}", self.username, self.host);
        res.push_str(&format!("&password={}", self.password));
        res.push_str(&format!("&client={}", self.clone().get_client()));
        if let Some(value) = self.referer.clone() {
            res.push_str(&format!("&referer={}", value));
        }
        if let Some(value) = self.ip.clone() {
            res.push_str(&format!("&ip={}", value));
        }
        if let Some(value) = self.expiration.clone() {
            res.push_str(&format!("&expiration={}", value));
        }
        res.push_str(&format!("&f={}", self.clone().get_f()));
        res
    }

    pub async fn authorize(&self, url: &str) -> Result<AuthResponse, error::ArcError> {
        let client = reqwest::Client::new();
        info!("Client created.");
        info!("Body: {}", self.body());
        let res = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            // .header(reqwest::header::CONTENT_LENGTH, "[]")
            // .header(reqwest::header::HOST, format!("gisserver.{}", self.host))
            .body(self.body())
            .send()
            .await?;
        let res1 = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            // .header(reqwest::header::CONTENT_LENGTH, "[]")
            // .header(reqwest::header::HOST, format!("gisserver.{}", self.host))
            .body(self.body())
            .send()
            .await?;
        info!("Status: {}", res.status());
        info!("Response: {}", res.text().await?);
        // match &res.status() {
        //     &reqwest::StatusCode::OK => Ok(res.json::<AuthResponse>().await?),
        //     _ => Err(error::ArcError::AuthError),
        // }
        Ok(res1.json::<AuthResponse>().await?)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct AuthResponse {
    token: String,
    expires: String,
}

pub fn health_check() {
    info!("Authorize module.")
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArcError {
    #[error("Value not provided for {value:?}.")]
    UserBuildError { value: Vec<String> },
    #[error("HTTP request error.")]
    HttpError(#[from] reqwest::Error),
    #[error("Input/output error from std.")]
    Io(#[from] std::io::Error),
    #[error("Could not read environmental variables from .env.")]
    EnvError(#[from] std::env::VarError),
    #[error("Authorization failed.")]
    AuthError,
    #[error("Bad file name {0:?}.")]
    FileNameError(std::ffi::OsString),
    #[error("Parse error.")]
    ParseError(#[from] url::ParseError),
    #[error("Oauth2 error.")]
    Oauth2Error(#[from] oauth2::RequestTokenError<oauth2::reqwest::Error<reqwest::Error>,
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>)
}

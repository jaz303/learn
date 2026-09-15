use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct State {
    pub entity_id: String,
    pub state: String,
}

#[derive(Debug)]
pub enum Error {
    Transport(reqwest::Error), // network failure
    Auth(),
    UnexpectedHttpStatus(StatusCode), // received unexpected HTTP status
    Json(reqwest::Error),             // JSON deserialization failed
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::Transport(err) => {
                write!(f, "transport error: {}", err)
            }
            Error::Auth() => {
                write!(f, "auth error")
            }
            Error::UnexpectedHttpStatus(status) => {
                write!(f, "unexpected status: {}", status)
            }
            Error::Json(err) => {
                write!(f, "JSON error: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {}

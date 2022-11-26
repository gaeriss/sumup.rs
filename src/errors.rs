pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    Api(Response),
    #[error("{0}")]
    Auth(&'static str),
    #[error("Invalid scope: {0}")]
    InvalidScope(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        error
            .into_response()
            .unwrap()
            .into_json()
            .map(Self::Api)
            .unwrap_or_else(Self::from)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Response {
    Message(Message),
    Messages(Vec<Message>),
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    message: String,
    instance: Option<String>,
    error_code: String,
    param: Option<String>,
    #[serde(rename = "type")]
    ty: Option<String>,
    title: Option<String>,
    status: Option<u8>,
    detail: Option<String>,
}

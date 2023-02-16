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
            .map_or_else(Self::from, Self::Api)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Response {
    Message(Box<Message>),
    Messages(Vec<Message>),
}

#[derive(Debug, serde::Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Message {
    #[serde(alias = "error_message")]
    pub message: String,
    pub instance: Option<String>,
    pub error_code: Option<String>,
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub title: Option<String>,
    pub status: Option<u8>,
    pub detail: Option<String>,
}

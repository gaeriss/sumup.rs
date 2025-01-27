pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    Api(Response),
    #[error("{0}")]
    Auth(&'static str),
    #[error("{0:?}")]
    Http(#[from] reqwest::Error),
    #[error("Invalid scope: {0}")]
    InvalidScope(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Response {
    Message(Box<Message>),
    Messages(Vec<Message>),
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Message {
    Short {
        #[serde(alias = "error")]
        message: String,
        #[serde(alias = "error_description")]
        description: String,
    },
    Long {
        #[serde(alias = "error_message")]
        message: String,
        instance: Option<String>,
        error_code: Option<String>,
        param: Option<String>,
        #[serde(rename = "type")]
        ty: Option<String>,
        title: Option<String>,
        status: Option<u8>,
        detail: Option<String>,
    },
}

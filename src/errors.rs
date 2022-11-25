pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Api(#[from] Box<ureq::Error>),
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
        Self::Api(Box::new(error))
    }
}

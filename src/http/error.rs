use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ApiError {
    Http(reqwest::Error),
    Url(url::ParseError),
    Serialize(serde_json::Error),
    UnexpectedStatus {
        status: reqwest::StatusCode,
        body: String,
    },
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => write!(f, "HTTP 请求失败: {error}"),
            Self::Url(error) => write!(f, "URL 解析失败: {error}"),
            Self::Serialize(error) => write!(f, "序列化失败: {error}"),
            Self::UnexpectedStatus { status, body } => {
                write!(f, "接口返回异常状态 {status}: {body}")
            }
        }
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            Self::Url(error) => Some(error),
            Self::Serialize(error) => Some(error),
            Self::UnexpectedStatus { .. } => None,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

impl From<url::ParseError> for ApiError {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialize(value)
    }
}

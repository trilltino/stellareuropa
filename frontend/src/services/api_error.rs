

#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    NetworkError(String),
    ParseError(String),
    HttpError {
        status: u16,
        message: String,
    },
    ValidationError(String),
    Timeout,
    Unknown(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            ApiError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            ApiError::HttpError { status, message } => {
                write!(f, "HTTP {status} error: {message}")
            }
            ApiError::ValidationError(msg) => write!(f, "Validation error: {msg}"),
            ApiError::Timeout => write!(f, "Request timed out"),
            ApiError::Unknown(msg) => write!(f, "Unknown error: {msg}"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<String> for ApiError {
    fn from(s: String) -> Self {
        ApiError::Unknown(s)
    }
}

impl From<&str> for ApiError {
    fn from(s: &str) -> Self {
        ApiError::Unknown(s.to_string())
    }
}

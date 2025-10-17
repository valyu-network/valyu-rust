use thiserror::Error;

/// Errors that can occur when using the Valyu SDK
#[derive(Error, Debug)]
pub enum ValyuError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error: {0}")]
    ApiError(String),

    /// Failed to parse API response
    #[error("Failed to parse API response: {0}")]
    ParseError(String),

    /// Invalid API key
    #[error("Invalid API key provided")]
    InvalidApiKey,

    /// Invalid request parameters
    #[error("Invalid request parameters: {0}")]
    InvalidRequest(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Service unavailable
    #[error("Service unavailable")]
    ServiceUnavailable,
}

/// Result type alias for Valyu SDK operations
pub type Result<T> = std::result::Result<T, ValyuError>;

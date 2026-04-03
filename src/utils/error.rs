use thiserror::Error;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("Adapter error: {0}")]
    Adapter(String),

    #[error("Routing error: no available model")]
    NoModelAvailable,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Unauthorized : {0}")]
    Unauthorized(String),

    #[error("Upstream HTTP error {status}: {body}")]
    UpstreamHttp { status: u16, body: String },

    #[error("Stream error: {0}")]
    Config(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl actix_web::ResponseError for GatewayError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        let body = serde_json::json!({"error": self.to_string()});
        match self {
            GatewayError::Unauthorized(_) => HttpResponse::Unauthorized().json(body),
            GatewayError::RateLimitExceeded => HttpResponse::TooManyRequests().json(body),
            GatewayError::NoModelAvailable => HttpResponse::ServiceUnavailable().json(body),
            _ => HttpResponse::InternalServerError().json(body),
        }
    }
}

use crate::utils::error::GatewayError;
use reqwest::{Client, Response};
use serde_json::Value;

pub struct OpenAiClient {
    http: Client,
    pub base_url: String,
    pub api_key: String,
}

impl OpenAiClient {
    pub fn new(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            http: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap(),
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }
    pub async fn chat_stream(&self, body: Value) -> Result<Response, GatewayError> {
        let resp = self
            .http
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| GatewayError::Adapter(e.to_string()))?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(GatewayError::UpstreamHttp { status, body });
        }
        Ok(resp)
    }
}

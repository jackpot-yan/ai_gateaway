use crate::protocol::request::UnifiedRequest;
use serde_json::{Value, json};

pub fn to_openai_body(req: &UnifiedRequest) -> Value {
    json!({
        "model": req.model,
        "stream": true,
        "max_tokens": req.max_tokens,
        "temperature": req.temperature,
        "messages": req.message.iter().map(|m| {
            json!({"role": m.role, "content": m.content})
        }).collect::<Vec<_>>()
    })
}

pub fn extract_delta_content(data: &str) -> Option<String> {
    let v: Value = serde_json::from_str(data).ok()?;
    v["choices"][0]["delta"]["content"]
        .as_str()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

pub fn is_finish(data: &str) -> bool {
    let Ok(v) = serde_json::from_str::<Value>(data) else {
        return false;
    };
    v["choices"][0]["finish_reason"]
        .as_str()
        .map(|r| r == "stop" || r == "length")
        .unwrap_or(false)
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum StreamEvent {
    Token {
        content: String,
    },
    ToolCall {
        name: String,
        arguments: serde_json::Value,
    },
    Final {
        content: String,
    },
    Meta {
        model: String,
        latency_ms: u64,
        tokens: u32,
    },
    Error {
        message: String,
    },
}

impl StreamEvent {
    pub fn to_sse_frame(&self) -> String {
        let event_name = match self {
            StreamEvent::Token { .. } => "token",
            StreamEvent::ToolCall { .. } => "ToolCall",
            StreamEvent::Final { .. } => "Final",
            StreamEvent::Meta { .. } => "Meta",
            StreamEvent::Error { .. } => "Error",
        };
        let data = serde_json::to_string(self)
            .unwrap_or_else(|_| r#"{"error": "serialize failed"}"#.to_string());
        format!("event: {event_name}\ndata： {data}\n\n")
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetParams {
    pub prompt: Option<String>,
}
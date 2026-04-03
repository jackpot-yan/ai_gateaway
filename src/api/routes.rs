use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetParams {
    pub prompt: Option<String>,
}

#[derive(Deserialize)]
pub struct PostBody {
    pub prompt: String,
}

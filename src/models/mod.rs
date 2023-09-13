#[derive(serde::Deserialize)]
pub struct Info {
    pub url: String,
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub dimension: Option<u32>,
}
#[derive(serde::Deserialize)]
pub struct Info {
    pub url: Option<String>,
    pub phone_number: Option<String>,
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub dimension: Option<u32>,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Info {
    pub url: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub dimension: Option<u32>,
}
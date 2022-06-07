use serde::Deserialize;

#[derive(Deserialize)]
pub enum PresenceKind {
    Custom,
    SystemInfo,
}

#[derive(Deserialize)]
pub struct Assets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,

    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Deserialize)]
pub struct Button {
    pub label: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub app_id: u64,

    #[serde(alias = "type")]
    pub kind: PresenceKind,

    pub state: Option<String>,
    pub details: Option<String>,

    pub assets: Option<Assets>,

    pub timestamp: Option<bool>,

    pub buttons: Option<Vec<Button>>,
}

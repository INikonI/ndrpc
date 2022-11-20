use serde::Deserialize;

#[derive(Deserialize)]
pub enum PresenceKind {
    #[serde(alias = "customstatic")]
    CustomStatic,
    #[serde(alias = "customdynamic")]
    CustomDynamic,
    #[serde(alias = "systeminfo")]
    SystemInfo,
}

#[derive(Deserialize)]
pub struct Config {
    pub app_id: u64,

    #[serde(alias = "type")]
    pub kind: PresenceKind,

    pub static_preset_name: Option<String>,
    pub dynamic_preset_names: Option<Vec<String>>,

    pub cpu_freq_button: Option<bool>,
    pub with_elapsed_time: Option<bool>,
}

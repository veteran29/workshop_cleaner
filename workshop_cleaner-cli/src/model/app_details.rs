use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct AppDetails {
    pub name: String,
    pub steam_appid: u32,
    pub short_description: String,
}

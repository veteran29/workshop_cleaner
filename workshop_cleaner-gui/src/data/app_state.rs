use druid::im::{vector, Vector};
use druid::{Data, Lens};

use super::SteamApp;

#[derive(Clone, Debug, Data, Lens)]
pub struct AppState {
    pub apps: Option<Vector<SteamApp>>,
    pub items: Vector<super::SteamWorkshopItem>,
    pub selected_app: Option<SteamApp>,
    pub selected_app_confirmed: bool,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            apps: None,
            items: vector![],
            selected_app: None,
            selected_app_confirmed: false,
        }
    }
}

use druid::im::{Vector, vector};
use druid::{Data, Lens};

#[derive(Clone, Debug, Data, Lens)]
pub struct AppState {
    pub apps: Option<Vector<super::SteamApp>>,
    pub items: Vector<super::SteamWorkshopItem>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            apps: None,
            items: vector![]
        }
    }
}

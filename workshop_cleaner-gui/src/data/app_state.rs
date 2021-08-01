use druid::im::Vector;
use druid::{Data, Lens};

#[derive(Clone, Debug, Data, Lens)]
pub struct AppState {
    pub apps: Vector<super::SteamApp>,
    pub items: Vector<super::SteamWorkshopItem>,
}

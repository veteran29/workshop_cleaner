use std::fmt;

use druid::im::Vector;
use druid::{Data, Lens};

#[derive(Clone, Debug, Data, Lens)]
pub struct SteamApp {
    pub app_id: u32,
    pub name: String,
    pub workshop_items: Vector<super::SteamWorkshopItem>,
}

impl fmt::Display for SteamApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.app_id)
    }
}

#[derive(Clone, Debug, Data)]
pub struct SteamWorkshopItem(pub u64);

impl fmt::Display for SteamWorkshopItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

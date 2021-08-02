use druid::im::Vector;
use druid::Selector;

use crate::data::SteamApp;

pub const SET_STEAM_APPS: Selector<Vector<SteamApp>> = Selector::new("data.set_steam_apps");

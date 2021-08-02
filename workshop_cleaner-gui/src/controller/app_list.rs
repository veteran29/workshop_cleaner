use druid::im::{vector, Vector};
use druid::Target;
use druid::{widget::Controller, Data, Widget};
use workshop_cleaner_core::locator::SteamLocator;

use crate::cmd;
use crate::data::{AppState, SteamApp};
use std::thread;

pub struct AppListController {}

impl AppListController {
    pub fn new() -> Self {
        AppListController {}
    }
}

impl<W> Controller<Vector<SteamApp>, W> for AppListController where W: Widget<Vector<SteamApp>> {}

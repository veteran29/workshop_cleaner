use druid::{widget::Controller, Widget};

use crate::data::AppState;

pub struct AppListController {}

impl AppListController {}

impl<W> Controller<AppState, W> for AppListController where W: Widget<AppState> {}

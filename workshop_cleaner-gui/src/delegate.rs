use druid::AppDelegate;

use crate::{cmd as commands, data::AppState};

pub struct Delegate {}

impl Delegate {
    pub fn new() -> Self {
        Delegate {}
    }
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        env: &druid::Env,
    ) -> druid::Handled {
        if let Some(apps) = cmd.get(commands::SET_STEAM_APPS).cloned() {
            data.apps = Some(apps);
        }

        druid::Handled::No
    }
}

use druid::{AppDelegate, Handled};
use workshop_cleaner_core::AppId;

use crate::{
    cmd as commands,
    data::{AppState, SteamWorkshopItem},
};

pub struct Delegate {
    cleaner: Option<workshop_cleaner_core::cleaner::WorkshopCleaner>,
}

impl Delegate {
    pub fn new() -> Self {
        Delegate { cleaner: None }
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
            data.selected_app = None;
            data.apps = Some(apps);

            return Handled::Yes;
        }

        if let Some(app) = cmd.get(commands::SELECT_STEAM_APP).cloned().as_mut() {
            if !data.selected_app_confirmed {
                data.selected_app = Some(app.clone());
            }

            return Handled::Yes;
        }

        if cmd.is(commands::CONFIRM_STEAM_APP) {
            data.selected_app_confirmed = true;

            let app_id = &data.selected_app.as_ref().unwrap().app_id;
            let client = workshop_cleaner_core::init(AppId(*app_id));

            if (client.is_err()) {
                println!("Failed to init WorkshopCleaner");

                return Handled::Yes;
            }

            self.cleaner = Some(client.unwrap());

            data.items = self
                .cleaner
                .as_ref()
                .unwrap()
                .get_installed_not_subscribed_items()
                .into_iter()
                .map(|i| SteamWorkshopItem(i.0))
                .collect();

            return Handled::Yes;
        }

        druid::Handled::No
    }
}

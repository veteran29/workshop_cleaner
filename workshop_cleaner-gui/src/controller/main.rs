use druid::im::{vector, Vector};
use druid::{widget::Controller, Target, Widget};
use workshop_cleaner_core::locator::SteamLocator;

use crate::{
    cmd,
    data::{AppState, SteamApp},
};

pub struct MainController {}

impl MainController {
    pub fn new() -> Self {
        MainController {}
    }

    fn on_window_connected(
        &self,
        ctx: &druid::EventCtx,
        _event: &druid::Event,
        data: &mut AppState,
        _env: &druid::Env,
    ) {
        // Initialize apps list
        data.apps = None;

        let sink = ctx.get_external_handle();
        std::thread::spawn(move || {
            let apps: Vector<SteamApp> = SteamLocator::new()
                .get_installed_workshop_apps()
                .iter()
                .map(|a| SteamApp {
                    app_id: a.0,
                    name: "Unkown".to_string(),
                    workshop_items: vector!(),
                })
                .collect();

            sink.submit_command(cmd::SET_STEAM_APPS, apps, Target::Auto)
                .expect("Failed to send command");
        });
    }
}

impl<W> Controller<AppState, W> for MainController
where
    W: Widget<AppState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        match event {
            druid::Event::WindowConnected => self.on_window_connected(ctx, event, data, env),
            _ => (),
        };

        child.event(ctx, event, data, env)
    }
}

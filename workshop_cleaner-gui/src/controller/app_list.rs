use druid::im::{vector, Vector};
use druid::widget::{Container, Flex};
use druid::{widget::Controller, Data, Widget};
use druid::{Color, Cursor, Target};
use workshop_cleaner_core::locator::SteamLocator;

use crate::cmd as command;
use crate::data::{AppState, SteamApp};
use std::thread;

pub struct AppListController {}

impl AppListController {
    pub fn new() -> Self {
        AppListController {}
    }
}

impl<W> Controller<Vector<SteamApp>, W> for AppListController where W: Widget<Vector<SteamApp>> {}

pub struct AppListItemController {
    hover: bool,
    selected: bool,
}

impl AppListItemController {
    pub fn new() -> Self {
        AppListItemController {
            hover: false,
            selected: false,
        }
    }

    fn get_current_color(&self) -> Color {
        if self.selected {
            Color::RED
        } else {
            Color::rgba8(0, 0, 0, 0)
        }
    }

    fn set_background<T: Data>(
        &self,
        child: &mut Container<T>,
        ctx: &mut druid::EventCtx,
        color: Color,
    ) {
        child.set_background(color);
        ctx.request_paint();
    }
}

impl Controller<SteamApp, Container<SteamApp>> for AppListItemController {
    fn event(
        &mut self,
        child: &mut Container<SteamApp>,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut SteamApp,
        env: &druid::Env,
    ) {
        match event {
            druid::Event::MouseDown(_) => {
                ctx.get_external_handle()
                    .submit_command(command::SELECT_STEAM_APP, data.clone(), Target::Auto)
                    .expect("Failed to send command");
            }
            // TODO custom wrapping widget with "on hover" color
            druid::Event::MouseMove(_) => {
                let hover = ctx.is_hot();
                if hover != self.hover {
                    self.hover = hover;

                    println!("{:?} - {}", data, hover);

                    if hover {
                        ctx.set_cursor(&Cursor::OpenHand);
                        self.set_background(child, ctx, Color::GRAY);
                    } else {
                        ctx.clear_cursor();
                        self.set_background(child, ctx, self.get_current_color());
                    }

                    ctx.request_paint();
                }
            }
            druid::Event::Command(cmd) => {
                if let Some(app) = cmd.get(command::SELECT_STEAM_APP) {
                    self.selected = app.app_id == data.app_id;
                    self.set_background(child, ctx, self.get_current_color());
                }
            }
            _ => (),
        }

        child.event(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut Container<SteamApp>,
        ctx: &mut druid::UpdateCtx,
        old_data: &SteamApp,
        data: &SteamApp,
        env: &druid::Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}

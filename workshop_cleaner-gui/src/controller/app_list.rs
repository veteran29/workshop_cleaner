use druid::im::{vector, Vector};
use druid::widget::{Container, Flex};
use druid::{widget::Controller, Data, Widget};
use druid::{Color, Cursor, Rect, Selector, Target};
use workshop_cleaner_core::locator::SteamLocator;

use crate::cmd as command;
use crate::data::{AppState, SteamApp};
use std::thread;

const COLOR_TRANSPARENT: Color = Color::rgba8(0, 0, 0, 0);

const SET_HOVER: Selector<bool> = Selector::new("app-list.hover");

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

    fn on_hover<T: Data>(
        &mut self,
        child: &mut Container<T>,
        ctx: &mut druid::EventCtx,
        hover: bool,
    ) {
        if hover != self.hover {
            self.hover = hover;

            if hover {
                ctx.set_cursor(&Cursor::OpenHand);
            } else {
                ctx.clear_cursor();
            }

            self.paint_background(child);
        }
    }

    fn on_selected<T: Data>(
        &mut self,
        child: &mut Container<T>,
        ctx: &mut druid::EventCtx,
        selected: bool,
    ) {
        self.selected = selected;

        self.paint_background(child);
        ctx.request_paint();
    }

    fn paint_background<T: Data>(&self, child: &mut Container<T>) {
        let color = match (self.hover, self.selected) {
            (true, true) => Color::RED,
            (false, true) => Color::MAROON,
            (true, false) => Color::GRAY,
            _ => COLOR_TRANSPARENT,
        };

        child.set_background(color);
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
            druid::Event::Command(cmd) => {
                if let Some(app) = cmd.get(command::SELECT_STEAM_APP) {
                    let selected = app.app_id == data.app_id;
                    self.on_selected(child, ctx, selected);
                }

                if let Some(hot) = cmd.get(SET_HOVER) {
                    self.on_hover(child, ctx, *hot);
                }
            }
            _ => (),
        }

        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut Container<SteamApp>,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &SteamApp,
        env: &druid::Env,
    ) {
        match event {
            druid::LifeCycle::HotChanged(hot) => {
                ctx.submit_command(SET_HOVER.with(*hot).to(Target::Widget(ctx.widget_id())));
            }
            _ => (),
        }

        child.lifecycle(ctx, event, data, env)
    }
}

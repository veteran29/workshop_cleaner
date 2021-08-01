use std::fmt::{Debug, Display};

use data::{AppState, SteamApp, SteamWorkshopItem};
use druid::im::{vector, Vector};
use druid::lens::{self, LensExt};
use druid::widget::{Button, Either, Flex, Label, List, Scroll, Split};
use druid::{AppLauncher, Color, Data, Lens, UnitPoint, Widget, WidgetExt, WindowDesc};

use workshop_cleaner_core::locator::SteamLocator;

mod controller;
mod data;

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((1000.0, 512.0))
        .title("Workshop Cleaner");

    let apps = SteamLocator::new()
        .get_installed_workshop_apps()
        .iter()
        .map(|a| data::SteamApp {
            app_id: a.0,
            name: "Unkown".to_string(),
            workshop_items: vector!(),
        })
        .collect();

    let state = data::AppState {
        apps,
        items: vector!(),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(state)
        .expect("App launch failed");
}

const MAIN_LAYOUT_SPLIT_POINT: f64 = 0.3;

fn ui_builder() -> impl Widget<data::AppState> {
    let apps_col = Scroll::new(Either::new(
        |data: &Vector<SteamApp>, _env| !data.is_empty(),
        List::new(app_widget),
        Label::new("No workshop apps found")
            .padding(20.)
            .align_horizontal(UnitPoint::CENTER)
            .expand_width(),
    ))
    .vertical()
    .lens(AppState::apps);

    let items_col = Scroll::new(Either::new(
        |data: &Vector<SteamWorkshopItem>, _env| !data.is_empty(),
        List::new(item_widget),
        Label::new("Nothing found, your workshop is clean")
            .padding(20.)
            .align_horizontal(UnitPoint::CENTER)
            .expand_width(),
    ))
    .vertical()
    .lens(AppState::items);

    Split::columns(apps_col, items_col)
        .split_point(MAIN_LAYOUT_SPLIT_POINT)
        .solid_bar(true)
        .draggable(true)
        .debug_paint_layout()
}

const LIST_ITEM_HEIGHT: f64 = 50.0;
const LIST_ITEM_PADDING: f64 = 10.0;

fn app_widget<T>() -> impl Widget<T>
where
    T: Data + Display,
{
    Label::new(|item: &T, _env: &_| format!("{}", item))
        .align_vertical(UnitPoint::LEFT)
        .padding(LIST_ITEM_PADDING)
        .expand()
        .height(LIST_ITEM_HEIGHT)
}

fn item_widget<T>() -> impl Widget<T>
where
    T: Data + Display,
{
    Flex::row()
        .with_child(
            Label::new(|item: &T, _env: &_| format!("{}", item)).align_vertical(UnitPoint::LEFT),
        )
        .with_flex_spacer(1.0)
        .with_child(
            Button::new("Delete")
                .fix_size(80.0, 30.0)
                .align_vertical(UnitPoint::CENTER),
        )
        .padding(LIST_ITEM_PADDING)
        .background(Color::rgb(0.5, 0.0, 0.5))
        .fix_height(LIST_ITEM_HEIGHT)
}

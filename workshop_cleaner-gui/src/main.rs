use std::fmt::{Debug, Display};

use controller::{AppListController, MainController};
use data::{AppState, SteamApp, SteamWorkshopItem};
use druid::im::{vector, Vector};
use druid::lens::{self, LensExt};
use druid::widget::{Button, Either, Flex, Label, List, Scroll, Split};
use druid::{AppLauncher, Color, Data, EventCtx, Lens, UnitPoint, Widget, WidgetExt, WindowDesc};

use crate::{controller::AppListItemController, delegate::Delegate};

mod cmd;
mod controller;
mod data;
mod delegate;

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((1000.0, 512.0))
        .title("Workshop Cleaner");

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .delegate(Delegate::new())
        .launch(AppState::default())
        .expect("App launch failed");
}

const MAIN_LAYOUT_SPLIT_POINT: f64 = 0.3;

fn ui_builder() -> impl Widget<AppState> {
    // TODO move to module, LayoutSwitcher
    let apps_col = Scroll::new(Either::new(
        |data: &Option<Vector<SteamApp>>, _env| data.is_some(),
        app_list_widget(),
        Label::new("Scanning for apps...")
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
        .controller(MainController::new())
    // .debug_paint_layout()
}

const LIST_ITEM_HEIGHT: f64 = 50.0;
const LIST_ITEM_PADDING: f64 = 10.0;

fn app_list_widget() -> impl Widget<Option<Vector<SteamApp>>> {
    Either::new(
        |data: &Vector<SteamApp>, _env| !data.is_empty(),
        List::new(app_list_item_widget).controller(AppListController::new()),
        Label::new("No workshop apps")
            .padding(20.)
            .align_horizontal(UnitPoint::CENTER)
            .expand_width(),
    )
    .lens(lens::Identity.map(
        |d: &Option<Vector<SteamApp>>| d.clone().unwrap_or_else(|| vector![]),
        |_d, _input| (),
    )) // TODO pass tuple of item and currently selected app to the list so it can dynamically detect which one is selected
}

fn app_list_item_widget() -> impl Widget<SteamApp> {
    let label =
        Label::new(|item: &SteamApp, _env: &_| format!("{}", item)).align_vertical(UnitPoint::LEFT);

    Flex::row()
        .with_child(label)
        .padding(LIST_ITEM_PADDING)
        .expand_width()
        .height(LIST_ITEM_HEIGHT)
        .background(Color::from_hex_str("0000").unwrap())
        .controller(AppListItemController::new())
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

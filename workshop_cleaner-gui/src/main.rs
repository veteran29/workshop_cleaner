use std::fmt::{Debug, Display};
use std::num::NonZeroU64;

use controller::MainController;
use data::{AppState, SteamApp, SteamWorkshopItem};
use druid::im::{vector, Vector};
use druid::lens::{self, LensExt};
use druid::widget::{
    Button, Container, Either, Flex, Label, LensWrap, List, Scroll, Split, ViewSwitcher,
};
use druid::{
    AppLauncher, Color, Data, EventCtx, ImageBuf, Lens, Rect, UnitPoint, Widget, WidgetExt,
    WidgetId, WindowDesc,
};
use widget::list::NavList;

use delegate::Delegate;

mod cmd;
mod controller;
mod data;
mod delegate;
mod ui;
mod widget;

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

type NavListState = (Option<Vector<SteamApp>>, Option<SteamApp>);

fn ui_builder() -> impl Widget<AppState> {
    Split::columns(nav_list_widget(), items_widget())
        .split_point(MAIN_LAYOUT_SPLIT_POINT)
        .solid_bar(true)
        .draggable(true)
        .controller(MainController::new())
    // .debug_paint_layout()
}

fn nav_list_widget() -> impl Widget<AppState> {
    let view_switcher = ViewSwitcher::new(
        |(items, _): &NavListState, _| items.clone(),
        |items, _, _| match items {
            Some(_) => Box::new(app_list_widget()),
            None => Box::new(
                Label::new("Scanning for apps...")
                    .padding(20.)
                    .align_horizontal(UnitPoint::TOP)
                    .expand(),
            ),
        },
    )
    .lens(lens::Map::new(
        |data: &AppState| (data.apps.clone(), data.selected_app.clone()),
        |_, _| (),
    ));

    Container::new(view_switcher)
}

fn app_list_widget() -> impl Widget<NavListState> {
    let list = widget::list::NavList::new(|ctx, data: &SteamApp| {
        ctx.submit_command(cmd::SELECT_STEAM_APP.with(data.clone()))
    });

    Scroll::new(list).vertical()
}

fn items_widget() -> impl Widget<AppState> {
    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _| data.selected_app.clone(),
        |selected_app, _, _| match selected_app {
            Some(_) => Box::new(Either::new(
                |data: &AppState, _| !data.selected_app_confirmed,
                items_confirm_widget(),
                Either::new(
                    |data: &AppState, _| !data.items.is_empty(),
                    List::new(item_widget).lens(AppState::items),
                    Label::new("Nothing found, your workshop is clean.")
                        .padding(20.)
                        .align_horizontal(UnitPoint::TOP)
                        .expand(),
                ),
            )),
            None => Box::new(
                Label::new("Select app to continue...")
                    .padding(20.)
                    .align_horizontal(UnitPoint::TOP)
                    .expand_width(),
            ),
        },
    );

    Container::new(view_switcher)
}

fn items_confirm_widget() -> impl Widget<AppState> {
    let widget = Flex::column();

    let label = Label::new(
        "Please confirm app selection, you will need to restart the application to change it.",
    )
    .padding(20.)
    .align_horizontal(UnitPoint::TOP)
    .expand_width();
    let button =
        Button::new("Confirm").on_click(|ctx, _, _| ctx.submit_command(cmd::CONFIRM_STEAM_APP));

    widget.with_child(label).with_child(button)
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
        .padding(ui::theme::NAV_LIST_ITEM_PADDING)
        .background(Color::rgb(0.5, 0.0, 0.5))
        .fix_height(ui::theme::NAV_LIST_ITEM_HEIGHT)
}

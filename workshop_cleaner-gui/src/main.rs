use druid::im::{vector, Vector};
use druid::lens::{self, LensExt};
use druid::widget::{Button, Flex, Label, List, Scroll, Split};
use druid::{
    AppLauncher, Color, Data, Lens, UnitPoint, Widget, WidgetExt, WindowDesc,
};


#[derive(Clone, Data, Lens)]
struct AppState {
    apps: Vector<String>,
    items: Vector<String>,
}

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((1000.0, 512.0))
        .title("Workshop Cleaner")
    ;

    let state = AppState {
        apps: vector!("Arma 3".to_string(), "Dota 2".to_string()),
        items: vector!("1234".to_string(), "3456".to_string()),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(state)
        .expect("App launch failed")
    ;
}

const MAIN_LAYOUT_SPLIT_POINT: f64 = 0.3;

fn ui_builder() -> impl Widget<AppState> {
    let left_col = Scroll::new(List::new(app_widget))
        .vertical()
        .lens(AppState::apps)
    ;

    let right_col = Scroll::new(List::new(item_widget))
    .vertical()
    .lens(AppState::items)
;

    Split::columns(left_col, right_col)
        .split_point(MAIN_LAYOUT_SPLIT_POINT)
        .solid_bar(true)
        .draggable(true)
        .debug_paint_layout()
}

const LIST_ITEM_HEIGHT: f64 = 50.0;
const LIST_ITEM_PADDING: f64 = 10.0;

fn app_widget() -> impl Widget<String> {
    Label::new(|item: &String, _env: &_| {
        format!("App: {}", item)
    })
    .align_vertical(UnitPoint::LEFT)
    .padding(LIST_ITEM_PADDING)
    .expand()
    .height(LIST_ITEM_HEIGHT)
}

fn item_widget() -> impl Widget<String> {
    Flex::row()
        .with_child(
            Label::new(|item: &String, _env: &_| {
                format!("List item #{}", item)
            })
            .align_vertical(UnitPoint::LEFT),
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

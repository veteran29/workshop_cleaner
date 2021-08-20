use ::druid::Color;
pub use druid::theme as druid;

pub fn grid(m: f64) -> f64 {
    GRID * m
}

pub const GRID: f64 = 8.0;

pub const COLOR_TRANSPARENT: Color = Color::rgba8(0, 0, 0, 0);

pub const COLOR_GREY_400: Color = Color::grey8(0x82);
pub const COLOR_GREY_500: Color = Color::grey8(0x4f);

pub const NAV_LIST_ITEM_HEIGHT: f64 = 50.0;
pub const NAV_LIST_ITEM_PADDING: f64 = 10.0;

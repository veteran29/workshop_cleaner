use std::f64::consts::PI;

use druid::{
    kurbo::Circle, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Vec2, Widget, WidgetExt,
};

use crate::ui::theme;

// https://github.com/jpochyla/psst/blob/6484d242a99b4d1c7431e40d5a3300471ba37905/psst-gui/src/ui/utils.rs
struct Spinner {
    t: f64,
}

impl Spinner {
    pub fn new() -> Self {
        Self { t: 0.0 }
    }
}

impl<T: Data> Widget<T> for Spinner {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        if let Event::AnimFrame(interval) = event {
            self.t += (*interval as f64) * 1e-9;
            if self.t >= 1.0 {
                self.t = 0.0;
            }
            ctx.request_anim_frame();
            ctx.request_paint();
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &T, _env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            ctx.request_anim_frame();
            ctx.request_paint();
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        bc.constrain(Size::new(theme::grid(6.0), theme::grid(6.0)))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let center = ctx.size().to_rect().center();
        let c0 = theme::COLOR_GREY_400;
        let c1 = Color::WHITE;
        let active = 7 - (1 + (6.0 * self.t).floor() as i32);
        for i in 1..=6 {
            let step = f64::from(i);
            let angle = Vec2::from_angle((step / 6.0) * -2.0 * PI);
            let dot_center = center + angle * theme::grid(2.0);
            let dot = Circle::new(dot_center, theme::grid(0.8));
            if i == active {
                ctx.fill(dot, &c1);
            } else {
                ctx.fill(dot, &c0);
            }
        }
    }
}

pub fn spinner_widget<T: Data>() -> impl Widget<T> {
    Spinner::new().center()
}

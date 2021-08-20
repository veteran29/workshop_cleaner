use druid::{
    kurbo::{CircleSegment},
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, Size, UpdateCtx, Widget, WidgetExt,
};

use crate::ui::theme;

// https://github.com/jpochyla/psst/blob/6484d242a99b4d1c7431e40d5a3300471ba37905/psst-gui/src/ui/utils.rs
struct Spinner {
    t: f64,
}

impl Spinner {
    pub fn new() -> Self {
        Self { t: 360.0 }
    }
}

impl<T: Data> Widget<T> for Spinner {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        if let Event::AnimFrame(interval) = event {
            self.t -= (*interval as f64) * 1e-7 * 3.;
            if self.t <= 0.0 {
                self.t = 360.0;
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
        bc.constrain(Size::new(theme::grid(8.0), theme::grid(8.0)))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, _env: &Env) {
        let center = ctx.size().to_rect().center();

        for i in 1..=360 {
            if i % 5 != 0 {
                continue;
            };

            let segment = CircleSegment {
                center,
                inner_radius: 18.0,
                outer_radius: 25.0,
                start_angle: (i as f64).to_radians(),
                sweep_angle: (10 as f64).to_radians(),
            };

            let deg = ((i as f64) + self.t) % 360.0;
            let alpha = (360.0 - (deg * 0.95)) / 360.0;

            ctx.fill(segment, &Color::WHITE.with_alpha(1.0 - alpha * 1.3));
        }
    }
}

pub fn spinner_widget<T: Data>() -> impl Widget<T> {
    Spinner::new().center()
}

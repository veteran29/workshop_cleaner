use druid::{Data, Point, Size, Widget, WidgetPod};

type ShowOverlayCallback<T> = dyn Fn(&T) -> bool;
type PassthroughOverlayEventsCallback<T> = dyn Fn(&T) -> bool;

/// A widget that allows rendering of two widgets on top of each other.
pub struct StackedContainer<T> {
    inner_bottom: WidgetPod<T, Box<dyn Widget<T>>>,
    inner_top: WidgetPod<T, Box<dyn Widget<T>>>,
    show_overlay_callback: Box<ShowOverlayCallback<T>>,
    passthrough_overlay_events_callback: Box<PassthroughOverlayEventsCallback<T>>,
    overlay_shown: bool,
}

impl<T: Data> StackedContainer<T> {
    /// Create stacked container with two child widgets.
    pub fn new(bottom: impl Widget<T> + 'static, top: impl Widget<T> + 'static) -> Self {
        Self {
            inner_bottom: WidgetPod::new(bottom).boxed(),
            inner_top: WidgetPod::new(top).boxed(),
            show_overlay_callback: Box::new(|_| false),
            passthrough_overlay_events_callback: Box::new(|_| false),
            overlay_shown: false,
        }
    }

    /// Builder style method for setting show overlay callback.
    ///
    /// Visibility of the top element depends on the return value of this callback.
    /// No events will be sent to top element if it's not visible.
    pub fn with_show_overlay_callback(mut self, callback: Box<ShowOverlayCallback<T>>) -> Self {
        self.show_overlay_callback = callback;

        self
    }

    /// Builder style method for setting passthrough overlay events callback.
    ///
    /// When the callback returns `true` then events will be passed to the bottom widget,
    /// in case `false` will be returned then events will "stop" at the top element.
    pub fn with_passthrough_overlay_events_callback(
        mut self,
        callback: Box<ShowOverlayCallback<T>>,
    ) -> Self {
        self.passthrough_overlay_events_callback = callback;

        self
    }
}

// TODO: callback condition to render top element, callback condition to pass events to bottom element
impl<T: Data> Widget<T> for StackedContainer<T> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) {
        self.overlay_shown = (self.show_overlay_callback)(data);
        if !self.overlay_shown || (self.passthrough_overlay_events_callback)(data) {
            self.inner_bottom.event(ctx, event, data, env);
        }
        if self.overlay_shown {
            self.inner_top.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        self.inner_bottom.lifecycle(ctx, event, data, env);
        self.inner_top.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &T, data: &T, env: &druid::Env) {
        self.inner_bottom.update(ctx, data, env);
        self.inner_top.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        let bottom_layout = self.inner_bottom.layout(ctx, bc, data, env);
        let top_layout = self.inner_top.layout(ctx, bc, data, env);

        self.inner_bottom.set_origin(ctx, data, env, Point::ORIGIN);
        self.inner_top.set_origin(ctx, data, env, Point::ORIGIN);

        Size {
            width: bottom_layout.width.max(top_layout.width),
            height: bottom_layout.height.max(top_layout.height),
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        self.inner_bottom.paint(ctx, data, env);
        if self.overlay_shown {
            self.inner_top.paint(ctx, data, env);
        }
    }
}

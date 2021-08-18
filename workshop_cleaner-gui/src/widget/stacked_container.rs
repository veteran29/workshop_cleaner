use druid::{Data, Point, Size, Widget, WidgetPod};

pub struct StackedContainer<T> {
    inner_bottom: WidgetPod<T, Box<dyn Widget<T>>>,
    inner_top: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> StackedContainer<T> {
    pub fn new(bottom: impl Widget<T> + 'static, top: impl Widget<T> + 'static) -> Self {
        Self {
            inner_bottom: WidgetPod::new(bottom).boxed(),
            inner_top: WidgetPod::new(top).boxed(),
        }
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
        self.inner_bottom.event(ctx, event, data, env);
        self.inner_top.event(ctx, event, data, env);
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
        self.inner_top.paint(ctx, data, env);
    }
}

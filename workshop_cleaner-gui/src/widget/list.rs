use std::{cmp::Ordering, fmt::Display};

use druid::{
    widget::{Axis, Flex, Label, ListIter, Widget},
    BoxConstraints, Color, Data, Env, EventCtx, KeyOrValue, LifeCycle, Point, Rect, RenderContext,
    Selector, Size, UnitPoint, WidgetExt, WidgetPod,
};

use crate::ui;

type FnOnSelected<C> = dyn Fn(&mut EventCtx, &C);

pub struct NavList<C> {
    item_builder: Box<dyn Fn() -> Box<dyn Widget<(C, bool)>>>,
    on_selected: Box<FnOnSelected<C>>,
    children: Vec<WidgetPod<(C, bool), Box<dyn Widget<(C, bool)>>>>,
    axis: Axis,
    spacing: KeyOrValue<f64>,
}

impl<C: Data + Display> NavList<C> {
    pub fn new(on_selected: impl Fn(&mut EventCtx, &C) + 'static) -> Self {
        let item_builder = || NavListItem::new();

        Self {
            item_builder: Box::new(move || Box::new(item_builder())),
            on_selected: Box::new(on_selected),
            children: Vec::new(),
            axis: Axis::Vertical,
            spacing: KeyOrValue::Concrete(0.),
        }
    }

    const SELECTOR_ON_CLICK: Selector<C> = Selector::new("naw-item.on-click");

    /// When the widget is created or the data changes, create or remove children as needed
    ///
    /// Returns `true` if children were added or removed.
    fn update_child_count(&mut self, data: &impl ListIter<C>, _env: &Env) -> bool {
        let len = self.children.len();
        match len.cmp(&data.data_len()) {
            Ordering::Greater => self.children.truncate(data.data_len()),
            Ordering::Less => data.for_each(|_, i| {
                if i >= len {
                    let child = WidgetPod::new((self.item_builder)());
                    self.children.push(child);
                }
            }),
            Ordering::Equal => (),
        }
        len != data.data_len()
    }

    fn axis_constraints(
        axis: &Axis,
        bc: &BoxConstraints,
        min_major: f64,
        major: f64,
    ) -> BoxConstraints {
        match axis {
            Axis::Horizontal => BoxConstraints::new(
                Size::new(min_major, bc.min().height),
                Size::new(major, bc.max().height),
            ),
            Axis::Vertical => BoxConstraints::new(
                Size::new(bc.min().width, min_major),
                Size::new(bc.max().width, major),
            ),
        }
    }
}

type NavListData<C, I> = (Option<I>, Option<C>);

impl<C, I> Widget<NavListData<C, I>> for NavList<C>
where
    C: Data + Display,
    I: ListIter<C> + Default,
{
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        (items, selected_item): &mut NavListData<C, I>,
        env: &druid::Env,
    ) {
        match event {
            druid::Event::Notification(notification) => {
                if let Some(payload) = notification.get(Self::SELECTOR_ON_CLICK) {
                    (self.on_selected)(ctx, payload);
                    return ctx.set_handled();
                }
            }
            _ => (),
        }

        let data = items.as_mut().unwrap();

        let mut children = self.children.iter_mut();
        data.for_each_mut(|child_data, _| {
            if let Some(child) = children.next() {
                let selected = if let Some(selected_item) = selected_item {
                    child_data.same(selected_item)
                } else {
                    false
                };

                child.event(ctx, event, &mut (child_data.clone(), selected), env);
            }
        });
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        (items, selected_item): &NavListData<C, I>,
        env: &druid::Env,
    ) {
        let data = items.as_ref().unwrap();

        if let LifeCycle::WidgetAdded = event {
            if self.update_child_count(data, env) {
                ctx.children_changed();
            }
        }

        let mut children = self.children.iter_mut();
        data.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                let selected = if let Some(selected_item) = selected_item {
                    child_data.same(selected_item)
                } else {
                    false
                };

                child.lifecycle(ctx, event, &(child_data.clone(), selected), env);
            }
        });
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &NavListData<C, I>,
        data: &NavListData<C, I>,
        env: &druid::Env,
    ) {
        let selected_item = data.1.as_ref();
        let data = data.0.as_ref().unwrap();

        // we send update to children first, before adding or removing children;
        // this way we avoid sending update to newly added children, at the cost
        // of potentially updating children that are going to be removed.
        let mut children = self.children.iter_mut();
        data.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                let selected = if let Some(selected_item) = selected_item {
                    child_data.same(selected_item)
                } else {
                    false
                };

                child.update(ctx, &(child_data.clone(), selected), env);
            }
        });

        if self.update_child_count(data, env) {
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        (items, selected_item): &NavListData<C, I>,
        env: &druid::Env,
    ) -> druid::Size {
        let data = items.as_ref().unwrap();

        let axis = self.axis;
        let spacing = self.spacing.resolve(env);
        let mut minor = axis.minor(bc.min());
        let mut major_pos = 0.0;
        let mut paint_rect = Rect::ZERO;
        let mut children = self.children.iter_mut();
        let child_bc = Self::axis_constraints(&axis, bc, 0., f64::INFINITY);
        data.for_each(|child_data, _| {
            let child = match children.next() {
                Some(child) => child,
                None => {
                    return;
                }
            };

            let selected = if let Some(selected_item) = selected_item {
                child_data.same(selected_item)
            } else {
                false
            };

            let child_size = child.layout(ctx, &child_bc, &(child_data.clone(), selected), env);
            let child_pos: Point = axis.pack(major_pos, 0.).into();
            child.set_origin(ctx, &(child_data.clone(), selected), env, child_pos);
            paint_rect = paint_rect.union(child.paint_rect());
            minor = minor.max(axis.minor(child_size));
            major_pos += axis.major(child_size) + spacing;
        });

        // correct overshoot at end.
        major_pos -= spacing;

        let my_size = bc.constrain(Size::from(axis.pack(major_pos, minor)));
        let insets = paint_rect - my_size.to_rect();
        ctx.set_paint_insets(insets);
        my_size
    }

    fn paint(
        &mut self,
        ctx: &mut druid::PaintCtx,
        (items, selected_item): &NavListData<C, I>,
        env: &druid::Env,
    ) {
        let data = items.as_ref().unwrap();

        let mut children = self.children.iter_mut();
        data.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                let selected = if let Some(selected_item) = selected_item {
                    child_data.same(selected_item)
                } else {
                    false
                };

                child.paint(ctx, &(child_data.clone(), selected), env);
            }
        });
    }
}

type NavListItemWidget<T> = WidgetPod<T, Box<dyn Widget<T>>>;

struct NavListItem<T> {
    inner: Option<NavListItemWidget<T>>,
    focus: bool,
}

impl<T: Data + Display> NavListItem<T> {
    fn new() -> Self {
        Self {
            inner: None,
            focus: false,
        }
    }

    fn build_inner_item() -> NavListItemWidget<T> {
        let label =
            Label::new(|item: &T, _env: &_| format!("{}", item)).align_vertical(UnitPoint::LEFT);

        let item = Flex::row()
            .with_child(label)
            .padding(ui::theme::NAV_LIST_ITEM_PADDING)
            .expand_width()
            .height(ui::theme::NAV_LIST_ITEM_HEIGHT);

        WidgetPod::new(Box::new(item))
    }
}

impl<I: Data + Display> Widget<(I, bool)> for NavListItem<I> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        (data, _): &mut (I, bool),
        env: &druid::Env,
    ) {
        match event {
            druid::Event::MouseDown(_) => {
                ctx.submit_notification(NavList::SELECTOR_ON_CLICK.with(data.clone()));
            }
            _ => (),
        }

        if let Some(child) = self.inner.as_mut() {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        (data, _): &(I, bool),
        env: &druid::Env,
    ) {
        match event {
            LifeCycle::WidgetAdded => {
                self.inner = Some(Self::build_inner_item());
            }
            LifeCycle::HotChanged(_hot) => {
                ctx.request_paint();
            }
            _ => (),
        };

        if let Some(child) = self.inner.as_mut() {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        (old_data, _): &(I, bool),
        (data, focus): &(I, bool),
        env: &druid::Env,
    ) {
        if self.focus != *focus {
            self.focus = *focus;
            self.inner = Some(Self::build_inner_item());
            ctx.children_changed();
        // Because the new child has not yet been initialized, we have to skip the update after switching.
        } else if let Some(child) = self.inner.as_mut() {
            child.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        (data, _): &(I, bool),
        env: &druid::Env,
    ) -> druid::Size {
        match self.inner {
            Some(ref mut child) => {
                let size = child.layout(ctx, bc, data, env);
                child.set_origin(ctx, data, env, Point::ORIGIN);
                size
            }
            None => bc.max(),
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, (data, _): &(I, bool), env: &druid::Env) {
        if let Some(child) = self.inner.as_mut() {
            if ctx.is_hot() {
                let rect = ctx.size().to_rect();
                ctx.fill(rect, &Color::GRAY);
            }
            if self.focus {
                let rect = ctx.size().to_rect();
                let bar = rect.with_size(Size {
                    height: rect.height(),
                    width: ui::theme::NAV_LIST_ITEM_PADDING / 2.,
                });
                ctx.fill(bar, &Color::WHITE);
            }

            child.paint_raw(ctx, data, env);
        }
    }
}

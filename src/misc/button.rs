use engine::{
    drawable::{Anchor, Drawable},
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
    layout::{LayoutElement, bounds::Bounds2D, tracker::LayoutTracker},
    memory::MemoryKey,
};

pub struct Button<T: ButtonContent> {
    asset: T,
    key: MemoryKey,
}

#[derive(Default)]
struct ButtonState {
    hover_time: f32,
}

pub trait ButtonContent: Drawable + LayoutElement {
    fn dynamic_scale(self, scale: Vector2<f32>, anchor: Anchor) -> Self;
    fn get_scale(&self) -> Vector2<f32>;
}

pub trait ButtonExt
where
    Self: ButtonContent + Sized,
{
    fn button(self, key: MemoryKey) -> Button<Self>;
}

impl<T: ButtonContent> Button<T> {
    pub fn new(key: MemoryKey, asset: T) -> Self {
        Self { asset, key }
    }

    pub fn is_clicked(&self, ctx: &mut GraphicsContext) -> bool {
        let hovered = LayoutTracker::new(self.key).hovered(ctx);
        hovered && ctx.input.mouse_pressed(MouseButton::Left)
    }

    pub fn on_click(self, ctx: &mut GraphicsContext, callback: impl FnOnce()) -> Self {
        self.is_clicked(ctx).then(callback);
        self
    }
}

impl<T: ButtonContent + 'static> Drawable for Button<T> {
    fn draw(mut self, ctx: &mut GraphicsContext) {
        let tracker = LayoutTracker::new(self.key);
        let hover = tracker.hovered(ctx);
        hover.then(|| ctx.window.cursor(CursorIcon::Pointer));

        let state = ctx.memory.get_or_insert(self.key, ButtonState::default());
        state.hover_time += ctx.delta_time * if hover { 1.0 } else { -1.0 };
        state.hover_time = state.hover_time.clamp(0.0, 0.1);
        let t = state.hover_time / 0.1;

        let scale = self.asset.get_scale();
        let scale = scale + Vector2::repeat(t / 20.0).component_mul(&scale);
        self.asset = self.asset.dynamic_scale(scale, Anchor::Center);

        self.asset.tracked(tracker).draw(ctx);
    }
}

impl<T: ButtonContent + 'static> LayoutElement for Button<T> {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.asset.translate(distance);
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        self.asset.bounds(ctx)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}

impl ButtonContent for Sprite {
    fn dynamic_scale(self, scale: Vector2<f32>, anchor: Anchor) -> Self {
        self.dynamic_scale(scale, anchor)
    }

    fn get_scale(&self) -> Vector2<f32> {
        self.get_scale()
    }
}

impl ButtonContent for Text {
    fn dynamic_scale(self, scale: Vector2<f32>, anchor: Anchor) -> Self {
        self.dynamic_scale(scale, anchor)
    }

    fn get_scale(&self) -> Vector2<f32> {
        self.get_scale()
    }
}

impl<T: ButtonContent + Sized> ButtonExt for T {
    fn button(self, key: MemoryKey) -> Button<Self> {
        Button::new(key, self)
    }
}

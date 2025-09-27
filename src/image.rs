use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable, shape::rectangle::Rectangle},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
};

// todo: split out image data from image

// im sorry
pub struct Image {
    width: u32,
    pixels: Vec<Rgb<f32>>,

    scale: f32,
}

impl Image {
    pub fn empty(size: Vector2<u32>) -> Self {
        Self {
            width: size.x,
            pixels: vec![Rgb::repeat(1.0); (size.x * size.y) as usize],

            scale: 6.0,
        }
    }

    pub fn set(&mut self, px: Vector2<u32>, color: Rgb<f32>) {
        self.pixels[(px.x + px.y * self.width) as usize] = color;
    }

    pub fn get(&self, px: Vector2<u32>) -> Rgb<f32> {
        self.pixels[(px.x + px.y * self.width) as usize]
    }
}

impl Drawable for Image {
    fn draw(self, ctx: &mut GraphicsContext) {
        for y in 0..(self.pixels.len() as u32 / self.width) {
            for x in 0..self.width {
                let pos = Vector2::new(x, y);
                Rectangle::new(Vector2::repeat(self.scale))
                    .position(pos.map(|x| x as f32) * self.scale, Anchor::BottomLeft)
                    .color(self.get(pos))
                    .draw(ctx);
            }
        }
    }
}

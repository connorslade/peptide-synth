use engine::{exports::nalgebra::Vector2, graphics_context::GraphicsContext};
use serde::Deserialize;

use crate::game::{amino::Amino, peptide::Peptide};

#[derive(Deserialize)]
pub struct Level {
    pub title: String,
    pub description: String,

    pub peptide: Peptide,
}

impl Level {
    pub fn get(&self, pos: Vector2<i32>) -> Option<&Amino> {
        self.peptide.get(pos)
    }

    pub fn render(&self, ctx: &mut GraphicsContext) -> Vector2<f32> {
        let size = (self.peptide.size() * 12 * 6).map(|x| x as f32);
        let center = Vector2::new(5.0 * 6.0 - size.x / 2.0, 0.0);
        let pos = center + Vector2::new(ctx.center().x, size.y / 2.0 + 16.0);
        self.peptide.render(ctx, pos);
        pos
    }
}

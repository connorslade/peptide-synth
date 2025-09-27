use engine::{exports::nalgebra::Vector2, graphics_context::GraphicsContext};

use crate::game::peptide::{Peptide, peptide};

pub struct Level {
    pub peptide: Peptide,
}

impl Level {
    pub fn example() -> Self {
        Self {
            peptide: peptide! {
                Arg at (0, 0) -> (Right),
                Leu at (1, 0) -> (Right),
                Asp at (2, 0) -> (Right),
                Ala at (3, 0) -> (Right),
                Cys at (4, 0),
            },
        }
    }

    pub fn render(&self, ctx: &mut GraphicsContext) {
        let size = (self.peptide.size() * 12 * 6).map(|x| x as f32);
        let center = Vector2::new(5.0 * 6.0 - size.x / 2.0, 0.0);
        let pos = center + Vector2::new(ctx.center().x, size.y / 2.0 + 16.0);
        self.peptide.render(ctx, pos);
    }
}

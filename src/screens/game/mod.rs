use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
};

use crate::{
    assets::{SELECTED, include_asset},
    game::{level::Level, peptide::Peptide, world_to_screen},
};

mod interface;
mod selection;

pub struct GameScreen {
    peptide: Peptide,
    level: Level,

    child_idx: u8,
    selected: Option<Vector2<i32>>,
}

impl GameScreen {
    pub fn new() -> Self {
        let level = ron::de::from_bytes(include_asset!("levels/test2.ron")).unwrap();

        Self {
            peptide: Peptide::for_level(&level),
            level,

            child_idx: 0,
            selected: None,
        }
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext) {
        self.interface(ctx);

        // Render the board and level peptides
        let level_origin = self.level.render(ctx);
        let hover = self.peptide.render(ctx, ctx.center());

        let mut remove = None;
        if let Some(pos) = hover
            && self.selected.is_none()
        {
            let path = self.peptide.path(pos);
            if let Some(level) = self.level.peptide.find(&path) {
                Sprite::new(SELECTED)
                    .scale(Vector2::repeat(6.0))
                    .position(level_origin + world_to_screen(level), Anchor::Center)
                    .draw(ctx);
            }

            let (left, right) = (
                ctx.input.mouse_pressed(MouseButton::Left),
                ctx.input.mouse_pressed(MouseButton::Right),
            );
            right.then(|| remove = Some(pos));
            left.then(|| self.selected = Some(pos));

            ctx.window.cursor(CursorIcon::Pointer);
            Sprite::new(SELECTED)
                .scale(Vector2::repeat(6.0))
                .position(ctx.center() + world_to_screen(pos), Anchor::Center)
                .z_index(1)
                .draw(ctx);
        }

        if let Some(pos) = remove {
            self.peptide.remove(pos);
        }

        self.selection(ctx, level_origin);
    }
}

use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
};
use rand::{rng, seq::IndexedRandom};

use crate::{
    assets::{SELECTED, include_asset},
    game::{
        amino::{Amino, AminoType},
        level::Level,
        peptide::Peptide,
        world_to_screen,
    },
    misc::{direction::Directions, exp_decay},
};

mod interface;
mod selection;

pub struct GameScreen {
    peptide: Peptide,
    level: Level,

    pan: Vector2<f32>,
    offset: Vector2<f32>,

    child_idx: u8,
    selected: Option<Vector2<i32>>,
}

impl GameScreen {
    pub fn new() -> Self {
        let level = ron::de::from_bytes::<Level>(include_asset!("levels/level_5.ron")).unwrap();
        println!("RANGE: {:?}", level.solve());

        Self {
            peptide: Peptide::for_level(&level),
            level,

            pan: Vector2::zeros(),
            offset: Vector2::zeros(),

            child_idx: 0,
            selected: None,
        }
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext) {
        if ctx.input.key_pressed(KeyCode::Space) {
            self.level.peptide.inner.clear();
            self.level.peptide.inner.insert(
                Vector2::repeat(0),
                Amino {
                    amino: *AminoType::ALL.choose(&mut rng()).unwrap(),
                    children: Directions::empty(),
                },
            );
            for _ in 0..10 {
                self.level.peptide.mutate();
            }
            dbg!(self.level.solve());
        }

        self.interface(ctx);

        if ctx.input.mouse_down(MouseButton::Middle) {
            ctx.window.cursor(CursorIcon::Move);
            self.pan += ctx.input.mouse_delta();
        }

        let offset_goal = self.peptide.offset_goal();
        self.offset.x = exp_decay(self.offset.x, offset_goal.x, 10.0, ctx.delta_time);
        self.offset.y = exp_decay(self.offset.y, -offset_goal.y, 10.0, ctx.delta_time);
        let origin = ctx.center() + self.offset + self.pan;

        // Render the board and level peptides
        let hover = self.peptide.render(ctx, origin, |_, s| s);
        let level_origin = self.level.render(ctx, &self.peptide);

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
                .position(origin + world_to_screen(pos), Anchor::Center)
                .z_index(1)
                .draw(ctx);
        }

        if let Some(pos) = remove {
            self.peptide.remove(pos);
        }

        self.selection(ctx, origin, level_origin);
    }
}

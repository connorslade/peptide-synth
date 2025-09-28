use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
};

use crate::{
    assets::SELECTED,
    game::{
        level::{LEVELS, Level},
        peptide::Peptide,
        world_to_screen,
    },
    misc::exp_decay,
};

mod interface;
mod selection;

pub struct GameScreen {
    peptide: Peptide,
    level: &'static Level,
    level_idx: usize,
    unlocked: usize,

    pan: Vector2<f32>,
    offset: Vector2<f32>,

    show_desc: bool,
    child_idx: u8,
    selected: Option<Vector2<i32>>,
}

impl GameScreen {
    pub fn new() -> Self {
        let level = &LEVELS[0];
        Self {
            peptide: Peptide::for_level(level),
            level,
            level_idx: 0,
            unlocked: 0,

            pan: Vector2::zeros(),
            offset: Vector2::zeros(),

            show_desc: true,
            child_idx: 0,
            selected: None,
        }
    }

    pub fn load_level(&mut self, idx: usize) {
        if idx >= LEVELS.len() || idx > self.unlocked {
            return;
        }

        let level = &LEVELS[idx];

        self.level = level;
        self.level_idx = idx;
        self.peptide = Peptide::for_level(level);
        self.pan = Vector2::zeros();
        self.offset = Vector2::zeros();
        self.child_idx = 0;
        self.selected = None;
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext) {
        self.interface(ctx);

        if ctx.input.mouse_down(MouseButton::Middle) {
            ctx.window.cursor(CursorIcon::Move);
            self.pan += ctx.input.mouse_delta();
        }

        let offset_goal = self.peptide.offset_goal();
        self.offset.x = exp_decay(self.offset.x, offset_goal.x, 10.0, ctx.delta_time);
        self.offset.y = exp_decay(self.offset.y, offset_goal.y, 10.0, ctx.delta_time);
        let origin = ctx.center() + self.offset + self.pan;

        // Render the board and level peptides
        let hover = self.peptide.render(ctx, origin, true, |_, s| s);
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

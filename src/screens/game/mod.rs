use std::collections::{HashSet, VecDeque};

use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
};

use crate::{
    assets::{SELECTED, include_asset},
    game::{amino::Amino, level::Level, peptide::Peptide, world_to_screen},
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

    queue: VecDeque<Peptide>,
    seen: HashSet<Peptide>,
    stats: (u32, f32, f32),
}

impl GameScreen {
    pub fn new() -> Self {
        let level = ron::de::from_bytes(include_asset!("levels/test.ron")).unwrap();
        let peptide = Peptide::for_level(&level);

        let mut queue = VecDeque::new();
        queue.push_back(peptide.clone());

        Self {
            peptide,
            level,

            pan: Vector2::zeros(),
            offset: Vector2::zeros(),

            child_idx: 0,
            selected: None,

            queue,
            seen: HashSet::new(),
            stats: (0, f32::MAX, f32::MIN),
        }
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext) {
        self.interface(ctx);

        while let Some(peptide) = self.queue.pop_front() {
            if !self.seen.insert(peptide.clone()) {
                continue;
            }

            self.peptide = peptide;
            for (amino, pos, direction) in self.level.options(&self.peptide) {
                let mut peptide = self.peptide.clone();
                peptide.inner.insert(
                    pos,
                    Amino {
                        amino,
                        children: Directions::empty() | direction,
                    },
                );
                self.queue.push_back(peptide);
            }

            if self.peptide.inner.len() == self.level.peptide.inner.len() {
                self.stats.0 += 1;
                let score = self.peptide.score();
                self.stats.1 = self.stats.1.min(score);
                self.stats.2 = self.stats.2.max(score);

                println!(
                    "#{} SCORE: {} | MIN: {} MAX: {}",
                    self.stats.0, score, self.stats.1, self.stats.2
                );
                break;
            }
        }

        if ctx.input.mouse_down(MouseButton::Middle) {
            ctx.window.cursor(CursorIcon::Move);
            self.pan += ctx.input.mouse_delta();
        }

        let offset_goal = self.peptide.offset_goal();
        self.offset.x = exp_decay(self.offset.x, offset_goal.x, 10.0, ctx.delta_time);
        self.offset.y = exp_decay(self.offset.y, -offset_goal.y, 10.0, ctx.delta_time);
        let origin = ctx.center() + self.offset + self.pan;

        // Render the board and level peptides
        let hover = self.peptide.render(ctx, origin);
        let level_origin = self.level.render(ctx);

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

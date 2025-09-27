use std::collections::hash_map::Entry;

use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
};

use crate::{
    assets::{GHOST, SELECTED},
    game::{
        amino::{Amino, AminoType},
        world_to_screen,
    },
    misc::direction::{Direction, Directions},
    screens::game::GameScreen,
};

impl GameScreen {
    pub fn selection(&mut self, ctx: &mut GraphicsContext, level_origin: Vector2<f32>) {
        let delta = ctx.input.scroll_delta() as i8;
        if delta > 0 {
            self.child_idx = (self.child_idx + 1) % 3;
        } else if delta < 0 {
            self.child_idx = (self.child_idx + 2) % 3
        }

        if ctx.input.mouse_pressed(MouseButton::Right) {
            self.selected = None;
            return;
        }

        let Some(selected) = self.selected else {
            return;
        };

        let path = self.peptide.path(selected);
        let Some(level_pos) = self.level.peptide.find(&path) else {
            self.selected = None;
            return;
        };

        Sprite::new(SELECTED)
            .scale(Vector2::repeat(6.0))
            .position(level_origin + world_to_screen(level_pos), Anchor::Center)
            .draw(ctx);

        let level = self.level.get(level_pos).unwrap();
        let dirs = level.children.iter();
        for next_dir in dirs.cycle().skip(self.child_idx as usize).take(4) {
            let next_pos = level_pos + next_dir.delta();
            let next = self.level.get(next_pos).unwrap();

            if self.max_children_of_type(level_pos, selected, next.amino) {
                continue;
            }

            self.render_ghost(ctx, level_origin, next_pos, selected, *next);
            return;
        }

        self.selected = None;
    }

    fn render_ghost(
        &mut self,
        ctx: &mut GraphicsContext,
        level_origin: Vector2<f32>,
        next_pos: Vector2<i32>,
        selected: Vector2<i32>,
        next: Amino,
    ) {
        Sprite::new(GHOST)
            .scale(Vector2::repeat(6.0))
            .position(level_origin + world_to_screen(next_pos), Anchor::Center)
            .draw(ctx);

        let dir = ((ctx.input.mouse() - ctx.center()) / 72.0 - selected.map(|x| x as f32))
            .map(|x| x.round() as i32);
        let dir = if dir.x.abs() > dir.y.abs() {
            Vector2::new(dir.x.signum(), 0)
        } else {
            Vector2::new(0, dir.y.signum())
        };

        let child = selected + dir;
        let clicked = ctx.input.mouse_pressed(MouseButton::Left);
        (clicked && child != selected).then(|| self.selected = None);
        if let Entry::Vacant(e) = self.peptide.inner.entry(child) {
            let render_pos = world_to_screen(child);
            Sprite::new(next.amino.asset())
                .scale(Vector2::repeat(6.0))
                .position(ctx.center() + render_pos, Anchor::Center)
                .draw(ctx);
            Sprite::new(GHOST)
                .scale(Vector2::repeat(6.0))
                .position(ctx.center() + render_pos, Anchor::Center)
                .z_index(1)
                .draw(ctx);

            if clicked {
                let direction = Direction::from_delta(dir).unwrap().opposite();
                let amino = Amino {
                    amino: next.amino,
                    children: Directions::empty() | direction,
                };
                e.insert(amino);
                self.selected = Some(child);
            }
        }
    }

    fn max_children_of_type(
        &self,
        level_pos: Vector2<i32>,
        pos: Vector2<i32>,
        amino: AminoType,
    ) -> bool {
        let level = self.level.get(level_pos).unwrap();
        let max = level
            .children
            .iter()
            .filter(|dir| {
                self.level
                    .get(level_pos + dir.delta())
                    .map(|cell| cell.amino)
                    == Some(amino)
            })
            .count();

        let current = Direction::ALL
            .iter()
            .filter(|dir| {
                let Some(child) = self.peptide.get(pos + dir.delta()) else {
                    return false;
                };
                child.children.contains(dir.opposite()) && child.amino == amino
            })
            .count();

        current >= max
    }
}

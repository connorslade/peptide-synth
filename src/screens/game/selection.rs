use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::GraphicsContext,
};

use crate::{
    assets::{GHOST, SELECTED},
    game::{amino::Amino, world_to_screen},
    misc::direction::{Direction, Directions},
    screens::game::GameScreen,
};

impl GameScreen {
    pub fn selection(
        &mut self,
        ctx: &mut GraphicsContext,
        origin: Vector2<f32>,
        level_origin: Vector2<f32>,
    ) {
        let delta = ctx.input.scroll_delta() as i8;
        if delta > 0 || ctx.input.key_pressed(KeyCode::ArrowRight) {
            self.child_idx = (self.child_idx + 1) % 3;
        } else if delta < 0 || ctx.input.key_pressed(KeyCode::ArrowLeft) {
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

            let max = self.level.peptide.children_of_type(level_pos, next.amino);
            let current = self.peptide.children_of_type(selected, next.amino);
            if current >= max {
                continue;
            }

            self.render_ghost(ctx, origin, level_origin, next_pos, selected, *next);
            return;
        }

        self.selected = None;
    }

    fn render_ghost(
        &mut self,
        ctx: &mut GraphicsContext,
        origin: Vector2<f32>,
        level_origin: Vector2<f32>,
        next_pos: Vector2<i32>,
        selected: Vector2<i32>,
        next: Amino,
    ) {
        Sprite::new(GHOST)
            .scale(Vector2::repeat(6.0))
            .position(level_origin + world_to_screen(next_pos), Anchor::Center)
            .draw(ctx);

        let dir = ((ctx.input.mouse() - origin) / 72.0 - selected.map(|x| x as f32))
            .map(|x| x.round() as i32);
        let dir = if dir.x.abs() > dir.y.abs() {
            Vector2::new(dir.x.signum(), 0)
        } else {
            Vector2::new(0, dir.y.signum())
        };

        let child = selected + dir;
        let clicked = ctx.input.mouse_pressed(MouseButton::Left);
        (clicked && child != selected).then(|| self.selected = None);
        if !self.peptide.inner.contains_key(&child) {
            let render_pos = world_to_screen(child);
            Sprite::new(next.amino.asset())
                .scale(Vector2::repeat(6.0))
                .position(origin + render_pos, Anchor::Center)
                .draw(ctx);
            Sprite::new(GHOST)
                .scale(Vector2::repeat(6.0))
                .position(origin + render_pos, Anchor::Center)
                .z_index(1)
                .draw(ctx);

            if clicked {
                (self.peptide.inner.get_mut(&selected).unwrap().children)
                    .set(Direction::from_delta(dir).unwrap());
                let amino = Amino {
                    amino: next.amino,
                    children: Directions::empty(),
                };
                self.peptide.inner.insert(child, amino);
                self.selected = Some(child);
            }
        }
    }
}

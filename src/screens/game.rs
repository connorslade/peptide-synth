use std::collections::{HashMap, VecDeque};

use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
};

use crate::{
    amino::{Amino, AminoType},
    assets::{CONNECTOR_H, CONNECTOR_V, GHOST, SELECTED},
    consts::colors,
    misc::direction::{Direction, Directions},
};

pub struct GameScreen {
    protein: HashMap<Vector2<i32>, Amino>,
    selected: Option<Vector2<i32>>,
}

impl GameScreen {
    pub fn new() -> Self {
        let mut protein = HashMap::new();
        protein.insert(
            Vector2::new(0, 0),
            Amino {
                amino: AminoType::Arg,
                children: Directions::empty() | Direction::Left | Direction::Right,
            },
        );
        protein.insert(
            Vector2::new(-1, 0),
            Amino {
                amino: AminoType::Leu,
                children: Directions::empty(),
            },
        );
        protein.insert(
            Vector2::new(1, 0),
            Amino {
                amino: AminoType::Pro,
                children: Directions::empty(),
            },
        );

        Self {
            protein,
            selected: None,
        }
    }

    fn remove(&mut self, pos: Vector2<i32>) {
        let mut queue = VecDeque::new();
        queue.push_back(pos);

        while let Some(next) = queue.pop_front() {
            self.protein.remove(&next);

            for dir in Direction::ALL {
                let pos = next + dir.delta();
                if let Some(child) = self.protein.get(&pos)
                    && child.children.contains(dir.opposite())
                {
                    queue.push_back(pos);
                }
            }
        }
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext) {
        ctx.background(colors::BACKGROUND);

        let mut remove = None;
        for (pos, amino) in self.protein.iter() {
            let render_pos = world_to_screen(*pos);
            let sprite = Sprite::new(amino.amino.asset())
                .scale(Vector2::repeat(6.0))
                .position(ctx.center() + render_pos, Anchor::Center);
            if sprite.is_hovered(ctx) {
                if ctx.input.mouse_pressed(MouseButton::Left) {
                    self.selected = Some(*pos)
                }
                if ctx.input.mouse_pressed(MouseButton::Right) {
                    remove = Some(*pos)
                }

                ctx.window.cursor(CursorIcon::Pointer);
                Sprite::new(SELECTED)
                    .scale(Vector2::repeat(6.0))
                    .position(ctx.center() + render_pos, Anchor::Center)
                    .z_index(1)
                    .draw(ctx);
            }
            sprite.draw(ctx);

            for dir in amino.children.iter() {
                let connector_offset = match dir {
                    Direction::Up => Vector2::y() * 6.5,
                    Direction::Down => -Vector2::y() * 5.5,
                    Direction::Left => -Vector2::x() * 6.0,
                    Direction::Right => Vector2::x() * 6.0,
                } * 6.0;

                Sprite::new([CONNECTOR_V, CONNECTOR_H][dir.horizontal() as usize])
                    .scale(Vector2::repeat(6.0))
                    .position(ctx.center() + render_pos + connector_offset, Anchor::Center)
                    .z_index(2)
                    .draw(ctx);
            }
        }

        if let Some(pos) = remove {
            self.remove(pos);
        }

        if let Some(selected) = self.selected {
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
            if !self.protein.contains_key(&child) {
                let render_pos = world_to_screen(child);
                Sprite::new(GHOST)
                    .scale(Vector2::repeat(6.0))
                    .position(ctx.center() + render_pos, Anchor::Center)
                    .draw(ctx);

                if clicked {
                    let direction = Direction::from_delta(dir).unwrap().opposite();
                    let amino = Amino {
                        amino: AminoType::Arg,
                        children: Directions::empty() | direction,
                    };
                    self.protein.insert(child, amino);
                }
            }
        }
    }
}

fn screen_to_world(screen: Vector2<f32>) -> Vector2<i32> {
    screen.map(|x| (x / 12.0 / 6.0).round() as i32)
}

fn world_to_screen(world: Vector2<i32>) -> Vector2<f32> {
    world.map(|x| (x * 12 * 6) as f32)
}

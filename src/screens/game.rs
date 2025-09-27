use std::collections::{HashMap, VecDeque, hash_map::Entry};

use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable, spacer::Spacer, sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
    layout::{
        Justify, LayoutElement, LayoutMethods, column::ColumnLayout, root::RootLayout,
        row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};

use crate::{
    amino::{Amino, AminoType},
    assets::{CONNECTOR_H, CONNECTOR_V, GHOST, SELECTED, UNDEAD_FONT},
    misc::direction::{Direction, Directions},
};

const DUMMY_DESCRIPTION: &str = "This is a very simple peptide, only consisting of three amino acids. The two charged amino acids should be kept as far apart as possible.";

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
                amino: AminoType::Ala,
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
        let mut root = RootLayout::new(Vector2::new(16.0, ctx.size().y - 16.0), Anchor::TopLeft);
        root.nest(ctx, ColumnLayout::new(16.0), |ctx, layout| {
            Text::new(UNDEAD_FONT, "Level One")
                .scale(Vector2::repeat(6.0))
                .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                .layout(ctx, layout);
            Text::new(UNDEAD_FONT, DUMMY_DESCRIPTION)
                .scale(Vector2::repeat(2.0))
                .max_width(480.0)
                .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                .layout(ctx, layout);
            Spacer::new_y(16.0).layout(ctx, layout);

            layout.nest(ctx, ColumnLayout::new(8.0), |ctx, layout| {
                for acid in AminoType::ALL {
                    let tracker = LayoutTracker::new(memory_key!(&acid));
                    tracker
                        .hovered(ctx)
                        .then(|| ctx.window.cursor(CursorIcon::Pointer));

                    RowLayout::new(16.0)
                        .justify(Justify::Center)
                        .tracked(tracker)
                        .show(ctx, layout, |ctx, layout| {
                            Sprite::new(acid.asset())
                                .scale(Vector2::repeat(6.0))
                                .layout(ctx, layout);
                            ColumnLayout::new(12.0).show(ctx, layout, |ctx, layout| {
                                Text::new(UNDEAD_FONT, acid.name())
                                    .scale(Vector2::repeat(3.0))
                                    .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                                    .layout(ctx, layout);
                                Text::new(UNDEAD_FONT, acid.desc())
                                    .scale(Vector2::repeat(3.0))
                                    .color(Rgb::hex(0x847e87))
                                    .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                                    .layout(ctx, layout);
                            });
                        });
                }
            });
        });

        root.draw(ctx);

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
            if let Entry::Vacant(e) = self.protein.entry(child) {
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
                    e.insert(amino);
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

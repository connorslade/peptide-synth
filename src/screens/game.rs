use std::{collections::hash_map::Entry, f32::EPSILON};

use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable, spacer::Spacer, sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
    layout::{
        Justify, LayoutElement, LayoutMethods, column::ColumnLayout, convenience::NoPaddingExt,
        root::RootLayout, row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};

use crate::{
    assets::{GHOST, SCORE_ARROW, SCORE_BAR, SELECTED, UNDEAD_FONT},
    game::{
        amino::{Amino, AminoType},
        level::Level,
        peptide::Peptide,
        world_to_screen,
    },
    misc::direction::{Direction, Directions},
};

const DUMMY_DESCRIPTION: &str = "This is a very simple peptide, only consisting of three amino acids. The two charged amino acids should be kept as far apart as possible.";

pub struct GameScreen {
    peptide: Peptide,
    level: Level,

    selected: Option<(Vector2<i32>, u8)>,
}

impl GameScreen {
    pub fn new() -> Self {
        let level = Level::example();

        Self {
            peptide: Peptide::for_level(&level),
            level,

            selected: None,
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

            let energy = self.peptide.score();
            let score = energy / -4.4;

            Sprite::new(SCORE_ARROW)
                .scale(Vector2::repeat(6.0))
                .position(Vector2::x() * score * 60.0 * 6.0, Anchor::BottomLeft)
                .no_padding()
                .layout(ctx, layout);
            Spacer::new_y(6.0).no_padding().layout(ctx, layout);
            RowLayout::new(16.0)
                .justify(Justify::Center)
                .show(ctx, layout, |ctx, layout| {
                    Sprite::new(SCORE_BAR)
                        .scale(Vector2::repeat(6.0))
                        .layout(ctx, layout);

                    let duration = 10.0 * score + EPSILON;
                    Text::new(UNDEAD_FONT, format!("{duration:.1} years"))
                        .scale(Vector2::repeat(3.0))
                        .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                        .layout(ctx, layout);
                });
            Spacer::new_y(8.0).layout(ctx, layout);

            layout.nest(ctx, ColumnLayout::new(8.0), |ctx, layout| {
                for acid in AminoType::ALL {
                    let tracker = LayoutTracker::new(memory_key!(&acid));
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

        let level_origin = self.level.render(ctx);
        let hover = self.peptide.render(ctx, ctx.center());
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
            left.then(|| self.selected = Some((pos, 0)));
            right.then(|| remove = Some(pos));

            ctx.window.cursor(CursorIcon::Pointer);
            let render_pos = world_to_screen(pos);
            Sprite::new(SELECTED)
                .scale(Vector2::repeat(6.0))
                .position(ctx.center() + render_pos, Anchor::Center)
                .z_index(1)
                .draw(ctx);
        }

        if let Some(pos) = remove {
            if pos == Vector2::zeros() {
                for dir in Direction::ALL {
                    self.peptide.remove(pos + dir.delta());
                }
            } else {
                self.peptide.remove(pos);
            }
        }

        if let Some((selected, next_idx)) = self.selected {
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
            let Some(next_dir) = level.parents.iter().skip(next_idx as usize).next() else {
                self.selected = None;
                return;
            };
            let next_pos = level_pos + next_dir.delta();
            let next = self.level.get(next_pos).unwrap();

            // abort if selected is already connected to the max number of this type of amino
            let max = level
                .parents
                .iter()
                .filter(|x| {
                    self.level.get(level_pos + x.delta()).map(|x| x.amino) == Some(next.amino)
                })
                .count();

            let current = Direction::ALL
                .iter()
                .filter(|x| {
                    let Some(child) = self.peptide.get(selected + x.delta()) else {
                        return false;
                    };
                    child.parents.contains(x.opposite()) && child.amino == next.amino
                })
                .count();

            if current >= max {
                self.selected = None;
                return;
            }

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
                Sprite::new(GHOST)
                    .scale(Vector2::repeat(6.0))
                    .position(ctx.center() + render_pos, Anchor::Center)
                    .draw(ctx);

                if clicked {
                    let direction = Direction::from_delta(dir).unwrap().opposite();
                    let amino = Amino {
                        amino: next.amino,
                        parents: Directions::empty() | direction,
                    };
                    e.insert(amino);
                }
            }
        }
    }
}

use std::collections::hash_map::Entry;

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
    assets::{GHOST, SELECTED, UNDEAD_FONT},
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

    selected: Option<Vector2<i32>>,
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

        self.level.render(ctx);
        let hover = self.peptide.render(ctx, ctx.center());
        if let Some(pos) = hover
            && self.selected.is_none()
        {
            let (left, right) = (
                ctx.input.mouse_pressed(MouseButton::Left),
                ctx.input.mouse_pressed(MouseButton::Right),
            );
            left.then(|| self.selected = Some(pos));
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
            if let Entry::Vacant(e) = self.peptide.inner.entry(child) {
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

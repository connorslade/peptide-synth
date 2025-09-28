use engine::{
    color::Rgb,
    drawable::{
        Anchor, Drawable,
        shape::{rectangle::Rectangle, rectangle_outline::RectangleOutline},
        spacer::Spacer,
        sprite::Sprite,
        text::Text,
    },
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        convenience::NoPaddingExt, root::RootLayout, row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};

use crate::{
    assets::{COLLAPSE, EX, EXPAND, LEFT_ARROW, RIGHT_ARROW, SCORE_ARROW, SCORE_BAR, UNDEAD_FONT},
    consts::SCREEN,
    game::{amino::AminoType, level::LEVELS},
    misc::{button::ButtonExt, exp_decay},
    screens::{
        Screen,
        game::{GameScreen, LevelStatus},
    },
};

impl GameScreen {
    pub fn interface(&mut self, ctx: &mut GraphicsContext) {
        let mut root = RootLayout::new(Vector2::new(16.0, ctx.size().y - 16.0), Anchor::TopLeft);
        let (mut close, mut win) = (false, false);

        ColumnLayout::new(16.0)
            .sized(ctx.size() - Vector2::x() * 32.0)
            .show(ctx, &mut root, |ctx, layout| {
                RowLayout::new(16.0)
                    .justify(Justify::Center)
                    .show(ctx, layout, |ctx, layout| {
                        Text::new(UNDEAD_FONT, &self.level.title)
                            .scale(Vector2::repeat(6.0))
                            .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                            .layout(ctx, layout);
                        Sprite::new(if self.show_desc { COLLAPSE } else { EXPAND })
                            .position(Vector2::y() * -6.0, Anchor::BottomLeft)
                            .scale(Vector2::repeat(3.0))
                            .button(memory_key!())
                            .on_click(ctx, || self.show_desc ^= true)
                            .layout(ctx, layout);
                        Spacer::new_x(16.0).no_padding().layout(ctx, layout);

                        RowLayout::new(8.0).direction(Direction::MaxToMin).show(
                            ctx,
                            layout,
                            |ctx, layout| {
                                let arrow = |sprite, color| {
                                    Sprite::new(sprite)
                                        .scale(Vector2::repeat(4.0))
                                        .color(color)
                                        .button(memory_key!(sprite))
                                };

                                arrow(EX, Rgb::repeat(1.0))
                                    .on_click(ctx, || close = true)
                                    .layout(ctx, layout);

                                match self.level_status {
                                    LevelStatus::Campaign {
                                        level_idx,
                                        unlocked,
                                    } => {
                                        let left_color = Rgb::repeat(1_f32)
                                            .lerp(Rgb::repeat(0.6), (level_idx == 0) as u8 as f32);
                                        let right_color = Rgb::repeat(1_f32).lerp(
                                            Rgb::repeat(0.6),
                                            (level_idx + 1 > unlocked) as u8 as f32,
                                        );

                                        arrow(RIGHT_ARROW, right_color)
                                            .on_click(ctx, || {
                                                if level_idx + 1 == LEVELS.len() {
                                                    win = true;
                                                } else {
                                                    self.load_level(level_idx + 1)
                                                }
                                            })
                                            .layout(ctx, layout);
                                        arrow(LEFT_ARROW, left_color)
                                            .on_click(ctx, || self.load_level(level_idx - 1))
                                            .layout(ctx, layout);
                                    }
                                    LevelStatus::Random { solved } => {
                                        let color = Rgb::repeat(1_f32)
                                            .lerp(Rgb::repeat(0.6), !solved as u8 as f32);
                                        arrow(RIGHT_ARROW, color)
                                            .on_click(ctx, || {
                                                solved.then(|| self.randomize());
                                            })
                                            .layout(ctx, layout);
                                    }
                                }

                                Spacer::new_x(layout.available().x)
                                    .no_padding()
                                    .layout(ctx, layout);
                            },
                        );
                    });

                if self.show_desc {
                    Text::new(UNDEAD_FONT, &self.level.description)
                        .scale(Vector2::repeat(2.0))
                        .max_width(530.0)
                        .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                        .layout(ctx, layout);
                }

                let energy = self.peptide.score();
                let range = self.level.range;
                let score = (energy - range.1) / (range.0 - range.1);

                if score >= 0.95 && self.peptide.inner.len() == self.level.peptide.inner.len() {
                    match &mut self.level_status {
                        LevelStatus::Campaign {
                            level_idx,
                            unlocked,
                        } => *unlocked = (*unlocked).max(*level_idx + 1),
                        LevelStatus::Random { solved } => *solved = true,
                    }
                }

                let offset_goal = score.clamp(0.0, 1.0) * 57.0 * 6.0;
                let offset = ctx.memory.get_or_insert(memory_key!(), offset_goal);
                *offset = exp_decay(*offset, offset_goal, 10.0, ctx.delta_time);
                Sprite::new(SCORE_ARROW)
                    .scale(Vector2::repeat(4.0))
                    .position(Vector2::x() * *offset, Anchor::BottomLeft)
                    .no_padding()
                    .layout(ctx, layout);
                Spacer::new_y(6.0).no_padding().layout(ctx, layout);
                RowLayout::new(16.0)
                    .justify(Justify::Center)
                    .show(ctx, layout, |ctx, layout| {
                        Sprite::new(SCORE_BAR)
                            .scale(Vector2::repeat(6.0))
                            .layout(ctx, layout);

                        let duration = if score >= 0.95 {
                            format!("{score:.1} decade{}", if score >= 1.05 { "s" } else { "" })
                        } else {
                            format!("{:.1} years", score * 10.0 + f32::EPSILON)
                        };
                        Text::new(UNDEAD_FONT, duration)
                            .scale(Vector2::repeat(3.0))
                            .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                            .layout(ctx, layout);
                    });
                Spacer::new_y(8.0).layout(ctx, layout);

                layout.nest(ctx, ColumnLayout::new(8.0), |ctx, layout| {
                    for acid in AminoType::ALL {
                        let tracker = LayoutTracker::new(memory_key!(&acid));
                        if tracker.hovered(ctx) {
                            let origin = ctx.input.mouse() + Vector2::repeat(16.0);
                            let text = Text::new(UNDEAD_FONT, acid.long_description())
                                .position(origin, Anchor::BottomLeft)
                                .scale(Vector2::repeat(2.0))
                                .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                                .z_index(2);
                            Rectangle::new(text.size(ctx) + Vector2::repeat(16.0))
                                .position(origin - Vector2::repeat(8.0), Anchor::BottomLeft)
                                .color(Rgb::hex(0x292845))
                                .z_index(1)
                                .draw(ctx);
                            RectangleOutline::new(text.size(ctx) + Vector2::repeat(16.0), 4.0)
                                .position(origin - Vector2::repeat(12.0), Anchor::BottomLeft)
                                .color(Rgb::hex(0x3f3f74))
                                .z_index(1)
                                .draw(ctx);
                            text.draw(ctx);
                        }

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
                                    Text::new(UNDEAD_FONT, acid.description())
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

        close.then(|| ctx.memory.insert(SCREEN, Screen::Title));
        win.then(|| ctx.memory.insert(SCREEN, Screen::Win));
    }
}

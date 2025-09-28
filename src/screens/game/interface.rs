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
        Justify, LayoutElement, LayoutMethods, column::ColumnLayout, convenience::NoPaddingExt,
        root::RootLayout, row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};

use crate::{
    assets::{SCORE_ARROW, SCORE_BAR, UNDEAD_FONT},
    game::amino::AminoType,
    misc::exp_decay,
    screens::game::GameScreen,
};

impl GameScreen {
    pub fn interface(&mut self, ctx: &mut GraphicsContext) {
        let mut root = RootLayout::new(Vector2::new(16.0, ctx.size().y - 16.0), Anchor::TopLeft);
        root.nest(ctx, ColumnLayout::new(16.0), |ctx, layout| {
            Text::new(UNDEAD_FONT, &self.level.title)
                .scale(Vector2::repeat(6.0))
                .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                .layout(ctx, layout);
            Text::new(UNDEAD_FONT, &self.level.description)
                .scale(Vector2::repeat(2.0))
                .max_width(480.0)
                .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
                .layout(ctx, layout);

            let energy = self.peptide.score();
            let range = self.level.range;
            let score = (energy - range.1) / (range.0 - range.1);

            let offset_goal = score.clamp(0.0, 1.0) * 60.0 * 6.0;
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

                    let duration = if score > 1.0 {
                        format!("{score:.1} decades")
                    } else {
                        format!("{:.1} years", score * 10.0)
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
    }
}

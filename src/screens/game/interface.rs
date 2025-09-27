use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable, spacer::Spacer, sprite::Sprite, text::Text},
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
            let score = energy / -4.4;
            let offset_goal = score * 60.0 * 6.0;

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

                    let duration = 10.0 * score + f32::EPSILON;
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
    }
}

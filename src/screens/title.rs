use std::f32::consts::{SQRT_2, TAU};

use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable, sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::{Justify, LayoutElement, LayoutMethods, column::ColumnLayout, root::RootLayout},
    memory_key,
};
use rand::{Rng, rng, seq::IndexedRandom};

use crate::{
    assets::{CAMPAIGN_BUTTON, QUIT_BUTTON, RANDOM_BUTTON, UNDEAD_FONT},
    consts::{LEVEL_STATUS, SCREEN},
    game::amino::AminoType,
    misc::button::ButtonExt,
    screens::{Screen, game::LevelStatus},
};

pub struct TitleScreen {
    elements: Vec<Element>,
}

struct Element {
    amino: AminoType,
    speed: f32,

    theta: f32,
    distance: f32,
}

impl TitleScreen {
    pub fn new() -> Self {
        let mut elements = Vec::new();

        let mut rng = rng();
        for _ in 0..25 {
            elements.push(Element {
                amino: *AminoType::ALL.choose(&mut rng).unwrap(),
                speed: rng.random_range(0.2..=0.4),
                theta: rng.random_range(0.0..=TAU),
                distance: rng.random_range(0_f32..=SQRT_2).sqrt(),
            });
        }

        Self { elements }
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext) {
        let (scale, pos) = title_layout(ctx, 15.0);
        Text::new(UNDEAD_FONT, "Peptide Synth")
            .scale(Vector2::repeat(1.5 * scale.round()))
            .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
            .position(pos, Anchor::TopCenter)
            .draw(ctx);

        Text::new(UNDEAD_FONT, "By Connor Slade")
            .scale(Vector2::repeat(4.0))
            .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
            .position(Vector2::new(ctx.size().x - 16.0, 16.0), Anchor::BottomRight)
            .color(Rgb::repeat(0.5))
            .draw(ctx);

        let (mut quit, mut game) = (false, None);
        let mut root = RootLayout::new(ctx.center(), Anchor::TopCenter);
        ColumnLayout::new(16.0)
            .justify(Justify::Center)
            .show(ctx, &mut root, |ctx, layout| {
                Sprite::new(CAMPAIGN_BUTTON)
                    .scale(Vector2::repeat(6.0))
                    .button(memory_key!())
                    .scale_effect()
                    .on_click(ctx, || {
                        game = Some(LevelStatus::Campaign {
                            level_idx: 0,
                            unlocked: 0,
                        })
                    })
                    .layout(ctx, layout);
                Sprite::new(RANDOM_BUTTON)
                    .scale(Vector2::repeat(6.0))
                    .button(memory_key!())
                    .scale_effect()
                    .on_click(ctx, || {
                        game = Some(LevelStatus::Random {
                            solved: false,
                            count: 0,
                            generator: None,
                            next_generator: None,
                        })
                    })
                    .layout(ctx, layout);
                Sprite::new(QUIT_BUTTON)
                    .scale(Vector2::repeat(6.0))
                    .button(memory_key!())
                    .scale_effect()
                    .on_click(ctx, || quit = true)
                    .layout(ctx, layout);
            });
        root.draw(ctx);
        quit.then(|| ctx.window.close());
        if let Some(status) = game {
            ctx.memory.insert(SCREEN, Screen::Game);
            ctx.memory.insert(LEVEL_STATUS, Some(status));
        }

        for element in self.elements.iter_mut() {
            element.theta += element.speed * ctx.delta_time;

            let pos = (Vector2::new(element.theta.cos(), element.theta.sin()) * element.distance)
                .map(|x| x / 2.0 + 0.5)
                .component_mul(&ctx.size());
            Sprite::new(element.amino.asset())
                .scale(Vector2::repeat(6.0))
                .position(pos, Anchor::Center)
                .z_index(-1)
                .draw(ctx);
        }
    }
}

fn title_layout(ctx: &GraphicsContext, max_scale: f32) -> (f32, Vector2<f32>) {
    let size = ctx.size();

    let (x_scale, y_scale) = (size.x / 160.0, size.y / 70.0);
    let scale = (x_scale).min(y_scale).clamp(4.0, max_scale);

    let y_offset = (y_scale.min(max_scale) - 3.0) * 16.0;
    let pos = Vector2::new(size.x / 2.0, size.y - y_offset);

    (scale, pos)
}

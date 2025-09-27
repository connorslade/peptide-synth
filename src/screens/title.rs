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
    assets::{CAMPAIGN_BUTTON, QUIT_BUTTON, UNDEAD_FONT},
    consts::SCREEN,
    game::amino::AminoType,
    misc::button::ButtonExt,
    screens::Screen,
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
        Text::new(UNDEAD_FONT, "Peptide Synth")
            .scale(Vector2::repeat(16.0))
            .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
            .position(
                ctx.size().component_mul(&Vector2::new(0.5, 0.85)),
                Anchor::TopCenter,
            )
            .draw(ctx);

        let (mut quit, mut game) = (false, false);
        let mut root = RootLayout::new(ctx.center(), Anchor::TopCenter);
        ColumnLayout::new(16.0)
            .justify(Justify::Center)
            .show(ctx, &mut root, |ctx, layout| {
                Sprite::new(CAMPAIGN_BUTTON)
                    .scale(Vector2::repeat(6.0))
                    .button(memory_key!())
                    .on_click(ctx, || game = true)
                    .layout(ctx, layout);
                Sprite::new(QUIT_BUTTON)
                    .scale(Vector2::repeat(6.0))
                    .button(memory_key!())
                    .on_click(ctx, || quit = true)
                    .layout(ctx, layout);
            });
        root.draw(ctx);
        quit.then(|| ctx.window.close());
        game.then(|| ctx.memory.insert(SCREEN, Screen::Game));

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

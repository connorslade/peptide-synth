#![feature(decl_macro)]

use engine::{
    application::{Application, ApplicationArgs},
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::{nalgebra::Vector2, winit::window::WindowAttributes},
};

use crate::{assets::AMINO_R, consts::colors};

mod assets;
mod consts;

fn main() {
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default(),
        asset_constructor: Box::new(|ctx| assets::init(ctx)),
        resumed: Box::new(|| {
            Box::new(|ctx| {
                ctx.background(colors::BACKGROUND);

                Sprite::new(AMINO_R)
                    .scale(Vector2::repeat(6.0))
                    .position(ctx.input.mouse(), Anchor::Center)
                    .draw(ctx);
            })
        }),
        multisample: None,
    })
    .run()
    .unwrap();
}

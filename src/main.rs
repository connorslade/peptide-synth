#![feature(decl_macro)]

use engine::{
    application::{Application, ApplicationArgs},
    exports::winit::window::WindowAttributes,
};

use crate::screens::game::GameScreen;

mod amino;
mod assets;
mod consts;
mod image;
mod misc;
mod screens;

fn main() {
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default().with_title("Protein Folding"),
        asset_constructor: Box::new(assets::init),
        resumed: Box::new(|| {
            let mut screen = GameScreen::new();
            Box::new(move |ctx| screen.render(ctx))
        }),
        multisample: None,
    })
    .run()
    .unwrap();
}

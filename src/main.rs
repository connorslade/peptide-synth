#![feature(decl_macro)]

use engine::{
    application::{Application, ApplicationArgs},
    exports::winit::{dpi::PhysicalSize, window::WindowAttributes},
};

use crate::{
    consts::{SCREEN, colors},
    screens::{Screen, game::GameScreen, title::TitleScreen},
};

mod assets;
mod consts;
mod game;
mod misc;
mod screens;

fn main() {
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default()
            .with_title("Peptide Synth")
            .with_inner_size(PhysicalSize::new(1920 * 3 / 4, 1080 * 3 / 4)),
        asset_constructor: Box::new(assets::init),
        resumed: Box::new(|| {
            let mut title = TitleScreen::new();
            let mut game = GameScreen::new();

            Box::new(move |ctx| {
                ctx.window.vsync(true);
                ctx.background(colors::BACKGROUND);
                let screen = ctx.memory.get_or_insert(SCREEN, Screen::Title);

                match screen {
                    Screen::Title => title.render(ctx),
                    Screen::Game => game.render(ctx),
                }
            })
        }),
        multisample: None,
    })
    .run()
    .unwrap();
}

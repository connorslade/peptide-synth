#![feature(decl_macro)]

use std::collections::HashMap;

use engine::{
    application::{Application, ApplicationArgs},
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::{
        nalgebra::Vector2,
        winit::window::{CursorIcon, WindowAttributes},
    },
};

use crate::{
    amino::AminoType,
    assets::{CONNECTOR_H, CONNECTOR_V, SELECTED},
    consts::colors,
    misc::direction::{Direction, Directions},
};

mod amino;
mod assets;
mod consts;
mod misc;
use amino::Amino;

fn main() {
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default(),
        asset_constructor: Box::new(assets::init),
        resumed: Box::new(|| {
            let mut protein = HashMap::new();
            protein.insert(
                Vector2::new(0, 0),
                Amino {
                    amino: AminoType::Arg,
                    children: Directions::empty() | Direction::Left | Direction::Right,
                },
            );
            protein.insert(
                Vector2::new(-1, 0),
                Amino {
                    amino: AminoType::Leu,
                    children: Directions::empty(),
                },
            );
            protein.insert(
                Vector2::new(1, 0),
                Amino {
                    amino: AminoType::Pro,
                    children: Directions::empty(),
                },
            );

            Box::new(move |ctx| {
                ctx.background(colors::BACKGROUND);

                for (pos, amino) in protein.iter() {
                    let pos = pos.map(|x| (x * 12 * 6) as f32);

                    let sprite = Sprite::new(amino.amino.asset())
                        .scale(Vector2::repeat(6.0))
                        .position(ctx.center() + pos, Anchor::Center);
                    if sprite.is_hovered(ctx) {
                        ctx.window.cursor(CursorIcon::Pointer);
                        Sprite::new(SELECTED)
                            .scale(Vector2::repeat(6.0))
                            .position(ctx.center() + pos, Anchor::Center)
                            .z_index(1)
                            .draw(ctx);
                    }
                    sprite.draw(ctx);

                    for dir in amino.children.iter() {
                        let connector_offset = match dir {
                            Direction::Up => Vector2::y() * 6.5,
                            Direction::Down => -Vector2::y() * 5.5,
                            Direction::Left => -Vector2::x() * 6.0,
                            Direction::Right => Vector2::x() * 6.0,
                        } * 6.0;

                        Sprite::new([CONNECTOR_V, CONNECTOR_H][dir.horizontal() as usize])
                            .scale(Vector2::repeat(6.0))
                            .position(ctx.center() + pos + connector_offset, Anchor::Center)
                            .z_index(2)
                            .draw(ctx);
                    }
                }
            })
        }),
        multisample: None,
    })
    .run()
    .unwrap();
}

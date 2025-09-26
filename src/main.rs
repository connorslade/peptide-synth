#![feature(decl_macro)]

use std::collections::VecDeque;

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
    misc::direction::Direction,
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
            let protein = Amino {
                amino: AminoType::Arg,
                pos: Direction::Down, // ignored
                children: vec![Amino {
                    amino: AminoType::Leu,
                    pos: Direction::Right,
                    children: vec![
                        Amino {
                            amino: AminoType::Pro,
                            pos: Direction::Up,
                            children: vec![],
                        },
                        Amino {
                            amino: AminoType::Leu,
                            pos: Direction::Down,
                            children: vec![Amino {
                                amino: AminoType::Pro,
                                pos: Direction::Left,
                                children: vec![],
                            }],
                        },
                    ],
                }],
            };

            Box::new(move |ctx| {
                ctx.background(colors::BACKGROUND);

                let mut queue = VecDeque::new();
                queue.push_back((&protein, -protein.pos.delta()));

                while let Some((next, offset)) = queue.pop_front() {
                    let offset = offset + next.pos.delta();
                    let pos = offset.map(|x| (x * 12 * 6) as f32);

                    let amino = Sprite::new(next.amino.asset())
                        .scale(Vector2::repeat(6.0))
                        .position(ctx.center() + pos, Anchor::Center);
                    if amino.is_hovered(ctx) {
                        ctx.window.cursor(CursorIcon::Pointer);
                        Sprite::new(SELECTED)
                            .scale(Vector2::repeat(6.0))
                            .position(ctx.center() + pos, Anchor::Center)
                            .z_index(1)
                            .draw(ctx);
                    }
                    amino.draw(ctx);

                    for child in next.children.iter() {
                        let connector_offset = match child.pos {
                            Direction::Up => Vector2::y() * 6.5,
                            Direction::Down => -Vector2::y() * 5.5,
                            Direction::Left => -Vector2::x() * 6.0,
                            Direction::Right => Vector2::x() * 6.0,
                        } * 6.0;

                        Sprite::new([CONNECTOR_V, CONNECTOR_H][child.pos.horizontal() as usize])
                            .scale(Vector2::repeat(6.0))
                            .position(ctx.center() + pos + connector_offset, Anchor::Center)
                            .z_index(2)
                            .draw(ctx);

                        queue.push_back((child, offset));
                    }
                }
            })
        }),
        multisample: None,
    })
    .run()
    .unwrap();
}

use std::{
    collections::{HashMap, VecDeque},
    i32,
};

use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
};

use crate::{
    assets::{CONNECTOR_H, CONNECTOR_V},
    game::{amino::Amino, level::Level, world_to_screen},
    misc::direction::{Direction, Directions},
};

pub struct Peptide {
    pub inner: HashMap<Vector2<i32>, Amino>,
}

impl Peptide {
    pub fn remove(&mut self, pos: Vector2<i32>) {
        let mut queue = VecDeque::new();
        queue.push_back(pos);

        while let Some(next) = queue.pop_front() {
            self.inner.remove(&next);

            for dir in Direction::ALL {
                let pos = next + dir.delta();
                if let Some(child) = self.inner.get(&pos)
                    && child.children.contains(dir.opposite())
                {
                    queue.push_back(pos);
                }
            }
        }
    }

    pub fn size(&self) -> Vector2<u32> {
        let mut min = Vector2::repeat(i32::MAX);
        let mut max = Vector2::repeat(i32::MIN);

        for pos in self.inner.keys() {
            min.x = min.x.min(pos.x);
            max.x = max.x.max(pos.x);
            min.y = min.y.min(pos.y);
            max.y = max.y.max(pos.y);
        }

        Vector2::new(max.x - min.x + 1, max.y - min.y + 1).map(|x| x as u32)
    }

    pub fn render(&self, ctx: &mut GraphicsContext, origin: Vector2<f32>) -> Option<Vector2<i32>> {
        let mut hover = None;
        for (pos, amino) in self.inner.iter() {
            let render_pos = world_to_screen(*pos);
            let sprite = Sprite::new(amino.amino.asset())
                .scale(Vector2::repeat(6.0))
                .position(origin + render_pos, Anchor::Center);
            sprite.is_hovered(ctx).then(|| hover = Some(*pos));
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
                    .position(origin + render_pos + connector_offset, Anchor::Center)
                    .z_index(2)
                    .draw(ctx);
            }
        }

        hover
    }
}

impl Peptide {
    pub fn for_level(level: &Level) -> Self {
        let mut inner = HashMap::new();
        let mut amino = *level.peptide.inner.get(&Vector2::zeros()).unwrap();
        amino.children = Directions::empty();
        inner.insert(Vector2::zeros(), amino);
        Self { inner }
    }
}

pub macro peptide($($aa:ident at ($x:expr, $y:expr) $(-> ($($dir:ident),*))?),* $(,)?) {{
    use crate::{
        game::amino::{Amino, AminoType},
        misc::direction::{Direction, Directions},
    };
    use engine::exports::nalgebra::Vector2;
    use std::collections::HashMap;

    let mut inner = HashMap::new();

    $(
        inner.insert(
            Vector2::new($x, $y),
            Amino {
                amino: AminoType::$aa,
                children: Directions::empty()$($(| Direction::$dir.into())*)?,
            }
        );
    )*

    crate::game::peptide::Peptide { inner }
}}

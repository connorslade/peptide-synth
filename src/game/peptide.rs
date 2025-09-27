use std::{
    collections::{HashMap, VecDeque},
    i32,
};

use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
};
use serde::Deserialize;

use crate::{
    assets::{CONNECTOR_H, CONNECTOR_V},
    game::{
        amino::{Amino, AminoType},
        level::Level,
        world_to_screen,
    },
    misc::direction::{Direction, Directions},
};

#[derive(Deserialize)]
pub struct Peptide {
    pub inner: HashMap<Vector2<i32>, Amino>,
}

impl Peptide {
    pub fn for_level(level: &Level) -> Self {
        let mut inner = HashMap::new();
        let mut amino = *level.peptide.inner.get(&Vector2::zeros()).unwrap();
        amino.children = Directions::empty();
        inner.insert(Vector2::zeros(), amino);
        Self { inner }
    }

    pub fn get(&self, pos: Vector2<i32>) -> Option<&Amino> {
        self.inner.get(&pos)
    }

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

    pub fn path(&self, pos: Vector2<i32>) -> Vec<AminoType> {
        let mut queue = VecDeque::new();
        queue.push_back((pos, Vec::new()));

        while let Some((pos, mut history)) = queue.pop_front() {
            if pos == Vector2::zeros() {
                history.reverse();
                return history;
            }

            let Some(amino) = self.inner.get(&pos) else {
                continue;
            };

            for dir in amino.children.iter() {
                let pos = pos + dir.delta();
                let mut history = history.clone();
                history.push(amino.amino);
                queue.push_back((pos, history));
            }
        }

        vec![]
    }

    pub fn find(&self, path: &[AminoType]) -> Option<Vector2<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back((Vector2::zeros(), Vec::new()));

        while let Some((pos, history)) = queue.pop_front() {
            if history == path {
                return Some(pos);
            }

            if history.len() >= path.len() {
                continue;
            }

            let Some(amino) = self.inner.get(&pos) else {
                continue;
            };

            for dir in amino.children.iter() {
                let next_pos = pos + dir.delta();
                if let Some(next_amino) = self.inner.get(&next_pos) {
                    let mut new_history = history.clone();
                    new_history.push(next_amino.amino);
                    queue.push_back((next_pos, new_history));
                }
            }
        }

        None
    }

    pub fn score(&self) -> f32 {
        let mut energy = 0.0;

        for (pos, amino) in &self.inner {
            for dir in Direction::ALL {
                let Some(neighbor) = self.get(pos + dir.delta()) else {
                    continue;
                };

                let adjacency = amino.amino.adjacency();
                if let Some((_, bouns)) = adjacency.iter().find(|x| x.0 == neighbor.amino)
                    && !neighbor.children.contains(dir.opposite())
                    && !amino.children.contains(dir)
                {
                    energy += bouns / 2.0;
                }

                energy += amino.amino.hydrophobic();
            }

            // electrostatics, q₁q₂/r
            for (pos_b, amino_b) in &self.inner {
                if pos == pos_b {
                    continue;
                }
                energy += (amino.amino.charge() * amino_b.amino.charge()) as f32
                    / (pos - pos_b).map(|x| x as f32).magnitude();
            }
        }

        energy
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
                parents: Directions::empty()$($(| Direction::$dir.into())*)?,
            }
        );
    )*

    crate::game::peptide::Peptide { inner }
}}

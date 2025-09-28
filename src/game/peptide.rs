use std::{
    collections::{HashMap, VecDeque},
    hash::{Hash, Hasher},
};

use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
};
use rand::{
    rng,
    seq::{IndexedRandom, IteratorRandom},
};
use serde::Deserialize;

use crate::{
    assets::{CONNECTOR_H, CONNECTOR_V, INTERACTION_H, INTERACTION_V},
    game::{
        amino::{Amino, AminoType},
        level::Level,
        world_to_screen,
    },
    misc::direction::{Direction, Directions},
};

const CONNECTOR_OFFSETS: [Vector2<f32>; 4] = [
    Vector2::new(0.0, 6.5),
    Vector2::new(0.0, -5.5),
    Vector2::new(-6.0, 0.0),
    Vector2::new(6.0, 0.0),
];

#[derive(Deserialize, PartialEq, Eq, Clone)]
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

    pub fn children_of_type(&self, pos: Vector2<i32>, amino: AminoType) -> u8 {
        let mut count = 0;
        let this = self.get(pos).unwrap();

        for dir in this.children.iter() {
            let Some(child) = self.get(pos + dir.delta()) else {
                continue;
            };

            count += (child.amino == amino) as u8;
        }

        count
    }

    pub fn parent(&self, pos: Vector2<i32>) -> Option<(Vector2<i32>, Direction)> {
        for dir in Direction::ALL {
            let pos = pos + dir.delta();
            let Some(next) = self.get(pos) else {
                continue;
            };

            if next.children.contains(dir.opposite()) {
                return Some((pos, dir));
            }
        }

        None
    }

    pub fn remove(&mut self, pos: Vector2<i32>) {
        if pos == Vector2::zeros() {
            (Direction::ALL.iter()).for_each(|dir| self.remove(pos + dir.delta()));
            return;
        }

        if let Some((parent, dir)) = self.parent(pos) {
            (self.inner.get_mut(&parent).unwrap().children).remove(dir.opposite());
        }

        let mut queue = VecDeque::new();
        queue.push_back(pos);

        while let Some(pos) = queue.pop_front() {
            let Some(next) = self.get(pos) else {
                continue;
            };

            for dir in next.children.iter() {
                queue.push_back(pos + dir.delta());
            }

            self.inner.remove(&pos);
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

            for dir in Direction::ALL {
                let pos = pos + dir.delta();
                let Some(child) = self.get(pos) else {
                    continue;
                };

                if !child.children.contains(dir.opposite()) {
                    continue;
                }

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
            energy += amino.amino.intrinsic_cost() as f32;

            let mut covered_sides = 0;
            for dir in Direction::ALL {
                let Some(neighbor) = self.get(pos + dir.delta()) else {
                    continue;
                };

                covered_sides += 1;
                if !neighbor.children.contains(dir.opposite()) && !amino.children.contains(dir) {
                    let adjacency = amino.amino.adjacency();
                    if let Some((_, bouns)) = adjacency.iter().find(|x| x.0 == neighbor.amino) {
                        energy += *bouns as f32 / 2.0;
                    }
                }
            }

            energy += (amino.amino.hydrophobic() * (4 - covered_sides)) as f32;

            // electrostatics, q₁q₂/r
            for (pos_b, amino_b) in &self.inner {
                if pos == pos_b {
                    continue;
                }

                let delta = pos - pos_b;
                let distance = delta.x.abs() + delta.y.abs();
                energy += (amino.amino.charge() * amino_b.amino.charge()) as f32 / distance as f32;
            }
        }

        energy
    }

    pub fn mutate(&mut self) {
        let mut rng = rng();

        loop {
            let pos = *self.inner.keys().choose(&mut rng).unwrap();
            let dir = Direction::ALL
                .choose_weighted(&mut rng, |dir| if *dir == Direction::Right { 4 } else { 1 })
                .unwrap();
            let next = pos + dir.delta();

            if self.inner.contains_key(&next) {
                continue;
            }

            let amino = Amino {
                amino: *AminoType::ALL.choose(&mut rng).unwrap(),
                children: Directions::empty(),
            };

            self.inner.get_mut(&pos).unwrap().children.set(*dir);
            self.inner.insert(next, amino);
            break;
        }
    }

    pub fn render(
        &self,
        ctx: &mut GraphicsContext,
        origin: Vector2<f32>,
        interactions: bool,
        callback: impl Fn(&Vector2<i32>, Sprite) -> Sprite,
    ) -> Option<Vector2<i32>> {
        let mut hover = None;
        for (pos, amino) in self.inner.iter() {
            let render_pos = world_to_screen(*pos);
            let sprite = Sprite::new(amino.amino.asset())
                .scale(Vector2::repeat(6.0))
                .position(origin + render_pos, Anchor::Center);
            let sprite = callback(pos, sprite);
            sprite.is_hovered(ctx).then(|| hover = Some(*pos));
            sprite.draw(ctx);

            for dir in amino.children.iter() {
                let connector_offset = CONNECTOR_OFFSETS[dir as usize] * 6.0;
                Sprite::new([CONNECTOR_V, CONNECTOR_H][dir.horizontal() as usize])
                    .scale(Vector2::repeat(6.0))
                    .position(origin + render_pos + connector_offset, Anchor::Center)
                    .z_index(2)
                    .draw(ctx);
            }

            if !interactions {
                continue;
            }

            for dir in Direction::ALL {
                if amino.children.contains(dir) {
                    continue;
                }

                let Some(neighbor) = self.get(pos + dir.delta()) else {
                    continue;
                };

                if neighbor.children.contains(dir.opposite()) {
                    continue;
                }

                let interactions = amino.amino.adjacency();
                if let Some((_, _)) = interactions.iter().find(|x| x.0 == neighbor.amino) {
                    let connector_offset = CONNECTOR_OFFSETS[dir as usize] * 6.0;
                    Sprite::new([INTERACTION_V, INTERACTION_H][dir.horizontal() as usize])
                        .scale(Vector2::repeat(6.0))
                        .position(origin + render_pos + connector_offset, Anchor::Center)
                        .z_index(2)
                        .draw(ctx);
                }
            }
        }

        hover
    }

    pub fn offset_goal(&self) -> Vector2<f32> {
        let size = (self.size() * 12 * 6).map(|x| x as f32);
        Vector2::new(5.0 * 6.0 - size.x / 2.0, size.y / 2.0)
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

// its gross but it works...
impl Hash for Peptide {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut items = self.inner.iter().collect::<Vec<_>>();
        items.sort_by(|a, b| a.0.x.cmp(&b.0.x).then(a.0.y.cmp(&b.0.y)));
        items.hash(state);
    }
}

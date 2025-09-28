use std::collections::{HashSet, VecDeque};

use engine::{color::Rgb, exports::nalgebra::Vector2, graphics_context::GraphicsContext};
use serde::Deserialize;

use crate::{
    game::{
        amino::{Amino, AminoType},
        peptide::Peptide,
    },
    misc::direction::{Direction, Directions},
};

#[derive(Deserialize)]
pub struct Level {
    pub title: String,
    pub description: String,
    pub range: (f32, f32),

    pub peptide: Peptide,
}

impl Level {
    pub fn get(&self, pos: Vector2<i32>) -> Option<&Amino> {
        self.peptide.get(pos)
    }

    pub fn render(&self, ctx: &mut GraphicsContext, peptide: &Peptide) -> Vector2<f32> {
        let pos = self.peptide.offset_goal() + Vector2::new(ctx.center().x, 16.0);
        self.peptide.render(ctx, pos, |pos, sprite| {
            let path = self.peptide.path(*pos);
            if peptide.find(&path).is_some() {
                sprite.color(Rgb::hex(0x222034).lerp(Rgb::repeat(1.0), 0.6))
            } else {
                sprite
            }
        });
        pos
    }
}

impl Level {
    pub fn solve(&self) -> (f32, f32) {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(Peptide::for_level(self));

        let mut scores = Vec::new();

        let (mut min, mut max) = (f32::MAX, f32::MIN);
        while let Some(peptide) = queue.pop_front() {
            if !seen.insert(peptide.clone()) {
                continue;
            }

            for (amino, pos, dir) in self.options(&peptide) {
                let amino = Amino {
                    amino,
                    children: Directions::empty(),
                };

                let mut peptide = peptide.clone();
                (peptide.inner.get_mut(&(pos - dir.delta())).unwrap())
                    .children
                    .set(dir);
                peptide.inner.insert(pos, amino);
                queue.push_back(peptide);
            }

            let score = peptide.score();
            max = max.max(score);
            if peptide.inner.len() == self.peptide.inner.len() {
                min = min.min(score);
                scores.push(score);
            }
        }

        println!("{scores:?}");

        (min, max)
    }

    // enumerates all possible amino acids that can be added to the peptide
    fn options(&self, peptide: &Peptide) -> Vec<(AminoType, Vector2<i32>, Direction)> {
        let mut out = Vec::new();

        for pos in peptide.inner.keys() {
            let path = peptide.path(*pos);
            let level_pos = self.peptide.find(&path).unwrap();
            let level = self.get(level_pos).unwrap();

            for child in level.children.iter() {
                let amino = self.get(level_pos + child.delta()).unwrap();

                let max = self.peptide.children_of_type(level_pos, amino.amino);
                let current = peptide.children_of_type(*pos, amino.amino);
                if current >= max {
                    continue;
                }

                for dir in Direction::ALL {
                    let next = pos + dir.delta();
                    if peptide.inner.contains_key(&next) {
                        continue;
                    }

                    out.push((amino.amino, next, dir));
                }
            }
        }

        out
    }
}

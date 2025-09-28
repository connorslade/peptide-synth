use engine::{exports::nalgebra::Vector2, graphics_context::GraphicsContext};
use serde::Deserialize;

use crate::{
    game::{
        amino::{Amino, AminoType},
        peptide::Peptide,
    },
    misc::direction::Direction,
};

#[derive(Deserialize)]
pub struct Level {
    pub title: String,
    pub description: String,

    pub peptide: Peptide,
}

impl Level {
    pub fn get(&self, pos: Vector2<i32>) -> Option<&Amino> {
        self.peptide.get(pos)
    }

    pub fn render(&self, ctx: &mut GraphicsContext) -> Vector2<f32> {
        let pos = self.peptide.offset_goal() + Vector2::new(ctx.center().x, 16.0);
        self.peptide.render(ctx, pos);
        pos
    }

    // enumerates all possible amino acids that can be added to the peptide
    pub fn options(&self, peptide: &Peptide) -> Vec<(AminoType, Vector2<i32>, Direction)> {
        let mut out = Vec::new();

        for pos in peptide.inner.keys() {
            let path = peptide.path(*pos);
            let level_pos = self.peptide.find(&path).unwrap();
            let level = self.get(level_pos).unwrap();

            for child in level.children.iter() {
                let amino = self.get(level_pos + child.delta()).unwrap();
                if enough_children_of_type(self, peptide, level_pos, *pos, amino.amino) {
                    continue;
                }

                for dir in Direction::ALL {
                    let next = pos + dir.delta();
                    if peptide.inner.contains_key(&next) {
                        continue;
                    }

                    out.push((amino.amino, next, dir.opposite()));
                }
            }
        }

        out
    }
}

fn enough_children_of_type(
    level: &Level,
    peptide: &Peptide,
    level_pos: Vector2<i32>,
    pos: Vector2<i32>,
    amino: AminoType,
) -> bool {
    let level_item = level.get(level_pos).unwrap();
    let max = level_item
        .children
        .iter()
        .filter(|dir| level.get(level_pos + dir.delta()).map(|cell| cell.amino) == Some(amino))
        .count();

    let current = Direction::ALL
        .iter()
        .filter(|dir| {
            let Some(child) = peptide.get(pos + dir.delta()) else {
                return false;
            };
            child.children.contains(dir.opposite()) && child.amino == amino
        })
        .count();

    current >= max
}

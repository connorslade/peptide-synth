use engine::assets::SpriteRef;

use crate::{assets, misc::direction::Direction};

pub struct Amino {
    pub amino: AminoType,
    pub pos: Direction,

    pub children: Vec<Amino>,
}

pub enum AminoType {
    Arg, // R
    Leu, // L
    Pro, // P
}

impl AminoType {
    pub fn asset(&self) -> SpriteRef {
        match self {
            AminoType::Arg => assets::AMINO_R,
            AminoType::Leu => assets::AMINO_L,
            AminoType::Pro => assets::AMINO_P,
        }
    }
}

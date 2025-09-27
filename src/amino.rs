use engine::assets::SpriteRef;

use crate::{assets, misc::direction::Directions};

pub struct Amino {
    pub amino: AminoType,
    pub children: Directions,
}

pub enum AminoType {
    Arg, // R
    Leu, // L
    Ala, // A
    Cys, // C
    Asp, // D
    Glu, // E
}

impl AminoType {
    pub const ALL: [AminoType; 6] = [
        AminoType::Arg,
        AminoType::Leu,
        AminoType::Ala,
        AminoType::Cys,
        AminoType::Asp,
        AminoType::Glu,
    ];

    pub fn name(&self) -> &str {
        match self {
            AminoType::Arg => "Arginine",
            AminoType::Leu => "Leucine",
            AminoType::Ala => "Alanine",
            AminoType::Cys => "Cysteine",
            AminoType::Asp => "Aspartate ",
            AminoType::Glu => "Glutamate",
        }
    }

    pub fn asset(&self) -> SpriteRef {
        match self {
            AminoType::Arg => assets::AMINO_R,
            AminoType::Leu => assets::AMINO_L,
            AminoType::Ala => assets::AMINO_A,
            AminoType::Cys => assets::AMINO_C,
            AminoType::Asp => assets::AMINO_D,
            AminoType::Glu => assets::AMINO_E,
        }
    }

    pub fn charge(&self) -> f32 {
        match self {
            AminoType::Arg => 12.5,
            AminoType::Leu => 0.0,
            AminoType::Ala => 0.0,
            AminoType::Cys => 0.0,
            AminoType::Asp => -1.0, //IDKKK
            AminoType::Glu => -1.0, //IDKKK
        }
    }
}

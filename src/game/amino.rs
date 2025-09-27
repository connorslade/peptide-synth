use engine::assets::SpriteRef;

use crate::{assets, misc::direction::Directions};

#[derive(Clone, Copy)]
pub struct Amino {
    pub amino: AminoType,
    pub parents: Directions,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum AminoType {
    Ala, // A
    Cys, // C
    Asp, // D
    Phe, // F
    Leu, // L
    Arg, // R
}

impl AminoType {
    pub const ALL: [AminoType; 6] = [
        AminoType::Ala,
        AminoType::Cys,
        AminoType::Asp,
        AminoType::Phe,
        AminoType::Leu,
        AminoType::Arg,
    ];

    pub fn letter(&self) -> char {
        match self {
            AminoType::Ala => 'A',
            AminoType::Cys => 'C',
            AminoType::Asp => 'D',
            AminoType::Phe => 'F',
            AminoType::Leu => 'L',
            AminoType::Arg => 'R',
        }
    }

    pub fn name(&self) -> &str {
        match self {
            AminoType::Arg => "Arginine",
            AminoType::Leu => "Leucine",
            AminoType::Ala => "Alanine",
            AminoType::Cys => "Cysteine",
            AminoType::Asp => "Aspartate ",
            AminoType::Phe => "Phenylalanine",
        }
    }

    pub fn desc(&self) -> String {
        let charge = self.charge();
        let adjacency = self.adjacency();

        let charge = if charge == 0 {
            ""
        } else if charge > 0 {
            "+∙"
        } else {
            "-∙"
        };
        let hydrophobic = if self.hydrophobic() < 0.0 {
            "δ∙"
        } else {
            ""
        };
        let adjacency = if adjacency.is_empty() {
            ""
        } else {
            let mut letters = adjacency
                .iter()
                .map(|x| x.0.letter())
                .collect::<Vec<char>>();
            letters.sort();
            &format!("{}∙", letters.iter().collect::<String>())
        };

        let mut desc = format!("{charge}{hydrophobic}{adjacency}");
        let _ = desc.pop();
        desc
    }

    pub fn asset(&self) -> SpriteRef {
        match self {
            AminoType::Ala => assets::AMINO_A,
            AminoType::Cys => assets::AMINO_C,
            AminoType::Asp => assets::AMINO_D,
            AminoType::Phe => assets::AMINO_F,
            AminoType::Leu => assets::AMINO_L,
            AminoType::Arg => assets::AMINO_R,
        }
    }

    pub fn charge(&self) -> i32 {
        match self {
            AminoType::Ala => 0,
            AminoType::Cys => 0,  //Brønsted acid
            AminoType::Asp => -1, //Brønsted base
            AminoType::Leu => 0,
            AminoType::Phe => 0,
            AminoType::Arg => 1, //Basic polar
        }
    }

    // Must not be adjacent in chain!!
    pub fn adjacency(&self) -> &[(AminoType, f32)] {
        match self {
            // Salt-bridge
            // Todo: -0.2 per additional pair
            AminoType::Arg => &[(AminoType::Asp, -2.0)],
            AminoType::Asp => &[(AminoType::Arg, -2.0)],

            AminoType::Cys => &[
                (AminoType::Cys, -3.0),
                (AminoType::Leu, -0.2),
                (AminoType::Phe, -0.2),
                (AminoType::Ala, -0.2),
            ], // Disulfide bond
            AminoType::Phe => &[
                (AminoType::Phe, -1.0),
                (AminoType::Leu, -0.2),
                (AminoType::Ala, -0.2),
                (AminoType::Cys, -0.2),
            ], // Aromatic stacking

            // Hydrophobic stacking
            AminoType::Ala => &[
                (AminoType::Leu, -0.2),
                (AminoType::Phe, -0.2),
                (AminoType::Cys, -0.2),
            ],
            AminoType::Leu => &[
                (AminoType::Ala, -0.2),
                (AminoType::Phe, -0.2),
                (AminoType::Cys, -0.2),
            ],
        }
    }

    // Stability per exposed side
    pub fn hydrophobic(&self) -> f32 {
        match self {
            AminoType::Ala => -0.3,
            AminoType::Leu => -0.6,
            AminoType::Phe => -0.6,
            AminoType::Cys => -0.2,
            _ => 0.0,
        }
    }
}

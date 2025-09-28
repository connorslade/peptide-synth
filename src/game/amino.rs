use engine::assets::SpriteRef;
use serde::{Deserialize, Deserializer};

use crate::{
    assets,
    misc::direction::{Direction, Directions},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize)]
pub struct Amino {
    pub amino: AminoType,
    #[serde(deserialize_with = "parse_directions")]
    pub children: Directions,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Deserialize)]
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

    pub fn description(&self) -> String {
        let (charge, hydrophobic, adjacency) =
            (self.charge(), self.hydrophobic(), self.adjacency());

        let hydrophobic = if hydrophobic == 0 {
            ""
        } else if hydrophobic < 0 {
            "δ∙"
        } else {
            "Ψ∙"
        };

        let charge = if charge == 0 {
            ""
        } else if charge > 0 {
            "+∙"
        } else {
            "-∙"
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

        let mut desc = format!("{hydrophobic}{charge}{adjacency}");
        let _ = desc.pop();
        desc
    }

    pub fn long_description(&self) -> String {
        let mut out = format!(
            "{}\n\nCost: {}\nCharge: {}\nHydrophobic: {}\nInteractions:",
            self.name(),
            self.intrinsic_cost(),
            self.charge(),
            self.hydrophobic()
        );
        for (amino, cost) in self.adjacency() {
            out.push_str(&format!("\n ∙ {}: {cost}", amino.letter()));
        }

        out
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

    pub fn intrinsic_cost(&self) -> i32 {
        match self {
            AminoType::Ala => 1,
            AminoType::Leu => 2,
            AminoType::Phe => 3,

            AminoType::Cys => 3,
            AminoType::Asp => 4,
            AminoType::Arg => 4,
        }
    }

    pub fn charge(&self) -> i32 {
        match self {
            AminoType::Ala => 0,
            AminoType::Cys => 0,
            AminoType::Asp => -1,
            AminoType::Leu => 0,
            AminoType::Phe => 0,
            AminoType::Arg => 1,
        }
    }

    // Must not be adjacent in chain!!
    pub fn adjacency(&self) -> &[(AminoType, i32)] {
        match self {
            // Salt-bridge
            AminoType::Arg => &[(AminoType::Asp, -10)],
            AminoType::Asp => &[(AminoType::Arg, -10)],

            // Disulfide
            AminoType::Cys => &[(AminoType::Cys, -12)],

            // Aromatic stacking
            AminoType::Phe => &[
                (AminoType::Phe, -6),
                (AminoType::Leu, -2),
                (AminoType::Cys, -2),
            ],

            AminoType::Leu => &[(AminoType::Leu, -4), (AminoType::Cys, -2)],
            AminoType::Ala => &[
                (AminoType::Leu, -2),
                (AminoType::Phe, -2),
                (AminoType::Cys, -2),
            ],
        }
    }

    // Energy per exposed side
    pub fn hydrophobic(&self) -> i32 {
        match self {
            AminoType::Leu => -2,
            AminoType::Phe => -2,
            AminoType::Cys => -1,
            AminoType::Ala => 0,
            AminoType::Asp => 1,
            AminoType::Arg => 1,
        }
    }
}

pub fn parse_directions<'de, D>(from: D) -> Result<Directions, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(from)?;

    let mut out = Directions::empty();
    for chr in str.chars() {
        out = out
            | match chr {
                'L' => Direction::Left,
                'R' => Direction::Right,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => panic!(),
            };
    }

    Ok(out)
}

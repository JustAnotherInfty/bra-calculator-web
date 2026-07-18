/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use crate::{length::Length, length_unit::LengthUnit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Country {
    UK,
    JP,
}

impl Country {
    pub fn unit(&self) -> LengthUnit {
        match self {
            Self::UK => LengthUnit::Inch,
            Self::JP => LengthUnit::Cm,
        }
    }

    pub fn band_step(&self) -> Length {
        match self {
            Self::UK => Length::new(2.0, self.unit()),
            Self::JP => Length::new(5.0, self.unit()),
        }
    }

    pub fn band_offset(&self) -> Length {
        match self {
            Self::UK => Length::new(4.0, self.unit()),
            Self::JP => Length::new(0.0, self.unit()),
        }
    }

    pub fn cup_step(&self) -> Length {
        match self {
            Self::UK => Length::new(1.0, self.unit()),
            Self::JP => Length::new(2.5, self.unit()),
        }
    }

    pub fn cup_offset(&self) -> Length {
        match self {
            Self::UK => Length::new(0.0, self.unit()),
            Self::JP => Length::new(-7.5, self.unit()),
        }
    }

    pub fn get_cup(&self, diff: Length) -> &'static str {
        let index = (diff + self.cup_offset())
            .divide_by(self.cup_step())
            .round() as i32;
        if index < -1 {
            "Too small"
        } else if index == -1 {
            "AAA"
        } else if let Some(cup) = self.get_cup_array().get(index as usize) {
            cup
        } else {
            "Too big"
        }
    }

    pub fn get_diff_from_cup(&self, cup: &str) -> Option<Length> {
        self.get_cup_array()
            .iter()
            .position(|x| *x == cup)
            .map(|index| index as f32 * self.cup_step() - self.cup_offset())
    }

    pub fn get_cup_array(&self) -> &'static [&'static str] {
        match self {
            Self::UK => &UK_CUPS,
            Self::JP => &JP_CUPS,
        }
    }
}

impl Country {
    pub fn self_array() -> &'static [Self] {
        &[Self::UK, Self::JP]
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::UK => "UK",
            Self::JP => "JP",
        }
    }
    pub fn id(&self) -> &'static str {
        match self {
            Self::UK => "uk",
            Self::JP => "jp",
        }
    }
    pub fn from_id(id: &str) -> Self {
        match id {
            "uk" => Self::UK,
            "jp" => Self::JP,
            _ => Self::default(),
        }
    }
    pub fn default() -> Self {
        Self::UK
    }
}

const UK_CUPS: [&str; 47] = [
    "AA", // 0
    "A",  // 1
    "B",  // 2
    "C",  // 3
    "D",  // 4
    "DD", // 5
    "E",  // 6
    "F",  // 7
    "FF", // 8
    "G",  // 9
    "GG", // 10
    "H",  // 11
    "HH", // 12
    "J",  // 13
    "JJ", // 14
    "K",  // 15
    "KK", // 16
    "L",  // 17
    "LL", // 18
    "M",  // 19
    "MM", // 20
    "N",  // 21
    "NN", // 22
    "O",  // 23
    "OO", // 24
    "P",  // 25
    "PP", // 26
    "Q",  // 27
    "QQ", // 28
    "R",  // 29
    "RR", // 30
    "S",  // 31
    "SS", // 32
    "T",  // 33
    "TT", // 34
    "U",  // 35
    "UU", // 36
    "V",  // 37
    "VV", // 38
    "W",  // 39
    "WW", // 40
    "X",  // 41
    "XX", // 42
    "Y",  // 43
    "YY", // 44
    "Z",  // 45
    "ZZ", // 46
];

const JP_CUPS: [&str; 27] = [
    "AA", // 0
    "A",  // 1
    "B",  // 2
    "C",  // 3
    "D",  // 4
    "E",  // 5
    "F",  // 6
    "G",  // 7
    "H",  // 8
    "I",  // 9
    "J",  // 10
    "K",  // 11
    "L",  // 12
    "M",  // 13
    "N",  // 14
    "O",  // 15
    "P",  // 16
    "Q",  // 17
    "R",  // 18
    "S",  // 19
    "T",  // 20
    "U",  // 21
    "V",  // 22
    "W",  // 23
    "X",  // 24
    "Y",  // 25
    "Z",  // 26
];

/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use crate::{length::Length, length_unit::LengthUnit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Country {
    UK,
}

impl Country {
    pub fn get_cup(&self, diff: Length) -> &'static str {
        let diff = diff.into_raw_unit(LengthUnit::Inch).round() as i32;
        if diff < -1 {
            "Too small"
        } else if diff == -1 {
            "AAA"
        } else if let Some(cup) = self.get_cup_array().get(diff as usize) {
            cup
        } else {
            "Too big"
        }
    }

    pub fn get_cup_array(&self) -> &'static [&'static str] {
        match self {
            Self::UK => &UK_CUPS,
        }
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

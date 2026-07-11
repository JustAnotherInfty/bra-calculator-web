/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use crate::length::Length;

pub enum Country {
    UK,
}

impl Country {
    pub fn get_cup(&self, diff: Length) -> &'static str {
        let diff = diff.into_raw_inch().round() as i32;
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
    "AA", "A", "B", "C", "D", "DD", "E", "F", "FF", "G", "GG", "H", "HH", "J", "JJ", "K", "KK",
    "L", "LL", "M", "MM", "N", "NN", "O", "OO", "P", "PP", "Q", "QQ", "R", "RR", "S", "SS", "T",
    "TT", "U", "UU", "V", "VV", "W", "WW", "X", "XX", "Y", "YY", "Z", "ZZ",
];

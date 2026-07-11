/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::ops::{Add, Div, Mul, Sub};

const CM_PER_INCH: f32 = 2.54;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Length {
    Cm(f32),
    Inch(f32),
}

impl Length {
    pub fn magnitude(self) -> f32 {
        match self {
            Self::Cm(cm) => cm,
            Self::Inch(inch) => inch,
        }
    }

    pub fn toggle_into_cm(&mut self) {
        let cm = match *self {
            Self::Cm(cm) => cm,
            Self::Inch(inch) => inch * CM_PER_INCH,
        };
        *self = Self::Cm(cm);
    }

    pub fn toggle_into_inch(&mut self) {
        let inch = match *self {
            Self::Cm(cm) => cm / CM_PER_INCH,
            Self::Inch(inch) => inch,
        };
        *self = Self::Inch(inch);
    }

    pub fn into_raw_cm(self) -> f32 {
        match self {
            Self::Cm(cm) => cm,
            Self::Inch(inch) => inch * CM_PER_INCH,
        }
    }

    pub fn into_raw_inch(self) -> f32 {
        match self {
            Self::Cm(cm) => cm / CM_PER_INCH,
            Self::Inch(inch) => inch,
        }
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Cm(a), Self::Cm(b)) => Self::Cm(a + b),
            (Self::Cm(a), Self::Inch(b)) => Self::Cm(a + b * CM_PER_INCH),
            (Self::Inch(a), Self::Cm(b)) => Self::Inch(a + b / CM_PER_INCH),
            (Self::Inch(a), Self::Inch(b)) => Self::Inch(a + b),
        }
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Cm(a), Self::Cm(b)) => Self::Cm(a - b),
            (Self::Cm(a), Self::Inch(b)) => Self::Cm(a - b * CM_PER_INCH),
            (Self::Inch(a), Self::Cm(b)) => Self::Inch(a - b / CM_PER_INCH),
            (Self::Inch(a), Self::Inch(b)) => Self::Inch(a - b),
        }
    }
}

impl Mul<f32> for Length {
    type Output = Length;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Cm(cm) => Self::Cm(cm * rhs),
            Self::Inch(inch) => Self::Inch(inch * rhs),
        }
    }
}

impl Div<f32> for Length {
    type Output = Length;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            Self::Cm(cm) => Self::Cm(cm / rhs),
            Self::Inch(inch) => Self::Inch(inch / rhs),
        }
    }
}

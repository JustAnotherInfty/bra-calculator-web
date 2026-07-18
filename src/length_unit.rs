/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::ops::Mul;

use crate::length::Length;

const CM_PER_INCH: f32 = 2.54;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LengthUnit {
    Cm,
    Inch,
}

impl LengthUnit {
    pub const fn convert_to(self, magnitude: f32, dst_unit: Self) -> f32 {
        match (self, dst_unit) {
            (Self::Cm, Self::Cm) => magnitude,
            (Self::Inch, Self::Inch) => magnitude,
            (Self::Inch, Self::Cm) => magnitude * CM_PER_INCH,
            (Self::Cm, Self::Inch) => magnitude / CM_PER_INCH,
        }
    }
}

impl Mul<f32> for LengthUnit {
    type Output = Length;

    fn mul(self, rhs: f32) -> Self::Output {
        Length::new(rhs, self)
    }
}

impl Mul<LengthUnit> for f32 {
    type Output = Length;

    fn mul(self, rhs: LengthUnit) -> Self::Output {
        Length::new(self, rhs)
    }
}

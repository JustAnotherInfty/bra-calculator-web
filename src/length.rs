/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::ops::{Add, Div, Mul, Sub};

use crate::LengthUnit;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Length {
    magnitude: f32,
    unit: LengthUnit,
}

impl Length {
    pub const fn new(magnitude: f32, unit: LengthUnit) -> Self {
        Self { magnitude, unit }
    }

    pub const fn into_raw_unit(self, dst_unit: LengthUnit) -> f32 {
        self.unit.convert_to(self.magnitude, dst_unit)
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        let other = rhs.into_raw_unit(self.unit);
        Self::new(self.magnitude + other, self.unit)
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Self) -> Self::Output {
        let other = rhs.into_raw_unit(self.unit);
        Self::new(self.magnitude - other, self.unit)
    }
}

impl Mul<f32> for Length {
    type Output = Length;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.magnitude * rhs, self.unit)
    }
}

impl Mul<Length> for f32 {
    type Output = Length;

    fn mul(self, rhs: Length) -> Self::Output {
        Length::mul(rhs, self)
    }
}

impl Div<f32> for Length {
    type Output = Length;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.magnitude / rhs, self.unit)
    }
}

impl Div<Length> for f32 {
    type Output = Length;

    fn div(self, rhs: Length) -> Self::Output {
        Length::div(rhs, self)
    }
}

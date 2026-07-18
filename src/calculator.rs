/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::borrow::Cow;

use crate::{countries::Country, length::Length, length_unit::LengthUnit};

const ZERO_INCHES: Length = Length::new(0.0, LengthUnit::Inch);
const FOUR_INCHES: Length = Length::new(4.0, LengthUnit::Inch);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Calculator {
    unit: LengthUnit,
    country: Country,
    plus_four: bool,
    under_bust: String,
    bust: String,
}

impl Calculator {
    pub const fn new(
        unit: LengthUnit,
        country: Country,
        plus_four: bool,
        under_bust: String,
        bust: String,
    ) -> Self {
        Self {
            unit,
            country,
            plus_four,
            under_bust,
            bust,
        }
    }

    pub fn set_unit(&mut self, unit: LengthUnit) {
        let old = self.unit;
        self.unit = unit;

        let under_bust: Result<f32, _> = self.under_bust.parse();
        let bust: Result<f32, _> = self.bust.parse();

        if let (Ok(under_bust), Ok(bust)) = (under_bust, bust) {
            let under_bust = old.convert_to(under_bust, unit);
            let bust = old.convert_to(bust, unit);

            self.under_bust = str_round_two_decimals(under_bust.to_string());
            self.bust = str_round_two_decimals(bust.to_string());
        }
    }

    pub fn set_country(&mut self, country: Country) {
        self.country = country;
    }

    pub fn set_plus_four(&mut self, plus_four: bool) {
        self.plus_four = plus_four;
    }

    pub fn set_under_bust(&mut self, under_bust: String) {
        self.under_bust = under_bust;
    }
    pub fn set_bust(&mut self, bust: String) {
        self.bust = bust;
    }
    pub fn set_band(&mut self, band: &str) {
        let band: Result<f32, _> = band.parse();
        if let Ok(band) = band {
            let offset = self.plus_four_offset().into_raw_unit(self.country.unit());
            let diff = band - offset;
            if diff > 0.0 {
                let diff = Length::new(diff, self.country.unit());
                let under_bust = diff.into_raw_unit(self.unit);
                self.under_bust = f32_round_two_decimals(under_bust);
            }
        }
    }
    pub fn set_cup(&mut self, cup: &str) {
        if let Some(diff) = self.country.get_diff_from_cup(cup) {
            let under_bust: Result<f32, _> = self.under_bust.parse();
            if let Ok(under_bust) = under_bust {
                let under_bust = Length::new(under_bust, self.unit);
                let offset = self.plus_four_offset();
                let bust = (under_bust + offset + diff).into_raw_unit(self.unit);
                self.bust = f32_round_two_decimals(bust);
            }
        }
    }
    pub fn set_bra(&mut self, band: &str, cup: &str) {
        self.set_band(band);
        self.set_cup(cup);
    }

    pub fn unit(&self) -> LengthUnit {
        self.unit
    }
    pub fn country(&self) -> Country {
        self.country
    }
    pub fn plus_four(&self) -> bool {
        self.plus_four
    }

    pub const fn plus_four_offset(&self) -> Length {
        // TODO: temporary hack
        if self.plus_four && matches!(self.country, Country::UK) {
            FOUR_INCHES
        } else {
            ZERO_INCHES
        }
    }

    pub fn under_bust(&self) -> String {
        self.under_bust.clone()
    }
    pub fn bust(&self) -> String {
        self.bust.clone()
    }
    pub fn bust_diff(&self) -> String {
        let under_bust: Result<f32, _> = self.under_bust.parse();
        let bust: Result<f32, _> = self.bust.parse();

        if let (Ok(under_bust), Ok(bust)) = (under_bust, bust) {
            f32_round_two_decimals(bust - under_bust)
        } else {
            "".to_string()
        }
    }

    pub fn band<'a>(&self, old_value: &'a str) -> Cow<'a, str> {
        let under_bust: Result<f32, _> = self.under_bust.parse();

        if let Ok(under_bust) = under_bust {
            let under_bust = Length::new(under_bust, self.unit);

            let offset = self.plus_four_offset();
            let band_step = self.country.band_step();
            let band = (under_bust + offset).round_to(band_step) as i32;
            Cow::from(band.to_string())
        } else {
            Cow::from(old_value)
        }
    }

    pub fn cup<'a>(&self, old_value: &'a str) -> Cow<'a, str> {
        let under_bust: Result<f32, _> = self.under_bust.parse();
        let bust: Result<f32, _> = self.bust.parse();

        if let (Ok(under_bust), Ok(bust)) = (under_bust, bust) {
            let under_bust = Length::new(under_bust, self.unit);
            let bust = Length::new(bust, self.unit);

            let offset = self.plus_four_offset();
            let diff = bust - (under_bust + offset);
            Cow::from(self.country.get_cup(diff))
        } else {
            Cow::from(old_value)
        }
    }
}

fn str_round_two_decimals(value: String) -> String {
    value.parse().map(f32_round_two_decimals).unwrap_or(value)
}

fn f32_round_two_decimals(value: f32) -> String {
    format!("{value:.02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ops::Deref;

    #[test]
    fn test_uk() {
        let under_bust = "83.5".to_string();
        let bust = "107".to_string();
        let mut calc = Calculator::new(LengthUnit::Cm, Country::UK, true, under_bust, bust);

        calc.set_unit(LengthUnit::Inch);
        assert_eq!(
            (calc.band("").deref(), calc.cup("").deref()), //
            ("36", "DD"),                                  //
        );
    }

    #[test]
    fn test_jp() {
        let under_bust = "83.5".to_string();
        let bust = "107".to_string();
        let calc = Calculator::new(LengthUnit::Cm, Country::JP, true, under_bust, bust);

        assert_eq!(
            (calc.band("").deref(), calc.cup("").deref()), //
            ("85", "F"),                                   //
        );
    }
}

/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::borrow::Cow;

use crate::{countries::Country, length::Length};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Calculator {
    use_inches: bool,
    country: Country,
    plus_four: bool,
    under_bust: String,
    bust: String,
}

impl Calculator {
    pub const fn new(
        use_inches: bool,
        country: Country,
        plus_four: bool,
        under_bust: String,
        bust: String,
    ) -> Self {
        Self {
            use_inches,
            country,
            plus_four,
            under_bust,
            bust,
        }
    }

    pub fn set_use_inches(&mut self, use_inches: bool) {
        let old = self.use_inches;
        self.use_inches = use_inches;

        let under_bust: Result<f32, _> = self.under_bust.parse();
        let bust: Result<f32, _> = self.bust.parse();

        if let (Ok(under_bust), Ok(bust)) = (under_bust, bust) {
            let (under_bust, bust) = if old {
                (Length::Inch(under_bust), Length::Inch(bust))
            } else {
                (Length::Cm(under_bust), Length::Cm(bust))
            };

            let (under_bust, bust) = if use_inches {
                (under_bust.into_raw_inch(), bust.into_raw_inch())
            } else {
                (under_bust.into_raw_cm(), bust.into_raw_cm())
            };
            self.under_bust = str_round_two_decimals(under_bust.to_string());
            self.bust = str_round_two_decimals(bust.to_string());
        }
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
            let offset = if self.plus_four { 4.0 } else { 0.0 };
            let under_bust = band - offset;
            if under_bust > 0.0 {
                let under_bust = if self.use_inches {
                    under_bust
                } else {
                    Length::Inch(under_bust).into_raw_cm()
                };
                self.under_bust = f32_round_two_decimals(under_bust);
            }
        }
    }
    pub fn set_cup(&mut self, cup: &str) {
        if let Some(diff) = self.country.get_cup_array().iter().position(|x| *x == cup) {
            let diff = diff as f32;
            let under_bust: Result<f32, _> = self.under_bust.parse();
            if let Ok(under_bust) = under_bust {
                let offset = if self.plus_four { 4.0 } else { 0.0 };
                let under_bust = under_bust + offset;
                let bust = if self.use_inches {
                    under_bust + diff
                } else {
                    under_bust + Length::Inch(diff).into_raw_cm()
                };
                self.bust = f32_round_two_decimals(bust);
            }
        }
    }

    pub fn use_inches(&self) -> bool {
        self.use_inches
    }
    pub fn plus_four(&self) -> bool {
        self.plus_four
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
            let under_bust = if self.use_inches {
                Length::Inch(under_bust)
            } else {
                Length::Cm(under_bust)
            };

            let offset = if self.plus_four { 4 } else { 0 };
            let band = (under_bust.into_raw_inch() / 2.0).round() as i32 * 2 + offset;
            Cow::from(band.to_string())
        } else {
            Cow::from(old_value)
        }
    }

    pub fn cup<'a>(&self, old_value: &'a str) -> Cow<'a, str> {
        let under_bust: Result<f32, _> = self.under_bust.parse();
        let bust: Result<f32, _> = self.bust.parse();

        if let (Ok(under_bust), Ok(bust)) = (under_bust, bust) {
            let (under_bust, bust) = if self.use_inches {
                (Length::Inch(under_bust), Length::Inch(bust))
            } else {
                (Length::Cm(under_bust), Length::Cm(bust))
            };

            let offset = Length::Inch(if self.plus_four { 4.0 } else { 0.0 });
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

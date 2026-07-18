/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{Component, Context, Event, Html, TargetCast, events::InputEvent, html, html::Scope};

mod calculator;
mod countries;
mod length;
mod length_unit;

use calculator::Calculator;
use countries::Country;
use length_unit::LengthUnit;

pub enum Msg {
    InputUnderBust(String),
    InputBust(String),
    InputBand(String),
    InputCup(String),
    InputUseInches(bool),
    InputCountry(Country),
}

pub struct App {
    calculator: Calculator,
    band: String,
    cup: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let unit = LengthUnit::Inch;
        let country = Country::UK;
        let under_bust = 32;
        let diff = 5 + 4;

        let calculator = Calculator::new(
            unit,
            country,
            under_bust.to_string(),
            (under_bust + diff).to_string(),
        );
        let band = calculator.band("");
        let cup = calculator.cup("");

        Self {
            calculator,
            band: band.to_string(),
            cup: cup.to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputUnderBust(under_bust) => {
                self.calculator.set_under_bust(under_bust);
                self.band = self.calculator.band(&self.band).to_string();
                self.cup = self.calculator.cup(&self.cup).to_string();
            }
            Msg::InputBust(bust) => {
                self.calculator.set_bust(bust);
                self.band = self.calculator.band(&self.band).to_string();
                self.cup = self.calculator.cup(&self.cup).to_string();
            }
            Msg::InputBand(band) => {
                self.band = band;
                self.calculator.set_bra(&self.band, &self.cup);
            }
            Msg::InputCup(cup) => {
                self.cup = cup.to_uppercase();
                self.calculator.set_bra(&self.band, &self.cup);
            }
            Msg::InputUseInches(use_inches) => {
                let unit = if use_inches {
                    LengthUnit::Inch
                } else {
                    LengthUnit::Cm
                };
                self.calculator.set_unit(unit);
            }
            Msg::InputCountry(country) => {
                self.calculator.set_country(country);
                self.band = self.calculator.band(&self.band).to_string();
                self.cup = self.calculator.cup(&self.cup).to_string();
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let main = self.view_main(ctx);

        html! {
          <div class="root_class">
            { main }
          </div>
        }
    }
}

impl App {
    fn view_main(&self, ctx: &Context<Self>) -> Html {
        html! {
          <main>
            <>
              { self.input_under_bust(ctx.link()) }
            </>

            <>
              { self.input_bust(ctx.link()) }
            </>

            <>
              { self.input_band(ctx.link()) }
            </>

            <>
              { self.input_cup(ctx.link()) }
            </>

            <>
              { self.input_use_inches(ctx.link()) }
            </>

            <>
              { self.input_country(ctx.link()) }
            </>
          </main>
        }
    }
}

impl App {
    fn input_under_bust(&self, link: &Scope<Self>) -> Html {
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Some(Msg::InputUnderBust(input.value()))
        });
        let value = self.calculator.under_bust();

        html! {
          <div class="input-box">
            <h2 for="input-under_bust"> { "Under bust" } </h2>
            <input
              id="input-under_bust"
              type="text"
              {oninput}
              {value}
            />
          </div>
        }
    }

    fn input_bust(&self, link: &Scope<Self>) -> Html {
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Some(Msg::InputBust(input.value()))
        });
        let value = self.calculator.bust();
        let diff = format!("Bust diff: {}", self.calculator.bust_diff());

        html! {
          <div class="input-box">
            <h2 for="input-bust"> { "Bust" } </h2>
            <input
              id="input-bust"
              type="text"
              {oninput}
              {value}
            />
            <br/>
            <>
            { diff }
            </>
          </div>
        }
    }

    fn input_band(&self, link: &Scope<Self>) -> Html {
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Some(Msg::InputBand(input.value()))
        });
        let value = self.band.clone();

        html! {
          <div class="input-box">
            <h2 for="input-band"> { "Band" } </h2>
            <input
              id="input-band"
              type="text"
              {oninput}
              {value}
            />
          </div>
        }
    }

    fn input_cup(&self, link: &Scope<Self>) -> Html {
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Some(Msg::InputCup(input.value()))
        });
        let value = self.cup.clone();

        html! {
          <div class="input-box">
            <h2 for="input-cup"> { "Cup" } </h2>
            <input
              id="input-cup"
              type="text"
              {oninput}
              {value}
            />
          </div>
        }
    }

    fn input_use_inches(&self, link: &Scope<Self>) -> Html {
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Some(Msg::InputUseInches(input.checked()))
        });
        let checked = self.calculator.unit() == LengthUnit::Inch;

        html! {
          <div class="input-box">
            <h2 for="input-use-inches"> { "Use inches" } </h2>
            <input
              id="input-use-inches"
              type="checkbox"
              {oninput}
              {checked}
            />
          </div>
        }
    }

    fn input_country(&self, link: &Scope<Self>) -> Html {
        let onchange = link.batch_callback(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            Some(Msg::InputCountry(Country::from_id(&select.value())))
        });
        let current_value = self.calculator.country();

        let elements: Vec<Html> = Country::self_array()
            .iter()
            .map(|x| {
                let selected = x == &current_value;

                html! {
                    <option value={x.id()} {selected}> { {x.name()} } </option>
                }
            })
            .collect();

        html! {
          <div class="input-box">
            <h2 for="input-country"> { "Country" } </h2>
            <select
              id="input-country"
              {onchange}
            >
              { elements }
            </select>
          </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

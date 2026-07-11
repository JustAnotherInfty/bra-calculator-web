/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::sync::Arc;

use web_sys::HtmlInputElement;
use yew::{Component, Context, Html, TargetCast, events::InputEvent, html, html::Scope};

mod countries;
mod length;

use countries::Country;

use length::Length;

pub enum Msg {
    InputUnderBust(Length),
    InputBust(Length),
    InputBand(u32),
    InputCup(String),
}

pub struct App {
    under_bust: Length,
    bust: Length,
    band: u32,
    cup: String,
    country: Country,
    // inches/cm
    // plus_four
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let under_bust = Length::Inch(32.0);
        let diff = Length::Inch(5.0);
        let country = Country::UK;
        Self {
            under_bust,
            bust: under_bust + diff,
            band: (under_bust + Length::Inch(4.0)).into_raw_inch().round() as u32,
            cup: country.get_cup(diff).into(),
            country,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputUnderBust(under_bust) => {
                self.under_bust = under_bust;
                self.band = (under_bust / 2.0).into_raw_inch().round() as u32 * 2 + 4;
                self.cup = self.country.get_cup(self.bust - self.under_bust).into()
            }
            Msg::InputBust(bust) => {
                self.bust = bust;
                self.cup = self.country.get_cup(self.bust - self.under_bust).into()
            }
            Msg::InputBand(band) => {
                self.band = band;
                self.under_bust = Length::Inch((band - 4) as f32);
                self.cup = self.country.get_cup(self.bust - self.under_bust).into()
            }
            Msg::InputCup(cup) => {
                self.cup = cup;
                if let Some(diff) = self
                    .country
                    .get_cup_array()
                    .iter()
                    .position(|x| *x == self.cup)
                {
                    self.bust = self.under_bust + Length::Inch(diff as f32);
                }
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
          </main>
        }
    }
}

impl App {
    fn input_under_bust(&self, link: &Scope<Self>) -> Html {
        let under_bust = self.under_bust;
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let parsed: Result<f32, _> = input.value().parse();
            Some(Msg::InputUnderBust(
                parsed.map(Length::Inch).unwrap_or(under_bust),
            ))
        });
        let value = self.under_bust.magnitude().to_string();

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
        let bust = self.bust;
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let parsed: Result<f32, _> = input.value().parse();
            Some(Msg::InputBust(parsed.map(Length::Inch).unwrap_or(bust)))
        });
        let value = self.bust.magnitude().to_string();

        html! {
          <div class="input-box">
            <h2 for="input-bust"> { "Bust" } </h2>
            <input
              id="input-bust"
              type="text"
              {oninput}
              {value}
            />
          </div>
        }
    }

    fn input_band(&self, link: &Scope<Self>) -> Html {
        let band = self.band;
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Some(Msg::InputBand(input.value().parse().unwrap_or(band)))
        });
        let value = self.band.to_string();

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
        let cup: Arc<String> = Arc::from(self.cup.clone());
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let cup2 = cup.to_string();
            Some(Msg::InputCup(input.value().parse().unwrap_or(cup2)))
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
}

fn main() {
    yew::Renderer::<App>::new().render();
}

/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::sync::Arc;

use web_sys::HtmlInputElement;
use yew::{Component, Context, Html, TargetCast, events::InputEvent, html, html::Scope};

mod countries;

use countries::Country;

pub enum Msg {
    InputUnderBust(u32),
    InputBust(u32),
    InputBand(u32),
    InputCup(String),
}

pub struct App {
    under_bust: u32,
    bust: u32,
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
        let under_bust = 32; // +4?
        let diff = 5;
        let country = Country::UK;
        Self {
            under_bust,
            bust: under_bust.wrapping_add_signed(diff),
            band: under_bust + 4,
            cup: country.get_cup(diff).into(),
            country,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputUnderBust(under_bust) => {
                self.under_bust = under_bust;
                self.band = under_bust + 4;
                self.cup = self
                    .country
                    .get_cup(self.bust.wrapping_sub(self.under_bust) as i32)
                    .into()
            }
            Msg::InputBust(bust) => {
                self.bust = bust;
                self.cup = self
                    .country
                    .get_cup(self.bust.wrapping_sub(self.under_bust) as i32)
                    .into()
            }
            Msg::InputBand(band) => {
                self.band = band;
                self.under_bust = band - 4;
                self.cup = self
                    .country
                    .get_cup(self.bust.wrapping_sub(self.under_bust) as i32)
                    .into()
            }
            Msg::InputCup(cup) => {
                self.cup = cup;
                if let Some(diff) = self
                    .country
                    .get_cup_array()
                    .iter()
                    .position(|x| *x == self.cup)
                {
                    self.bust = self.under_bust.wrapping_add(diff as u32);
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
            Some(Msg::InputUnderBust(
                input.value().parse().unwrap_or(under_bust),
            ))
        });
        let value = self.under_bust.to_string();

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
            Some(Msg::InputBust(input.value().parse().unwrap_or(bust)))
        });
        let value = self.bust.to_string();

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

/* SPDX-FileCopyrightText: © 2026 JustAnotherInfty */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::sync::Arc;

use web_sys::HtmlInputElement;
use yew::{Component, Context, Html, TargetCast, events::InputEvent, html, html::Scope};

pub enum Msg {
    InputBand(u32),
    InputBust(u32),
    InputCup(String),
}

pub struct App {
    band: u32,
    bust: u32,
    cup: String,
    // country?
    // inches/cm
    // plus_four
}

const UK_CUPS: [&str; 47] = [
    "AA", "A", "B", "C", "D", "DD", "E", "F", "FF", "G", "GG", "H", "HH", "J", "JJ", "K", "KK",
    "L", "LL", "M", "MM", "N", "NN", "O", "OO", "P", "PP", "Q", "QQ", "R", "RR", "S", "SS", "T",
    "TT", "U", "UU", "V", "VV", "W", "WW", "X", "XX", "Y", "YY", "Z", "ZZ",
];

fn get_cup(diff: i32) -> &'static str {
    if diff < -1 {
        "Too small"
    } else if diff == -1 {
        "AAA"
    } else if let Some(cup) = UK_CUPS.get(diff as usize) {
        cup
    } else {
        "Too big"
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let band = 32; // +4?
        let diff = 5;
        Self {
            band,
            bust: band.wrapping_add_signed(diff),
            cup: get_cup(diff).into(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputBand(band) => {
                self.band = band;
                self.cup = get_cup(self.bust.wrapping_sub(self.band) as i32).into()
            }
            Msg::InputBust(bust) => {
                self.bust = bust;
                self.cup = get_cup(self.bust.wrapping_sub(self.band) as i32).into()
            }
            Msg::InputCup(cup) => {
                self.cup = cup;
                if let Some(diff) = UK_CUPS.iter().position(|x| *x == self.cup) {
                    self.bust = self.band.wrapping_add(diff as u32);
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
              { self.input_band(ctx.link()) }
            </>

            <>
              { self.input_bust(ctx.link()) }
            </>

            <>
              { self.input_cup(ctx.link()) }
            </>
          </main>
        }
    }
}

impl App {
    fn input_band(&self, link: &Scope<Self>) -> Html {
        let band = self.band;
        let oninput = link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Some(Msg::InputBand(input.value().parse().unwrap_or(band)))
        });
        let value = (self.band/* + 4 */).to_string();

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

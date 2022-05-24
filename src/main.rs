pub mod roma;

use std::fmt::Display;

use roma::IME;
use yew::events::{FocusEvent, KeyboardEvent};
use yew::html::Scope;
use yew::{classes, html, Classes, Component, Context, Html, NodeRef, TargetCast};

pub struct App {
    sentence: Sentence<'static>,
    ime: IME,
}

pub struct Sentence<'a> {
    origin: &'a str,
    hira: &'a str,
}

impl<'a> Sentence<'a> {
    pub fn new(origin: &'a str, hira: &'a str) -> Self {
        let count_char = |s: &str, c: char| s.chars().filter(|&x| x == c).count();

        assert_eq!(count_char(origin, '|'), count_char(hira, '|'));
        assert_eq!(count_char(origin, '/'), count_char(hira, '/'));
        assert!(count_char(origin, '|') % 2 == 0);

        Sentence { origin, hira }
    }
}

#[derive(Debug)]
pub enum AppMessage {
    Type(String),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            // [A("これはタイピングゲームの"), B("試", "し"), B("作", "さく), B("品", "ひん"),
            //  A("です")] でいいかも？
            sentence: Sentence::new(
                "これはタイピングゲームの|試/作/品|です",
                "これはたいぴんぐげーむの|し/さく/ひん|です",
            ),
            ime: IME::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AppMessage::*;

        log::info!("root update: {:?}", msg);

        match msg {
            Type(b) if b == "Backspace" => {
                self.ime.pop();
            }

            Type(a) => {
                let chars = a.chars().collect::<Vec<_>>();
                if let &[c] = chars.as_slice() {
                    self.ime.put(c);
                }
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{ CharDisplay(self.ime.buffer()) }</p>
                <input
                    placeholder="type here"
                    value=""
                    onkeydown={ctx.link().callback(|e: KeyboardEvent| AppMessage::Type(e.key()) )}
                />
            </>
        }
    }
}

struct CharDisplay<'a>(&'a [char]);
impl<'a> Display for CharDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            write!(f, "{c}")?;
        }

        Ok(())
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

// false-positive on `use $macro`: we need it to use macro above its definition
#![allow(clippy::single_component_path_imports)]
#![allow(dead_code)]

mod ext;
mod roma;

use macros::segments;
use roma::Ime;
use yew::{events::KeyboardEvent, html, Component, Context, Html};

#[rustfmt::skip]
const SENTENCES: &[&[Segment]] = &[
    &segments![
        日本国民       / "は、" / 正当     / に / 選挙     / された / 国会     / における / 代表者         / を / 通   / じて / 行動     / "し、",
        にほんこくみん / "は、" / せいとう / に / せんきょ / された / こっかい / における / だいひょうしゃ / を / つう / じて / こうどう / "し、",
    ],
    &segments![
        われらとわれらの / 子孫   / "のために、" / 諸国民       / との / 協和     / による / 成果   / "と、",
        われらとわれらの / しそん / "のために、" / しょこくみん / との / きょうわ / による / せいか / "と、",
    ],
    &segments![
        わが / 国   / 全土   / にわたつて / 自由   / のもたらす / 恵沢     / を / 確保   / "し、",
        わが / くに / ぜんど / にわたつて / じゆう / のもたらす / けいたく / を / かくほ / "し、",
    ],
];

pub struct App<'a> {
    ime: Ime,
    sentences: Vec<Sentence<'a>>,
    index: usize,
}

pub struct Sentence<'a> {
    segments: &'a [Segment<'a>],
    index: usize,
}

pub struct Segment<'a> {
    origin: &'a [char],
    hira: &'a [char],
}

impl<'a> Sentence<'a> {
    fn new(segments: &'a [Segment<'a>]) -> Self {
        Self { segments, index: 0 }
    }

    fn segments(&self) -> &'a [Segment<'a>] {
        self.segments
    }

    fn advance_segment(&mut self) -> bool {
        if self.index + 1 == self.segments.len() {
            false
        } else {
            self.index += 1;
            true
        }
    }

    fn current_segment(&self) -> &'a Segment<'a> {
        &self.segments[self.index]
    }

    fn typed_segments(&self) -> &'a [Segment<'a>] {
        &self.segments[..self.index]
    }

    fn untyped_segments(&self) -> &'a [Segment<'a>] {
        self.segments.get(self.index + 1..).unwrap_or(&[])
    }
}

impl<'a> Segment<'a> {
    pub const fn new(origin: &'a [char], hira: &'a [char]) -> Self {
        Self { origin, hira }
    }
}

#[derive(Debug)]
pub enum AppMessage {
    Type(String),
}

struct SegmentTypingStatus {
    c: char,
    ok: bool,
}

impl SegmentTypingStatus {
    fn new(c: char, ok: bool) -> Self {
        Self { c, ok }
    }

    fn ok(c: char) -> Self {
        Self { c, ok: true }
    }

    fn not_ok(c: char) -> Self {
        Self { c, ok: false }
    }
}

impl<'a> App<'a> {
    fn sentence(&self) -> &Sentence<'a> {
        &self.sentences[self.index]
    }
    fn sentence_mut(&mut self) -> &mut Sentence<'a> {
        &mut self.sentences[self.index]
    }

    fn typing_status(&self) -> Vec<SegmentTypingStatus> {
        let mut ret = vec![];

        let ime_buf = self.ime.buffer();
        let segment = self.sentence().current_segment();

        for (i, &c) in ime_buf.iter().enumerate() {
            if c.is_ascii() {
                break;
            }
            if Some(c) != segment.hira.get(i).copied() {
                for &ime_buf in &ime_buf[i..] {
                    ret.push(SegmentTypingStatus::not_ok(ime_buf));
                }
                return ret;
            }

            ret.push(SegmentTypingStatus::ok(c));
        }

        let ime_candidates = self.ime.candidates();
        let hira_len_in_ime_buf = ime_buf.iter().take_while(|x| !x.is_ascii()).count();

        let typing_correctly = matches!(segment.hira.get(hira_len_in_ime_buf), Some(&s) if ime_candidates.contains(&[s].as_slice()))
            || matches!(segment.hira.get(hira_len_in_ime_buf..=hira_len_in_ime_buf+1), Some(s) if ime_candidates.contains(&s));

        for &ime_buf in &ime_buf[hira_len_in_ime_buf..] {
            ret.push(SegmentTypingStatus::new(ime_buf, typing_correctly));
        }

        ret
    }
}

type DefaultApp = App<'static>;

impl Component for DefaultApp {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ime: Ime::new(),
            sentences: SENTENCES
                .iter()
                .map(|x| Sentence::new(x))
                .collect::<Vec<_>>(),
            index: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AppMessage::*;

        match msg {
            Type(b) if b == "Backspace" => {
                self.ime.pop();
            }

            Type(c) => {
                let chars = c.chars().collect::<Vec<_>>();
                if let &[c] = chars.as_slice() {
                    self.ime.put(c);
                }

                let segment = self.sentence().current_segment();
                if self.ime.buffer().get(0..segment.hira.len()) == Some(segment.hira) {
                    let len = segment.hira.len();
                    self.ime.trim_beginning(len);

                    if !self.sentence_mut().advance_segment() {
                        self.index += 1;
                    }
                }
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let typed_origin = self
            .sentence()
            .typed_segments()
            .iter()
            .flat_map(|x| x.origin)
            .collect::<String>();

        let untyped_origin = self
            .sentence()
            .segments()
            .iter()
            .flat_map(|x| x.origin)
            .skip(typed_origin.chars().count())
            .collect::<String>();

        let mut typed_hira = self
            .sentence()
            .typed_segments()
            .iter()
            .flat_map(|x| x.hira)
            .collect::<String>();

        let s = self
            .typing_status()
            .iter()
            .map(|x| {
                let color = if x.ok { "green" } else { "red" };
                let style = format!("color: {color}; text-decoration: underline {color};");
                html!(<span {style}>{x.c}</span>)
            })
            .collect::<Html>();
        let s = html!(<><span style={"color:green"}>{&typed_hira}</span>{s}</>);

        let untyped_hira = self
            .sentence()
            .segments()
            .iter()
            .flat_map(|x| x.hira)
            .skip(
                typed_hira.chars().count()
                    + self
                        .typing_status()
                        .iter()
                        .take_while(|x| !x.c.is_ascii())
                        .count(),
            )
            .collect::<String>();

        let typing = self
            .typing_status()
            .iter()
            .map(|x| {
                let color = if x.ok { "green" } else { "red" };
                let style = format!("color: {color}; text-decoration: underline {color};");
                html!(<span {style}>{x.c}</span>)
            })
            .collect::<Html>();

        let next = 'd: {
            let Some(next) = self.sentences.get(self.index + 1) else {
                break 'd html!();
            };

            let s = next
                .segments()
                .iter()
                .flat_map(|x| x.origin)
                .collect::<String>();

            html!(<p style={"color:gray"}>{s}</p>)
        };

        html! {
            <>
                <input
                    placeholder="type here"
                    value=""
                    onkeydown={ctx.link().callback(|e: KeyboardEvent| AppMessage::Type(e.key()))}
                />
                <p> {self.ime.input_history().collect::<String>()} {"　"} </p>
                <p>
                    <span style={"color:green"}>{&typed_origin}</span>
                    <span style={"color:gray"}>{&untyped_origin}</span>
                    <br />
                    <span style={"color:green"}>{&typed_origin}</span>
                    {typing.clone()}
                    {"　"}
                    <br />

                    <span style={"color:green"}>{s}</span>
                    <span style={"color:gray"}>{&untyped_hira}</span>
                </p>
                {next}
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<DefaultApp>::new().render();
}

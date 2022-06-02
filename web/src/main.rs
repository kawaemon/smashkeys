// false-positive on `use $macro`: we need it to use macro above its definition
#![allow(clippy::single_component_path_imports)]
#![allow(dead_code)]

mod ext;
mod roma;

use ext::eq_ref;
use macros::segments;
use roma::Ime;
use yew::{events::KeyboardEvent, html, Component, Context, Html};

include!("../.kprivate/lyric.rs");

pub struct App<'a, S: 'a> {
    ime: Ime,
    sentences: S,
    sentence: Sentence<'a>,
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

impl<'a, S> App<'a, S>
where
    S: 'a + Iterator<Item = Sentence<'a>>,
{
    fn typing_status(&self) -> Vec<SegmentTypingStatus> {
        let mut ret = vec![];

        let ime_buf = self.ime.buffer();
        let segment = self.sentence.current_segment();

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

type DefaultApp = App<'static, Box<dyn Iterator<Item = Sentence<'static>>>>;

impl Component for DefaultApp {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut iter = SENTENCES.iter().map(|x| Sentence::new(x));
        let sentence = iter.next().unwrap();
        Self {
            ime: Ime::new(),
            sentences: Box::new(iter),
            sentence,
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

                let segment = self.sentence.current_segment();
                if self.ime.buffer().get(0..segment.hira.len()) == Some(segment.hira) {
                    let len = segment.hira.len();
                    self.ime.trim_beginning(len);

                    if !self.sentence.advance_segment() {
                        self.sentence = self.sentences.next().unwrap();
                    }
                }
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let typed = self
            .sentence
            .typed_segments()
            .iter()
            .flat_map(|x| x.origin)
            .collect::<String>();

        let typing_status = self.typing_status();
        let typing = typing_status
            .iter()
            .map(|x| {
                let color = if x.ok { "green" } else { "red" };
                let style = format!("color: {color}; text-decoration: underline {color};");
                html!(<span {style}>{x.c}</span>)
            })
            .collect::<Html>();

        let untyped = self
            .sentence
            .segments()
            .iter()
            .flat_map(|x| x.origin)
            .skip(typed.chars().count())
            .collect::<String>();

        let correctly_typing_hira = self
            .ime
            .buffer()
            .iter()
            .zip(self.sentence.current_segment().hira.iter())
            .take_while(eq_ref)
            .map(|(&a, _)| a)
            .collect::<Vec<char>>();

        let typed_hira_segments = self
            .sentence
            .typed_segments()
            .iter()
            .flat_map(|x| x.hira)
            .copied()
            .collect::<String>();

        let typed_hira = self
            .sentence
            .typed_segments()
            .iter()
            .flat_map(|x| x.hira)
            .copied()
            .chain(correctly_typing_hira.iter().copied())
            .collect::<String>();

        let untyped_hira = self
            .sentence
            .current_segment()
            .hira
            .iter()
            .copied()
            .skip(correctly_typing_hira.len())
            .chain(
                self.sentence
                    .untyped_segments()
                    .iter()
                    .flat_map(|x| x.hira)
                    .copied(),
            )
            .collect::<Vec<char>>();

        html! {
            <>
                <input
                    placeholder="type here"
                    value=""
                    onkeydown={ctx.link().callback(|e: KeyboardEvent| AppMessage::Type(e.key()))}
                />
                <p>
                    <span style={"color:green"}>{typed_hira}</span>
                    <span style={"color:gray"}>{untyped_hira}</span>
                    <br /> // forvive me
                    <span style={"color:green"}>{typed_hira_segments}</span>
                    {typing.clone()}
                </p>
                <p>
                    <span style={"color:green"}>{&typed}</span>
                    <span style={"color:gray"}>{untyped}</span>
                    <br /> // forgive me
                    <span style={"color:green"}>{&typed}</span>
                    {typing}
                </p>
                // <p> {self.ime.input_history().collect::<String>()} </p>
                <video src="video.webm" width=240 controls={true} />
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<DefaultApp>::new().render();
}

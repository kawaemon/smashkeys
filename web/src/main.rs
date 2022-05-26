pub mod roma;

use macros::ident_as_char_array;

use roma::IME;
use yew::events::KeyboardEvent;

use yew::{html, Component, Context, Html};

pub struct App {
    ime: IME,
    segments: &'static [Segment<'static>],
    current_segment_index: usize,
}

pub struct Segment<'a> {
    origin: &'a [char],
    hira: &'a [char],
}

macro_rules! segments {
    ($($origin: ident)/+, $($hira: ident)/+) => {
        [$(Segment::new(ident_as_char_array!($origin).as_slice(), ident_as_char_array!($hira).as_slice()),)+]
    };
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

impl App {
    fn current_segment(&self) -> &Segment<'_> {
        &self.segments[self.current_segment_index]
    }

    fn typed_segments(&self) -> &[Segment<'_>] {
        &self.segments[..self.current_segment_index]
    }

    fn untyped_segments(&self) -> &[Segment<'_>] {
        let len = self.segments.len();
        let index = (self.current_segment_index + 1).min(len);
        &self.segments[index..]
    }

    fn typing_status(&self) -> Vec<SegmentTypingStatus> {
        let mut ret = vec![];

        let ime_buf = self.ime.buffer();
        let segment = self.current_segment();

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
            || matches!(segment.hira.get(hira_len_in_ime_buf..hira_len_in_ime_buf+1), Some(s) if ime_candidates.contains(&s));

        for &ime_buf in &ime_buf[hira_len_in_ime_buf..] {
            ret.push(SegmentTypingStatus::new(ime_buf, typing_correctly));
        }

        ret
    }
}

const SENTENCE: &[Segment] = &segments![
    これは / タイピング / ゲーム / の / 試作品 / です,
    これは / たいぴんぐ / げーむ / の / しさくひん / です
];

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ime: IME::new(),
            segments: SENTENCE,
            current_segment_index: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AppMessage::*;

        log::info!("root update: {:?}", msg);

        match msg {
            Type(b) if b == "Backspace" => {
                self.ime.pop();
            }

            Type(c) => {
                let chars = c.chars().collect::<Vec<_>>();
                if let &[c] = chars.as_slice() {
                    self.ime.put(c);
                }

                if self.ime.buffer() == self.current_segment().hira {
                    self.current_segment_index += 1;
                    self.ime.clear();
                }
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let typed = self
            .typed_segments()
            .iter()
            .flat_map(|x| x.origin)
            .collect::<String>();

        let typing_status = self.typing_status();
        let typing = typing_status
            .iter()
            .map(|x| {
                let color = if x.ok { "green" } else { "red" };
                let style = format!("color:{color}; text-decoration: underline {color};");
                html!(<span {style}>{x.c}</span>)
            })
            .collect::<Html>();

        let untyped = self
            .segments
            .iter()
            .flat_map(|x| x.origin)
            .skip(typed.chars().count())
            .collect::<String>();

        html! {
            <>
                <p>
                    <span style={"color:green"}>{&typed}</span>
                    <span style={"color:gray"}>{untyped}</span>
                </p>
                <p>
                    <span style={"color:green"}>{&typed}</span>
                    {typing}
                </p>
                <input
                    placeholder="type here"
                    value=""
                    onkeydown={ctx.link().callback(|e: KeyboardEvent| AppMessage::Type(e.key()) )}
                />
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

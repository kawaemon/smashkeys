use std::{cell::RefCell, collections::HashMap};

use macros::as_char_array;
use once_cell::sync::Lazy;
use smallvec::SmallVec;

use crate::ext::IteratorAllEqExt as _;

#[derive(Debug, Default)]
pub struct Ime {
    // On perspective of we frequently pop front, `VecDeque` is suitable here, but we need contiguous
    // memory space (&[char]) to lookup on HashMap (ROMA_TABLE). We could use `VecDeque::make_contiguous`
    // function, but we need to call it every time we lookup ROMA_TABLE.
    // Probably it pays more cost than faster `pop_front` we get by using `VecDeque`.
    buffer: Vec<char>,
    input_history: Vec<(char, usize)>, // input char, buffer.len()
}

impl Ime {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            input_history: vec![],
        }
    }

    pub fn buffer(&self) -> &[char] {
        &self.buffer
    }

    pub fn input_history(&self) -> impl Iterator<Item = char> + '_ {
        self.input_history.iter().map(|&(c, _)| c)
    }

    pub fn pop(&mut self) -> Option<char> {
        let pop_count = self
            .input_history
            .iter()
            .filter(|&&(_, i)| i == self.buffer.len())
            .count();

        for _ in 0..pop_count {
            self.input_history.pop();
        }

        self.buffer.pop()
    }

    pub fn trim_beginning(&mut self, n: usize) {
        self.buffer.drain(0..n);

        let drain_until = self.input_history.iter().filter(|&&(_, i)| i <= n).count();
        self.input_history.drain(..drain_until);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn candidates(&self) -> SmallVec<[&[char]; 10]> {
        let remains = self
            .buffer
            .iter()
            .copied()
            .skip_while(|x| !x.is_ascii())
            .collect::<SmallVec<[char; 3]>>();

        let mut ret = SmallVec::new();

        for (&roma, &hira) in ROMA_TABLE.iter() {
            if roma.iter().zip(remains.iter()).all_eq() {
                ret.push(hira);
            }
        }

        if matches!(remains.as_slice(), &[a] if !"aiueon".contains(a)) {
            ret.push(['っ'].as_slice());
        }

        ret
    }

    fn record_input_history(&mut self, input: char) {
        self.input_history.push((input, self.buffer.len()));
    }

    pub fn put(&mut self, input: char) {
        struct RecordInputHistoryGuard<'a, 'b> {
            ime: &'a RefCell<&'b mut Ime>,
            input: char,
        }
        impl Drop for RecordInputHistoryGuard<'_, '_> {
            fn drop(&mut self) {
                let mut ime = self.ime.borrow_mut();
                ime.record_input_history(self.input);
                log::info!("{:#?}", ime);
            }
        }

        assert!(input.is_ascii());

        let input = input.to_ascii_lowercase();

        let me = RefCell::new(self);
        let _guard = RecordInputHistoryGuard { ime: &me, input };

        let mut me = me.borrow_mut();

        me.buffer.push(input);

        // 通常の変換, a -> あ, tya -> ちゃ
        let len = me.buffer.len();
        for i in (1..=3.min(len)).rev() {
            if let Some(hira) = ROMA_TABLE.get(&me.buffer[len - i..]) {
                for _ in 0..i {
                    me.buffer.pop();
                }
                for c in *hira {
                    me.buffer.push(*c);
                }
                return;
            }
        }

        if_bind! {
            // tt -> っt
            if match me.buffer.as_slice(); &[.., a, b] if a == b && !"aiueon".contains(a) => {
                me.buffer.pop();
                me.buffer.pop();

                me.buffer.push('っ');
                me.buffer.push(a);

                return;
            }
        }

        // nr -> んr
        if matches!(me.buffer.as_slice(), &[.., 'n', a] if a != 'y') {
            let i = me.buffer.len() - 2;
            me.buffer[i] = 'ん';

            me.record_input_history(input);
        }
    }
}

macro_rules! if_bind {
    ($(if match $expr:expr; $(|)? $( $pat:pat_param )|+ $( if $guard: expr )? => $block:block )*) => {
        $(
            match $expr {
                $( $pat )|+ $( if $guard )? => $block,
                _ => {}
            }
        )*
    }
}

// we need this to use macro above its definition
use if_bind;

macro_rules! roma_pairs {
    ($(($roma: literal, $hira: literal)),*$(,)?) => {
        [$((as_char_array!($roma).as_slice(), as_char_array!($hira).as_slice())),*]
    }
}

static ROMA_TABLE: Lazy<HashMap<&[char], &[char]>> = Lazy::new(|| {
    roma_pairs![
        (",", "、"),
        (".", "。"),
        ("a", "あ"),
        ("i", "い"),
        ("u", "う"),
        ("e", "え"),
        ("o", "お"),
        ("ka", "か"),
        ("ki", "き"),
        ("ku", "く"),
        ("ke", "け"),
        ("ko", "こ"),
        ("ga", "が"),
        ("gi", "ぎ"),
        ("gu", "ぐ"),
        ("ge", "げ"),
        ("go", "ご"),
        ("sa", "さ"),
        ("si", "し"),
        ("su", "す"),
        ("se", "せ"),
        ("so", "そ"),
        ("za", "ざ"),
        ("zi", "じ"),
        ("ji", "じ"),
        ("zu", "ず"),
        ("ze", "ぜ"),
        ("zo", "ぞ"),
        ("ta", "た"),
        ("ti", "ち"),
        ("chi", "ち"),
        ("tu", "つ"),
        ("tsu", "つ"),
        ("te", "て"),
        ("to", "と"),
        ("da", "だ"),
        ("di", "ぢ"),
        ("du", "づ"),
        ("de", "で"),
        ("do", "ど"),
        ("na", "な"),
        ("ni", "に"),
        ("nu", "ぬ"),
        ("ne", "ね"),
        ("no", "の"),
        ("ha", "は"),
        ("hi", "ひ"),
        ("hu", "ふ"),
        ("fu", "ふ"),
        ("he", "へ"),
        ("ho", "ほ"),
        ("ba", "ば"),
        ("bi", "び"),
        ("bu", "ぶ"),
        ("be", "べ"),
        ("bo", "ぼ"),
        ("pa", "ぱ"),
        ("pi", "ぴ"),
        ("pu", "ぷ"),
        ("pe", "ぺ"),
        ("po", "ぽ"),
        ("ma", "ま"),
        ("mi", "み"),
        ("mu", "む"),
        ("me", "め"),
        ("mo", "も"),
        ("ya", "や"),
        ("yu", "ゆ"),
        ("yo", "よ"),
        ("ra", "ら"),
        ("ri", "り"),
        ("ru", "る"),
        ("re", "れ"),
        ("ro", "ろ"),
        ("wa", "わ"),
        ("wo", "を"),
        ("nn", "ん"),
        ("xa", "ぁ"),
        ("la", "ぁ"),
        ("xi", "ぃ"),
        ("li", "ぃ"),
        ("xu", "ぅ"),
        ("lu", "ぅ"),
        ("xe", "ぇ"),
        ("le", "ぇ"),
        ("xo", "ぉ"),
        ("lo", "ぉ"),
        ("xya", "ゃ"),
        ("lya", "ゃ"),
        ("xyu", "ゅ"),
        ("lyu", "ゅ"),
        ("xyo", "ょ"),
        ("lyo", "ょ"),
        ("ltu", "っ"),
        ("xtu", "っ"),
        ("kya", "きゃ"),
        ("kyi", "きぃ"),
        ("kyu", "きゅ"),
        ("kye", "きぇ"),
        ("kyo", "きょ"),
        ("qa", "くぁ"),
        ("qi", "くぃ"),
        ("qwu", "くぅ"),
        ("qe", "くぇ"),
        ("qo", "くぉ"),
        ("gya", "ぎゃ"),
        ("gyi", "ぎぃ"),
        ("gyu", "ぎゅ"),
        ("gye", "ぎぇ"),
        ("gyo", "ぎょ"),
        ("gwa", "ぐぁ"),
        ("gwi", "ぐぃ"),
        ("gwu", "ぐぅ"),
        ("gwe", "ぐぇ"),
        ("gwo", "ぐぉ"),
        ("sya", "しゃ"),
        ("sha", "しゃ"),
        ("syi", "しぃ"),
        ("syu", "しゅ"),
        ("shu", "しゅ"),
        ("sye", "しぇ"),
        ("she", "しぇ"),
        ("syo", "しょ"),
        ("sho", "しょ"),
        ("swa", "すぁ"),
        ("swi", "すぃ"),
        ("swu", "すぅ"),
        ("swe", "すぇ"),
        ("swo", "すぉ"),
        ("ja", "じゃ"),
        ("zya", "じゃ"),
        ("zyi", "じぃ"),
        ("ju", "じゅ"),
        ("zyu", "じゅ"),
        ("je", "じぇ"),
        ("zye", "じぇ"),
        ("jo", "じょ"),
        ("zyo", "じょ"),
        ("tya", "ちゃ"),
        ("cha", "ちゃ"),
        ("tyi", "ちぃ"),
        ("tyu", "ちゅ"),
        ("chu", "ちゅ"),
        ("tye", "ちぇ"),
        ("che", "ちぇ"),
        ("tyo", "ちょ"),
        ("cho", "ちょ"),
        ("tha", "てゃ"),
        ("thi", "てぃ"),
        ("thu", "てゅ"),
        ("the", "てぇ"),
        ("tho", "てょ"),
        ("twa", "とぁ"),
        ("twi", "とぃ"),
        ("two", "とぅ"),
        ("twe", "とぇ"),
        ("two", "とぉ"),
        ("dya", "ぢゃ"),
        ("dyi", "ぢぃ"),
        ("dyu", "ぢゅ"),
        ("dye", "ぢぇ"),
        ("dyo", "ぢょ"),
        ("dha", "でゃ"),
        ("dhi", "でぃ"),
        ("dhu", "でゅ"),
        ("dhe", "でぇ"),
        ("dho", "でょ"),
        ("dwa", "どぁ"),
        ("dwi", "どぃ"),
        ("dwu", "どぅ"),
        ("dwe", "どぇ"),
        ("dwo", "どぉ"),
        ("nya", "にゃ"),
        ("nyi", "にぃ"),
        ("nyu", "にゅ"),
        ("nye", "にぇ"),
        ("nyo", "にょ"),
        ("hya", "ひゃ"),
        ("hyi", "ひぃ"),
        ("hyu", "ひゅ"),
        ("hye", "ひぇ"),
        ("hyo", "ひょ"),
        ("fa", "ふぁ"),
        ("fi", "ふぃ"),
        ("fwu", "ふぅ"),
        ("fe", "ふぇ"),
        ("fo", "ふぉ"),
        ("bya", "びゃ"),
        ("byi", "びぃ"),
        ("byu", "びゅ"),
        ("bye", "びぇ"),
        ("byo", "びょ"),
        ("pya", "ぴゃ"),
        ("pyi", "ぴぃ"),
        ("pyu", "ぴゅ"),
        ("pye", "ぴぇ"),
        ("pyo", "ぴょ"),
        ("mya", "みゃ"),
        ("myi", "みぃ"),
        ("myu", "みゅ"),
        ("mye", "みぇ"),
        ("myo", "みょ"),
        ("rya", "りゃ"),
        ("ryi", "りぃ"),
        ("ryu", "りゅ"),
        ("rye", "りぇ"),
        ("ryo", "りょ"),
        ("wha", "うぁ"),
        ("wi", "うぃ"),
        ("we", "うぇ"),
        ("who", "うぉ"),
    ]
    .into_iter()
    .collect()
});

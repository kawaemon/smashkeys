use std::collections::HashMap;

use once_cell::sync::Lazy;
use smallvec::SmallVec;

use macros::as_char_array;

#[derive(Debug, Default)]
pub struct IME {
    buffer: Vec<char>,
}

impl IME {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn buffer(&self) -> &[char] {
        &self.buffer
    }

    pub fn pop(&mut self) -> Option<char> {
        self.buffer.pop()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn trim_beginning(&mut self, n: usize) {
        self.buffer.drain(0..n);
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
            let is_candidate = roma.iter().zip(remains.iter()).all(|(a, b)| a == b);
            if is_candidate {
                ret.push(hira);
            }
        }

        if matches!(remains.as_slice(), &[a] if !"aiueon".contains(a)) {
            ret.push(['っ'].as_slice());
        }

        ret
    }

    pub fn put(&mut self, c: char) {
        assert!(c.is_ascii());

        let c = c.to_ascii_lowercase();

        self.buffer.push(c);

        // tt -> っt
        if matches!(self.buffer.as_slice(), &[.., a, b] if a == b && !"aiueon".contains(a)) {
            self.buffer.pop();
            let a = self.buffer.pop().unwrap();
            self.buffer.push('っ');
            self.buffer.push(a);
        }

        // 通常の変換, a -> あ, tya -> ちゃ
        let len = self.buffer.len();
        for i in (1..=3.min(len)).rev() {
            if let Some(hira) = ROMA_TABLE.get(&self.buffer[len - i..]) {
                for _ in 0..i {
                    self.buffer.pop();
                }
                for c in *hira {
                    self.buffer.push(*c);
                }
                return;
            }
        }

        // nr -> んr
        if matches!(self.buffer.as_slice(), &[.., 'n', a] if a != 'y') {
            let i = self.buffer.len() - 2;
            self.buffer[i] = 'ん';
        }
    }
}

macro_rules! roma_pairs {
    ($(($roma: literal, $hira: literal)),*$(,)?) => {
        [$((as_char_array!($roma).as_slice(), as_char_array!($hira).as_slice())),*]
    }
}

static ROMA_TABLE: Lazy<HashMap<&[char], &[char]>> = Lazy::new(|| {
    roma_pairs! [
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

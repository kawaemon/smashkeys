use std::collections::HashMap;

use macros::as_char_array;
use once_cell::sync::Lazy;
use smallvec::SmallVec;

use crate::ext::eq;

#[derive(Debug, Default)]
pub struct Ime {
    // On perspective of we frequently pop front, `VecDeque` is suitable here, but we need contiguous
    // memory space (&[char]) to lookup on HashMap We could use `VecDeque::make_contiguous`
    // function, but we need to call it every time we lookup HashMap.
    // Probably it pays more cost than faster `pop_front` we get by using `VecDeque`.
    buffer: Vec<char>,
}

impl Ime {
    pub fn new() -> Self {
        Self { buffer: vec![] }
    }

    pub fn buffer(&self) -> &[char] {
        &self.buffer
    }

    pub fn set_buffer(&mut self, new_buf: Vec<char>) {
        self.buffer = new_buf;
    }

    pub fn pop(&mut self) -> Option<char> {
        self.buffer.pop()
    }

    pub fn trim_beginning(&mut self, n: usize) {
        self.buffer.drain(0..n);
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

        for (&roma, &hira) in ROMA_TO_HIRA_TABLE.iter() {
            if roma.iter().zip(remains.iter()).all(eq) {
                ret.push(hira);
            }
        }

        if matches!(remains.as_slice(), &[a] if !"aiueon".contains(a)) {
            ret.push(['っ'].as_slice());
        }

        ret
    }

    pub fn put(&mut self, input: char) {
        assert!(input.is_ascii());

        let input = input.to_ascii_lowercase();

        self.buffer.push(input);

        // 通常の変換, a -> あ, tya -> ちゃ
        let len = self.buffer.len();
        for i in (1..=3.min(len)).rev() {
            if let Some(hira) = ROMA_TO_HIRA_TABLE.get(&self.buffer[len - i..]) {
                for _ in 0..i {
                    self.buffer.pop();
                }
                for c in *hira {
                    self.buffer.push(*c);
                }
                return;
            }
        }

        if_bind! {
            // tt -> っt
            if match self.buffer.as_slice(); &[.., a, b] if a == b && !"aiueon".contains(a) => {
                self.buffer.pop();
                self.buffer.pop();

                self.buffer.push('っ');
                self.buffer.push(a);

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
    ($(($hira: literal, [$($roma: literal),+$(,)?])),*$(,)?) => {
        [$((as_char_array!($hira).as_slice(), [$(as_char_array!($roma).as_slice()),+].as_slice())),*]
    }
}

static ROMA_TO_HIRA_TABLE: Lazy<HashMap<&[char], &[char]>> = Lazy::new(|| {
    ROMA_PAIRS
        .into_iter()
        .flat_map(|(hira, romas)| romas.iter().map(move |&roma| (roma, hira)))
        .collect()
});

static HIRA_TO_ROMA_TABLE: Lazy<HashMap<&[char], &[&[char]]>> =
    Lazy::new(|| ROMA_PAIRS.into_iter().collect());

const ROMA_PAIRS: [(&[char], &[&[char]]); 183] = roma_pairs![
    (",", ["、"]),
    (".", ["。"]),
    ("-", ["ー"]),
    ("あ", ["a"]),
    ("い", ["i"]),
    ("う", ["u"]),
    ("え", ["e"]),
    ("お", ["o"]),
    ("か", ["ka"]),
    ("き", ["ki"]),
    ("く", ["ku"]),
    ("け", ["ke"]),
    ("こ", ["ko"]),
    ("が", ["ga"]),
    ("ぎ", ["gi"]),
    ("ぐ", ["gu"]),
    ("げ", ["ge"]),
    ("ご", ["go"]),
    ("さ", ["sa"]),
    ("し", ["si"]),
    ("す", ["su"]),
    ("せ", ["se"]),
    ("そ", ["so"]),
    ("ざ", ["za"]),
    ("じ", ["zi", "ji"]),
    ("ず", ["zu"]),
    ("ぜ", ["ze"]),
    ("ぞ", ["zo"]),
    ("た", ["ta"]),
    ("ち", ["ti", "chi"]),
    ("つ", ["tu", "tsu"]),
    ("て", ["te"]),
    ("と", ["to"]),
    ("だ", ["da"]),
    ("ぢ", ["di"]),
    ("づ", ["du"]),
    ("で", ["de"]),
    ("ど", ["do"]),
    ("な", ["na"]),
    ("に", ["ni"]),
    ("ぬ", ["nu"]),
    ("ね", ["ne"]),
    ("の", ["no"]),
    ("は", ["ha"]),
    ("ひ", ["hi"]),
    ("ふ", ["hu", "fu"]),
    ("へ", ["he"]),
    ("ほ", ["ho"]),
    ("ば", ["ba"]),
    ("び", ["bi"]),
    ("ぶ", ["bu"]),
    ("べ", ["be"]),
    ("ぼ", ["bo"]),
    ("ぱ", ["pa"]),
    ("ぴ", ["pi"]),
    ("ぷ", ["pu"]),
    ("ぺ", ["pe"]),
    ("ぽ", ["po"]),
    ("ま", ["ma"]),
    ("み", ["mi"]),
    ("む", ["mu"]),
    ("め", ["me"]),
    ("も", ["mo"]),
    ("や", ["ya"]),
    ("ゆ", ["yu"]),
    ("よ", ["yo"]),
    ("ら", ["ra"]),
    ("り", ["ri"]),
    ("る", ["ru"]),
    ("れ", ["re"]),
    ("ろ", ["ro"]),
    ("わ", ["wa"]),
    ("を", ["wo"]),
    ("ん", ["nn"]),
    ("ぁ", ["xa", "la"]),
    ("ぃ", ["xi", "li"]),
    ("ぅ", ["xu", "lu"]),
    ("ぇ", ["xe", "le"]),
    ("ぉ", ["xo", "lo"]),
    ("きゃ", ["kya"]),
    ("きぃ", ["kyi"]),
    ("きゅ", ["kyu"]),
    ("きぇ", ["kye"]),
    ("きょ", ["kyo"]),
    ("くぁ", ["qa"]),
    ("くぃ", ["qi"]),
    ("くぅ", ["qwu"]),
    ("くぇ", ["qe"]),
    ("くぉ", ["qo"]),
    ("ぎゃ", ["gya"]),
    ("ぎぃ", ["gyi"]),
    ("ぎゅ", ["gyu"]),
    ("ぎぇ", ["gye"]),
    ("ぎょ", ["gyo"]),
    ("ぐぁ", ["gwa"]),
    ("ぐぃ", ["gwi"]),
    ("ぐぅ", ["gwu"]),
    ("ぐぇ", ["gwe"]),
    ("ぐぉ", ["gwo"]),
    ("しゃ", ["sya", "sha"]),
    ("しぃ", ["syi"]),
    ("しゅ", ["syu", "shu"]),
    ("しぇ", ["sye", "she"]),
    ("しょ", ["syo", "sho"]),
    ("すぁ", ["swa"]),
    ("すぃ", ["swi"]),
    ("すぅ", ["swu"]),
    ("すぇ", ["swe"]),
    ("すぉ", ["swo"]),
    ("じゃ", ["ja", "zya"]),
    ("じぃ", ["zyi"]),
    ("じゅ", ["ju", "zyu"]),
    ("じぇ", ["je", "zye"]),
    ("じょ", ["jo", "zyo"]),
    ("ちゃ", ["tya", "cha"]),
    ("ちぃ", ["tyi"]),
    ("ちゅ", ["tyu", "chu"]),
    ("ちぇ", ["tye", "che"]),
    ("ちょ", ["tyo", "cho"]),
    ("てゃ", ["tha"]),
    ("てぃ", ["thi"]),
    ("てゅ", ["thu"]),
    ("てぇ", ["the"]),
    ("てょ", ["tho"]),
    ("とぁ", ["twa"]),
    ("とぃ", ["twi"]),
    ("とぅ", ["two"]),
    ("とぇ", ["twe"]),
    ("とぉ", ["two"]),
    ("ぢゃ", ["dya"]),
    ("ぢぃ", ["dyi"]),
    ("ぢゅ", ["dyu"]),
    ("ぢぇ", ["dye"]),
    ("ぢょ", ["dyo"]),
    ("でゃ", ["dha"]),
    ("でぃ", ["dhi"]),
    ("でゅ", ["dhu"]),
    ("でぇ", ["dhe"]),
    ("でょ", ["dho"]),
    ("どぁ", ["dwa"]),
    ("どぃ", ["dwi"]),
    ("どぅ", ["dwu"]),
    ("どぇ", ["dwe"]),
    ("どぉ", ["dwo"]),
    ("にゃ", ["nya"]),
    ("にぃ", ["nyi"]),
    ("にゅ", ["nyu"]),
    ("にぇ", ["nye"]),
    ("にょ", ["nyo"]),
    ("ひゃ", ["hya"]),
    ("ひぃ", ["hyi"]),
    ("ひゅ", ["hyu"]),
    ("ひぇ", ["hye"]),
    ("ひょ", ["hyo"]),
    ("ふぁ", ["fa"]),
    ("ふぃ", ["fi"]),
    ("ふぅ", ["fwu"]),
    ("ふぇ", ["fe"]),
    ("ふぉ", ["fo"]),
    ("びゃ", ["bya"]),
    ("びぃ", ["byi"]),
    ("びゅ", ["byu"]),
    ("びぇ", ["bye"]),
    ("びょ", ["byo"]),
    ("ぴゃ", ["pya"]),
    ("ぴぃ", ["pyi"]),
    ("ぴゅ", ["pyu"]),
    ("ぴぇ", ["pye"]),
    ("ぴょ", ["pyo"]),
    ("みゃ", ["mya"]),
    ("みぃ", ["myi"]),
    ("みゅ", ["myu"]),
    ("みぇ", ["mye"]),
    ("みょ", ["myo"]),
    ("りゃ", ["rya"]),
    ("りぃ", ["ryi"]),
    ("りゅ", ["ryu"]),
    ("りぇ", ["rye"]),
    ("りょ", ["ryo"]),
    ("うぁ", ["wha"]),
    ("うぃ", ["wi"]),
    ("うぇ", ["we"]),
    ("うぉ", ["who"]),
];

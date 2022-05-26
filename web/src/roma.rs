use std::collections::HashMap;

use once_cell::sync::Lazy;
use smallvec::SmallVec;

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

static ROMA_TABLE: Lazy<HashMap<&[char], &[char]>> = Lazy::new(|| {
    [
        (['-'].as_slice(), ['ー'].as_slice()),
        ([','].as_slice(), ['、'].as_slice()),
        (['.'].as_slice(), ['。'].as_slice()),
        (['a'].as_slice(), ['あ'].as_slice()),
        (['i'].as_slice(), ['い'].as_slice()),
        (['u'].as_slice(), ['う'].as_slice()),
        (['e'].as_slice(), ['え'].as_slice()),
        (['o'].as_slice(), ['お'].as_slice()),
        (['k', 'a'].as_slice(), ['か'].as_slice()),
        (['k', 'i'].as_slice(), ['き'].as_slice()),
        (['k', 'u'].as_slice(), ['く'].as_slice()),
        (['k', 'e'].as_slice(), ['け'].as_slice()),
        (['k', 'o'].as_slice(), ['こ'].as_slice()),
        (['g', 'a'].as_slice(), ['が'].as_slice()),
        (['g', 'i'].as_slice(), ['ぎ'].as_slice()),
        (['g', 'u'].as_slice(), ['ぐ'].as_slice()),
        (['g', 'e'].as_slice(), ['げ'].as_slice()),
        (['g', 'o'].as_slice(), ['ご'].as_slice()),
        (['s', 'a'].as_slice(), ['さ'].as_slice()),
        (['s', 'i'].as_slice(), ['し'].as_slice()),
        (['s', 'u'].as_slice(), ['す'].as_slice()),
        (['s', 'e'].as_slice(), ['せ'].as_slice()),
        (['s', 'o'].as_slice(), ['そ'].as_slice()),
        (['z', 'a'].as_slice(), ['ざ'].as_slice()),
        (['z', 'i'].as_slice(), ['じ'].as_slice()),
        (['j', 'i'].as_slice(), ['じ'].as_slice()),
        (['z', 'u'].as_slice(), ['ず'].as_slice()),
        (['z', 'e'].as_slice(), ['ぜ'].as_slice()),
        (['z', 'o'].as_slice(), ['ぞ'].as_slice()),
        (['t', 'a'].as_slice(), ['た'].as_slice()),
        (['t', 'i'].as_slice(), ['ち'].as_slice()),
        (['c', 'h', 'i'].as_slice(), ['ち'].as_slice()),
        (['t', 'u'].as_slice(), ['つ'].as_slice()),
        (['t', 's', 'u'].as_slice(), ['つ'].as_slice()),
        (['t', 'e'].as_slice(), ['て'].as_slice()),
        (['t', 'o'].as_slice(), ['と'].as_slice()),
        (['d', 'a'].as_slice(), ['だ'].as_slice()),
        (['d', 'i'].as_slice(), ['ぢ'].as_slice()),
        (['d', 'u'].as_slice(), ['づ'].as_slice()),
        (['d', 'e'].as_slice(), ['で'].as_slice()),
        (['d', 'o'].as_slice(), ['ど'].as_slice()),
        (['n', 'a'].as_slice(), ['な'].as_slice()),
        (['n', 'i'].as_slice(), ['に'].as_slice()),
        (['n', 'u'].as_slice(), ['ぬ'].as_slice()),
        (['n', 'e'].as_slice(), ['ね'].as_slice()),
        (['n', 'o'].as_slice(), ['の'].as_slice()),
        (['h', 'a'].as_slice(), ['は'].as_slice()),
        (['h', 'i'].as_slice(), ['ひ'].as_slice()),
        (['h', 'u'].as_slice(), ['ふ'].as_slice()),
        (['f', 'u'].as_slice(), ['ふ'].as_slice()),
        (['h', 'e'].as_slice(), ['へ'].as_slice()),
        (['h', 'o'].as_slice(), ['ほ'].as_slice()),
        (['b', 'a'].as_slice(), ['ば'].as_slice()),
        (['b', 'i'].as_slice(), ['び'].as_slice()),
        (['b', 'u'].as_slice(), ['ぶ'].as_slice()),
        (['b', 'e'].as_slice(), ['べ'].as_slice()),
        (['b', 'o'].as_slice(), ['ぼ'].as_slice()),
        (['p', 'a'].as_slice(), ['ぱ'].as_slice()),
        (['p', 'i'].as_slice(), ['ぴ'].as_slice()),
        (['p', 'u'].as_slice(), ['ぷ'].as_slice()),
        (['p', 'e'].as_slice(), ['ぺ'].as_slice()),
        (['p', 'o'].as_slice(), ['ぽ'].as_slice()),
        (['m', 'a'].as_slice(), ['ま'].as_slice()),
        (['m', 'i'].as_slice(), ['み'].as_slice()),
        (['m', 'u'].as_slice(), ['む'].as_slice()),
        (['m', 'e'].as_slice(), ['め'].as_slice()),
        (['m', 'o'].as_slice(), ['も'].as_slice()),
        (['y', 'a'].as_slice(), ['や'].as_slice()),
        (['y', 'u'].as_slice(), ['ゆ'].as_slice()),
        (['y', 'o'].as_slice(), ['よ'].as_slice()),
        (['r', 'a'].as_slice(), ['ら'].as_slice()),
        (['r', 'i'].as_slice(), ['り'].as_slice()),
        (['r', 'u'].as_slice(), ['る'].as_slice()),
        (['r', 'e'].as_slice(), ['れ'].as_slice()),
        (['r', 'o'].as_slice(), ['ろ'].as_slice()),
        (['w', 'a'].as_slice(), ['わ'].as_slice()),
        (['w', 'o'].as_slice(), ['を'].as_slice()),
        (['n', 'n'].as_slice(), ['ん'].as_slice()),
        (['x', 'a'].as_slice(), ['ぁ'].as_slice()),
        (['l', 'a'].as_slice(), ['ぁ'].as_slice()),
        (['x', 'i'].as_slice(), ['ぃ'].as_slice()),
        (['l', 'i'].as_slice(), ['ぃ'].as_slice()),
        (['x', 'u'].as_slice(), ['ぅ'].as_slice()),
        (['l', 'u'].as_slice(), ['ぅ'].as_slice()),
        (['x', 'e'].as_slice(), ['ぇ'].as_slice()),
        (['l', 'e'].as_slice(), ['ぇ'].as_slice()),
        (['x', 'o'].as_slice(), ['ぉ'].as_slice()),
        (['l', 'o'].as_slice(), ['ぉ'].as_slice()),
        (['k', 'y', 'a'].as_slice(), ['き', 'ゃ'].as_slice()),
        (['k', 'y', 'i'].as_slice(), ['き', 'ぃ'].as_slice()),
        (['k', 'y', 'u'].as_slice(), ['き', 'ゅ'].as_slice()),
        (['k', 'y', 'e'].as_slice(), ['き', 'ぇ'].as_slice()),
        (['k', 'y', 'o'].as_slice(), ['き', 'ょ'].as_slice()),
        (['q', 'a'].as_slice(), ['く', 'ぁ'].as_slice()),
        (['q', 'i'].as_slice(), ['く', 'ぃ'].as_slice()),
        (['q', 'w', 'u'].as_slice(), ['く', 'ぅ'].as_slice()),
        (['q', 'e'].as_slice(), ['く', 'ぇ'].as_slice()),
        (['q', 'o'].as_slice(), ['く', 'ぉ'].as_slice()),
        (['g', 'y', 'a'].as_slice(), ['ぎ', 'ゃ'].as_slice()),
        (['g', 'y', 'i'].as_slice(), ['ぎ', 'ぃ'].as_slice()),
        (['g', 'y', 'u'].as_slice(), ['ぎ', 'ゅ'].as_slice()),
        (['g', 'y', 'e'].as_slice(), ['ぎ', 'ぇ'].as_slice()),
        (['g', 'y', 'o'].as_slice(), ['ぎ', 'ょ'].as_slice()),
        (['g', 'w', 'a'].as_slice(), ['ぐ', 'ぁ'].as_slice()),
        (['g', 'w', 'i'].as_slice(), ['ぐ', 'ぃ'].as_slice()),
        (['g', 'w', 'u'].as_slice(), ['ぐ', 'ぅ'].as_slice()),
        (['g', 'w', 'e'].as_slice(), ['ぐ', 'ぇ'].as_slice()),
        (['g', 'w', 'o'].as_slice(), ['ぐ', 'ぉ'].as_slice()),
        (['s', 'y', 'a'].as_slice(), ['し', 'ゃ'].as_slice()),
        (['s', 'h', 'a'].as_slice(), ['し', 'ゃ'].as_slice()),
        (['s', 'y', 'i'].as_slice(), ['し', 'ぃ'].as_slice()),
        (['s', 'y', 'u'].as_slice(), ['し', 'ゅ'].as_slice()),
        (['s', 'h', 'u'].as_slice(), ['し', 'ゅ'].as_slice()),
        (['s', 'y', 'e'].as_slice(), ['し', 'ぇ'].as_slice()),
        (['s', 'h', 'e'].as_slice(), ['し', 'ぇ'].as_slice()),
        (['s', 'y', 'o'].as_slice(), ['し', 'ょ'].as_slice()),
        (['s', 'h', 'o'].as_slice(), ['し', 'ょ'].as_slice()),
        (['s', 'w', 'a'].as_slice(), ['す', 'ぁ'].as_slice()),
        (['s', 'w', 'i'].as_slice(), ['す', 'ぃ'].as_slice()),
        (['s', 'w', 'u'].as_slice(), ['す', 'ぅ'].as_slice()),
        (['s', 'w', 'e'].as_slice(), ['す', 'ぇ'].as_slice()),
        (['s', 'w', 'o'].as_slice(), ['す', 'ぉ'].as_slice()),
        (['j', 'a'].as_slice(), ['じ', 'ゃ'].as_slice()),
        (['z', 'y', 'a'].as_slice(), ['じ', 'ゃ'].as_slice()),
        (['z', 'y', 'i'].as_slice(), ['じ', 'ぃ'].as_slice()),
        (['j', 'u'].as_slice(), ['じ', 'ゅ'].as_slice()),
        (['z', 'y', 'u'].as_slice(), ['じ', 'ゅ'].as_slice()),
        (['j', 'e'].as_slice(), ['じ', 'ぇ'].as_slice()),
        (['z', 'y', 'e'].as_slice(), ['じ', 'ぇ'].as_slice()),
        (['j', 'o'].as_slice(), ['じ', 'ょ'].as_slice()),
        (['z', 'y', 'o'].as_slice(), ['じ', 'ょ'].as_slice()),
        (['t', 'y', 'a'].as_slice(), ['ち', 'ゃ'].as_slice()),
        (['c', 'h', 'a'].as_slice(), ['ち', 'ゃ'].as_slice()),
        (['t', 'y', 'i'].as_slice(), ['ち', 'ぃ'].as_slice()),
        (['t', 'y', 'u'].as_slice(), ['ち', 'ゅ'].as_slice()),
        (['c', 'h', 'u'].as_slice(), ['ち', 'ゅ'].as_slice()),
        (['t', 'y', 'e'].as_slice(), ['ち', 'ぇ'].as_slice()),
        (['c', 'h', 'e'].as_slice(), ['ち', 'ぇ'].as_slice()),
        (['t', 'y', 'o'].as_slice(), ['ち', 'ょ'].as_slice()),
        (['c', 'h', 'o'].as_slice(), ['ち', 'ょ'].as_slice()),
        (['t', 'h', 'a'].as_slice(), ['て', 'ゃ'].as_slice()),
        (['t', 'h', 'i'].as_slice(), ['て', 'ぃ'].as_slice()),
        (['t', 'h', 'u'].as_slice(), ['て', 'ゅ'].as_slice()),
        (['t', 'h', 'e'].as_slice(), ['て', 'ぇ'].as_slice()),
        (['t', 'h', 'o'].as_slice(), ['て', 'ょ'].as_slice()),
        (['t', 'w', 'a'].as_slice(), ['と', 'ぁ'].as_slice()),
        (['t', 'w', 'i'].as_slice(), ['と', 'ぃ'].as_slice()),
        (['t', 'w', 'o'].as_slice(), ['と', 'ぅ'].as_slice()),
        (['t', 'w', 'e'].as_slice(), ['と', 'ぇ'].as_slice()),
        (['t', 'w', 'o'].as_slice(), ['と', 'ぉ'].as_slice()),
        (['d', 'y', 'a'].as_slice(), ['ぢ', 'ゃ'].as_slice()),
        (['d', 'y', 'i'].as_slice(), ['ぢ', 'ぃ'].as_slice()),
        (['d', 'y', 'u'].as_slice(), ['ぢ', 'ゅ'].as_slice()),
        (['d', 'y', 'e'].as_slice(), ['ぢ', 'ぇ'].as_slice()),
        (['d', 'y', 'o'].as_slice(), ['ぢ', 'ょ'].as_slice()),
        (['d', 'h', 'a'].as_slice(), ['で', 'ゃ'].as_slice()),
        (['d', 'h', 'i'].as_slice(), ['で', 'ぃ'].as_slice()),
        (['d', 'h', 'u'].as_slice(), ['で', 'ゅ'].as_slice()),
        (['d', 'h', 'e'].as_slice(), ['で', 'ぇ'].as_slice()),
        (['d', 'h', 'o'].as_slice(), ['で', 'ょ'].as_slice()),
        (['d', 'w', 'a'].as_slice(), ['ど', 'ぁ'].as_slice()),
        (['d', 'w', 'i'].as_slice(), ['ど', 'ぃ'].as_slice()),
        (['d', 'w', 'u'].as_slice(), ['ど', 'ぅ'].as_slice()),
        (['d', 'w', 'e'].as_slice(), ['ど', 'ぇ'].as_slice()),
        (['d', 'w', 'o'].as_slice(), ['ど', 'ぉ'].as_slice()),
        (['n', 'y', 'a'].as_slice(), ['に', 'ゃ'].as_slice()),
        (['n', 'y', 'i'].as_slice(), ['に', 'ぃ'].as_slice()),
        (['n', 'y', 'u'].as_slice(), ['に', 'ゅ'].as_slice()),
        (['n', 'y', 'e'].as_slice(), ['に', 'ぇ'].as_slice()),
        (['n', 'y', 'o'].as_slice(), ['に', 'ょ'].as_slice()),
        (['h', 'y', 'a'].as_slice(), ['ひ', 'ゃ'].as_slice()),
        (['h', 'y', 'i'].as_slice(), ['ひ', 'ぃ'].as_slice()),
        (['h', 'y', 'u'].as_slice(), ['ひ', 'ゅ'].as_slice()),
        (['h', 'y', 'e'].as_slice(), ['ひ', 'ぇ'].as_slice()),
        (['h', 'y', 'o'].as_slice(), ['ひ', 'ょ'].as_slice()),
        (['f', 'a'].as_slice(), ['ふ', 'ぁ'].as_slice()),
        (['f', 'i'].as_slice(), ['ふ', 'ぃ'].as_slice()),
        (['f', 'w', 'u'].as_slice(), ['ふ', 'ぅ'].as_slice()),
        (['f', 'e'].as_slice(), ['ふ', 'ぇ'].as_slice()),
        (['f', 'o'].as_slice(), ['ふ', 'ぉ'].as_slice()),
        (['b', 'y', 'a'].as_slice(), ['び', 'ゃ'].as_slice()),
        (['b', 'y', 'i'].as_slice(), ['び', 'ぃ'].as_slice()),
        (['b', 'y', 'u'].as_slice(), ['び', 'ゅ'].as_slice()),
        (['b', 'y', 'e'].as_slice(), ['び', 'ぇ'].as_slice()),
        (['b', 'y', 'o'].as_slice(), ['び', 'ょ'].as_slice()),
        (['p', 'y', 'a'].as_slice(), ['ぴ', 'ゃ'].as_slice()),
        (['p', 'y', 'i'].as_slice(), ['ぴ', 'ぃ'].as_slice()),
        (['p', 'y', 'u'].as_slice(), ['ぴ', 'ゅ'].as_slice()),
        (['p', 'y', 'e'].as_slice(), ['ぴ', 'ぇ'].as_slice()),
        (['p', 'y', 'o'].as_slice(), ['ぴ', 'ょ'].as_slice()),
        (['m', 'y', 'a'].as_slice(), ['み', 'ゃ'].as_slice()),
        (['m', 'y', 'i'].as_slice(), ['み', 'ぃ'].as_slice()),
        (['m', 'y', 'u'].as_slice(), ['み', 'ゅ'].as_slice()),
        (['m', 'y', 'e'].as_slice(), ['み', 'ぇ'].as_slice()),
        (['m', 'y', 'o'].as_slice(), ['み', 'ょ'].as_slice()),
        (['r', 'y', 'a'].as_slice(), ['り', 'ゃ'].as_slice()),
        (['r', 'y', 'i'].as_slice(), ['り', 'ぃ'].as_slice()),
        (['r', 'y', 'u'].as_slice(), ['り', 'ゅ'].as_slice()),
        (['r', 'y', 'e'].as_slice(), ['り', 'ぇ'].as_slice()),
        (['r', 'y', 'o'].as_slice(), ['り', 'ょ'].as_slice()),
        (['w', 'h', 'a'].as_slice(), ['う', 'ぁ'].as_slice()),
        (['w', 'i'].as_slice(), ['う', 'ぃ'].as_slice()),
        (['w', 'e'].as_slice(), ['う', 'ぇ'].as_slice()),
        (['w', 'h', 'o'].as_slice(), ['う', 'ぉ'].as_slice()),
    ]
    .into_iter()
    .collect()
});

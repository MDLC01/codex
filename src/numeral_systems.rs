//! Various ways of displaying non-negative integers.

use chinese_number::{ChineseCase, ChineseVariant, from_u64_to_chinese_ten_thousand};
use std::fmt::{Display, Formatter};

/// Represents a numeral system of one of multiple predefined kinds.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum NumeralSystem<'a> {
    /// A big-endian
    /// [positional notation](https://en.wikipedia.org/wiki/Positional_notation)
    /// system.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any non-negative integer.
    ///
    /// ## Example
    ///
    /// With the digits `['0', '1', '2']`, we obtain the ternary numeral system:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 0      | 0              |
    /// | 1      | 1              |
    /// | 2      | 2              |
    /// | 3      | 10             |
    /// | 4      | 12             |
    /// | 5      | 12             |
    /// | 6      | 20             |
    Positional(&'a [char]),

    /// A big-endian
    /// [bijective numeration](https://en.wikipedia.org/wiki/Bijective_numeration)
    /// system. This is similar to positional notation, but without a digit for
    /// zero.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any positive integer.
    ///
    /// ## Example
    ///
    /// With the digits `['A', 'B', 'C']`, we obtain a system similar to one
    /// commonly used to number columns in spreadsheet software:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 1      | A              |
    /// | 2      | B              |
    /// | 3      | C              |
    /// | 4      | AA             |
    /// | 5      | AB             |
    /// | 6      | AC             |
    /// | 7      | BA             |
    Bijective(&'a [char]),

    /// An additive
    /// [sign-value notation](https://en.wikipedia.org/wiki/Sign-value_notation)
    /// system.
    ///
    /// The numerals must be specified by decreasing value.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any positive integer.
    ///
    /// ## Example
    ///
    /// With the numerals `[("V", 5), ("IV", 4), ("I", 1)]`, we obtain the start
    /// of the Roman numeral system:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 1      | I              |
    /// | 2      | II             |
    /// | 3      | III            |
    /// | 4      | IV             |
    /// | 5      | V              |
    /// | 6      | VI             |
    /// | 7      | VII            |
    Additive(&'a [(&'a str, u64)]),

    /// A system that uses repeating symbols.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any positive integer.
    ///
    /// ## Example
    ///
    /// With the symbols `['A', 'B', 'C']`, we obtain the following
    /// representations:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 1      | A              |
    /// | 2      | B              |
    /// | 3      | C              |
    /// | 4      | AA             |
    /// | 5      | BB             |
    /// | 6      | CC             |
    /// | 7      | AAA            |
    Symbolic(&'a [char]),

    /// A system that uses a fixed set of symbols to represent the first
    /// non-negative integers.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any non-negative integer.
    ///
    /// ## Example
    ///
    /// With the symbols `['A', 'B', 'C']`, we obtain the following
    /// representations:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 0      | A              |
    /// | 1      | B              |
    /// | 2      | C              |
    ZeroableFixed(&'a [char]),

    /// A system that uses a fixed set of symbols to represent the first
    /// positive integers.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any positive integer.
    ///
    /// ## Example
    ///
    /// With the symbols `['A', 'B', 'C']`, we obtain the following
    /// representations:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 1      | A              |
    /// | 2      | B              |
    /// | 3      | C              |
    NonZeroableFixed(&'a [char]),

    /// A Chinese numeral system.
    ///
    /// ## Representable Numbers
    ///
    /// Chinese numeral systems can represent any non-negative integer.
    ///
    /// ## Example
    ///
    /// With [`ChineseVariant::Simple`] and [`ChineseCase::Lower`], we
    /// obtain the following representations:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 0      | 零              |
    /// | 1      | 一              |
    /// | 2      | 二              |
    /// | 3      | 三              |
    /// | 4      | 四              |
    /// | 5      | 五              |
    /// | 6      | 六              |
    Chinese(ChineseVariant, ChineseCase),
}

impl<'a> NumeralSystem<'a> {
    pub const fn apply(
        &'a self,
        number: u64,
    ) -> Result<RepresentedNumber<'a>, RepresentationError> {
        match self {
            Self::Positional(_) | Self::Chinese(_, _) => {}
            Self::Bijective(_) | Self::Symbolic(_) => {
                if number == 0 {
                    return Err(RepresentationError::Zero);
                }
            }
            Self::Additive(numerals) => {
                if !matches!(numerals.last(), Some((_, 0))) {
                    return Err(RepresentationError::Zero);
                }
            }
            Self::ZeroableFixed(symbols) => {
                if number as usize >= symbols.len() {
                    return Err(RepresentationError::TooLarge);
                }
            }
            Self::NonZeroableFixed(symbols) => {
                if number == 0 {
                    return Err(RepresentationError::Zero);
                }
                if number as usize > symbols.len() {
                    return Err(RepresentationError::TooLarge);
                }
            }
        }
        Ok(RepresentedNumber { system: self, number })
    }

    /// Base-ten
    /// [Arabic numerals](https://en.wikipedia.org/wiki/Arabic_numerals): 0,
    /// 1, 2, 3, ...
    pub const ARABIC: Self =
        NumeralSystem::Positional(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

    /// Circled Arabic numerals up to fifty: ⓪, ①, ②, ...
    pub const CIRCLED_ARABIC: Self = NumeralSystem::ZeroableFixed(&[
        '⓪', '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', '⑪', '⑫', '⑬', '⑭', '⑮',
        '⑯', '⑰', '⑱', '⑲', '⑳', '㉑', '㉒', '㉓', '㉔', '㉕', '㉖', '㉗', '㉘', '㉙',
        '㉚', '㉛', '㉜', '㉝', '㉞', '㉟', '㊱', '㊲', '㊳', '㊴', '㊵', '㊶', '㊷',
        '㊸', '㊹', '㊺', '㊻', '㊼', '㊽', '㊾', '㊿',
    ]);

    /// Double-circled Arabic numerals up to ten: ⓵, ⓶, ⓷, ...
    pub const DOUBLE_CIRCLED_ARABIC: Self = NumeralSystem::NonZeroableFixed(&[
        '⓵', '⓶', '⓷', '⓸', '⓹', '⓺', '⓻', '⓼', '⓽', '⓾',
    ]);

    /// Lowercase
    /// [Latin letters](https://en.wikipedia.org/wiki/Latin_alphabet): a, b,
    /// c, ..., y, z, aa, ab, ...
    pub const LOWER_LATIN: Self = NumeralSystem::Bijective(&[
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]);

    /// Uppercase
    /// [Latin letters](https://en.wikipedia.org/wiki/Latin_alphabet): A, B,
    /// C, ..., Y, Z, AA, AB, ...
    pub const UPPER_LATIN: Self = NumeralSystem::Bijective(&[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]);

    /// Lowercase
    /// [Roman numerals](https://en.wikipedia.org/wiki/Roman_numerals): i,
    /// ii, iii, ...
    pub const LOWER_ROMAN: Self = NumeralSystem::Additive(&[
        ("m̅", 1000000),
        ("d̅", 500000),
        ("c̅", 100000),
        ("l̅", 50000),
        ("x̅", 10000),
        ("v̅", 5000),
        ("i̅v̅", 4000),
        ("m", 1000),
        ("cm", 900),
        ("d", 500),
        ("cd", 400),
        ("c", 100),
        ("xc", 90),
        ("l", 50),
        ("xl", 40),
        ("x", 10),
        ("ix", 9),
        ("v", 5),
        ("iv", 4),
        ("i", 1),
        ("n", 0),
    ]);

    /// Uppercase
    /// [Roman numerals](https://en.wikipedia.org/wiki/Roman_numerals): I,
    /// II, III, ...
    pub const UPPER_ROMAN: Self = NumeralSystem::Additive(&[
        ("M̅", 1000000),
        ("D̅", 500000),
        ("C̅", 100000),
        ("L̅", 50000),
        ("X̅", 10000),
        ("V̅", 5000),
        ("I̅V̅", 4000),
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
        ("N", 0),
    ]);

    /// Lowercase
    /// [Greek numerals](https://en.wikipedia.org/wiki/Greek_numerals): α,
    /// β, γ, ...
    pub const LOWER_GREEK: Self = NumeralSystem::Additive(&[
        ("͵θ", 9000),
        ("͵η", 8000),
        ("͵ζ", 7000),
        ("͵ϛ", 6000),
        ("͵ε", 5000),
        ("͵δ", 4000),
        ("͵γ", 3000),
        ("͵β", 2000),
        ("͵α", 1000),
        ("ϡ", 900),
        ("ω", 800),
        ("ψ", 700),
        ("χ", 600),
        ("φ", 500),
        ("υ", 400),
        ("τ", 300),
        ("σ", 200),
        ("ρ", 100),
        ("ϟ", 90),
        ("π", 80),
        ("ο", 70),
        ("ξ", 60),
        ("ν", 50),
        ("μ", 40),
        ("λ", 30),
        ("κ", 20),
        ("ι", 10),
        ("θ", 9),
        ("η", 8),
        ("ζ", 7),
        ("ϛ", 6),
        ("ε", 5),
        ("δ", 4),
        ("γ", 3),
        ("β", 2),
        ("α", 1),
        ("𐆊", 0),
    ]);

    /// Uppercase
    /// [Greek numerals](https://en.wikipedia.org/wiki/Greek_numerals): Α,
    /// Β, Γ, ...
    pub const UPPER_GREEK: Self = NumeralSystem::Additive(&[
        ("͵Θ", 9000),
        ("͵Η", 8000),
        ("͵Ζ", 7000),
        ("͵Ϛ", 6000),
        ("͵Ε", 5000),
        ("͵Δ", 4000),
        ("͵Γ", 3000),
        ("͵Β", 2000),
        ("͵Α", 1000),
        ("Ϡ", 900),
        ("Ω", 800),
        ("Ψ", 700),
        ("Χ", 600),
        ("Φ", 500),
        ("Υ", 400),
        ("Τ", 300),
        ("Σ", 200),
        ("Ρ", 100),
        ("Ϟ", 90),
        ("Π", 80),
        ("Ο", 70),
        ("Ξ", 60),
        ("Ν", 50),
        ("Μ", 40),
        ("Λ", 30),
        ("Κ", 20),
        ("Ι", 10),
        ("Θ", 9),
        ("Η", 8),
        ("Ζ", 7),
        ("Ϛ", 6),
        ("Ε", 5),
        ("Δ", 4),
        ("Γ", 3),
        ("Β", 2),
        ("Α", 1),
        ("𐆊", 0),
    ]);

    /// Hebrew numerals, including Geresh/Gershayim.
    pub const HEBREW: Self = NumeralSystem::Additive(&[
        ("ת", 400),
        ("ש", 300),
        ("ר", 200),
        ("ק", 100),
        ("צ", 90),
        ("פ", 80),
        ("ע", 70),
        ("ס", 60),
        ("נ", 50),
        ("מ", 40),
        ("ל", 30),
        ("כ", 20),
        ("יט", 19),
        ("יח", 18),
        ("יז", 17),
        ("טז", 16),
        ("טו", 15),
        ("י", 10),
        ("ט", 9),
        ("ח", 8),
        ("ז", 7),
        ("ו", 6),
        ("ה", 5),
        ("ד", 4),
        ("ג", 3),
        ("ב", 2),
        ("א", 1),
        ("-", 0),
    ]);

    /// Simplified Chinese standard numerals.
    pub const LOWER_SIMPLIFIED_CHINESE: Self =
        NumeralSystem::Chinese(ChineseVariant::Simple, ChineseCase::Lower);

    /// Simplified Chinese "banknote" numerals.
    pub const UPPER_SIMPLIFIED_CHINESE: Self =
        NumeralSystem::Chinese(ChineseVariant::Simple, ChineseCase::Upper);

    /// Traditional Chinese standard numerals.
    pub const LOWER_TRADITIONAL_CHINESE: Self =
        NumeralSystem::Chinese(ChineseVariant::Traditional, ChineseCase::Lower);

    /// Traditional Chinese "banknote" numerals.
    pub const UPPER_TRADITIONAL_CHINESE: Self =
        NumeralSystem::Chinese(ChineseVariant::Traditional, ChineseCase::Upper);

    /// Hiragana in the gojūon order. Includes n but excludes wi and we.
    pub const HIRAGANA_AIUEO: Self = NumeralSystem::Bijective(&[
        'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ', 'し', 'す',
        'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に', 'ぬ', 'ね', 'の', 'は',
        'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ', 'ら',
        'り', 'る', 'れ', 'ろ', 'わ', 'を', 'ん',
    ]);

    /// Hiragana in the iroha order. Includes wi and we but excludes n.
    pub const HIRAGANA_IROHA: Self = NumeralSystem::Bijective(&[
        'い', 'ろ', 'は', 'に', 'ほ', 'へ', 'と', 'ち', 'り', 'ぬ', 'る', 'を', 'わ',
        'か', 'よ', 'た', 'れ', 'そ', 'つ', 'ね', 'な', 'ら', 'む', 'う', 'ゐ', 'の',
        'お', 'く', 'や', 'ま', 'け', 'ふ', 'こ', 'え', 'て', 'あ', 'さ', 'き', 'ゆ',
        'め', 'み', 'し', 'ゑ', 'ひ', 'も', 'せ', 'す',
    ]);

    /// Katakana in the gojūon order. Includes n but excludes wi and we.
    pub const KATAKANA_AIUEO: Self = NumeralSystem::Bijective(&[
        'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス',
        'セ', 'ソ', 'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ',
        'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ', 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ',
        'リ', 'ル', 'レ', 'ロ', 'ワ', 'ヲ', 'ン',
    ]);

    /// Katakana in the iroha order. Includes wi and we but excludes n.
    pub const KATAKANA_IROHA: Self = NumeralSystem::Bijective(&[
        'イ', 'ロ', 'ハ', 'ニ', 'ホ', 'ヘ', 'ト', 'チ', 'リ', 'ヌ', 'ル', 'ヲ', 'ワ',
        'カ', 'ヨ', 'タ', 'レ', 'ソ', 'ツ', 'ネ', 'ナ', 'ラ', 'ム', 'ウ', 'ヰ', 'ノ',
        'オ', 'ク', 'ヤ', 'マ', 'ケ', 'フ', 'コ', 'エ', 'テ', 'ア', 'サ', 'キ', 'ユ',
        'メ', 'ミ', 'シ', 'ヱ', 'ヒ', 'モ', 'セ', 'ス',
    ]);

    /// Korean jamo: ㄱ, ㄴ, ㄷ, ...
    pub const KOREAN_JAMO: Self = NumeralSystem::Bijective(&[
        'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ',
        'ㅎ',
    ]);

    /// Korean syllables: 가, 나, 다, ...
    pub const KOREAN_SYLLABLE: Self = NumeralSystem::Bijective(&[
        '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '카', '타', '파',
        '하',
    ]);

    /// Eastern Arabic numerals, used in some Arabic-speaking countries.
    pub const EASTERN_ARABIC: Self =
        NumeralSystem::Positional(&['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩']);

    /// The variant of Eastern Arabic numerals used in Persian and Urdu.
    pub const EASTERN_ARABIC_PERSIAN: Self =
        NumeralSystem::Positional(&['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹']);

    /// Devanagari numerals.
    pub const DEVANAGARI_NUMBER: Self =
        NumeralSystem::Positional(&['०', '१', '२', '३', '४', '५', '६', '७', '८', '९']);

    /// Bengali numerals.
    pub const BENGALI_NUMBER: Self =
        NumeralSystem::Positional(&['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯']);

    /// Bengali letters: ক, খ, গ, ..., কক, কখ, ...
    pub const BENGALI_LETTER: Self = NumeralSystem::Bijective(&[
        'ক', 'খ', 'গ', 'ঘ', 'ঙ', 'চ', 'ছ', 'জ', 'ঝ', 'ঞ', 'ট', 'ঠ', 'ড', 'ঢ', 'ণ', 'ত',
        'থ', 'দ', 'ধ', 'ন', 'প', 'ফ', 'ব', 'ভ', 'ম', 'য', 'র', 'ল', 'শ', 'ষ', 'স', 'হ',
    ]);

    /// [Paragraph/note-like symbols](https://en.wikipedia.org/wiki/Note_(typography)#Numbering_and_symbols):
    /// *, †, ‡, §, ¶, and ‖.
    ///
    /// Further items use repeated symbols.
    pub const SYMBOL: Self = NumeralSystem::Symbolic(&['*', '†', '‡', '§', '¶', '‖']);
}

/// A number, together with a numeral system in which it is representable.
///
/// Notably, this type implements [`Display`] and is thus compatible with
/// [`format!()`].
#[derive(Debug, Clone, Copy)]
pub struct RepresentedNumber<'a> {
    /// Invariant: This system must be able to represent the number.
    system: &'a NumeralSystem<'a>,
    number: u64,
}

impl<'a> Display for RepresentedNumber<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.system {
            NumeralSystem::Positional(digits) => {
                let mut n = self.number;

                if n == 0 {
                    return write!(f, "{}", digits[0]);
                }

                let radix = digits.len() as u64;
                let size = n.ilog(radix) + 1;
                // The place value of the most significant digit. For a number
                // of size 1, the MSD's place is the ones place, hence `- 1`.
                let mut msd_place = radix.pow(size - 1);
                for _ in 0..size {
                    let msd = n / msd_place;
                    write!(f, "{}", digits[msd as usize])?;
                    n -= msd * msd_place;
                    msd_place /= radix;
                }
                Ok(())
            }

            NumeralSystem::Bijective(digits) => {
                let mut n = self.number;

                assert_ne!(n, 0);

                let radix = digits.len() as u64;
                // Number of digits when representing `n` in this system.
                // From https://en.wikipedia.org/wiki/Bijective_numeration#Properties_of_bijective_base-k_numerals.
                let size = ((n + 1) * (radix - 1)).ilog(radix);
                // Remove from `n` the number consisting of `size - 1` ones in
                // base-`radix`, and the print the result using the symbols as
                // a positional numeral system.
                n -= (radix.pow(size) - 1) / (radix - 1);
                // The place value of the most significant digit. For a number
                // of size 1, the MSD's place is the ones place, hence `- 1`.
                let mut msd_place = radix.pow(size - 1);
                for _ in 0..size {
                    let msd = n / msd_place;
                    write!(f, "{}", digits[msd as usize])?;
                    n -= msd * msd_place;
                    msd_place /= radix;
                }
                Ok(())
            }

            NumeralSystem::Additive(numerals) => {
                let mut n = self.number;

                if n == 0 {
                    if let Some(&(numeral, 0)) = numerals.last() {
                        return write!(f, "{}", numeral);
                    }
                    unreachable!()
                }

                // Greedily add any symbol that fits.
                for (numeral, weight) in *numerals {
                    if *weight == 0 || *weight > n {
                        continue;
                    }
                    let reps = n / weight;
                    for _ in 0..reps {
                        write!(f, "{}", numeral)?
                    }

                    n -= weight * reps;
                }
                Ok(())
            }
            NumeralSystem::Symbolic(symbols) => {
                let n = self.number;
                assert_ne!(n, 0);
                let symbol_count = symbols.len() as u64;
                for _ in 0..n.div_ceil(symbol_count) {
                    write!(f, "{}", symbols[((n - 1) % symbol_count) as usize])?
                }
                Ok(())
            }

            NumeralSystem::ZeroableFixed(symbols) => {
                write!(f, "{}", symbols[self.number as usize])
            }

            NumeralSystem::NonZeroableFixed(symbols) => {
                write!(f, "{}", symbols[(self.number - 1) as usize])
            }

            NumeralSystem::Chinese(variant, case) => write!(
                f,
                "{}",
                from_u64_to_chinese_ten_thousand(*variant, *case, self.number),
            ),
        }
    }
}

/// A reason why a number cannot be represented in a numeral system.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RepresentationError {
    /// Zero cannot be represented in the numeral system.
    Zero,
    /// The number is too large for the numeral system.
    TooLarge,
}

#[cfg(test)]
mod tests {
    use super::NumeralSystem;

    #[test]
    fn test_arabic_numerals() {
        for n in 0..=9999 {
            assert_eq!(NumeralSystem::ARABIC.apply(n).unwrap().to_string(), n.to_string(),)
        }
    }

    #[test]
    fn test_latin() {
        let mut n = 1;
        for c1 in 'a'..='z' {
            assert_eq!(
                NumeralSystem::LOWER_LATIN.apply(n).unwrap().to_string(),
                format!("{c1}"),
            );
            assert_eq!(
                NumeralSystem::UPPER_LATIN.apply(n).unwrap().to_string(),
                format!("{c1}").to_uppercase(),
            );
            n += 1
        }
        for c2 in 'a'..='z' {
            for c1 in 'a'..='z' {
                assert_eq!(
                    NumeralSystem::LOWER_LATIN.apply(n).unwrap().to_string(),
                    format!("{c2}{c1}"),
                );
                assert_eq!(
                    NumeralSystem::UPPER_LATIN.apply(n).unwrap().to_string(),
                    format!("{c2}{c1}").to_uppercase(),
                );
                n += 1
            }
        }
        for c3 in 'a'..='z' {
            for c2 in 'a'..='z' {
                for c1 in 'a'..='z' {
                    assert_eq!(
                        NumeralSystem::LOWER_LATIN.apply(n).unwrap().to_string(),
                        format!("{c3}{c2}{c1}"),
                    );
                    assert_eq!(
                        NumeralSystem::UPPER_LATIN.apply(n).unwrap().to_string(),
                        format!("{c3}{c2}{c1}").to_uppercase(),
                    );
                    n += 1
                }
            }
        }
    }

    #[test]
    fn test_roman() {
        for (n, expect) in [
            "n", "i", "ii", "iii", "iv", "v", "vi", "vii", "viii", "ix", "x", "xi",
            "xii", "xiii", "xiv", "xv", "xvi", "xvii", "xviii", "xix", "xx", "xxi",
            "xxii", "xxiii", "xxiv", "xxv", "xxvi", "xxvii", "xxviii", "xxix", "xxx",
            "xxxi", "xxxii", "xxxiii", "xxxiv", "xxxv", "xxxvi", "xxxvii", "xxxviii",
            "xxxix", "xl", "xli", "xlii", "xliii", "xliv", "xlv", "xlvi",
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(
                &NumeralSystem::LOWER_ROMAN.apply(n as u64).unwrap().to_string(),
                expect,
            );
            assert_eq!(
                NumeralSystem::UPPER_ROMAN.apply(n as u64).unwrap().to_string(),
                expect.to_uppercase(),
            );
        }
    }
}

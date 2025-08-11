//! Various ways of displaying integers.

use chinese_number::{from_u64_to_chinese_ten_thousand, ChineseCase, ChineseVariant};
use ecow::{eco_format, EcoString};

macro_rules! declare_variants {
    {
        $( #[$attr:meta] )*
        $vis:vis enum $Variants:ident {
            $(
                $( #[$variant_attr:meta] )*
                $variant:ident = $name:literal,
            )*
        }
    } => {
        $( #[$attr] )*
        $vis enum $Variants {
            $(
                $( #[$variant_attr] )*
                $variant,
            )*
        }

        impl $Variants {
            pub fn from_name(s: &str) -> Option<Self> {
                match s {
                    $( $name => Some(Self::$variant), )*
                    _ => None,
                }
            }

            pub fn name(self) -> &'static str {
                match self {
                    $( Self::$variant => $name, )*
                }
            }
        }
    };
}

declare_variants! {
    /// Various numeral systems used worldwide.
    #[non_exhaustive]
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum NumeralSystem {
        /// Base-ten Arabic numerals: 0, 1, 2, 3, ...
        Arabic = "arabic",
        /// Lowercase Latin letters: a, b, c, ..., y, z, aa, ab, ...
        LowerLatin = "latin",
        /// Uppercase Latin letters: A, B, C, ..., Y, Z, AA, AB, ...
        UpperLatin = "Latin",
        /// Lowercase Roman numerals: i, ii, iii, ...
        LowerRoman = "roman",
        /// Uppercase Roman numerals: I, II, III, ...
        UpperRoman = "Roman",
        /// Lowercase Greek letters: α, β, γ, ...
        LowerGreek = "greek",
        /// Uppercase Greek letters: Α, Β, Γ, ...
        UpperGreek = "Greek",
        /// Paragraph/note-like symbols: *, †, ‡, §, ¶, and ‖. Further items use
        /// repeated symbols.
        Symbol = "symbols",
        /// Hebrew numerals, including Geresh/Gershayim.
        Hebrew = "hebrew",
        /// Simplified Chinese standard numerals. This corresponds to the
        /// `ChineseCase::Lower` variant.
        LowerSimplifiedChinese = "chinese.simplified",
        /// Simplified Chinese "banknote" numerals. This corresponds to the
        /// `ChineseCase::Upper` variant.
        UpperSimplifiedChinese = "Chinese.simplified",
        /// Traditional Chinese standard numerals. This corresponds to the
        /// `ChineseCase::Lower` variant.
        LowerTraditionalChinese = "chinese.traditional",
        /// Traditional Chinese "banknote" numerals. This corresponds to the
        /// `ChineseCase::Upper` variant.
        UpperTraditionalChinese = "Chinese.traditional",
        /// Hiragana in the gojūon order. Includes n but excludes wi and we.
        HiraganaAiueo = "hiragana.aiueo",
        /// Hiragana in the iroha order. Includes wi and we but excludes n.
        HiraganaIroha = "hiragana.iroha",
        /// Katakana in the gojūon order. Includes n but excludes wi and we.
        KatakanaAiueo = "katakana.aiueo",
        /// Katakana in the iroha order. Includes wi and we but excludes n.
        KatakanaIroha = "katakana.oroha",
        /// Korean jamo: ㄱ, ㄴ, ㄷ, ...
        KoreanJamo = "korean.jamo",
        /// Korean syllables: 가, 나, 다, ...
        KoreanSyllable = "korean.syllable",
        /// Eastern Arabic numerals, used in some Arabic-speaking countries.
        EasternArabic = "arabic.eastern",
        /// The variant of Eastern Arabic numerals used in Persian and Urdu.
        EasternArabicPersian = "arabic.persian",
        /// Devanagari numerals.
        DevanagariNumber = "devanagari",
        /// Bengali numerals.
        BengaliNumber = "bengali.number",
        /// Bengali letters: ক, খ, গ, ..., কক, কখ, ...
        BengaliLetter = "bengali.letter",
        /// Circled numbers up to fifty: ①, ②, ③, ...
        CircledNumber = "circled",
        /// Double-circled numbers up to ten: ⓵, ⓶, ⓷, ...
        DoubleCircledNumber = "circled.double",
    }
}

impl NumeralSystem {
    /// Represents a non-negative integer with this numeral system.
    pub fn apply(self, n: u64) -> EcoString {
        match self {
            Self::Arabic => {
                numeric(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], n)
            }
            Self::LowerRoman => additive(
                &[
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
                ],
                n,
            ),
            Self::UpperRoman => additive(
                &[
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
                ],
                n,
            ),
            Self::LowerGreek => additive(
                &[
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
                ],
                n,
            ),
            Self::UpperGreek => additive(
                &[
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
                ],
                n,
            ),
            Self::Hebrew => additive(
                &[
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
                ],
                n,
            ),
            Self::LowerLatin => alphabetic(
                &[
                    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                ],
                n,
            ),
            Self::UpperLatin => alphabetic(
                &[
                    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
                    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                ],
                n,
            ),
            Self::HiraganaAiueo => alphabetic(
                &[
                    'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ',
                    'し', 'す', 'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に',
                    'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み', 'む',
                    'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'わ',
                    'を', 'ん',
                ],
                n,
            ),
            Self::HiraganaIroha => alphabetic(
                &[
                    'い', 'ろ', 'は', 'に', 'ほ', 'へ', 'と', 'ち', 'り', 'ぬ', 'る',
                    'を', 'わ', 'か', 'よ', 'た', 'れ', 'そ', 'つ', 'ね', 'な', 'ら',
                    'む', 'う', 'ゐ', 'の', 'お', 'く', 'や', 'ま', 'け', 'ふ', 'こ',
                    'え', 'て', 'あ', 'さ', 'き', 'ゆ', 'め', 'み', 'し', 'ゑ', 'ひ',
                    'も', 'せ', 'す',
                ],
                n,
            ),
            Self::KatakanaAiueo => alphabetic(
                &[
                    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ',
                    'シ', 'ス', 'セ', 'ソ', 'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ',
                    'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ', 'ム',
                    'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ',
                    'ヲ', 'ン',
                ],
                n,
            ),
            Self::KatakanaIroha => alphabetic(
                &[
                    'イ', 'ロ', 'ハ', 'ニ', 'ホ', 'ヘ', 'ト', 'チ', 'リ', 'ヌ', 'ル',
                    'ヲ', 'ワ', 'カ', 'ヨ', 'タ', 'レ', 'ソ', 'ツ', 'ネ', 'ナ', 'ラ',
                    'ム', 'ウ', 'ヰ', 'ノ', 'オ', 'ク', 'ヤ', 'マ', 'ケ', 'フ', 'コ',
                    'エ', 'テ', 'ア', 'サ', 'キ', 'ユ', 'メ', 'ミ', 'シ', 'ヱ', 'ヒ',
                    'モ', 'セ', 'ス',
                ],
                n,
            ),
            Self::KoreanJamo => alphabetic(
                &[
                    'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ',
                    'ㅌ', 'ㅍ', 'ㅎ',
                ],
                n,
            ),
            Self::KoreanSyllable => alphabetic(
                &[
                    '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '카',
                    '타', '파', '하',
                ],
                n,
            ),
            Self::BengaliLetter => alphabetic(
                &[
                    'ক', 'খ', 'গ', 'ঘ', 'ঙ', 'চ', 'ছ', 'জ', 'ঝ', 'ঞ', 'ট', 'ঠ', 'ড', 'ঢ',
                    'ণ', 'ত', 'থ', 'দ', 'ধ', 'ন', 'প', 'ফ', 'ব', 'ভ', 'ম', 'য', 'র', 'ল',
                    'শ', 'ষ', 'স', 'হ',
                ],
                n,
            ),
            Self::CircledNumber => fixed(
                &[
                    '⓪', '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', '⑪', '⑫', '⑬',
                    '⑭', '⑮', '⑯', '⑰', '⑱', '⑲', '⑳', '㉑', '㉒', '㉓', '㉔', '㉕',
                    '㉖', '㉗', '㉘', '㉙', '㉚', '㉛', '㉜', '㉝', '㉞', '㉟', '㊱',
                    '㊲', '㊳', '㊴', '㊵', '㊶', '㊷', '㊸', '㊹', '㊺', '㊻', '㊼',
                    '㊽', '㊾', '㊿',
                ],
                n,
            ),
            Self::DoubleCircledNumber => {
                fixed(&['0', '⓵', '⓶', '⓷', '⓸', '⓹', '⓺', '⓻', '⓼', '⓽', '⓾'], n)
            }

            Self::LowerSimplifiedChinese => from_u64_to_chinese_ten_thousand(
                ChineseVariant::Simple,
                ChineseCase::Lower,
                n,
            )
            .into(),
            Self::UpperSimplifiedChinese => from_u64_to_chinese_ten_thousand(
                ChineseVariant::Simple,
                ChineseCase::Upper,
                n,
            )
            .into(),
            Self::LowerTraditionalChinese => from_u64_to_chinese_ten_thousand(
                ChineseVariant::Traditional,
                ChineseCase::Lower,
                n,
            )
            .into(),
            Self::UpperTraditionalChinese => from_u64_to_chinese_ten_thousand(
                ChineseVariant::Traditional,
                ChineseCase::Upper,
                n,
            )
            .into(),

            Self::EasternArabic => {
                numeric(&['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'], n)
            }
            Self::EasternArabicPersian => {
                numeric(&['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'], n)
            }
            Self::DevanagariNumber => {
                numeric(&['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'], n)
            }
            Self::BengaliNumber => {
                numeric(&['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'], n)
            }
            Self::Symbol => symbolic(&['*', '†', '‡', '§', '¶', '‖'], n),
        }
    }
}

/// Formats a number using a
/// [sign-value notation](https://en.wikipedia.org/wiki/Sign-value_notation).
///
/// The symbols must be specified by decreasing values.
///
/// The value of a stringified number is recovered by summing over the values of
/// the symbols present.
///
/// Consider the situation where `[("V", 5), ("IV", 4), ("I", 1)]` are the
/// provided symbols:
///
/// ```text
/// 1 => 'I'
/// 2 => 'II'
/// 3 => 'III'
/// 4 => 'IV'
/// 5 => 'V'
/// 6 => 'VI'
/// 7 => 'VII'
/// 8 => 'VIII'
/// ```
///
/// This is the start of the familiar Roman numeral system.
fn additive(symbols: &[(&str, u64)], mut n: u64) -> EcoString {
    if n == 0 {
        if let Some(&(symbol, 0)) = symbols.last() {
            return symbol.into();
        }
        return '0'.into();
    }

    let mut s = EcoString::new();
    for (symbol, weight) in symbols {
        if *weight == 0 || *weight > n {
            continue;
        }
        let reps = n / weight;
        for _ in 0..reps {
            s.push_str(symbol);
        }

        n -= weight * reps;
        if n == 0 {
            return s;
        }
    }
    s
}

/// Formats a number using a big-endian
/// [bijective base-_b_](https://en.wikipedia.org/wiki/Bijective_numeration)
/// system (where _b_ is the number of provided symbols). This is similar to
/// regular base-_b_ systems, but without a symbol for zero.
///
/// Consider the situation where `['A', 'B', 'C']` are the provided symbols:
///
/// ```text
/// 1 => "A"
/// 2 => "B"
/// 3 => "C"
/// 4 => "AA"
/// 5 => "AB"
/// 6 => "AC"
/// 7 => "BA"
/// ...
/// ```
///
/// A similar system is commonly used in spreadsheet software.
fn alphabetic(symbols: &[char], mut n: u64) -> EcoString {
    let n_digits = symbols.len() as u64;
    if n == 0 {
        return '-'.into();
    }
    let mut s = EcoString::new();
    while n != 0 {
        n -= 1;
        s.push(symbols[(n % n_digits) as usize]);
        n /= n_digits;
    }
    s.chars().rev().collect()
}

/// Formats a number using the symbols provided, defaulting to the arabic
/// representation when the number is greater than the number of symbols.
///
/// Consider the situation where `['0', 'A', 'B', 'C']` are the provided
/// symbols:
///
/// ```text
/// 0 => "0"
/// 1 => "A"
/// 2 => "B"
/// 3 => "C"
/// 4 => "4"
/// ...
/// ```
fn fixed(symbols: &[char], n: u64) -> EcoString {
    let n_digits = symbols.len() as u64;
    if n < n_digits {
        return symbols[n as usize].into();
    }
    eco_format!("{n}")
}

/// Formats a number using a big-endian
/// [positional notation](https://en.wikipedia.org/wiki/Positional_notation).
///
/// Consider the situation where `['0', '1', '2']` are the provided symbols:
///
/// ```text
/// 0 => "0"
/// 1 => "1"
/// 2 => "2"
/// 3 => "10"
/// 4 => "11"
/// 5 => "12"
/// 6 => "20"
/// ...
/// ```
///
/// This is the familiar ternary numeral system.
fn numeric(symbols: &[char], mut n: u64) -> EcoString {
    let n_digits = symbols.len() as u64;
    if n == 0 {
        return symbols[0].into();
    }
    let mut s = EcoString::new();
    while n != 0 {
        s.push(symbols[(n % n_digits) as usize]);
        n /= n_digits;
    }
    s.chars().rev().collect()
}

/// Formats a number using repeating symbols.
///
/// Consider the situation where `['A', 'B', 'C']` are the provided symbols:
///
/// ```text
/// 0 => "-"
/// 1 => "A"
/// 2 => "B"
/// 3 => "C"
/// 4 => "AA"
/// 5 => "BB"
/// 6 => "CC"
/// 7 => "AAA"
/// ...
/// ```
fn symbolic(symbols: &[char], n: u64) -> EcoString {
    let n_digits = symbols.len() as u64;
    if n == 0 {
        return '-'.into();
    }
    EcoString::from(symbols[((n - 1) % n_digits) as usize])
        .repeat(n.div_ceil(n_digits) as usize)
}

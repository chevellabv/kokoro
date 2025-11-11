/// 文本到国际音标的转换
mod v10;
mod v11;

use super::PinyinError;
use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese};
#[cfg(feature = "use-cmudict")]
use cmudict_fast::{Cmudict, Error as CmudictError};
use pinyin::ToPinyin;
use regex::{Captures, Error as RegexError, Regex};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub enum G2PError {
    #[cfg(feature = "use-cmudict")]
    CmudictError(CmudictError),
    EnptyData,
    #[cfg(not(feature = "use-cmudict"))]
    Nul(std::ffi::NulError),
    Pinyin(PinyinError),
    Regex(RegexError),
    #[cfg(not(feature = "use-cmudict"))]
    Utf8(std::str::Utf8Error),
}

impl Display for G2PError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "G2PError: ")?;
        match self {
            #[cfg(feature = "use-cmudict")]
            Self::CmudictError(e) => Display::fmt(e, f),
            Self::EnptyData => Display::fmt("EmptyData", f),
            #[cfg(not(feature = "use-cmudict"))]
            Self::Nul(e) => Display::fmt(e, f),
            Self::Pinyin(e) => Display::fmt(e, f),
            Self::Regex(e) => Display::fmt(e, f),
            #[cfg(not(feature = "use-cmudict"))]
            Self::Utf8(e) => Display::fmt(e, f),
        }
    }
}

impl Error for G2PError {}

impl From<PinyinError> for G2PError {
    fn from(value: PinyinError) -> Self {
        Self::Pinyin(value)
    }
}

impl From<RegexError> for G2PError {
    fn from(value: RegexError) -> Self {
        Self::Regex(value)
    }
}

#[cfg(feature = "use-cmudict")]
impl From<CmudictError> for G2PError {
    fn from(value: CmudictError) -> Self {
        Self::CmudictError(value)
    }
}

#[cfg(not(feature = "use-cmudict"))]
impl From<std::ffi::NulError> for G2PError {
    fn from(value: std::ffi::NulError) -> Self {
        Self::Nul(value)
    }
}

#[cfg(not(feature = "use-cmudict"))]
impl From<std::str::Utf8Error> for G2PError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

fn word2ipa_zh(word: &str) -> Result<String, G2PError> {
    let iter = word.chars().map(|i| match i.to_pinyin() {
        None => Ok(i.to_string()),
        Some(p) => v10::py2ipa(p.with_tone_num_end()),
    });

    let mut result = String::new();
    for i in iter {
        result.push_str(&i?);
    }
    Ok(result)
}

#[cfg(feature = "use-cmudict")]
fn word2ipa_en(word: &str) -> Result<String, G2PError> {
    use super::{arpa_to_ipa, letters_to_ipa};
    use std::{
        io::{Error as IoError, ErrorKind},
        str::FromStr,
        sync::LazyLock,
    };

    fn get_cmudict<'a>() -> Result<&'a Cmudict, CmudictError> {
        static CMUDICT: LazyLock<Result<Cmudict, CmudictError>> =
            LazyLock::new(|| Cmudict::from_str(include_str!("../dict/cmudict.dict")));
        CMUDICT.as_ref().map_err(|i| match i {
            CmudictError::IoErr(e) => CmudictError::IoErr(IoError::new(ErrorKind::Other, e)),
            CmudictError::InvalidLine(e) => CmudictError::InvalidLine(*e),
            CmudictError::RuleParseError(e) => CmudictError::RuleParseError(e.clone()),
        })
    }

    // Lowercase the word for CMUDict lookup (CMUDict only has lowercase entries)
    let word_lower = word.to_lowercase();

    // Try compound word splitting for common prefixes if word not found
    let Some(rules) = get_cmudict()?.get(&word_lower) else {
        // Try splitting compound words with common prefixes
        let prefixes = ["down", "up", "out", "under", "over", "in", "off"];
        for prefix in prefixes {
            if word_lower.starts_with(prefix) && word_lower.len() > prefix.len() {
                let rest = &word_lower[prefix.len()..];
                // Check if both parts exist in dictionary
                if get_cmudict()?.get(prefix).is_some() && get_cmudict()?.get(rest).is_some() {
                    // Recursively get phonemes for both parts
                    let prefix_ipa = word2ipa_en(prefix)?;
                    let rest_ipa = word2ipa_en(rest)?;
                    return Ok(format!("{}{}", prefix_ipa, rest_ipa));
                }
            }
        }
        return Ok(letters_to_ipa(word));
    };
    if rules.is_empty() {
        return Ok(word.to_owned());
    }
    let i = rand::random_range(0..rules.len());
    let result = rules[i]
        .pronunciation()
        .iter()
        .map(|i| arpa_to_ipa(&i.to_string()).unwrap_or_default())
        .collect::<String>();
    Ok(result)
}

#[cfg(not(feature = "use-cmudict"))]
fn word2ipa_en(word: &str) -> Result<String, G2PError> {
    use super::letters_to_ipa;
    use std::{
        ffi::{CStr, CString, c_char},
        sync::Once,
    };

    // Handle common contractions that eSpeak pronounces incorrectly
    let word_lower = word.to_lowercase();
    let contraction_phonemes = match word_lower.as_str() {
        "you're" | "youre" => Some("jɔːɹ"),     // Like "your", not "you re"
        "they're" | "theyre" => Some("ðɛɹ"),    // Like "there"
        "we're" | "were" => Some("wɪɹ"),        // Like "weer"
        "you'll" | "youll" => Some("juːl"),     // "yool"
        "i'll" | "ill" => Some("aɪl"),          // Be careful with "ill" (sick)
        "he'll" | "hell" => Some("hiːl"),       // Be careful with "hell"
        "she'll" | "shell" => Some("ʃiːl"),     // Be careful with "shell"
        "we'll" | "well" => Some("wiːl"),       // Be careful with "well"
        "they'll" | "theyll" => Some("ðeɪl"),   // "they'll"
        "won't" | "wont" => Some("woʊnt"),      // "wohnt"
        "can't" | "cant" => Some("kænt"),       // "kant"
        "don't" | "dont" => Some("doʊnt"),      // "dohnt"
        "doesn't" | "doesnt" => Some("dʌzənt"), // "duzzent"
        "didn't" | "didnt" => Some("dɪdənt"),   // "diddent"
        "wouldn't" | "wouldnt" => Some("wʊdənt"), // "woodent"
        "shouldn't" | "shouldnt" => Some("ʃʊdənt"), // "shoodent"
        "couldn't" | "couldnt" => Some("kʊdənt"), // "coodent"
        "i'm" | "im" => Some("aɪm"),            // "ime"
        "that's" | "thats" => Some("ðæts"),     // "thats"
        "what's" | "whats" => Some("wʌts"),     // "whuts"
        "it's" | "its" => Some("ɪts"),          // "its"
        "let's" | "lets" => Some("lɛts"),       // "lets"
        _ => None
    };

    if let Some(phonemes) = contraction_phonemes {
        // Only use contraction if word actually has apostrophe or is known contraction
        if word.contains('\'') || word.contains('\u{2019}') {
            return Ok(phonemes.to_string());
        }
    }

    // Handle common abbreviations before they get spelled out letter-by-letter
    let abbreviation_phonemes = match word_lower.as_str() {
        "ai" => Some("ˈeɪaɪ"),     // "ay-eye"
        "ui" => Some("juːˈaɪ"),    // "you-eye"
        "api" => Some("ˈeɪpiːˈaɪ"), // "ay-pee-eye"
        "cpu" => Some("siːpiːjˈuː"), // "see-pee-you"
        "gpu" => Some("dʒiːpiːjˈuː"), // "gee-pee-you"
        "usb" => Some("juːˈɛsbiː"), // "you-ess-bee"
        _ => None
    };

    if let Some(phonemes) = abbreviation_phonemes {
        return Ok(phonemes.to_string());
    }

    if word.chars().count() < 4 && word.chars().all(|c| c.is_ascii_uppercase()) {
        return Ok(letters_to_ipa(word));
    }

    unsafe extern "C" {
        fn TextToPhonemes(text: *const c_char) -> *const ::std::os::raw::c_char;
        fn Initialize(data_dictlist: *const c_char);
    }

    unsafe {
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            // Priority 1: Try bundled espeak-ng-data (shipped with library)
            static BUNDLED_EN_DICT: &[u8] = include_bytes!("../espeak-ng-data/en_dict");
            Initialize(BUNDLED_EN_DICT.as_ptr() as _);

            // Note: If ESPEAK_NG_DATA_DIR override is needed, users can still set it
            // but bundled data takes priority for consistency
        });

        let word_lower = word.to_lowercase();
        let word_cstr = CString::new(word_lower.clone())?.into_raw() as *const c_char;
        let res = TextToPhonemes(word_cstr);
        let mut result = CStr::from_ptr(res).to_str()?.to_string();

        // eSpeak sometimes includes lookahead context after ||
        // e.g., "too" -> "tˈuː||mʌtʃ" which causes "much" to appear twice
        // Strip everything after || to get only the current word's phonemes
        if let Some(pos) = result.find("||") {
            result = result[..pos].to_string();
        }

        // eSpeak includes stress/prosody markers as ASCII digits (0-9)
        // v1.0 NOTE: Keep stress markers for better intonation!
        // Stress markers provide crucial prosody information for natural speech
        // v0.19 had issues with them, but v1.0 handles them correctly
        // Examples: "the" → "ðə2", "time" → "t2ˈaɪmɛ", "improvement" → "ˈɪmpɹəʊvɪmˌɛnt"
        let mut cleaned_result = result.clone(); // Keep stress markers!

        // eSpeak also appends context words that should be stripped
        // Examples:
        // "this" → "ðˈɪswˌɒn" (includes "one")
        // "will" → "wˈɪltə" (includes "to")
        // "fastest" → "fˈastɛsənt" (includes "escent/ascent")
        // "for" → "fˈɔːwɒn" (includes "one")
        // "that" → "ðɐthɐzbˈɪn" (includes "has been")
        // "should" → "ʃˈʊdhavtə" (includes "have" + "to")
        // "we" → "wiːʃˈal" (includes "shall")
        // "adjust" → "ɐdʒˈʌsənt" (includes "ənt")
        // These are common context suffixes that need removal
        // Ordered by length (longest first) to match greedily
        let context_suffixes = [
            "hɐzbˈɪn", // "has been"
            "ɛsənt",   // "escent/ascent" (appears in superlatives like "fastest")
            "ɔnðə",    // "on the"
            "ʃˈal",    // "shall"
            "wˌɒn",    // "one" (with stress)
            "wɒn",     // "one" (without stress)
            "ɪntʊ",    // "into" (when it's context)
            "ənt",     // schwa + "nt" suffix (appears in adjust, etc.)
            "hav",     // "have"
            "tə",      // "to"
            "ðə",      // "the"
        ];

        for suffix in &context_suffixes {
            if cleaned_result.ends_with(suffix) && cleaned_result.len() > suffix.len() {
                cleaned_result = cleaned_result[..cleaned_result.len() - suffix.len()].to_string();
                break;
            }
        }

        // Fix archaic "hw" pronunciation to modern American English "w"
        // eSpeak uses "hw" for words like "what", "when", "where", "which", "why"
        // Modern American English pronounces these with just "w" (not "hw")
        // Examples:
        //   "what" → "hwˈət" should be "wˈət"
        //   "when" → "hwˈɛn" should be "wˈɛn"
        //   "where" → "hwˈɛɹ" should be "wˈɛɹ"
        //   "which" → "hwˈɪtʃ" should be "wˈɪtʃ"
        //   "why" → "hwˈaɪ" should be "wˈaɪ"
        if cleaned_result.starts_with("hw") {
            cleaned_result = cleaned_result.replacen("hw", "w", 1);
        }

        Ok(cleaned_result)
    }
}

fn to_half_shape(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 2); // 预分配合理空间
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // 处理需要后看的情况
            '«' | '《' => result.push_str("“"),
            '»' | '》' => result.push_str("”"),
            '（' => result.push_str("("),
            '）' => result.push_str(")"),
            // 简单替换规则
            '、' | '，' => result.push_str(","),
            '。' => result.push_str("."),
            '！' => result.push_str("!"),
            '：' => result.push_str(":"),
            '；' => result.push_str(";"),
            '？' => result.push_str("?"),
            // 默认字符
            _ => result.push(c),
        }
    }

    // 清理多余空格并返回
    result
}

fn num_repr(text: &str) -> Result<String, G2PError> {
    let regex = Regex::new(r#"\d+(\.\d+)?"#)?;
    Ok(regex
        .replace(text, |caps: &Captures| {
            let text = &caps[0];
            if let Ok(num) = text.parse::<f64>() {
                num.to_chinese(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    ChineseCountMethod::Low,
                )
                .map_or(text.to_owned(), |i| i)
            } else if let Ok(num) = text.parse::<i64>() {
                num.to_chinese(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    ChineseCountMethod::Low,
                )
                .map_or(text.to_owned(), |i| i)
            } else {
                text.to_owned()
            }
        })
        .to_string())
}

pub fn g2p(text: &str, use_v11: bool) -> Result<String, G2PError> {
    // Only convert numbers to Chinese for v1.1 (Chinese model)
    // v1.0 is English and should keep numbers as-is or spell them out
    let text = if use_v11 { num_repr(&text)? } else { text.to_string() };
    let sentence_pattern = Regex::new(
        r#"([\u4E00-\u9FFF]+)|([，。：·？、！《》（）【】〖〗〔〕""''〈〉…—　]+)|([\u0000-\u00FF]+)+"#,
    )?;
    // Keep apostrophes within words to handle contractions like "you're"
    let en_word_pattern = Regex::new(r"[\w']+|[^\w']+")?;
    let jieba = jieba_rs::Jieba::new();
    let mut result = String::new();
    for i in sentence_pattern.captures_iter(&text) {
        match (i.get(1), i.get(2), i.get(3)) {
            (Some(text), _, _) => {
                let text = to_half_shape(text.as_str());
                if use_v11 {
                    if !result.is_empty() && !result.ends_with(' ') {
                        result.push(' ');
                    }
                    result.push_str(&v11::g2p(&text, true));
                    result.push(' ');
                } else {
                    for i in jieba.cut(&text, true) {
                        result.push_str(&word2ipa_zh(i)?);
                        result.push(' ');
                    }
                }
            }
            (_, Some(text), _) => {
                let text = to_half_shape(text.as_str());
                result = result.trim_end().to_string();
                result.push_str(&text);
                result.push(' ');
            }
            (_, _, Some(text)) => {
                for i in en_word_pattern.captures_iter(text.as_str()) {
                    let c = (&i[0]).chars().nth(0).unwrap_or_default();
                    if c == '\''
                        || c == '_'
                        || c == '-'
                        || c <= 'z' && c >= 'a'
                        || c <= 'Z' && c >= 'A'
                    {
                        let i = &i[0];
                        if result
                            .trim_end()
                            .ends_with(|c| c == '.' || c == ',' || c == '!' || c == '?')
                            && !result.ends_with(' ')
                        {
                            result.push(' ');
                        }
                        result.push_str(&word2ipa_en(i)?);
                    } else if c == ' ' && result.ends_with(' ') {
                        result.push_str((&i[0]).trim_start());
                    } else {
                        result.push_str(&i[0]);
                    }
                }
            }
            _ => (),
        };
    }

    // Second pass: Fix wh- word pronunciation for modern American English
    // Strategy: Replace "hw" → "w" but also fix the vowel to distinguish from other w- words
    let mut result = result.trim().to_string();

    // Fix "what" pronunciation: need to catch both "hwˈət" and "wˈət" variants
    // eSpeak is inconsistent:
    //   - Lowercase "what" → "hwˈət"
    //   - Capitalized "What" → "wˈət" (no hw!)
    // Both need to become "wˈʌt" (ʌ = "uh" vowel, distinguishes from "with" = "wˈɪθ")

    // Fix "hwˈət" → "wˈʌt" (lowercase "what")
    result = result.replace("hwˈət", "wˈʌt");
    result = result.replace(" hwˈət", " wˈʌt");

    // Fix "wˈət" → "wˈʌt" (capitalized "What" or in sentences)
    // This is safe because English doesn't have other common words with "wˈət" phoneme
    result = result.replace("wˈət", "wˈʌt");
    result = result.replace(" wˈət", " wˈʌt");

    // For other hw → w conversions that don't need vowel change
    if result.starts_with("hw") {
        result = format!("w{}", &result[2..]);
    }
    result = result.replace(" hw", " w");

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "use-cmudict"))]
    #[test]
    fn test_word2ipa_en() -> Result<(), super::G2PError> {
        use super::word2ipa_en;

        // println!("{:?}", espeak_rs::text_to_phonemes("days", "en", None, true, false));
        assert_eq!("kjˌuːkjˈuː", word2ipa_en("qq")?);
        assert_eq!("həlˈəʊ", word2ipa_en("hello")?);
        assert_eq!("wˈɜːld", word2ipa_en("world")?);
        assert_eq!("ˈapəl", word2ipa_en("apple")?);
        assert_eq!("tʃˈɪldɹɛn", word2ipa_en("children")?);
        assert_eq!("ˈaʊə", word2ipa_en("hour")?);
        assert_eq!("dˈeɪz", word2ipa_en("days")?);

        Ok(())
    }

    #[test]
    fn test_g2p() -> Result<(), super::G2PError> {
        use super::g2p;

        assert_eq!("ni↓xau↓ ʂɻ↘ʨje↘", g2p("你好世界", false)?);
        assert_eq!("ㄋㄧ2ㄏㄠ3/ㄕ十4ㄐㄝ4", g2p("你好世界", true)?);

        Ok(())
    }
}

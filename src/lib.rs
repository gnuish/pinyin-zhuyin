#![warn(missing_docs)]
//! Convert between pinyin forms or zhuyin.
extern crate phf;

use std::str;
use std::string::String;
use std::vec::Vec;

// MAP_P2Z and MAP_Z2P static maps
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

static PINYIN_TONES: [[char; 5]; 6] =
   [['a', 'ā', 'á', 'ǎ', 'à'],
    ['o', 'ō', 'ó', 'ǒ', 'ò'],
    ['e', 'ē', 'é', 'ě', 'è'],
    ['i', 'ī', 'í', 'ǐ', 'ì'],
    ['u', 'ū', 'ú', 'ǔ', 'ù'],
    ['ü', 'ǖ', 'ǘ', 'ǚ', 'ǜ']];
static ZHUYIN_TONES: [&str; 5] =
    ["˙", "", "ˊ", "ˇ", "ˋ"];

/// Returns the toned char for `c` of `tone` in pinyin
fn get_tonal_mark<C>(c: C, tone: u8) -> char
    where C: Into<char>
{
    let mut c = c.into();
    if c == 'v' {
        c = 'ü';
    }
    for t in PINYIN_TONES.iter() {
        if c == t[0] {
            return t[tone as usize]
        }
    }
    unreachable!();
}

fn tone_rhyme(s: &[u8], tone: u8) -> String {
    let mut ret = String::with_capacity(3);

    // If only one character, tone it and done
    if s.len() == 1 {
        ret.push(get_tonal_mark(s[0], tone));
        return ret
    }

    let (c1, c2) = (s[0], s[1]);

    // Tone the 1st character if:
    // * the 1st character is 'a'
    // * the 1st character is 'o' or 'e' and there's no 'a'
    // * the 2nd character is not a rhyme
    if c1 == b'a' || ((c1 == b'o' || c1 == b'e') && c2 != b'a') || !is_rhyme(c2) {
        ret.push(get_tonal_mark(c1, tone));
        ret.push_str(unsafe { str::from_utf8_unchecked(&s[1..]) });
        return ret
    }

    // Tone the 2nd character otherwise
    ret.push(c1 as char);
    ret.push(get_tonal_mark(c2, tone));
    ret.push_str(unsafe { str::from_utf8_unchecked(&s[2..]) });
    return ret
}

fn is_rhyme(c: u8) -> bool {
    return c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' || c == b'v'
}

fn is_consonant(c: u8) -> bool {
    return c >= b'a' && c <= b'z' && !is_rhyme(c)
}

/// Decode a toned rhyme to (rhyme, tone)
fn decode_rhyme(s: &str) -> Option<(String, u8)> {
    let mut rhyme = String::with_capacity(4);
    let mut tone = 0;

    // TODO possibly iterate over bytes
    // Push each char to the return string, un-accenting chars along the way
    for mut c in s.chars() {
        for t in PINYIN_TONES.iter() {
            for j in 1..t.len() {
                if c == t[j] {
                    // If there's two toned vowels, it's malformed
                    c = t[0];
                    if tone != 0 {
                        return None
                    }
                    tone = j as u8;
                }
            }
        }
        if c == 'ü' {
            c = 'v';
        }
        rhyme.push(c);
    }

    if tone == 0 {
        tone = 5;
    }
    Some((rhyme, tone))
}

/// Split numbered pinyin to (consonant, rhyme, tone)
///
/// No tone number will be converted to tone 5.
///
/// # Examples
/// ```
/// # use zhuyin::*;
/// assert_eq!(split("shuang1"), Some(("sh", "uang", 1)));
/// assert_eq!(split("a"), Some(("", "a", 5)));
///
/// assert_eq!(split("zh9"), None);
/// ```
pub fn split(s: &str) -> Option<(&str, &str, u8)> {
    if s == "r" || s == "r5" {
        return Some(("", "r", 5))
    }
    _split(s)
}

fn _split(s: &str) -> Option<(&str, &str, u8)> {
    let s = s.as_bytes();
    let mut pos = 0;

    for &b in s.iter() {
        if !is_consonant(b) {
            break
        }
        pos += 1;
    }
    let consonant = unsafe { str::from_utf8_unchecked(&s[0..pos]) };

    for &b in s.iter().skip(pos) {
        if b < b'a' || b > b'z' {
            break;
        }
        pos += 1;
    }
    let rhyme = unsafe { str::from_utf8_unchecked(&s[consonant.len()..pos]) };

    // Rhyme could not be empty, and the length of tone is at most 1
    if rhyme.len() == 0 || s.len() - pos > 2 {
        return None
    }

    let tone;
    if pos < s.len() {
        tone = s[pos] - b'0';
        if tone > 5 {
            return None
        }
    } else {
        tone = 5;
    }
    Some((consonant, rhyme, tone))
}

/// Encode pinyin
///
/// No tone number will be converted to tone 5.
///
/// # Examples
/// ```
/// # use zhuyin::*;
/// assert_eq!(encode_pinyin("ma3"), Some("mǎ".to_owned()));
/// assert_eq!(encode_pinyin("er2"), Some("ér".to_owned()));
/// assert_eq!(encode_pinyin("r5"), Some("r".to_owned()));
///
/// assert_eq!(encode_pinyin("llama"), None);
/// ```
pub fn encode_pinyin<S>(s: S) -> Option<String>
    where S: AsRef<str>
{
    let s = s.as_ref();

    if s == "e5" {
        return Some("ê".to_owned())
    } else if s == "r" || s == "r5" {
        return Some("r".to_owned())
    }

    let (consonant, rhyme, mut tone) = _split(s)?;
    if tone == 5 {
        tone = 0;
    }
    encode_pinyin_from_parts(consonant, rhyme, tone)
}

fn encode_pinyin_from_parts(consonant: &str, rhyme: &str, tone: u8) -> Option<String> {
    let consonant = consonant.as_bytes();
    let mut rhyme = rhyme.as_bytes().to_vec();

    if consonant.len() > 0 {
        // Is it a valid consonant?
        if MAP_P2Z.get(unsafe { str::from_utf8_unchecked(consonant) }) == None {
            return None
        }

        // Convert 'ü' to 'u' if consonant is 'j', 'q', 'x' or 'y'
        if rhyme[0] == b'v' {
            let c = consonant[0];
            if c == b'j' || c == b'q' || c == b'x' || c == b'y' {
                rhyme[0] = b'u';
            }
        }
    }

    // Is it a valid rhyme?
    if MAP_P2Z.get(unsafe { str::from_utf8_unchecked(&rhyme) }) == None {
        return None
    }

    // Tone the rhyme and convert 'v' to 'ü'
    let mut rhyme: String = tone_rhyme(&rhyme, tone);
    if rhyme.as_bytes()[0] == b'v' {
        rhyme = "ü".to_owned() + &rhyme[1..];
    }

    return Some(unsafe { str::from_utf8_unchecked(&consonant).to_owned() } + &rhyme)
}

/// Decode pinyin
///
/// Returns None if invalid pinyin
///
/// # Example
/// ```
/// # use zhuyin::*;
/// assert_eq!(decode_pinyin("mǎ"), Some("ma3".to_owned()));
/// assert_eq!(decode_pinyin("ér"), Some("er2".to_owned()));
/// assert_eq!(decode_pinyin("r"), Some("r5".to_owned()));
/// ```
pub fn decode_pinyin<S>(s: S) -> Option<String>
    where S: AsRef<str>
{
    let s = s.as_ref();

    if s == "ê" {
        return Some("e5".to_owned())
    } else if s == "r" {
        return Some("r5".to_owned())
    }

    let (consonant, rhyme, tone) = decode_pinyin_to_parts(s)?;

    let mut ret = Vec::with_capacity(7);
    ret.extend_from_slice(&consonant.as_bytes());
    ret.extend_from_slice(&rhyme.as_bytes());
    ret.push(tone + b'0');
    Some(unsafe { String::from_utf8_unchecked(ret) })
}

fn decode_pinyin_to_parts(s: &str) -> Option<(&str, String, u8)> {
    let s = s.as_bytes();
    let mut consonant: &[u8] = &[];
    let mut rhyme: &[u8] = &[];

    for (i, &b) in s.iter().enumerate() {
        if !is_consonant(b) {
            consonant = &s[..i];
            rhyme = &s[i..];
            break;
        }
    }

    // Is it a valid consonant?
    if consonant.len() > 0 {
        if MAP_P2Z.get(unsafe { str::from_utf8_unchecked(consonant) }) == None {
            return None
        }
    }

    let (mut untoned_rhyme, tone) = decode_rhyme(unsafe { str::from_utf8_unchecked(rhyme) })?;
    {
        let rhyme_bytes = unsafe { untoned_rhyme.as_bytes_mut() };
        // convert 'u' to 'v' if consonant is 'j', 'q', 'x' or 'y'
        if consonant.len() > 0 && rhyme_bytes[0] == b'u' {
            let c = consonant[0];
            if c == b'j' || c == b'q' || c == b'x' || c == b'y' {
                rhyme_bytes[0] = b'v';
            }
        }
    }

    // Is it a valid rhyme?
    if MAP_P2Z.get(&*untoned_rhyme) == None {
        return None
    }

    let consonant = unsafe { str::from_utf8_unchecked(consonant) };
    Some((consonant, untoned_rhyme, tone))
}

/// Encode zhuyin
///
/// No tone number will be converted to tone 5.
///
/// # Example
/// ```
/// # use zhuyin::*;
/// assert_eq!(encode_zhuyin("ma3"), Some("ㄇㄚˇ".to_owned()));
/// ```
pub fn encode_zhuyin<S>(s: S) -> Option<String>
    where S: AsRef<str>
{
    let s = s.as_ref();
    if s == "e5" {
        return Some("ㄝ".to_owned())
    } else if s == "r" || s == "r5" {
        return Some("ㄦ˙".to_owned())
    }

    let (consonant, rhyme, mut tone) = _split(s)?;
    if tone == 5 {
        tone = 0;
    }
    encode_zhuyin_from_parts(consonant, rhyme, tone)
}

fn encode_zhuyin_from_parts(consonant: &str, rhyme: &str, tone: u8) -> Option<String> {
    // Let's give this capacity to append rhyme and tone later
    let mut ret_vec = Vec::with_capacity(11); 
    ret_vec.extend_from_slice(consonant.as_bytes());
    let mut consonant = ret_vec;

    let mut rhyme = rhyme.as_bytes().to_vec();

    // Convert 'u' to 'v' since it's enforced in Zhuyin and our table
    if rhyme[0] == b'u' && consonant.len() > 0 {
        let c = consonant[0];
        if c == b'j' || c == b'q' || c == b'x' || c == b'y' {
            rhyme[0] = b'v';
        }
    }

    // Handle 整體認讀
    if rhyme == b"i" {
        if consonant == b"zh" || consonant == b"ch" || consonant == b"sh" ||
            consonant == b"r" || consonant == b"z" || consonant == b"c" ||
            consonant == b"s" || consonant == b"y" {
                rhyme.clear();
        }
    } else if consonant == b"w" {
        if rhyme == b"u" {
            consonant.clear();
        }
    } else if consonant == b"y" {
        if rhyme == b"v" || rhyme == b"e" || rhyme == b"ve" || rhyme == b"in" ||
            rhyme == b"van" || rhyme == b"ing" || rhyme == b"vn" {
                consonant.clear();
        }
    }

    if consonant.len() > 0 {
        if let Some(zhuyin) = MAP_P2Z.get(unsafe { str::from_utf8_unchecked(&consonant) }) {
            consonant.clear();
            consonant.extend_from_slice(zhuyin.as_bytes());
        } else {
            return None
        }
    }

    if rhyme.len() > 0 {
        if let Some(zhuyin) = MAP_P2Z.get(unsafe { str::from_utf8_unchecked(&rhyme) }) {
            rhyme.clear();
            rhyme.extend_from_slice(zhuyin.as_bytes());
        } else {
            return None
        }
    }

    consonant.extend_from_slice(&rhyme);
    let mut ret = unsafe { String::from_utf8_unchecked(consonant) };
    ret.push_str(ZHUYIN_TONES[tone as usize]);
    Some(ret)
}

/// Decode zhuyin
///
/// # Example
/// ```
/// # use zhuyin::*;
/// assert_eq!(decode_zhuyin("ㄇㄚˇ"), Some("ma3".to_owned()));
/// ```
pub fn decode_zhuyin<S>(s: S) -> Option<String>
    where S: AsRef<str>
{
    let s = s.as_ref();
    if s == "ㄝ" {
        return Some("e5".to_owned())
    } else if s == "ㄦ˙" {
        return Some("r5".to_owned())
    }

    let (consonant, rhyme, tone) = decode_zhuyin_to_parts(s)?;
    if rhyme.len() == 0 {
        return None
    }
    let mut ret = Vec::with_capacity(7);
    ret.extend_from_slice(&consonant.as_bytes());
    ret.extend_from_slice(&rhyme.as_bytes());
    ret.push(tone + b'0');
    Some(unsafe { String::from_utf8_unchecked(ret) })
}

fn decode_zhuyin_to_parts(s: &str) -> Option<(String, String, u8)> {
    let mut consonant = Vec::with_capacity(2);
    let mut rhyme = Vec::with_capacity(4);
    let mut tone: u8 = 1;
    let mut useless_char_buf = [0; 4];

    // TODO is iterating over bytes instead worth it?
    'split_input: for (i, c) in s.char_indices() {
        let c = c.encode_utf8(&mut useless_char_buf);
        if let Some(decoded) = MAP_Z2P.get(c) {
            // Add char as consonant or rhyme accordingly
            let decoded = decoded.as_bytes();
            match i == 0 && is_consonant(decoded[0]) {
                true => consonant.extend_from_slice(decoded),
                false => rhyme.extend_from_slice(c.as_bytes()),
            }
            continue
        }

        // The last char should be a tone
        let tone_slice = &s[i..];
        for (j, t) in ZHUYIN_TONES.into_iter().enumerate() {
            if &tone_slice == t {
                if j == 0 {
                    tone = 5;
                } else {
                    tone = j as u8;
                }
                break 'split_input;
            }
        }
        return None
    }

    if rhyme.len() == 0 {
        // If 整體認讀, the rhyme should be 'i'
        match consonant.as_slice() {
            b"zh" | b"ch" | b"sh" | b"r" |
            b"z" | b"c" | b"s" => {
                rhyme.push(b'i');
                return Some(unsafe {
                    (String::from_utf8_unchecked(consonant),
                    String::from_utf8_unchecked(rhyme),
                    tone)
                })
            },
            _ => return None
        }
    }

	// Is it an valid rhyme?
    match MAP_Z2P.get(unsafe { str::from_utf8_unchecked(&rhyme) }) {
        Some(decoded) if is_rhyme(decoded.as_bytes()[0]) => {
            rhyme.clear();
            rhyme.extend_from_slice(decoded.as_bytes());
        },
        _ => return None,
    };

	if consonant.len() == 0 {
        // Handle yi, wu, yv 整體認讀, and the special case "ong" to "weng"
		if rhyme == b"i" || rhyme == b"v" || rhyme == b"e" ||
			rhyme == b"ve" || rhyme == b"in" || rhyme == b"van" ||
			rhyme == b"ing" || rhyme == b"vn" {
                consonant.clear();
                consonant.push(b'y');
		} else if rhyme == b"u" {
            consonant.clear();
            consonant.push(b'w');
		} else if rhyme[0] == b'u' {
            consonant.clear();
            consonant.push(b'w');
            rhyme.drain(0..1);
		} else if rhyme[0] == b'i' {
			consonant.clear();
            consonant.push(b'y');
            rhyme.drain(0..1);
		} else if rhyme == b"ong" {
            consonant.clear();
            consonant.push(b'w');
            rhyme.clear();
            rhyme.extend_from_slice(b"eng");
		}
	}

    Some(unsafe {
        (String::from_utf8_unchecked(consonant),
        String::from_utf8_unchecked(rhyme),
        tone)
    })
}

/// Convert pinyin to zhuyin
///
/// # Example
/// ```
/// # use zhuyin::*;
/// assert_eq!(pinyin_to_zhuyin("mǎ"), Some("ㄇㄚˇ".to_owned()));
/// ```
pub fn pinyin_to_zhuyin<S>(s: S) -> Option<String>
    where S: AsRef<str>
{
    let s = s.as_ref();
    if s == "ê" {
		return Some("ㄝ".to_owned())
    }
    encode_zhuyin(decode_pinyin(s)?)
}

/// Convert zhuyin to pinyin
///
/// # Example
/// ```
/// # use zhuyin::*;
/// assert_eq!(zhuyin_to_pinyin("ㄇㄚˇ"), Some("mǎ".to_owned()));
/// ```
pub fn zhuyin_to_pinyin<S>(s: S) -> Option<String>
    where S: AsRef<str>
{
    let s = s.as_ref();
    if s == "ㄝ" {
		return Some("ê".to_owned())
    }
    encode_pinyin(decode_zhuyin(s)?)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_macros)]
    macro_rules! s(
        ($i:expr) => (Some($i.to_owned()));
    );

    #[test]
    fn encode_pinyin_test() {
        assert_eq!(encode_pinyin("e5"), s!("ê"));
        assert_eq!(encode_pinyin("ju0"), s!("ju"));
        assert_eq!(encode_pinyin("jv"), s!("ju"));
        assert_eq!(encode_pinyin("lv3"), s!("lǚ"));
        assert_eq!(encode_pinyin("lvan4"), s!("lüàn")); // not valid, for test only
        assert_eq!(encode_pinyin("zhuan4"), s!("zhuàn"));
        assert_eq!(encode_pinyin("zhao2"), s!("zháo"));
        assert_eq!(encode_pinyin("leng1"), s!("lēng"));
        assert_eq!(encode_pinyin("shui3"), s!("shuǐ"));
        assert_eq!(encode_pinyin("liu2"), s!("liú"));
        assert_eq!(encode_pinyin("an3"), s!("ǎn"));
        assert_eq!(encode_pinyin("yi2"), s!("yí"));
        assert_eq!(encode_pinyin("yuan2"), s!("yuán"));
        assert_eq!(encode_pinyin("yvan2"), s!("yuán"));
        assert_eq!(encode_pinyin("min2"), s!("mín"));
        assert_eq!(encode_pinyin("er2"), s!("ér"));
        assert_eq!(encode_pinyin("r5"), s!("r"));

        assert_eq!(encode_pinyin("alkfj"), None);
        assert_eq!(encode_pinyin("zh3"), None);
        assert_eq!(encode_pinyin("zhaang4"), None);
        assert_eq!(encode_pinyin("你好"), None);
        assert_eq!(encode_pinyin("a你好"), None);
        assert_eq!(encode_pinyin("你好zhang1"), None);
        assert_eq!(encode_pinyin(""), None);
    }

    #[test]
    fn decode_pinyin_test() {
        assert_eq!(decode_pinyin("ê"), s!("e5"));
        assert_eq!(decode_pinyin("ju"), s!("jv5"));
        assert_eq!(decode_pinyin("lǚ"), s!("lv3"));
        assert_eq!(decode_pinyin("lüàn"), s!("lvan4")); // not valid, for test only
        assert_eq!(decode_pinyin("zhuàn"), s!("zhuan4"));
        assert_eq!(decode_pinyin("zháo"), s!("zhao2"));
        assert_eq!(decode_pinyin("lēng"), s!("leng1"));
        assert_eq!(decode_pinyin("shuǐ"), s!("shui3"));
        assert_eq!(decode_pinyin("liú"), s!("liu2"));
        assert_eq!(decode_pinyin("ǎn"), s!("an3"));
        assert_eq!(decode_pinyin("yí"), s!("yi2"));
        assert_eq!(decode_pinyin("yuán"), s!("yvan2"));
        assert_eq!(decode_pinyin("mín"), s!("min2"));
        assert_eq!(decode_pinyin("ér"), s!("er2"));
        assert_eq!(decode_pinyin("r"), s!("r5"));

        assert_eq!(decode_pinyin("alkfj"), None);
        assert_eq!(decode_pinyin("zh3"), None);
        assert_eq!(decode_pinyin("zhaang4"), None);
        assert_eq!(decode_pinyin("你好"), None);
        assert_eq!(decode_pinyin("a你好"), None);
        assert_eq!(decode_pinyin("你好zhang1"), None);
        assert_eq!(decode_pinyin(""), None);
    }

    #[test]
    fn encode_zhuyin_test() {
        assert_eq!(encode_zhuyin("e5"), s!("ㄝ"));
        assert_eq!(encode_zhuyin("ju0"), s!("ㄐㄩ˙"));
        assert_eq!(encode_zhuyin("jv"), s!("ㄐㄩ˙"));
        assert_eq!(encode_zhuyin("lv3"), s!("ㄌㄩˇ"));
        assert_eq!(encode_zhuyin("lvan4"), s!("ㄌㄩㄢˋ")); // not valid, for test only
        assert_eq!(encode_zhuyin("zhuan4"), s!("ㄓㄨㄢˋ"));
        assert_eq!(encode_zhuyin("zhao2"), s!("ㄓㄠˊ"));
        assert_eq!(encode_zhuyin("leng1"), s!("ㄌㄥ"));
        assert_eq!(encode_zhuyin("shui3"), s!("ㄕㄨㄟˇ"));
        assert_eq!(encode_zhuyin("liu2"), s!("ㄌㄧㄡˊ"));
        assert_eq!(encode_zhuyin("an3"), s!("ㄢˇ"));
        assert_eq!(encode_zhuyin("yi2"), s!("ㄧˊ"));
        assert_eq!(encode_zhuyin("yuan2"), s!("ㄩㄢˊ"));
        assert_eq!(encode_zhuyin("yvan2"), s!("ㄩㄢˊ"));
        assert_eq!(encode_zhuyin("min2"), s!("ㄇㄧㄣˊ"));
        assert_eq!(encode_zhuyin("er2"), s!("ㄦˊ"));
        assert_eq!(encode_zhuyin("r5"), s!("ㄦ˙"));
        // Zhuyin-specific
        assert_eq!(encode_zhuyin("yu1"), s!("ㄩ"));
        assert_eq!(encode_zhuyin("wu2"), s!("ㄨˊ"));
        assert_eq!(encode_zhuyin("yve3"), s!("ㄩㄝˇ"));
        assert_eq!(encode_zhuyin("yue4"), s!("ㄩㄝˋ"));
        assert_eq!(encode_zhuyin("zhi4"), s!("ㄓˋ"));

        assert_eq!(encode_pinyin("alkfj"), None);
        assert_eq!(encode_pinyin("zh3"), None);
        assert_eq!(encode_pinyin("zhaang4"), None);
        assert_eq!(encode_pinyin("你好"), None);
        assert_eq!(encode_pinyin("a你好"), None);
        assert_eq!(encode_pinyin("你好zhang1"), None);
        assert_eq!(encode_pinyin(""), None);
    }

    #[test]
    fn decode_zhuyin_test() {
        assert_eq!(decode_zhuyin("ㄝ"), s!("e5"));
        assert_eq!(decode_zhuyin("ㄧ"), s!("yi1"));
        assert_eq!(decode_zhuyin("ㄐㄩ˙"), s!("jv5"));
        assert_eq!(decode_zhuyin("ㄌㄩˇ"), s!("lv3"));
        assert_eq!(decode_zhuyin("ㄌㄩㄢˋ"), s!("lvan4")); // not valid, for test only
        assert_eq!(decode_zhuyin("ㄓㄨㄢˋ"), s!("zhuan4"));
        assert_eq!(decode_zhuyin("ㄓㄠˊ"), s!("zhao2"));
        assert_eq!(decode_zhuyin("ㄓˋ"), s!("zhi4"));
        assert_eq!(decode_zhuyin("ㄌㄥ"), s!("leng1"));
        assert_eq!(decode_zhuyin("ㄕㄨㄟˇ"), s!("shui3"));
        assert_eq!(decode_zhuyin("ㄌㄧㄡˊ"), s!("liu2"));
        assert_eq!(decode_zhuyin("ㄢˇ"), s!("an3"));
        assert_eq!(decode_zhuyin("ㄩ"), s!("yv1"));
        assert_eq!(decode_zhuyin("ㄨˊ"), s!("wu2"));
        assert_eq!(decode_zhuyin("ㄩㄝˇ"), s!("yve3"));
        assert_eq!(decode_zhuyin("ㄩㄝˋ"), s!("yve4"));
        assert_eq!(decode_zhuyin("ㄧˊ"), s!("yi2"));
        assert_eq!(decode_zhuyin("ㄩㄢˊ"), s!("yvan2"));
        assert_eq!(decode_zhuyin("ㄇㄧㄣˊ"), s!("min2"));
        assert_eq!(decode_zhuyin("ㄦˊ"), s!("er2"));
        assert_eq!(decode_zhuyin("ㄦ˙"), s!("r5"));
        // Zhuyin-specific
        assert_eq!(decode_zhuyin("ㄨㄥˊ"), s!("weng2"));

        assert_eq!(decode_zhuyin("ㄩㄝㄝ"), None);
        assert_eq!(decode_zhuyin("ㄐˇ"), None);
        assert_eq!(decode_zhuyin("ㄨㄕ"), None);
        assert_eq!(decode_zhuyin("ㄕㄨㄕㄨ"), None);
        assert_eq!(decode_pinyin("你好"), None);
        assert_eq!(decode_pinyin("ㄚ你好"), None);
        assert_eq!(decode_pinyin("你好zhang1"), None);
        assert_eq!(decode_pinyin(""), None);
    }

    #[test]
    fn pinyin_to_zhuyin_test() {
        assert_eq!(pinyin_to_zhuyin("mín"), s!("ㄇㄧㄣˊ"));
        assert_eq!(pinyin_to_zhuyin("zhāng"), s!("ㄓㄤ"));
        assert_eq!(pinyin_to_zhuyin("wéng"), s!("ㄨㄥˊ"));
        assert_eq!(pinyin_to_zhuyin("ér"), s!("ㄦˊ"));
        assert_eq!(pinyin_to_zhuyin("r"), s!("ㄦ˙"));

        assert_eq!(pinyin_to_zhuyin("wengg"), None);
        assert_eq!(pinyin_to_zhuyin("你好"), None);
        assert_eq!(pinyin_to_zhuyin(""), None);
    }

    #[test]
    fn zhuyin_to_pinyin_test() {
        assert_eq!(zhuyin_to_pinyin("ㄇㄧㄣˊ"), s!("mín"));
        assert_eq!(zhuyin_to_pinyin("ㄓㄤ"), s!("zhāng"));
        assert_eq!(zhuyin_to_pinyin("ㄨㄥˊ"), s!("wéng"));
        assert_eq!(zhuyin_to_pinyin("ㄦˊ"), s!("ér"));
        assert_eq!(zhuyin_to_pinyin("ㄦ˙"), s!("r"));

        assert_eq!(pinyin_to_zhuyin("ㄥㄥ"), None);
        assert_eq!(pinyin_to_zhuyin("你好"), None);
        assert_eq!(pinyin_to_zhuyin(""), None);
    }
}

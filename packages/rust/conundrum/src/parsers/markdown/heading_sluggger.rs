use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

/// A wrapper around the Slugger struct provided by the github slugger package
/// because I need to make that shit clonable
#[derive(Clone, Debug, Default)]
pub struct Slugger {
    slugs: HashSet<String>,
}

impl Slugger {
    /// Generate a slug for the given string.
    pub fn slug(&mut self, s: &str) -> String {
        // if we've already seen this slug, add a number to the end
        let base = slug(s);
        let mut result = base.clone();
        let mut i = 1;
        while self.slugs.contains(&result) {
            result = format!("{}-{}", base, i);
            i += 1;
        }

        self.slugs.insert(result.clone());
        result
    }

    /// Clear the set of slugs we've seen so far.
    pub fn reset(&mut self) {
        self.slugs.clear();
    }
}

static REMOVE_PAT: &str = r"[\p{Other_Number}\p{Close_Punctuation}\p{Final_Punctuation}\p{Initial_Punctuation}\p{Open_Punctuation}\p{Other_Punctuation}\p{Dash_Punctuation}\p{Symbol}\p{Control}\p{Private_Use}\p{Format}\p{Unassigned}\p{Separator}]";
static REMOVE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(REMOVE_PAT).unwrap());

pub fn slug(input: &str) -> String {
    let s = input.to_lowercase();

    // apply function to regex matches
    let s = REMOVE_RE.replace_all(&s, |caps: &regex::Captures| {
                         let c = caps.get(0).unwrap().as_str();
                         if c == " " || c == "-" {
                             "-".to_string()
                         } else if c.chars().all(|a| a.is_alphabetic()) {
                             // note in "Other Symbols" this matches:
                             // ⓐⓑⓒⓓⓔⓕⓖⓗⓘⓙⓚⓛⓜⓝⓞⓟⓠⓡⓢⓣⓤⓥⓦⓧⓨⓩ
                             // ⓐⓑⓒⓓⓔⓕⓖⓗⓘⓙⓚⓛⓜⓝⓞⓟⓠⓡⓢⓣⓤⓥⓦⓧⓨⓩ
                             // 🄰🄱🄲🄳🄴🄵🄶🄷🄸🄹🄺🄻🄼🄽🄾🄿🅀🅁🅂🅃🅄🅅🅆🅇🅈🅉
                             // 🅐🅑🅒🅓🅔🅕🅖🅗🅘🅙🅚🅛🅜🅝🅞🅟🅠🅡🅢🅣🅤🅥🅦🅧🅨🅩
                             // 🅰🅱🅲🅳🅴🅵🅶🅷🅸🅹🅺🅻🅼🅽🅾🅿🆀🆁🆂🆃🆄🆅🆆🆇🆈🆉
                             c.to_string()
                         } else {
                             "".to_string()
                         }
                     });
    s.replace(|c: char| c.is_whitespace(), "-")
}

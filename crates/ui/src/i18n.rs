//! The interface's words in the user's language, through gettext.
//!
//! The messages are the Apple app's catalog keys, `%lld` and `%@` and all, so its translations carry over.
//! [`tf`] puts the values in, in order or where a translation numbers them (`%2$@`).

use std::fmt::Display;
use std::path::Path;

use gettextrs::{LocaleCategory, bind_textdomain_codeset, bindtextdomain, gettext, setlocale, textdomain};

/// Reads the translations of `domain` from `locale_dir`. Call once, before the first word is looked up.
#[allow(unsafe_code)]
pub fn init(domain: &str, locale_dir: &Path) {
    // SAFETY: called at launch, before the app starts any thread that could use the locale meanwhile.
    unsafe { setlocale(LocaleCategory::LcAll, "") };
    let _ = bindtextdomain(domain, locale_dir);
    let _ = bind_textdomain_codeset(domain, "UTF-8");
    let _ = textdomain(domain);
}

/// A message in the user's language.
pub fn t(message: &str) -> String {
    gettext(message)
}

/// A message in the user's language with its values put in.
pub fn tf(message: &str, values: &[&dyn Display]) -> String {
    fill(&gettext(message), values)
}

/// Replaces `%lld`, `%d` and `%@` with `values` in order, and `%1$lld`, `%2$@` with the value of that number.
pub fn fill(template: &str, values: &[&dyn Display]) -> String {
    let mut text = String::with_capacity(template.len() + 16);
    let mut next = 0;
    let mut rest = template;
    while let Some(start) = rest.find('%') {
        text.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        if let Some(after) = after.strip_prefix('%') {
            text.push('%');
            rest = after;
            continue;
        }
        let digits = after.chars().take_while(char::is_ascii_digit).count();
        let (position, spec) = match after[digits..].strip_prefix('$') {
            Some(spec) if digits > 0 => (after[..digits].parse::<usize>().ok().map(|position| position - 1), spec),
            _ => (None, after),
        };
        let Some(length) = ["lld", "ld", "d", "@"].iter().find(|kind| spec.starts_with(**kind)).map(|kind| kind.len())
        else {
            text.push('%');
            rest = after;
            continue;
        };
        let index = position.unwrap_or_else(|| {
            next += 1;
            next - 1
        });
        if let Some(value) = values.get(index) {
            text.push_str(&value.to_string());
        }
        rest = &spec[length..];
    }
    text.push_str(rest);
    text
}

#[cfg(test)]
mod tests {
    use super::fill;

    #[test]
    fn values_go_in() {
        assert_eq!(fill("Key %lld", &[&3]), "Key 3");
        assert_eq!(fill("Keys: %lld · Knobs: %lld", &[&6, &1]), "Keys: 6 · Knobs: 1");
        assert_eq!(fill("%2$@ — %1$@", &[&"a", &"b"]), "b — a");
        assert_eq!(fill("100%% %@", &[&"x"]), "100% x");
        assert_eq!(fill("50% off", &[]), "50% off");
    }
}

//! Translations from gettext `.po` files.
//!
//! The texts of the interface are looked up by their English wording: a [`Catalog`] maps an
//! English text to the text in the user's language, and gives the English text back when there
//! is no translation. The language is chosen from the usual environment variables
//! ([`preferred_languages`]) among the catalogs the program has ([`pick`]).

use std::collections::HashMap;

/// One message of a `.po` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: String,
    pub text: String,
    /// marked `#, fuzzy`: a translator's guess that has not been checked
    pub fuzzy: bool,
}

/// The translated messages of one language.
#[derive(Debug, Default, Clone)]
pub struct Catalog {
    messages: HashMap<String, String>,
}

impl Catalog {
    /// Reads a `.po` file. Fuzzy and empty translations are left out.
    pub fn parse(po: &str) -> Catalog {
        let mut messages = HashMap::new();
        for entry in read_po(po) {
            if !entry.fuzzy && !entry.id.is_empty() && !entry.text.is_empty() {
                messages.insert(entry.id, entry.text);
            }
        }
        Catalog { messages }
    }

    /// The translation of `text`, if there is one.
    pub fn get(&self, text: &str) -> Option<&str> {
        self.messages.get(text).map(String::as_str)
    }

    /// The translation of `text`, or `text` itself.
    pub fn translate<'a>(&'a self, text: &'a str) -> &'a str {
        self.messages.get(text).map(String::as_str).unwrap_or(text)
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

/// `"text with \"escapes\"\n"` -> the text. `None` if `s` is not a quoted string.
fn unquote(s: &str) -> Option<String> {
    let inner = s.trim().strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    Some(out)
}

fn push(entries: &mut Vec<Entry>, id: Option<String>, text: Option<String>, fuzzy: bool) {
    if let (Some(id), Some(text)) = (id, text) {
        entries.push(Entry { id, text, fuzzy });
    }
}

/// All messages of a `.po` (or `.pot`) file, including the header (whose `id` is empty).
/// Plural forms and message contexts are not used by Charto and are ignored.
pub fn read_po(po: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    let mut id: Option<String> = None;
    let mut text: Option<String> = None;
    let mut in_text = false; // which of the two a continuation line belongs to
    let mut fuzzy = false; // the message being read is fuzzy
    let mut next_fuzzy = false; // a "#, fuzzy" line was seen for the message that comes next

    for line in po.lines() {
        let line = line.trim();
        if line.starts_with("#,") {
            if line.contains("fuzzy") {
                next_fuzzy = true;
            }
            continue;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("msgid ") {
            push(&mut entries, id.take(), text.take(), fuzzy);
            fuzzy = next_fuzzy;
            next_fuzzy = false;
            id = unquote(rest);
            in_text = false;
        } else if let Some(rest) = line.strip_prefix("msgstr ") {
            text = unquote(rest);
            in_text = true;
        } else if line.starts_with('"') {
            if let Some(more) = unquote(line) {
                let target = if in_text { &mut text } else { &mut id };
                if let Some(t) = target {
                    t.push_str(&more);
                }
            }
        }
    }
    push(&mut entries, id.take(), text.take(), fuzzy);
    entries
}

/// Adds the language codes in `value` (a locale such as `de_DE.UTF-8`, or a colon separated list
/// of them), most specific first: `de_DE`, then `de`.
fn add_codes(out: &mut Vec<String>, value: &str) {
    for part in value.split(':') {
        let code = part.split(['.', '@']).next().unwrap_or("").trim().replace('-', "_");
        if code.is_empty() || code == "C" || code == "POSIX" {
            continue;
        }
        if !out.contains(&code) {
            out.push(code.clone());
        }
        if let Some((base, region)) = code.split_once('_') {
            // Traditional Chinese must not fall back to the Simplified catalog
            let traditional = base == "zh" && matches!(region, "TW" | "HK" | "MO" | "Hant");
            if !traditional && !out.iter().any(|c| c == base) {
                out.push(base.to_string());
            }
        }
    }
}

/// The languages the user asks for, most wanted first, from the environment (`get` reads one
/// variable). `CHARTO_LANG` overrides everything, then come `LANGUAGE` and the locale
/// (`LC_ALL`, `LC_MESSAGES`, `LANG`). The locales `C` and `POSIX` mean "no translation".
pub fn preferred_languages<F: Fn(&str) -> Option<String>>(get: F) -> Vec<String> {
    let non_empty = |name: &str| get(name).filter(|v| !v.trim().is_empty());
    let mut out = Vec::new();

    if let Some(value) = non_empty("CHARTO_LANG") {
        add_codes(&mut out, &value);
        return out;
    }
    let locale = non_empty("LC_ALL").or_else(|| non_empty("LC_MESSAGES")).or_else(|| non_empty("LANG"));
    if let Some(l) = &locale {
        let code = l.split(['.', '@']).next().unwrap_or("");
        if code == "C" || code == "POSIX" {
            return out;
        }
    }
    if let Some(value) = non_empty("LANGUAGE") {
        add_codes(&mut out, &value);
    }
    if let Some(l) = &locale {
        add_codes(&mut out, l);
    }
    out
}

/// Simplified Chinese goes by several names.
fn canonical(code: &str) -> &str {
    match code {
        "zh" | "zh_Hans" | "zh_SG" => "zh_CN",
        other => other,
    }
}

/// The first of the wanted `languages` that `available` (pairs of language code and `.po` text)
/// has a catalog for.
pub fn pick<'a>(languages: &[String], available: &[(&str, &'a str)]) -> Option<&'a str> {
    for language in languages {
        let wanted = canonical(language);
        if let Some(entry) = available.iter().find(|entry| entry.0 == wanted) {
            return Some(entry.1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> =
            pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        move |name: &str| map.get(name).cloned()
    }

    fn langs(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn reads_po_files() {
        let po = "# comment\n\
                  msgid \"\"\n\
                  msgstr \"\"\n\
                  \"Language: de\\n\"\n\
                  \n\
                  msgid \"Open\"\n\
                  msgstr \"Öffnen\"\n\
                  \n\
                  #, fuzzy\n\
                  msgid \"Save\"\n\
                  msgstr \"Sichern\"\n\
                  \n\
                  msgid \"Quit\"\n\
                  msgstr \"\"\n\
                  \n\
                  msgid \"Two \"\n\
                  \"lines\"\n\
                  msgstr \"Zwei \"\n\
                  \"Zeilen\\n\"\n\
                  \n\
                  msgid \"Say \\\"hi\\\"\"\n\
                  msgstr \"Sag \\\"hallo\\\"\"\n";
        let catalog = Catalog::parse(po);
        assert_eq!(catalog.translate("Open"), "Öffnen");
        assert_eq!(catalog.translate("Save"), "Save"); // fuzzy: not used
        assert_eq!(catalog.translate("Quit"), "Quit"); // empty: not translated
        assert_eq!(catalog.translate("Two lines"), "Zwei Zeilen\n");
        assert_eq!(catalog.translate("Say \"hi\""), "Sag \"hallo\"");
        assert_eq!(catalog.translate("Unknown"), "Unknown");
        assert_eq!(catalog.len(), 3);
        assert!(!catalog.is_empty());
        assert!(Catalog::default().is_empty());
    }

    #[test]
    fn languages_from_the_environment() {
        assert_eq!(preferred_languages(env(&[("LANG", "de_DE.UTF-8")])), ["de_DE", "de"]);
        assert_eq!(
            preferred_languages(env(&[("LANG", "fr_FR.UTF-8"), ("LANGUAGE", "es:de")])),
            ["es", "de", "fr_FR", "fr"]
        );
        assert_eq!(preferred_languages(env(&[("LC_ALL", "ru_RU.UTF-8"), ("LANG", "en_US.UTF-8")])), ["ru_RU", "ru"]);
        assert_eq!(preferred_languages(env(&[("LANG", "pt_BR")])), ["pt_BR", "pt"]);
        assert_eq!(preferred_languages(env(&[("LANG", "zh_TW.UTF-8")])), ["zh_TW"]);
        assert_eq!(preferred_languages(env(&[("CHARTO_LANG", "ja"), ("LANG", "de_DE.UTF-8")])), ["ja"]);
        assert!(preferred_languages(env(&[("LANG", "C")])).is_empty());
        assert!(preferred_languages(env(&[("LC_ALL", "C"), ("LANGUAGE", "de"), ("LANG", "de_DE.UTF-8")])).is_empty());
        assert!(preferred_languages(env(&[])).is_empty());
    }

    #[test]
    fn picks_a_catalog() {
        let available = [("de", "German"), ("zh_CN", "Chinese"), ("pt", "Portuguese")];
        assert_eq!(pick(&langs(&["de_DE", "de"]), &available), Some("German"));
        assert_eq!(pick(&langs(&["zh"]), &available), Some("Chinese"));
        assert_eq!(pick(&langs(&["zh_Hans"]), &available), Some("Chinese"));
        assert_eq!(pick(&langs(&["pt_BR", "pt"]), &available), Some("Portuguese"));
        assert_eq!(pick(&langs(&["fr", "de"]), &available), Some("German"));
        assert_eq!(pick(&langs(&["en_US", "en"]), &available), None);
        assert_eq!(pick(&[], &available), None);
    }

    const TEMPLATE: &str = include_str!("../../charto-app/po/charto.pot");
    const CATALOGS: [(&str, &str); 9] = [
        ("de", include_str!("../../charto-app/po/de.po")),
        ("es", include_str!("../../charto-app/po/es.po")),
        ("fr", include_str!("../../charto-app/po/fr.po")),
        ("hi", include_str!("../../charto-app/po/hi.po")),
        ("it", include_str!("../../charto-app/po/it.po")),
        ("ja", include_str!("../../charto-app/po/ja.po")),
        ("pt", include_str!("../../charto-app/po/pt.po")),
        ("ru", include_str!("../../charto-app/po/ru.po")),
        ("zh_CN", include_str!("../../charto-app/po/zh_CN.po")),
    ];

    /// Every text of the interface is translated in every language, with the same keyboard
    /// mnemonic (&) and the same %1 %2 %3 placeholders as the English text.
    #[test]
    fn the_shipped_catalogs_are_complete() {
        let template: Vec<String> = read_po(TEMPLATE).into_iter().map(|e| e.id).filter(|id| !id.is_empty()).collect();
        assert!(template.len() > 100);
        for (code, po) in CATALOGS {
            let entries = read_po(po);
            for id in &template {
                let entry = entries
                    .iter()
                    .find(|e| &e.id == id)
                    .unwrap_or_else(|| panic!("{code}: \"{id}\" is missing"));
                assert!(!entry.fuzzy && !entry.text.is_empty(), "{code}: \"{id}\" is not translated");
                assert_eq!(id.matches('&').count(), entry.text.matches('&').count(), "{code}: mnemonic of \"{id}\"");
                for placeholder in ["%1", "%2", "%3"] {
                    assert_eq!(
                        id.matches(placeholder).count(),
                        entry.text.matches(placeholder).count(),
                        "{code}: {placeholder} in \"{id}\""
                    );
                }
                assert_eq!(id.ends_with(' '), entry.text.ends_with(' '), "{code}: trailing space of \"{id}\"");
            }
            // nothing left over from an older version of the interface
            let known = entries.iter().filter(|e| !e.id.is_empty()).count();
            assert_eq!(known, template.len(), "{code}: has texts the template does not have");
        }
    }
}

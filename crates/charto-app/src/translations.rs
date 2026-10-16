//! The texts of the interface in the user's language.
//!
//! The `.po` files in `po/` are built into the program, so there is nothing to install. The
//! language comes from the environment (`LANGUAGE`, `LC_ALL`, `LC_MESSAGES`, `LANG`); setting
//! `CHARTO_LANG` (for example `CHARTO_LANG=de charto`) overrides it, and `CHARTO_LANG=en` shows
//! the English texts.

use std::sync::OnceLock;

use charto_core::i18n::{pick, preferred_languages, Catalog};

const LANGUAGES: [(&str, &str); 9] = [
    ("de", include_str!("../po/de.po")),
    ("es", include_str!("../po/es.po")),
    ("fr", include_str!("../po/fr.po")),
    ("hi", include_str!("../po/hi.po")),
    ("it", include_str!("../po/it.po")),
    ("ja", include_str!("../po/ja.po")),
    ("pt", include_str!("../po/pt.po")),
    ("ru", include_str!("../po/ru.po")),
    ("zh_CN", include_str!("../po/zh_CN.po")),
];

fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let languages = preferred_languages(|name| std::env::var(name).ok());
        match pick(&languages, &LANGUAGES) {
            Some(po) => Catalog::parse(po),
            None => Catalog::default(),
        }
    })
}

/// The translation of `text` into the user's language, or `text` itself when there is none.
pub fn translate(text: &str) -> String {
    let catalog = catalog();
    if let Some(translated) = catalog.get(text) {
        return translated.to_string();
    }
    // texts read from files (the symbol library) may carry a stray line break or space
    let trimmed = text.trim();
    if trimmed != text {
        if let Some(translated) = catalog.get(trimmed) {
            return translated.to_string();
        }
    }
    text.to_string()
}

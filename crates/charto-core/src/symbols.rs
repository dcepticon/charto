//! Knitting symbol library, ported from `sconcho/util/symbol_parser.py`.
//!
//! A symbol is a directory containing a `description` XML file and an SVG:
//!
//! ```xml
//! <sconcho>
//!   <knittingSymbol>
//!     <svgName>2over2left</svgName>
//!     <category>4 stitch cables:10</category>
//!     <symbolName>2 over 2 left</symbolName>
//!     <symbolDescription>…</symbolDescription>
//!     <symbolWidth>4</symbolWidth>
//!   </knittingSymbol>
//! </sconcho>
//! ```
//!
//! The part of `category` after the colon is the position of the symbol
//! *within* its category tab in the palette; symbols without one sort last.
//! Category tabs themselves are ordered alphabetically by name.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Sort position used for symbols without an explicit position in their category.
pub const LARGE_INT: i32 = 100_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolDef {
    pub svg_name: String,
    pub category: String,
    pub category_pos: i32,
    pub name: String,
    pub description: String,
    /// Number of grid cells the symbol spans horizontally.
    pub width: u32,
    pub background_color: Option<String>,
    pub svg_path: PathBuf,
}

/// Parses the text of one `description` file. Returns `None` for anything
/// that is not a valid sconcho symbol description.
pub fn parse_description(xml: &str, dir: &Path) -> Option<SymbolDef> {
    let doc = roxmltree::Document::parse(xml).ok()?;
    let root = doc.root_element();
    if root.tag_name().name() != "sconcho" {
        return None;
    }
    let symbol = root.children().find(|n| n.is_element())?;
    if symbol.tag_name().name() != "knittingSymbol" {
        return None;
    }

    let mut svg_name = None;
    let mut category = None;
    let mut category_pos = LARGE_INT;
    let mut name = None;
    let mut description = None;
    let mut width = None;
    let mut background_color = None;

    for item in symbol.children().filter(|n| n.is_element()) {
        let text = item.text().unwrap_or("").to_string();
        match item.tag_name().name() {
            "svgName" => svg_name = Some(text),
            "category" => {
                let parts: Vec<&str> = text.split(':').collect();
                match parts.as_slice() {
                    [cat] => category = Some(cat.to_string()),
                    [cat, pos] => {
                        category = Some(cat.to_string());
                        category_pos = pos.trim().parse().unwrap_or(LARGE_INT);
                    }
                    _ => return None,
                }
            }
            "symbolName" => name = Some(text),
            "symbolDescription" => description = Some(text),
            "symbolWidth" => width = text.trim().parse::<u32>().ok(),
            "backgroundColor" => background_color = Some(text),
            _ => {}
        }
    }

    let svg_name = svg_name?;
    let svg_path = dir.join(format!("{svg_name}.svg"));
    Some(SymbolDef {
        svg_name,
        category: category?,
        category_pos,
        name: name?,
        description: description?,
        width: width?,
        background_color,
        svg_path,
    })
}

/// All known symbols, keyed by symbol name (as in the original).
#[derive(Debug, Default, Clone)]
pub struct SymbolLibrary {
    symbols: BTreeMap<String, SymbolDef>,
}

impl SymbolLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads every symbol directory below each of `roots`. Later roots
    /// override earlier ones when names collide, so pass the built-in
    /// directory first and the user's personal symbols after it.
    /// Unreadable symbols are skipped; their paths are returned.
    pub fn load(roots: &[PathBuf]) -> (Self, Vec<PathBuf>) {
        let mut lib = SymbolLibrary::new();
        let mut skipped = Vec::new();

        for root in roots {
            let Ok(entries) = std::fs::read_dir(root) else { continue };
            let mut dirs: Vec<PathBuf> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .collect();
            dirs.sort();

            for dir in dirs {
                let parsed = std::fs::read_to_string(dir.join("description"))
                    .ok()
                    .and_then(|xml| parse_description(&xml, &dir));
                match parsed {
                    Some(sym) => {
                        lib.symbols.insert(sym.name.clone(), sym);
                    }
                    None => skipped.push(dir),
                }
            }
        }
        (lib, skipped)
    }

    pub fn get(&self, name: &str) -> Option<&SymbolDef> {
        self.symbols.get(name)
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &SymbolDef> {
        self.symbols.values()
    }

    /// Palette layout: category tabs ordered alphabetically by name; inside a
    /// tab, symbols ordered by their `category_pos` (ties broken by name,
    /// where the original used arbitrary dictionary order).
    pub fn categories(&self) -> Vec<(String, Vec<&SymbolDef>)> {
        let mut map: BTreeMap<String, Vec<&SymbolDef>> = BTreeMap::new();
        for s in self.symbols.values() {
            map.entry(s.category.clone()).or_default().push(s);
        }
        map.into_iter()
            .map(|(cat, mut syms)| {
                syms.sort_by(|a, b| a.category_pos.cmp(&b.category_pos).then_with(|| a.name.cmp(&b.name)));
                (cat, syms)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KNIT: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<sconcho>
  <knittingSymbol>
    <svgName>knit</svgName>
    <category>basic:1</category>
    <symbolName>knit</symbolName>
    <symbolDescription>knit on RS; purl on WS</symbolDescription>
    <symbolWidth>1</symbolWidth>
  </knittingSymbol>
</sconcho>"#;

    #[test]
    fn parses_basic_symbol() {
        let s = parse_description(KNIT, Path::new("/x/knit")).unwrap();
        assert_eq!(s.name, "knit");
        assert_eq!(s.category, "basic");
        assert_eq!(s.category_pos, 1);
        assert_eq!(s.width, 1);
        assert_eq!(s.svg_path, PathBuf::from("/x/knit/knit.svg"));
    }

    #[test]
    fn symbol_without_position_sorts_last_in_category() {
        let xml = KNIT.replace("basic:1", "custom");
        let s = parse_description(&xml, Path::new("/x")).unwrap();
        assert_eq!(s.category_pos, LARGE_INT);
    }

    #[test]
    fn rejects_wrong_root_and_bad_category() {
        assert!(parse_description("<other/>", Path::new("/x")).is_none());
        let xml = KNIT.replace("basic:1", "a:b:c");
        assert!(parse_description(&xml, Path::new("/x")).is_none());
        assert!(parse_description("not xml", Path::new("/x")).is_none());
    }
}

//! Vector export of a chart as one SVG file.
//!
//! Every symbol used is embedded once, as its own SVG document inside an
//! `<image>` (data URI) wrapped in a `<symbol>`, and placed with `<use>`.
//! That keeps the original symbol drawings intact and sidesteps the id and
//! namespace clashes you get when pasting many Inkscape files into one.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::chart::Chart;
use crate::labels;
use crate::session::{color_hex, parse_color};
use crate::spf::ProjectSettings;
use crate::symbols::SymbolLibrary;

/// Space around the grid for row labels, repeat brackets and column labels
/// (the same values the GUI uses).
pub const LEFT_MARGIN: f64 = 80.0;
pub const RIGHT_MARGIN: f64 = 80.0;
pub const BOTTOM_MARGIN: f64 = 34.0;

pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

pub fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { TABLE[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[n as usize & 63] as char } else { '=' });
    }
    out
}

fn parse_length(s: &str) -> Option<f64> {
    s.trim().trim_end_matches(|c: char| c.is_ascii_alphabetic() || c == '%').trim().parse().ok()
}

/// Intrinsic size of a symbol drawing: the root `width`/`height`, or 90 px
/// per cell (the size the bundled symbols are drawn at).
pub fn symbol_size(svg_text: &str, width_cells: u32) -> (f64, f64) {
    let fallback = (90.0 * width_cells.max(1) as f64, 90.0);
    let Ok(doc) = roxmltree::Document::parse(svg_text) else { return fallback };
    let root = doc.root_element();
    match (root.attribute("width").and_then(parse_length), root.attribute("height").and_then(parse_length)) {
        (Some(w), Some(h)) if w > 0.0 && h > 0.0 => (w, h),
        _ => fallback,
    }
}

fn num(v: f64) -> String {
    if (v - v.round()).abs() < 1e-9 {
        format!("{}", v.round() as i64)
    } else {
        let s = format!("{v:.2}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Renders the chart. Hidden cells are left out entirely.
pub fn render(chart: &Chart, s: &ProjectSettings, library: &SymbolLibrary) -> Result<String, String> {
    let cw = s.grid_cell_width.max(8) as f64;
    let ch = s.grid_cell_height.max(8) as f64;
    let cols = chart.num_columns() as f64;
    let rows = chart.num_rows() as f64;
    let width = LEFT_MARGIN + cols * cw + RIGHT_MARGIN;
    let height = rows * ch + BOTTOM_MARGIN;

    // one <symbol> per distinct symbol used by a visible cell
    let mut ids: BTreeMap<String, usize> = BTreeMap::new();
    let mut defs = String::new();
    for c in chart.cells().filter(|c| !c.hidden) {
        let name = &c.symbol.name;
        if ids.contains_key(name) {
            continue;
        }
        let Some(def) = library.get(name) else { continue };
        let Ok(bytes) = std::fs::read(&def.svg_path) else { continue };
        let (sw, sh) = symbol_size(&String::from_utf8_lossy(&bytes), def.width);
        let idx = ids.len();
        ids.insert(name.clone(), idx);
        let _ = writeln!(
            defs,
            "<symbol id=\"s{idx}\" viewBox=\"0 0 {w} {h}\" preserveAspectRatio=\"xMidYMid meet\">\
             <image width=\"{w}\" height=\"{h}\" xlink:href=\"data:image/svg+xml;base64,{data}\"/></symbol>",
            w = num(sw),
            h = num(sh),
            data = base64(&bytes)
        );
    }

    let mut out = String::new();
    let _ = writeln!(
        out,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" \
         width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">",
        w = num(width),
        h = num(height)
    );
    let _ = writeln!(out, "<defs>\n{defs}</defs>");
    let _ = writeln!(out, "<rect width=\"{}\" height=\"{}\" fill=\"#ffffff\"/>", num(width), num(height));

    // cells
    for c in chart.cells().filter(|c| !c.hidden) {
        let x = LEFT_MARGIN + c.column as f64 * cw;
        let y = c.row as f64 * ch;
        let w = c.width as f64 * cw;
        let _ = writeln!(
            out,
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"#8a8a8a\" stroke-width=\"1\"/>",
            num(x),
            num(y),
            num(w),
            num(ch),
            color_hex(c.color)
        );
        if let Some(idx) = ids.get(&c.symbol.name) {
            let _ = writeln!(
                out,
                "<use xlink:href=\"#s{idx}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
                num(x),
                num(y),
                num(w),
                num(ch)
            );
        }
    }

    // row highlighting
    if s.highlight_rows {
        let tint = parse_color(&s.highlight_rows_color).map(color_hex).unwrap_or_else(|| "#808080".into());
        for r in labels::highlighted_rows(chart, s) {
            let _ = writeln!(
                out,
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" fill-opacity=\"{}\"/>",
                num(LEFT_MARGIN),
                num(r as f64 * ch),
                num(cols * cw),
                num(ch),
                tint,
                num(s.highlight_rows_opacity as f64 / 100.0)
            );
        }
    }

    // labels
    let font = escape_xml(&s.label_font.family);
    let size = (ch * 0.4).max(9.0);
    for l in labels::row_labels(chart, s) {
        let (x, anchor) = if l.left {
            (LEFT_MARGIN - 6.0, "end")
        } else {
            (LEFT_MARGIN + cols * cw + 6.0, "start")
        };
        let _ = writeln!(
            out,
            "<text x=\"{}\" y=\"{}\" font-family=\"{font}\" font-size=\"{}\" text-anchor=\"{anchor}\" \
             dominant-baseline=\"central\" fill=\"#333333\">{}</text>",
            num(x),
            num(l.row as f64 * ch + ch / 2.0),
            num(size),
            escape_xml(&l.text)
        );
    }
    for l in labels::column_labels(chart, s) {
        let _ = writeln!(
            out,
            "<text x=\"{}\" y=\"{}\" font-family=\"{font}\" font-size=\"{}\" text-anchor=\"middle\" \
             dominant-baseline=\"central\" fill=\"#333333\">{}</text>",
            num(LEFT_MARGIN + l.col as f64 * cw + cw / 2.0),
            num(rows * ch + 15.0),
            num(size),
            escape_xml(&l.text)
        );
    }

    // row repeats: bracket on the left with the multiplier
    for rep in chart.row_repeats() {
        let (Some(&first), Some(&last)) = (rep.rows.iter().min(), rep.rows.iter().max()) else { continue };
        let top = first as f64 * ch;
        let bottom = (last + 1) as f64 * ch;
        let _ = writeln!(
            out,
            "<line x1=\"31.5\" y1=\"{}\" x2=\"31.5\" y2=\"{}\" stroke=\"#cc3300\" stroke-width=\"3\"/>",
            num(top),
            num(bottom)
        );
        let _ = writeln!(
            out,
            "<text x=\"26\" y=\"{}\" font-family=\"{font}\" font-size=\"{}\" font-weight=\"bold\" \
             text-anchor=\"end\" dominant-baseline=\"central\" fill=\"#cc3300\">×{}</text>",
            num((top + bottom) / 2.0),
            num(size),
            rep.multiplicity
        );
    }

    out.push_str("</svg>\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qdatastream::Color;
    use crate::spf::SymbolRef;
    use std::path::PathBuf;

    fn library() -> SymbolLibrary {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/symbols");
        SymbolLibrary::load(&[root]).0
    }

    fn chart(rows: i32, cols: i32) -> Chart {
        Chart::new(rows, cols, SymbolRef { category: "basic".into(), name: "knit".into() }, Color::WHITE)
    }

    fn count(doc: &roxmltree::Document, tag: &str) -> usize {
        doc.descendants().filter(|n| n.is_element() && n.tag_name().name() == tag).count()
    }

    #[test]
    fn base64_known_vectors() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foob"), "Zm9vYg==");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn xml_escaping() {
        assert_eq!(escape_xml("a<b>&\"c\""), "a&lt;b&gt;&amp;&quot;c&quot;");
    }

    #[test]
    fn symbol_sizes() {
        assert_eq!(symbol_size("<svg width=\"360\" height=\"90px\"/>", 4), (360.0, 90.0));
        assert_eq!(symbol_size("<svg/>", 2), (180.0, 90.0));
        assert_eq!(symbol_size("not xml", 1), (90.0, 90.0));
    }

    #[test]
    fn bundled_symbols_report_their_drawn_size() {
        let lib = library();
        let knit = lib.get("knit").unwrap();
        let text = std::fs::read_to_string(&knit.svg_path).unwrap();
        assert_eq!(symbol_size(&text, knit.width), (90.0, 90.0));
        let wide = lib.iter().find(|d| d.width == 4).expect("a 4-wide symbol");
        let text = std::fs::read_to_string(&wide.svg_path).unwrap();
        assert_eq!(symbol_size(&text, wide.width), (360.0, 90.0));
    }

    #[test]
    fn output_is_well_formed_and_complete() {
        let lib = library();
        let mut c = chart(3, 4);
        c.set_hidden(&[(1, 1)], true);
        let svg = render(&c, &ProjectSettings::default(), &lib).unwrap();
        let doc = roxmltree::Document::parse(&svg).expect("well-formed XML");

        assert_eq!(doc.root_element().tag_name().name(), "svg");
        assert_eq!(count(&doc, "symbol"), 1); // only "knit" is used
        assert_eq!(count(&doc, "use"), 11); // 12 cells, one hidden
        assert_eq!(count(&doc, "text"), 3 + 4); // row and column labels
        assert!(svg.contains("data:image/svg+xml;base64,"));
        // total size: 80 + 4*30 + 80 by 3*30 + 34
        assert_eq!(doc.root_element().attribute("width"), Some("280"));
        assert_eq!(doc.root_element().attribute("height"), Some("124"));
    }

    #[test]
    fn row_repeats_highlighting_and_escaping_are_drawn() {
        let lib = library();
        let mut c = chart(4, 2);
        c.row_labels.insert(0, "a<&>b".into());
        c.select_cells(&[(1, 0)], None);
        c.add_row_repeat(3).unwrap();
        let mut s = ProjectSettings::default();
        s.row_labels_editable = true;
        s.highlight_rows_color = "#ff0000".into();

        let svg = render(&c, &s, &lib).unwrap();
        let doc = roxmltree::Document::parse(&svg).expect("escaped text keeps the XML valid");
        assert_eq!(count(&doc, "line"), 1);
        assert_eq!(count(&doc, "text"), 4 + 2 + 1); // rows, columns, "×3"
        assert!(svg.contains("a&lt;&amp;&gt;b"));
        assert!(svg.contains("fill=\"#ff0000\" fill-opacity=\"0.1\""));
    }

    #[test]
    fn wide_symbols_and_unknown_symbols() {
        let lib = library();
        let wide = lib.iter().find(|d| d.width == 2).expect("2-wide").name.clone();
        let mut c = chart(1, 4);
        let sym = crate::chart::ActiveSymbol {
            symbol: SymbolRef { category: "x".into(), name: wide.clone() },
            width: 2,
            color: Color::WHITE,
        };
        c.select_cells(&[(0, 0), (0, 1)], Some(&sym));
        c.select_cells(&[(0, 2)], Some(&crate::chart::ActiveSymbol {
            symbol: SymbolRef { category: "x".into(), name: "not-installed".into() },
            width: 1,
            color: Color::WHITE,
        }));
        let svg = render(&c, &ProjectSettings::default(), &lib).unwrap();
        let doc = roxmltree::Document::parse(&svg).unwrap();
        assert_eq!(count(&doc, "symbol"), 2); // the wide symbol and knit
        assert_eq!(count(&doc, "use"), 2); // wide + the remaining knit; the unknown one has no drawing
    }
}

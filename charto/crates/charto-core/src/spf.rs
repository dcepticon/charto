//! The sconcho project file format (`.spf`), API versions 1–3.
//!
//! Ported from `sconcho/util/io.py`. Files are Qt `QDataStream` blobs
//! (see [`crate::qdatastream`]): header `0xA3D1`, API version, then
//!
//! * API 1/2: a fixed list of counts followed by the items in fixed order;
//! * API 3: named sections (`patternGridItems`, `legendItems`, …).
//!
//! Reading supports all three versions; writing always produces API 3,
//! exactly like the original program.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use crate::geometry::{sort_vertices, Segment};
use crate::qdatastream::{Color, Font, PointF, Reader, Writer};

pub const MAGIC_NUMBER: i32 = 0xA3D1;
pub const API_VERSION: i32 = 3;

#[derive(Debug)]
pub enum SpfError {
    Io(std::io::Error),
    NotSpf,
    UnsupportedVersion(i32),
    Truncated(&'static str),
    UnknownSection(String),
    BadPatternRepeat,
}

impl fmt::Display for SpfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpfError::Io(e) => write!(f, "I/O error: {e}"),
            SpfError::NotSpf => write!(f, "not a valid .spf pattern file"),
            SpfError::UnsupportedVersion(v) => write!(f, "unsupported spf API version {v}"),
            SpfError::Truncated(what) => write!(f, "file truncated while reading {what}"),
            SpfError::UnknownSection(s) => write!(f, "unknown section \"{s}\" in spf file"),
            SpfError::BadPatternRepeat => write!(
                f,
                "a pattern repeat in this file is malformed (outline has a hole or gap)"
            ),
        }
    }
}

impl std::error::Error for SpfError {}

impl From<std::io::Error> for SpfError {
    fn from(e: std::io::Error) -> Self {
        SpfError::Io(e)
    }
}

// ---------------------------------------------------------------------------
// data model
// ---------------------------------------------------------------------------

/// Reference to a knitting symbol: category plus name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolRef {
    pub category: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridItem {
    pub category: String,
    pub name: String,
    pub column: i32,
    pub row: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub is_hidden: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LegendItem {
    pub category: String,
    pub name: String,
    pub item_pos: PointF,
    pub label_pos: PointF,
    pub color: Color,
    pub description: String,
}

/// One entry of the colour selector: colour plus its selection state.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorEntry {
    pub color: Color,
    pub state: i16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatternRepeat {
    pub polygon: Vec<PointF>,
    pub position: PointF,
    pub legend_id: u16,
    pub width: i16,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatLegend {
    pub legend_id: u16,
    pub is_visible: bool,
    pub item_pos: PointF,
    pub text_item_pos: PointF,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RowRepeat {
    pub multiplicity: i32,
    pub rows: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextItem {
    pub pos: PointF,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowLabelMode {
    AllRows,
    ShowRowsWithInterval,
    ShowOddRows,
    ShowEvenRows,
}

impl RowLabelMode {
    /// Integer id stored in the file (ids 103 and below 101 are legacy).
    pub fn code(self) -> i32 {
        match self {
            RowLabelMode::AllRows => 101,
            RowLabelMode::ShowRowsWithInterval => 102,
            RowLabelMode::ShowOddRows => 104,
            RowLabelMode::ShowEvenRows => 105,
        }
    }
    pub fn from_code(code: i32) -> Self {
        match code {
            102 => RowLabelMode::ShowRowsWithInterval,
            104 => RowLabelMode::ShowOddRows,
            105 => RowLabelMode::ShowEvenRows,
            _ => RowLabelMode::AllRows,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnLabelMode {
    AllColumns,
    ShowColumnsWithInterval,
}

impl ColumnLabelMode {
    pub fn code(self) -> i32 {
        match self {
            ColumnLabelMode::AllColumns => 101,
            ColumnLabelMode::ShowColumnsWithInterval => 102,
        }
    }
    pub fn from_code(code: i32) -> Self {
        if code == 102 {
            ColumnLabelMode::ShowColumnsWithInterval
        } else {
            ColumnLabelMode::AllColumns
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelLocation {
    LeftOf,
    RightOf,
}

impl LabelLocation {
    pub fn code(self) -> i32 {
        match self {
            LabelLocation::LeftOf => 11,
            LabelLocation::RightOf => 10,
        }
    }
    pub fn from_code(code: i32) -> Self {
        if code == 11 {
            LabelLocation::LeftOf
        } else {
            LabelLocation::RightOf
        }
    }
}

/// Per-project settings (fonts, label options, row highlighting).
/// Defaults match `DefaultSettings` in the original.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectSettings {
    pub label_font: Font,
    pub legend_font: Font,
    pub grid_cell_width: i32,
    pub grid_cell_height: i32,
    pub row_label_mode: RowLabelMode,
    pub row_label_start: i32,
    pub even_row_label_location: LabelLocation,
    pub odd_row_label_location: LabelLocation,
    pub row_labels_show_interval: i32,
    pub row_labels_show_interval_start: i32,
    pub column_label_mode: ColumnLabelMode,
    pub column_labels_show_interval: i32,
    pub column_labels_show_interval_start: i32,
    pub row_labels_editable: bool,
    pub column_labels_editable: bool,
    pub highlight_rows: bool,
    pub highlight_rows_opacity: i32,
    pub highlight_rows_start: i32,
    pub highlight_rows_color: String,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        ProjectSettings {
            label_font: Font::new("Arial", 10.0),
            legend_font: Font::new("Arial", 10.0),
            grid_cell_width: 30,
            grid_cell_height: 30,
            row_label_mode: RowLabelMode::AllRows,
            row_label_start: 1,
            even_row_label_location: LabelLocation::RightOf,
            odd_row_label_location: LabelLocation::RightOf,
            row_labels_show_interval: 1,
            row_labels_show_interval_start: 1,
            column_label_mode: ColumnLabelMode::AllColumns,
            column_labels_show_interval: 1,
            column_labels_show_interval_start: 1,
            row_labels_editable: false,
            column_labels_editable: false,
            highlight_rows: true,
            highlight_rows_opacity: 10,
            highlight_rows_start: 0,
            highlight_rows_color: "gray".to_string(),
        }
    }
}

/// Everything stored in an spf file.
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    /// API version the file was read from (always 3 for freshly created ones).
    pub api_version: i32,
    pub grid_items: Vec<GridItem>,
    pub legend_items: Vec<LegendItem>,
    pub colors: Vec<ColorEntry>,
    pub active_symbol: Option<SymbolRef>,
    pub pattern_repeats: Vec<PatternRepeat>,
    /// Keyed by the legend id that links a legend to its repeat.
    pub repeat_legends: BTreeMap<u16, RepeatLegend>,
    pub row_repeats: Vec<RowRepeat>,
    pub text_items: Vec<TextItem>,
    pub row_labels: BTreeMap<i32, String>,
    pub column_labels: BTreeMap<i32, String>,
    pub settings: ProjectSettings,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            api_version: API_VERSION,
            grid_items: Vec::new(),
            legend_items: Vec::new(),
            colors: Vec::new(),
            active_symbol: None,
            pattern_repeats: Vec::new(),
            repeat_legends: BTreeMap::new(),
            row_repeats: Vec::new(),
            text_items: Vec::new(),
            row_labels: BTreeMap::new(),
            column_labels: BTreeMap::new(),
            settings: ProjectSettings::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// public API
// ---------------------------------------------------------------------------

pub fn read_project(path: &Path) -> Result<Project, SpfError> {
    let bytes = std::fs::read(path)?;
    parse_project(&bytes)
}

pub fn write_project(path: &Path, project: &Project) -> Result<(), SpfError> {
    std::fs::write(path, serialize_project(project))?;
    Ok(())
}

pub fn parse_project(bytes: &[u8]) -> Result<Project, SpfError> {
    let mut r = Reader::new(bytes);
    let magic = r.i32();
    if r.overrun() || magic != MAGIC_NUMBER {
        return Err(SpfError::NotSpf);
    }
    let version = r.i32();
    if r.overrun() {
        return Err(SpfError::Truncated("header"));
    }
    match version {
        1 | 2 => read_legacy(&mut r, version),
        3 => read_api3(&mut r),
        v => Err(SpfError::UnsupportedVersion(v)),
    }
}

pub fn serialize_project(p: &Project) -> Vec<u8> {
    let mut w = Writer::new();
    w.i32(MAGIC_NUMBER);
    w.i32(API_VERSION);

    section(&mut w, "patternGridItems", p.grid_items.len());
    for it in &p.grid_items {
        w.qstring(&it.category);
        w.qstring(&it.name);
        w.i32(it.column);
        w.i32(it.row);
        w.i32(it.width);
        w.i32(it.height);
        w.color(it.color);
        w.bool(it.is_hidden);
    }

    section(&mut w, "legendItems", p.legend_items.len());
    for it in &p.legend_items {
        w.qstring(&it.category);
        w.qstring(&it.name);
        w.f64(it.item_pos.0);
        w.f64(it.item_pos.1);
        w.f64(it.label_pos.0);
        w.f64(it.label_pos.1);
        w.color(it.color);
        w.qstring(&it.description);
    }

    section(&mut w, "colors", p.colors.len());
    for c in &p.colors {
        w.color(c.color);
        w.i16(c.state);
    }

    section(&mut w, "activeSymbol", 1);
    match &p.active_symbol {
        Some(s) => {
            w.qstring(&s.category);
            w.qstring(&s.name);
        }
        None => {
            w.qstring("None");
            w.qstring("None");
        }
    }

    section(&mut w, "patternRepeats", p.pattern_repeats.len());
    for rep in &p.pattern_repeats {
        w.polygon(&rep.polygon);
        w.point(rep.position);
        w.u16(rep.legend_id);
        w.i16(rep.width);
        w.color(rep.color);
    }

    section(&mut w, "repeatLegends", p.repeat_legends.len());
    for l in p.repeat_legends.values() {
        w.u16(l.legend_id);
        w.u16(l.is_visible as u16);
        w.point(l.item_pos);
        w.point(l.text_item_pos);
        w.qstring(&l.text);
    }

    section(&mut w, "rowRepeats", p.row_repeats.len());
    for rr in &p.row_repeats {
        w.i32(rr.multiplicity);
        w.i32(rr.rows.len() as i32);
        for &row in &rr.rows {
            w.i32(row);
        }
    }

    section(&mut w, "textItems", p.text_items.len());
    for t in &p.text_items {
        w.point(t.pos);
        w.qstring(&t.text);
    }

    section(&mut w, "rowLabels", p.row_labels.len());
    for (k, v) in &p.row_labels {
        w.i32(*k);
        w.qstring(v);
    }

    section(&mut w, "columnLabels", p.column_labels.len());
    for (k, v) in &p.column_labels {
        w.i32(*k);
        w.qstring(v);
    }

    section(&mut w, "settings", 1);
    write_settings(&mut w, &p.settings);

    w.into_bytes()
}

// ---------------------------------------------------------------------------
// writing helpers
// ---------------------------------------------------------------------------

fn section(w: &mut Writer, name: &str, len: usize) {
    w.qstring(name);
    w.i32(len as i32);
}

fn write_settings(w: &mut Writer, s: &ProjectSettings) {
    w.font(&s.label_font);
    w.i32(s.row_label_mode.code());
    w.font(&s.legend_font);
    w.i32(s.grid_cell_width);
    w.i32(s.grid_cell_height);

    w.i32(s.row_label_start);
    w.i32(s.even_row_label_location.code());

    w.i32(s.highlight_rows as i32);
    w.i32(s.highlight_rows_opacity);
    w.i32(s.highlight_rows_start);
    w.qstring(&s.highlight_rows_color);

    w.i32(s.odd_row_label_location.code());
    w.i32(s.row_labels_show_interval);
    w.i32(s.row_labels_show_interval_start);

    w.i32(s.column_label_mode.code());
    w.i32(s.column_labels_show_interval);
    w.i32(s.column_labels_show_interval_start);

    w.i32(s.row_labels_editable as i32);
    w.i32(s.column_labels_editable as i32);
}

// ---------------------------------------------------------------------------
// reading helpers
// ---------------------------------------------------------------------------

fn check(r: &Reader, what: &'static str) -> Result<(), SpfError> {
    if r.overrun() {
        Err(SpfError::Truncated(what))
    } else {
        Ok(())
    }
}

fn read_grid_items(r: &mut Reader, n: i32, with_hidden: bool) -> Vec<GridItem> {
    let mut items = Vec::new();
    for _ in 0..n {
        if r.overrun() {
            break;
        }
        let category = r.qstring();
        let name = r.qstring();
        let column = r.i32();
        let row = r.i32();
        let width = r.i32();
        let height = r.i32();
        let color = r.color();
        let is_hidden = if with_hidden { r.bool() } else { false };
        items.push(GridItem { category, name, column, row, width, height, color, is_hidden });
    }
    items
}

fn read_legend_items(r: &mut Reader, n: i32) -> Vec<LegendItem> {
    let mut items = Vec::new();
    for _ in 0..n {
        if r.overrun() {
            break;
        }
        let category = r.qstring();
        let name = r.qstring();
        let ix = r.f64();
        let iy = r.f64();
        let lx = r.f64();
        let ly = r.f64();
        let color = r.color();
        let description = r.qstring();
        items.push(LegendItem {
            category,
            name,
            item_pos: (ix, iy),
            label_pos: (lx, ly),
            color,
            description,
        });
    }
    items
}

fn read_colors(r: &mut Reader, n: i32) -> Vec<ColorEntry> {
    let mut items = Vec::new();
    for _ in 0..n {
        if r.overrun() {
            break;
        }
        let color = r.color();
        let state = r.i16();
        items.push(ColorEntry { color, state });
    }
    items
}

fn read_active_symbol(r: &mut Reader) -> Option<SymbolRef> {
    let category = r.qstring();
    let name = r.qstring();
    if category != "None" {
        Some(SymbolRef { category, name })
    } else {
        None
    }
}

/// Repeat as stored by API 1/2: a bag of line segments.
struct LegacyRepeat {
    lines: Vec<Segment>,
    position: PointF,
    width: i16,
    color: Color,
    legend_id: u16,
}

fn read_legacy_repeats(r: &mut Reader, n: i32) -> Vec<LegacyRepeat> {
    let mut repeats = Vec::new();
    for _ in 0..n {
        if r.overrun() {
            break;
        }
        let num_points = r.i32();
        let mut lines = Vec::new();
        // the original steps through the points two at a time
        let mut count = 0;
        while count < num_points {
            if r.overrun() {
                break;
            }
            let p1 = r.point();
            let p2 = r.point();
            lines.push((p1, p2));
            count += 2;
        }
        let position = r.point();
        let legend_id = r.u16();
        let width = r.i16();
        let color = r.color();
        repeats.push(LegacyRepeat { lines, position, width, color, legend_id });
    }
    repeats
}

fn convert_repeats(legacy: Vec<LegacyRepeat>) -> Result<Vec<PatternRepeat>, SpfError> {
    let mut out = Vec::new();
    for rep in legacy {
        let polygon = sort_vertices(rep.lines).ok_or(SpfError::BadPatternRepeat)?;
        out.push(PatternRepeat {
            polygon,
            position: rep.position,
            legend_id: rep.legend_id,
            width: rep.width,
            color: rep.color,
        });
    }
    Ok(out)
}

fn read_pattern_repeats(r: &mut Reader, n: i32) -> Vec<PatternRepeat> {
    let mut items = Vec::new();
    for _ in 0..n {
        if r.overrun() {
            break;
        }
        let polygon = r.polygon();
        let position = r.point();
        let legend_id = r.u16();
        let width = r.i16();
        let color = r.color();
        items.push(PatternRepeat { polygon, position, legend_id, width, color });
    }
    items
}

fn read_repeat_legends(r: &mut Reader, n: i32) -> BTreeMap<u16, RepeatLegend> {
    let mut map = BTreeMap::new();
    for _ in 0..n {
        if r.overrun() {
            break;
        }
        let legend_id = r.u16();
        let is_visible = r.u16() != 0;
        let item_pos = r.point();
        let text_item_pos = r.point();
        let text = r.qstring();
        map.insert(legend_id, RepeatLegend { legend_id, is_visible, item_pos, text_item_pos, text });
    }
    map
}

fn read_row_repeats(r: &mut Reader, n: i32) -> Vec<RowRepeat> {
    let mut items = Vec::new();
    for _ in 0..n {
        if r.overrun() {
            break;
        }
        let multiplicity = r.i32();
        let len = r.i32();
        let mut rows = Vec::new();
        for _ in 0..len {
            if r.overrun() {
                break;
            }
            rows.push(r.i32());
        }
        items.push(RowRepeat { multiplicity, rows });
    }
    items
}

fn read_text_items(r: &mut Reader, n: i32) -> Vec<TextItem> {
    let mut items = Vec::new();
    for _ in 0..n {
        if r.overrun() {
            break;
        }
        let pos = r.point();
        let text = r.qstring();
        items.push(TextItem { pos, text });
    }
    items
}

fn read_labels(r: &mut Reader, n: i32) -> BTreeMap<i32, String> {
    let mut map = BTreeMap::new();
    for _ in 0..n {
        if r.overrun() {
            break;
        }
        let key = r.i32();
        let label = r.qstring();
        map.insert(key, label);
    }
    map
}

/// Reads the settings block. `api` selects how much of it exists:
/// 1 = fonts/cell size only, 2 = + label/highlight options (followed by 200
/// reserved ints), 3 = + editable-label flags.
///
/// Deviation from the original: if the extended part is missing (a few old
/// files are truncated there) the defaults are kept instead of zeros.
fn read_settings(r: &mut Reader, s: &mut ProjectSettings, api: i32) {
    let label_font = r.font();
    let label_interval = r.i32();
    let legend_font = r.font();
    let cell_w = r.i32();
    let cell_h = r.i32();

    if !label_font.family.is_empty() {
        s.label_font = label_font;
    }
    s.row_label_mode = RowLabelMode::from_code(label_interval);
    if !legend_font.family.is_empty() {
        s.legend_font = legend_font;
    }
    if cell_w != 0 {
        s.grid_cell_width = cell_w;
    }
    if cell_h != 0 {
        s.grid_cell_height = cell_h;
    }

    if api < 2 {
        return;
    }

    let row_label_start = r.i32();
    let even_loc = r.i32();
    let highlight_rows = r.i32();
    let highlight_opacity = r.i32();
    let highlight_start = r.i32();
    let highlight_color = r.qstring();
    let odd_loc = r.i32();
    let row_interval = r.i32();
    let row_interval_start = r.i32();
    let col_mode = r.i32();
    let col_interval = r.i32();
    let col_interval_start = r.i32();
    let (rows_editable, cols_editable) = if api >= 3 { (r.i32(), r.i32()) } else { (0, 0) };

    if r.overrun() {
        r.clear_overrun();
        return;
    }

    s.row_label_start = row_label_start;
    s.even_row_label_location = LabelLocation::from_code(even_loc);
    s.highlight_rows = highlight_rows != 0;
    s.highlight_rows_opacity = highlight_opacity;
    s.highlight_rows_start = highlight_start;
    if !highlight_color.is_empty() {
        s.highlight_rows_color = highlight_color;
    }
    s.odd_row_label_location = LabelLocation::from_code(odd_loc);
    s.row_labels_show_interval = row_interval;
    s.row_labels_show_interval_start = row_interval_start;
    s.column_label_mode = ColumnLabelMode::from_code(col_mode);
    s.column_labels_show_interval = col_interval;
    s.column_labels_show_interval_start = col_interval_start;
    if api >= 3 {
        s.row_labels_editable = rows_editable != 0;
        s.column_labels_editable = cols_editable != 0;
    }

    if api == 2 {
        // 200 reserved ints
        r.skip_i32(200);
        if r.overrun() {
            r.clear_overrun();
        }
    }
}

fn read_legacy(r: &mut Reader, version: i32) -> Result<Project, SpfError> {
    let num_grid = r.i32();
    let num_legend = r.i32();
    let num_colors = r.i32();
    let num_repeats = r.i32();
    let num_repeat_legends = r.i32();
    let num_row_repeats = r.i32();
    let num_text_items = r.i32();
    let _dummy = r.i32();
    check(r, "header counts")?;

    let mut p = Project { api_version: version, ..Project::default() };

    p.grid_items = read_grid_items(r, num_grid, false);
    check(r, "pattern grid items")?;
    p.legend_items = read_legend_items(r, num_legend);
    check(r, "legend items")?;
    p.colors = read_colors(r, num_colors);
    check(r, "colors")?;
    p.active_symbol = read_active_symbol(r);
    check(r, "active symbol")?;

    if version == 1 {
        read_settings(r, &mut p.settings, 1);
        check(r, "settings")?;
        let legacy = read_legacy_repeats(r, num_repeats);
        check(r, "pattern repeats")?;
        p.pattern_repeats = convert_repeats(legacy)?;
    } else {
        let legacy = read_legacy_repeats(r, num_repeats);
        check(r, "pattern repeats")?;
        p.pattern_repeats = convert_repeats(legacy)?;
        read_settings(r, &mut p.settings, 2);
        check(r, "settings")?;
        p.repeat_legends = read_repeat_legends(r, num_repeat_legends);
        check(r, "repeat legends")?;
        p.row_repeats = read_row_repeats(r, num_row_repeats);
        check(r, "row repeats")?;
        p.text_items = read_text_items(r, num_text_items);
        check(r, "text items")?;
    }
    Ok(p)
}

fn read_api3(r: &mut Reader) -> Result<Project, SpfError> {
    let mut p = Project::default();

    while !r.at_end() {
        let name = r.qstring();
        let len = r.i32();
        check(r, "section header")?;

        match name.as_str() {
            "patternGridItems" => {
                p.grid_items = read_grid_items(r, len, true);
                check(r, "pattern grid items")?;
            }
            "legendItems" => {
                p.legend_items = read_legend_items(r, len);
                check(r, "legend items")?;
            }
            "colors" => {
                p.colors = read_colors(r, len);
                check(r, "colors")?;
            }
            "activeSymbol" => {
                p.active_symbol = read_active_symbol(r);
                check(r, "active symbol")?;
            }
            "patternRepeats" => {
                p.pattern_repeats = read_pattern_repeats(r, len);
                check(r, "pattern repeats")?;
            }
            "repeatLegends" => {
                p.repeat_legends = read_repeat_legends(r, len);
                check(r, "repeat legends")?;
            }
            "rowRepeats" => {
                p.row_repeats = read_row_repeats(r, len);
                check(r, "row repeats")?;
            }
            "textItems" => {
                p.text_items = read_text_items(r, len);
                check(r, "text items")?;
            }
            "rowLabels" => {
                p.row_labels = read_labels(r, len);
                check(r, "row labels")?;
            }
            "columnLabels" => {
                p.column_labels = read_labels(r, len);
                check(r, "column labels")?;
            }
            "settings" => {
                read_settings(r, &mut p.settings, 3);
                check(r, "settings")?;
            }
            // section lengths are element counts, not byte counts, so an
            // unknown section cannot be skipped safely
            other => return Err(SpfError::UnknownSection(other.to_string())),
        }
    }
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Project {
        let mut p = Project::default();
        p.grid_items.push(GridItem {
            category: "basic".into(),
            name: "knit".into(),
            column: 3,
            row: 4,
            width: 1,
            height: 1,
            color: Color::rgb(255, 0, 128),
            is_hidden: true,
        });
        p.legend_items.push(LegendItem {
            category: "basic".into(),
            name: "knit".into(),
            item_pos: (0.0, 100.0),
            label_pos: (40.0, 100.0),
            color: Color::WHITE,
            description: "knit on RS; purl on WS".into(),
        });
        p.colors.push(ColorEntry { color: Color::rgb(1, 2, 3), state: 1 });
        p.active_symbol = Some(SymbolRef { category: "basic".into(), name: "purl".into() });
        p.pattern_repeats.push(PatternRepeat {
            polygon: vec![(0.0, 0.0), (30.0, 0.0), (30.0, 30.0), (0.0, 30.0), (0.0, 0.0)],
            position: (0.0, 0.0),
            legend_id: 7,
            width: 4,
            color: Color::rgb(0, 0, 255),
        });
        p.repeat_legends.insert(
            7,
            RepeatLegend {
                legend_id: 7,
                is_visible: true,
                item_pos: (1.0, 2.0),
                text_item_pos: (3.0, 4.0),
                text: "repeat".into(),
            },
        );
        p.row_repeats.push(RowRepeat { multiplicity: 3, rows: vec![2, 3, 4] });
        p.text_items.push(TextItem { pos: (5.0, 6.0), text: "Zöpfe".into() });
        p.row_labels.insert(2, "two".into());
        p.column_labels.insert(1, "A".into());
        p.settings.row_label_mode = RowLabelMode::ShowOddRows;
        p.settings.even_row_label_location = LabelLocation::LeftOf;
        p.settings.row_labels_editable = true;
        p.settings.highlight_rows_color = "red".into();
        p
    }

    #[test]
    fn api3_roundtrip() {
        let p = sample();
        let bytes = serialize_project(&p);
        let q = parse_project(&bytes).expect("parse");
        assert_eq!(p, q);
    }

    #[test]
    fn rejects_garbage() {
        assert!(matches!(parse_project(&[1, 2, 3, 4, 5, 6, 7, 8]), Err(SpfError::NotSpf)));
        assert!(matches!(parse_project(&[]), Err(SpfError::NotSpf)));
    }

    #[test]
    fn rejects_future_version() {
        let mut w = Writer::new();
        w.i32(MAGIC_NUMBER);
        w.i32(99);
        assert!(matches!(
            parse_project(&w.into_bytes()),
            Err(SpfError::UnsupportedVersion(99))
        ));
    }

    #[test]
    fn truncated_is_error_not_panic() {
        let bytes = serialize_project(&sample());
        for cut in [10, 40, bytes.len() / 2, bytes.len() - 3] {
            assert!(parse_project(&bytes[..cut]).is_err(), "cut at {cut}");
        }
    }

    #[test]
    fn label_codes_match_original_mapping() {
        assert_eq!(RowLabelMode::AllRows.code(), 101);
        assert_eq!(RowLabelMode::from_code(104), RowLabelMode::ShowOddRows);
        assert_eq!(RowLabelMode::from_code(7), RowLabelMode::AllRows);
        assert_eq!(LabelLocation::from_code(11), LabelLocation::LeftOf);
        assert_eq!(LabelLocation::from_code(10), LabelLocation::RightOf);
        assert_eq!(ColumnLabelMode::from_code(102), ColumnLabelMode::ShowColumnsWithInterval);
    }
}

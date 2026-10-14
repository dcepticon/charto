//! Everything the user interface needs, without any GUI dependency.
//!
//! A [`Session`] owns the open project, the editable chart (with undo/redo),
//! the symbol library and the current tool state (active symbol, colour,
//! select/hide/unhide mode). The GUI sends it user actions and renders the
//! JSON returned by [`Session::snapshot_json`] and
//! [`Session::palette_json`], which keeps the Qt layer very thin.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::chart::{ActiveSymbol, CellKey, Chart, ColumnLocation, EditError, RowLocation};
use crate::config::RecentFiles;
use crate::document::Document;
use crate::qdatastream::Color;
use crate::spf::{
    self, ColorEntry, ColumnLabelMode, LabelLocation, Project, ProjectSettings, RowLabelMode,
    SymbolRef,
};
use crate::labels;
use crate::svg;
use crate::symbols::{SymbolDef, SymbolLibrary};

pub const MAX_DIMENSION: i32 = 1000;

/// What clicking/selecting cells does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Select cells (and paint them if a symbol is active).
    Select,
    /// Hide the touched cells.
    Hide,
    /// Make hidden cells visible again.
    Unhide,
}

impl Mode {
    pub fn from_index(i: i32) -> Mode {
        match i {
            1 => Mode::Hide,
            2 => Mode::Unhide,
            _ => Mode::Select,
        }
    }

    pub fn index(self) -> i32 {
        match self {
            Mode::Select => 0,
            Mode::Hide => 1,
            Mode::Unhide => 2,
        }
    }
}

/// Parses `#rgb`, `#rrggbb`, `#aarrggbb` and a few common colour names.
pub fn parse_color(s: &str) -> Option<Color> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix('#') {
        let byte = |i: usize| u8::from_str_radix(hex.get(i..i + 2)?, 16).ok();
        return match hex.len() {
            3 => {
                let nib = |i: usize| u8::from_str_radix(hex.get(i..i + 1)?, 16).ok().map(|v| v * 17);
                Some(Color::rgb(nib(0)?, nib(1)?, nib(2)?))
            }
            6 => Some(Color::rgb(byte(0)?, byte(2)?, byte(4)?)),
            8 => Some(Color { a: byte(0)?, r: byte(2)?, g: byte(4)?, b: byte(6)? }),
            _ => None,
        };
    }
    match s.to_ascii_lowercase().as_str() {
        "white" => Some(Color::rgb(255, 255, 255)),
        "black" => Some(Color::rgb(0, 0, 0)),
        "red" => Some(Color::rgb(255, 0, 0)),
        "green" => Some(Color::rgb(0, 128, 0)),
        "blue" => Some(Color::rgb(0, 0, 255)),
        "yellow" => Some(Color::rgb(255, 255, 0)),
        "cyan" => Some(Color::rgb(0, 255, 255)),
        "magenta" => Some(Color::rgb(255, 0, 255)),
        "gray" | "grey" => Some(Color::rgb(128, 128, 128)),
        "lightgray" | "lightgrey" => Some(Color::rgb(211, 211, 211)),
        "darkgray" | "darkgrey" => Some(Color::rgb(169, 169, 169)),
        _ => None,
    }
}

/// `#rrggbb` (alpha is dropped).
pub fn color_hex(c: Color) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)
}

fn file_url(p: &Path) -> String {
    let abs = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let mut out = String::from("file://");
    for ch in abs.to_string_lossy().chars() {
        match ch {
            ' ' => out.push_str("%20"),
            '#' => out.push_str("%23"),
            '%' => out.push_str("%25"),
            '?' => out.push_str("%3F"),
            c => out.push(c),
        }
    }
    out
}

fn default_symbol(library: &SymbolLibrary) -> SymbolRef {
    match library.get("knit") {
        Some(d) => SymbolRef { category: d.category.clone(), name: d.name.clone() },
        None => SymbolRef { category: "basic".into(), name: "knit".into() },
    }
}

fn default_colors() -> Vec<ColorEntry> {
    ["#ffffff", "#d3d3d3", "#ff0000", "#ffa500", "#ffff00", "#008000", "#0000ff", "#800080", "#000000"]
        .iter()
        .filter_map(|h| parse_color(h))
        .map(|color| ColorEntry { color, state: 0 })
        .collect()
}

fn edit<T>(r: Result<T, EditError>) -> Result<T, String> {
    r.map_err(|e| e.to_string())
}

fn row_mode_index(m: RowLabelMode) -> i64 {
    match m {
        RowLabelMode::AllRows => 0,
        RowLabelMode::ShowRowsWithInterval => 1,
        RowLabelMode::ShowOddRows => 2,
        RowLabelMode::ShowEvenRows => 3,
    }
}

fn row_mode_from_index(i: i64) -> RowLabelMode {
    match i {
        1 => RowLabelMode::ShowRowsWithInterval,
        2 => RowLabelMode::ShowOddRows,
        3 => RowLabelMode::ShowEvenRows,
        _ => RowLabelMode::AllRows,
    }
}

fn side_name(l: LabelLocation) -> &'static str {
    if l == LabelLocation::LeftOf { "left" } else { "right" }
}

fn settings_to_json(s: &ProjectSettings) -> Value {
    let col_mode = i64::from(s.column_label_mode == ColumnLabelMode::ShowColumnsWithInterval);
    json!({
        "cellW": s.grid_cell_width,
        "cellH": s.grid_cell_height,
        "rowMode": row_mode_index(s.row_label_mode),
        "rowStart": s.row_label_start,
        "evenLoc": side_name(s.even_row_label_location),
        "oddLoc": side_name(s.odd_row_label_location),
        "rowInterval": s.row_labels_show_interval,
        "rowIntervalStart": s.row_labels_show_interval_start,
        "colMode": col_mode,
        "colInterval": s.column_labels_show_interval,
        "colIntervalStart": s.column_labels_show_interval_start,
        "rowsEditable": s.row_labels_editable,
        "colsEditable": s.column_labels_editable,
        "hlOn": s.highlight_rows,
        "hlOpacity": s.highlight_rows_opacity,
        "hlStart": s.highlight_rows_start,
        "hlColor": parse_color(&s.highlight_rows_color).map(color_hex).unwrap_or_else(|| "#808080".to_string()),
        "labelFont": s.label_font.family,
        "labelSize": s.label_font.point_size,
        "legendFont": s.legend_font.family,
        "legendSize": s.legend_font.point_size,
    })
}

/// Applies the fields present in `v` on top of `base`; any field out of
/// range or of the wrong type is an error.
fn settings_from_json(base: &ProjectSettings, v: &Value) -> Result<ProjectSettings, String> {
    let int = |key: &str, min: i64, max: i64| -> Result<Option<i32>, String> {
        match v.get(key) {
            None => Ok(None),
            Some(x) => {
                let n = x.as_i64().ok_or_else(|| format!("{key} must be a whole number"))?;
                if n < min || n > max {
                    Err(format!("{key} must be between {min} and {max}"))
                } else {
                    Ok(Some(n as i32))
                }
            }
        }
    };
    let float = |key: &str, min: f64, max: f64| -> Result<Option<f64>, String> {
        match v.get(key) {
            None => Ok(None),
            Some(x) => {
                let n = x.as_f64().ok_or_else(|| format!("{key} must be a number"))?;
                if n < min || n > max {
                    Err(format!("{key} must be between {min} and {max}"))
                } else {
                    Ok(Some(n))
                }
            }
        }
    };
    let boolean = |key: &str| -> Result<Option<bool>, String> {
        match v.get(key) {
            None => Ok(None),
            Some(x) => x.as_bool().map(Some).ok_or_else(|| format!("{key} must be true or false")),
        }
    };
    let text = |key: &str| -> Result<Option<String>, String> {
        match v.get(key) {
            None => Ok(None),
            Some(x) => x.as_str().map(|t| Some(t.to_string())).ok_or_else(|| format!("{key} must be text")),
        }
    };
    let side = |key: &str| -> Result<Option<LabelLocation>, String> {
        Ok(match text(key)?.as_deref() {
            None => None,
            Some("left") => Some(LabelLocation::LeftOf),
            Some("right") => Some(LabelLocation::RightOf),
            Some(_) => return Err(format!("{key} must be \"left\" or \"right\"")),
        })
    };

    let mut s = base.clone();
    if let Some(n) = int("cellW", 10, 200)? { s.grid_cell_width = n; }
    if let Some(n) = int("cellH", 10, 200)? { s.grid_cell_height = n; }
    if let Some(n) = int("rowMode", 0, 3)? { s.row_label_mode = row_mode_from_index(n as i64); }
    if let Some(n) = int("rowStart", 0, 100_000)? { s.row_label_start = n; }
    if let Some(l) = side("evenLoc")? { s.even_row_label_location = l; }
    if let Some(l) = side("oddLoc")? { s.odd_row_label_location = l; }
    if let Some(n) = int("rowInterval", 1, 1000)? { s.row_labels_show_interval = n; }
    if let Some(n) = int("rowIntervalStart", 0, 100_000)? { s.row_labels_show_interval_start = n; }
    if let Some(n) = int("colMode", 0, 1)? {
        s.column_label_mode =
            if n == 1 { ColumnLabelMode::ShowColumnsWithInterval } else { ColumnLabelMode::AllColumns };
    }
    if let Some(n) = int("colInterval", 1, 1000)? { s.column_labels_show_interval = n; }
    if let Some(n) = int("colIntervalStart", 0, 100_000)? { s.column_labels_show_interval_start = n; }
    if let Some(b) = boolean("rowsEditable")? { s.row_labels_editable = b; }
    if let Some(b) = boolean("colsEditable")? { s.column_labels_editable = b; }
    if let Some(b) = boolean("hlOn")? { s.highlight_rows = b; }
    if let Some(n) = int("hlOpacity", 0, 100)? { s.highlight_rows_opacity = n; }
    if let Some(n) = int("hlStart", 0, 1)? { s.highlight_rows_start = n; }
    if let Some(c) = text("hlColor")? {
        let color = parse_color(&c).ok_or_else(|| format!("unknown colour \"{c}\""))?;
        s.highlight_rows_color = color_hex(color);
    }
    if let Some(f) = text("labelFont")? {
        if !f.trim().is_empty() { s.label_font.family = f; }
    }
    if let Some(n) = float("labelSize", 1.0, 200.0)? { s.label_font.point_size = n; }
    if let Some(f) = text("legendFont")? {
        if !f.trim().is_empty() { s.legend_font.family = f; }
    }
    if let Some(n) = float("legendSize", 1.0, 200.0)? { s.legend_font.point_size = n; }
    Ok(s)
}

/// Number of recently used symbols kept for the quick-access row.
pub const MAX_RECENT_SYMBOLS: usize = 12;

pub struct Session {
    project: Project,
    doc: Document,
    library: SymbolLibrary,
    svg_urls: BTreeMap<String, String>,
    active: Option<String>,
    color: Color,
    mode: Mode,
    path: Option<PathBuf>,
    /// The chart as last saved or loaded, to tell whether there are unsaved changes.
    saved_chart: Chart,
    /// Unsaved changes that are not in the chart (settings, recovered data).
    force_dirty: bool,
    recent: RecentFiles,
    recent_symbols: Vec<String>,
    /// Preferences remembered for new patterns.
    defaults: Option<ProjectSettings>,
    config_dir: Option<PathBuf>,
    state_dir: Option<PathBuf>,
}

impl Session {
    /// A new session with an empty 10 x 10 chart and no files kept between
    /// runs (used by tests).
    pub fn new(library: SymbolLibrary) -> Session {
        Session::with_dirs(library, None, None)
    }

    /// Like [`Session::new`], keeping the recent files list and preferences
    /// in `config_dir` and crash-recovery files in `state_dir` (see
    /// [`crate::config`]).
    pub fn with_dirs(library: SymbolLibrary, config_dir: Option<PathBuf>, state_dir: Option<PathBuf>) -> Session {
        let svg_urls = library.iter().map(|d| (d.name.clone(), file_url(&d.svg_path))).collect();
        let chart = Chart::new(10, 10, default_symbol(&library), Color::WHITE);
        let project = Project { colors: default_colors(), ..Project::default() };
        let recent = config_dir
            .as_ref()
            .map(|d| RecentFiles::load(&d.join("recent-files.txt")))
            .unwrap_or_default();
        let defaults = config_dir.as_ref().and_then(|d| {
            let text = std::fs::read_to_string(d.join("defaults.json")).ok()?;
            let v: Value = serde_json::from_str(&text).ok()?;
            settings_from_json(&ProjectSettings::default(), &v).ok()
        });
        let mut s = Session {
            project,
            saved_chart: chart.clone(),
            doc: Document::new(chart),
            library,
            svg_urls,
            active: None,
            color: Color::WHITE,
            mode: Mode::Select,
            path: None,
            force_dirty: false,
            recent,
            recent_symbols: Vec::new(),
            defaults,
            config_dir,
            state_dir,
        };
        if let Some(d) = s.defaults.clone() {
            s.project.settings = d;
        }
        s
    }

    pub fn chart(&self) -> &Chart {
        self.doc.chart()
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    // ---- files ------------------------------------------------------------

    pub fn new_chart(&mut self, rows: i32, columns: i32) -> Result<(), String> {
        if !(1..=MAX_DIMENSION).contains(&rows) || !(1..=MAX_DIMENSION).contains(&columns) {
            return Err(format!("rows and columns must be between 1 and {MAX_DIMENSION}"));
        }
        self.clear_recovery();
        let chart = Chart::new(rows, columns, default_symbol(&self.library), Color::WHITE);
        self.project = Project {
            colors: self.project.colors.clone(),
            settings: self.defaults.clone().unwrap_or_default(),
            ..Project::default()
        };
        self.doc = Document::new(chart);
        self.path = None;
        self.recent_symbols.clear();
        self.mark_saved();
        Ok(())
    }

    /// Loads `file` as the pattern `display` (they differ when a recovery
    /// file is opened in place of the real one).
    fn load_from(&mut self, file: &Path, display: &Path) -> Result<Option<String>, String> {
        let project = spf::read_project(file).map_err(|e| format!("{}: {e}", display.display()))?;
        let chart = Chart::from_project(&project, default_symbol(&self.library), Color::WHITE)
            .map_err(|e| format!("{}: {e}", display.display()))?;
        let unknown = chart.cells().filter(|c| self.library.get(&c.symbol.name).is_none()).count();
        self.clear_recovery();
        self.project = project;
        if self.project.colors.is_empty() {
            self.project.colors = default_colors();
        }
        self.doc = Document::new(chart);
        self.path = Some(display.to_path_buf());
        self.recent_symbols.clear();
        self.mark_saved();
        Ok(if unknown > 0 {
            Some(format!("{unknown} cells use symbols that are not installed and are shown as '?'"))
        } else {
            None
        })
    }

    /// Opens an spf file. `Ok(Some(warning))` if it opened but uses symbols
    /// that are not installed.
    pub fn open(&mut self, path: &Path) -> Result<Option<String>, String> {
        let warning = self.load_from(path, path)?;
        self.add_recent(path);
        Ok(warning)
    }

    /// Saves in the current spf format (API 3). Adds `.spf` if missing.
    pub fn save(&mut self, path: &Path) -> Result<(), String> {
        let mut target = path.to_path_buf();
        if target.extension().is_none() {
            target.set_extension("spf");
        }
        self.doc.chart().write_to_project(&mut self.project);
        self.project.active_symbol = self.active.as_ref().and_then(|name| {
            self.library
                .get(name)
                .map(|d| SymbolRef { category: d.category.clone(), name: d.name.clone() })
        });
        spf::write_project(&target, &self.project).map_err(|e| format!("{}: {e}", target.display()))?;
        self.clear_recovery(); // of the old name
        self.path = Some(target.clone());
        self.clear_recovery(); // and of the new one
        self.mark_saved();
        self.add_recent(&target);
        Ok(())
    }

    /// Exports the chart as a vector graphic. Adds `.svg` if missing.
    pub fn export_svg(&self, path: &Path) -> Result<(), String> {
        let text = svg::render(self.doc.chart(), &self.project.settings, &self.library)?;
        let mut target = path.to_path_buf();
        if target.extension().is_none() {
            target.set_extension("svg");
        }
        std::fs::write(&target, text).map_err(|e| format!("{}: {e}", target.display()))
    }

    // ---- unsaved changes, recent files, crash recovery ---------------------

    fn mark_saved(&mut self) {
        self.saved_chart = self.doc.chart().clone();
        self.force_dirty = false;
    }

    /// True if there is anything that has not been saved to the pattern file.
    pub fn is_dirty(&self) -> bool {
        self.force_dirty || !self.doc.chart().same_content(&self.saved_chart)
    }

    fn add_recent(&mut self, path: &Path) {
        let abs = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        self.recent.add(abs);
        self.persist_recent();
    }

    fn persist_recent(&self) {
        if let Some(dir) = &self.config_dir {
            let _ = self.recent.save(&dir.join("recent-files.txt"));
        }
    }

    pub fn recent_files(&self) -> &[PathBuf] {
        self.recent.files()
    }

    pub fn clear_recent_files(&mut self) {
        self.recent.clear();
        self.persist_recent();
    }

    /// Where the recovery copy of the pattern file `file` lives.
    pub fn recovery_path(&self, file: &Path) -> Option<PathBuf> {
        let dir = self.state_dir.as_ref()?.join("recovery");
        let abs = std::fs::canonicalize(file).unwrap_or_else(|_| file.to_path_buf());
        let name: String = abs
            .file_name()?
            .to_string_lossy()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') { c } else { '_' })
            .collect();
        // FNV-1a over the full path keeps same-named files in different folders apart
        let mut hash: u32 = 0x811c_9dc5;
        for b in abs.to_string_lossy().as_bytes() {
            hash ^= *b as u32;
            hash = hash.wrapping_mul(16_777_619);
        }
        Some(dir.join(format!("{name}-{hash:08x}.recovery")))
    }

    pub fn has_recovery(&self, file: &Path) -> bool {
        self.recovery_path(file).map_or(false, |p| p.exists())
    }

    /// Writes a recovery copy if the open pattern has unsaved changes
    /// (called every couple of minutes). Returns whether one was written.
    pub fn autosave(&mut self) -> Result<bool, String> {
        let Some(path) = self.path.clone() else { return Ok(false) };
        let Some(recovery) = self.recovery_path(&path) else { return Ok(false) };
        if !self.is_dirty() {
            let _ = std::fs::remove_file(&recovery);
            return Ok(false);
        }
        if let Some(dir) = recovery.parent() {
            std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        }
        self.doc.chart().write_to_project(&mut self.project);
        spf::write_project(&recovery, &self.project).map_err(|e| format!("{}: {e}", recovery.display()))?;
        Ok(true)
    }

    /// Opens the recovery copy of `file` in its place; the pattern then
    /// counts as having unsaved changes.
    pub fn open_recovery(&mut self, file: &Path) -> Result<Option<String>, String> {
        let recovery = self
            .recovery_path(file)
            .filter(|p| p.exists())
            .ok_or_else(|| "there is no recovery file for this pattern".to_string())?;
        let warning = self.load_from(&recovery, file)?;
        self.force_dirty = true;
        Ok(warning)
    }

    /// Removes the recovery copy of the open pattern (after saving or a clean exit).
    pub fn clear_recovery(&mut self) {
        if let Some(path) = self.path.clone() {
            if let Some(recovery) = self.recovery_path(&path) {
                let _ = std::fs::remove_file(recovery);
            }
        }
    }

    // ---- tools ------------------------------------------------------------

    pub fn set_active_symbol(&mut self, name: &str) -> Result<(), String> {
        if name.is_empty() {
            self.active = None;
        } else if self.library.get(name).is_some() {
            self.active = Some(name.to_string());
            self.recent_symbols.retain(|n| n != name);
            self.recent_symbols.insert(0, name.to_string());
            self.recent_symbols.truncate(MAX_RECENT_SYMBOLS);
        } else {
            return Err(format!("unknown symbol \"{name}\""));
        }
        Ok(())
    }

    pub fn set_color(&mut self, hex: &str) -> Result<(), String> {
        self.color = parse_color(hex).ok_or_else(|| format!("unknown colour \"{hex}\""))?;
        Ok(())
    }

    /// Makes `hex` the active colour and adds it to the colour list.
    pub fn add_color(&mut self, hex: &str) -> Result<(), String> {
        self.set_color(hex)?;
        if !self.project.colors.iter().any(|c| c.color == self.color) {
            self.project.colors.push(ColorEntry { color: self.color, state: 0 });
        }
        Ok(())
    }

    pub fn set_mode(&mut self, index: i32) {
        self.mode = Mode::from_index(index);
    }

    /// The symbol that new cells are painted with: the active symbol in the
    /// active colour, unless the symbol defines its own background colour.
    fn active_symbol(&self) -> Option<ActiveSymbol> {
        let def = self.library.get(self.active.as_deref()?)?;
        let color = def.background_color.as_deref().and_then(parse_color).unwrap_or(self.color);
        Some(ActiveSymbol {
            symbol: SymbolRef { category: def.category.clone(), name: def.name.clone() },
            width: def.width as i32,
            color,
        })
    }

    // ---- selection --------------------------------------------------------

    fn apply_keys(&mut self, keys: Vec<CellKey>) {
        match self.mode {
            Mode::Select => {
                let active = self.active_symbol();
                self.doc.select(&keys, active.as_ref());
            }
            Mode::Hide => self.doc.hide_cells(&keys),
            Mode::Unhide => self.doc.unhide_cells(&keys),
        }
    }

    pub fn click_cell(&mut self, row: i32, column: i32) {
        if let Some(c) = self.doc.chart().cell_at(row, column) {
            let key = c.key();
            self.apply_keys(vec![key]);
        }
    }

    pub fn select_rect(&mut self, column: i32, row: i32, num_columns: i32, num_rows: i32) {
        let keys = self.doc.chart().keys_in_rect(column, row, num_columns, num_rows);
        self.apply_keys(keys);
    }

    pub fn select_row(&mut self, row: i32) {
        let keys = self.doc.chart().row_keys(row);
        self.apply_keys(keys);
    }

    pub fn select_column(&mut self, column: i32) {
        let keys = self.doc.chart().column_keys(column);
        self.apply_keys(keys);
    }

    pub fn clear_selection(&mut self) {
        self.doc.clear_selection();
    }

    pub fn select_same_color(&mut self, row: i32, column: i32) {
        if let Some(c) = self.doc.chart().cell_at(row, column) {
            let key = c.key();
            self.doc.select_same(key, true);
        }
    }

    pub fn select_same_symbol(&mut self, row: i32, column: i32) {
        if let Some(c) = self.doc.chart().cell_at(row, column) {
            let key = c.key();
            self.doc.select_same(key, false);
        }
    }

    // ---- editing ----------------------------------------------------------

    pub fn apply_color(&mut self) -> Result<(), String> {
        edit(self.doc.apply_color(self.color))
    }

    pub fn undo(&mut self) {
        self.doc.undo();
    }

    pub fn redo(&mut self) {
        self.doc.redo();
    }

    pub fn copy(&mut self) {
        self.doc.copy();
    }

    /// `at` = `(column, row)` of a clicked cell, or `None` to paste into the
    /// current selection.
    pub fn paste(&mut self, at: Option<(i32, i32)>) -> Result<(), String> {
        edit(self.doc.paste(at))
    }

    pub fn insert_rows(&mut self, num: i32, above: bool) -> Result<(), String> {
        let loc = if above { RowLocation::Above } else { RowLocation::Below };
        edit(self.doc.insert_rows(num, loc))
    }

    pub fn delete_rows(&mut self) -> Result<(), String> {
        edit(self.doc.delete_rows())
    }

    pub fn insert_columns(&mut self, num: i32, left: bool) -> Result<(), String> {
        let loc = if left { ColumnLocation::LeftOf } else { ColumnLocation::RightOf };
        edit(self.doc.insert_columns(num, loc))
    }

    pub fn delete_columns(&mut self) -> Result<(), String> {
        edit(self.doc.delete_columns())
    }

    pub fn add_row_repeat(&mut self, multiplicity: i32) -> Result<(), String> {
        edit(self.doc.add_row_repeat(multiplicity))
    }

    pub fn delete_row_repeat(&mut self) -> Result<(), String> {
        edit(self.doc.delete_row_repeat())
    }

    pub fn set_row_label(&mut self, row: i32, text: &str) -> Result<(), String> {
        edit(self.doc.set_row_label(row, text))
    }

    pub fn set_column_label(&mut self, column: i32, text: &str) -> Result<(), String> {
        edit(self.doc.set_column_label(column, text))
    }

    // ---- preferences ------------------------------------------------------

    /// The project settings as JSON for the preferences dialog.
    pub fn settings_json(&self) -> String {
        settings_to_json(&self.project.settings).to_string()
    }

    /// Applies settings from the preferences dialog to this pattern and
    /// remembers them for new patterns.
    pub fn apply_settings_json(&mut self, json: &str) -> Result<(), String> {
        let v: Value = serde_json::from_str(json).map_err(|e| format!("invalid settings: {e}"))?;
        let new = settings_from_json(&self.project.settings, &v)?;
        if new != self.project.settings {
            self.project.settings = new.clone();
            self.force_dirty = true;
        }
        self.defaults = Some(new);
        if let Some(dir) = &self.config_dir {
            let _ = std::fs::create_dir_all(dir);
            let _ = std::fs::write(dir.join("defaults.json"), self.settings_json());
        }
        Ok(())
    }

    // ---- what the GUI shows -----------------------------------------------

    fn svg_url(&self, name: &str) -> String {
        self.svg_urls.get(name).cloned().unwrap_or_default()
    }

    fn symbol_json(&self, d: &SymbolDef) -> Value {
        json!({
            "name": d.name,
            "description": d.description,
            "w": d.width,
            "svg": self.svg_url(&d.name),
            "bg": d.background_color.as_deref().and_then(parse_color).map(color_hex).unwrap_or_default(),
        })
    }

    /// Everything needed to draw the window, as JSON.
    pub fn snapshot_json(&self) -> String {
        let chart = self.doc.chart();
        let s = &self.project.settings;
        let rows = chart.num_rows();
        let cols = chart.num_columns();

        let cells: Vec<Value> = chart
            .cells()
            .map(|c| {
                json!({
                    "r": c.row,
                    "c": c.column,
                    "w": c.width,
                    "name": c.symbol.name,
                    "svg": self.svg_url(&c.symbol.name),
                    "bg": color_hex(c.color),
                    "sel": chart.is_selected(c.key()),
                    "hid": c.hidden,
                })
            })
            .collect();

        // rows are numbered from the bottom, columns from the right
        let row_labels: Vec<Value> = labels::row_labels(chart, s)
            .into_iter()
            .map(|l| {
                let side = if l.left { "left" } else { "right" };
                json!({ "row": l.row, "text": l.text, "side": side })
            })
            .collect();
        let col_labels: Vec<Value> = labels::column_labels(chart, s)
            .into_iter()
            .map(|l| json!({ "col": l.col, "text": l.text }))
            .collect();
        let highlight_color = parse_color(&s.highlight_rows_color)
            .map(color_hex)
            .unwrap_or_else(|| "#808080".to_string());

        let repeats: Vec<Value> = chart
            .row_repeats()
            .iter()
            .filter_map(|rep| {
                let first = *rep.rows.iter().min()?;
                let last = *rep.rows.iter().max()?;
                Some(json!({ "first": first, "last": last, "mult": rep.multiplicity }))
            })
            .collect();

        let colors: Vec<String> = self.project.colors.iter().map(|c| color_hex(c.color)).collect();
        let file = self
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();

        json!({
            "rows": rows,
            "cols": cols,
            "cellW": s.grid_cell_width.max(8),
            "cellH": s.grid_cell_height.max(8),
            "cells": cells,
            "rowLabels": row_labels,
            "colLabels": col_labels,
            "repeats": repeats,
            "canUndo": self.doc.can_undo(),
            "canRedo": self.doc.can_redo(),
            "undoText": self.doc.undo_text().unwrap_or(""),
            "redoText": self.doc.redo_text().unwrap_or(""),
            "hasClipboard": !self.doc.clipboard().is_empty(),
            "selected": chart.selection_len(),
            "active": self.active.clone().unwrap_or_default(),
            "mode": self.mode.index(),
            "color": color_hex(self.color),
            "colors": colors,
            "file": file,
            "dirty": self.is_dirty(),
            "rowsEditable": s.row_labels_editable,
            "colsEditable": s.column_labels_editable,
            "highlight": {
                "on": s.highlight_rows,
                "color": highlight_color,
                "opacity": s.highlight_rows_opacity as f64 / 100.0,
                "rows": labels::highlighted_rows(chart, s),
            },
            "recent": self.recent.files().iter().map(|p| json!({
                "path": p.to_string_lossy(),
                "name": p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
            })).collect::<Vec<Value>>(),
            "recentSymbols": self.recent_symbols.iter()
                .filter_map(|n| self.library.get(n))
                .map(|d| self.symbol_json(d))
                .collect::<Vec<Value>>(),
            "path": self.path.as_ref().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default(),
        })
        .to_string()
    }

    /// The symbol palette: category tabs (alphabetical) with their symbols.
    pub fn palette_json(&self) -> String {
        let cats: Vec<Value> = self
            .library
            .categories()
            .into_iter()
            .map(|(category, symbols)| {
                let syms: Vec<Value> = symbols
                    .iter()
                    .map(|d| {
                        json!({
                            "name": d.name,
                            "description": d.description,
                            "w": d.width,
                            "svg": self.svg_url(&d.name),
                            "bg": d.background_color.as_deref().and_then(parse_color).map(color_hex).unwrap_or_default(),
                        })
                    })
                    .collect();
                json!({ "category": category, "symbols": syms })
            })
            .collect();
        Value::Array(cats).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn library() -> SymbolLibrary {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/symbols");
        let (lib, skipped) = SymbolLibrary::load(&[root]);
        assert!(skipped.is_empty());
        lib
    }

    fn session() -> Session {
        Session::new(library())
    }

    fn snap(s: &Session) -> Value {
        serde_json::from_str(&s.snapshot_json()).expect("valid json")
    }

    fn cell(v: &Value, r: i64, c: i64) -> Value {
        v["cells"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["r"] == r && x["c"] == c)
            .cloned()
            .expect("cell exists")
    }

    #[test]
    fn colors_parse_and_format() {
        assert_eq!(parse_color("#fff"), Some(Color::rgb(255, 255, 255)));
        assert_eq!(parse_color("#6a6a6a"), Some(Color::rgb(0x6a, 0x6a, 0x6a)));
        assert_eq!(parse_color("#80ff0000"), Some(Color { a: 0x80, r: 255, g: 0, b: 0 }));
        assert_eq!(parse_color("Gray"), Some(Color::rgb(128, 128, 128)));
        assert_eq!(parse_color("#12"), None);
        assert_eq!(parse_color("#gggggg"), None);
        assert_eq!(parse_color("nonsense"), None);
        assert_eq!(color_hex(Color::rgb(1, 2, 255)), "#0102ff");
    }

    #[test]
    fn new_session_snapshot() {
        let s = session();
        let v = snap(&s);
        assert_eq!(v["rows"], 10);
        assert_eq!(v["cols"], 10);
        assert_eq!(v["cells"].as_array().unwrap().len(), 100);
        assert_eq!(cell(&v, 0, 0)["name"], "knit");
        assert!(cell(&v, 0, 0)["svg"].as_str().unwrap().ends_with("knit.svg"));
        assert_eq!(v["canUndo"], false);
        assert_eq!(v["mode"], 0);
    }

    #[test]
    fn labels_count_rows_from_the_bottom_and_columns_from_the_right() {
        let v = snap(&session());
        let rl = v["rowLabels"].as_array().unwrap();
        assert_eq!(rl.len(), 10);
        assert_eq!(rl[0]["row"], 0);
        assert_eq!(rl[0]["text"], "10");
        assert_eq!(rl[9]["text"], "1");
        assert_eq!(rl[0]["side"], "right");
        let cl = v["colLabels"].as_array().unwrap();
        assert_eq!(cl[0]["text"], "10");
        assert_eq!(cl[9]["text"], "1");
    }

    #[test]
    fn painting_with_an_active_symbol_and_undo() {
        let mut s = session();
        s.set_active_symbol("purl").unwrap();
        s.click_cell(0, 0);
        assert_eq!(cell(&snap(&s), 0, 0)["name"], "purl");
        s.undo();
        assert_eq!(cell(&snap(&s), 0, 0)["name"], "knit");
        s.redo();
        assert_eq!(cell(&snap(&s), 0, 0)["name"], "purl");
        assert!(s.set_active_symbol("no such symbol").is_err());
    }

    #[test]
    fn multi_cell_symbol_needs_enough_clicks() {
        let lib = library();
        let wide = lib.iter().find(|d| d.width == 2).expect("a 2-wide symbol").name.clone();
        let mut s = Session::new(lib);
        s.set_active_symbol(&wide).unwrap();
        s.click_cell(0, 0);
        assert_eq!(snap(&s)["selected"], 1);
        s.click_cell(0, 1);
        let v = snap(&s);
        assert_eq!(v["selected"], 0);
        let c = cell(&v, 0, 0);
        assert_eq!(c["w"], 2);
        assert_eq!(c["name"], wide.as_str());
    }

    #[test]
    fn symbols_with_their_own_background_colour_override_the_active_colour() {
        let mut s = session();
        s.set_color("#ff0000").unwrap();
        s.set_active_symbol("nostitch").unwrap();
        s.click_cell(0, 0);
        assert_eq!(cell(&snap(&s), 0, 0)["bg"], "#6a6a6a");
        s.set_active_symbol("purl").unwrap();
        s.click_cell(1, 1);
        assert_eq!(cell(&snap(&s), 1, 1)["bg"], "#ff0000");
    }

    #[test]
    fn hide_and_unhide_modes() {
        let mut s = session();
        s.set_mode(1);
        s.click_cell(2, 2);
        let v = snap(&s);
        assert_eq!(cell(&v, 2, 2)["hid"], true);
        assert_eq!(v["selected"], 0);
        s.set_mode(2);
        s.click_cell(2, 2);
        assert_eq!(cell(&snap(&s), 2, 2)["hid"], false);
    }

    #[test]
    fn edit_errors_become_messages() {
        let mut s = session();
        let err = s.insert_rows(1, true).unwrap_err();
        assert!(err.contains("exactly one row"), "{err}");
        assert!(s.apply_color().is_err());
        assert!(s.new_chart(0, 5).is_err());
        assert!(s.new_chart(5, MAX_DIMENSION + 1).is_err());
    }

    #[test]
    fn copy_paste_and_structure_edits() {
        let mut s = session();
        s.set_active_symbol("purl").unwrap();
        s.click_cell(0, 0);
        s.set_active_symbol("").unwrap();
        s.click_cell(0, 0);
        s.copy();
        assert_eq!(snap(&s)["hasClipboard"], true);
        s.paste(Some((3, 3))).unwrap();
        assert_eq!(cell(&snap(&s), 3, 3)["name"], "purl");

        s.click_cell(5, 5);
        s.insert_rows(2, false).unwrap();
        assert_eq!(snap(&s)["rows"], 12);
        s.click_cell(0, 0);
        s.insert_columns(1, true).unwrap();
        assert_eq!(snap(&s)["cols"], 11);
        s.chart().validate().unwrap();

        s.click_cell(1, 1);
        s.add_row_repeat(3).unwrap();
        let v = snap(&s);
        assert_eq!(v["repeats"][0]["first"], 1);
        assert_eq!(v["repeats"][0]["mult"], 3);
        s.click_cell(1, 1);
        s.delete_row_repeat().unwrap();
        assert!(snap(&s)["repeats"].as_array().unwrap().is_empty());
    }

    #[test]
    fn sample_project_opens_saves_and_reopens() {
        let sample = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_api_1.spf");
        let mut s = session();
        assert_eq!(s.open(&sample).unwrap(), None);
        let v = snap(&s);
        assert_eq!(v["rows"], 14);
        assert_eq!(v["cols"], 18);
        assert_eq!(v["file"], "test_api_1.spf");

        s.set_active_symbol("purl").unwrap();
        s.click_cell(0, 0);
        let out = std::env::temp_dir().join(format!("charto_session_test_{}", std::process::id()));
        s.save(&out).unwrap();
        let saved = out.with_extension("spf");
        assert!(saved.exists());

        let mut t = session();
        t.open(&saved).unwrap();
        assert_eq!(cell(&snap(&t), 0, 0)["name"], "purl");
        assert_eq!(snap(&t)["cells"].as_array().unwrap().len(), v["cells"].as_array().unwrap().len());
        let _ = std::fs::remove_file(saved);
    }

    #[test]
    fn opening_a_missing_or_broken_file_is_an_error() {
        let mut s = session();
        assert!(s.open(Path::new("/definitely/not/here.spf")).is_err());
        let junk = std::env::temp_dir().join(format!("charto_junk_{}.spf", std::process::id()));
        std::fs::write(&junk, b"this is not a pattern").unwrap();
        assert!(s.open(&junk).is_err());
        let _ = std::fs::remove_file(junk);
        // the session is still usable
        assert_eq!(snap(&s)["rows"], 10);
    }

    fn temp_dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("charto_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn sample() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_api_1.spf")
    }

    #[test]
    fn unsaved_changes_are_tracked() {
        let mut s = session();
        assert!(!s.is_dirty());
        s.click_cell(0, 0); // only a selection
        assert!(!s.is_dirty());
        s.clear_selection();
        s.set_active_symbol("purl").unwrap();
        s.click_cell(0, 0);
        assert!(s.is_dirty());
        s.undo();
        assert!(!s.is_dirty()); // back to the saved content
        s.redo();
        assert_eq!(snap(&s)["dirty"], true);

        let dir = temp_dir("dirty");
        s.save(&dir.join("pattern")).unwrap(); // ".spf" is added
        assert!(dir.join("pattern.spf").exists());
        assert!(!s.is_dirty());
        s.apply_settings_json(r#"{"cellW": 40}"#).unwrap();
        assert!(s.is_dirty()); // settings count too
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn recent_files_are_kept_between_sessions() {
        let cfg = temp_dir("recent_cfg");
        let work = temp_dir("recent_work");
        let a = work.join("a.spf");
        std::fs::copy(sample(), &a).unwrap();

        let mut s = Session::with_dirs(library(), Some(cfg.clone()), None);
        s.open(&a).unwrap();
        s.save(&work.join("b.spf")).unwrap();
        assert_eq!(s.recent_files().len(), 2);
        assert!(s.recent_files()[0].ends_with("b.spf"));
        assert_eq!(snap(&s)["recent"][0]["name"], "b.spf");

        let mut t = Session::with_dirs(library(), Some(cfg.clone()), None);
        assert_eq!(t.recent_files(), s.recent_files());
        t.clear_recent_files();
        let u = Session::with_dirs(library(), Some(cfg.clone()), None);
        assert!(u.recent_files().is_empty());
        let _ = std::fs::remove_dir_all(cfg);
        let _ = std::fs::remove_dir_all(work);
    }

    #[test]
    fn crash_recovery_roundtrip() {
        let state = temp_dir("rec_state");
        let work = temp_dir("rec_work");
        let file = work.join("pattern.spf");
        std::fs::copy(sample(), &file).unwrap();

        let mut s = Session::with_dirs(library(), None, Some(state.clone()));
        s.open(&file).unwrap();
        let original = cell(&snap(&s), 0, 0)["name"].as_str().unwrap().to_string();
        let changed = if original == "purl" { "knit" } else { "purl" };
        assert!(!s.has_recovery(&file));
        assert!(!s.autosave().unwrap()); // nothing to save yet
        s.set_active_symbol(changed).unwrap();
        s.click_cell(0, 0);
        assert!(s.autosave().unwrap());
        assert!(s.has_recovery(&file));

        // "crash": a new session finds the recovery copy and can open it
        let mut t = Session::with_dirs(library(), None, Some(state.clone()));
        assert!(t.has_recovery(&file));
        t.open_recovery(&file).unwrap();
        assert_eq!(cell(&snap(&t), 0, 0)["name"], changed);
        assert!(t.is_dirty());
        assert_eq!(t.path(), Some(file.as_path()));

        // the pattern file itself is untouched until it is saved
        let mut u = Session::with_dirs(library(), None, None);
        u.open(&file).unwrap();
        assert_eq!(cell(&snap(&u), 0, 0)["name"], original.as_str());

        t.save(&file).unwrap();
        assert!(!t.has_recovery(&file));
        assert!(!t.is_dirty());
        assert!(t.open_recovery(&file).is_err());

        // clearing removes a recovery copy too
        s.set_active_symbol(original.as_str()).unwrap();
        s.click_cell(1, 0);
        s.autosave().unwrap();
        assert!(s.has_recovery(&file));
        s.clear_recovery();
        assert!(!s.has_recovery(&file));
        let _ = std::fs::remove_dir_all(state);
        let _ = std::fs::remove_dir_all(work);
    }

    #[test]
    fn same_named_files_in_different_folders_get_different_recovery_files() {
        let state = temp_dir("rec_names");
        let s = Session::with_dirs(library(), None, Some(state.clone()));
        let a = s.recovery_path(Path::new("/one/pattern.spf")).unwrap();
        let b = s.recovery_path(Path::new("/two/pattern.spf")).unwrap();
        assert_ne!(a, b);
        assert!(a.to_string_lossy().contains("pattern.spf-"));
        assert!(Session::new(library()).recovery_path(Path::new("/x/y.spf")).is_none());
        let _ = std::fs::remove_dir_all(state);
    }

    #[test]
    fn preferences_are_validated_applied_and_remembered() {
        let cfg = temp_dir("prefs");
        let mut s = Session::with_dirs(library(), Some(cfg.clone()), None);
        s.apply_settings_json(
            r##"{"cellW":44,"cellH":33,"rowMode":2,"hlOn":false,"hlColor":"#336699",
                 "labelFont":"DejaVu Sans","labelSize":12.5,"evenLoc":"left","rowsEditable":true}"##,
        )
        .unwrap();
        let v = snap(&s);
        assert_eq!(v["cellW"], 44);
        assert_eq!(v["cellH"], 33);
        assert_eq!(v["highlight"]["on"], false);
        assert_eq!(v["highlight"]["color"], "#336699");
        assert_eq!(v["rowsEditable"], true);
        let rows: Vec<String> =
            v["rowLabels"].as_array().unwrap().iter().map(|l| l["text"].as_str().unwrap().to_string()).collect();
        assert_eq!(rows, vec!["9", "7", "5", "3", "1"]); // odd rows only

        // bad input is rejected and changes nothing
        assert!(s.apply_settings_json(r#"{"cellW": 5}"#).is_err());
        assert!(s.apply_settings_json(r#"{"evenLoc": "up"}"#).is_err());
        assert!(s.apply_settings_json(r#"{"hlColor": "nonsense"}"#).is_err());
        assert!(s.apply_settings_json(r#"{"rowsEditable": "yes"}"#).is_err());
        assert!(s.apply_settings_json("not json").is_err());
        assert_eq!(snap(&s)["cellW"], 44);

        // remembered for new patterns, also in the next session
        s.new_chart(5, 5).unwrap();
        assert_eq!(snap(&s)["cellW"], 44);
        let t = Session::with_dirs(library(), Some(cfg.clone()), None);
        assert_eq!(snap(&t)["cellW"], 44);

        // the dialog's JSON round-trips
        let json = s.settings_json();
        let mut u = session();
        u.apply_settings_json(&json).unwrap();
        assert_eq!(u.settings_json(), json);
        let _ = std::fs::remove_dir_all(cfg);
    }

    #[test]
    fn selecting_similar_cells() {
        let mut s = session();
        s.set_active_symbol("purl").unwrap();
        s.click_cell(0, 0);
        s.click_cell(5, 5);
        s.set_active_symbol("").unwrap();
        s.select_same_symbol(0, 0);
        assert_eq!(snap(&s)["selected"], 2);
        s.clear_selection();
        s.select_same_color(3, 3); // everything is white
        assert_eq!(snap(&s)["selected"], 100);
        s.undo();
        assert_eq!(snap(&s)["selected"], 0);
    }

    #[test]
    fn recently_used_symbols() {
        let mut s = session();
        assert!(snap(&s)["recentSymbols"].as_array().unwrap().is_empty());
        for name in ["purl", "knit", "purl"] {
            s.set_active_symbol(name).unwrap();
        }
        s.set_active_symbol("").unwrap(); // deselecting is not "using" a symbol
        let v = snap(&s);
        let names: Vec<&str> = v["recentSymbols"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, vec!["purl", "knit"]);
        s.new_chart(3, 3).unwrap(); // a new pattern starts with a clean list
        assert!(snap(&s)["recentSymbols"].as_array().unwrap().is_empty());
    }

    #[test]
    fn label_editing_and_row_highlighting_in_the_snapshot() {
        let mut s = session();
        let v = snap(&s);
        assert_eq!(v["highlight"]["on"], true);
        assert_eq!(v["highlight"]["opacity"], 0.1);
        assert_eq!(v["highlight"]["rows"], json!([1, 3, 5, 7, 9]));

        s.set_row_label(0, "top").unwrap();
        // typed labels only show once row labels are editable
        assert_eq!(snap(&s)["rowLabels"][0]["text"], "10");
        s.apply_settings_json(r#"{"rowsEditable": true}"#).unwrap();
        assert_eq!(snap(&s)["rowLabels"][0]["text"], "top");
        s.undo();
        assert_eq!(snap(&s)["rowLabels"][0]["text"], "10");
        assert!(s.set_row_label(99, "x").is_err());
        assert!(s.set_column_label(99, "x").is_err());
    }

    #[test]
    fn svg_export_writes_a_file() {
        let mut s = session();
        s.set_mode(1);
        s.click_cell(0, 0);
        let dir = temp_dir("svg");
        s.export_svg(&dir.join("chart")).unwrap(); // ".svg" is added
        let text = std::fs::read_to_string(dir.join("chart.svg")).unwrap();
        let doc = roxmltree::Document::parse(&text).expect("well-formed SVG");
        let uses = doc.descendants().filter(|n| n.is_element() && n.tag_name().name() == "use").count();
        assert_eq!(uses, 99); // one cell is hidden
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn palette_groups_symbols_by_category() {
        let s = session();
        let v: Value = serde_json::from_str(&s.palette_json()).unwrap();
        let cats = v.as_array().unwrap();
        assert!(cats.iter().any(|c| c["category"] == "basic"));
        let total: usize = cats.iter().map(|c| c["symbols"].as_array().unwrap().len()).sum();
        assert_eq!(total, 88);
    }
}

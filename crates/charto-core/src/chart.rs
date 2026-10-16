//! The editable chart: grid of stitch cells, selection, symbol placement,
//! copy/paste, row/column insertion and deletion, labels and row repeats.
//!
//! Ported from `gui/pattern_canvas.py`, `gui/undo_framework.py` and
//! `util/canvas.py` of the original, minus everything that is drawing.
//!
//! Model: the chart is `num_rows` x `num_columns` unit cells. Every row is
//! tiled without gaps or overlaps by [`Cell`]s of width >= 1 (a cable
//! symbol is a cell of width 4, for instance). Rows are numbered 0.. from
//! the top, as in the original and in `.spf` files; the *displayed* row
//! number is derived by the UI.
//!
//! Cells are addressed by [`CellKey`] = `(row, column of the cell's left
//! edge)`.
//!
//! Deliberate deviations from the original (each one avoids a way the
//! original could leave the grid inconsistent):
//! * pasting into a non-rectangular selection requires the *exact same*
//!   covered cells per row, not just a subset;
//! * deleting columns is refused if any symbol would be cut in two;
//! * deleting rows/columns never removes the last remaining one;
//! * deleting rows keeps row repeats consistent even when rows inside a
//!   repeat are removed.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use crate::qdatastream::Color;
use crate::spf::{GridItem, Project, RowRepeat, SymbolRef};

/// `(row, column)` of a cell's left edge.
pub type CellKey = (i32, i32);

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub column: i32,
    pub row: i32,
    pub width: i32,
    pub symbol: Arc<SymbolRef>,
    pub color: Color,
    pub hidden: bool,
}

impl Cell {
    pub fn key(&self) -> CellKey {
        (self.row, self.column)
    }
}

/// The symbol currently armed for painting. `color` is the colour the new
/// cells get; callers resolve a symbol's own background colour (which
/// overrides the active colour in the original) before building this.
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveSymbol {
    pub symbol: SymbolRef,
    pub width: i32,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowLocation {
    Above,
    Below,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnLocation {
    LeftOf,
    RightOf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditError {
    InconsistentGrid(String),
    NoSelection,
    SymbolDoesNotFit,
    NoClipboard,
    PasteGeometry,
    PasteMismatch,
    PasteBlocked,
    NoPasteTarget,
    NeedSingleRow,
    NeedSingleColumn,
    CannotInsertColumns,
    CannotDeleteColumns,
    CannotDeleteEverything,
    CannotAddRowRepeat,
    CannotDeleteRowRepeat,
    BadCount,
}

impl fmt::Display for EditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EditError::*;
        match self {
            InconsistentGrid(why) => write!(f, "the pattern grid is inconsistent: {why}"),
            NoSelection => write!(f, "no cells are selected"),
            SymbolDoesNotFit => write!(f, "the active symbol does not fit into the selected cells"),
            NoClipboard => write!(f, "nothing has been copied yet"),
            PasteGeometry => write!(f, "the selection is not a multiple of the copied area"),
            PasteMismatch => write!(f, "the selection does not have the same shape as the copied cells"),
            PasteBlocked => write!(f, "the copied cells do not fit at this location"),
            NoPasteTarget => write!(f, "select the cells to paste into or click a target cell"),
            NeedSingleRow => write!(f, "select cells in exactly one row"),
            NeedSingleColumn => write!(f, "select cells in exactly one stitch column"),
            CannotInsertColumns => write!(f, "stitches cannot be inserted here without splitting a symbol"),
            CannotDeleteColumns => write!(f, "these stitches cannot be deleted without splitting a symbol"),
            CannotDeleteEverything => write!(f, "the last row or stitch cannot be deleted"),
            CannotAddRowRepeat => write!(f, "the selected rows cannot form a row repeat"),
            CannotDeleteRowRepeat => write!(f, "the selected rows are not part of a single row repeat"),
            BadCount => write!(f, "the number must be at least 1"),
        }
    }
}

impl std::error::Error for EditError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Chart {
    num_rows: i32,
    num_columns: i32,
    cells: BTreeMap<CellKey, Cell>,
    selection: BTreeSet<CellKey>,
    pub row_labels: BTreeMap<i32, String>,
    pub column_labels: BTreeMap<i32, String>,
    row_repeats: Vec<RowRepeat>,
    default_symbol: Arc<SymbolRef>,
    default_color: Color,
}

// ---------------------------------------------------------------------------
// helpers on groups of cells (ports of util/canvas.py)
// ---------------------------------------------------------------------------

fn by_row<'a>(cells: &[&'a Cell]) -> BTreeMap<i32, Vec<&'a Cell>> {
    let mut map: BTreeMap<i32, Vec<&'a Cell>> = BTreeMap::new();
    for c in cells {
        map.entry(c.row).or_default().push(*c);
    }
    for row in map.values_mut() {
        row.sort_by_key(|c| c.column);
    }
    map
}

/// Row and column of the upper left corner: the top row, and the leftmost
/// column *within that row*.
fn upper_left(cells: &[&Cell]) -> Option<(i32, i32)> {
    let min_row = cells.iter().map(|c| c.row).min()?;
    let min_col = cells.iter().filter(|c| c.row == min_row).map(|c| c.column).min()?;
    Some((min_row, min_col))
}

/// Cells (sorted by column) form one gap-free run.
fn consecutive(items: &[&Cell]) -> bool {
    match items.first() {
        None => false,
        Some(first) => {
            let mut next = first.column + first.width;
            for c in &items[1..] {
                if c.column != next {
                    return false;
                }
                next = c.column + c.width;
            }
            true
        }
    }
}

/// `Some((columns, rows))` if the cells form a full rectangle.
fn is_rectangular(cells: &[&Cell]) -> Option<(i32, i32)> {
    if cells.is_empty() {
        return None;
    }
    let rows = by_row(cells);
    let keys: Vec<i32> = rows.keys().copied().collect();
    if keys.windows(2).any(|w| w[1] - w[0] != 1) {
        return None;
    }
    let widths: BTreeSet<i32> = rows
        .values()
        .map(|r| r.iter().map(|c| c.width).sum::<i32>())
        .collect();
    if widths.len() != 1 {
        return None;
    }
    if !rows.values().all(|r| consecutive(r)) {
        return None;
    }
    Some((*widths.iter().next().unwrap(), rows.len() as i32))
}

/// Which unit columns (relative to `left`) the items of one row cover.
fn coverage(items: &[&Cell], left: i32) -> Vec<bool> {
    let end = items.iter().map(|c| c.column + c.width).max().unwrap_or(left);
    let mut v = vec![false; (end - left).max(0) as usize];
    for c in items {
        for i in c.column..c.column + c.width {
            v[(i - left) as usize] = true;
        }
    }
    v
}

/// Pairs rows of the copy with rows of the paste selection (in sorted order)
/// and checks that they cover exactly the same unit cells.
/// Returns (column offset, copy row -> paste row, cells to overwrite).
fn match_selections(
    copy: &[&Cell],
    paste: &[&Cell],
) -> Option<(i32, BTreeMap<i32, i32>, Vec<CellKey>)> {
    let copy_rows = by_row(copy);
    let paste_rows = by_row(paste);
    if copy_rows.len() != paste_rows.len() {
        return None;
    }
    let left_copy = copy.iter().map(|c| c.column).min()?;
    let left_paste = paste.iter().map(|c| c.column).min()?;

    let mut row_map = BTreeMap::new();
    let mut dead = Vec::new();
    for ((cr, citems), (pr, pitems)) in copy_rows.iter().zip(paste_rows.iter()) {
        if coverage(citems, left_copy) != coverage(pitems, left_paste) {
            return None;
        }
        row_map.insert(*cr, *pr);
        dead.extend(pitems.iter().map(|c| c.key()));
    }
    Some((left_paste - left_copy, row_map, dead))
}

fn set_label(map: &mut BTreeMap<i32, String>, key: i32, text: &str) {
    if text.trim().is_empty() {
        map.remove(&key);
    } else {
        map.insert(key, text.to_string());
    }
}

fn shift_keys(map: &BTreeMap<i32, String>, pivot: i32, shift: i32) -> BTreeMap<i32, String> {
    map.iter()
        .map(|(&k, v)| (if k >= pivot { k + shift } else { k }, v.clone()))
        .collect()
}

/// Row repeat bookkeeping after `num` rows were inserted at `pivot`
/// (port of `RowRepeatTracker.shift_and_expand_repeats`).
fn adjust_repeat_for_insert(rep: &mut RowRepeat, pivot: i32, num: i32) {
    let (Some(&first), Some(&last)) = (rep.rows.first(), rep.rows.last()) else { return };
    if pivot > first && pivot <= last {
        rep.rows = (first..=last + num).collect();
    } else if pivot <= first {
        for r in rep.rows.iter_mut() {
            *r += num;
        }
    }
}

// ---------------------------------------------------------------------------
// Chart
// ---------------------------------------------------------------------------

impl Chart {
    /// A fresh chart filled with the default symbol (new project).
    pub fn new(num_rows: i32, num_columns: i32, default_symbol: SymbolRef, default_color: Color) -> Chart {
        let mut chart = Chart {
            num_rows: num_rows.max(1),
            num_columns: num_columns.max(1),
            cells: BTreeMap::new(),
            selection: BTreeSet::new(),
            row_labels: BTreeMap::new(),
            column_labels: BTreeMap::new(),
            row_repeats: Vec::new(),
            default_symbol: Arc::new(default_symbol),
            default_color,
        };
        for row in 0..chart.num_rows {
            for col in 0..chart.num_columns {
                let cell = chart.new_cell(row, col);
                chart.cells.insert(cell.key(), cell);
            }
        }
        chart
    }

    /// Builds the editable chart from a loaded project file.
    pub fn from_project(
        p: &Project,
        default_symbol: SymbolRef,
        default_color: Color,
    ) -> Result<Chart, EditError> {
        if p.grid_items.is_empty() {
            return Err(EditError::InconsistentGrid("no cells".into()));
        }
        let mut interned: BTreeMap<(String, String), Arc<SymbolRef>> = BTreeMap::new();
        let mut cells = BTreeMap::new();
        let mut num_rows = 0;
        let mut num_columns = 0;
        for it in &p.grid_items {
            if it.row < 0 || it.column < 0 || it.width < 1 {
                return Err(EditError::InconsistentGrid(format!(
                    "bad cell at row {}, column {}",
                    it.row, it.column
                )));
            }
            let symbol = interned
                .entry((it.category.clone(), it.name.clone()))
                .or_insert_with(|| {
                    Arc::new(SymbolRef { category: it.category.clone(), name: it.name.clone() })
                })
                .clone();
            let cell = Cell {
                column: it.column,
                row: it.row,
                width: it.width,
                symbol,
                color: it.color,
                hidden: it.is_hidden,
            };
            if cells.insert(cell.key(), cell).is_some() {
                return Err(EditError::InconsistentGrid(format!(
                    "two cells at row {}, column {}",
                    it.row, it.column
                )));
            }
            num_rows = num_rows.max(it.row + 1);
            num_columns = num_columns.max(it.column + it.width);
        }
        let chart = Chart {
            num_rows,
            num_columns,
            cells,
            selection: BTreeSet::new(),
            row_labels: p.row_labels.clone(),
            column_labels: p.column_labels.clone(),
            row_repeats: p.row_repeats.clone(),
            default_symbol: Arc::new(default_symbol),
            default_color,
        };
        chart.validate()?;
        Ok(chart)
    }

    /// Copies grid, labels and row repeats into `p` (legends, pattern
    /// repeats and settings are left alone).
    pub fn write_to_project(&self, p: &mut Project) {
        p.grid_items = self
            .cells
            .values()
            .map(|c| GridItem {
                category: c.symbol.category.clone(),
                name: c.symbol.name.clone(),
                column: c.column,
                row: c.row,
                width: c.width,
                height: 1,
                color: c.color,
                is_hidden: c.hidden,
            })
            .collect();
        p.row_labels = self.row_labels.clone();
        p.column_labels = self.column_labels.clone();
        p.row_repeats = self.row_repeats.clone();
    }

    /// Checks the tiling invariant: every row is covered exactly once.
    pub fn validate(&self) -> Result<(), EditError> {
        let bad = |msg: String| Err(EditError::InconsistentGrid(msg));
        let mut next = vec![0i32; self.num_rows.max(0) as usize];
        for (key, c) in &self.cells {
            if *key != c.key() || c.width < 1 {
                return bad(format!("cell {:?} is stored under the wrong key", key));
            }
            if c.row < 0 || c.row >= self.num_rows {
                return bad(format!("cell in row {} outside the chart", c.row));
            }
            if c.column != next[c.row as usize] {
                return bad(format!("gap or overlap in row {} at column {}", c.row, c.column));
            }
            next[c.row as usize] += c.width;
        }
        for (row, n) in next.iter().enumerate() {
            if *n != self.num_columns {
                return bad(format!("row {row} is {n} cells wide, expected {}", self.num_columns));
            }
        }
        Ok(())
    }

    fn new_cell(&self, row: i32, column: i32) -> Cell {
        Cell {
            column,
            row,
            width: 1,
            symbol: self.default_symbol.clone(),
            color: self.default_color,
            hidden: false,
        }
    }

    // ---- queries ----------------------------------------------------------

    pub fn num_rows(&self) -> i32 {
        self.num_rows
    }

    pub fn num_columns(&self) -> i32 {
        self.num_columns
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// All cells in row-major order.
    pub fn cells(&self) -> impl Iterator<Item = &Cell> + '_ {
        self.cells.values()
    }

    pub fn cell(&self, key: CellKey) -> Option<&Cell> {
        self.cells.get(&key)
    }

    /// The cell covering the unit cell at (row, column).
    pub fn cell_at(&self, row: i32, column: i32) -> Option<&Cell> {
        self.cells
            .range((row, i32::MIN)..=(row, column))
            .next_back()
            .map(|(_, c)| c)
            .filter(|c| column < c.column + c.width)
    }

    pub fn row_repeats(&self) -> &[RowRepeat] {
        &self.row_repeats
    }

    pub fn selected_cells(&self) -> impl Iterator<Item = &Cell> + '_ {
        self.selection.iter().filter_map(move |k| self.cells.get(k))
    }

    pub fn selection_keys(&self) -> Vec<CellKey> {
        self.selection.iter().copied().collect()
    }

    pub fn selection_len(&self) -> usize {
        self.selection.len()
    }

    pub fn is_selected(&self, key: CellKey) -> bool {
        self.selection.contains(&key)
    }

    /// Rows containing at least one selected cell.
    pub fn marked_rows(&self) -> Vec<i32> {
        let rows: BTreeSet<i32> = self.selection.iter().map(|k| k.0).collect();
        rows.into_iter().collect()
    }

    /// Columns covered by at least one selected cell.
    pub fn marked_columns(&self) -> Vec<i32> {
        let mut cols = BTreeSet::new();
        for c in self.selected_cells() {
            cols.extend(c.column..c.column + c.width);
        }
        cols.into_iter().collect()
    }

    /// Keys of all cells touching the given block of unit cells (what a
    /// rubber-band selection picks up).
    pub fn keys_in_rect(&self, column: i32, row: i32, num_columns: i32, num_rows: i32) -> Vec<CellKey> {
        let mut out = BTreeSet::new();
        for r in row..row + num_rows {
            for col in column..column + num_columns {
                if let Some(c) = self.cell_at(r, col) {
                    out.insert(c.key());
                }
            }
        }
        out.into_iter().collect()
    }

    /// All cells of a row (click on a row label).
    pub fn row_keys(&self, row: i32) -> Vec<CellKey> {
        self.cells.range((row, i32::MIN)..=(row, i32::MAX)).map(|(k, _)| *k).collect()
    }

    /// The cells crossing a column, one per row (click on a column label).
    pub fn column_keys(&self, column: i32) -> Vec<CellKey> {
        (0..self.num_rows)
            .filter_map(|r| self.cell_at(r, column).map(|c| c.key()))
            .collect()
    }

    // ---- selection, painting ---------------------------------------------

    /// Toggles the given cells in the selection (hidden cells cannot be
    /// selected). With an active symbol the selection is then painted if the
    /// symbol fits, exactly like the original: a single click paints a
    /// 1-wide symbol, a 4-wide cable needs four selected cells.
    pub fn select_cells(&mut self, keys: &[CellKey], active: Option<&ActiveSymbol>) {
        let wanted: BTreeSet<CellKey> = keys.iter().copied().collect();
        let mut select = Vec::new();
        let mut unselect = Vec::new();
        for k in wanted {
            if let Some(c) = self.cells.get(&k) {
                if self.selection.contains(&k) {
                    unselect.push(k);
                } else if !c.hidden {
                    select.push(k);
                }
            }
        }
        for k in unselect {
            self.selection.remove(&k);
        }
        for k in select {
            self.selection.insert(k);
        }
        if let Some(a) = active {
            let _ = self.paint(a);
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    /// Splits the selection into chunks that can each be filled with
    /// symbols of `width` (port of `chunkify_cell_arrangement`). Empty if the
    /// symbol does not fit.
    fn chunkify(&self, width: i32) -> Vec<Vec<CellKey>> {
        let cells: Vec<&Cell> = self.selected_cells().collect();
        let total: i32 = cells.iter().map(|c| c.width).sum();
        if width < 1 || total % width != 0 {
            return Vec::new();
        }
        let rows = by_row(&cells);
        for row in rows.values() {
            if row.iter().map(|c| c.width).sum::<i32>() % width != 0 {
                return Vec::new();
            }
        }
        let mut out = Vec::new();
        for row in rows.values() {
            let mut chunks: Vec<Vec<&Cell>> = Vec::new();
            let mut chunk: Vec<&Cell> = Vec::new();
            let mut length = 0;
            for c in row {
                chunk.push(*c);
                length += c.width;
                if length % width == 0 {
                    chunks.push(std::mem::take(&mut chunk));
                    length = 0;
                }
            }
            if !chunks.iter().all(|ch| consecutive(ch)) {
                return Vec::new();
            }
            for ch in chunks {
                out.push(ch.iter().map(|c| c.key()).collect());
            }
        }
        out
    }

    /// Fills the selection with the active symbol if it fits; clears the
    /// selection on success, leaves it untouched otherwise.
    pub fn paint(&mut self, active: &ActiveSymbol) -> Result<(), EditError> {
        let chunks = self.chunkify(active.width);
        if chunks.is_empty() {
            return Err(EditError::SymbolDoesNotFit);
        }
        let symbol = Arc::new(active.symbol.clone());
        for chunk in chunks {
            let first = self.cells[&chunk[0]].clone();
            let row = first.row;
            let mut column = first.column;
            let mut total = 0;
            for k in &chunk {
                total += self.cells.remove(k).map_or(0, |c| c.width);
            }
            for _ in 0..total / active.width {
                let cell = Cell {
                    column,
                    row,
                    width: active.width,
                    symbol: symbol.clone(),
                    color: active.color,
                    hidden: false,
                };
                self.cells.insert(cell.key(), cell);
                column += active.width;
            }
        }
        self.selection.clear();
        Ok(())
    }

    /// Colours all selected cells (cells with the "nostitch" symbol keep
    /// their colour, as in the original) and clears the selection.
    pub fn apply_color(&mut self, color: Color) -> Result<(), EditError> {
        if self.selection.is_empty() {
            return Err(EditError::NoSelection);
        }
        for k in self.selection.clone() {
            if let Some(c) = self.cells.get_mut(&k) {
                if c.symbol.name != "nostitch" {
                    c.color = color;
                }
            }
        }
        self.selection.clear();
        Ok(())
    }

    /// Hides or unhides cells (hidden cells give non-rectangular patterns).
    pub fn set_hidden(&mut self, keys: &[CellKey], hidden: bool) {
        for k in keys {
            if let Some(c) = self.cells.get_mut(k) {
                c.hidden = hidden;
                if hidden {
                    self.selection.remove(k);
                }
            }
        }
    }

    // ---- copy / paste -----------------------------------------------------

    /// Returns the selected cells for the clipboard and clears the selection.
    pub fn copy_selection(&mut self) -> Vec<Cell> {
        let clip: Vec<Cell> = self.selected_cells().cloned().collect();
        self.selection.clear();
        clip
    }

    fn insert_translated(&mut self, clipboard: &[Cell], from: (i32, i32), to: (i32, i32)) {
        // from/to are (column, row)
        for c in clipboard {
            let cell = Cell {
                column: c.column - from.0 + to.0,
                row: c.row - from.1 + to.1,
                width: c.width,
                symbol: c.symbol.clone(),
                color: c.color,
                hidden: false,
            };
            self.cells.insert(cell.key(), cell);
        }
    }

    /// Pastes `clipboard`. In order of precedence (as in the original):
    ///
    /// 1. selection and clipboard both rectangular: tile the clipboard over
    ///    the selection (its size must be a multiple of the clipboard's);
    /// 2. neither rectangular and something selected: the selection must
    ///    have exactly the shape of the clipboard;
    /// 3. `at = Some((column, row))`: paste with the clipboard's upper left
    ///    corner at that cell, if no symbol would be cut.
    pub fn paste(&mut self, clipboard: &[Cell], at: Option<(i32, i32)>) -> Result<(), EditError> {
        if clipboard.is_empty() {
            return Err(EditError::NoClipboard);
        }
        let clip_refs: Vec<&Cell> = clipboard.iter().collect();
        let sel: Vec<Cell> = self.selected_cells().cloned().collect();
        let sel_refs: Vec<&Cell> = sel.iter().collect();
        let sel_rect = is_rectangular(&sel_refs);
        let clip_rect = is_rectangular(&clip_refs);

        // case 1
        if let (Some((pc, pr)), Some((cc, cr))) = (sel_rect, clip_rect) {
            if pc % cc != 0 || pr % cr != 0 {
                return Err(EditError::PasteGeometry);
            }
            let (p_row, p_col) = upper_left(&sel_refs).unwrap();
            let (c_row, c_col) = upper_left(&clip_refs).unwrap();
            self.selection.clear();
            for row_tile in 0..pr / cr {
                for col_tile in 0..pc / cc {
                    let col_id = p_col + col_tile * cc;
                    let row_id = p_row + row_tile * cr;
                    for k in self.keys_in_rect(col_id, row_id, cc, cr) {
                        self.cells.remove(&k);
                    }
                    self.insert_translated(clipboard, (c_col, c_row), (col_id, row_id));
                }
            }
            return Ok(());
        }

        // case 2
        if sel_rect.is_none() && clip_rect.is_none() && !sel.is_empty() {
            let (offset, row_map, dead) =
                match_selections(&clip_refs, &sel_refs).ok_or(EditError::PasteMismatch)?;
            self.selection.clear();
            for k in dead {
                self.cells.remove(&k);
            }
            for c in clipboard {
                let cell = Cell {
                    column: c.column + offset,
                    row: row_map[&c.row],
                    width: c.width,
                    symbol: c.symbol.clone(),
                    color: c.color,
                    hidden: false,
                };
                self.cells.insert(cell.key(), cell);
            }
            return Ok(());
        }

        // case 3
        if let Some((col, row)) = at {
            let (min_row, min_col) = upper_left(&clip_refs).unwrap();
            let clip_rows = by_row(&clip_refs);
            let mut dead: BTreeSet<CellKey> = BTreeSet::new();
            for (r, items) in &clip_rows {
                let act_row = row + (r - min_row);
                if act_row < 0 || act_row >= self.num_rows {
                    return Err(EditError::PasteBlocked);
                }
                let mut covered: BTreeMap<CellKey, (i32, i32)> = BTreeMap::new();
                for item in items {
                    let start = col + (item.column - min_col);
                    for k in 0..item.width {
                        let act_col = start + k;
                        if act_col < 0 || act_col >= self.num_columns {
                            return Err(EditError::PasteBlocked);
                        }
                        let target = self.cell_at(act_row, act_col).ok_or(EditError::PasteBlocked)?;
                        dead.insert(target.key());
                        let e = covered.entry(target.key()).or_insert((target.width, 0));
                        e.1 += 1;
                    }
                }
                // every touched cell must be covered completely
                if covered.values().any(|(w, n)| w != n) {
                    return Err(EditError::PasteBlocked);
                }
            }
            self.selection.clear();
            for k in dead {
                self.cells.remove(&k);
            }
            self.insert_translated(clipboard, (min_col, min_row), (col, row));
            return Ok(());
        }

        Err(EditError::NoPasteTarget)
    }

    // ---- rows -------------------------------------------------------------

    /// Inserts `num` default rows above/below the single marked row.
    pub fn insert_rows(&mut self, num: i32, location: RowLocation) -> Result<(), EditError> {
        if num < 1 {
            return Err(EditError::BadCount);
        }
        let rows = self.marked_rows();
        if rows.len() != 1 {
            return Err(EditError::NeedSingleRow);
        }
        let pivot = match location {
            RowLocation::Above => rows[0],
            RowLocation::Below => rows[0] + 1,
        };
        self.selection.clear();

        let old = std::mem::take(&mut self.cells);
        for (_, mut c) in old {
            if c.row >= pivot {
                c.row += num;
            }
            self.cells.insert(c.key(), c);
        }
        for r in pivot..pivot + num {
            for col in 0..self.num_columns {
                let cell = self.new_cell(r, col);
                self.cells.insert(cell.key(), cell);
            }
        }
        self.row_labels = shift_keys(&self.row_labels, pivot, num);
        for rep in self.row_repeats.iter_mut() {
            adjust_repeat_for_insert(rep, pivot, num);
        }
        self.num_rows += num;
        Ok(())
    }

    /// Deletes all marked rows.
    pub fn delete_rows(&mut self) -> Result<(), EditError> {
        let dead = self.marked_rows();
        if dead.is_empty() {
            return Err(EditError::NoSelection);
        }
        if dead.len() as i32 >= self.num_rows {
            return Err(EditError::CannotDeleteEverything);
        }
        self.selection.clear();
        let dead_set: BTreeSet<i32> = dead.iter().copied().collect();
        let below = |r: i32| dead.iter().filter(|&&d| d < r).count() as i32;

        let old = std::mem::take(&mut self.cells);
        for (_, mut c) in old {
            if dead_set.contains(&c.row) {
                continue;
            }
            c.row -= below(c.row);
            self.cells.insert(c.key(), c);
        }

        let old_labels = std::mem::take(&mut self.row_labels);
        for (k, v) in old_labels {
            if !dead_set.contains(&k) {
                self.row_labels.insert(k - below(k), v);
            }
        }

        let old_repeats = std::mem::take(&mut self.row_repeats);
        for rep in old_repeats {
            let rows: Vec<i32> = rep
                .rows
                .iter()
                .filter(|r| !dead_set.contains(r))
                .map(|&r| r - below(r))
                .collect();
            if !rows.is_empty() {
                self.row_repeats.push(RowRepeat { multiplicity: rep.multiplicity, rows });
            }
        }
        self.num_rows -= dead.len() as i32;
        Ok(())
    }

    // ---- columns ----------------------------------------------------------

    /// Columns can only be inserted where every row has a cell starting
    /// exactly at the insertion point (or at the chart's outer edge).
    pub fn can_insert_columns(&self, pivot: i32, location: ColumnLocation) -> bool {
        let (shift, external) = match location {
            ColumnLocation::LeftOf => (0, pivot == 0),
            ColumnLocation::RightOf => (1, pivot == self.num_columns - 1),
        };
        if external {
            return true;
        }
        let at = pivot + shift;
        (0..self.num_rows).all(|r| self.cell_at(r, at).map_or(false, |c| c.column == at))
    }

    pub fn insert_columns(&mut self, num: i32, location: ColumnLocation) -> Result<(), EditError> {
        if num < 1 {
            return Err(EditError::BadCount);
        }
        let cols = self.marked_columns();
        if cols.len() != 1 {
            return Err(EditError::NeedSingleColumn);
        }
        if !self.can_insert_columns(cols[0], location) {
            return Err(EditError::CannotInsertColumns);
        }
        let pivot = match location {
            ColumnLocation::LeftOf => cols[0],
            ColumnLocation::RightOf => cols[0] + 1,
        };
        self.selection.clear();

        let old = std::mem::take(&mut self.cells);
        for (_, mut c) in old {
            if c.column >= pivot {
                c.column += num;
            }
            self.cells.insert(c.key(), c);
        }
        for col in pivot..pivot + num {
            for r in 0..self.num_rows {
                let cell = self.new_cell(r, col);
                self.cells.insert(cell.key(), cell);
            }
        }
        self.column_labels = shift_keys(&self.column_labels, pivot, num);
        self.num_columns += num;
        Ok(())
    }

    /// A set of columns can be deleted if no symbol would be cut in two.
    pub fn can_delete_columns(&self, dead: &BTreeSet<i32>) -> bool {
        self.cells.values().all(|c| {
            let span = c.column..c.column + c.width;
            let touches = span.clone().any(|x| dead.contains(&x));
            !touches || span.clone().all(|x| dead.contains(&x))
        })
    }

    pub fn delete_columns(&mut self) -> Result<(), EditError> {
        let dead = self.marked_columns();
        if dead.is_empty() {
            return Err(EditError::NoSelection);
        }
        if dead.len() as i32 >= self.num_columns {
            return Err(EditError::CannotDeleteEverything);
        }
        let dead_set: BTreeSet<i32> = dead.iter().copied().collect();
        if !self.can_delete_columns(&dead_set) {
            return Err(EditError::CannotDeleteColumns);
        }
        self.selection.clear();
        let below = |c: i32| dead.iter().filter(|&&d| d < c).count() as i32;

        let old = std::mem::take(&mut self.cells);
        for (_, mut c) in old {
            if dead_set.contains(&c.column) {
                continue; // whole symbol lies inside the dead columns
            }
            c.column -= below(c.column);
            self.cells.insert(c.key(), c);
        }
        let old_labels = std::mem::take(&mut self.column_labels);
        for (k, v) in old_labels {
            if !dead_set.contains(&k) {
                self.column_labels.insert(k - below(k), v);
            }
        }
        self.num_columns -= dead.len() as i32;
        Ok(())
    }

    // ---- selecting similar cells, label editing, comparison ---------------

    /// Everything that is saved with the pattern; the selection is not.
    pub fn same_content(&self, other: &Chart) -> bool {
        self.num_rows == other.num_rows
            && self.num_columns == other.num_columns
            && self.cells == other.cells
            && self.row_labels == other.row_labels
            && self.column_labels == other.column_labels
            && self.row_repeats == other.row_repeats
    }

    /// Replaces the selection by all visible cells with the same colour
    /// (`by_color`) or the same symbol as the cell `key`.
    pub fn select_matching(&mut self, key: CellKey, by_color: bool) {
        let Some(reference) = self.cells.get(&key) else { return };
        let matches: BTreeSet<CellKey> = self
            .cells
            .values()
            .filter(|c| !c.hidden)
            .filter(|c| {
                if by_color {
                    c.color == reference.color
                } else {
                    c.symbol.name == reference.symbol.name
                }
            })
            .map(|c| c.key())
            .collect();
        if !matches.is_empty() {
            self.selection = matches;
        }
    }

    /// Sets the text of a row label; blank text removes it (the number is
    /// shown again).
    pub fn set_row_label(&mut self, row: i32, text: &str) -> Result<(), EditError> {
        if row < 0 || row >= self.num_rows {
            return Err(EditError::InconsistentGrid(format!("there is no row {row}")));
        }
        set_label(&mut self.row_labels, row, text);
        Ok(())
    }

    pub fn set_column_label(&mut self, column: i32, text: &str) -> Result<(), EditError> {
        if column < 0 || column >= self.num_columns {
            return Err(EditError::InconsistentGrid(format!("there is no column {column}")));
        }
        set_label(&mut self.column_labels, column, text);
        Ok(())
    }

    // ---- row repeats ------------------------------------------------------

    /// A row repeat needs a contiguous block of marked rows, none of which
    /// is already part of a repeat.
    pub fn can_add_row_repeat(&self) -> bool {
        let rows = self.marked_rows();
        if rows.is_empty() {
            return false;
        }
        if rows.iter().any(|r| self.row_repeats.iter().any(|rep| rep.rows.contains(r))) {
            return false;
        }
        rows.windows(2).all(|w| w[1] - w[0] == 1)
    }

    pub fn add_row_repeat(&mut self, multiplicity: i32) -> Result<(), EditError> {
        if multiplicity < 1 {
            return Err(EditError::BadCount);
        }
        if !self.can_add_row_repeat() {
            return Err(EditError::CannotAddRowRepeat);
        }
        let rows = self.marked_rows();
        self.row_repeats.push(RowRepeat { multiplicity, rows });
        self.selection.clear();
        Ok(())
    }

    fn repeat_index_for_marked_rows(&self) -> Option<usize> {
        let rows = self.marked_rows();
        if rows.is_empty() {
            return None;
        }
        let mut matches = BTreeSet::new();
        for r in rows {
            let idx = self.row_repeats.iter().position(|rep| rep.rows.contains(&r))?;
            matches.insert(idx);
        }
        if matches.len() == 1 {
            matches.into_iter().next()
        } else {
            None
        }
    }

    /// Deletable if all marked rows belong to one and the same repeat.
    pub fn can_delete_row_repeat(&self) -> bool {
        self.repeat_index_for_marked_rows().is_some()
    }

    pub fn delete_row_repeat(&mut self) -> Result<(), EditError> {
        let idx = self.repeat_index_for_marked_rows().ok_or(EditError::CannotDeleteRowRepeat)?;
        self.row_repeats.remove(idx);
        self.selection.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sym(name: &str) -> SymbolRef {
        SymbolRef { category: "basic".into(), name: name.into() }
    }

    fn chart(rows: i32, cols: i32) -> Chart {
        Chart::new(rows, cols, sym("knit"), Color::WHITE)
    }

    fn active(name: &str, width: i32) -> ActiveSymbol {
        ActiveSymbol { symbol: sym(name), width, color: Color::WHITE }
    }

    fn name_at(c: &Chart, row: i32, col: i32) -> String {
        c.cell_at(row, col).unwrap().symbol.name.clone()
    }

    #[test]
    fn new_chart_is_valid() {
        let c = chart(3, 4);
        assert_eq!(c.cell_count(), 12);
        c.validate().unwrap();
        assert_eq!(c.cell_at(2, 3).unwrap().key(), (2, 3));
    }

    #[test]
    fn single_click_paints_a_one_wide_symbol() {
        let mut c = chart(3, 3);
        c.select_cells(&[(1, 2)], Some(&active("purl", 1)));
        assert_eq!(name_at(&c, 1, 2), "purl");
        assert_eq!(c.selection_len(), 0);
        c.validate().unwrap();
    }

    #[test]
    fn cable_needs_enough_selected_cells() {
        let mut c = chart(2, 4);
        let cable = active("cable", 2);
        c.select_cells(&[(0, 0)], Some(&cable));
        assert_eq!(name_at(&c, 0, 0), "knit"); // does not fit yet
        assert_eq!(c.selection_len(), 1);
        c.select_cells(&[(0, 1)], Some(&cable));
        assert_eq!(c.selection_len(), 0);
        assert_eq!(c.cell_at(0, 1).unwrap().column, 0);
        assert_eq!(c.cell_at(0, 1).unwrap().width, 2);
        assert_eq!(c.cell_count(), 8 - 1);
        c.validate().unwrap();
    }

    #[test]
    fn symbol_does_not_fit_odd_selection() {
        let mut c = chart(2, 4);
        c.select_cells(&[(0, 0), (0, 1)], None);
        assert_eq!(c.paint(&active("x", 3)), Err(EditError::SymbolDoesNotFit));
        assert_eq!(c.selection_len(), 2);
    }

    #[test]
    fn four_wide_symbol_over_two_rows_of_four() {
        let mut c = chart(2, 4);
        let keys: Vec<CellKey> = (0..2).flat_map(|r| (0..4).map(move |col| (r, col))).collect();
        c.select_cells(&keys, Some(&active("c4", 4)));
        assert_eq!(c.cell_count(), 2);
        c.validate().unwrap();
    }

    #[test]
    fn toggle_deselects_and_hidden_cells_cannot_be_selected() {
        let mut c = chart(2, 2);
        c.select_cells(&[(0, 0)], None);
        assert!(c.is_selected((0, 0)));
        c.select_cells(&[(0, 0)], None);
        assert!(!c.is_selected((0, 0)));
        c.set_hidden(&[(1, 1)], true);
        c.select_cells(&[(1, 1)], None);
        assert_eq!(c.selection_len(), 0);
    }

    #[test]
    fn apply_color_skips_nostitch() {
        let mut c = chart(1, 3);
        c.select_cells(&[(0, 0)], Some(&active("nostitch", 1)));
        c.select_cells(&[(0, 1)], Some(&active("purl", 1)));
        c.select_cells(&[(0, 0), (0, 1)], None);
        let red = Color::rgb(255, 0, 0);
        c.apply_color(red).unwrap();
        assert_eq!(c.cell_at(0, 0).unwrap().color, Color::WHITE);
        assert_eq!(c.cell_at(0, 1).unwrap().color, red);
        assert_eq!(c.apply_color(red), Err(EditError::NoSelection));
    }

    #[test]
    fn insert_rows_above_shifts_content_labels_and_repeats() {
        let mut c = chart(3, 3);
        c.select_cells(&[(1, 0)], Some(&active("purl", 1)));
        c.row_labels.insert(1, "one".into());
        c.row_labels.insert(0, "zero".into());
        c.select_cells(&[(1, 1)], None);
        c.insert_rows(2, RowLocation::Above).unwrap();
        c.validate().unwrap();
        assert_eq!(c.num_rows(), 5);
        assert_eq!(name_at(&c, 3, 0), "purl");
        assert_eq!(name_at(&c, 1, 0), "knit");
        assert_eq!(c.row_labels.get(&3).map(String::as_str), Some("one"));
        assert_eq!(c.row_labels.get(&0).map(String::as_str), Some("zero"));
    }

    #[test]
    fn insert_rows_needs_exactly_one_marked_row() {
        let mut c = chart(3, 3);
        c.select_cells(&[(0, 0), (1, 0)], None);
        assert_eq!(c.insert_rows(1, RowLocation::Above), Err(EditError::NeedSingleRow));
        c.clear_selection();
        c.select_cells(&[(0, 0)], None);
        assert_eq!(c.insert_rows(0, RowLocation::Above), Err(EditError::BadCount));
    }

    #[test]
    fn insert_inside_row_repeat_extends_it() {
        let mut c = chart(6, 2);
        c.select_cells(&[(1, 0), (2, 0), (3, 0)], None);
        c.add_row_repeat(3).unwrap();
        assert_eq!(c.row_repeats()[0].rows, vec![1, 2, 3]);
        c.select_cells(&[(2, 0)], None);
        c.insert_rows(2, RowLocation::Above).unwrap();
        assert_eq!(c.row_repeats()[0].rows, vec![1, 2, 3, 4, 5]);
        c.select_cells(&[(0, 0)], None);
        c.insert_rows(1, RowLocation::Above).unwrap(); // above the repeat: shifts it
        assert_eq!(c.row_repeats()[0].rows, vec![2, 3, 4, 5, 6]);
        c.select_cells(&[(7, 0)], None);
        c.insert_rows(1, RowLocation::Below).unwrap(); // below the repeat: untouched
        assert_eq!(c.row_repeats()[0].rows, vec![2, 3, 4, 5, 6]);
        c.validate().unwrap();
    }

    #[test]
    fn delete_rows_keeps_repeats_and_labels_consistent() {
        let mut c = chart(6, 2);
        c.select_cells(&[(1, 0), (2, 0), (3, 0)], None);
        c.add_row_repeat(2).unwrap();
        c.row_labels.insert(4, "four".into());
        c.row_labels.insert(2, "two".into());
        c.select_cells(&[(2, 1)], None);
        c.delete_rows().unwrap();
        c.validate().unwrap();
        assert_eq!(c.num_rows(), 5);
        assert_eq!(c.row_repeats()[0].rows, vec![1, 2]);
        assert_eq!(c.row_labels.get(&3).map(String::as_str), Some("four"));
        assert!(!c.row_labels.values().any(|v| v == "two"));
    }

    #[test]
    fn deleting_all_rows_of_a_repeat_removes_it() {
        let mut c = chart(4, 2);
        c.select_cells(&[(1, 0), (2, 0)], None);
        c.add_row_repeat(2).unwrap();
        c.select_cells(&[(1, 0), (2, 0)], None);
        c.delete_rows().unwrap();
        assert!(c.row_repeats().is_empty());
        c.validate().unwrap();
    }

    #[test]
    fn cannot_delete_every_row() {
        let mut c = chart(2, 2);
        c.select_cells(&[(0, 0), (1, 0)], None);
        assert_eq!(c.delete_rows(), Err(EditError::CannotDeleteEverything));
    }

    #[test]
    fn row_repeat_rules() {
        let mut c = chart(6, 2);
        c.select_cells(&[(0, 0), (2, 0)], None); // not contiguous
        assert_eq!(c.add_row_repeat(2), Err(EditError::CannotAddRowRepeat));
        c.clear_selection();
        c.select_cells(&[(1, 0), (2, 0)], None);
        c.add_row_repeat(2).unwrap();
        c.select_cells(&[(2, 0), (3, 0)], None); // overlaps existing repeat
        assert!(!c.can_add_row_repeat());
        assert!(!c.can_delete_row_repeat()); // row 3 is in no repeat
        c.clear_selection();
        c.select_cells(&[(2, 0)], None);
        assert!(c.can_delete_row_repeat());
        c.delete_row_repeat().unwrap();
        assert!(c.row_repeats().is_empty());
    }

    fn chart_with_cable() -> Chart {
        // row 0 has a width-2 cable on columns 1-2
        let mut c = chart(2, 4);
        let cable = active("cable", 2);
        c.select_cells(&[(0, 1)], Some(&cable));
        c.select_cells(&[(0, 2)], Some(&cable));
        c
    }

    #[test]
    fn insert_columns_is_blocked_inside_a_cable() {
        let mut c = chart_with_cable();
        c.select_cells(&[(1, 2)], None); // column 2 is the cable's right half
        assert_eq!(c.insert_columns(1, ColumnLocation::LeftOf), Err(EditError::CannotInsertColumns));
        c.insert_columns(1, ColumnLocation::RightOf).unwrap();
        assert_eq!(c.num_columns(), 5);
        c.validate().unwrap();
    }

    #[test]
    fn insert_columns_at_edges_and_label_shift() {
        let mut c = chart(2, 3);
        c.column_labels.insert(0, "a".into());
        c.column_labels.insert(2, "c".into());
        c.select_cells(&[(0, 0)], None);
        c.insert_columns(2, ColumnLocation::LeftOf).unwrap();
        c.validate().unwrap();
        assert_eq!(c.num_columns(), 5);
        assert_eq!(c.column_labels.get(&2).map(String::as_str), Some("a"));
        assert_eq!(c.column_labels.get(&4).map(String::as_str), Some("c"));
    }

    #[test]
    fn delete_columns_refuses_to_cut_a_cable() {
        let mut c = chart_with_cable();
        c.select_cells(&[(1, 2)], None);
        assert_eq!(c.delete_columns(), Err(EditError::CannotDeleteColumns));
        c.clear_selection();
        c.select_cells(&[(0, 1)], None); // selecting the cable marks columns 1 and 2
        c.delete_columns().unwrap();
        assert_eq!(c.num_columns(), 2);
        c.validate().unwrap();
    }

    #[test]
    fn paste_tiles_clipboard_over_rectangular_selection() {
        let mut c = chart(3, 4);
        c.select_cells(&[(0, 0)], Some(&active("purl", 1)));
        c.select_cells(&[(0, 0)], None);
        let clip = c.copy_selection();
        let keys = c.row_keys(1);
        c.select_cells(&keys, None);
        c.paste(&clip, None).unwrap();
        for col in 0..4 {
            assert_eq!(name_at(&c, 1, col), "purl");
        }
        assert_eq!(name_at(&c, 2, 0), "knit");
        c.validate().unwrap();
    }

    #[test]
    fn paste_rejects_selection_that_is_not_a_multiple() {
        let mut c = chart(3, 4);
        c.select_cells(&[(0, 0), (0, 1)], None);
        let clip = c.copy_selection(); // 2x1
        c.select_cells(&[(1, 0), (1, 1), (1, 2)], None); // 3x1
        assert_eq!(c.paste(&clip, None), Err(EditError::PasteGeometry));
    }

    #[test]
    fn paste_at_clicked_cell_and_blocked_outside() {
        let mut c = chart(3, 4);
        c.select_cells(&[(0, 0)], Some(&active("purl", 1)));
        c.select_cells(&[(0, 1)], Some(&active("purl", 1)));
        c.select_cells(&[(0, 0), (0, 1)], None);
        let clip = c.copy_selection();
        c.paste(&clip, Some((2, 2))).unwrap();
        assert_eq!(name_at(&c, 2, 2), "purl");
        assert_eq!(name_at(&c, 2, 3), "purl");
        assert_eq!(c.paste(&clip, Some((3, 1))), Err(EditError::PasteBlocked));
        assert_eq!(c.paste(&clip, None), Err(EditError::NoPasteTarget));
        c.validate().unwrap();
    }

    #[test]
    fn paste_at_refuses_to_cut_a_cable_but_covers_it_completely() {
        let mut c = chart_with_cable();
        c.select_cells(&[(1, 0)], Some(&active("purl", 1)));
        c.select_cells(&[(1, 0)], None);
        let clip = c.copy_selection(); // a single 1-wide cell
        // pasting one cell onto half of the cable would cut it
        assert_eq!(c.paste(&clip, Some((1, 0))), Err(EditError::PasteBlocked));
        c.validate().unwrap();
    }

    #[test]
    fn paste_l_shaped_selection_into_l_shape() {
        let mut c = chart(4, 4);
        for k in [(0, 0), (1, 0), (1, 1)] {
            c.select_cells(&[k], Some(&active("purl", 1)));
        }
        c.select_cells(&[(0, 0), (1, 0), (1, 1)], None);
        let clip = c.copy_selection();
        c.select_cells(&[(2, 2), (3, 2), (3, 3)], None);
        c.paste(&clip, None).unwrap();
        assert_eq!(name_at(&c, 2, 2), "purl");
        assert_eq!(name_at(&c, 3, 2), "purl");
        assert_eq!(name_at(&c, 3, 3), "purl");
        assert_eq!(name_at(&c, 2, 3), "knit");
        c.validate().unwrap();
    }

    #[test]
    fn paste_into_differently_shaped_selection_is_rejected() {
        let mut c = chart(4, 4);
        c.select_cells(&[(0, 0), (1, 0), (1, 1)], None);
        let clip = c.copy_selection();
        c.select_cells(&[(2, 2), (2, 3), (3, 3)], None); // other L orientation
        assert_eq!(c.paste(&clip, None), Err(EditError::PasteMismatch));
    }

    #[test]
    fn project_roundtrip() {
        let mut c = chart_with_cable();
        c.row_labels.insert(1, "x".into());
        c.select_cells(&[(1, 0)], None);
        c.add_row_repeat(3).unwrap();
        let mut p = Project::default();
        c.write_to_project(&mut p);
        let back = Chart::from_project(&p, sym("knit"), Color::WHITE).unwrap();
        assert_eq!(c, back);
    }

    #[test]
    fn select_all_with_same_colour_or_symbol() {
        let mut c = chart(3, 3);
        let red = Color::rgb(255, 0, 0);
        c.select_cells(&[(0, 0), (2, 2)], None);
        c.apply_color(red).unwrap();
        c.select_cells(&[(1, 1)], Some(&active("purl", 1)));
        c.set_hidden(&[(0, 0)], true); // hidden cells are never selected
        c.select_matching((2, 2), true);
        assert_eq!(c.selection_keys(), vec![(2, 2)]);
        c.select_matching((1, 1), false); // replaces the selection
        assert_eq!(c.selection_keys(), vec![(1, 1)]);
        c.select_matching((0, 1), false); // all knit cells except the hidden one
        assert_eq!(c.selection_len(), 7);
        c.select_matching((9, 9), true); // no such cell: unchanged
        assert_eq!(c.selection_len(), 7);
    }

    #[test]
    fn same_content_ignores_the_selection_only() {
        let mut a = chart(2, 2);
        let b = a.clone();
        a.select_cells(&[(0, 0)], None);
        assert!(a.same_content(&b));
        assert_ne!(a, b);
        a.row_labels.insert(0, "x".into());
        assert!(!a.same_content(&b));
        let mut c = chart(2, 2);
        c.select_cells(&[(0, 0)], Some(&active("purl", 1)));
        assert!(!c.same_content(&b));
    }

    #[test]
    fn label_editing() {
        let mut c = chart(2, 3);
        c.set_row_label(1, "bottom").unwrap();
        c.set_column_label(2, "edge").unwrap();
        assert_eq!(c.row_labels.get(&1).map(String::as_str), Some("bottom"));
        assert_eq!(c.column_labels.get(&2).map(String::as_str), Some("edge"));
        c.set_row_label(1, "   ").unwrap(); // blank removes it
        assert!(c.row_labels.is_empty());
        assert!(c.set_row_label(2, "x").is_err());
        assert!(c.set_column_label(-1, "x").is_err());
    }

    #[test]
    fn from_project_rejects_broken_grids() {
        let mut p = Project::default();
        let mk = |col: i32, row: i32| GridItem {
            category: "basic".into(),
            name: "knit".into(),
            column: col,
            row,
            width: 1,
            height: 1,
            color: Color::WHITE,
            is_hidden: false,
        };
        p.grid_items = vec![mk(0, 0), mk(2, 0)]; // gap at column 1
        assert!(matches!(
            Chart::from_project(&p, sym("knit"), Color::WHITE),
            Err(EditError::InconsistentGrid(_))
        ));
        p.grid_items = vec![mk(0, 0), mk(0, 0)]; // duplicate
        assert!(Chart::from_project(&p, sym("knit"), Color::WHITE).is_err());
    }
}

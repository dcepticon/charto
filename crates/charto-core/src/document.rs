//! An editing session: the chart, the clipboard and unlimited undo/redo.
//!
//! As in the original, *every* canvas action is undoable, including
//! selecting cells. Each undo step is a snapshot of the chart taken before
//! the action (cells share their symbol data through `Arc`, so a snapshot
//! is cheap compared to the size of the grid). An action that fails leaves
//! the chart untouched and does not create an undo step; neither does an
//! action that changes nothing.
//!
//! The clipboard is not part of the undo history.

use crate::chart::{
    ActiveSymbol, Cell, CellKey, Chart, ColumnLocation, EditError, RowLocation,
};
use crate::qdatastream::Color;

pub struct Document {
    chart: Chart,
    clipboard: Vec<Cell>,
    undo_stack: Vec<(String, Chart)>,
    redo_stack: Vec<(String, Chart)>,
}

impl Document {
    pub fn new(chart: Chart) -> Self {
        Document { chart, clipboard: Vec::new(), undo_stack: Vec::new(), redo_stack: Vec::new() }
    }

    pub fn chart(&self) -> &Chart {
        &self.chart
    }

    pub fn clipboard(&self) -> &[Cell] {
        &self.clipboard
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn undo_text(&self) -> Option<&str> {
        self.undo_stack.last().map(|(t, _)| t.as_str())
    }

    pub fn redo_text(&self) -> Option<&str> {
        self.redo_stack.last().map(|(t, _)| t.as_str())
    }

    /// Forget all history (after loading or saving a project).
    pub fn clear_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    fn apply<F>(&mut self, text: &str, f: F) -> Result<(), EditError>
    where
        F: FnOnce(&mut Chart) -> Result<(), EditError>,
    {
        let before = self.chart.clone();
        match f(&mut self.chart) {
            Ok(()) => {
                if self.chart != before {
                    self.undo_stack.push((text.to_string(), before));
                    self.redo_stack.clear();
                }
                Ok(())
            }
            Err(e) => {
                self.chart = before;
                Err(e)
            }
        }
    }

    pub fn undo(&mut self) -> bool {
        match self.undo_stack.pop() {
            Some((text, before)) => {
                let current = std::mem::replace(&mut self.chart, before);
                self.redo_stack.push((text, current));
                true
            }
            None => false,
        }
    }

    pub fn redo(&mut self) -> bool {
        match self.redo_stack.pop() {
            Some((text, after)) => {
                let current = std::mem::replace(&mut self.chart, after);
                self.undo_stack.push((text, current));
                true
            }
            None => false,
        }
    }

    // ---- actions ----------------------------------------------------------

    /// Toggle cells in the selection and paint them if an active symbol fits.
    pub fn select(&mut self, keys: &[CellKey], active: Option<&ActiveSymbol>) {
        let _ = self.apply("paint cells", |c| {
            c.select_cells(keys, active);
            Ok(())
        });
    }

    pub fn clear_selection(&mut self) {
        let _ = self.apply("clear selection", |c| {
            c.clear_selection();
            Ok(())
        });
    }

    pub fn apply_color(&mut self, color: Color) -> Result<(), EditError> {
        self.apply("color selected cells", |c| c.apply_color(color))
    }

    pub fn hide_cells(&mut self, keys: &[CellKey]) {
        let _ = self.apply("hide cells", |c| {
            c.set_hidden(keys, true);
            Ok(())
        });
    }

    pub fn unhide_cells(&mut self, keys: &[CellKey]) {
        let _ = self.apply("unhide cells", |c| {
            c.set_hidden(keys, false);
            Ok(())
        });
    }

    /// Copy the selection to the clipboard (and clear the selection).
    pub fn copy(&mut self) {
        if self.chart.selection_len() == 0 {
            return;
        }
        let mut clip = Vec::new();
        let _ = self.apply("copy selection", |c| {
            clip = c.copy_selection();
            Ok(())
        });
        self.clipboard = clip;
    }

    /// `at` is `(column, row)` of a clicked cell, if any.
    pub fn paste(&mut self, at: Option<(i32, i32)>) -> Result<(), EditError> {
        let clip = self.clipboard.clone();
        self.apply("paste cells", |c| c.paste(&clip, at))
    }

    pub fn insert_rows(&mut self, num: i32, location: RowLocation) -> Result<(), EditError> {
        self.apply("insert rows", |c| c.insert_rows(num, location))
    }

    pub fn delete_rows(&mut self) -> Result<(), EditError> {
        self.apply("delete rows", |c| c.delete_rows())
    }

    pub fn insert_columns(&mut self, num: i32, location: ColumnLocation) -> Result<(), EditError> {
        self.apply("insert stitches", |c| c.insert_columns(num, location))
    }

    pub fn delete_columns(&mut self) -> Result<(), EditError> {
        self.apply("delete stitches", |c| c.delete_columns())
    }

    /// Select all cells with the same colour/symbol as the cell `key`.
    pub fn select_same(&mut self, key: CellKey, by_color: bool) {
        let _ = self.apply("select cells", |c| {
            c.select_matching(key, by_color);
            Ok(())
        });
    }

    pub fn set_row_label(&mut self, row: i32, text: &str) -> Result<(), EditError> {
        self.apply("edit row label", |c| c.set_row_label(row, text))
    }

    pub fn set_column_label(&mut self, column: i32, text: &str) -> Result<(), EditError> {
        self.apply("edit stitch label", |c| c.set_column_label(column, text))
    }

    pub fn add_row_repeat(&mut self, multiplicity: i32) -> Result<(), EditError> {
        self.apply("add row repeat", |c| c.add_row_repeat(multiplicity))
    }

    pub fn delete_row_repeat(&mut self) -> Result<(), EditError> {
        self.apply("delete row repeat", |c| c.delete_row_repeat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spf::SymbolRef;

    fn doc(rows: i32, cols: i32) -> Document {
        Document::new(Chart::new(
            rows,
            cols,
            SymbolRef { category: "basic".into(), name: "knit".into() },
            Color::WHITE,
        ))
    }

    fn purl() -> ActiveSymbol {
        ActiveSymbol {
            symbol: SymbolRef { category: "basic".into(), name: "purl".into() },
            width: 1,
            color: Color::WHITE,
        }
    }

    #[test]
    fn selecting_is_undoable() {
        let mut d = doc(2, 2);
        d.select(&[(0, 0)], None);
        assert_eq!(d.chart().selection_len(), 1);
        assert!(d.undo());
        assert_eq!(d.chart().selection_len(), 0);
        assert!(d.redo());
        assert_eq!(d.chart().selection_len(), 1);
        assert!(!d.redo());
    }

    #[test]
    fn painting_undo_redo_roundtrip() {
        let mut d = doc(2, 2);
        d.select(&[(1, 1)], Some(&purl()));
        assert_eq!(d.chart().cell_at(1, 1).unwrap().symbol.name, "purl");
        assert_eq!(d.undo_text(), Some("paint cells"));
        d.undo();
        assert_eq!(d.chart().cell_at(1, 1).unwrap().symbol.name, "knit");
        d.redo();
        assert_eq!(d.chart().cell_at(1, 1).unwrap().symbol.name, "purl");
    }

    #[test]
    fn failed_action_changes_nothing_and_adds_no_step() {
        let mut d = doc(2, 2);
        d.select(&[(0, 0), (1, 0)], None);
        let steps = d.undo_stack.len();
        assert_eq!(d.insert_rows(1, RowLocation::Above), Err(EditError::NeedSingleRow));
        assert_eq!(d.undo_stack.len(), steps);
        assert_eq!(d.chart().selection_len(), 2);
    }

    #[test]
    fn no_op_adds_no_step_and_new_action_clears_redo() {
        let mut d = doc(2, 2);
        d.clear_selection(); // nothing selected: no-op
        assert!(!d.can_undo());
        d.select(&[(0, 0)], None);
        d.undo();
        assert!(d.can_redo());
        d.select(&[(1, 1)], None);
        assert!(!d.can_redo());
    }

    #[test]
    fn label_edits_and_select_same_are_undoable() {
        let mut d = doc(3, 3);
        d.set_row_label(1, "mid").unwrap();
        assert_eq!(d.chart().row_labels.get(&1).map(String::as_str), Some("mid"));
        assert_eq!(d.undo_text(), Some("edit row label"));
        d.undo();
        assert!(d.chart().row_labels.is_empty());
        assert!(d.set_row_label(9, "x").is_err());

        d.select_same((0, 0), false);
        assert_eq!(d.chart().selection_len(), 9);
        d.undo();
        assert_eq!(d.chart().selection_len(), 0);
    }

    #[test]
    fn copy_paste_and_structure_edits_undo() {
        let mut d = doc(3, 3);
        d.select(&[(0, 0)], Some(&purl()));
        d.select(&[(0, 0)], None);
        d.copy();
        assert_eq!(d.clipboard().len(), 1);
        d.paste(Some((2, 2))).unwrap();
        assert_eq!(d.chart().cell_at(2, 2).unwrap().symbol.name, "purl");
        d.undo();
        assert_eq!(d.chart().cell_at(2, 2).unwrap().symbol.name, "knit");

        d.select(&[(1, 1)], None);
        d.insert_rows(2, RowLocation::Below).unwrap();
        assert_eq!(d.chart().num_rows(), 5);
        d.undo();
        assert_eq!(d.chart().num_rows(), 3);
        d.chart().validate().unwrap();
    }
}

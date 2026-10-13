//! Row and column numbering and row highlighting, derived from the chart
//! and the project settings. Shared by the GUI and the SVG export.
//!
//! Like the original, rows are numbered from the bottom of the chart and
//! columns from the right, as is usual in knitting charts.

use crate::chart::Chart;
use crate::spf::{ColumnLabelMode, LabelLocation, ProjectSettings, RowLabelMode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowLabel {
    pub row: i32,
    pub text: String,
    /// Drawn left of the chart (otherwise right of it).
    pub left: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnLabel {
    pub col: i32,
    pub text: String,
}

fn show_row_label(s: &ProjectSettings, n: i32) -> bool {
    match s.row_label_mode {
        RowLabelMode::AllRows => true,
        RowLabelMode::ShowOddRows => n.rem_euclid(2) == 1,
        RowLabelMode::ShowEvenRows => n.rem_euclid(2) == 0,
        RowLabelMode::ShowRowsWithInterval => {
            (n - s.row_labels_show_interval_start).rem_euclid(s.row_labels_show_interval.max(1)) == 0
        }
    }
}

fn show_column_label(s: &ProjectSettings, n: i32) -> bool {
    match s.column_label_mode {
        ColumnLabelMode::AllColumns => true,
        ColumnLabelMode::ShowColumnsWithInterval => {
            (n - s.column_labels_show_interval_start)
                .rem_euclid(s.column_labels_show_interval.max(1))
                == 0
        }
    }
}

/// The row labels to draw. A label the user typed is shown instead of the
/// number if row labels are editable.
pub fn row_labels(chart: &Chart, s: &ProjectSettings) -> Vec<RowLabel> {
    let rows = chart.num_rows();
    (0..rows)
        .filter_map(|r| {
            let n = rows - r + (s.row_label_start - 1);
            if !show_row_label(s, n) {
                return None;
            }
            let loc = if n.rem_euclid(2) == 0 { s.even_row_label_location } else { s.odd_row_label_location };
            let text = match chart.row_labels.get(&r) {
                Some(t) if s.row_labels_editable && !t.trim().is_empty() => t.clone(),
                _ => n.to_string(),
            };
            Some(RowLabel { row: r, text, left: loc == LabelLocation::LeftOf })
        })
        .collect()
}

pub fn column_labels(chart: &Chart, s: &ProjectSettings) -> Vec<ColumnLabel> {
    let cols = chart.num_columns();
    (0..cols)
        .filter_map(|c| {
            let n = cols - c;
            if !show_column_label(s, n) {
                return None;
            }
            let text = match chart.column_labels.get(&c) {
                Some(t) if s.column_labels_editable && !t.trim().is_empty() => t.clone(),
                _ => n.to_string(),
            };
            Some(ColumnLabel { col: c, text })
        })
        .collect()
}

/// Rows that get the highlight tint (every other row; `highlight_rows_start`
/// picks which of the two alternatives).
pub fn highlighted_rows(chart: &Chart, s: &ProjectSettings) -> Vec<i32> {
    let offset = chart.num_rows() % 2;
    (0..chart.num_rows())
        .filter(|r| (r + offset + s.highlight_rows_start).rem_euclid(2) != 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qdatastream::Color;
    use crate::spf::SymbolRef;

    fn chart(rows: i32, cols: i32) -> Chart {
        Chart::new(rows, cols, SymbolRef { category: "basic".into(), name: "knit".into() }, Color::WHITE)
    }

    fn texts(l: &[RowLabel]) -> Vec<&str> {
        l.iter().map(|x| x.text.as_str()).collect()
    }

    #[test]
    fn default_numbering() {
        let c = chart(4, 3);
        let s = ProjectSettings::default();
        let rl = row_labels(&c, &s);
        assert_eq!(texts(&rl), vec!["4", "3", "2", "1"]);
        assert!(rl.iter().all(|l| !l.left));
        let cl = column_labels(&c, &s);
        assert_eq!(cl.iter().map(|l| l.text.as_str()).collect::<Vec<_>>(), vec!["3", "2", "1"]);
    }

    #[test]
    fn odd_even_and_interval_modes() {
        let c = chart(6, 6);
        let mut s = ProjectSettings::default();
        s.row_label_mode = RowLabelMode::ShowOddRows;
        assert_eq!(texts(&row_labels(&c, &s)), vec!["5", "3", "1"]);
        s.row_label_mode = RowLabelMode::ShowEvenRows;
        assert_eq!(texts(&row_labels(&c, &s)), vec!["6", "4", "2"]);
        s.row_label_mode = RowLabelMode::ShowRowsWithInterval;
        s.row_labels_show_interval = 2;
        s.row_labels_show_interval_start = 1;
        assert_eq!(texts(&row_labels(&c, &s)), vec!["5", "3", "1"]);
        s.column_label_mode = ColumnLabelMode::ShowColumnsWithInterval;
        s.column_labels_show_interval = 3;
        s.column_labels_show_interval_start = 3;
        let cl = column_labels(&c, &s);
        assert_eq!(cl.iter().map(|l| l.text.as_str()).collect::<Vec<_>>(), vec!["6", "3"]);
    }

    #[test]
    fn start_number_and_sides() {
        let c = chart(3, 2);
        let mut s = ProjectSettings::default();
        s.row_label_start = 11; // bottom row is 11
        s.even_row_label_location = LabelLocation::LeftOf;
        let rl = row_labels(&c, &s);
        assert_eq!(texts(&rl), vec!["13", "12", "11"]);
        assert_eq!(rl.iter().map(|l| l.left).collect::<Vec<_>>(), vec![false, true, false]);
    }

    #[test]
    fn typed_labels_only_when_editable() {
        let mut c = chart(2, 2);
        c.row_labels.insert(0, "top".into());
        c.column_labels.insert(1, "  ".into());
        let mut s = ProjectSettings::default();
        assert_eq!(texts(&row_labels(&c, &s)), vec!["2", "1"]);
        s.row_labels_editable = true;
        s.column_labels_editable = true;
        assert_eq!(texts(&row_labels(&c, &s)), vec!["top", "1"]);
        // blank typed text falls back to the number
        assert_eq!(column_labels(&c, &s)[1].text, "1");
    }

    #[test]
    fn highlight_alternates_from_the_bottom_row_parity() {
        let mut s = ProjectSettings::default();
        s.highlight_rows_start = 0;
        assert_eq!(highlighted_rows(&chart(10, 2), &s), vec![1, 3, 5, 7, 9]);
        assert_eq!(highlighted_rows(&chart(9, 2), &s), vec![0, 2, 4, 6, 8]);
        s.highlight_rows_start = 1;
        assert_eq!(highlighted_rows(&chart(10, 2), &s), vec![0, 2, 4, 6, 8]);
    }
}

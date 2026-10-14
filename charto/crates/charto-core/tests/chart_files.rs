//! Runs the chart editor on the real sample projects of the original.

use charto_core::chart::{Chart, ColumnLocation, RowLocation};
use charto_core::document::Document;
use charto_core::qdatastream::Color;
use charto_core::spf::{parse_project, serialize_project, Project, SymbolRef};

fn knit() -> SymbolRef {
    SymbolRef { category: "basic".into(), name: "knit".into() }
}

fn load(bytes: &[u8]) -> (Project, Chart) {
    let p = parse_project(bytes).expect("project");
    let c = Chart::from_project(&p, knit(), Color::WHITE).expect("chart");
    (p, c)
}

#[test]
fn sample_projects_load_as_consistent_charts() {
    for bytes in [
        &include_bytes!("data/test_api_1.spf")[..],
        &include_bytes!("data/test_api_2.spf")[..],
        &include_bytes!("data/test_delete_insert_rows_columns.spf")[..],
    ] {
        let (p, c) = load(bytes);
        c.validate().unwrap();
        assert_eq!(c.cell_count(), p.grid_items.len());
        assert!(c.num_rows() > 0 && c.num_columns() > 0);
    }
}

#[test]
fn edits_on_a_real_project_keep_the_grid_valid_and_undo_restores_it() {
    let (_, chart) = load(include_bytes!("data/test_delete_insert_rows_columns.spf"));
    let original = chart.clone();
    let rows = chart.num_rows();
    let cols = chart.num_columns();
    let mut d = Document::new(chart);

    // insert two rows above row 1
    let keys = d.chart().row_keys(1);
    d.select(&keys[..1], None);
    d.insert_rows(2, RowLocation::Above).unwrap();
    assert_eq!(d.chart().num_rows(), rows + 2);
    d.chart().validate().unwrap();

    // delete the first row again
    let keys = d.chart().row_keys(0);
    d.select(&keys[..1], None);
    d.delete_rows().unwrap();
    assert_eq!(d.chart().num_rows(), rows + 1);
    d.chart().validate().unwrap();

    // insert a column at the left edge, then at the right edge
    let keys = d.chart().column_keys(0);
    d.select(&keys[..1], None);
    d.insert_columns(1, ColumnLocation::LeftOf).unwrap();
    let last = d.chart().num_columns() - 1;
    let keys = d.chart().column_keys(last);
    d.select(&keys[..1], None);
    d.insert_columns(3, ColumnLocation::RightOf).unwrap();
    assert_eq!(d.chart().num_columns(), cols + 4);
    d.chart().validate().unwrap();

    while d.undo() {}
    assert_eq!(d.chart(), &original);
}

#[test]
fn edited_chart_saves_and_reloads() {
    let (mut p, chart) = load(include_bytes!("data/test_api_1.spf"));
    let mut d = Document::new(chart);
    let keys = d.chart().row_keys(0);
    d.select(&keys[..1], None);
    d.insert_rows(1, RowLocation::Below).unwrap();

    d.chart().write_to_project(&mut p);
    let bytes = serialize_project(&p);
    let (_, reloaded) = load(&bytes);
    assert_eq!(reloaded.num_rows(), d.chart().num_rows());
    assert_eq!(reloaded.num_columns(), d.chart().num_columns());
    assert_eq!(reloaded.cell_count(), d.chart().cell_count());
}
